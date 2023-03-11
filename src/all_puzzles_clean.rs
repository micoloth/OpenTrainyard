
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
    let puzzles: [PuzzleData; 152] = [

    
    PuzzleData {
        // "local_filename_map": "Red Line.png",
        name: "Red Line".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/redLine",
        city: "Abbotsford Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "3+0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798688_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/redLine_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Two Liner.png",
        name: "Two Liner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/twoLiner",
        city: "British Columbia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 St_g 00 00 00\n00 00 00 00 00 00 00\n00 E0010_g 00 00 00 Sr_r 00\n00 00 00 00 00 00 00\n00 00 00 E0100_r 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "6+0".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798848_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/twoLiner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Colour Theory.png",
        name: "Colour Theory".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/colourTheory",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 Sr_b 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 E1000_g 00\n00 00 00 00 00 00 00\n00 00 Sl_y 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "5+1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796331_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/colourTheory_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Blue Boys.png",
        name: "Blue Boys".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/blueBoys",
        city: "Edmonton Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sb_b 00 00 00 E1000_b 00\n00 00 00 00 00 00 00\n00 00 00 Sl_b 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "4+1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796326_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/blueBoys_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The First.png",
        name: "The First".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theFirst",
        city: "Fredericton Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_r 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_y 00 00 00 00 00 E1000_o\n00 00 00 00 00 00 00\n00 00 00 E0010_o 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7+2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2794758_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theFirst_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Embrace.png",
        name: "Embrace".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/embrace",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "Sb_b 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_p 00 00 00 00 00 E1000_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 St_r".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "11+2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798326_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/embrace_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Prellow.png",
        name: "Prellow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/prellow",
        city: "Delson Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 E0001_p 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 St_py 00\n00 00 00 00 00 00 00\n00 E0010_y 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "6+1".to_string(),
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
        track_count: "18+1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797977_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aroundTheBend_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Challenger.png",
        name: "Challenger".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/challenger",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_br 00 00 00 00 00 St_yy\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_o 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "11+3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798227_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/challenger_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Match Up.png",
        name: "Match Up".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/matchUp",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "E0100_p 00 00 00 00 00 E1000_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_b 00 00 00\n00 00 00 Sb_rr 00 00 00\n00 00 00 00 00 00 00\nE0100_o 00 00 00 00 00 St_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "18+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798095_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/matchUp_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Please Wait.png",
        name: "Please Wait".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pleaseWait",
        city: "Ontario Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sl_r 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM 00 00 00 00 E0010_p\n00 MM MM MM MM MM MM\n00 00 00 00 00 00 St_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "23+5".to_string(),
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
        track_count: "21+4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798774_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/berlin_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Red Pear.png",
        name: "Red Pear".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/redPear",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "r6 00 00 Sr_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_r 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "9+2".to_string(),
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
        track_count: "13+1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798547_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/paintTheTown_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Stuck To You.png",
        name: "Stuck To You".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/stuckToYou",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 Sr_p MM Sr_p 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 b2 y2 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 MM E0010_g MM 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "12+3".to_string(),
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
        track_count: "17+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798676_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/diagonalMirror_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Round The Twist.png",
        name: "Round The Twist".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/roundTheTwist",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 D2 00 00 St_p\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_b".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "12+4".to_string(),
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
        track_count: "9+2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797918_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/moreIsMerrier_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Warm Up.png",
        name: "Warm Up".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/warmUp",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0001_bby 00 00 00 00 00 St_byr\nD2 00 00 00 00 00 00\nE0010_rry 00 00 00 00 00 St_rby\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "7+1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2782610_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/warmUp_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Primer.png",
        name: "Primer".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/primer",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_y 00 D1 00 00 00 E1000_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "15+3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796605_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/primer_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Hookshot.png",
        name: "Hookshot".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/hookshot",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 St_b 00 00 00 E0010_r 00\n00 00 00 00 00 00 00\n00 00 00 D4 00 00 00\n00 00 00 00 00 00 00\n00 E0001_g 00 00 00 Sb_o 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "20+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798681_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/hookshot_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Preenies.png",
        name: "Preenies".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/preenies",
        city: "Delson Puzzles".to_string(),
        parsed_map: "Sr_pgpgpgpgp MM 00 00 00 00 E1001_gggg\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM MM MM 00\n00 MM 00 MM 00 00 00\n00 MM 00 MM 00 00 00\n00 00 00 MM 00 00 E1010_ppppp".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "23+1".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797122_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/preenies_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Reunited.png",
        name: "Reunited".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/reunited",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nD2 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 Sl_b 00 Sr_p 00 E0010_p 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "16+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798691_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/reunited_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sunburst.png",
        name: "Sunburst".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sunburst",
        city: "Quebec Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 Sr_y 00 00 00\n00 00 00 00 00 00 00\n00 Sb_y 00 E1111_y 00 St_y 00\n00 00 00 00 00 00 00\n00 00 00 Sl_y 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "13+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798357_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sunburst_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sapling.png",
        name: "Sapling".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sapling",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "E0001_r 00 00 00 00 00 E0001_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_b 00 00 00 00 00 Sl_rbrbrb".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "18+8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798315_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sapling_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pick Your Partner.png",
        name: "Pick Your Partner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pickYourPartner",
        city: "London Puzzles".to_string(),
        parsed_map: "E0001_b E0001_b E0001_b MM 00 Sr_p 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 D3 00\n00 00 D2 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 E0010_r MM 00 Sl_b 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "18+4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798809_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pickYourPartner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Star Stuck.png",
        name: "Star Stuck".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/starStuck",
        city: "London Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 St_oooo 00 00 00 00\n00 00 00 00 b6 r1 00\n00 00 E1000_rbbr 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "14+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798692_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/starStuck_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Casualty.png",
        name: "Casualty".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/casualty",
        city: "New Brunswick Puzzles".to_string(),
        parsed_map: "00 00 Sr_r E0001_p Sr_r 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_y 00 00 00 00 00 St_y\n00 00 00 00 00 00 00\n00 00 Sl_b E0010_pp Sl_b 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "15+8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797373_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/casualty_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Roundabout.png",
        name: "Roundabout".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/roundabout",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 MM\n00 00 00 Sl_r 00 MM 00\n00 00 00 00 MM 00 00\n00 St_y 00 E0100_ggpp 00 Sb_b 00\n00 00 00 00 00 00 00\n00 00 00 Sr_b 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "21+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798764_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/roundabout_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Dousing The Flame.png",
        name: "Dousing The Flame".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/dousingTheFlame",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "Sb_r 00 00 00 00 00 00\nMM Sb_r 00 00 00 00 00\nSb_r 00 00 00 00 00 00\nE0100_b 00 00 00 00 b2 00\nSb_r 00 00 00 00 00 00\nMM Sb_r 00 00 00 00 00\nSb_r 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "19+9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797663_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/dousingTheFlame_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Balsam.png",
        name: "Balsam".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/balsam",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_pp Sb_rrrrbbbb 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "12+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798762_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/balsam_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Rainbow.png",
        name: "Rainbow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rainbow",
        city: "Calgary Puzzles".to_string(),
        parsed_map: "E0100_r 00 00 00 00 00 St_r\nSb_p 00 00 00 00 00 E1000_p\nE0100_b 00 00 00 00 00 St_b\nSb_g 00 00 00 00 00 E1000_g\nE0100_y 00 00 00 00 00 St_y\nSb_o 00 00 00 00 00 E1000_o\nE0100_r 00 00 00 00 00 St_r".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "25+15".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797499_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/rainbow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Wailing.png",
        name: "Wailing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/wailing",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 Sr_b 00 Sr_r 00 Sr_y 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 E0010_r 00 E0010_y 00 E0010_b 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "21+4".to_string(),
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
        track_count: "8+4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798619_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/laserMaster_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Duplex.png",
        name: "Duplex".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/duplex",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "MM Sr_b MM Sr_b Sr_b MM E0001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM Sl_y Sl_y MM Sl_y MM E0010_g".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "18+11".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797112_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/duplex_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Aspire.png",
        name: "Aspire".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/aspire",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 E0010_g E0010_r MM\n00 00 00 00 Sb_br 00 00\n00 00 00 00 Sb_ry 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "14+4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798447_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/aspire_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Pals.png",
        name: "Pals".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/pals",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "E0001_go 00 00 00 00 00 00\n00 00 00 Sl_yy 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 Sb_bb 00\n00 00 00 00 00 00 00\n00 00 00 Sr_rr 00 00 00\nE0010_po 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "21+10".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798815_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/pals_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Under The Fence.png",
        name: "Under The Fence".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/underTheFence",
        city: "Halifax Puzzles".to_string(),
        parsed_map: "Sr_r Sr_b 00 00 00 00 E1000_r\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nMM MM MM MM MM 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSl_r Sl_b 00 00 00 00 E1000_b".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "21+7".to_string(),
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
        track_count: "22+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796230_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/inverse_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Partnership.png",
        name: "Partnership".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/partnership",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nE0010_g 00 00 00 00 00 Sl_bby\n00 00 00 00 00 00 00\nE0100_p 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0001_o 00 00 00 00 00 Sr_rry\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "23+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798785_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/partnership_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Totem Pole.png",
        name: "Totem Pole".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/totemPole",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_oo 00 00 Sl_y Sl_r Sr_r Sr_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "11+3".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796584_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/totemPole_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Reciprocate.png",
        name: "Reciprocate".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/reciprocate",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 St_o Sr_p Sb_g 00 00\n00 00 E1000_p 00 E0100_p 00 00\n00 00 00 00 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_g 00 E0100_o 00 00\n00 00 St_o Sl_p Sb_g 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "24+16".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796902_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/reciprocate_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Western.png",
        name: "Western".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/western",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "Sr_y 00 00 00 00 00 Sr_b\n00 Sr_b 00 Sr_r 00 Sr_y 00\n00 00 00 00 00 00 00\nMM MM MM 00 MM MM MM\n00 00 00 00 00 00 00\n00 E0010_r 00 E0010_yb 00 E0010_b 00\n00 00 00 00 00 00 E1000_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "26+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795409_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/western_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Impact.png",
        name: "Impact".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/impact",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 00 Sr_rrrr 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_pppp 00 00 00 00 00 E1000_pppp\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 Sl_bbbb 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "12+2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798786_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/impact_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Collider.png",
        name: "Collider".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/collider",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 00 Sr_y Sr_y Sr_y 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E1111_gggggg 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_b Sl_b Sl_b 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "13+6".to_string(),
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
        track_count: "23+10".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798556_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/starshipSandwich_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sibilant.png",
        name: "Sibilant".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sibilant",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "E0001_g 00 E0001_gg 00 E0001_gg 00 E0001_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nSb_b 00 00 00 00 00 St_y\nSb_b 00 00 00 00 00 St_y\nSb_b 00 00 00 00 00 St_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "20+9".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798839_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sibilant_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Boomerang.png",
        name: "Boomerang".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/boomerang",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\ny5 g5 b5 00 00 00 00\n00 00 00 Sb_rrr 00 00 E1000_ygb".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "12+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798763_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/boomerang_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Chromashift.png",
        name: "Chromashift".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/chromaShift",
        city: "Nova Scotia Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 St_g Sl_g Sb_g 00 00\n00 00 00 00 00 00 00\n00 00 r5 E0011_rr r5 00 00\n00 00 00 00 00 00 00\n00 00 St_b Sr_b Sb_b 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "22+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798243_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/chromaShift_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Classic.png",
        name: "The Classic".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theClassic",
        city: "Iqaluit Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 E1000_g\n00 00 Sr_r 00 Sr_yy 00 Sr_b\n00 00 00 00 00 00 00\nE0100_g 00 00 00 00 00 00\nE0100_o 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "12+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798558_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theClassic_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Down Under.png",
        name: "Down Under".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/downUnder",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 St_r 00 E0100_y 00 00\n00 00 St_p 00 E0100_g 00 00\n00 00 St_b 00 E0100_b 00 00\n00 00 St_g 00 E0100_p 00 00\n00 00 St_y 00 E0100_r 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "20+11".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798167_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/downUnder_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Lopsided.png",
        name: "Lopsided".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/lopsided",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_o 00 MM MM MM y4 St_o\nE0001_o 00 MM MM MM r6 St_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "14+2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2784517_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/lopsided_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Right of Passage.png",
        name: "Right of Passage".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/rightOfPassage",
        city: "Bonus: Northwest Territories Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\nSb_b 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nE0100_gp 00 St_g 00 Sb_y 00 St_r\nMM 00 00 00 00 00 00\nSb_b 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "19+9".to_string(),
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
        track_count: "20+6".to_string(),
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
        track_count: "17+12".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798395_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/midland_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Orange Wall.png",
        name: "Orange Wall".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/orangeWall",
        city: "Joliette Puzzles".to_string(),
        parsed_map: "00 00 00 St_bb 00 00 E0001_o\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 o2 E1000_o o5 E0100_o o2 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_o 00 00 Sb_bb 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "19+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798562_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/orangeWall_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Entanglement.png",
        name: "Entanglement".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/entanglement",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "E0100_g 00 00 00 b2 00 00\nMM 00 00 00 00 y2 00\nMM 00 00 00 00 00 r5\nE0100_o 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nMM 00 00 00 00 00 00\nSb_pppp 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "20+7".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798414_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/entanglement_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Podded Peas.png",
        name: "Podded Peas".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/poddedPeas",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 00 00 Sb_p 00 y3\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_gg 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 Sb_p 00 b1".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "14+2".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796769_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/poddedPeas_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Dufferin Gate.png",
        name: "Dufferin Gate".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/dufferinGate",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "E0001_b 00 00 Sr_rrrr 00 00 E0001_y\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\ny5 y5 y5 b5 y5 y5 y5\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0010_y 00 00 00 00 00 E0010_b".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "17+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798424_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/dufferinGate_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Let Them Yellow.png",
        name: "Let Them Yellow".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/letThemYellow",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "E0100_y 00 00 MM 00 00 E1000_y\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\n00 00 00 y2 00 00 St_pppp\n00 00 00 MM 00 00 00\n00 00 00 MM 00 00 00\nE0100_y 00 00 MM 00 00 E1000_y".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "18+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798211_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/letThemYellow_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Jumping Joy.png",
        name: "Jumping Joy".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/jumpingJoy",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_b MM Sl_b 00 00\nE0011_y 00 00 y2 00 00 E0011_g\n00 00 Sr_b MM Sr_b 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "18+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798430_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/jumpingJoy_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Original.png",
        name: "The Original".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theOriginal",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 St_bb E0100_pp 00 00 00\n00 00 St_y E0100_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 Sl_r".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "13+5".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2796593_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/theOriginal_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Sidewinder.png",
        name: "Sidewinder".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/sidewinder",
        city: "Bonus: Nunavut Puzzles".to_string(),
        parsed_map: "00 00 St_r 00 St_r 00 St_r\n00 00 00 00 00 00 00\n00 MM 00 00 00 00 00\n00 MM MM y4 r3 MM MM\n00 MM 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 E1000_y 00 E1000_y 00 E1000_y".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "27+10".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798724_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/sidewinder_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Niner.png",
        name: "Niner".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/niner",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 E0001_g 00 00 00\n00 00 00 00 00 00 00\n00 00 00 y2 00 00 00\n00 00 St_r MM Sb_r 00 00\n00 00 00 b2 00 00 00\n00 00 00 00 00 00 00\n00 00 00 E0010_g 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "14+4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798345_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/niner_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Return Policing.png",
        name: "Return Policing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/returnPolicing",
        city: "Prince Edward Island Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 MM\n00 00 00 00 00 MM 00\n00 00 00 00 MM 00 00\n00 00 00 b5 00 00 00\n00 00 MM 00 00 00 00\n00 MM 00 00 00 00 E0010_bbbb\nMM 00 00 00 00 00 St_rrrr".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "11+4".to_string(),
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
        track_count: "10+5".to_string(),
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
        track_count: "16+12".to_string(),
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
        track_count: "12+6".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798765_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/oneWay_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Refresh.png",
        name: "Refresh".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/refresh",
        city: "Newfoundland Puzzles".to_string(),
        parsed_map: "00 00 E0001_p E0001_p E0001_p 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 r5 00 b5 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 Sl_y Sl_y Sl_y 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "11+8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798838_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/refresh_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Moonbeams.png",
        name: "Moonbeams".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/moonbeams",
        city: "Bonus: Yukon Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 Sb_pppp 00 00\ng5 MM 00 00 00 00 00\n00 MM 00 00 00 00 00\nb5 MM 00 00 00 00 00\n00 00 00 00 E0100_gbbg 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Express puzzles".to_string(),
        track_count: "19+4".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2797115_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/moonbeams_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Four Shadowing.png",
        name: "Four Shadowing".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/fourShadowing",
        city: "Kamloops Puzzles".to_string(),
        parsed_map: "Sb_rrrr 00 00 00 00 00 00\n00 00 00 00 00 00 00\n00 00 00 b5 00 00 00\n00 00 y2 00 g2 00 00\n00 00 00 o5 00 00 00\n00 00 00 00 00 00 00\n00 00 00 00 00 00 E1000_byog".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "17+8".to_string(),
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
        track_count: "26+8".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798253_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/fireballIsland_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "The Numerator.png",
        name: "The Numerator".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/theNumerator",
        city: "Mississauga Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 00 00 00 00 00 00\nE0100_y 00 00 D2 00 00 St_o\n00 00 00 00 00 00 00\nE0100_p 00 00 D2 00 00 St_g\n00 00 00 00 00 00 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "12+5".to_string(),
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
        track_count: "13+2".to_string(),
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
        track_count: "20+8".to_string(),
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
        track_count: "22+4".to_string(),
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
        track_count: "17+7".to_string(),
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
        track_count: "16+4".to_string(),
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
        track_count: "20+12".to_string(),
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
        track_count: "11+5".to_string(),
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
        track_count: "24+10".to_string(),
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
        track_count: "18+10".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2795039_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/candlesticks_thumb.png"
    },
    PuzzleData {
        // "local_filename_map": "Wagon Wheels.png",
        name: "Wagon Wheels".to_string(),
        // "solutions_url": "http://www.trainyard.ca/solutions/wagonWheels",
        city: "Niagara Falls Puzzles".to_string(),
        parsed_map: "00 00 00 00 00 00 00\n00 g6 00 E1000_y 00 y3 00\n00 00 00 00 00 00 00\n00 E0001_g 00 St_zzzz 00 E0010_r 00\n00 00 00 00 00 00 00\n00 b4 00 E0100_b 00 r1 00\n00 00 00 00 00 00 00".to_string(),
        type_: "Regular puzzles".to_string(),
        track_count: "18+7".to_string(),
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
        track_count: "21+6".to_string(),
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
        track_count: "18+12".to_string(),
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
        track_count: "24+10".to_string(),
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
        track_count: "25+12".to_string(),
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
        track_count: "22+6".to_string(),
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
        track_count: "12+3".to_string(),
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
        track_count: "17+8".to_string(),
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
        track_count: "24+6".to_string(),
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
        track_count: "18+8".to_string(),
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
        track_count: "16+8".to_string(),
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
        track_count: "22+8".to_string(),
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
        track_count: "17+10".to_string(),
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
        track_count: "24+12".to_string(),
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
        track_count: "24+5".to_string(),
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
        track_count: "19+5".to_string(),
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
        track_count: "21+9".to_string(),
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
        track_count: "13+7".to_string(),
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
        track_count: "15+5".to_string(),
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
        track_count: "24+17".to_string(),
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
        track_count: "24+12".to_string(),
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
        track_count: "24+10".to_string(),
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
        track_count: "13+9".to_string(),
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
        track_count: "22+8".to_string(),
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
        track_count: "30+8".to_string(),
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
        track_count: "16+11".to_string(),
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
        track_count: "19+11".to_string(),
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
        track_count: "23+12".to_string(),
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
        track_count: "13+5".to_string(),
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
        track_count: "19+9".to_string(),
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
        track_count: "23+15".to_string(),
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
        track_count: "16+7".to_string(),
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
        track_count: "18+6".to_string(),
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
        track_count: "23+15".to_string(),
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
        track_count: "22+12".to_string(),
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
        track_count: "27+9".to_string(),
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
        track_count: "18+11".to_string(),
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
        track_count: "19+8".to_string(),
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
        track_count: "19+9".to_string(),
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
        track_count: "23+14".to_string(),
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
        track_count: "16+4".to_string(),
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
        track_count: "29+8".to_string(),
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
        track_count: "10+6".to_string(),
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
        track_count: "18+10".to_string(),
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
        track_count: "14+5".to_string(),
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
        track_count: "24+9".to_string(),
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
        track_count: "31+14".to_string(),
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
        track_count: "17+10".to_string(),
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
        track_count: "24+8".to_string(),
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
        track_count: "19+8".to_string(),
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
        track_count: "17+10".to_string(),
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
        track_count: "22+10".to_string(),
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
        track_count: "18+5".to_string(),
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
        track_count: "13+7".to_string(),
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
        track_count: "30+16".to_string(),
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
        track_count: "24+12".to_string(),
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
        track_count: "25+17".to_string(),
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
        track_count: "26+14".to_string(),
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
        track_count: "22+15".to_string(),
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
        track_count: "24+15".to_string(),
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
        track_count: "25+12".to_string(),
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
        track_count: "22+10".to_string(),
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
        track_count: "26+13".to_string(),
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
        track_count: "19+15".to_string(),
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
        track_count: "26+18".to_string(),
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
        track_count: "25+11".to_string(),
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
        track_count: "23+11".to_string(),
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
        track_count: "26+10".to_string(),
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
        track_count: "17+10".to_string(),
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
        track_count: "27+11".to_string(),
        // "big_image_url": "http://www.trainyard.ca//system/content/images/blueprintsByID/png/2798131_large.png",
        // "thumb": "http://s3.amazonaws.com/TrainyardSiteMisc/images/puzzles/focusPocus_thumb.png"
    },
    ];
    //Turn array into Vec:
    let p = PuzzlesData{puzzles: puzzles.to_vec()};
    count_duplicates(&p);
    return p;
}
