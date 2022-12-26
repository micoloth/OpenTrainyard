use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;



/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
            )
        .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_static_resources),)
        ;
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "samples/s_elem_1_blue.png")]
    pub s_elem_1_blue: Handle<Image>,
    #[asset(path = "samples/s_base.png")]
    pub s_base: Handle<Image>,
    #[asset(path = "samples/e_elem_9_yellow.png")]
    pub e_elem_9_yellow: Handle<Image>,
    #[asset(path = "samples/e_elem_9_blue.png")]
    pub e_elem_9_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_4_orange.png")]
    pub s_elem_4_orange: Handle<Image>,
    #[asset(path = "samples/e_elem_4_green.png")]
    pub e_elem_4_green: Handle<Image>,
    #[asset(path = "samples/e_elem_4_yellow.png")]
    pub e_elem_4_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_9_orange.png")]
    pub s_elem_9_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_1_red.png")]
    pub s_elem_1_red: Handle<Image>,
    #[asset(path = "samples/e_elem_9_green.png")]
    pub e_elem_9_green: Handle<Image>,
    #[asset(path = "samples/e_elem_1_yellow.png")]
    pub e_elem_1_yellow: Handle<Image>,
    #[asset(path = "samples/p_outer_str_lr.png")]
    pub p_outer_str_lr: Handle<Image>,
    #[asset(path = "samples/tb.png")]
    pub tb: Handle<Image>,
    #[asset(path = "samples/lr_over_tb.png")]
    pub lr_over_tb: Handle<Image>,
    #[asset(path = "samples/p_green.png")]
    pub p_green: Handle<Image>,
    #[asset(path = "samples/p_red.png")]
    pub p_red: Handle<Image>,
    #[asset(path = "samples/s_elem_1_orange.png")]
    pub s_elem_1_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_9_red.png")]
    pub s_elem_9_red: Handle<Image>,
    #[asset(path = "samples/p_outer_cur_br.png")]
    pub p_outer_cur_br: Handle<Image>,
    #[asset(path = "samples/empty.png")]
    pub empty: Handle<Image>,
    #[asset(path = "samples/s_elem_1_purple.png")]
    pub s_elem_1_purple: Handle<Image>,
    #[asset(path = "samples/track_funny_tr_bl.png")]
    pub track_funny_tr_bl: Handle<Image>,
    #[asset(path = "samples/p_blue.png")]
    pub p_blue: Handle<Image>,
    #[asset(path = "samples/e_elem_4_brown.png")]
    pub e_elem_4_brown: Handle<Image>,
    #[asset(path = "samples/e_funnel_elem_rigth.png")]
    pub e_funnel_elem_rigth: Handle<Image>,
    #[asset(path = "samples/e_elem_9_brown.png")]
    pub e_elem_9_brown: Handle<Image>,
    #[asset(path = "samples/e_elem_4_red.png")]
    pub e_elem_4_red: Handle<Image>,
    #[asset(path = "samples/s_elem_9_purple.png")]
    pub s_elem_9_purple: Handle<Image>,
    #[asset(path = "samples/scissor_u.png")]
    pub scissor_u: Handle<Image>,
    #[asset(path = "samples/rock.png")]
    pub rock: Handle<Image>,
    #[asset(path = "samples/p_yellow.png")]
    pub p_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_1_green.png")]
    pub s_elem_1_green: Handle<Image>,
    #[asset(path = "samples/e_elem_4_blue.png")]
    pub e_elem_4_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_4_purple.png")]
    pub s_elem_4_purple: Handle<Image>,
    #[asset(path = "samples/s_elem_9_green.png")]
    pub s_elem_9_green: Handle<Image>,
    #[asset(path = "samples/br_over_tb.png")]
    pub br_over_tb: Handle<Image>,
    #[asset(path = "samples/s_elem_4_green.png")]
    pub s_elem_4_green: Handle<Image>,
    #[asset(path = "samples/s_arrow_elem_rigth.png")]
    pub s_arrow_elem_rigth: Handle<Image>,
    #[asset(path = "samples/e_base.png")]
    pub e_base: Handle<Image>,
    #[asset(path = "samples/e_elem_1_red.png")]
    pub e_elem_1_red: Handle<Image>,
    #[asset(path = "samples/e_elem_1_purple.png")]
    pub e_elem_1_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_1_blue.png")]
    pub e_elem_1_blue: Handle<Image>,
    #[asset(path = "samples/s_elem_9_blue.png")]
    pub s_elem_9_blue: Handle<Image>,
    #[asset(path = "samples/tb_over_br.png")]
    pub tb_over_br: Handle<Image>,
    #[asset(path = "samples/e_elem_1_brown.png")]
    pub e_elem_1_brown: Handle<Image>,
    #[asset(path = "samples/e_elem_4_purple.png")]
    pub e_elem_4_purple: Handle<Image>,
    #[asset(path = "samples/p_orange.png")]
    pub p_orange: Handle<Image>,
    #[asset(path = "samples/e_elem_9_red.png")]
    pub e_elem_9_red: Handle<Image>,
    #[asset(path = "samples/e_elem_9_purple.png")]
    pub e_elem_9_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_9_orange.png")]
    pub e_elem_9_orange: Handle<Image>,
    #[asset(path = "samples/tr_over_tl.png")]
    pub tr_over_tl: Handle<Image>,
    #[asset(path = "samples/s_elem_4_yellow.png")]
    pub s_elem_4_yellow: Handle<Image>,
    #[asset(path = "samples/s_elem_4_blue.png")]
    pub s_elem_4_blue: Handle<Image>,
    #[asset(path = "samples/p_purple.png")]
    pub p_purple: Handle<Image>,
    #[asset(path = "samples/e_elem_4_orange.png")]
    pub e_elem_4_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_4_brown.png")]
    pub s_elem_4_brown: Handle<Image>,
    #[asset(path = "samples/s_elem_9_yellow.png")]
    pub s_elem_9_yellow: Handle<Image>,
    #[asset(path = "samples/e_elem_1_orange.png")]
    pub e_elem_1_orange: Handle<Image>,
    #[asset(path = "samples/s_elem_4_red.png")]
    pub s_elem_4_red: Handle<Image>,
    #[asset(path = "samples/br.png")]
    pub br: Handle<Image>,
    #[asset(path = "samples/e_elem_1_green.png")]
    pub e_elem_1_green: Handle<Image>,
    #[asset(path = "samples/s_elem_1_yellow.png")]
    pub s_elem_1_yellow: Handle<Image>,
}

use std::collections::HashMap;


// Assets for the board. Must be used as a resource.
//
// Use the loader for partial setup
#[derive(Debug, Clone)]
pub struct BoardAssetsMap {
    pub assets: HashMap<String, Handle<Image>>,
}


fn setup_static_resources(mut commands: Commands, asset_server: Res<TextureAssets>, ) {
    let assets = get_assets_in_a_hashmap(&asset_server);
    commands.insert_resource(assets);
}

pub fn get_assets_in_a_hashmap(asset_server: &Res<TextureAssets>)-> BoardAssetsMap {
    
    let mut assets: HashMap<String, Handle<Image>> = HashMap::new();
    assets.insert("s_elem_1_blue.png".to_string(), asset_server.s_elem_1_blue.clone());
    assets.insert("s_base.png".to_string(), asset_server.s_base.clone());
    assets.insert("e_elem_9_yellow.png".to_string(), asset_server.e_elem_9_yellow.clone());
    assets.insert("e_elem_9_blue.png".to_string(), asset_server.e_elem_9_blue.clone());
    assets.insert("s_elem_4_orange.png".to_string(), asset_server.s_elem_4_orange.clone());
    assets.insert("e_elem_4_green.png".to_string(), asset_server.e_elem_4_green.clone());
    assets.insert("e_elem_4_yellow.png".to_string(), asset_server.e_elem_4_yellow.clone());
    assets.insert("s_elem_9_orange.png".to_string(), asset_server.s_elem_9_orange.clone());
    assets.insert("s_elem_1_red.png".to_string(), asset_server.s_elem_1_red.clone());
    assets.insert("e_elem_9_green.png".to_string(), asset_server.e_elem_9_green.clone());
    assets.insert("e_elem_1_yellow.png".to_string(), asset_server.e_elem_1_yellow.clone());
    assets.insert("p_outer_str_lr.png".to_string(), asset_server.p_outer_str_lr.clone());
    assets.insert("tb.png".to_string(), asset_server.tb.clone());
    assets.insert("lr_over_tb.png".to_string(), asset_server.lr_over_tb.clone());
    assets.insert("p_green.png".to_string(), asset_server.p_green.clone());
    assets.insert("p_red.png".to_string(), asset_server.p_red.clone());
    assets.insert("s_elem_1_orange.png".to_string(), asset_server.s_elem_1_orange.clone());
    assets.insert("s_elem_9_red.png".to_string(), asset_server.s_elem_9_red.clone());
    assets.insert("p_outer_cur_br.png".to_string(), asset_server.p_outer_cur_br.clone());
    assets.insert("empty.png".to_string(), asset_server.empty.clone());
    assets.insert("s_elem_1_purple.png".to_string(), asset_server.s_elem_1_purple.clone());
    assets.insert("track_funny_tr_bl.png".to_string(), asset_server.track_funny_tr_bl.clone());
    assets.insert("p_blue.png".to_string(), asset_server.p_blue.clone());
    assets.insert("e_elem_4_brown.png".to_string(), asset_server.e_elem_4_brown.clone());
    assets.insert("e_funnel_elem_rigth.png".to_string(), asset_server.e_funnel_elem_rigth.clone());
    assets.insert("e_elem_9_brown.png".to_string(), asset_server.e_elem_9_brown.clone());
    assets.insert("e_elem_4_red.png".to_string(), asset_server.e_elem_4_red.clone());
    assets.insert("s_elem_9_purple.png".to_string(), asset_server.s_elem_9_purple.clone());
    assets.insert("scissor_u.png".to_string(), asset_server.scissor_u.clone());
    assets.insert("rock.png".to_string(), asset_server.rock.clone());
    assets.insert("p_yellow.png".to_string(), asset_server.p_yellow.clone());
    assets.insert("s_elem_1_green.png".to_string(), asset_server.s_elem_1_green.clone());
    assets.insert("e_elem_4_blue.png".to_string(), asset_server.e_elem_4_blue.clone());
    assets.insert("s_elem_4_purple.png".to_string(), asset_server.s_elem_4_purple.clone());
    assets.insert("s_elem_9_green.png".to_string(), asset_server.s_elem_9_green.clone());
    assets.insert("br_over_tb.png".to_string(), asset_server.br_over_tb.clone());
    assets.insert("s_elem_4_green.png".to_string(), asset_server.s_elem_4_green.clone());
    assets.insert("s_arrow_elem_rigth.png".to_string(), asset_server.s_arrow_elem_rigth.clone());
    assets.insert("e_base.png".to_string(), asset_server.e_base.clone());
    assets.insert("e_elem_1_red.png".to_string(), asset_server.e_elem_1_red.clone());
    assets.insert("e_elem_1_purple.png".to_string(), asset_server.e_elem_1_purple.clone());
    assets.insert("e_elem_1_blue.png".to_string(), asset_server.e_elem_1_blue.clone());
    assets.insert("s_elem_9_blue.png".to_string(), asset_server.s_elem_9_blue.clone());
    assets.insert("tb_over_br.png".to_string(), asset_server.tb_over_br.clone());
    assets.insert("e_elem_1_brown.png".to_string(), asset_server.e_elem_1_brown.clone());
    assets.insert("e_elem_4_purple.png".to_string(), asset_server.e_elem_4_purple.clone());
    assets.insert("p_orange.png".to_string(), asset_server.p_orange.clone());
    assets.insert("e_elem_9_red.png".to_string(), asset_server.e_elem_9_red.clone());
    assets.insert("e_elem_9_purple.png".to_string(), asset_server.e_elem_9_purple.clone());
    assets.insert("e_elem_9_orange.png".to_string(), asset_server.e_elem_9_orange.clone());
    assets.insert("tr_over_tl.png".to_string(), asset_server.tr_over_tl.clone());
    assets.insert("s_elem_4_yellow.png".to_string(), asset_server.s_elem_4_yellow.clone());
    assets.insert("s_elem_4_blue.png".to_string(), asset_server.s_elem_4_blue.clone());
    assets.insert("p_purple.png".to_string(), asset_server.p_purple.clone());
    assets.insert("e_elem_4_orange.png".to_string(), asset_server.e_elem_4_orange.clone());
    assets.insert("s_elem_4_brown.png".to_string(), asset_server.s_elem_4_brown.clone());
    assets.insert("s_elem_9_yellow.png".to_string(), asset_server.s_elem_9_yellow.clone());
    assets.insert("e_elem_1_orange.png".to_string(), asset_server.e_elem_1_orange.clone());
    assets.insert("s_elem_4_red.png".to_string(), asset_server.s_elem_4_red.clone());
    assets.insert("br.png".to_string(), asset_server.br.clone());
    assets.insert("e_elem_1_green.png".to_string(), asset_server.e_elem_1_green.clone());
    assets.insert("s_elem_1_yellow.png".to_string(), asset_server.s_elem_1_yellow.clone());
    let res =  BoardAssetsMap{assets: assets};
    // println!("assets keys are: {:?}", res.assets.keys());
    return res;
}