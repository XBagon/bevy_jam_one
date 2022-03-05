use crate::nalgebra::Vector2;
use crate::{game, util, PlayerDamaged, PHYSICS_SCALE};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_ase::asset::{Animation, AseFileMap};
use bevy_rapier2d::prelude::*;
use std::path::Path;

#[derive(Component)]
pub struct Player {
    idle_animation: util::Animation,
    slimed_animation: util::Animation,
}

impl Player {
    pub fn spawn(
        mut commands: Commands,
        ase_file_map: Res<AseFileMap>,
        animations: Res<Assets<Animation>>,
        mut sprite_sheet_animations: ResMut<Assets<benimator::SpriteSheetAnimation>>,
    ) {
        let asset_map = ase_file_map
            .get(Path::new("sprites/person_player.aseprite"))
            .unwrap();

        let (texture_atlas, anim) = util::Animation::get_components(&animations, asset_map, "Idle");
        let idle_animation =
            util::Animation::from_components(texture_atlas, sprite_sheet_animations.add(anim));

        let (texture_atlas, anim) =
            util::Animation::get_components(&animations, asset_map, "Slimed");
        let slimed_animation =
            util::Animation::from_components(texture_atlas, sprite_sheet_animations.add(anim));

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: idle_animation.texture_atlas.clone(),
                transform: Transform::from_xyz(0., 0., 5.),
                ..Default::default()
            })
            .insert(idle_animation.sprite_sheet_animation.clone())
            .insert(benimator::Play)
            .insert(Player {
                idle_animation,
                slimed_animation,
            })
            .insert_bundle(RigidBodyBundle {
                position: Vec2::new(0., -105. / PHYSICS_SCALE).into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                //shape: ColliderShape::cuboid(1.25, 2.2).into(),
                shape: ColliderShape::capsule(Point::new(0.0, -1.5), Point::new(0.0, 1.2), 0.8)
                    .into(),
                mass_properties: ColliderMassProps::Density(500.).into(),
                flags: ColliderFlags {
                    solver_groups: InteractionGroups::new(1 << 1, !0),
                    collision_groups: InteractionGroups::new(1 << 1, !0),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete);
    }

    pub fn on_damaged(
        mut ev_player_damaged: EventReader<PlayerDamaged>,
        mut q_player: Query<(
            &Player,
            &mut Handle<TextureAtlas>,
            &mut Handle<SpriteSheetAnimation>,
        )>,
    ) {
        for _ in ev_player_damaged.iter() {
            info!("OUCH!");
            let (player, mut texture_atlas, mut sprite_sheet_animation) = q_player.single_mut();
            player
                .slimed_animation
                .apply_animation(&mut texture_atlas, &mut sprite_sheet_animation);
        }
    }

    pub fn on_phase(
        mut ev_phase: EventReader<game::Phase>,
        mut q_player: Query<(
            &Player,
            &mut RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &mut Handle<TextureAtlas>,
            &mut Handle<SpriteSheetAnimation>,
        )>,
    ) {
        for ev in ev_phase.iter() {
            if let game::Phase::TransDead = ev {
                info!("Reset Player");
                let (
                    player,
                    mut rigid_body_position,
                    mut rigid_body_velocity,
                    mut texture_atlas,
                    mut sprite_sheet_animation,
                ) = q_player.single_mut();
                player
                    .idle_animation
                    .apply_animation(&mut texture_atlas, &mut sprite_sheet_animation);
                rigid_body_position.position.translation.y = -10.5;
                rigid_body_position.position.rotation = Rotation::new(0.);
                rigid_body_velocity.linvel = Vector2::repeat(0.);
                rigid_body_velocity.angvel = 0.;
            }
        }
    }
}
