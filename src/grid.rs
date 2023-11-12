use rand::Rng;
use nannou::prelude::*;

use crate::cell::Cell;
use crate::agent::Agent;
use crate::constants::SCALE;

pub struct Grid {
    pub cells: Vec<Cell>,
    pub columns: usize,
    pub rows: usize,
    pub cell_width: f32,
    pub cell_height: f32,
    pub population: usize,
    pub population_age: usize,
}

impl Grid {
    pub fn new(window_width: f32, window_height: f32, agent: &Agent) -> Self {
        let cell_width = SCALE * window_width;
        let cell_height = SCALE * window_height;
        let cols = f32::floor(window_width / cell_width) as usize;
        let rows = f32::floor(window_height / cell_height) as usize;
        let x_padding = (window_width - (cols as f32 * cell_width)) / 2.0;
        let y_padding = (window_height - (rows as f32 * cell_height)) / 2.0;

        let num_cells = cols * rows;
        let mut population = 0;
        let population_age = 0;
        
        let mut rng = rand::thread_rng();
        let mut cells = Vec::with_capacity(num_cells);

        // Set positions for cells
        for y in 0..rows {
            for x in 0..cols {
                let x_pos = x_padding + x as f32 * cell_width - window_width / 2.0 + cell_width / 2.0;
                let y_pos = y_padding+ y as f32 * cell_height - window_height / 2.0 + cell_height / 2.0;
                let pos = pt2(x_pos, y_pos);

                let idx = y * cols + x;
                let state: bool;

                if rng.gen::<f32>() < agent.epsilon {
                    // Exploration: Choose state randomly
                    state = rng.gen::<f32>() < 0.5;
                } else {
                    // Exploitation: Use the learned probability
                    let probability = agent.probabilities[idx];
                    state = rng.gen::<f32>() < probability;
                }

                let age = 0;
                if state {
                    population += 1;
                }
                cells.push(Cell { pos, state, age });
            }
        }

        Grid { cells, columns: cols, rows, cell_width, cell_height, population, population_age}
    }

    pub fn update(&mut self) {
        let mut new_states = vec![false; self.cells.len()];

        for y in 0..self.rows {
            for x in 0..self.columns {
                let idx = y * self.columns + x;
                let cell = &self.cells[idx];
                let live_neighbors = self.count_live_neighbors(x, y);

                new_states[idx] = match (cell.state, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };
            }
        }

        for (cell, &state) in self.cells.iter_mut().zip(new_states.iter()) {
            if state && !cell.state {
                self.population += 1;
            } else if !state && cell.state {
                self.population -= 1;
            }

            if state {
                cell.age += 1;
            } 

            cell.state = state;
        }
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut live_neighbors = 0;

        for y_offset in -1..1 {
            for x_offset in -1..1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let neighbor_x = x as i32 + x_offset;
                let neighbor_y = y as i32 + y_offset;

                if neighbor_x < 0 || neighbor_x >= self.columns as i32 {
                    continue;
                }

                if neighbor_y < 0 || neighbor_y >= self.rows as i32 {
                    continue;
                }

                let neighbor_idx = neighbor_y as usize * self.columns + neighbor_x as usize;

                if self.cells[neighbor_idx].state {
                    live_neighbors += 1;
                }
            }
        }

        live_neighbors
    }
}