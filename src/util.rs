use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_ase::asset::AseAssetMap;
use crate::nalgebra::Vector2;
use crate::nalgebra;

pub fn screen_to_world_pos(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Option<Vec2> {
    window.cursor_position().map(|screen_pos| {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    })
}

pub fn full_angle_between(v1: &Vector2<f32>, v2: &Vector2<f32>) -> f32 {
    let dot = v1.dot(v2);
    let perp = v1.perp(v2);
    nalgebra::RealField::atan2(perp, dot)
}

pub struct Animation {
    pub texture_atlas: Handle<TextureAtlas>,
    pub sprite_sheet_animation: Handle<SpriteSheetAnimation>,
}

impl Animation {
    pub fn get_components(
        animations: &Assets<bevy_ase::asset::Animation>,
        asset_map: &AseAssetMap,
        tag: &str,
    ) -> (Handle<TextureAtlas>, SpriteSheetAnimation) {
        let animation_handle = asset_map.animations(tag).unwrap().first().unwrap();
        let animation = animations.get(animation_handle).unwrap();
        let texture_atlas = animation.atlas();
        let ssa = SpriteSheetAnimation::from(animation);
        (texture_atlas, ssa)
    }

    pub fn from_components(
        texture_atlas: Handle<TextureAtlas>,
        sprite_sheet_animation: Handle<SpriteSheetAnimation>,
    ) -> Self {
        Self {
            texture_atlas,
            sprite_sheet_animation,
        }
    }

    pub fn apply_animation(
        &self,
        texture_atlas: &mut Handle<TextureAtlas>,
        sprite_sheet_animation: &mut Handle<SpriteSheetAnimation>,
    ) {
        *texture_atlas = self.texture_atlas.clone();
        *sprite_sheet_animation = self.sprite_sheet_animation.clone();
    }
}

pub struct DespawnEntity(pub Entity);

impl DespawnEntity {
    pub fn handle_event(mut commands: Commands, mut ev_despawn_entity: EventReader<DespawnEntity>) {
        for DespawnEntity(e) in ev_despawn_entity.iter() {
            commands.entity(*e).despawn();
        }
    }
}