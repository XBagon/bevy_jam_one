use crate::nalgebra::{Isometry2, Point2};
use crate::{
    nalgebra::Vector2, util, util::DespawnEntity, Player, PlayerDamaged, HALF_HEIGHT, HALF_WIDTH,
    PHYSICS_SCALE,
};
use benimator::SpriteSheetAnimation;
use bevy::prelude::*;
use bevy_ase::asset::{Animation, AseFileMap};
use bevy_rapier2d::prelude::*;
use std::path::Path;

#[derive(Component, Clone)]
pub struct SlimeBall {
    pub health: i32,
    pub invincible: bool,
}

pub struct SpawnSlimeBall {
    pub position: Option<Isometry2<f32>>,
    pub velocity: Option<Vector2<f32>>,
    pub health: i32,
}

impl SpawnSlimeBall {
    pub fn handle_event(
        mut commands: Commands,
        slime_ball_bundle: Res<SlimeBallBundle>,
        mut ev_spawn_slime_ball: EventReader<SpawnSlimeBall>,
        time: Res<Time>,
    ) {
        let rand = time.time_since_startup().as_secs_f32();
        for ev in ev_spawn_slime_ball.iter() {
            let position = ev
                .position
                .unwrap_or_else(|| Isometry::translation(0., HALF_HEIGHT * 1.2 / PHYSICS_SCALE));
            let velocity = ev
                .velocity
                .unwrap_or_else(|| Vector::new(rand.sin() * 0.1, -0.1));

            let mut slime_ball_bundle = slime_ball_bundle.clone();
            slime_ball_bundle.slime_ball.health = ev.health;

            commands
                .spawn_bundle(slime_ball_bundle)
                .insert_bundle(RigidBodyBundle {
                    position: position.into(),
                    velocity: RigidBodyVelocity {
                        linvel: velocity,
                        ..Default::default()
                    }
                    .into(),
                    ccd: RigidBodyCcd {
                        ccd_enabled: true,
                        ccd_thickness: 0.1,
                        ccd_max_dist: 0.6,
                        ..Default::default()
                    }
                    .into(),
                    mass_properties: RigidBodyMassProps {
                        flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::ball(0.6).into(),
                    mass_properties: ColliderMassProps::Density(40.).into(),
                    flags: ColliderFlags {
                        active_events: ActiveEvents::CONTACT_EVENTS,
                        solver_groups: InteractionGroups::new(1 << 2, !(1 << 0)),
                        ..Default::default()
                    }
                    .into(),
                    material: ColliderMaterial {
                        friction: 0.0,
                        restitution: 1.0,
                        friction_combine_rule: CoefficientCombineRule::Min,
                        restitution_combine_rule: CoefficientCombineRule::Max,
                    }
                    .into(),
                    ..Default::default()
                })
                .insert(RigidBodyPositionSync::Discrete);
        }
    }
}

impl SlimeBall {
    pub fn on_contact_stopped(
        mut q_slime_ball: Query<&mut ColliderFlagsComponent, With<SlimeBall>>,
        mut contact_events: EventReader<ContactEvent>,
    ) {
        for contact_event in contact_events.iter() {
            match contact_event {
                ContactEvent::Stopped(a, b) => {
                    let collider_flags =
                        if let Ok(collider_flags) = q_slime_ball.get_mut(a.entity()) {
                            Some(collider_flags)
                        } else if let Ok(collider_flags) = q_slime_ball.get_mut(b.entity()) {
                            Some(collider_flags)
                        } else {
                            None
                        };

                    if let Some(mut collider_flags) = collider_flags {
                        collider_flags.solver_groups = InteractionGroups::new(1 << 2, !0);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn on_contact_started(
        mut q_slime_ball: Query<(Entity, &mut SlimeBall, &RigidBodyVelocityComponent)>,
        mut contact_events: EventReader<ContactEvent>,
        q_player: Query<&RigidBodyPositionComponent, With<Player>>,
        mut ev_despawn_entity: EventWriter<util::DespawnEntity>,
        mut ev_damaged_player: EventWriter<PlayerDamaged>,
    ) {
        for contact_event in contact_events.iter() {
            match contact_event {
                ContactEvent::Started(a, b) => {
                    let info = if let Ok(slime_ball) = q_slime_ball.get_mut(a.entity()) {
                        Some((b, slime_ball))
                    } else if let Ok(slime_ball) = q_slime_ball.get_mut(b.entity()) {
                        Some((a, slime_ball))
                    } else {
                        None
                    };

                    if let Some((other_collider, (entity, mut slime_ball, rigid_body_velocity))) =
                        info
                    {
                        if !slime_ball.invincible {
                            if let Ok(rigid_body_position) = q_player.get(other_collider.entity()) {
                                let magnitude = rigid_body_velocity.linvel.magnitude();
                                slime_ball.health -= 25;
                                ev_damaged_player.send(PlayerDamaged {
                                    pos: rigid_body_position.position.translation.vector,
                                    vel: rigid_body_velocity.linvel,
                                    slime_ball_health: slime_ball.health,
                                })
                            } else {
                                slime_ball.health -= 15;
                            }

                            if slime_ball.health <= 0 {
                                ev_despawn_entity.send(DespawnEntity(entity));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update(
        mut q_slime_ball: Query<(
            Entity,
            &mut SlimeBall,
            &mut RigidBodyPositionComponent,
            &RigidBodyVelocityComponent,
        )>,
        mut ev_despawn_entity: EventWriter<DespawnEntity>,
    ) {
        for (entity, mut slime_ball, mut rigid_body_position, rigid_body_velocity) in
            q_slime_ball.iter_mut()
        {
            slime_ball.invincible = false;
            rigid_body_position.position.rotation = Rotation::new(util::full_angle_between(
                &(rigid_body_velocity
                    .linvel
                    .component_mul(&Vector::new(-1., 1.))),
                &Vector2::new(0.0, 1.0),
            ));

            let distance = nalgebra::distance(
                &rigid_body_position.position.translation.vector.into(),
                &Point2::new(0., 0.),
            );
            if distance > HALF_WIDTH / PHYSICS_SCALE {
                ev_despawn_entity.send(DespawnEntity(entity));
            }
        }
    }
}

#[derive(Bundle, Clone)]
pub struct SlimeBallBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    slime_ball: SlimeBall,
    anim_handle: Handle<SpriteSheetAnimation>,
    play: benimator::Play,
}

impl SlimeBallBundle {
    pub fn setup(
        mut commands: Commands,
        ase_file_map: Res<AseFileMap>,
        animations: Res<Assets<Animation>>,
        mut sprite_sheet_animations: ResMut<Assets<benimator::SpriteSheetAnimation>>,
    ) {
        let asset_map = ase_file_map
            .get(Path::new("sprites/slime_ball.aseprite"))
            .unwrap();

        let (texture_atlas, anim) = util::Animation::get_components(&animations, asset_map, "Idle");
        let idle_animation =
            util::Animation::from_components(texture_atlas, sprite_sheet_animations.add(anim));

        commands.insert_resource(SlimeBallBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: idle_animation.texture_atlas.clone(),
                transform: Transform::from_xyz(0., 0., 6.),
                ..Default::default()
            },
            slime_ball: SlimeBall {
                health: 100,
                invincible: true,
            },
            anim_handle: idle_animation.sprite_sheet_animation,
            play: benimator::Play,
        })
    }
}
