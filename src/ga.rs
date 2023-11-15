use std::collections::HashMap;

use rand::prelude::*;
use bitvec::prelude::*;


pub struct GA {
    tournament_winners_percentage: f32,
    tournament_size: f32,
    selection_pressure: f32,
    mutation_rate: f32,
    crossover_rate: f32,
}

impl GA {
    pub fn new(tournament_winners_percentage: f32, tournament_size: f32, selection_pressure: f32, mutation_rate: f32, crossover_rate: f32) -> Self {
        GA { 
            tournament_winners_percentage, 
            tournament_size: 0.2,
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

    fn tournament_selection(&self, population: &mut HashMap<BitVec, f32>) -> HashMap<BitVec, f32> {
        let mut rng = thread_rng();
        let mut winners: HashMap<BitVec, f32> = HashMap::new();
        let number_of_winners = (population.len() as f32 * self.tournament_winners_percentage).ceil() as usize;
        let tournament_size = (population.len() as f32 * self.tournament_size).ceil() as usize;
        let population_keys: Vec<&BitVec> = population.keys().collect();
    
        // TODO: Finish the rest of the code
        todo!();
    }
    

    fn crossover(&self, tournament_winners: &HashMap<BitVec, f32>) -> Vec<BitVec> {
        // Perform crossover at a rate equal to crossover_rate on the tournament winners to get the new states
        // Returns Vec<BitVec> since these are new states which haven't been evaluated yet
        todo!()
    }

    fn mutate(&self, new_states: &mut Vec<BitVec>) {
        // Perform mutation at a rate equal to mutation_rate on the new states
        // Returns Vec<BitVec> since these are new states which haven't been evaluated yet
        todo!()
    }
}