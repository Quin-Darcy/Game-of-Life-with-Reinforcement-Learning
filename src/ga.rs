use std::collections::HashMap;

use rand::prelude::*;
use bitvec::prelude::*;

use crate::constants::{MAX_CROSSOVER_POINTS, MAX_CROSSOVER_SECTION_SIZE, MAX_MUTATION_POINTS};

pub struct GA {
    tournament_winners_percentage: f32,
    selection_pressure: f32,
    mutation_rate: f32,
    crossover_rate: f32,
}

impl GA {
    pub fn new(tournament_winners_percentage: f32, selection_pressure: f32, mutation_rate: f32, crossover_rate: f32) -> Self {
        GA { 
            tournament_winners_percentage, 
            selection_pressure, 
            mutation_rate, 
            crossover_rate,
        }
    }

    pub fn evolve(&self, population: &mut HashMap<BitVec, f32>) -> Vec<BitVec> {
        // Perform tournament selection to get the best states
        let tournament_winners = self.tournament_selection(population);

        // Perform crossover to get the new states
        let mut new_states = self.crossover(&tournament_winners);

        // Perform mutation on the new states
        self.mutate(&mut new_states);

        new_states
    }

    fn tournament_selection(&self, population: &HashMap<BitVec, f32>) -> HashMap<BitVec, f32> {
        let mut rng = thread_rng();
        let population_size = population.len();
        let mut winners: HashMap<BitVec, f32> = HashMap::new();
        let number_of_winners = (population_size as f32 * self.tournament_winners_percentage).ceil() as usize;

        // Break the population into a queue of batches where each batch will compete in a tournament
        let queue = self.get_queue(population, number_of_winners, population_size);

        // Perform tournament selection on each batch and add the winner to the winners HashMap
        for i in 0..number_of_winners {
            // Using selection pressure, decide if the fittest will win or a random individual
            let mut winner_index = if rng.gen::<f32>() > self.selection_pressure {
                    // Get the index of a random individual in the batch
                    rng.gen_range(0..queue[i].len())
                } else {
                    // Get the index of the fittest individual in the batch
                    let mut fittest_index = 0;
                    let mut max_fitness = f32::MIN;

                    for j in 0..queue[i].len() {
                        let fitness = population[&queue[i][j]];
                        if fitness > max_fitness {
                            max_fitness = fitness;
                            fittest_index = j;
                        }
                    }

                    fittest_index
                };

            // Add the winner to the winners HashMap
            winners.insert(queue[i][winner_index].clone(), population[&queue[i][winner_index]]);
        }

        winners
    }

    fn get_queue(&self, population: &HashMap<BitVec, f32>, number_of_winners: usize, population_size: usize) -> Vec<Vec<BitVec>> {
        // Collect the keys (BitVecs) of the population HashMap into a vector
        let population_keys: Vec<&BitVec> = population.keys().collect();

        // Divide the population into number_of_winners many batches - Each batch will compete in a tournament 
        let batch_size = population_size / number_of_winners;
        let remainder = population_size % number_of_winners;

        let mut queue: Vec<Vec<BitVec>> = Vec::with_capacity(number_of_winners);

        let mut current_index = 0;
        for i in 0..number_of_winners {
            let mut current_batch_size = batch_size;
            if i < remainder {
                current_batch_size += 1;
            }

            let end_index = current_index + current_batch_size;
            let batch: Vec<BitVec> = population_keys[current_index..end_index]
                .iter()
                .map(|&bitvec_ref| bitvec_ref.clone())
                .collect();
            
            queue.push(batch);

            current_index = end_index;
        }

        queue
    }

    fn crossover(&self, tournament_winners: &HashMap<BitVec, f32>) -> Vec<BitVec> {
        // Perform crossover at a rate equal to crossover_rate on the tournament winners to get the new states
        // Returns Vec<BitVec> since these are new states which haven't been evaluated yet

        // We will iterate through each winner and based on crossover rate, that winner will either stay as it is
        // or it will be replaced by a new state which is a crossover of itself and another winner

        let mut rng = thread_rng();
        let num_states = tournament_winners.len();
        let mut new_states: Vec<BitVec> = Vec::with_capacity(tournament_winners.len());

        let grid_size = tournament_winners.keys().nth(0).unwrap().len();
        let grid_side_length = (grid_size as f32).sqrt() as usize;

        for i in 0..num_states {
            let parent_state = tournament_winners.keys().nth(i).unwrap();

            let state = if rng.gen::<f32>() > self.crossover_rate {
                // If the crossover rate is less than the random number, then the state will stay as it is
                parent_state.clone()
            } else {
                // Here we will perform crossover between the current state and another state
                // We will choose the other state randomly and confirm that it is not the same as the current state
                let other_state_index = (0..num_states).filter(|&x| x != i).choose(&mut rng).unwrap();
                let other_state = tournament_winners.keys().nth(other_state_index).unwrap();

                // Clone the parent state to start with 
                let mut new_state = parent_state.clone();

                // We begin by selecting a random percentage between 1 and MAX_CROSSOVER_POINTS
                // This percentage will be used to determine the number of crossover points
                let percentage = rng.gen_range(1.0..MAX_CROSSOVER_POINTS);

                // We then calculate the number of crossover points based on the percentage
                let num_crossover_points = (percentage * grid_size as f32).ceil() as usize;

                // Next, we calculate the dimensions of each crossover section
                // This is equal to the side length of the grid multiplied by a random percentage between 0 and MAX_CROSSOVER_SECTION_SIZE
                // We can get the grid side length by taking the square root of the length of one of the state
                let crossover_size_percentage = rng.gen_range(0.0..MAX_CROSSOVER_SECTION_SIZE);
                let crossover_side_length = (grid_side_length as f32 * crossover_size_percentage).ceil() as usize;

                // Next, we will iterate through each crossover point and perform crossover
                // This will require us to construct each rectangular crossover section based on the crossover_side_length,
                // the crossover point, and its distance from the edges of the grid
                // and we will contruct the new state as a composite of the crossover sections
                for _ in 0..num_crossover_points {
                    // Determine the crossover point
                    let point_x = rng.gen_range(0..grid_side_length);
                    let point_y = rng.gen_range(0..grid_side_length);

                    // Calculate the maximum possible crossover section side length - this will be the minimum of the distance from the point to the edges of the grid
                    let max_section_size = grid_side_length - point_x.max(point_y);

                    // Calculate the actual crossover section side length - this will be the minimum of the maximum possible crossover section side length and the crossover side length
                    let crossover_size = rng.gen_range(1..=max_section_size.min(crossover_side_length));

                    // Perform crossover in the square region around the point
                    for y in point_y ..(point_y + crossover_size).min(grid_side_length) {
                        for x in point_x ..(point_x + crossover_size).min(grid_side_length) {
                            let index = y * grid_side_length + x;
                            new_state.set(index, other_state[index]);
                        }
                    }

                }

                new_state
            };

            new_states.push(state);
        }

        new_states
    }

    fn mutate(&self, new_states: &mut Vec<BitVec>) {
        let mut rng = thread_rng();
        let num_states = new_states.len();
        let state_size = new_states[0].len();

        for i in 0..num_states {
            // Decide whether or not to mutate the state
            if rng.gen::<f32>() > self.mutation_rate {
                continue;
            } else {
                // Calculate the number of mutation points
                let percentage = rng.gen_range(0.0..MAX_MUTATION_POINTS);
                let num_mutation_points = (percentage * state_size as f32).ceil() as usize;

                // Mutate the state at the mutation points
                for _ in 0..num_mutation_points {
                    let index = rng.gen_range(0..state_size);
                    let bit = new_states[i][index];
                    if let Some(bitvec) = new_states.get_mut(i) {
                        bitvec.set(index, !bit);
                    }
                }
            }
        }
    }
}