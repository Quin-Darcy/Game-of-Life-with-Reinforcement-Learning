// Constants for the grid
pub const SCALE: f32 = 0.07;
pub const WINDOW_WIDTH_MAX: f32 = 800.0;
pub const WINDOW_HEIGHT_MAX: f32 = 800.0;

// Constants for the Model
pub const MAX_POPULATION_REPEATS: usize = 40;
pub const MAX_POPULATION_AGE: usize = 2000;

// Constants for the agent
pub const INITIAL_LIFE_RATIO: f32 = 0.3;
pub const INITIAL_PROBABILITY: f32 = 0.0;
pub const MAX_STATE_SPACE_SIZE: usize = 800;

// Constants controlling exploration and exploitation
pub const EPSILON: f32 = 0.5;
pub const MAX_EPSILON : f32 = 0.8;
pub const MIN_EPSILON : f32 = 0.05;
pub const INCREASE_FACTOR : f32 = 0.3;
pub const DECREASE_FACTOR : f32 = 0.3;
pub const MAX_CYCLE_LENGTH: usize = 4;

// Constants for the GA
pub const TOURNAMENT_WINNERS_PERCENTAGE: f32 = 0.7;
pub const SELECTION_PRESSURE: f32 = 0.69;
pub const MUTATION_RATE: f32 = 0.21;
pub const CROSSOVER_RATE: f32 = 0.65;
pub const MAX_CROSSOVER_POINTS: f32 = 0.5;
pub const MAX_CROSSOVER_SECTION_SIZE: f32 = 0.4;
pub const MAX_MUTATION_POINTS: f32 = 0.5;
