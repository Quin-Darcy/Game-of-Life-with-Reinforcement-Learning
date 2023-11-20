use std::cmp::min;

use rand::Rng;
use nannou::prelude::*;

mod grid;
mod cell;
mod agent;
mod ga;
mod constants;

use crate::grid::Grid;
use crate::agent::Agent;
use crate::constants::{WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX, EPSILON, MAX_POPULATION_REPEATS, MAX_POPULATION_AGE};


struct Model {
    agent: Agent,
    grid: Grid,

    // To track the number of times the population size has repeated
    last_population: usize,
    population_repeats: usize,

    // Counter to track the total number of iterations
    iterations: usize,
}

fn model(app: &App) -> Model {
    let num_cells = get_num_cells(WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX);
    let mut agent = Agent::new(EPSILON, num_cells);

    // Initialize grid with new state from agent
    let grid_state = agent.get_new_state();

    let grid = Grid::new(WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX, &grid_state);

    app.new_window()
        .size(WINDOW_WIDTH_MAX as u32, WINDOW_HEIGHT_MAX as u32)
        .resizable(true)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    Model { grid, agent, last_population: 0, population_repeats: 0, iterations: 0 }
}

// TODO: Implement centralized reset function which can be called from window_event
fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    // Trigger new grid if window is resized or if mouse is clicked
    match event {
        WindowEvent::Resized(_new_size) => {
            let new_rect = app.window_rect();
            let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
            let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);

            // Reset the agent
            model.agent = Agent::new(EPSILON, get_num_cells(w as f32, h as f32));

            // Reset the grid and initialize it to a new state from the agent
            let grid_state = model.agent.get_new_state();

            // Reset the number of iterations
            model.iterations = 0;

            model.grid = Grid::new(w as f32, h as f32, &grid_state);
        }
        WindowEvent::MousePressed(_button) => {
            let new_rect = app.window_rect();
            let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
            let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);

            // Reset the agent
            model.agent = Agent::new(EPSILON, get_num_cells(w as f32, h as f32));

            // Reset the grid and initialize it to a new state from the agent
            let grid_state = model.agent.get_new_state();

            // Reset the number of iterations
            model.iterations = 0;

            model.grid = Grid::new(w as f32, h as f32, &grid_state);
        }
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Increment the number of iterations
    model.iterations += 1;

    // Print data about the model and agent every 1000 iterations
    if model.iterations % 700 == 0 {
        println!("Epsilon: {}", model.agent.epsilon);
        println!("State Space Size: {}", model.agent.state_space.len());
        println!("Average Value: {}", model.agent.previous_avg_value);
        println!("-------------------------");
    }

    // Check if the population size has repeated
    if model.grid.population == model.last_population {
        model.population_repeats += 1;
    } else {
        model.last_population = model.grid.population;
        model.population_repeats = 0;
    }

    // Trigger new grid if population is zero or if the population size continues to repeat or if the population age is too high
    if model.grid.population == 0 || model.population_repeats >= MAX_POPULATION_REPEATS || model.grid.population_age >= MAX_POPULATION_AGE{
        let new_rect = app.window_rect();
        let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
        let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);

        // Reset population repeat counter
        model.population_repeats = 0;

        // Decide if the agent should explore or exploit
        let mut rng = rand::thread_rng();
        let explore = (rng.gen::<f32>() < model.agent.epsilon) || (model.agent.state_space.len() < 5);

        // If the agent is exploring, get a new state from the agent
        // Otherwise, generate new states by evolving the state space
        if explore {
            model.agent.explore();
        } else {
            model.agent.exploit();
        };

        // Update agent - With new states having been added to the state space, we need to update the agent
        model.agent.update(w, h);

        let grid_state = model.agent.get_best_state();

        // Reset grid
        model.grid = Grid::new(w as f32, h as f32, &grid_state);
    } else {
        // Update the grid and increase the population age
        model.grid.population_age += 1;
        model.grid.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw
    let draw = app.draw();

    // Set the background to black
    draw.background().color(BLACK);

    for (_, cell) in model.grid.cells.iter().enumerate() {
        // Determine the cell color based on its state
        let cell_color = if cell.state { WHITE } else { BLACK };
        let stroke_color = if cell.state { BLACK } else { WHITE };
    
        draw.rect()
            .xy(cell.pos)
            .w_h(model.grid.cell_width, model.grid.cell_height)
            .color(cell_color)
            .stroke(stroke_color)
            .stroke_weight(0.5);
    }    

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn get_num_cells(window_width: f32, window_height: f32) -> usize {
    let cell_width = constants::SCALE * window_width;
    let cell_height = constants::SCALE * window_height;
    let cols = f32::floor(window_width / cell_width) as usize;
    let rows = f32::floor(window_height / cell_height) as usize;

    cols * rows
}

fn main() {
    nannou::app(model).update(update).run();
}
