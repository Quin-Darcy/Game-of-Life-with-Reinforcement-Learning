use std::collections::HashMap;

use rand::Rng;
use bitvec::prelude::*;

use crate::grid::Grid;
use crate::ga::GA;
use crate::constants::{
    INITIAL_LIFE_RATIO, 
    INITIAL_PROBABILITY, 
    MAX_POPULATION_AGE, 
    MAX_POPULATION_REPEATS,
    MAX_STATE_SPACE_SIZE, 
    MAX_EPSILON, 
    MIN_EPSILON, 
    INCREASE_FACTOR, 
    DECREASE_FACTOR,
    TOURNAMENT_WINNERS_PERCENTAGE,
    SELECTION_PRESSURE,
    MUTATION_RATE,
    CROSSOVER_RATE,
};

pub struct Agent {
    pub state_space: HashMap<BitVec, f32>,
    pub epsilon: f32,
    pub num_cells: usize,
    pub previous_avg_value: f32,
    pub ga: GA,
}

impl Agent {
    pub fn new(epsilon: f32, num_cells: usize) -> Self {
        // Initialize the GA
        let ga = GA::new(TOURNAMENT_WINNERS_PERCENTAGE, SELECTION_PRESSURE, MUTATION_RATE, CROSSOVER_RATE);

        Agent { 
            state_space: HashMap::new(), 
            epsilon, 
            num_cells,
            previous_avg_value: 0.0,
            ga,
        }
    }

    pub fn update(&mut self, w: usize, h: usize) {
        // Clone the keys to avoid borrow conflict
        let keys: Vec<BitVec> = self.state_space.keys().cloned().collect();
    
        // Iterate over the cloned keys
        for grid_state in keys {
            // Only update states with a probability of 0
            if let Some(&probability) = self.state_space.get(&grid_state) {
                if probability == 0.0 {
                    let state_probability = self.run_state(w, h, &grid_state);
                    self.state_space.insert(grid_state, state_probability);
                }
            }
        }
    
        // Prune the state space if it exceeds the maximum size
        if self.state_space.len() > MAX_STATE_SPACE_SIZE {
            self.prune();
        }
    
        // Update epsilon
        self.update_epsilon();
    }
    

    pub fn explore(&mut self) {
        // Generate a new state and add it to the state space
        let new_state = self.get_new_state();
        self.state_space.insert(new_state.clone(), INITIAL_PROBABILITY);
    }

    pub fn exploit(&mut self) {
        // We will pass state_space over to our GA to evolve it and it will return a vector of new states
        // which we will then have to run and evaluate
        let new_states = match self.ga.evolve(&self.state_space) {
            Some(states) => states,
            None => {
                println!("GA failed to evolve the state space");
                return;
            }
        };

        // Add each state to the state space with INITIAL_PROBABILITY
        for state in new_states {
            self.state_space.insert(state, INITIAL_PROBABILITY);
        }
    }

    fn run_state(&self, w: usize, h: usize, state: &BitVec) -> f32 {
        let mut grid = Grid::new(w as f32, h as f32, state);
        let mut iterations = 0;
        let mut population_repeats = 0;
        let mut last_population = grid.population;

        while grid.population > 0 && iterations < MAX_POPULATION_AGE && population_repeats < MAX_POPULATION_REPEATS {
            grid.update();
            iterations += 1;

            // Check if the population size has repeated
            if grid.population == last_population {
                population_repeats += 1;
            } else {
                last_population = grid.population;
                population_repeats = 0;
            }
        }

        // Evaluate the state based on the final population size
        let population_difference = (grid.final_population as f32 - grid.initial_population as f32) / self.num_cells as f32;
        let population_size = grid.final_population as f32 / self.num_cells as f32;

        // Get the grid's population age and normalize it with MAX_POPULATION_AGE
        let population_age = grid.population_age as f32 / MAX_POPULATION_AGE as f32;

        // Calculate a scaled difference using an exponential function
        // This will ensure that positive differences are amplified and negative differences are diminished
        let scaled_difference = 1.0 / (1.0 + f32::exp(-population_difference));

        let standard_deviation = grid.standard_deviation / self.num_cells as f32;

        // Set the state's probability based on the population difference and population age
        let mut state_probability = scaled_difference * population_age * standard_deviation;

        // Clamp the state probability between 0.0 and 1.0
        state_probability = state_probability.max(0.0).min(1.0);
        
        state_probability
    }

    pub fn get_best_state(&mut self) -> BitVec {
        if self.state_space.is_empty() {
            // Generate a new state and add it to the state space
            let new_state = self.get_new_state();
            self.state_space.insert(new_state.clone(), INITIAL_PROBABILITY);
            return new_state;
        }

        let mut best_state = None;
        let mut highest_probability = f32::MIN;

        for (state, probability) in &self.state_space {
            if *probability > highest_probability {
                highest_probability = *probability;
                best_state = Some(state.clone());
            }
        }

        best_state.expect("Expected a best state but found none")
    }
    
    pub fn get_new_state(&mut self) -> BitVec {
        let mut rng = rand::thread_rng();
        let num_alive_cells = (self.num_cells as f32 * INITIAL_LIFE_RATIO).round() as usize;

        loop {
            // Initialize all cells to dead
            let mut new_state = bitvec![0; self.num_cells];

            // Randomly set the specified number of cells to alive
            let mut alive_cells_set = 0;
            while alive_cells_set < num_alive_cells {
                let cell_index = rng.gen_range(0..self.num_cells);
                if !new_state[cell_index] {
                    new_state.set(cell_index, true);
                    alive_cells_set += 1;
                }
            }

            // Check if the new state is already in the state space
            if !self.state_space.contains_key(&new_state) {
                // Add the new state to the state space with the initial probability
                self.state_space.insert(new_state.clone(), INITIAL_PROBABILITY);
                return new_state;
            }
            // If the state is already in the state space, loop again to generate a new state
        }
    }

    pub fn update_epsilon(&mut self) {
        let current_avg_value = self.get_average_state_value();
        let rate_of_change = current_avg_value - self.previous_avg_value;

        if rate_of_change > 0.0 {
            // The average value is increasing: reduce epsilon
            self.epsilon *= 1.0 - (rate_of_change * DECREASE_FACTOR);
        } else {
            // The average value is stagnant or decreasing: increase epsilon
            self.epsilon += INCREASE_FACTOR * -rate_of_change; 
        }

        // Clamp epsilon between a minimum and maximum value
        self.epsilon = self.epsilon.clamp(MIN_EPSILON, MAX_EPSILON);

        // Update previous average value
        self.previous_avg_value = current_avg_value;
    }

    fn get_average_state_value(&self) -> f32 {
        let mut total_probability = 0.0;

        for (_, probability) in &self.state_space {
            // Check if the probability is NaN
            if probability.is_nan() {
                continue;
            }
            total_probability += *probability;
        }

        total_probability / self.state_space.len() as f32
    }

    // Remove the states with the lowest probability from the state space until it is below the maximum size
    fn prune(&mut self) {
        while self.state_space.len() > MAX_STATE_SPACE_SIZE {
            let mut lowest_probability = f32::MAX;
            let mut lowest_probability_state = None;

            for (state, probability) in &self.state_space {
                if *probability < lowest_probability {
                    lowest_probability = *probability;
                    lowest_probability_state = Some(state.clone());
                }
            }

            self.state_space.remove(&lowest_probability_state.expect("Expected a lowest probability state but found none"));
        }
    }
}