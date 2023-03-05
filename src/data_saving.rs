


use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy_pkv::PkvStore;

use crate::{simulator::{count_tracks, count_double_tracks, pretty_print_map, Tile, parse_map}, all_puzzles_clean::PuzzlesData, utils::SelectedLevel};



/////////////////////////////////////////////////////////////////////////////////////
// COMPONENTS
/////////////////////////////////////////////////////////////////////////////////////

pub const LOCAL_STORAGE_DATA_KEY: &str = "solved_levels_v3";

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

    pub fn just_begun_level(&self, level_name: &String) -> bool {
        match self.levels.get(level_name) {
            Some(data) => {
                if data.len() == 0 { true }
                else {
                    // If ALL solutions have time ==0, true:
                    let mut all_unsolved = true;
                    for sol in data {
                        if sol.time != 0 {
                            all_unsolved = false;
                            break;
                        }
                    }
                    all_unsolved
                }},
            None => {
                true
            },
        }
    }
    pub fn just_begun(&self) -> bool {
        self.levels.len() == 0
    }

    pub fn number_solved_solutions(&self) -> u32 {
        let mut count = 0;
        for (_, solutions) in self.levels.iter() {
            for sol in solutions {
                if sol.time > 0 {
                    count += 1;
                    break;
                }
            }
        }
        count
    }

    pub fn expert_mode(&self) -> bool {
        self.number_solved_solutions() >= 40
        // true
    }
    pub fn just_became_expert(&self) -> bool {
        self.number_solved_solutions() == 40
    }
}


/////////////////////////////////////////////////////////////////////////////////////
// EVENTS
/////////////////////////////////////////////////////////////////////////////////////

// # Struct that is a Tuple<String, Vec<SolutionData>>:
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DataToInsert{
    pub level_name: String,
    pub maps: Vec<SolutionData>,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SelectedLevelSolvedDataEvent {
    pub data: Option<DataToInsert>  // IF NONE, it will be TAKEN FROM THE SELECTED LEVEL RESOURCE
}


/////////////////////////////////////////////////////////////////////////////////////
// SYSTEMS
/////////////////////////////////////////////////////////////////////////////////////

pub fn save_player_data(
    mut pkv: ResMut<PkvStore>,
    mut player_solutions_data: ResMut<SolutionsSavedData>,
    mut level_solved_events: EventReader<SelectedLevelSolvedDataEvent>,
    selected_level: ResMut<SelectedLevel>,
) {
    for ev in level_solved_events.iter() {
        let (level_name, maps) = match &ev.data {
            Some(data) => (data.level_name.clone(), data.maps.clone()),
            None => (selected_level.level.clone(), selected_level.player_maps.clone()),
        };
        let level_solution_data = player_solutions_data.levels.get_mut(&level_name);
        match level_solution_data {
            Some(solutions) => { *solutions = maps; }
            None => { player_solutions_data.levels.insert(level_name, maps); }
        }
        pkv.set(LOCAL_STORAGE_DATA_KEY, &player_solutions_data.clone()).expect("failed to store level data");
    }
}



/////////////////////////////////////////////////////////////////////////////////////
// HELPER FUNCTIONS
/////////////////////////////////////////////////////////////////////////////////////

// pub fn _add_solution_safe(player_solutions_data: &mut SolutionsSavedData, level_name: String, index: u16, solution_data: SolutionData) {
//     let level_solution_data = player_solutions_data.levels.get(&level_name);
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
//             player_solutions_data.levels.insert(level_name, vec![solution_data]);
//         }
//     }
// }