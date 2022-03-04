use crate::{BodyPart, BodyPartTextures, TargetStatus};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerDamaged {
    pub pos: Vector<Real>,
    pub vel: Vector<Real>,
}

impl PlayerDamaged {
    pub fn handle_event(
        mut commands: Commands,
        mut ev_spawn_slime_ball: EventReader<PlayerDamaged>,
        body_part_textures: Res<BodyPartTextures>,
        mut ev_target_status: EventWriter<TargetStatus>,
    ) {
        for ev in ev_spawn_slime_ball.iter() {
            ev_target_status.send(TargetStatus(false));
            let rand = (ev.pos.x * 1008. - ev.pos.y * 2502.) * std::f32::consts::PI;
            commands
                .spawn_bundle(SpriteBundle {
                    texture: body_part_textures.0[(rand as usize) % 6].clone(),
                    ..Default::default()
                })
                .insert_bundle(RigidBodyBundle {
                    position: Isometry::new(
                        ev.pos,
                        rand,
                    )
                    .into(),
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
        }
    }
}
