use std::cmp::min;

use nannou::prelude::*;

mod grid;
mod cell;
mod agent;
mod constants;

use crate::grid::Grid;
use crate::agent::Agent;
use crate::constants::{WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX, EPSILON, MAX_POPULATION_AGE_REPEAT};


struct Model {
    agent: Agent,
    grid: Grid,
    epochs: usize,
    last_population: usize,
    population_repeat: usize,
    population_percentage: f32,
    last_population_percentage: f32,
}

fn model(app: &App) -> Model {
    // Create the agent instance first
    let agent = Agent::new(WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX, EPSILON);

    // Then pass the reference of agent to Grid::new
    let grid = Grid::new(WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX, &agent);

    app.new_window()
        .size(WINDOW_WIDTH_MAX as u32, WINDOW_HEIGHT_MAX as u32)
        .resizable(true)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    Model { agent, grid, epochs: 0, last_population: 0, population_repeat: 0, population_percentage: 0.0, last_population_percentage: 0.0 }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    // Trigger new grid if window is resized or if mouse is clicked
    match event {
        WindowEvent::Resized(_new_size) => {
            let new_rect = app.window_rect();
            let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
            let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);

            model.agent = Agent::new(w as f32, h as f32, EPSILON);
            model.grid = Grid::new(w as f32, h as f32, &model.agent);

        }
        WindowEvent::MousePressed(_button) => {
            let new_rect = app.window_rect();
            let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
            let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);

            model.agent = Agent::new(w as f32, h as f32, EPSILON);
            model.grid = Grid::new(w as f32, h as f32, &model.agent);
        }
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let curent_percentage = model.grid.population as f32 / model.grid.cells.len() as f32;
    if model.grid.population_age == 0 {
        model.population_percentage = curent_percentage;
    } else {
        model.population_percentage = model.population_percentage + (1.0 / model.grid.population_age as f32) * (curent_percentage - model.population_percentage);
    }

    if model.grid.population == 0 || model.population_repeat >= MAX_POPULATION_AGE_REPEAT {
        let new_rect = app.window_rect();
        let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
        let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);

        let global_signal: f32;
        if model.population_percentage > model.last_population_percentage {
            global_signal = model.population_percentage;
        } else {
            global_signal = -model.population_percentage;
        }

        model.population_repeat = 0;
        model.epochs += 1;
        model.population_percentage = 0.0;
        model.agent.update(&model.grid, global_signal/10.0);
        model.grid = Grid::new(w as f32, h as f32, &model.agent);
    } else {
        if model.grid.population == model.last_population {
            model.population_repeat += 1;
        } else {
            model.population_repeat = 0;
        }

        model.grid.population_age += 1;
        model.grid.update();
    }

    model.last_population = model.grid.population;
    model.last_population_percentage = model.population_percentage;
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw
    let draw = app.draw();

    // Set the background to black
    draw.background().color(BLACK);

    for (i, cell) in model.grid.cells.iter().enumerate() {
        // Determine the cell color based on its state
        let cell_color = if cell.state { WHITE } else { BLACK };
    
        // Get the probability for this cell from the agent
        let probability = model.agent.probabilities[i];
    
        // Map the probability to a hue in the HSV color space
        // Here, we're mapping [0, 1] to [0, 360] degrees of hue
        //let hue = probability * 360.0;
        let stroke_color = nannou::color::hsv(180.0, probability, 1.0); // Full saturation and value for vibrant colors
    
        // Draw the cell with a color border based on probability
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

fn main() {
    nannou::app(model).update(update).run();
}
