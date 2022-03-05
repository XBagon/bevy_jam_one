use crate::{
    nalgebra::Isometry2, Monster, SlimeBall, TargetStatus, HALF_HEIGHT, HALF_WIDTH, PHYSICS_SCALE,
};
use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use bevy_ase::asset::AseFileMap;
use bevy_rapier2d::prelude::*;
use std::path::Path;

pub struct Game {
    phase: Phase,
}

#[derive(Clone, Debug)]
pub enum Phase {
    Start,
    TransIntro,
    Intro,
    TransDead,
    Dead,
    TransMain,
    TransMain2,
    Main,
    TransEnd,
    End,
    Egg,
}

pub struct Won;

#[derive(Component)]
pub struct Overlay {
    press_start_overlay: Handle<Image>,
    you_win_overlay: Handle<Image>,
    memory_overflow_overlay: Handle<Image>,
    egg_overlay: Handle<Image>,
}

impl Game {
    pub fn setup(mut commands: Commands, ase_file_map: Res<AseFileMap>) {
        let overlays_assets = ase_file_map
            .get(Path::new("sprites/overlays.aseprite"))
            .unwrap();
        let overlay = Overlay {
            press_start_overlay: overlays_assets.texture(0).unwrap().clone(),
            you_win_overlay: overlays_assets.texture(1).unwrap().clone(),
            memory_overflow_overlay: overlays_assets.texture(2).unwrap().clone(),
            egg_overlay: overlays_assets.texture(3).unwrap().clone(),
        };

        commands
            .spawn_bundle(SpriteBundle {
                texture: overlay.press_start_overlay.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::splat(3.8),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(overlay);

        commands.insert_resource(Game {
            phase: Phase::Start,
        });
        //TOP
        commands.spawn_bundle(ColliderBundle {
            position: Isometry2::new(
                Vector::new(0.0, HALF_HEIGHT / PHYSICS_SCALE),
                std::f32::consts::PI,
            )
            .into(),
            shape: ColliderShape::heightfield(
                DVector::from_element(2, 0.0),
                Vector::new(HALF_WIDTH * 2.0 / PHYSICS_SCALE, 0.),
            )
            .into(),
            flags: ColliderFlags {
                solver_groups: InteractionGroups::new(1 << 0, !0),
                collision_groups: InteractionGroups::new(1 << 0, !(1 << 5)),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
        //BOTTOM
        commands.spawn_bundle(ColliderBundle {
            position: Isometry2::new(Vector::new(0.0, -HALF_HEIGHT / PHYSICS_SCALE), 0.0).into(),
            shape: ColliderShape::heightfield(
                DVector::from_element(2, 0.0),
                Vector::new(HALF_WIDTH * 2.0 / PHYSICS_SCALE, 0.),
            )
            .into(),
            flags: ColliderFlags {
                solver_groups: InteractionGroups::new(1 << 0, !0),
                collision_groups: InteractionGroups::new(1 << 0, !(1 << 5)),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
        //LEFT
        commands.spawn_bundle(ColliderBundle {
            position: Isometry2::new(
                Vector::new(-HALF_WIDTH / PHYSICS_SCALE, 0.),
                std::f32::consts::FRAC_PI_2,
            )
            .into(),
            shape: ColliderShape::heightfield(
                DVector::from_element(2, 0.0),
                Vector::new(HALF_HEIGHT * 2.0 / PHYSICS_SCALE, 0.),
            )
            .into(),
            flags: ColliderFlags {
                solver_groups: InteractionGroups::new(1 << 0, !0),
                collision_groups: InteractionGroups::new(1 << 0, !(1 << 5)),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
        //RIGHT
        commands.spawn_bundle(ColliderBundle {
            position: Isometry2::new(
                Vector::new(HALF_WIDTH / PHYSICS_SCALE, 0.),
                -std::f32::consts::FRAC_PI_2,
            )
            .into(),
            shape: ColliderShape::heightfield(
                DVector::from_element(2, 0.0),
                Vector::new(HALF_HEIGHT * 2.0 / PHYSICS_SCALE, 0.),
            )
            .into(),
            flags: ColliderFlags {
                solver_groups: InteractionGroups::new(1 << 0, !0),
                collision_groups: InteractionGroups::new(1 << 0, !(1 << 5)),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
    }

    pub fn update(
        mut game: ResMut<Game>,
        keys: Res<Input<KeyCode>>,
        mut key_evr: EventReader<KeyboardInput>,
        mut q_overlay: Query<(&Overlay, &mut Handle<Image>, &mut Visibility)>,
        mut q_monster: Query<&mut Monster>,
        mut ev_target_status: EventWriter<TargetStatus>,
        mut ev_phase: EventWriter<Phase>,
        mut ev_won: EventReader<Won>,
    ) {
        ev_phase.send(game.phase.clone());
        dbg!(&game.phase);
        match game.phase {
            Phase::Start => {
                if key_evr.iter().any(|ev| ev.state == ElementState::Pressed) {
                    let (_, _, mut visibility) = q_overlay.single_mut();
                    visibility.is_visible = false;
                    game.phase = Phase::TransIntro;
                }
            }
            Phase::TransIntro => {
                for mut monster in q_monster.iter_mut() {
                    monster.phase = crate::monster::Phase::TransShoot;
                }
                game.phase = Phase::Intro;
            }
            Phase::Intro => {}
            Phase::TransDead => {
                let (overlay, mut handle, mut visibility) = q_overlay.single_mut();
                *handle = overlay.you_win_overlay.clone();
                visibility.is_visible = true;
                game.phase = Phase::Dead;
            }
            Phase::Dead => {
                if key_evr.iter().any(|ev| ev.state == ElementState::Pressed) {
                    let (_, _, mut visibility) = q_overlay.single_mut();
                    visibility.is_visible = false;
                    game.phase = Phase::TransMain;
                }
            }
            Phase::TransMain => {
                for mut monster in q_monster.iter_mut() {
                    monster.phase = crate::monster::Phase::TransShoot;
                }
                ev_target_status.send(TargetStatus(true));
                game.phase = Phase::TransMain2;
            }
            Phase::TransMain2 => {
                if ev_won.iter().next().is_some() {
                    game.phase = Phase::TransEnd;
                } else {
                    game.phase = Phase::Main;
                }
            }
            Phase::TransEnd => {
                let (overlay, mut handle, mut visibility) = q_overlay.single_mut();
                *handle = overlay.memory_overflow_overlay.clone();
                visibility.is_visible = true;
                game.phase = Phase::End;
            }
            Phase::Egg => {
                if key_evr.iter().any(|ev| ev.state == ElementState::Pressed) {
                    panic!("You managed to break out of the simulation! You can finally go outside and live your life!")
                }
            }
            _ => {}
        }
        if keys.just_pressed(KeyCode::Escape) {
            let (overlay, mut handle, mut visibility) = q_overlay.single_mut();
            *handle = overlay.egg_overlay.clone();
            visibility.is_visible = true;
            game.phase = Phase::Egg;
        }
    }

    pub fn detect_round_over(
        mut game: ResMut<Game>,
        removed_slime_ball: RemovedComponents<SlimeBall>,
        q_slime_ball: Query<(), With<SlimeBall>>,
    ) {
        if removed_slime_ball.iter().next().is_some() && q_slime_ball.iter().next().is_none() {
            match game.phase {
                Phase::Intro | Phase::Main => game.phase = Phase::TransDead,
                _ => {}
            }
        }
    }
}
