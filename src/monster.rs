use crate::{util, SpawnSlimeBall};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_ase::asset::{Animation, AseFileMap};
use std::path::Path;

#[derive(Component)]
pub struct Monster {
    idle_animation: util::Animation,
    shoot_animation: util::Animation,
    pub phase: Phase,
}

#[derive(Debug)]
pub enum Phase {
    TransIdle,
    Idle,
    TransShoot,
    Shoot,
}

impl Monster {
    pub fn spawn(
        mut commands: Commands,
        ase_file_map: Res<AseFileMap>,
        animations: Res<Assets<Animation>>,
        mut sprite_sheet_animations: ResMut<Assets<benimator::SpriteSheetAnimation>>,
    ) {
        let asset_map = ase_file_map
            .get(Path::new("sprites/monster.aseprite"))
            .unwrap();

        let (texture_atlas, anim) = util::Animation::get_components(&animations, asset_map, "Idle");
        let idle_animation = util::Animation::from_components(
            texture_atlas,
            sprite_sheet_animations.add(anim.once()),
        );

        let (texture_atlas, anim) =
            util::Animation::get_components(&animations, asset_map, "Shoot");
        let shoot_animation = util::Animation::from_components(
            texture_atlas,
            sprite_sheet_animations.add(anim.once()),
        );

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: idle_animation.texture_atlas.clone(),
                transform: Transform::from_xyz(-145., 111., 0.).with_scale(Vec3::splat(0.6)),
                ..Default::default()
            })
            .insert(idle_animation.sprite_sheet_animation.clone())
            .insert(benimator::Play)
            .insert(Monster {
                idle_animation,
                shoot_animation,
                phase: Phase::Idle,
            });
    }

    pub fn animation_finished(
        mut commands: Commands,
        mut q_monster: Query<(
            &mut Monster,
            &mut Handle<TextureAtlas>,
            &mut Handle<SpriteSheetAnimation>,
        )>,
        removed_play: RemovedComponents<benimator::Play>,
        mut ev_spawn_slime_ball: EventWriter<SpawnSlimeBall>,
    ) {
        for e in removed_play.iter() {
            if let Ok((mut monster, mut texture_atlas, mut sprite_sheet_animation)) =
                q_monster.get_mut(e)
            {
                match monster.phase {
                    Phase::TransIdle => {
                        monster
                            .idle_animation
                            .apply_animation(&mut texture_atlas, &mut sprite_sheet_animation);
                        monster.phase = Phase::Idle;
                    }
                    Phase::TransShoot => {
                        monster
                            .shoot_animation
                            .apply_animation(&mut texture_atlas, &mut sprite_sheet_animation);
                        monster.phase = Phase::Shoot;
                    }
                    Phase::Shoot => {
                        ev_spawn_slime_ball.send(SpawnSlimeBall);
                        monster
                            .idle_animation
                            .apply_animation(&mut texture_atlas, &mut sprite_sheet_animation);
                        monster.phase = Phase::TransIdle;
                    }
                    _ => {}
                }
                commands.entity(e).insert(benimator::Play);
            }
        }
    }
}
