// Constants for the grid
pub const SCALE: f32 = 0.02;
pub const WINDOW_WIDTH_MAX: f32 = 800.0;
pub const WINDOW_HEIGHT_MAX: f32 = 800.0;

// Constants for the Model
pub const MAX_POPULATION_REPEATS: usize = 50;
pub const MAX_POPULATION_AGE: usize = 3000;

// Constants for the agent
pub const INITIAL_LIFE_RATIO: f32 = 0.3;
pub const INITIAL_PROBABILITY: f32 = 0.5;
pub const MAX_STATE_SPACE_SIZE: usize = 1000;

// Constants controlling exploration and exploitation
pub const EPSILON: f32 = 0.31;
pub const MAX_EPSILON : f32 = 0.7;
pub const MIN_EPSILON : f32 = 0.1;
pub const INCREASE_FACTOR : f32 = 0.07;
pub const DECREASE_FACTOR : f32 = 0.025;

// Constants for the GA
pub const TOURNAMENT_WINNERS_PERCENTAGE: f32 = 0.6;
pub const SELECTION_PRESSURE: f32 = 0.8;
pub const MUTATION_RATE: f32 = 0.1;
pub const CROSSOVER_RATE: f32 = 0.5;
