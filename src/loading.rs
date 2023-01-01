use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

// use crate::all_puzzles_clean::PuzzlesData;
pub struct LoadingPlugin;



/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                // .with_collection::<AudioAssets>()
                .with_collection::<TrainAssets>()
                .with_collection::<TileAssets>()
                .continue_to_state(GameState::MenuTitle),
            )
        ;
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

// Assets for the trains. Must be used as a resource.
#[derive(Debug, Clone, Default, AssetCollection, Resource)]
pub struct TrainAssets {
    #[asset(path = "samples/s_elem_1_blue.png")] pub train_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_1_red.png")] pub train_red: Handle<Image>,
    #[asset(path = "samples/s_elem_1_yellow.png")] pub train_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_1_orange.png")] pub train_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_1_green.png")] pub train_green: Handle<Image>,
    #[asset(path = "samples/s_elem_1_purple.png")] pub train_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_1_brown.png")] pub train_brown: Handle<Image>,
}



#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "samples/s_elem_1_blue.png")] pub s_elem_1_blue: Handle<Image>,
    #[asset(path = "samples/s_base.png")] pub s_base: Handle<Image>,
    #[asset(path = "samples/e_elem_9_yellow.png")] pub e_elem_9_yellow: Handle<Image>,
    #[asset(path = "samples/e_elem_9_blue.png")] pub e_elem_9_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_4_orange.png")] pub s_elem_4_orange: Handle<Image>,
    #[asset(path = "samples/e_elem_4_green.png")] pub e_elem_4_green: Handle<Image>,
    #[asset(path = "samples/e_elem_4_yellow.png")] pub e_elem_4_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_9_orange.png")] pub s_elem_9_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_1_red.png")] pub s_elem_1_red: Handle<Image>,
    #[asset(path = "samples/e_elem_9_green.png")] pub e_elem_9_green: Handle<Image>,
    #[asset(path = "samples/e_elem_1_yellow.png")] pub e_elem_1_yellow: Handle<Image>,
    #[asset(path = "samples/p_outer_str_lr.png")] pub p_outer_str_lr: Handle<Image>,
    #[asset(path = "samples/tb.png")] pub tb: Handle<Image>,
    #[asset(path = "samples/lr_over_tb.png")] pub lr_over_tb: Handle<Image>,
    #[asset(path = "samples/p_green.png")] pub p_green: Handle<Image>,
    #[asset(path = "samples/p_red.png")] pub p_red: Handle<Image>,
    #[asset(path = "samples/s_elem_1_orange.png")] pub s_elem_1_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_9_red.png")] pub s_elem_9_red: Handle<Image>,
    #[asset(path = "samples/p_outer_cur_br.png")] pub p_outer_cur_br: Handle<Image>,
    #[asset(path = "samples/empty.png")] pub empty: Handle<Image>,
    #[asset(path = "samples/s_elem_1_purple.png")] pub s_elem_1_purple: Handle<Image>,
    #[asset(path = "samples/track_funny_tr_bl.png")] pub track_funny_tr_bl: Handle<Image>,
    #[asset(path = "samples/p_blue.png")] pub p_blue: Handle<Image>,
    #[asset(path = "samples/e_elem_4_brown.png")] pub e_elem_4_brown: Handle<Image>,
    #[asset(path = "samples/e_funnel_elem_rigth.png")] pub e_funnel_elem_rigth: Handle<Image>,
    #[asset(path = "samples/e_elem_9_brown.png")] pub e_elem_9_brown: Handle<Image>,
    #[asset(path = "samples/e_elem_4_red.png")] pub e_elem_4_red: Handle<Image>,
    #[asset(path = "samples/s_elem_9_purple.png")] pub s_elem_9_purple: Handle<Image>,
    #[asset(path = "samples/scissor_u.png")] pub scissor_u: Handle<Image>,
    #[asset(path = "samples/rock.png")] pub rock: Handle<Image>,
    #[asset(path = "samples/p_yellow.png")] pub p_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_1_green.png")] pub s_elem_1_green: Handle<Image>,
    #[asset(path = "samples/e_elem_4_blue.png")] pub e_elem_4_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_4_purple.png")] pub s_elem_4_purple: Handle<Image>,
    #[asset(path = "samples/s_elem_9_green.png")] pub s_elem_9_green: Handle<Image>,
    #[asset(path = "samples/br_over_tb.png")] pub br_over_tb: Handle<Image>,
    #[asset(path = "samples/s_elem_4_green.png")] pub s_elem_4_green: Handle<Image>,
    #[asset(path = "samples/s_arrow_elem_rigth.png")] pub s_arrow_elem_rigth: Handle<Image>,
    #[asset(path = "samples/e_base.png")] pub e_base: Handle<Image>,
    #[asset(path = "samples/e_elem_1_red.png")] pub e_elem_1_red: Handle<Image>,
    #[asset(path = "samples/e_elem_1_purple.png")] pub e_elem_1_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_1_blue.png")] pub e_elem_1_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_9_blue.png")] pub s_elem_9_blue: Handle<Image>,
    #[asset(path = "samples/tb_over_br.png")] pub tb_over_br: Handle<Image>,
    #[asset(path = "samples/e_elem_1_brown.png")] pub e_elem_1_brown: Handle<Image>,
    #[asset(path = "samples/e_elem_4_purple.png")] pub e_elem_4_purple: Handle<Image>,
    #[asset(path = "samples/p_orange.png")] pub p_orange: Handle<Image>,
    #[asset(path = "samples/e_elem_9_red.png")] pub e_elem_9_red: Handle<Image>,
    #[asset(path = "samples/e_elem_9_purple.png")] pub e_elem_9_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_9_orange.png")] pub e_elem_9_orange: Handle<Image>,
    #[asset(path = "samples/tr_over_tl.png")] pub tr_over_tl: Handle<Image>,
    #[asset(path = "samples/s_elem_4_yellow.png")] pub s_elem_4_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_4_blue.png")] pub s_elem_4_blue: Handle<Image>,
    #[asset(path = "samples/p_purple.png")] pub p_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_4_orange.png")] pub e_elem_4_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_4_brown.png")] pub s_elem_4_brown: Handle<Image>,
    #[asset(path = "samples/s_elem_9_yellow.png")] pub s_elem_9_yellow: Handle<Image>,
    #[asset(path = "samples/e_elem_1_orange.png")] pub e_elem_1_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_4_red.png")] pub s_elem_4_red: Handle<Image>,
    #[asset(path = "samples/br.png")] pub br: Handle<Image>,
    #[asset(path = "samples/e_elem_1_green.png")] pub e_elem_1_green: Handle<Image>,
    #[asset(path = "samples/s_elem_1_yellow.png")] pub s_elem_1_yellow: Handle<Image>,
}


pub fn get_asset(name: String, asset_server: &TileAssets)-> Handle<Image> {
    
    // Match name over the 2 possible string "s_elem_1_blue" and "s_base" :
    match &name[..] {
       "s_elem_1_blue.png" => {asset_server.s_elem_1_blue.clone()},
        "s_base.png" => {asset_server.s_base.clone()},
        "e_elem_9_yellow.png" => {asset_server.e_elem_9_yellow.clone()},
        "e_elem_9_blue.png" => {asset_server.e_elem_9_blue.clone()},
        "s_elem_4_orange.png" => {asset_server.s_elem_4_orange.clone()},
        "e_elem_4_green.png" => {asset_server.e_elem_4_green.clone()},
        "e_elem_4_yellow.png" => {asset_server.e_elem_4_yellow.clone()},
        "s_elem_9_orange.png" => {asset_server.s_elem_9_orange.clone()},
        "s_elem_1_red.png" => {asset_server.s_elem_1_red.clone()},
        "e_elem_9_green.png" => {asset_server.e_elem_9_green.clone()},
        "e_elem_1_yellow.png" => {asset_server.e_elem_1_yellow.clone()},
        "p_outer_str_lr.png" => {asset_server.p_outer_str_lr.clone()},
        "tb.png" => {asset_server.tb.clone()},
        "lr_over_tb.png" => {asset_server.lr_over_tb.clone()},
        "p_green.png" => {asset_server.p_green.clone()},
        "p_red.png" => {asset_server.p_red.clone()},
        "s_elem_1_orange.png" => {asset_server.s_elem_1_orange.clone()},
        "s_elem_9_red.png" => {asset_server.s_elem_9_red.clone()},
        "p_outer_cur_br.png" => {asset_server.p_outer_cur_br.clone()},
        "empty.png" => {asset_server.empty.clone()},
        "s_elem_1_purple.png" => {asset_server.s_elem_1_purple.clone()},
        "track_funny_tr_bl.png" => {asset_server.track_funny_tr_bl.clone()},
        "p_blue.png" => {asset_server.p_blue.clone()},
        "e_elem_4_brown.png" => {asset_server.e_elem_4_brown.clone()},
        "e_funnel_elem_rigth.png" => {asset_server.e_funnel_elem_rigth.clone()},
        "e_elem_9_brown.png" => {asset_server.e_elem_9_brown.clone()},
        "e_elem_4_red.png" => {asset_server.e_elem_4_red.clone()},
        "s_elem_9_purple.png" => {asset_server.s_elem_9_purple.clone()},
        "scissor_u.png" => {asset_server.scissor_u.clone()},
        "rock.png" => {asset_server.rock.clone()},
        "p_yellow.png" => {asset_server.p_yellow.clone()},
        "s_elem_1_green.png" => {asset_server.s_elem_1_green.clone()},
        "e_elem_4_blue.png" => {asset_server.e_elem_4_blue.clone()},
        "s_elem_4_purple.png" => {asset_server.s_elem_4_purple.clone()},
        "s_elem_9_green.png" => {asset_server.s_elem_9_green.clone()},
        "br_over_tb.png" => {asset_server.br_over_tb.clone()},
        "s_elem_4_green.png" => {asset_server.s_elem_4_green.clone()},
        "s_arrow_elem_rigth.png" => {asset_server.s_arrow_elem_rigth.clone()},
        "e_base.png" => {asset_server.e_base.clone()},
        "e_elem_1_red.png" => {asset_server.e_elem_1_red.clone()},
        "e_elem_1_purple.png" => {asset_server.e_elem_1_purple.clone()},
        "e_elem_1_blue.png" => {asset_server.e_elem_1_blue.clone()},
        "s_elem_9_blue.png" => {asset_server.s_elem_9_blue.clone()},
        "tb_over_br.png" => {asset_server.tb_over_br.clone()},
        "e_elem_1_brown.png" => {asset_server.e_elem_1_brown.clone()},
        "e_elem_4_purple.png" => {asset_server.e_elem_4_purple.clone()},
        "p_orange.png" => {asset_server.p_orange.clone()},
        "e_elem_9_red.png" => {asset_server.e_elem_9_red.clone()},
        "e_elem_9_purple.png" => {asset_server.e_elem_9_purple.clone()},
        "e_elem_9_orange.png" => {asset_server.e_elem_9_orange.clone()},
        "tr_over_tl.png" => {asset_server.tr_over_tl.clone()},
        "s_elem_4_yellow.png" => {asset_server.s_elem_4_yellow.clone()},
        "s_elem_4_blue.png" => {asset_server.s_elem_4_blue.clone()},
        "p_purple.png" => {asset_server.p_purple.clone()},
        "e_elem_4_orange.png" => {asset_server.e_elem_4_orange.clone()},
        "s_elem_4_brown.png" => {asset_server.s_elem_4_brown.clone()},
        "s_elem_9_yellow.png" => {asset_server.s_elem_9_yellow.clone()},
        "e_elem_1_orange.png" => {asset_server.e_elem_1_orange.clone()},
        "s_elem_4_red.png" => {asset_server.s_elem_4_red.clone()},
        "br.png" => {asset_server.br.clone()},
        "e_elem_1_green.png" => {asset_server.e_elem_1_green.clone()},
        "s_elem_1_yellow.png" => {asset_server.s_elem_1_yellow.clone()},
        // Otherwise panick:
        _ => panic!("Unknown asset: {}", name)
    }
}