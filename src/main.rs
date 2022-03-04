use std::path::Path;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ase::{
    self,
    loader::{self, Loader},
};

//use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
pub use body_part::{BodyPart, BodyPartTextures};
pub use game::Game;
pub use monster::Monster;
pub use mouse_cursor::MouseCursor;
pub use player::Player;
pub use player_damaged::PlayerDamaged;
pub use ready_to_jump::ReadyToJump;
pub use score::Score;
pub use slime_ball::{SlimeBall, SlimeBallBundle, SpawnSlimeBall};
pub use target_status::TargetStatus;

mod body_part;
mod game;
mod monster;
mod mouse_cursor;
mod player;
mod player_damaged;
mod ready_to_jump;
mod score;
mod slime_ball;
mod target_status;
mod util;

const PHYSICS_SCALE: f32 = 10.0;
const HALF_HEIGHT: f32 = 128.;
const HALF_WIDTH: f32 = HALF_HEIGHT * (16. / 9.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(loader::AseLoaderDefaultPlugin)
        .add_plugin(benimator::AnimationPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(Msaa::default())
        .insert_resource(RapierConfiguration {
            scale: PHYSICS_SCALE,
            gravity: Vector::new(0.0, -10.0),
            ..Default::default()
        })
        //.add_plugin(WorldInspectorPlugin::new())
        .add_state(AppState::Loading)
        .add_state_to_stage(CoreStage::PostUpdate, AppState::Loading)
        .add_system_set(
            SystemSet::on_enter(AppState::Loading)
                .label("loading_enter")
                .with_system(load_sprites),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Loading)
                .label("loading_update")
                .with_system(check_loading_sprites),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Ready)
                .label("ready_enter")
                .with_system(Game::setup)
                .with_system(SlimeBallBundle::setup)
                .with_system(BodyPart::setup)
                .with_system(MouseCursor::spawn)
                .with_system(Player::spawn)
                .with_system(Monster::spawn)
                .with_system(Score::spawn),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Ready)
                .label("ready_update")
                .with_system(Game::update.label("Game::update"))
                .with_system(MouseCursor::update)
                .with_system(ReadyToJump::update)
                .with_system(SlimeBall::update.after("SpawnSlimeBall::handle_event"))
                .with_system(SlimeBall::on_contact_stopped)
                .with_system(SlimeBall::on_contact_started.label("SlimeBall::on_contact_started"))
                .with_system(
                    PlayerDamaged::handle_event
                        .before("TargetStatus::changed")
                        .after("SlimeBall::on_contact_started"),
                )
                .with_system(TargetStatus::changed.label("TargetStatus::changed"))
                .with_system(SpawnSlimeBall::handle_event.label("SpawnSlimeBall::handle_event"))
                .with_system(Score::on_player_damaged.after("SlimeBall::on_contact_started"))
                .with_system(Player::on_damaged.after("SlimeBall::on_contact_started"))
                .with_system(Player::on_phase)
                .with_system(BodyPart::win_check.before("Game::update"))
                .with_system(Score::on_end),
        )        .add_system_set(
        SystemSet::on_update(AppState::Ready).before("ready_update").with_system(util::DespawnEntity::handle_event)
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(AppState::Ready)
                .label("ready_post_update")
                .with_system(Monster::animation_finished)
                .with_system(Game::detect_round_over),
        )
        .add_event::<TargetStatus>()
        .add_event::<PlayerDamaged>()
        .add_event::<util::DespawnEntity>()
        .add_event::<SpawnSlimeBall>()
        .add_event::<game::Phase>()
        .add_event::<game::Won>()
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Ready,
}

#[derive(Component)]
pub struct MainCamera;

// Collect the sprite and send it to the loader.
pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut aseloader: ResMut<Loader>,
) {
    info!("Loading assets");
    let person_player = asset_server.load(Path::new("sprites/person_player.aseprite"));
    aseloader.add(person_player);
    let slime_ball = asset_server.load(Path::new("sprites/slime_ball.aseprite"));
    aseloader.add(slime_ball);
    let monster = asset_server.load(Path::new("sprites/monster.aseprite"));
    aseloader.add(monster);
    let target = asset_server.load(Path::new("sprites/target.aseprite"));
    aseloader.add(target);
    let target = asset_server.load(Path::new("sprites/overlays.aseprite"));
    aseloader.add(target);
    let target = asset_server.load(Path::new("sprites/body_parts.aseprite"));
    aseloader.add(target);

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = HALF_HEIGHT;
    camera.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    commands.spawn_bundle(camera).insert(MainCamera);
}

// Wait until all sprites are loaded.
pub fn check_loading_sprites(mut state: ResMut<State<AppState>>, ase_loader: Res<Loader>) {
    if ase_loader.is_loaded() {
        info!("All Aseprite files loaded");
        state.set(AppState::Ready).unwrap()
    }
}
