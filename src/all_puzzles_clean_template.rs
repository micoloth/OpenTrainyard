
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct PuzzleData {
    // pub local_filename_map: String, 
    pub name: String, 
    // pub thumb: String, 
    // pub solutions_url: String, 
    pub city: String, 
    pub parsed_map: String, 
    pub type_: String, 
    // pub big_image_url: String, 
    pub track_count: String, 
}
// struc that handles a vec of puzzles:
#[derive(Resource)]
pub struct PuzzlesData {
    pub puzzles: Vec<PuzzleData>,
}

// Function that counts which names are present more than once:
pub fn count_duplicates(puzzles: &PuzzlesData) -> Vec<String> {
    let mut duplicates: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    for puzzle in &puzzles.puzzles {
        if names.contains(&puzzle.name) {
            duplicates.push(puzzle.name.clone());
        }
        else {
            names.push(puzzle.name.clone());
        }
    }
    // Print:
    println!("Duplicates: {:?}", duplicates);
    duplicates
}

pub fn get_next_puzzle(current_puzzle: String, pluzzles: &PuzzlesData) -> Option<PuzzleData> {
    for (i, puzzle) in pluzzles.puzzles.iter().enumerate() {
        if puzzle.name == current_puzzle {
            if i < pluzzles.puzzles.len() - 1 {
                return Some(pluzzles.puzzles[i + 1].clone());
            }
            else {
                return None;
            }
        }
    }
    return None;
}

pub fn get_prev_puzzle(current_puzzle: String, pluzzles: &PuzzlesData) -> Option<PuzzleData> {
    for (i, puzzle) in pluzzles.puzzles.iter().enumerate() {
        if puzzle.name == current_puzzle {
            if i > 0 {
                return Some(pluzzles.puzzles[i - 1].clone());
            }
            else {
                return None;
            }
        }
    }
    return None;
}


pub fn load_puzzles_data() -> PuzzlesData {
    let puzzles: [PuzzleData; {PUT HERE LENGTH}] = [

    {/// HEREEEE PUT THEM HERE}
    ];
    //Turn array into Vec:
    let p = PuzzlesData{puzzles: puzzles.to_vec()};
    count_duplicates(&p);
    return p;
}
