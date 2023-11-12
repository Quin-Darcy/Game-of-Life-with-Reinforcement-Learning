use std::cmp::min;

use nannou::prelude::*;

mod grid;
mod cell;
mod agent;
mod constants;

use crate::grid::Grid;
use crate::agent::Agent;
use crate::constants::{WINDOW_WIDTH_MAX, WINDOW_HEIGHT_MAX, EPSILON};


struct Model {
    agent: Agent,
    grid: Grid,
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

    Model { agent, grid }
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
    if model.grid.population == 0 {
        let new_rect = app.window_rect();
        let w = min(new_rect.w() as usize, WINDOW_WIDTH_MAX as usize);
        let h = min(new_rect.h() as usize, WINDOW_HEIGHT_MAX as usize);
        model.agent.update(&model.grid);
        model.grid = Grid::new(w as f32, h as f32, &model.agent);
    } else {
        model.grid.population_age += 1;
        model.grid.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw
    let draw = app.draw();

    // Set the background to black
    draw.background().color(BLACK);

    for cell in &model.grid.cells {
        // Determine the cell color based on its state
        let cell_color = if cell.state { WHITE } else { BLACK };
        let stroke_color = if cell.state { BLACK } else { WHITE };

        // Draw the cell with a white border
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
