use crate::{util::screen_to_world_pos, MainCamera, TargetStatus};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct ReadyToJump;

impl ReadyToJump {
    pub fn update(
        mut q_rb: Query<
            (
                &mut RigidBodyVelocityComponent,
                &RigidBodyMassPropsComponent,
                &Transform,
            ),
            With<ReadyToJump>,
        >,
        buttons: Res<Input<MouseButton>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        windows: Res<Windows>,
        mut ev_target_status: EventWriter<TargetStatus>,
    ) {
        if buttons.just_pressed(MouseButton::Left) {
            let (camera, camera_transform) = q_camera.single();
            let window = windows.get(camera.window).unwrap();
            debug_assert_eq!(window.id(), windows.get_primary().unwrap().id());

            if let Some(world_pos) = screen_to_world_pos(camera, camera_transform, window) {
                for (mut velocity, mass_props, transform) in q_rb.iter_mut() {
                    let impulse = (world_pos - transform.translation.xy()) * 300.0;
                    velocity.apply_impulse(mass_props, impulse.into());
                    info!("THIS IS SOME IMPULSE: {}", impulse);
                }
                ev_target_status.send(TargetStatus(false));
            }
        }
    }
}
