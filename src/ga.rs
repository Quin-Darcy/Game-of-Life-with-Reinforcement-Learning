use std::collections::HashMap;

use rand::prelude::*;
use bitvec::prelude::*;


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

        for i in 0..num_states {

        }

        todo!()
    }

    fn mutate(&self, new_states: &mut Vec<BitVec>) {
        // Perform mutation at a rate equal to mutation_rate on the new states
        // Returns Vec<BitVec> since these are new states which haven't been evaluated yet
        todo!()
    }
}