use crate::{MouseCursor, Player, ReadyToJump};
use bevy::prelude::*;

pub struct TargetStatus(pub bool);

impl TargetStatus {
    pub fn changed(
        mut ev_target_status: EventReader<TargetStatus>,
        mut commands: Commands,
        mut q_cursor: Query<(&mut Handle<Image>, &MouseCursor)>,
        mut q_player: Query<Entity, With<Player>>,
    ) {
        if let Some(TargetStatus(active)) = ev_target_status.iter().last() {
            for (mut texture, cursor) in q_cursor.iter_mut() {
                let target_texture = if *active {
                    cursor.active_sprite.clone()
                } else {
                    cursor.inactive_sprite.clone()
                };
                *texture = target_texture;
            }
            for e in q_player.iter_mut() {
                if *active {
                    commands.entity(e).insert(ReadyToJump);
                } else {
                    commands.entity(e).remove::<ReadyToJump>();
                }
            }
        }
    }
}
