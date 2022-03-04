use crate::{game, PlayerDamaged, HALF_HEIGHT, HALF_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct Score(u32);

impl Score {
    pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/iosevka-extendedsemibold.ttf");
        let text_style = TextStyle {
            font,
            font_size: 20.0,
            color: Color::GOLD,
        };
        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };
        commands
            .spawn_bundle(Text2dBundle {
                transform: Transform::from_xyz(HALF_WIDTH - 60., HALF_HEIGHT - 10., 9.),
                text: Text::with_section(String::from("Score: -0000"), text_style, text_alignment),
                ..Default::default()
            })
            .insert(Score(0));
    }

    pub fn on_player_damaged(
        mut ev_player_damaged: EventReader<PlayerDamaged>,
        mut q_score: Query<(&mut Score, &mut Text)>,
    ) {
        for _ev in ev_player_damaged.iter() {
            let (mut score, mut text) = q_score.single_mut();
            score.0 += 1;
            text.sections[0].value = format!("Score: -{:04}", score.0);
        }
    }

    pub fn on_end(
        mut q_score: Query<(&Score, &mut Transform, &mut Text)>,
        mut ev_phase: EventReader<game::Phase>,
    ) {
        for ev in ev_phase.iter() {
            if let game::Phase::TransEnd = ev {
                let (score, mut transform, mut text) = q_score.single_mut();
                transform.translation = Vec3::new(-90., -35., 1.);
                transform.scale = Vec3::new(-4., 4., 1.);
                let mut text = &mut text.sections[0];
                text.value = format!("-{:04}", score.0);
                //text.style.font_size = 16.;
            }
        }
    }
}
