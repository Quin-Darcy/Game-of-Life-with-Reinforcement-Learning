use crate::grid::Grid;
use crate::constants::SCALE;

pub struct Agent {
    pub probabilities: Vec<f32>,
    pub epsilon: f32,
}

impl Agent {
    pub fn new(window_width: f32, window_height: f32, epsilon: f32) -> Self {
        let cell_width = SCALE * window_width;
        let cell_height = SCALE * window_height;
        let cols = f32::floor(window_width / cell_width) as usize;
        let rows = f32::floor(window_height / cell_height) as usize;
        let num_cells = cols * rows;

        let probabilities = vec![0.5; num_cells];
        Agent { probabilities, epsilon }
    }

    pub fn update(&mut self, grid: &Grid, global_signal: f32) {
        // Update each cell's probability using the cumulative average of a weighted sum of its neighbors' ages
        let population_age = grid.population_age;
        for y in 0..grid.rows {
            for x in 0..grid.columns {
                let mut num_neighbors = 0;
                let idx = y * grid.columns + x;

                let mut weighted_sum = 0.0;
                for y_offset in -1..=1 {
                    for x_offset in -1..=1 {
                        let mut weight = 1.0;
                        let neighbor_x = x as i32 + x_offset;
                        let neighbor_y = y as i32 + y_offset;

                        if neighbor_x < 0 || neighbor_x >= grid.columns as i32 {
                            continue;
                        }

                        if neighbor_y < 0 || neighbor_y >= grid.rows as i32 {
                            continue;
                        }

                        num_neighbors += 1;

                        let neighbor_idx = neighbor_y as usize * grid.columns + neighbor_x as usize;

                        if y_offset == -1 && x_offset == -1 || y_offset == 1 && x_offset == 1 || y_offset == -1 && x_offset == 1 || y_offset == 1 && x_offset == -1 {
                            weight = 0.25
                        } else if y_offset == 0 && x_offset == -1 || y_offset == 0 && x_offset == 1 || y_offset == -1 && x_offset == 0 || y_offset == 1 && x_offset == 0 {
                            weight = 0.5
                        }

                        weighted_sum += weight * grid.cells[neighbor_idx].age as f32;
                    }
                }

                if num_neighbors == 0 {
                    continue;
                }

                let cell_health = weighted_sum / num_neighbors as f32;
                self.probabilities[idx] = self.probabilities[idx] + (1.0 / population_age as f32) * (cell_health - self.probabilities[idx]);
                self.probabilities[idx] += global_signal;
                self.probabilities[idx] = self.probabilities[idx].max(0.00).min(1.0);

                // Check if the probability is NaN
                if self.probabilities[idx].is_nan() {
                    //println!("cell_health: {:.5}", cell_health);
                    self.probabilities[idx] = 0.5;
                }

            }
        }
    }
}