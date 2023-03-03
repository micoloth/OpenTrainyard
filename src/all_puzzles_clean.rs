
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
    let puzzles: [PuzzleData; 231] = [
    PuzzleData {
        // "local_filename_map": "Red Line.png",
        name: "Red Line".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/redLine",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "3/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798688_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redLine_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Grorange lines.png",
        name: "Grorange lines".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/grorangeLines",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sb_g 00 00 00 E1000_g 00\n00 00 00 00 00 Sr_o 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 E0010_o 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "5/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798482_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/grorangeLines_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Yorple lines.png",
        name: "Yorple lines".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/yorpleLines",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "E0001_p 00 00 00 00 00 Sr_p\n00 00 Sb_y 00 E1000_y 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E0100_y 00 St_y 00 00\nSl_p 00 00 00 00 00 E0010_p".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795538_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/yorpleLines_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Magical Trains.png",
        name: "Magical Trains".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/magicalTrains",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_p 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_p 00 00 00 00 00 St_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_p 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "9/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797260_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/magicalTrains_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Red Corner.png",
        name: "The Red Corner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theRedCorner",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sb_r 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 E0010_r 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798844_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theRedCorner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Purpablu.png",
        name: "Purpablu".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/purpablu",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sl_p 00 00 00 Sb_b 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 E0001_p 00 00 00 E1000_b 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B6/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797882_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/purpablu_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "A Rock in the Way.png",
        name: "A Rock in the Way".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aRockInTheWay",
        city: "Brampton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_g 00 00 MM 00 00 St_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798539_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aRockInTheWay_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Green Wally.png",
        name: "Green Wally".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/greenWally",
        city: "Brampton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 E0100_g 00 00 00 00 00\n00 00 00 00 00 00 00\n00 MM MM MM MM MM MM\n00 00 00 00 00 00 00\n00 Sb_g 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "9/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796267_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/greenWally_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Yellow Snake.png",
        name: "Yellow Snake".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/yellowSnake",
        city: "Brampton Puzzles".to_string(),
        parsed_map: "00 00 00 MM 00 00 E0001_y\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\nSl_y MM 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797784_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/yellowSnake_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "U-Turn.png",
        name: "U-Turn".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/uTurn",
        city: "Brampton Puzzles".to_string(),
        parsed_map: "Sb_p 00 00 00 00 00 00\nMM MM MM MM MM MM 00\nE0100_p 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_r 00 00 00 00 00 00\nMM MM MM MM MM MM 00\nSb_r 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2789430_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/uTurn_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Journey.png",
        name: "Journey".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/journey",
        city: "Brampton Puzzles".to_string(),
        parsed_map: "MM 00 00 00 00 00 St_o\n00 00 MM 00 00 00 00\n00 00 00 00 00 MM 00\n00 MM 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 MM\nE0100_o 00 00 00 MM 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798551_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/journey_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Rainbow.png",
        name: "Rainbow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rainbow",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "E0100_r 00 00 00 00 00 St_r\nSb_p 00 00 00 00 00 E1000_p\nE0100_b 00 00 00 00 00 St_b\nSb_g 00 00 00 00 00 E1000_g\nE0100_y 00 00 00 00 00 St_y\nSb_o 00 00 00 00 00 E1000_o\nE0100_r 00 00 00 00 00 St_r".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797499_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rainbow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Innie Outie.png",
        name: "Innie Outie".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/innieOutie",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 E0010_r 00\n00 00 00 00 E0010_b 00 00\n00 00 00 00 00 00 00\n00 00 Sr_b 00 00 00 00\n00 Sr_r 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798738_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/innieOutie_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Around the Back.png",
        name: "Around the Back".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aroundTheBack",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_r 00 Sr_y 00 Sr_b 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 E0010_b 00 E0010_r 00 E0010_y 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797385_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aroundTheBack_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Multicolor.png",
        name: "Multicolor".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/multiColor",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_b 00 00 E1111_gbry 00 00 St_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_y 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "8/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798089_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/multiColor_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Squiggle.png",
        name: "Squiggle".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/squiggle",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 St_o E0011_go Sb_g 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "8/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796921_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/squiggle_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Two Two.png",
        name: "Two Two".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/twoTwo",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_rr 00 00 00 00 00 St_rr\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "5/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798757_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/twoTwo_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Crossover.png",
        name: "Crossover".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/crossover",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_b 00 00 00 00 00 E1000_b\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0001_r 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798577_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/crossover_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Mellow Yellow.png",
        name: "Mellow Yellow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/mellowYellow",
        city: "Delson Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 E0001_y 00\n00 00 00 00 00 00 00\n00 E0100_y 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 Sl_yy 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "6/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797943_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/mellowYellow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Delivering Oranges.png",
        name: "Delivering Oranges".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/deliveringOranges",
        city: "Delson Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 St_oo E0100_o 00 E1000_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "6/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798365_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/deliveringOranges_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Purple Parcels.png",
        name: "Purple Parcels".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/purpleParcels",
        city: "Delson Puzzles".to_string(),
        parsed_map: "Sb_ppp 00 00 00 00 00 E1001_p\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\n00 MM 00 MM 00 MM 00\n00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\nE0110_p 00 00 00 00 00 E1010_p".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B5/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798662_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/purpleParcels_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Prellow.png",
        name: "Prellow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/prellow",
        city: "Delson Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 E0001_p 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 St_py 00\n00 00 00 00 00 00 00\n00 E0010_y 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "6/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798510_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/prellow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Around the Bend.png",
        name: "Around the Bend".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aroundTheBend",
        city: "Delson Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 E0100_o 00 00 00\n00 00 00 St_ob MM MM 00\n00 00 00 E0100_b 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797977_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aroundTheBend_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Preenies.png",
        name: "Preenies".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/preenies",
        city: "Delson Puzzles".to_string(),
        parsed_map: "Sr_pgpgpgpgp MM 00 00 00 00 E1001_gggg\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM MM MM 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 00 00 MM 00 00 E1010_ppppp".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797122_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/preenies_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Yield.png",
        name: "Yield".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/yield",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_y 00 00 00 Sr_y 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_y 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796325_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/yield_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Blue Boys.png",
        name: "Blue Boys".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/blueBoys",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sb_b 00 00 00 E1000_b 00\n00 00 00 00 00 00 00\n00 00 00 Sl_b 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "4/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796326_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/blueBoys_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Timing Test.png",
        name: "Timing Test".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/timingTest",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_r 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E0010_r\n00 00 00 00 00 00 00\n00 Sr_r 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798075_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/timingTest_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Grimace Town.png",
        name: "Grimace Town".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/grimaceTown",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_p 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_p 00 00 00 00 00 St_ppp\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_p 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798739_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/grimaceTown_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Lemon Latency.png",
        name: "Lemon Latency".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/lemonLatency",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "E0100_y 00 00 00 00 00 00\nSb_y 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 Sl_y\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796329_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/lemonLatency_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Three Reds.png",
        name: "Three Reds".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/threeReds",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "Sb_r 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_r 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_r 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797750_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/threeReds_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Colour Theory.png",
        name: "Colour Theory".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/colourTheory",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 Sr_b 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 E1000_g 00\n00 00 00 00 00 00 00\n00 00 Sl_y 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "5/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796331_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/colourTheory_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Secondary.png",
        name: "Secondary".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/secondary",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "Sb_r 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_y 00 00 00 00 00 E1000_o".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798797_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/secondary_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Nurple.png",
        name: "Nurple".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/nurple",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sb_r 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_p 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 St_b 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "8/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798336_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/nurple_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Micro Mix.png",
        name: "Micro Mix".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/microMix",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_b 00 00 00 Sl_y 00\n00 00 00 00 00 E0001_g 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "9/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796334_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/microMix_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The First.png",
        name: "The First".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theFirst",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 E0001_p 00 00 00\n00 00 00 00 00 00 00\n00 Sb_b 00 00 00 St_r 00\n00 00 00 00 00 00 00\n00 00 00 E0010_p 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794758_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theFirst_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Wait Outside.png",
        name: "Wait Outside".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/waitOutside",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "00 00 00 MM 00 00 E0001_p\n00 Sb_r 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 MM MM MM 00 00 00\n00 00 00 00 00 00 00\n00 MM MM MM MM MM MM\n00 00 00 00 00 00 St_b".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797065_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/waitOutside_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Nine Men's Morris.png",
        name: "Nine Men's Morris".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/nineMensMorris",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "Sb_y 00 00 00 00 00 E1001_o\n00 00 00 00 00 00 00\n00 00 MM MM MM 00 00\n00 00 MM MM MM 00 00\n00 00 MM MM MM 00 00\n00 00 00 00 00 00 00\nE0110_o 00 00 00 00 00 St_r".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B5/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797262_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/nineMensMorris_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Eee Tee.png",
        name: "Eee Tee".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/eeeTee",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 St_r 00 Sl_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_op\n00 00 00 00 00 00 00\n00 00 00 00 St_y 00 Sr_b\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B5/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795165_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/eeeTee_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Relish.png",
        name: "Relish".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/relish",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "MM Sb_g 00 00 00 00 00\nSr_y E0100_g 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 MM MM\n00 00 00 00 00 MM Sr_b\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B6/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797712_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/relish_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Mirror Squad.png",
        name: "Mirror Squad".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/mirrorSquad",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "Sr_r 00 Sr_b MM 00 E0001_g 00\n00 00 00 MM 00 00 00\n00 MM MM MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM MM MM 00\n00 00 00 MM 00 00 00\n00 E0010_p 00 MM Sl_b 00 Sl_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2793475_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/mirrorSquad_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Cute Loop.png",
        name: "Cute Loop".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/cuteLoop",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "Sr_r MM 00 00 00 00 00\n00 00 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM MM MM MM MM 00\n00 MM 00 00 00 00 00\nSl_b MM E0010_p 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796660_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/cuteLoop_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Hourglass.png",
        name: "Hourglass".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/hourglass",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "Sr_r 00 00 00 00 00 Sr_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_p MM E0100_o 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_y 00 00 00 00 00 Sl_b".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796579_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/hourglass_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Gauss.png",
        name: "Gauss".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/gauss",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "00 00 Sr_r MM Sr_b 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_p MM E0010_p 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798485_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/gauss_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Third Wheel.png",
        name: "Third Wheel".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/thirdWheel",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "00 00 00 St_y 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_o 00 00 00 00 00 E1000_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 St_r 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798443_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/thirdWheel_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Turtles.png",
        name: "Turtles".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/turtles",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "00 00 00 00 Sr_g 00 Sr_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 Sl_g 00 Sl_g\n00 MM MM MM MM MM MM\n00 00 00 00 00 00 E1000_g".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798831_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/turtles_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Royals.png",
        name: "Royals".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/royals",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "Sb_p 00 00 00 MM 00 E1000_p\n00 00 00 00 MM 00 MM\n00 00 00 00 MM 00 Sr_p\n00 00 00 00 MM 00 00\n00 00 00 00 MM 00 00\n00 00 00 00 MM 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797933_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/royals_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Spiced.png",
        name: "Spiced".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/spiced",
        city: "Guelph Puzzles".to_string(),
        parsed_map: "Sr_rr 00 00 00 00 00 Sr_yy\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_oo 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_yy 00 00 00 00 00 Sl_rr".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B7/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798087_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/spiced_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Handlebars.png",
        name: "Handlebars".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/handlebars",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "E0001_g Sb_b 00 00 00 St_y E0001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_g Sb_y 00 00 00 St_b E0010_g".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798125_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/handlebars_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Compact.png",
        name: "Compact".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/compact",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1010_o Sl_b MM 00 00\n00 00 St_r MM Sb_y 00 00\n00 00 MM Sr_r E0101_p 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798642_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/compact_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Wailing.png",
        name: "Wailing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/wailing",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_b 00 Sr_r 00 Sr_y 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 E0010_r 00 E0010_y 00 E0010_b 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795233_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/wailing_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Laser Master.png",
        name: "Laser Master".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/laserMaster",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_pppp 00 00 00 00 00 St_rbrb\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "8/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798619_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/laserMaster_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Squads.png",
        name: "Squads".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/squads",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 Sb_b 00 00 00 00 00\nSr_r 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0100_ppoo 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 Sl_y\n00 00 00 00 00 St_r 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B5/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798455_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/squads_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Aspire.png",
        name: "Aspire".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aspire",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 E0010_g E0010_r MM\n00 00 00 00 Sb_br 00 00\n00 00 00 00 Sb_ry 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798447_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aspire_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Under The Fence.png",
        name: "Under The Fence".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/underTheFence",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "Sr_r Sr_b 00 00 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM MM MM MM MM 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_r Sl_b 00 00 00 00 E1000_b".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795826_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/underTheFence_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Inverse.png",
        name: "Inverse".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/inverse",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 St_yy MM MM MM E0100_g 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_o 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 St_br MM MM MM E0100_g 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796230_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/inverse_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Totem Pole.png",
        name: "Totem Pole".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/totemPole",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_oo 00 00 Sl_y Sl_r Sr_r Sr_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796584_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/totemPole_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Western.png",
        name: "Western".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/western",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "Sr_y 00 00 00 00 00 Sr_b\n00 Sr_b 00 Sr_r 00 Sr_y 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 E0010_r 00 E0010_yb 00 E0010_b 00\n00 00 00 00 00 00 E1000_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B6/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795409_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/western_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Collider.png",
        name: "Collider".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/collider",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 00 Sr_y Sr_y Sr_y 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_gggggg 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_b Sl_b Sl_b 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798457_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/collider_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Starship Sandwich.png",
        name: "Starship Sandwich".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/starshipSandwich",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "E0001_pp Sr_r Sr_r MM Sr_b Sr_b E0001_pp\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798556_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/starshipSandwich_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Classic.png",
        name: "The Classic".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theClassic",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_g\n00 00 Sr_r 00 Sr_yy 00 Sr_b\n00 00 00 00 00 00 00\nE0100_g 00 00 00 00 00 00\nE0100_o 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798558_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theClassic_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Red Pear.png",
        name: "Red Pear".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/redPear",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "r6 00 00 Sr_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "9/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798559_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redPear_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Paint The Town.png",
        name: "Paint The Town".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/paintTheTown",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "Sb_y 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 r2 00 00 E1000_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_b 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798547_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/paintTheTown_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Lopsided.png",
        name: "Lopsided".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/lopsided",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_o 00 MM MM MM y4 St_o\nE0001_o 00 MM MM MM r6 St_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2784517_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/lopsided_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Plus.png",
        name: "Plus".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/plus",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "r6 00 00 St_y 00 00 E1001_p\n00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 MM MM MM 00 00\n00 00 00 Sr_b 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B5/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2793669_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/plus_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Orange Wall.png",
        name: "Orange Wall".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/orangeWall",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "00 00 00 St_bb 00 00 E0001_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 o2 E1000_o o5 E0100_o o2 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_o 00 00 Sb_bb 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B9/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798562_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/orangeWall_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Podded Peas.png",
        name: "Podded Peas".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/poddedPeas",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 00 00 Sb_p 00 y3\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_gg 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 Sb_p 00 b1".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796769_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/poddedPeas_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Let Them Yellow.png",
        name: "Let Them Yellow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/letThemYellow",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "E0100_y 00 00 MM 00 00 E1000_y\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 y2 00 00 St_pppp\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\nE0100_y 00 00 MM 00 00 E1000_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798211_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/letThemYellow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Original.png",
        name: "The Original".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theOriginal",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 St_bb E0100_pp 00 00 00\n00 00 St_y E0100_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 Sl_r".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796593_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theOriginal_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Stuck To You.png",
        name: "Stuck To You".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/stuckToYou",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 Sr_p MM Sr_p 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 b2 y2 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 MM E0010_g MM 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795804_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/stuckToYou_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Diagonal Mirror.png",
        name: "Diagonal Mirror".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/diagonalMirror",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 y4 00\n00 00 00 00 MM 00 00\n00 00 00 MM 00 00 00\n00 St_b MM 00 00 00 00\n00 E1001_y Sr_b 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B7/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798676_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/diagonalMirror_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Four Shadowing.png",
        name: "Four Shadowing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/fourShadowing",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "Sb_rrrr 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 b5 00 00 00\n00 00 y2 00 g2 00 00\n00 00 00 o5 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_byog".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B7/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798581_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/fourShadowing_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Fireball Island.png",
        name: "Fireball Island".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/fireballIsland",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "E0101_gg 00 00 00 00 00 E1001_oo\n00 00 00 00 00 00 00\n00 MM 00 00 00 MM 00\n00 00 00 00 00 00 00\n00 St_r 00 00 00 Sb_b 00\n00 00 Sr_y 00 Sr_y 00 00\n00 00 00 MM 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B6/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798253_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/fireballIsland_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Round The Twist.png",
        name: "Round The Twist".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/roundTheTwist",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D2 00 00 St_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_b".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795884_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/roundTheTwist_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "More Is Merrier.png",
        name: "More Is Merrier".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/moreIsMerrier",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_gggg E0001_yyyy 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D1 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_bb E0010_bb 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "9/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797918_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/moreIsMerrier_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Three Peas.png",
        name: "Three Peas".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/threePeas",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_g Sl_g Sl_g 00 00\nE0100_yyy 00 00 D4 00 00 E1000_bbb\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795886_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/threePeas_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Ackee Tree.png",
        name: "Ackee Tree".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/akeeTree",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_g D1 E0001_b E0010_yy E0001_r D2 St_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2790464_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/akeeTree_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Hookshot.png",
        name: "Hookshot".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/hookshot",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 St_b 00 00 00 E0010_r 00\n00 00 00 00 00 00 00\n00 00 00 D4 00 00 00\n00 00 00 00 00 00 00\n00 E0001_g 00 00 00 Sb_o 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798681_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/hookshot_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pick Your Partner.png",
        name: "Pick Your Partner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pickYourPartner",
        city: "London Puzzles".to_string(),
        parsed_map: "E0001_b E0001_b E0001_b MM 00 Sr_p 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 D3 00\n00 00 D2 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 E0010_r MM 00 Sl_b 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798809_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pickYourPartner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Primer.png",
        name: "Primer".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/primer",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_y 00 D1 00 00 00 E1000_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B5/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796605_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/primer_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Reunited.png",
        name: "Reunited".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/reunited",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nD2 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sl_b 00 Sr_p 00 E0010_p 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B6/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798691_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/reunited_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Star Stuck.png",
        name: "Star Stuck".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/starStuck",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 St_oooo 00 00 00 00\n00 00 00 00 b6 r1 00\n00 00 E1000_rbbr 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B4/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798692_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/starStuck_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Warm Up.png",
        name: "Warm Up".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/warmUp",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0001_bby 00 00 00 00 00 St_byr\nD2 00 00 00 00 00 00\nE0010_rry 00 00 00 00 00 St_rby\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2782610_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/warmUp_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Numerator.png",
        name: "The Numerator".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theNumerator",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_y 00 00 D2 00 00 St_o\n00 00 00 00 00 00 00\nE0100_p 00 00 D2 00 00 St_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798265_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theNumerator_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Drone vs Probe.png",
        name: "Drone vs Probe".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/droneVsProbe",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 y2 00 00 00 00\n00 00 MM 00 00 00 00\nE0100_y D3 00 MM 00 D4 E1000_b\n00 00 00 00 MM 00 00\n00 00 00 00 00 00 00\n00 00 Sl_b 00 00 E0010_y 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2787351_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/droneVsProbe_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Ochos Rios.png",
        name: "Ochos Rios".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/ochosRios",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 St_p 00 00 00 00 00\n00 00 00 E0010_ob 00 00 00\n00 00 00 D4 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 Sb_g 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B0/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798696_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/ochosRios_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Port Credit.png",
        name: "Port Credit".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/portCredit",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "E0001_y Sb_r 00 00 00 St_r E0001_b\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 b5 00 00 00 y5 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B2/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798753_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/portCredit_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Turtle.png",
        name: "Turtle".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/turtle",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "E0001_y E0001_bryb 00 00 00 00 Sr_y\n00 D2 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 r2 p2 00 00\n00 00 00 y2 g2 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B7/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798483_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/turtle_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Factories.png",
        name: "Factories".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/factories",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 E0001_b 00 00\n00 b2 00 g2 00 St_oo 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 g2 00 b2 00 St_pp 00\n00 00 00 00 E0010_g 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B6/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798271_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/factories_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Tor.png",
        name: "Tor".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/tor",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 Sr_r 00 Sr_b 00 00 00\nE0100_g 00 00 00 00 00 00\n00 Sr_y 00 Sr_r 00 00 00\nE0100_p 00 00 00 00 00 00\n00 Sr_b 00 Sr_y 00 00 00\nE0100_o 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797936_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/tor_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Horhey.png",
        name: "Horhey".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/horhey",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00\n00 00 00 E1100_r 00 00 00\n00 00 00 Sr_rrrr 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797766_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/horhey_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Christmas Eve.png",
        name: "Christmas Eve".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/christmasEve",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 00 MM MM MM 00 00\n00 00 00 00 00 00 00\n00 St_r MM MM MM Sb_g 00\n00 E1000_g MM MM MM E0100_r 00\n00 St_r MM MM MM Sb_g 00\n00 00 00 00 00 00 00\n00 00 MM MM MM 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798400_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/christmasEve_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Candlesticks.png",
        name: "Candlesticks".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/candlesticks",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "E0100_b 00 00 MM 00 00 St_g\nE0100_b 00 00 MM 00 00 St_p\nE0100_y 00 00 MM 00 00 St_o\n00 00 D2 00 00 00 00\nE0100_r 00 00 MM 00 00 St_o\nE0100_r 00 00 MM 00 00 St_p\nE0100_y 00 00 MM 00 00 St_g".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795039_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/candlesticks_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Argentan.png",
        name: "Argentan".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/argentan",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_y Sr_o Sr_y Sr_o Sr_y 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0001_yo 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B3/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798280_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/argentan_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Wagon Wheels.png",
        name: "Wagon Wheels".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/wagonWheels",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 g6 00 E1000_y 00 y3 00\n00 00 00 00 00 00 00\n00 E0001_g 00 St_zzzz 00 E0010_r 00\n00 00 00 00 00 00 00\n00 b4 00 E0100_b 00 r1 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B8/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797769_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/wagonWheels_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Gaius.png",
        name: "Gaius".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/gaius",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 E0001_r 00 E0001_r 00 E0001_r E0001_r\n00 D1 00 D1 00 D1 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 D1 00 D1 00 D1\nSl_r 00 E0010_r 00 E0010_r 00 E0010_r".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "B1/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796390_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/gaius_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Machine Gun.png",
        name: "Machine Gun".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/machineGun",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "E0100_r 00 00 00 00 00 00\nE0100_r 00 00 00 00 00 00\nE0100_r 00 00 00 00 00 00\nE0100_r 00 00 00 00 00 00\nE0100_r 00 00 00 00 00 00\nE0100_r 00 00 00 D2 00 St_r\nE0100_r 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798849_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/machineGun_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Humber.png",
        name: "Humber".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/humber",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 00 St_g E0001_ry Sb_g 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D2 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_yy Sl_o E0100_bb 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798530_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/humber_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Cooksville Creek.png",
        name: "Cooksville Creek".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/cooksvilleCreek",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "Sr_b 00 D4 E1000_bbbb 00 D4 E1000_bbbb\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 D1\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_bbbb 00 D3 E1000_bbbb 00 D3 E1000_bbbb".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798130_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/cooksvilleCreek_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Rocky Road.png",
        name: "Rocky Road".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rockyRoad",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "Sr_y 00 00 MM 00 00 Sr_b\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 MM MM MM MM MM 00\nE0010_g MM MM MM MM MM E0010_g".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B2/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796944_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rockyRoad_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Alleyway.png",
        name: "Alleyway".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/alleyway",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "00 00 00 00 MM E0001_z MM\n00 00 00 00 MM 00 MM\nSb_r 00 00 00 MM 00 MM\n00 Sb_y 00 00 00 D1 MM\nSb_b 00 00 00 MM 00 MM\n00 00 00 00 MM 00 MM\n00 00 00 00 MM E0010_z MM".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B2/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798668_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/alleyway_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Recycling Garbage.png",
        name: "Recycling Garbage".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/recyclingGarbage",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "Sr_r 00 00 00 00 00 Sr_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_zzzz 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_g 00 00 00 00 00 Sl_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B7/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798487_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/recyclingGarbage_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Shanimal.png",
        name: "Shanimal".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/shanimal",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 St_bb MM\n00 00 00 00 00 MM E0001_bp\n00 00 MM MM MM 00 00\n00 00 00 00 00 00 00\n00 00 MM MM MM 00 00\nE0010_rp MM 00 00 00 00 00\nMM Sb_rr 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B4/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798381_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/shanimal_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Cayman.png",
        name: "Cayman".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/cayman",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "E0001_y E0001_brybry 00 00 00 00 Sr_y\n00 D2 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 r2 b2 00 00\n00 00 00 y2 b2 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B8/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798383_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/cayman_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pwr Ovrwhlmng.png",
        name: "Pwr Ovrwhlmng".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pwrOvrwhlmng",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "00 E0001_b 00 00 00 E0001_y 00\n00 D2 00 00 00 D1 00\n00 00 00 D4 00 00 00\nE0100_y D4 00 00 00 D4 E1000_b\nE0100_y 00 00 00 00 00 E1000_b\n00 00 00 00 00 00 00\n00 00 00 Sl_g 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B6/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798382_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pwrOvrwhlmng_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Axiom.png",
        name: "Axiom".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/axiom",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_g\n00 00 00 00 00 00 E1000_g\nb4 00 00 00 00 00 00\n00 00 00 D2 00 00 St_oooo\ny6 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_g\n00 00 00 00 00 00 E1000_g".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B2/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798526_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/axiom_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Jagd.png",
        name: "Jagd".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/jagd",
        city: "Oakville Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_y Sr_p Sr_y Sr_p Sr_y 00\n00 00 00 00 00 00 00\n00 MM MM MM MM MM 00\n00 00 00 00 00 00 00\n00 00 00 E0001_yp 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797700_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/jagd_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "A Barrel Roll.png",
        name: "A Barrel Roll".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aBarrelRoll",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_bbbb 00 Sb_ryry 00 00\n00 00 00 E1111_pg 00 00 00\n00 00 St_ryry 00 Sr_bbbb 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796413_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aBarrelRoll_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Klickers.png",
        name: "Klickers".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/klickers",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "MM Sb_rybr 00 00 00 00 00\nE0001_op MM 00 00 00 00 00\n00 00 MM 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 MM 00 00\n00 00 00 00 00 MM 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B4/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798654_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/klickers_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Hazard.png",
        name: "Hazard".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/hazard",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "MM 00 00 MM 00 Sr_y 00\nSb_y 00 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 00 00 MM 00 MM 00\n00 MM 00 00 00 00 00\n00 00 00 00 MM 00 MM\n00 St_y MM Sl_y 00 00 E1000_y".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B9/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798385_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/hazard_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Squelchen.png",
        name: "Squelchen".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/squelchen",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 St_pp MM\n00 00 00 00 00 MM E0001_rb\n00 00 MM b5 00 00 00\n00 00 r2 00 y2 00 00\n00 00 00 g5 MM 00 00\nE0010_gy MM 00 00 00 00 00\nMM Sb_pp 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B1/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798387_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/squelchen_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Mini-Yo-We.png",
        name: "Mini-Yo-We".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/miniyowe",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "00 00 Sr_g Sr_g Sr_g 00 00\n00 00 00 00 00 00 00\n00 00 r5 b5 r5 00 00\n00 00 b5 r5 b5 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E0010_p E0010_p E0010_p 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B3/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797765_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/miniyowe_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Asymmetric.png",
        name: "Asymmetric".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/asymmetric",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "00 00 00 St_o 00 00 00\n00 00 00 00 00 E1000_y 00\n00 00 00 MM 00 00 00\n00 00 D1 MM D2 00 00\n00 00 00 MM 00 00 00\n00 E0100_r 00 00 00 00 00\n00 00 00 Sb_o 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B5/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798386_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/asymmetric_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Lorne Park.png",
        name: "Lorne Park".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/lornePark",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "Sr_r Sr_y Sr_b 00 Sr_b Sr_y Sr_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0100_ryb 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_b Sl_y Sl_r 00 Sl_r Sl_y Sl_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795453_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/lornePark_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Adama.png",
        name: "Adama".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/adama",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "00 E0001_r 00 E0001_y 00 E0001_r E0001_o\n00 D1 00 D1 00 D1 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_g 00 00 00 00 00 00\n00 00 D1 00 D1 00 D1\nSl_o 00 E0010_y 00 E0010_r 00 E0010_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796415_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/adama_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Tonelympics.png",
        name: "Tonelympics".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/tonelympics",
        city: "Peterborough Puzzles".to_string(),
        parsed_map: "MM Sb_r 00 00 00 00 00\nE0001_o MM Sb_y 00 00 00 00\n00 E0001_p MM Sb_b 00 00 00\n00 00 E0001_o MM Sb_r 00 00\n00 00 00 E0001_p MM 00 00\n00 00 00 00 00 MM 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797091_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/tonelympics_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Picnic.png",
        name: "Picnic".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/picnic",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 00 Sr_rr Sr_yy Sr_bb 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E0010_o E0010_pp E0010_g 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B3/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798499_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/picnic_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Spindle.png",
        name: "Spindle".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/spindle",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 MM 00 00 00 00\n00 00 E1000_g Sl_y E0010_r MM MM\n00 00 St_r MM Sb_b 00 00\n00 MM E0001_b Sr_g E0100_y 00 00\n00 00 00 00 MM 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B2/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2793993_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/spindle_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Fire Eyed.png",
        name: "Fire Eyed".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/fireEyed",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 St_y 00 00 00 Sb_y 00\n00 00 00 Sl_r 00 00 00\n00 00 00 00 00 00 00\nE0100_o 00 00 00 00 00 E1000_o\n00 00 00 00 00 00 00\n00 00 00 Sr_y 00 00 00\n00 St_r 00 00 00 Sb_r 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B0/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798392_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/fireEyed_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Rebelt.png",
        name: "Rebelt".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rebelt",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 Sr_g 00\n00 00 00 00 00 00 00\nE0100_y 00 00 00 00 00 00\nE0100_bb 00 00 D2 00 00 00\nE0100_o 00 00 o4 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 Sl_g 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798501_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rebelt_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "RGB.png",
        name: "RGB".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rgb",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 00 E1000_b Sb_r 00 00 00\n00 00 E1000_g Sb_g 00 00 00\n00 00 E1000_r Sb_b 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_r Sb_b 00 00 00\n00 00 E1000_g Sb_g 00 00 00\n00 00 E1000_b Sb_r 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798502_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rgb_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Dr. Linus.png",
        name: "Dr. Linus".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/drLinus",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 St_yy MM Sb_r 00 00\n00 E1000_p MM MM MM E0100_oo 00\n00 00 St_rr MM Sb_b 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798503_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/drLinus_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Glockenspiel.png",
        name: "Glockenspiel".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/glockenspiel",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "y6 00 00 D4 00 00 b3\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nD2 00 00 E1111_rrrbbbyyy 00 00 D1\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_ggg 00 00 D3 00 00 r1".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B3/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2793999_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/glockenspiel_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Jamboree.png",
        name: "Jamboree".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/jamboree",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "b6 00 b2 00 y2 00 y3\n00 00 E0100_by D3 E1000_by 00 00\n00 00 00 E1100_g 00 00 00\n00 00 00 Sr_rrrr 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B9/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798504_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/jamboree_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Licorice Allsorts.png",
        name: "Licorice Allsorts".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/licoriceAllsorts",
        city: "Quebec City Puzzles".to_string(),
        parsed_map: "00 00 00 E1000_p Sb_r 00 00\n00 00 00 E1000_g Sb_y 00 00\n00 00 00 E1000_o Sb_b 00 00\n00 00 00 E1000_b Sb_o 00 00\n00 00 00 E1000_y Sb_g 00 00\n00 00 00 E1000_r Sb_p 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798505_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/licoriceAllsorts_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sangre Grande.png",
        name: "Sangre Grande".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sangreGrande",
        city: "Regina Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_pr\n00 Sb_r 00 00 00 00 00\n00 Sb_r 00 00 00 00 00\n00 Sb_r 00 00 00 00 00\n00 Sb_r 00 00 00 00 00\n00 St_b 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B6/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798064_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sangreGrande_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Variable.png",
        name: "The Variable".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theVariable",
        city: "Regina Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_rr 00 E0010_o 00 Sr_y 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sl_r 00 E0001_ry 00 Sl_yy 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B8/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796435_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theVariable_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Orff.png",
        name: "Orff".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/orff",
        city: "Regina Puzzles".to_string(),
        parsed_map: "E0100_r 00 00 00 00 00 St_r\nE0100_y 00 00 D2 00 00 St_y\nE0100_y 00 00 00 00 00 St_b\n00 00 00 00 00 00 00\nE0100_y 00 00 00 00 00 St_r\nE0100_y 00 00 D2 00 00 St_y\nE0100_b 00 00 00 00 00 St_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796436_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/orff_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Somewhere.png",
        name: "Somewhere".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/somewhere",
        city: "Regina Puzzles".to_string(),
        parsed_map: "E0001_roygbp 00 00 00 00 00 r3\n00 00 00 00 00 o2 00\n00 00 00 00 y2 00 00\n00 00 00 g2 00 00 00\n00 00 b2 00 00 00 00\n00 p2 00 00 00 00 00\nr4 00 00 00 00 00 St_roygbp".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796437_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/somewhere_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "U-Sector.png",
        name: "U-Sector".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/usector",
        city: "Regina Puzzles".to_string(),
        parsed_map: "00 00 St_p MM Sb_g 00 00\n00 00 E1000_o MM E0100_bb 00 00\n00 MM 00 00 00 MM 00\n00 MM 00 00 00 MM 00\n00 MM 00 D4 00 MM 00\n00 MM 00 00 00 MM 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B7/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2791691_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/usector_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Taking Trash.png",
        name: "Taking Trash".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/takingTrash",
        city: "Regina Puzzles".to_string(),
        parsed_map: "Sr_r Sr_y Sr_b 00 00 00 E1001_zzz\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D1 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 E0010_r E0010_y E0010_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797215_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/takingTrash_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Quotient.png",
        name: "The Quotient".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theQuotient",
        city: "Regina Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 St_o\n00 00 00 00 00 00 00\nE0100_bbrr 00 00 D2 00 00 St_p\n00 00 00 00 00 00 00\nE0100_gy 00 00 D2 00 00 St_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 St_g".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B9/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796440_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theQuotient_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Trinidad.png",
        name: "Trinidad".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/trinidad",
        city: "Regina Puzzles".to_string(),
        parsed_map: "Sr_r 00 00 E0001_g 00 00 Sr_b\n00 00 00 00 00 00 00\n00 00 00 D2 00 00 00\n00 00 D3 E1111_ryby D4 00 00\n00 00 00 D1 00 00 00\n00 00 00 00 00 00 00\nSl_y 00 00 00 00 00 Sl_g".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B9/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797834_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/trinidad_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Denominator.png",
        name: "The Denominator".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theDenominator",
        city: "Regina Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 MM\n00 00 00 00 00 00 St_o\n00 00 D2 00 00 St_y MM\n00 00 E0011_rb 00 00 00 St_g\n00 00 D2 00 00 St_y MM\n00 00 00 00 00 00 St_o\n00 00 00 00 00 00 E1000_y".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797759_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theDenominator_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Back To Basics.png",
        name: "Back To Basics".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/backToBasics",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E0001_oogg\n00 00 St_yy 00 00 00 00\n00 00 00 00 00 Sb_r 00\n00 00 MM 00 00 00 00\n00 00 Sb_b 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B6/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796453_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/backToBasics_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Constant.png",
        name: "The Constant".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theConstant",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_rr MM E0010_o MM Sr_y 00\n00 00 MM MM MM 00 00\n00 00 00 00 00 00 00\n00 00 MM MM MM 00 00\n00 Sl_r MM E0001_ry MM Sl_yy 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B9/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794014_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theConstant_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Willow.png",
        name: "Willow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/willow",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_br 00 00 00 00 00 St_brbrbrbr\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B0/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796455_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/willow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Conquistador.png",
        name: "Conquistador".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/conquistador",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 E1011_b E0011_p E0111_r 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_g Sl_gg Sl_g 00 00\n00 00 r2 00 b2 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797407_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/conquistador_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Squier.png",
        name: "Squier".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/squier",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1001_gg\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D4 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_g 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B4/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794632_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/squier_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Oakwood Ave.png",
        name: "Oakwood Ave".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/oakwoodAve",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "E0001_p 00 00 r2 00 00 Sr_o\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\nD2 00 00 MM 00 00 D1\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\nSl_o 00 00 b2 00 00 E0010_p".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B4/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2793686_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/oakwoodAve_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Erindale.png",
        name: "Erindale".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/erindale",
        city: "St. John's Puzzles".to_string(),
        parsed_map: "00 00 00 E1100_yy 00 00 00\n00 St_r Sr_o Sr_y Sr_g Sb_b 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D3 00 00 00\n00 00 00 00 00 00 00\n00 E1000_r E0010_g E0010_y E0010_o E0100_b 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794018_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/erindale_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Slice of Life.png",
        name: "Slice of Life".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sliceOfLife",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nSr_g 00 00 00 00 00 00\n00 00 00 D1 00 00 00\nSb_r 00 00 D3 00 00 E1000_yppy\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794034_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sliceOfLife_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Mr. Morgan.png",
        name: "Mr. Morgan".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/mrMorgan",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "E0001_rrrr 00 00 00 00 00 E1000_rrr\nD2 00 00 00 00 00 00\n00 00 St_g 00 D4 E1000_rrrr 00\n00 00 00 St_o 00 00 00\n00 00 r4 00 St_b 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B4/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798161_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/mrMorgan_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Chief.png",
        name: "Chief".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/chief",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "00 00 00 D4 00 E1000_r 00\n00 00 00 00 E0010_y Sr_o 00\n00 00 MM 00 MM 00 00\n00 00 00 00 00 00 00\n00 00 MM 00 00 00 00\n00 Sl_y E0100_r 00 00 00 00\n00 00 E0100_y D3 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B9/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794038_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/chief_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Waterfall.png",
        name: "Waterfall".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/waterfall",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "Sb_b 00 00 00 00 00 00\nMM Sb_b 00 00 00 00 00\nSb_b 00 00 00 00 00 00\nMM Sb_b 00 00 00 00 E1000_b\nSb_b 00 00 00 00 00 00\nMM Sb_b 00 00 00 00 00\nSb_b 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795457_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/waterfall_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Volcano.png",
        name: "Volcano".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/volcano",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sl_r MM Sl_r 00 00 00\n00 00 St_r MM Sb_r 00 E1000_r\n00 Sr_r MM Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796462_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/volcano_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Norwich.png",
        name: "Norwich".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/norwich",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_p 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\ng5 b5 MM MM MM y5 p5\n00 00 00 00 00 00 00\n00 00 00 Sl_rprp 00 00 00\n00 00 00 E1100_gg 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B8/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2792100_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/norwich_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Three Below.png",
        name: "Three Below".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/threeBelow",
        city: "Toronto Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 MM MM\n00 00 00 00 00 MM MM\nE0100_p 00 00 00 00 r2 00\nSb_oooo 00 00 00 00 y2 00\nE0100_g 00 00 00 00 b2 00\n00 00 00 00 00 MM MM\n00 00 00 00 00 MM MM".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "B3/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798674_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/threeBelow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Mockingbird.png",
        name: "Mockingbird".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/mockingbird",
        city: "Uxbridge Puzzles".to_string(),
        parsed_map: "00 00 00 MM 00 00 00\nE0010_o 00 00 MM 00 00 E0010_g\nMM MM D2 D4 D1 MM MM\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_o 00 00 00 00 00 E0010_g\nE0100_b 00 00 Sl_gbob 00 00 E1000_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2791907_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/mockingbird_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Drummer Boy.png",
        name: "Drummer Boy".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/drummerBoy",
        city: "Uxbridge Puzzles".to_string(),
        parsed_map: "r6 00 00 Sr_y 00 00 b3\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nD2 00 D1 y5 D2 00 D1\nE0010_bb MM E0010_bb E0010_y E0010_rr MM E0010_rr".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798012_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/drummerBoy_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Forest Ave.png",
        name: "Forest Ave".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/forestAve",
        city: "Uxbridge Puzzles".to_string(),
        parsed_map: "E0001_g E0001_g E0001_g E0001_g E0001_g E0001_g E0001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 y2 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_b Sl_b Sl_b Sl_y Sl_b Sl_b Sl_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794046_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/forestAve_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Parachute.png",
        name: "Parachute".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/parachute",
        city: "Uxbridge Puzzles".to_string(),
        parsed_map: "Sr_p E0001_y 00 00 00 E0001_r Sr_p\n00 00 00 00 00 00 00\n00 00 b2 00 g2 00 00\n00 00 00 MM 00 00 00\n00 00 r2 00 y2 00 00\n00 00 00 00 00 00 00\nSl_p E0010_g 00 00 00 E0010_b Sl_p".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797773_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/parachute_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Quest.png",
        name: "The Quest".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theQuest",
        city: "Uxbridge Puzzles".to_string(),
        parsed_map: "E0100_y D4 00 00 00 D4 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_b 00 00 00 00 00 E1000_b\nE0100_p 00 g2 00 D3 00 St_p".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796949_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theQuest_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Kes.png",
        name: "Kes".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/kes",
        city: "Uxbridge Puzzles".to_string(),
        parsed_map: "E0100_y 00 00 b2 00 00 E1000_y\n00 00 00 r2 00 00 St_p\n00 00 00 r2 00 00 St_p\nE0100_r 00 00 r2 00 00 St_p\n00 00 00 r2 00 00 St_p\n00 00 00 r2 00 00 St_p\nE0100_b 00 00 y2 00 00 E1000_b".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794072_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/kes_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Exhibition Station.png",
        name: "Exhibition Station".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/exhibitionStation",
        city: "Vancouver Puzzles".to_string(),
        parsed_map: "00 00 00 00 Sb_oo 00 y3\n00 00 00 00 00 00 00\n00 MM E0100_bp 00 00 00 00\n00 MM MM MM MM MM MM\n00 MM E0100_oy 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 Sb_pp 00 b1".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794908_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/exhibitionStation_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Transmogrify.png",
        name: "Transmogrify".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/transmogrify",
        city: "Vancouver Puzzles".to_string(),
        parsed_map: "Sr_g Sr_g Sr_g Sr_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0001_p E0010_o E0001_p E0010_o MM MM 00\n00 00 00 00 00 o2 00\n00 00 00 00 00 p2 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797772_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/transmogrify_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Magic Carpet.png",
        name: "Magic Carpet".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/magicCarpet",
        city: "Vancouver Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 Sl_b MM 00\n00 00 00 Sl_g MM Sb_b 00\n00 00 Sl_y MM Sb_g 00 00\n00 Sl_r MM Sb_y 00 00 00\n00 MM Sb_r 00 00 00 00\n00 00 00 00 00 00 E1010_rybg".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797944_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/magicCarpet_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Drop Off.png",
        name: "Drop Off".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/dropOff",
        city: "Vancouver Puzzles".to_string(),
        parsed_map: "00 00 E0001_rrrr 00 E0001_yyyy 00 00\n00 00 D2 00 D1 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_r E0010_o Sl_y 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798212_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/dropOff_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Doppelganger.png",
        name: "Doppelganger".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/doppelganger",
        city: "Vancouver Puzzles".to_string(),
        parsed_map: "00 00 Sr_r Sr_g Sr_b 00 00\n00 00 00 00 00 00 00\n00 00 Sr_r E0010_y Sr_b 00 00\n00 00 00 00 00 00 00\n00 00 E0010_r Sr_y E0010_b 00 00\n00 00 00 00 00 00 00\n00 00 Sl_r E0010_g Sl_b 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794087_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/doppelganger_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Circle Square.png",
        name: "Circle Square".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/circleSquare",
        city: "Whitehorse Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_ryb 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_b 00 Sl_o 00 00\n00 00 00 D3 00 00 00\n00 00 Sr_g 00 Sr_r 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_ryb 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797770_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/circleSquare_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Indusblue.png",
        name: "Indusblue".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/indusblue",
        city: "Whitehorse Puzzles".to_string(),
        parsed_map: "Sr_y 00 00 Sr_y 00 00 Sr_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_y 00 00 E1111_zzzzzzzz 00 00 St_b\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_y 00 00 Sl_y 00 00 Sl_y".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797869_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/indusblue_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Bramblewood Lane.png",
        name: "Bramblewood Lane".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/bramblewoodLane",
        city: "Whitehorse Puzzles".to_string(),
        parsed_map: "E0001_yy Sb_r 00 Sr_rr 00 St_r E0001_bb\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 b5 00 00 00 y5 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797056_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/bramblewoodLane_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Turing.png",
        name: "Turing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/turing",
        city: "Whitehorse Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_y 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 y2 D3 E1000_yyy 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0110_ggg 00 00 00 00 00 St_bbb".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797945_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/turing_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Focus Pocus.png",
        name: "Focus Pocus".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/focusPocus",
        city: "Whitehorse Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 Sb_ry 00 E1000_oo 00 00\n00 00 00 00 00 00 00\n00 00 Sb_br 00 E1000_pp 00 00\n00 00 00 00 00 00 00\n00 00 Sb_by 00 E1000_gg 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Bonus puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798131_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/focusPocus_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Green Line.png",
        name: "Green Line".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/greenLine",
        city: "British Columbia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 Sr_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_g 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "3/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798808_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/greenLine_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Three.png",
        name: "The Three".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theThree",
        city: "British Columbia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sb_r 00 00 00 E1000_r 00\nSb_y 00 00 00 00 00 E1000_y\n00 Sb_b 00 00 00 E1000_b 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798711_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theThree_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pure Magic.png",
        name: "Pure Magic".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pureMagic",
        city: "British Columbia Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_g 00 00 00 00 00 E1000_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_g 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "9/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798727_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pureMagic_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Two Liner.png",
        name: "Two Liner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/twoLiner",
        city: "British Columbia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 St_g 00 00 00\n00 00 00 00 00 00 00\n00 E0010_g 00 00 00 Sr_r 00\n00 00 00 00 00 00 00\n00 00 00 E0100_r 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "6/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798848_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/twoLiner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Little One.png",
        name: "The Little One".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theLittleOne",
        city: "British Columbia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 St_p E0100_p 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "6/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798587_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theLittleOne_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "A Hard Place.png",
        name: "A Hard Place".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aHardPlace",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "7/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798611_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aHardPlace_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "You Did.png",
        name: "You Did".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/youDid",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "E0001_y MM Sr_y 00 00 00 00\n00 MM 00 00 00 MM 00\n00 MM 00 00 00 MM 00\n00 MM 00 00 00 MM 00\n00 MM 00 00 00 MM 00\n00 MM 00 00 00 MM 00\n00 00 00 00 E0010_r MM Sl_r".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B5/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798349_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/youDid_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Traveller.png",
        name: "Traveller".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/traveller",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "Sr_b 00 00 00 00 00 00\n00 00 MM 00 00 00 00\n00 00 00 00 00 MM 00\nMM 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 MM 00 00 MM 00 00\n00 00 00 00 00 00 E1010_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798333_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/traveller_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "In The Middle.png",
        name: "In The Middle".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/inTheMiddle",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 St_p 00 00 00 00 00\n00 00 Sr_y 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 E0010_y 00 00\n00 00 00 00 00 E0100_p 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798833_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/inTheMiddle_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Foray.png",
        name: "Foray".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/foray",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "Sr_y 00 00 00 00 00 Sr_b\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_yrbg 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_r 00 00 00 00 00 Sl_g".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B6/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798835_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/foray_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "No Touching.png",
        name: "No Touching".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/noTouching",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_y 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_r 00 00 00 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_y 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798671_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/noTouching_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Two For Two.png",
        name: "Two For Two".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/twoForTwo",
        city: "Alberta Puzzles".to_string(),
        parsed_map: "00 MM 00 MM 00 00 E1000_gg\n00 00 00 00 00 00 00\n00 MM 00 00 MM MM 00\n00 00 00 MM 00 00 MM\n00 00 MM 00 00 00 00\n00 00 00 00 00 00 00\nMM 00 00 00 MM 00 St_gg".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B5/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798592_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/twoForTwo_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Bluesy.png",
        name: "Bluesy".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/bluesy",
        city: "Saskatchewan Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 E0010_b 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 St_bb 00\n00 00 00 00 00 00 00\n00 E0001_b 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798398_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/bluesy_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Lantern.png",
        name: "Lantern".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/lantern",
        city: "Saskatchewan Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 E0100_o 00 E1000_g Sb_og 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "6/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798816_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/lantern_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Boom Bah.png",
        name: "Boom Bah".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/boomBah",
        city: "Saskatchewan Puzzles".to_string(),
        parsed_map: "00 00 00 Sb_yr 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_yy 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_r 00 00 St_ry 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798245_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/boomBah_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Red Green.png",
        name: "Red Green".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/redGreen",
        city: "Saskatchewan Puzzles".to_string(),
        parsed_map: "E0100_r 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_g 00 00 00 00 00 00\n00 00 00 00 00 00 St_rggr\nE0100_r 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_g 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798640_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redGreen_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Yellow Triangle.png",
        name: "Yellow Triangle".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/yellowTriangle",
        city: "Manitoba Puzzles".to_string(),
        parsed_map: "Sb_y 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_y 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_y 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "9/2".to_string(),
        // "big_image_url": "https://trainyard.ca/system/content/images/blueprintsByID/png/2798180_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/yellowTriangle_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Purpeller.png",
        name: "Purpeller".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/purpeller",
        city: "Manitoba Puzzles".to_string(),
        parsed_map: "Sb_p 00 00 00 00 MM MM\n00 00 00 00 00 00 MM\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nMM MM 00 00 Sl_p E0100_p 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B3/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798194_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/purpeller_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Reflection.png",
        name: "Reflection".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/reflection",
        city: "Manitoba Puzzles".to_string(),
        parsed_map: "Sb_r 00 Sb_r 00 00 00 E1001_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0110_b 00 00 00 St_b 00 St_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B6/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798669_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/reflection_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Combinellow.png",
        name: "Combinellow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/combinellow",
        city: "Manitoba Puzzles".to_string(),
        parsed_map: "MM MM Sr_y MM Sr_y MM MM\n00 00 00 00 00 00 MM\n00 00 00 00 00 00 St_y\n00 00 00 00 00 00 MM\n00 00 00 00 00 00 St_y\n00 00 00 00 00 00 MM\nE0100_y 00 00 00 00 00 MM".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B4/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798767_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/combinellow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Colourize.png",
        name: "Colourize".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/colourize",
        city: "Manitoba Puzzles".to_string(),
        parsed_map: "Sb_r 00 00 00 00 00 00\n00 St_b 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1010_p".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B4/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798846_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/colourize_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Weighted Top.png",
        name: "Weighted Top".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/weightedTop",
        city: "Manitoba Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_bb 00 00 00\nE0100_g 00 00 MM 00 00 E1000_g\n00 00 00 Sr_yy 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B3/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797204_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/weightedTop_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Embrace.png",
        name: "Embrace".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/embrace",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "Sb_b 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_p 00 00 00 00 00 E1000_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 St_r".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798326_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/embrace_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pee Gee.png",
        name: "Pee Gee".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/peeGee",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "Sr_y 00 E0001_p 00 E0001_p 00 Sr_b\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_b 00 00 E0010_g 00 00 Sl_r".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798819_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/peeGee_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Autumn.png",
        name: "Autumn".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/autumn",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_r 00 00 00 00 00 St_y\n00 00 00 00 00 00 00\nE0100_o 00 00 00 00 00 St_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797334_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/autumn_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Unscathed.png",
        name: "Unscathed".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/unscathed",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "00 E0001_r Sr_b MM Sr_r E0001_b 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798768_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/unscathed_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Challenger.png",
        name: "Challenger".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/challenger",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_br 00 00 00 00 00 St_yy\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_o 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798227_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/challenger_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Please Wait.png",
        name: "Please Wait".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pleaseWait",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sl_r 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM 00 00 00 00 E0010_p\n00 MM MM MM MM MM MM\n00 00 00 00 00 00 St_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B3/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798769_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pleaseWait_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Berlin.png",
        name: "Berlin".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/berlin",
        city: "Quebec Puzzles".to_string(),
        parsed_map: "00 00 00 MM 00 00 00\n00 E0100_p 00 MM 00 St_g 00\n00 00 00 MM 00 00 00\n00 E0100_r 00 00 00 St_p 00\n00 00 00 MM 00 00 00\n00 E0100_g 00 MM 00 St_r 00\n00 00 00 MM 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798774_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/berlin_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Golem.png",
        name: "Golem".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/golem",
        city: "Quebec Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_g 00 00 00\n00 00 St_y MM Sb_b 00 00\n00 00 00 E0001_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B6/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798822_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/golem_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sunburst.png",
        name: "Sunburst".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sunburst",
        city: "Quebec Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 Sr_y 00 00 00\n00 00 00 00 00 00 00\n00 Sb_y 00 E1111_y 00 St_y 00\n00 00 00 00 00 00 00\n00 00 00 Sl_y 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B3/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798357_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sunburst_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pea Shooter.png",
        name: "Pea Shooter".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/peaShooter",
        city: "Quebec Puzzles".to_string(),
        parsed_map: "00 00 00 St_b 00 00 00\n00 00 00 St_b 00 00 00\n00 00 00 Sb_y 00 00 00\n00 00 00 Sb_y 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_gg 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798770_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/peaShooter_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Scarab.png",
        name: "Scarab".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/scarab",
        city: "Quebec Puzzles".to_string(),
        parsed_map: "E0100_g 00 00 00 00 00 E1000_o\n00 00 00 Sr_y 00 00 00\n00 00 00 00 00 00 00\n00 00 Sb_b 00 St_y 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_r 00 00 00\nE0100_g 00 00 00 00 00 E1000_o".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797975_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/scarab_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Prancing.png",
        name: "Prancing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/prancing",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "Sr_b 00 00 00 00 00 Sr_y\n00 00 00 00 00 Sl_g 00\n00 00 00 00 00 00 00\n00 00 00 E0011_pg 00 00 00\n00 00 00 00 00 00 00\n00 Sr_p 00 00 00 00 00\nSl_r 00 00 00 00 00 Sl_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B5/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797996_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/prancing_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Box Seven.png",
        name: "Box Seven".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/boxSeven",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "Sr_ry Sr_bb MM MM MM E0001_g E0001_p\n00 00 MM MM MM 00 00\n00 00 MM MM MM 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B4/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798403_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/boxSeven_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sapling.png",
        name: "Sapling".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sapling",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "E0001_r 00 00 00 00 00 E0001_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_b 00 00 00 00 00 Sl_rbrbrb".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B8/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798315_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sapling_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Match Up.png",
        name: "Match Up".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/matchUp",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "E0100_p 00 00 00 00 00 E1000_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_b 00 00 00\n00 00 00 Sb_rr 00 00 00\n00 00 00 00 00 00 00\nE0100_o 00 00 00 00 00 St_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B8/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798095_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/matchUp_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Being Green.png",
        name: "Being Green".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/beingGreen",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "00 00 St_b MM Sb_y 00 00\n00 St_b MM MM MM Sb_y 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 E1000_g MM MM MM E0100_g 00\n00 00 E1000_g MM E0100_g 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797578_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/beingGreen_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Casualty.png",
        name: "Casualty".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/casualty",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "00 00 Sr_r E0001_p Sr_r 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_y 00 00 00 00 00 St_y\n00 00 00 00 00 00 00\n00 00 Sl_b E0010_pp Sl_b 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B5/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797373_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/casualty_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Niner.png",
        name: "Niner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/niner",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 y2 00 00 00\n00 00 St_r MM Sb_r 00 00\n00 00 00 b2 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_g 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B4/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798345_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/niner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Teamwork.png",
        name: "Teamwork".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/teamwork",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 Sl_b\np5 E0011_p 00 00 00 St_g MM\n00 00 00 00 00 00 Sr_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B6/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797520_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/teamwork_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Return Policing.png",
        name: "Return Policing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/returnPolicing",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 MM\n00 00 00 00 00 MM 00\n00 00 00 00 MM 00 00\n00 00 00 b5 00 00 00\n00 00 MM 00 00 00 00\n00 MM 00 00 00 00 E0010_bbbb\nMM 00 00 00 00 00 St_rrrr".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798105_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/returnPolicing_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Formatic.png",
        name: "Formatic".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/formatic",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 y2 00 00 00\n00 00 00 St_rrrr E0111_gg 00 00\n00 00 00 b2 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798758_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/formatic_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Rainbow Arrow.png",
        name: "Rainbow Arrow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rainbowArrow",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 r5 y5 MM g5 b5 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 E0010_b E0010_g Sl_pppp E0010_y E0010_r 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798027_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rainbowArrow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "One Way.png",
        name: "One Way".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/oneWay",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 r5 00 00 00\n00 00 00 y5 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sl_b 00 Sl_b 00 Sl_b 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B2/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798765_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/oneWay_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Roundabout.png",
        name: "Roundabout".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/roundabout",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 MM\n00 00 00 Sl_r 00 MM 00\n00 00 00 00 MM 00 00\n00 St_y 00 E0100_ggpp 00 Sb_b 00\n00 00 00 00 00 00 00\n00 00 00 Sr_b 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798764_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/roundabout_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Chromashift.png",
        name: "Chromashift".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/chromaShift",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 St_g Sl_g Sb_g 00 00\n00 00 00 00 00 00 00\n00 00 r5 E0011_rr r5 00 00\n00 00 00 00 00 00 00\n00 00 St_b Sr_b Sb_b 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B2/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798243_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/chromaShift_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Balsam.png",
        name: "Balsam".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/balsam",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_pp Sb_rrrrbbbb 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B2/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798762_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/balsam_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Boomerang.png",
        name: "Boomerang".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/boomerang",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\ny5 g5 b5 00 00 00 00\n00 00 00 Sb_rrr 00 00 E1000_ygb".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B2/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798763_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/boomerang_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Taxing.png",
        name: "Taxing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/taxing",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "Sr_b Sb_y 00 00 00 00 E1001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0110_g 00 00 00 00 St_b Sl_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798803_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/taxing_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Dousing The Flame.png",
        name: "Dousing The Flame".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/dousingTheFlame",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "Sb_r 00 00 00 00 00 00\nMM Sb_r 00 00 00 00 00\nSb_r 00 00 00 00 00 00\nE0100_b 00 00 00 00 b2 00\nSb_r 00 00 00 00 00 00\nMM Sb_r 00 00 00 00 00\nSb_r 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B9/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797663_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/dousingTheFlame_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Duplex.png",
        name: "Duplex".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/duplex",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "MM Sr_b MM Sr_b Sr_b MM E0001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM Sl_y Sl_y MM Sl_y MM E0010_g".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797112_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/duplex_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pals.png",
        name: "Pals".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pals",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "E0001_go 00 00 00 00 00 00\n00 00 00 Sl_yy 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 Sb_bb 00\n00 00 00 00 00 00 00\n00 00 00 Sr_rr 00 00 00\nE0010_po 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798815_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pals_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Refresh.png",
        name: "Refresh".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/refresh",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "00 00 E0001_p E0001_p E0001_p 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 r5 00 b5 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_y Sl_y Sl_y 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B1/8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798838_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/refresh_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Partnership.png",
        name: "Partnership".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/partnership",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nE0010_g 00 00 00 00 00 Sl_bby\n00 00 00 00 00 00 00\nE0100_p 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0001_o 00 00 00 00 00 Sr_rry\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B3/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798785_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/partnership_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Reciprocate.png",
        name: "Reciprocate".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/reciprocate",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 St_o Sr_p Sb_g 00 00\n00 00 E1000_p 00 E0100_p 00 00\n00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_g 00 E0100_o 00 00\n00 00 St_o Sl_p Sb_g 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796902_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/reciprocate_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Impact.png",
        name: "Impact".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/impact",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_rrrr 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_pppp 00 00 00 00 00 E1000_pppp\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_bbbb 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B2/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798786_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/impact_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Moonbeams.png",
        name: "Moonbeams".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/moonbeams",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 Sb_pppp 00 00\ng5 MM 00 00 00 00 00\n00 MM 00 00 00 00 00\nb5 MM 00 00 00 00 00\n00 00 00 00 E0100_gbbg 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B9/4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797115_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/moonbeams_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sibilant.png",
        name: "Sibilant".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sibilant",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "E0001_g 00 E0001_gg 00 E0001_gg 00 E0001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_b 00 00 00 00 00 St_y\nSb_b 00 00 00 00 00 St_y\nSb_b 00 00 00 00 00 St_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798839_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sibilant_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Down Under.png",
        name: "Down Under".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/downUnder",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 St_r 00 E0100_y 00 00\n00 00 St_p 00 E0100_g 00 00\n00 00 St_b 00 E0100_b 00 00\n00 00 St_g 00 E0100_p 00 00\n00 00 St_y 00 E0100_r 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798167_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/downUnder_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Snowracer.png",
        name: "Snowracer".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/snowracer",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "00 00 00 00 Sr_b Sr_y Sr_r\n00 00 00 MM 00 00 00\n00 MM 00 00 MM 00 00\n00 00 MM 00 MM 00 00\n00 00 MM 00 00 MM 00\n00 00 00 MM 00 00 00\nE0010_r E0010_y E0010_b 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B4/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796218_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/snowracer_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Right of Passage.png",
        name: "Right of Passage".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rightOfPassage",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nSb_b 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nE0100_gp 00 St_g 00 Sb_y 00 St_r\nMM 00 00 00 00 00 00\nSb_b 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B9/9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798026_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rightOfPassage_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Lamport.png",
        name: "Lamport".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/lamport",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "E0100_g 00 00 00 00 00 E1000_g\nSb_y 00 00 MM 00 00 St_b\n00 MM 00 00 00 MM 00\nSb_y 00 00 MM 00 00 St_b\n00 MM 00 00 00 MM 00\nSb_y 00 00 MM 00 00 St_b\nE0100_g 00 00 00 00 00 E1000_g".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/6".to_string(),
        // "big_image_url": "https://trainyard.ca/system/content/images/blueprintsByID/png/2793794_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/lamport_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Midland.png",
        name: "Midland".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/midland",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "00 00 Sr_r Sr_r Sr_r 00 00\n00 00 00 00 00 00 00\nE0100_y 00 00 00 00 00 E1000_y\nE0100_y 00 00 y2 00 00 E1000_y\nE0100_y 00 00 00 00 00 E1000_y\n00 00 00 00 00 00 00\n00 00 Sl_r Sl_r Sl_r 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798395_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/midland_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Entanglement.png",
        name: "Entanglement".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/entanglement",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "E0100_g 00 00 00 b2 00 00\nMM 00 00 00 00 y2 00\nMM 00 00 00 00 00 r5\nE0100_o 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nSb_pppp 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B0/7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798414_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/entanglement_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Dufferin Gate.png",
        name: "Dufferin Gate".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/dufferinGate",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "E0001_b 00 00 Sr_rrrr 00 00 E0001_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\ny5 y5 y5 b5 y5 y5 y5\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_y 00 00 00 00 00 E0010_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B7/5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798424_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/dufferinGate_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Jumping Joy.png",
        name: "Jumping Joy".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/jumpingJoy",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_b MM Sl_b 00 00\nE0011_y 00 00 y2 00 00 E0011_g\n00 00 Sr_b MM Sr_b 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "B8/6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798430_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/jumpingJoy_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sidewinder.png",
        name: "Sidewinder".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sidewinder",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "00 00 St_r 00 St_r 00 St_r\n00 00 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM MM y4 r3 MM MM\n00 MM 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_y 00 E1000_y 00 E1000_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "Be+/0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798724_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sidewinder_thumb.png"
    }

    ];
    //Turn array into Vec:
    let p = PuzzlesData{puzzles: puzzles.to_vec()};
    count_duplicates(&p);
    return p;
}
