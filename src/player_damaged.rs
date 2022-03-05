use crate::{BodyPart, BodyPartTextures, Player, SpawnSlimeBall, TargetStatus};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerDamaged {
    pub pos: Vector<Real>,
    pub vel: Vector<Real>,
    pub slime_ball_health: i32,
}

impl PlayerDamaged {
    pub fn handle_event(
        mut commands: Commands,
        mut ev_player_damaged: EventReader<PlayerDamaged>,
        body_part_textures: Res<BodyPartTextures>,
        mut ev_target_status: EventWriter<TargetStatus>,
        q_rigid_body_velocity: Query<&RigidBodyVelocityComponent, With<Player>>,
        mut ev_spawn_slime_ball: EventWriter<SpawnSlimeBall>,
    ) {
        for ev in ev_player_damaged.iter() {
            ev_target_status.send(TargetStatus(false));
            let rand = (ev.pos.x * 1008. - ev.pos.y * 2502.) * std::f32::consts::PI;
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(0., 1000., 3.),
                    texture: body_part_textures.0[(rand as usize) % 6].clone(),
                    ..Default::default()
                })
                .insert_bundle(RigidBodyBundle {
                    position: Isometry::new(ev.pos, rand).into(),
                    velocity: RigidBodyVelocity {
                        linvel: ev.vel,
                        ..Default::default()
                    }
                    .into(),

                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::round_cuboid(0.5, 0.8, 0.2).into(),
                    mass_properties: ColliderMassProps::Density(20.).into(),
                    flags: ColliderFlags {
                        solver_groups: InteractionGroups::new(1 << 3, 0b1001),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert(RigidBodyPositionSync::Discrete)
                .insert(BodyPart {});

            let rigid_body_velocity = q_rigid_body_velocity.single();
            let angle = rigid_body_velocity.linvel.angle(&ev.vel);
            let split_power =
                angle / std::f32::consts::PI * rigid_body_velocity.linvel.magnitude() / 10.;
            //split_power *= split_power;
            let new_health = split_power * ev.slime_ball_health as f32;
            if new_health > 10. {
                ev_spawn_slime_ball.send(SpawnSlimeBall {
                    position: Some((ev.pos - (ev.vel).normalize() * 2.0).into()),
                    velocity: Some(-ev.vel),
                    health: new_health as i32,
                });
                ev_spawn_slime_ball.send(SpawnSlimeBall {
                    position: Some((ev.pos - (ev.vel).normalize() * 2.0).into()),
                    velocity: Some(-ev.vel),
                    health: new_health as i32,
                });
            }
        }
    }
}
