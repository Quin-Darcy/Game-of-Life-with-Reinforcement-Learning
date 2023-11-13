use std::collections::HashMap;

use rand::Rng;
use bitvec::prelude::*;

use crate::grid::Grid;
use crate::constants::{INITIAL_LIFE_RATIO, INITIAL_PROBABILITY, MAX_POPULATION_AGE, MAX_STATE_SPACE_SIZE, MAX_EPSILON, MIN_EPSILON, INCREASE_FACTOR, DECREASE_FACTOR};

pub struct Agent {
    pub state_space: HashMap<BitVec, f32>,
    pub epsilon: f32,
    pub num_cells: usize,
    previous_avg_value: f32,
}

impl Agent {
    pub fn new(epsilon: f32, num_cells: usize) -> Self {
        Agent { 
            state_space: HashMap::new(), 
            epsilon, 
            num_cells,
            previous_avg_value: 0.0,
        }
    }

    pub fn update(&mut self, grid: &Grid) {
        let grid_state = grid.grid_state.clone();

        // Get the current state probability from the state space
        let current_state_probability = self.state_space.get_mut(&grid_state).unwrap();

        // Calculate the difference between the initial and final population and normalize it with num_cells
        let population_difference = (grid.final_population as f32 - grid.initial_population as f32) / self.num_cells as f32;

        // Get the grid's population age and normalize it with MAX_POPULATION_AGE
        let population_age = grid.population_age as f32 / MAX_POPULATION_AGE as f32;

        // Update the current state probability based on the population difference and population age
        *current_state_probability += population_difference * population_age;

        // Clamp the current state probability between 0.0 and 1.0
        *current_state_probability = current_state_probability.max(0.0).min(1.0);

        // Prune the state space if it exceeds the maximum size
        if self.state_space.len() > MAX_STATE_SPACE_SIZE {
            self.prune();
        }

        // Update epsilon
        self.update_epsilon();

        // Print the current epsilon value
        println!("Epsilon: {}", self.epsilon);
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
            total_probability += *probability;
        }

        total_probability / self.state_space.len() as f32
    }

    // Remove the state with the lowest probability from the state space 
    fn prune(&mut self) {
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