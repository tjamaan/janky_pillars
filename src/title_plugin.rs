use bevy::prelude::*;

use crate::{despawn_non_persistent, GameAssets, GameState};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            setup.in_schedule(OnEnter(GameState::Title)),
            teardown.in_schedule(OnExit(GameState::Title)),
            despawn_non_persistent.after(teardown),
        ));
    }
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    println!("<tj> Title: Setup");
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|root| {
            root.spawn(TextBundle::from_section(
                "Janky Pillars",
                TextStyle {
                    font: game_assets.font.clone(),
                    font_size: 64.,
                    color: Color::YELLOW,
                },
            ));

            root.spawn(ButtonBundle {
                background_color: Color::CRIMSON.into(),
                ..default()
            })
            .with_children(|button| {
                button.spawn(TextBundle::from_section(
                    "Play",
                    TextStyle {
                        font: game_assets.font.clone(),
                        font_size: 32.,
                        color: Color::WHITE,
                    },
                ));
            });
        });
}

fn teardown() {
    println!("<tj> Title: Teardown");
}
