


use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy_pkv::PkvStore;

use crate::{simulator::{count_tracks, count_double_tracks, pretty_print_map, Tile, parse_map}, all_puzzles_clean::PuzzlesData, utils::SelectedLevel};


/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////


// Struct with a string and 2 ints:

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SolutionData {
    pub map: String,
    pub tracks: u32,
    pub second_tracks: u32,
    pub time: u32, // time == 0 means Unsolved
}

// Define constructor: reads map, calls count_tracks and count_double_tracks:
impl SolutionData {
    pub fn new_from_tiles(map: &Vec<Vec<Tile>>, time: u32) -> Self {
        Self {
            map: pretty_print_map(&map),
            tracks: count_tracks(&map),
            second_tracks: count_double_tracks(&map),
            time: time,
        }
    }
    pub fn new_from_string(map: String, time: u32) -> Self {
        Self {
            map: map.clone(),
            tracks: count_tracks(&parse_map(&map)),
            second_tracks: count_double_tracks(&parse_map(&map)),
            time: time,
        }
    }
}

// // Enum: either an array of SolutionData or a None:
// #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub enum LevelSolutionData {
//     Solved(),
//     Unsolved,
// }

// A struct with a strin -> LevelSolutionData hashmap:
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Resource)]
pub struct SolutionsSavedData {
    pub levels: HashMap<String, Vec<SolutionData>>,
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
    pub fn get(&self, level_name: &str) -> Vec<SolutionData> {
        match self.levels.get(level_name) {
            Some(data) => data.clone(),
            None => Vec::<SolutionData>::new(),
        }
    }
}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SelectedLevelSolvedDataEvent {
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////

// pub fn save_player_data(
//     mut pkv: ResMut<PkvStore>,
//     mut solution_data_map: ResMut<SolutionsSavedData>,
//     mut level_solved_events: EventReader<LevelSolvedDataEvent>,
// ) {
//     for event in level_solved_events.iter() {
//         let level_name = event.level_name.clone();
//         let index = event.solution_index;
//         let solution_data = event.solution_data.clone();
//         _add_solution_safe(&mut solution_data_map, level_name, index, solution_data);
//         pkv.set("solved_levels_v2", &solution_data_map.clone()).expect("failed to store level data");
//     }
// }
pub fn save_player_data(
    mut pkv: ResMut<PkvStore>,
    mut solution_data_map: ResMut<SolutionsSavedData>,
    mut level_solved_events: EventReader<SelectedLevelSolvedDataEvent>,
    selected_level: ResMut<SelectedLevel>,
) {
    for _ in level_solved_events.iter() {
        let level_name = selected_level.level.clone();
        let level_solution_data = solution_data_map.levels.get_mut(&level_name);
        match level_solution_data {
            Some(solutions) => { *solutions = selected_level.maps.clone(); }
            None => { solution_data_map.levels.insert(level_name, selected_level.maps.clone()); }
        }
        // if level_solution_data == None {
        //     solution_data_map.levels.insert(level_name, selected_level.maps.clone());
        // }
        // else {
        //     *level_solution_data.unwrap() = selected_level.maps.clone();
        // }
        pkv.set("solved_levels_v2", &solution_data_map.clone()).expect("failed to store level data");
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

// pub fn _add_solution_safe(solution_data_map: &mut SolutionsSavedData, level_name: String, index: u16, solution_data: SolutionData) {
//     let level_solution_data = solution_data_map.levels.get(&level_name);
//     match level_solution_data {
//         Some(solutions) => {
//             if index >= solutions.len() as u16 {
//                 solutions.push(solution_data);
//             }
//             else {
//                 solutions[index as usize] = solution_data;
//             }
//         }
//         None => {
//             solution_data_map.levels.insert(level_name, vec![solution_data]);
//         }
//     }
// }