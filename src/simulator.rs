

use bevy::prelude::Component;

// wrongs:
// lt 
// br 
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)] pub enum Colorz {RED_, BLUE_, YELLOW_, ORANGE_, GREEN_, PURPLE_, BROWN_}
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)] pub enum TrackOptions {TL, TB, TR, LB, LR, BR}
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)] pub enum Side {T_, B_, L_, R_}

pub fn flip_side(s: Side) -> Side {match s { Side::T_ => Side::B_, Side::B_ => Side::T_, Side::L_ => Side::R_, Side::R_ => Side::L_,}}

pub fn mix_colors(c1: Colorz, c2: Colorz) -> Colorz {
    match (c1, c2) {
        (Colorz::RED_, Colorz::YELLOW_) => Colorz::ORANGE_, (Colorz::YELLOW_, Colorz::RED_) => Colorz::ORANGE_,
        (Colorz::RED_, Colorz::BLUE_) => Colorz::PURPLE_, (Colorz::BLUE_, Colorz::RED_) => Colorz::PURPLE_,
        (Colorz::YELLOW_, Colorz::BLUE_) => Colorz::GREEN_, (Colorz::BLUE_, Colorz::YELLOW_) => Colorz::GREEN_,
        (c1, c2) => if c1 == c2 {c1} else {Colorz::BROWN_},
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)] pub struct VectorOfColorz {
    pub v: [Option<Colorz>; 12]  // This is only because in this dumb language, Vecs don't have the Copy trait
}
impl VectorOfColorz {
    // Constructor that takes a Vec<Colorz> and returns a VectorOfColorz with the first 12 elements
    // If the vector has <12 elements, the remaining elements are set to None
    // pub fn new(v: Vec<Colorz>) -> VectorOfColorz {
    //     let mut v2 = [None; 12];
    //     for i in 0..12 {
    //         if i < v.len() {
    //             v2[i] = Some(v[i]);
    //         }
    //     }
    //     VectorOfColorz {v: v2}
    // }
    // // Default constructor (all nones)
    // pub fn default() -> VectorOfColorz {
    //     VectorOfColorz {v: [None; 12]}
    // }
    // ITERATOR: iter() on a VectorOfColorz yields each elem in v and STOPS at the first None element
    pub fn iter(&self) -> VectorOfColorzIter {
        VectorOfColorzIter {v: self, i: 0}
    }
    // len:
    pub fn len(&self) -> usize {
        let mut i = 0;
        for _ in self.iter() {
            i += 1;
        }
        i
    }
    pub fn remove(&mut self, i: usize) {
        for j in i..self.len() {
            self.v[j] = self.v[j+1];
        }
    }
    pub fn push (&mut self, c: Colorz) {
        let l = self.len();
        self.v[l] = Some(c);
    }
}

pub struct VectorOfColorzIter<'a> {
    pub v: &'a VectorOfColorz,
    pub i: usize,
}
impl<'a> Iterator for VectorOfColorzIter<'a> {
    type Item = Colorz;
    fn next(&mut self) -> Option<Colorz> {
        // If there is a color at pos i (and the option is not None):
        if self.i < 12 && self.v.v[self.i].is_some() {
            let i = self.i;
            self.i += 1;
            self.v.v[i]
        } else {
            None
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct Track {  pub t_: bool, pub  b_: bool,  pub l_: bool,  pub r_: bool,}
pub fn track_has(t: Track, s: Side) -> bool {match s { Side::T_ => t.t_, Side::B_ => t.b_, Side::L_ => t.l_, Side::R_ => t.r_,}}
pub fn get_track(t: TrackOptions) -> Track {
    match t {
        TrackOptions::TL => Track{t_: true, b_: false, l_: true, r_: false},
        TrackOptions::TB => Track{t_: true, b_: true, l_: false, r_: false},
        TrackOptions::TR => Track{t_: true, b_: false, l_: false, r_: true},
        TrackOptions::LB => Track{t_: false, b_: true, l_: true, r_: false},
        TrackOptions::LR => Track{t_: false, b_: false, l_: true, r_: true},
        TrackOptions::BR => Track{t_: false, b_: true, l_: false, r_: true},
        // _ => Track{t_: false, b_: false, l_: false, r_: false},  // HOW DOES IT KNOW i dont need this
    }
}
pub fn get_track_option(t: Track) -> TrackOptions {
    if (t == Track{t_: true, b_: false, l_: true, r_: false}) {TrackOptions::TL}
    else if (t == Track{t_: true, b_: true, l_: false, r_: false}) {TrackOptions::TB}
    else if (t == Track{t_: true, b_: false, l_: false, r_: true}) {TrackOptions::TR}
    else if (t == Track{t_: false, b_: true, l_: true, r_: false}) {TrackOptions::LB}
    else if (t == Track{t_: false, b_: false, l_: true, r_: true}) {TrackOptions::LR}
    else if (t == Track{t_: false, b_: true, l_: false, r_: true}) {TrackOptions::BR}
    else {panic!("Whaat {:?}", t)}
}
#[derive(Debug, Clone, PartialEq, Eq, Copy, Component)] pub enum Tile {
    TrackTile{toptrack: Track, bottrack: Track},
    SingleTrackTile{track: Track},
    EmptyTile,
    RockTile,
    StartTile{dir: Side, elems: VectorOfColorz, orig_len: i8},
    EndTile{t_: bool, b_: bool, l_: bool, r_: bool, elems: VectorOfColorz, orig_len: i8},
    PaintTile{track: Track, c: Colorz},
    SplitTile{side_in: Side},
}
// Add a default constructor for EndTile, that when receives a elems, also always sets the orig_len to elems.len():
impl Tile {
    pub fn new_end_tile(t_: bool, b_: bool, l_: bool, r_: bool, elems: VectorOfColorz) -> Tile {
        Tile::EndTile{t_: t_, b_: b_, l_: l_, r_: r_, elems: elems, orig_len: elems.len() as i8}
    }
    pub fn new_start_tile(dir: Side, elems: VectorOfColorz) -> Tile {
        Tile::StartTile{dir: dir, elems: elems, orig_len: elems.len() as i8}
    }
}


pub fn split_out_sides(s: Side) -> (Side, Side, Side) {
    match s {
        Side::B_ => (Side::B_, Side::L_, Side::R_), Side::T_ => (Side::T_, Side::R_, Side::L_),
        Side::L_ => (Side::L_, Side::B_, Side::T_), Side::R_ => (Side::R_, Side::T_, Side::B_),
    }
}
// pub fn end_tile(s: Side, elems: VectorOfColorz) -> Tile {
//     match s {
//         Side::T_ => Tile::EndTile{t_: true, b_: false, l_: false, r_: false, elems: elems},
//         Side::B_ => Tile::EndTile{t_: false, b_: true, l_: false, r_: false, elems: elems},
//         Side::L_ => Tile::EndTile{t_: false, b_: false, l_: true, r_: false, elems: elems},
//         Side::R_ => Tile::EndTile{t_: false, b_: false, l_: false, r_: true, elems: elems},
//     }
// }
pub fn has(t: Tile, s: Side) -> bool {
    return match t {
        Tile::SingleTrackTile{track} => track_has(track, s),
        Tile::TrackTile{toptrack, bottrack} => track_has(toptrack, s) || track_has(bottrack, s),
        Tile::EndTile{ t_:_t_, b_:_b_, l_:_l_, r_:_r_, elems: _, orig_len: _} => match s {Side::T_ => _t_, Side::B_ => _b_, Side::L_ => _l_, Side::R_ => _r_,},
        Tile::StartTile{dir, elems: _, orig_len: _} => s == dir,
        Tile::PaintTile{track, c: _} => track_has(track, s),
        Tile::EmptyTile => {panic!("Undefined Side for Emptytile {:?}", t)},
        Tile::RockTile => false,
        Tile::SplitTile{side_in} => {
            let out = split_out_sides(side_in);
            (s == out.0) || (s == out.1) || (s == out.2)
        }
    };
}
pub fn will_collide(t: & Tile, s1: Side, s2: Side) -> bool {
    match t {
        Tile::SingleTrackTile{track} => track_has(*track, s1) && track_has(*track, s2),
        Tile::TrackTile{toptrack, bottrack: _} => track_has(*toptrack, s1) && track_has(*toptrack, s2),
        _ => false,
    }
}
pub fn is_cross(t: &Tile) -> bool {
    match t {
        Tile::TrackTile{toptrack, bottrack} => 
            (track_has(*toptrack, Side::T_) && track_has(*toptrack, Side::B_) && track_has(*bottrack, Side::L_) && track_has(*bottrack, Side::R_)) 
            || (track_has(*bottrack, Side::T_) && track_has(*bottrack, Side::B_) && track_has(*toptrack, Side::L_) && track_has(*toptrack, Side::R_)),
        _ => false,
    }
}
pub fn track_out_side_from_in(in_: Side, t: Track) -> Side {
    let out_: Vec<Side> = [Side::T_, Side::B_, Side::L_, Side::R_].iter().filter(|&s| track_has(t, *s) && *s != in_).cloned().collect();
    assert!(track_has(t, in_), "{:?}, {:?}", t, in_);
    assert!(out_.len() == 1, "{:?}, {:?}, in: {:?}", t, out_, in_);
    out_[0]
}
pub fn out_side_from_in(in_: Side, t: &Tile) -> Option<Side> {
    match t {
        Tile::TrackTile{toptrack, bottrack} => {
            if track_has(*toptrack, in_) {
                Some(track_out_side_from_in(in_, *toptrack))
            } else {
                Some(track_out_side_from_in(in_, *bottrack))
            }
        },
        Tile::SingleTrackTile{track} => Some(track_out_side_from_in(in_, *track)),
        Tile::PaintTile{track, c: _} => Some(track_out_side_from_in(in_, *track)),
        _ => None,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct GridPos {
    pub x: usize, // 1 is leftmost
    pub y: usize, // 1 is topmost
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct Pos {
    pub px: usize, // 1 is leftmost
    pub py: usize, // 1 is topmost
    pub side: Side,
    pub going_in: bool, // If True, it's pointed to the inside. If false, to the outside.
    // Nobody actually uses this because you know from context, but better have it to completely specify the train's state
    pub towards_side: Option<Side>,  // This is supposed to be used ONLY WHEN THE TRAIN IS GOING IN !!!
}
// implement constructor where towards_side is None:
impl Pos {
    pub fn new(px: usize, py: usize, side: Side, going_in: bool) -> Pos {
        Pos{px: px, py: py, side: side, going_in: going_in, towards_side: None}
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)] pub struct Train {
    pub c: Colorz,
    pub pos: Pos,
}
pub fn switch_tile(pos: Pos) -> Pos {
    assert!(pos.going_in == false, "{:?}", pos);
    match pos.side {
        Side::T_ => Pos::new(pos.px, pos.py-1, Side::B_, true),
        Side::B_ => Pos::new(pos.px, pos.py+1, Side::T_, true),
        Side::L_ => Pos::new(pos.px-1, pos.py, Side::R_, true),
        Side::R_ => Pos::new(pos.px+1, pos.py, Side::L_, true),
    }
}
pub fn flip_exchange(t:  &Tile) -> Tile{
    let mut t2 = t.clone();
    t2 = match t2 {
        Tile::TrackTile{toptrack, bottrack} => {
            if is_cross(&t2) { t2 } 
            else {
                Tile::TrackTile{toptrack: bottrack, bottrack: toptrack}
            }
        },
        _ => {t2},
    };
    return t2;
}
pub enum CSide {BlueSide, RedSide}
pub fn split_out_sides_from_in(in_: Side) -> ((Side, CSide), (Side, CSide)) {
    match in_ {
        Side::B_ => ((Side::L_, CSide::BlueSide), (Side::R_, CSide::RedSide)),
        Side::T_ => ((Side::R_, CSide::BlueSide), (Side::L_, CSide::RedSide)),
        Side::L_ => ((Side::B_, CSide::RedSide), (Side::T_, CSide::BlueSide)),
        Side::R_ => ((Side::T_, CSide::RedSide), (Side::B_, CSide::BlueSide)),
    }
}
pub fn split_out_color_from_in_and_side(c: Colorz, side: CSide) -> Colorz {
    match (c, side) {
        (Colorz::RED_, CSide::BlueSide) => Colorz::RED_, (Colorz::BLUE_, CSide::BlueSide) => Colorz::BLUE_,
        (Colorz::YELLOW_, CSide::BlueSide) => Colorz::YELLOW_, (Colorz::ORANGE_, CSide::BlueSide) => Colorz::YELLOW_,
        (Colorz::GREEN_, CSide::BlueSide) => Colorz::BLUE_, (Colorz::PURPLE_, CSide::BlueSide) => Colorz::BLUE_,
        (Colorz::BROWN_, CSide::BlueSide) => Colorz::BROWN_, (Colorz::RED_, CSide::RedSide) => Colorz::RED_,
        (Colorz::BLUE_, CSide::RedSide) => Colorz::BLUE_, (Colorz::YELLOW_, CSide::RedSide) => Colorz::YELLOW_,
        (Colorz::ORANGE_, CSide::RedSide) => Colorz::RED_, (Colorz::GREEN_, CSide::RedSide) => Colorz::YELLOW_,
        (Colorz::PURPLE_, CSide::RedSide) => Colorz::RED_, (Colorz::BROWN_, CSide::RedSide) => Colorz::BROWN_,

    }
}
pub fn out_trains_from_split(t:Train) -> (Train, Train){
    // THIS USES THE ASSUMPTION THAT:
    // for a train coming FROM BOT TOWARD TOP,
    // BLUE IS ALWAYS LEFT AND RED IS ALWAYS RIGHT
    assert!(t.pos.going_in == true, "{:?}", t);
    let ((outside1, cside1), (outside2, cside2)) = split_out_sides_from_in(t.pos.side);
    let t1 = Train{c: split_out_color_from_in_and_side(t.c, cside1), pos: Pos{px: t.pos.px, py: t.pos.py, side: t.pos.side, going_in: true, towards_side: Some(outside1)}};
    let t2 = Train{c: split_out_color_from_in_and_side(t.c, cside2), pos: Pos{px: t.pos.px, py: t.pos.py, side: t.pos.side, going_in: true, towards_side: Some(outside2)}};
    (t1, t2)
}
pub fn are_colliding_border_coloring(t1:Train, t2:Train) -> bool{
    let p1 = t1.pos;
    let p2 = t2.pos;
    assert!(p1.going_in == true && p2.going_in == true, "{:?} {:?}", p1, p2);
    if p1.px == p2.px && p1.py == p2.py+1 && p1.side==Side::T_ && p2.side==Side::B_ {true} // t1 below t2, t1 going up
    else if p1.px == p2.px && p1.py+1 == p2.py && p1.side==Side::B_ && p2.side==Side::T_ {true} // t1 above t2, t1 going down
    else if p1.px == p2.px+1 && p1.py == p2.py && p1.side==Side::L_ && p2.side==Side::R_ {true} // t1 dx to t2, t1 going left
    else if p1.px+1 == p2.px && p1.py == p2.py && p1.side==Side::R_ && p2.side==Side::L_ {true} // t1 sx to t2, t1 going right
    else {false}
}
pub fn are_colliding_center_coloring(t1:Train, t2:Train, f:&Vec<Vec<Tile>>) -> bool{
    let p1 = t1.pos;
    let p2 = t2.pos;
    assert!(p1.going_in == true && p2.going_in == true, "{:?} {:?}", p1, p2);
    if p1.px == p2.px && p1.py == p2.py && p1.side != p2.side && will_collide(&f[p1.py][p1.px], p1.side, p2.side) {true}
    else {false}
}
pub fn are_colliding_center_coloring_different_tracks(t1:Train, t2:Train, f:&Vec<Vec<Tile>>) -> bool{
    let (p1, p2) = (t1.pos, t2.pos);
    assert!(p1.going_in == true && p2.going_in == true, "{:?} {:?}", p1, p2);
    if p1.px == p2.px && p1.py == p2.py && is_cross(&f[p1.py][p1.px]){
        if ((p1.side == Side::T_ || p1.side == Side::B_) && (p2.side == Side::L_ || p2.side == Side::R_)) || ((p2.side == Side::T_ || p2.side == Side::B_) && (p1.side == Side::L_ || p1.side == Side::R_)) {true} else {false}
    } else {false}
}
pub fn are_merging(t1:Train, t2:Train) -> bool{
    let (p1, p2) = (t1.pos, t2.pos);
    assert!(p1.going_in == true && p2.going_in == true, "{:?} {:?}", p1, p2);
    if p1.px == p2.px && p1.py == p2.py && p1.side == p2.side {true}
    else {false}
}
pub fn helper_make_trains(trains: &Vec<Train>, field: &Vec<Vec<Tile>>) -> (Vec<Train>, Vec<Vec<Tile>>){
    let mut trains = trains.clone();
    let mut field = field.clone();
    for x in 0..7{for y in 0..7{
        if let Tile::StartTile{elems, dir, orig_len:_} = &mut field[y][x]{
            if elems.len() > 0{
                trains.push(Train{c: elems.v[0].unwrap(), pos: Pos::new(x, y, *dir, false)});
                elems.remove(0);
            }
        }
    }}
    return (trains, field);
}
pub fn helper_check_completed(field:&Vec<Vec<Tile>>) -> bool{
    let mut completed = true;
    for row in field{
        for f in row{
            if let Tile::EndTile{t_: _, b_: _, l_: _, r_: _, elems, orig_len: _} = f{
                if elems.len() > 0{
                    completed = false;
                }
            }
        } 
    }
    completed
}
pub fn can_pass_through(t:&Tile, p:Side)-> bool{
    return match t{
        Tile::TrackTile{toptrack:_, bottrack:_} | Tile::SingleTrackTile{track:_} => {has(*t, p)},
        Tile::PaintTile{c:_, track:_} | Tile::EndTile{t_:_, b_:_, l_:_, r_:_, elems:_, orig_len: _} => {has(*t, p)},
        Tile::SplitTile{side_in} => {p == *side_in},
        Tile::StartTile{dir:_, elems:_, orig_len: _} | Tile::EmptyTile | Tile::RockTile=> {false},
    } 
}


pub fn go_to_towards_side(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){
    // For each train going_in with a towards_side, turn it into a train with going_in=False and the towards_side as side:
    let mut new_trains: Vec<Train> = Vec::new();
    for t in trains{
        assert!(t.pos.going_in == true && t.pos.towards_side.is_some(), "WRONG ORDER: {:?} should not be here", t);
        new_trains.push(Train{c: t.c, pos: Pos{px: t.pos.px, py: t.pos.py, side: t.pos.towards_side.unwrap(), going_in: false, towards_side: None}});
    }
    return (field, new_trains);
}

pub fn add_beginnings(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){

    // 1. add trains from starts
    let (new_trains, new_field) = helper_make_trains(&trains, &field);
    // println!("after 1 ntrains{:?}:", new_trains);
    return (new_field, new_trains);
}
pub fn flip_exchanges(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){
    // 2. switch all new_trains into a new tile, AND flip the exchanges in their >>OLD<< position
    let mut new_trains : Vec<Train> = Vec::new();
    let mut new_field: Vec<Vec<Tile>> = field;
    for train in trains{
        new_field[train.pos.py][train.pos.px] = flip_exchange( &new_field[train.pos.py][train.pos.px]);
        let new_train = Train{c: train.c, pos: switch_tile(train.pos)};
        new_trains.push(new_train);
    };
    return (new_field, new_trains);
}
// # >> at which point in the loop you compute Collisions?
// # BORDER COLORING: it's exactly the same
// # CENTER COLORING: here or after follow the track, since you know the Tile and that's good enough.
// #           >> IT WOULD BE DIFFERENT IF YOU HAVE TO ANIMATE IT, then it would be somehow "inside" out_side_from_in()
// # CENTER COLORING DIFFERENT TRACKS: same as CENTER COLORING
// # MERGING: k THIS IS IMPORTANT: you have to do it >>AFTER<< YOU SWITCH YOUR TILE, SO THAT,
// # THE >previous< TILE IS FLIPPED TWICE==unchanged, WHICH IS THE BEHAVIOUR IN THE GAME.
// # ^But, Before you follow the track, and especially BEFORE YOU CHECK FOR ENDTILES, cuz merged trains Can arrive to endtiles.
// # So, HERE>>. Yes.

pub fn check_merges(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){
    let mut merged_trains : Vec<usize> = Vec::new();
    let mut new_trains: Vec<Train> = Vec::new();
    for (i, t1) in trains.iter().enumerate(){
        if merged_trains.contains(&i){ continue; }
        let mut col = t1.c;
        for (j, t2) in trains[i+1..].iter().enumerate(){
            // if (i+j+1) is in merged_trains continue:
            if are_merging(*t1, *t2){
                // // println!("\n>>Merging! {:?} {:?}", t1, t2);
                col = mix_colors(t1.c, t2.c);
                // println!("MERGED ???????");
                merged_trains.push(i+j+1);  // We will push t1 with col, and NOT push j when his turn will come
                break;
            }
        }
        new_trains.push(Train{c: col, pos: t1.pos});
    }
    // println!("after 3 trains{:?}:", new_trains);
    return (field, new_trains);
}

pub fn check_border_collisions(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){

    // I claim that whever or not this is before check_arrived_or_crashed is ONLY relevant for 2 trains that meet at the exit of a SplitTile,
    // And besides being completely pointles, this is PROBABLY ALSO CORRECT.
    // 3. Collisions
    let new_field = field;
    let mut new_trains : Vec<Train> = trains.clone();
    for (i, t1) in trains.iter().enumerate(){
        for (j, t2) in new_trains[i+1..].iter().enumerate(){
            if are_colliding_border_coloring(*t1, *t2){
                // // println!("\n>>Border colliding! {:?} {:?}", t1, t2);
                let newcol = mix_colors(t1.c, t2.c);
                new_trains[i].c = newcol;
                new_trains[i+j+1].c = newcol;
                break;
            }
        }
    }
    // println!("after 3.5 trains{:?}:", new_trains);
    return (new_field, new_trains);

}

//     # 4. For each train, change their >SIDE< only following track
pub fn check_arrived_or_crashed(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (bool, bool, Vec<Vec<Tile>>, Vec<Train>){

    let mut crashed = false;
    let mut new_trains: Vec<Train> = Vec::new();
    let mut new_field: Vec<Vec<Tile>> = field.clone();
    for (_, train) in trains.iter().enumerate(){
        let tile = &new_field[train.pos.py][train.pos.px];
        if !can_pass_through(tile, train.pos.side) {crashed=true; continue;}
        if let Tile::EndTile{elems, t_:_t_, b_:_b_, l_:_l_, r_:_r_, orig_len: orig_len} = tile {
            if elems.v.contains(&Some(train.c)){
                let mut newelems = elems.clone();
                // Remove the FIRST instance of train.c in elems:
                newelems.remove(elems.iter().position(|x| x == train.c).unwrap());
                new_field[train.pos.py][train.pos.px] = Tile::EndTile{t_: *_t_, b_: *_b_, l_: *_l_, r_: *_r_, elems: newelems, orig_len: *orig_len};
            }
            else{
                crashed = true;
            }
        }
        else{
            new_trains.push(*train);
        };
    };
    let completed = helper_check_completed(&new_field);

    // // println!(">>>> {:?}", print_tile(&new_field[3][5]));
    // println!("after 4 trains{:?}; Crashed: {:?}, Completed: {:?}", new_trains, crashed, completed);
    return (crashed, completed, new_field, new_trains);
}



pub fn set_towards_side(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){
    let mut new_trains: Vec<Train> = Vec::new();
    for train in trains.iter(){
        let tile = &field[train.pos.py][train.pos.px];
        match tile{
            Tile::TrackTile{toptrack:_, bottrack:_} | Tile::SingleTrackTile{track:_} => {
                new_trains.push(Train{c: train.c, pos: Pos{px: train.pos.px, py: train.pos.py, side: train.pos.side, going_in: true, towards_side: Some(out_side_from_in(train.pos.side, tile).unwrap())}});
            },
            Tile::PaintTile{c, track:_} => {
                new_trains.push(Train{c: train.c, pos: Pos{px: train.pos.px, py: train.pos.py, side: train.pos.side, going_in: true, towards_side: Some(out_side_from_in(train.pos.side, tile).unwrap())}});
            },
            Tile::SplitTile{side_in:_} => {
                new_trains.push(Train{c: train.c, pos: Pos{px: train.pos.px, py: train.pos.py, side: train.pos.side, going_in: true, towards_side: Some(flip_side(train.pos.side))}});
            },
            _ => panic!("How can you ever get to this path !!! {:?} {:?}", train, tile),
        }
    }
    return (field, new_trains);
}




pub fn check_center_colliding(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){
    let mut new_trains = trains.clone();
    for (i, t1) in trains.iter().enumerate(){
        for (j, t2) in trains[i+1..].iter().enumerate(){
            if are_colliding_center_coloring(*t1, *t2, &field) || are_colliding_center_coloring_different_tracks(*t1, *t2, &field){
                // // println!("\n>>Center colliding! {:?} {:?}", t1, t2);
                let newcol = mix_colors(t1.c, t2.c);
                new_trains[i].c = newcol;
                new_trains[i+j+1].c = newcol;
                break;
            }
        }
    }
    return (field, new_trains);
}

pub fn do_center_coloring_things(trains: Vec<Train>, field: Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, Vec<Train>){
    let mut new_trains: Vec<Train> = Vec::new();
    for train in trains.iter(){
        let tile = &field[train.pos.py][train.pos.px];
        match tile{
            Tile::PaintTile{c, track:_} => {
                new_trains.push(Train{c: *c, pos: train.pos});
            },
            Tile::SplitTile{side_in:_} => {
                let (t1, t2) = out_trains_from_split(*train);
                new_trains.push(t1); new_trains.push(t2);
            },
            _ => { new_trains.push(*train); },
        }
    }
    // println!("Triggering coloring now!!{:?}:", new_trains);
    return (field, new_trains);
}




pub fn run_level(field:  Vec<Vec<Tile>>, stop_at_crash: bool, deepcopy_: bool, max_run: usize) -> (bool, usize){
    let mut field = field.clone();
    let mut crashed: bool;
    let mut completed: bool;
    let mut won = false;
    let mut trains: Vec<Train> = Vec::new();
    let mut t = 0;

    while t<max_run{
        t+=1;
        println!("\n TRAINS GOING IN: {:?}:", trains);
        (field, trains) = check_center_colliding(trains, field);
        (field, trains) = do_center_coloring_things(trains, field);
        
        (field, trains) = go_to_towards_side(trains, field);
        (field, trains) = add_beginnings(trains, field);
        (field, trains) = flip_exchanges(trains, field);
        (field, trains) = check_merges(trains, field);
        (field, trains) = check_border_collisions(trains, field);
        (crashed, completed, field, trains) = check_arrived_or_crashed(trains, field);
        (field, trains) = set_towards_side(trains, field);
        
        won = completed && !crashed && trains.len() == 0;

        // println!("\n{:?}\ntrains:{:?}\n{:?}\ncrashed: {:?}, completed: {:?}, won: {:?}", t, trains, field, crashed, completed, won);

        if won {break;}
        if crashed && stop_at_crash {break;}
    }
    return (won, t);
}


// #############################################################################



pub fn to_index_side(s: Side) -> usize {match s{Side::T_ => 1, Side::B_ => 2, Side::L_ => 3, Side::R_ => 4}}
pub fn to_index_trackoptions(s: TrackOptions) -> usize {match s{TrackOptions::TL => 1, TrackOptions::TB => 2, TrackOptions::TR => 3, TrackOptions::LB => 4, TrackOptions::LR => 5, TrackOptions::BR => 6}}
pub fn from_idx_str_trackoptions(s: char) -> TrackOptions {match s {'1' => TrackOptions::TL, '2' => TrackOptions::TB, '3' => TrackOptions::TR, '4' => TrackOptions::LB, '5' => TrackOptions::LR, '6' => TrackOptions::BR, _ => panic!("from_index_TrackOptions")}}
pub fn c_str_dict (c: Colorz) -> String{match c{ Colorz::BLUE_ => "b".to_string(), Colorz::RED_ => "r".to_string(), Colorz::YELLOW_ => "y".to_string(), Colorz::GREEN_ => "g".to_string(), Colorz::ORANGE_ => "o".to_string(), Colorz::PURPLE_ => "p".to_string(), Colorz::BROWN_ => "z".to_string(),}}
pub fn reverse_c_str_dict (s: char) -> Colorz{match s{ 'b' => Colorz::BLUE_, 'r' => Colorz::RED_, 'y' => Colorz::YELLOW_, 'g' => Colorz::GREEN_, 'o' => Colorz::ORANGE_, 'p' => Colorz::PURPLE_, 'z' => Colorz::BROWN_, _ => panic!("Invalid color string! {:?}", s),}}
pub fn str_of_dirs(t_: bool, b_: bool, l_: bool, r_: bool) -> String{return format!("{}{}{}{}", t_ as usize, b_ as usize, l_ as usize, r_ as usize);}
pub fn dirs_of_str(str: &str) -> (bool, bool, bool, bool){return (str.chars().nth(0).unwrap() == '1', str.chars().nth(1).unwrap() == '1', str.chars().nth(2).unwrap() == '1', str.chars().nth(3).unwrap() == '1');}
pub fn dir_from_string(s: &str) -> Side{match s{ "t" => Side::T_, "b" => Side::B_, "l" => Side::L_, "r" => Side::R_, _ => panic!("Invalid direction string!"),}}
pub fn string_from_dir(dir: Side) -> String{match dir{ Side::T_ => "t_".to_string(), Side::B_ => "b_".to_string(), Side::L_ => "l_".to_string(), Side::R_ => "r_".to_string(),}}
pub fn pos_to_int(s:String, pos:usize)->i32{ let c: char = s.chars().nth(pos).unwrap(); let i: i32 = c.to_digit(10).unwrap() as i32; return i; }
pub fn pos_to_trackoption(s: String, pos: usize) -> TrackOptions{ let c: char = s.chars().nth(pos).unwrap(); return from_idx_str_trackoptions(c); }
pub fn pos_to_side(s: String, pos: usize) -> Side{ let c: char = s.chars().nth(pos).unwrap(); return match c { '1' => Side::T_, '2' => Side::B_, '3' => Side::L_, '4' => Side::R_, _ => panic!("Invalid direction int!"),}; }
pub fn mk_end_tile((t_, b_, l_, r_): (bool, bool, bool, bool), colors: VectorOfColorz) -> Tile{ return Tile::new_end_tile(t_, b_, l_, r_, colors); }
pub fn colorz_to_long_str(color: Colorz) -> String { match color { Colorz::RED_ => "red".to_string(), Colorz::BLUE_ => "blue".to_string(), Colorz::GREEN_ => "green".to_string(), Colorz::YELLOW_ => "yellow".to_string(), Colorz::ORANGE_ => "orange".to_string(), Colorz::PURPLE_ => "purple".to_string(), Colorz::BROWN_ => "brown".to_string(), } }


fn colors_from_string(s: &str) -> VectorOfColorz{
    // Divide in chars and use reverse_c_str_dict on each:
    let mut colors: VectorOfColorz = VectorOfColorz{v: [None; 12]};
    for c in s.chars(){
        colors.push(reverse_c_str_dict(c));
    }
    return colors;
}


pub fn parse_tile(t: &str) -> Tile{
    if t == "00" {return Tile::EmptyTile;}
    else if t == "MM" {return Tile::RockTile;}
    else if t.chars().nth(0).unwrap() == 'E' {return mk_end_tile(dirs_of_str(&t[1..5]), colors_from_string(&t[6..t.len()]));}
    else if t.chars().nth(0).unwrap() == 'S' {return Tile::new_start_tile(dir_from_string(&t[1..2]), colors_from_string(&t[3..t.len()]));}
    else if t.chars().nth(0).unwrap() == 'D' {return Tile::SplitTile{side_in: pos_to_side(t.to_string(), 1)};}
    else if t.chars().nth(0).unwrap() == 'b' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::BLUE_};}
    else if t.chars().nth(0).unwrap() == 'r' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::RED_};}
    else if t.chars().nth(0).unwrap() == 'y' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::YELLOW_};}
    else if t.chars().nth(0).unwrap() == 'g' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::GREEN_};}
    else if t.chars().nth(0).unwrap() == 'o' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::ORANGE_};}
    else if t.chars().nth(0).unwrap() == 'p' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::PURPLE_};}
    else if t.chars().nth(0).unwrap() == 'z' {return Tile::PaintTile{track: get_track(pos_to_trackoption(t.to_string(), 1)), c: Colorz::BROWN_};}
    else if t.chars().nth(0).unwrap() == '0' {return Tile::SingleTrackTile{track: get_track(pos_to_trackoption(t.to_string(), 1))};}
    else {return Tile::TrackTile{toptrack: get_track(pos_to_trackoption(t.to_string(), 0)), bottrack: get_track(pos_to_trackoption(t.to_string(), 1))};}
}

pub fn parse_map(map_: Vec<String>) -> Vec<Vec<Tile>>{
    let mut m: Vec<Vec<Tile>> = Vec::new();
    for row in map_{
        let mut row_vec: Vec<Tile> = Vec::new();
        for s in row.split(" "){
            // If the string is empty, skip it:
            if s != "" {
                row_vec.push(parse_tile(s));
            }
        }
        m.push(row_vec);
    }
    // Assert that this has size 7x7:
    assert!(m.len() == 7);
    for row in &m{
        assert!(row.len() == 7);
    }
    return m;
}

pub fn print_tile(t: &Tile) -> String{
    match t{
        Tile::EmptyTile => return "00".to_string(),
        Tile::RockTile => return "MM".to_string(),
        Tile::StartTile{dir, elems, orig_len:_} => return format!("S{}{}", string_from_dir(*dir), elems.iter().map(|x| c_str_dict(x)).collect::<String>()),
        Tile::EndTile{t_, b_, l_, r_, elems, orig_len:_} => return format!("E{}_{}", str_of_dirs(*t_, *b_, *l_, *r_), elems.iter().map(|x| c_str_dict(x)).collect::<String>()),
        Tile::SplitTile{side_in} => return format!("D{}", to_index_side(*side_in)),
        Tile::PaintTile{track, c} => return format!("{}{}", c_str_dict(*c), to_index_trackoptions(get_track_option(*track))),
        Tile::SingleTrackTile{track} => return format!("{}{}", "0", to_index_trackoptions(get_track_option(*track))),
        Tile::TrackTile{toptrack, bottrack} => return format!("{}{}", to_index_trackoptions(get_track_option(*toptrack)), to_index_trackoptions(get_track_option(*bottrack))),
    }
}

pub fn print_map(map_: &Vec<Vec<Tile>>) -> Vec<String>{
    let mut map_str: Vec<String> = Vec::new();
    for i in 0..7{
        let mut row_str: String = String::new();
        for j in 0..7{
            row_str.push_str(&print_tile(&map_[i][j]));
            row_str.push_str(" ");
        }
        map_str.push(row_str);
    }
    return map_str;
}

pub fn pretty_print_map(map_: &Vec<Vec<Tile>>) -> String{
    let reprmap = print_map(map_);
    let res = reprmap.join("\n");
    return res;
    // println!("{}", res);
}

pub fn count_tracks(map: &Vec<Vec<Tile>>) -> i32 {
    let mut count = 0;
    for row in map {
        for tile in row {
            match tile {
                Tile::TrackTile{toptrack: _, bottrack: _} => count += 1,
                Tile::SingleTrackTile{track: _} => count += 1,
                _ => (),
            }
        }
    }
    count
}

pub fn count_double_tracks(map: &Vec<Vec<Tile>>) -> i32 {
    let mut count = 0;
    for row in map {
        for tile in row {
            match tile {
                Tile::TrackTile{toptrack: _, bottrack: _} => count += 1,
                _ => (),
            }
        }
    }
    count
}