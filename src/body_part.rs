use crate::nalgebra::Vector2;
use crate::{game, HALF_WIDTH, PHYSICS_SCALE};
use bevy::prelude::*;
use bevy_ase::asset::AseFileMap;
use bevy_rapier2d::prelude::*;
use std::path::Path;

#[derive(Component)]
pub struct BodyPart {}

pub struct BodyPartTextures(pub [Handle<Image>; 6]);

impl BodyPart {
    pub fn setup(mut commands: Commands, ase_file_map: Res<AseFileMap>) {
        //commands.spawn().insert(Game);
        let target_assets = ase_file_map
            .get(Path::new("sprites/body_parts.aseprite"))
            .unwrap();

        commands.insert_resource(BodyPartTextures([
            target_assets.texture(0).unwrap().clone(),
            target_assets.texture(1).unwrap().clone(),
            target_assets.texture(2).unwrap().clone(),
            target_assets.texture(3).unwrap().clone(),
            target_assets.texture(4).unwrap().clone(),
            target_assets.texture(5).unwrap().clone(),
        ]));
    }

    pub fn win_check(
        mut ev_phase: EventReader<game::Phase>,
        query_pipeline: Res<QueryPipeline>,
        collider_query: QueryPipelineColliderComponentsQuery,
        q_body_part: Query<&RigidBodyVelocityComponent, With<BodyPart>>,
        mut ev_won: EventWriter<game::Won>,
    ) {
        for ev in ev_phase.iter() {
            if let game::Phase::TransMain = ev {
                let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

                let ray = Ray::new(
                    Point::new(-HALF_WIDTH / PHYSICS_SCALE, 0.),
                    Vector2::new(1.0, 0.0),
                );
                let max_toi = HALF_WIDTH * 2. / PHYSICS_SCALE;
                let solid = true;
                let groups = InteractionGroups::new(1 << 5, 1 << 3);
                let filter = None;

                query_pipeline.intersections_with_ray(
                    &collider_set,
                    &ray,
                    max_toi,
                    solid,
                    groups,
                    filter,
                    |handle, _| {
                        if let Ok(rigid_body_velocity) = q_body_part.get(handle.entity()) {
                            if rigid_body_velocity.linvel.magnitude() < 0.1 {
                                ev_won.send(game::Won);
                                return false;
                            }
                        }
                        true
                    },
                );
            }
        }
    }
}
