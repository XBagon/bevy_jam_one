use crate::{util::screen_to_world_pos, MainCamera};
use bevy::prelude::*;
use bevy_ase::asset::AseFileMap;
use std::path::Path;

#[derive(Component)]
pub struct MouseCursor {
    pub active_sprite: Handle<Image>,
    pub inactive_sprite: Handle<Image>,
}

impl MouseCursor {
    pub fn spawn(mut commands: Commands, ase_file_map: Res<AseFileMap>) {
        //commands.spawn().insert(Game);
        let target_assets = ase_file_map.get(Path::new("sprites/target.aseprite")).unwrap();
        let cursor = MouseCursor {
            active_sprite: target_assets.texture(0).unwrap().clone(),
            inactive_sprite: target_assets.texture(1).unwrap().clone(),
        };
        commands
            .spawn_bundle(SpriteBundle {
                texture: cursor.inactive_sprite.clone(),
                ..Default::default()
            })
            .insert(cursor);
    }

    pub fn update(
        mut q_cursor: Query<&mut Transform, With<MouseCursor>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        windows: Res<Windows>,
    ) {
        let (camera, camera_transform) = q_camera.single();
        let window = windows.get(camera.window).unwrap();
        debug_assert_eq!(window.id(), windows.get_primary().unwrap().id());

        if let Some(world_pos) = screen_to_world_pos(camera, camera_transform, window) {
            q_cursor.single_mut().translation = world_pos.extend(10.);
        }
    }
}
