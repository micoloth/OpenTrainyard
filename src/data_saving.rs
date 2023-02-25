


use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy_pkv::PkvStore;

use crate::simulator::{count_tracks, count_double_tracks, pretty_print_map, Tile, parse_map};


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


// Struct with a string and 2 ints:

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SolutionData {
    pub map: String,
    pub tracks: i32,
    pub second_tracks: i32,
}

// Define constructor: reads map, calls count_tracks and count_double_tracks:
impl SolutionData {
    pub fn new_from_tiles(map: &Vec<Vec<Tile>>) -> Self {
        Self {
            map: pretty_print_map(&map),
            tracks: count_tracks(&map),
            second_tracks: count_double_tracks(&map),
        }
    }
    pub fn new_from_string(map: String) -> Self {
        Self {
            map: map.clone(),
            tracks: count_tracks(&parse_map(&map)),
            second_tracks: count_double_tracks(&parse_map(&map)),
        }
    }
}

// Enum: either an array of SolutionData or a None:
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum LevelSolutionData {
    Solved(Vec<SolutionData>),
    Unsolved,
}

// A struct with a strin -> LevelSolutionData hashmap:
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Resource)]
pub struct SolutionsSavedData {
    pub levels: HashMap<String, LevelSolutionData>,
}
// Default: empty hashmap:
impl Default for SolutionsSavedData {
    fn default() -> Self {
        Self {
            levels: HashMap::new(),
        }
    }
}
impl SolutionsSavedData {
    // Getter: take a string, return the data if present, else Unsolved:
    pub fn get(&self, level_name: &str) -> LevelSolutionData {
        match self.levels.get(level_name) {
            Some(data) => data.clone(),
            None => LevelSolutionData::Unsolved,
        }
    }
}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LevelSolvedDataEvent {
    pub level_name: String,
    pub solution_data: SolutionData,
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////

pub fn save_player_data(
    mut pkv: ResMut<PkvStore>,
    mut solution_data_map: ResMut<SolutionsSavedData>,
    mut level_solved_events: EventReader<LevelSolvedDataEvent>,
) {
    for event in level_solved_events.iter() {
        let level_name = event.level_name.clone();
        let solution_data = event.solution_data.clone();
        let mut level_solution_data = solution_data_map.get(&level_name);
        match level_solution_data {
            LevelSolutionData::Solved(ref mut data) => {
                data.push(solution_data);
            }
            LevelSolutionData::Unsolved => {
                // Assign LevelSolutionData::Solved(vec![solution_data]) to solution_data_map[level_name]:
                solution_data_map.levels.insert(level_name, LevelSolutionData::Solved(vec![solution_data]));
            }
        }
        pkv.set("solved_levels", &solution_data_map.clone()).expect("failed to store level data");
    }
}

