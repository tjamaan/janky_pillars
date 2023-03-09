use bevy::prelude::*;

use crate::{despawn_non_persistent, GameAssets, GameState, NonPersistent};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            // on enter
            setup.in_schedule(OnEnter(GameState::Title)),
            // on exit
            teardown.in_schedule(OnExit(GameState::Title)),
            despawn_non_persistent
                .in_schedule(OnExit(GameState::Title))
                .after(teardown),
            // on update
            handle_play_button.in_set(OnUpdate(GameState::Title)),
        ));
    }
}

#[derive(Component)]
struct PlayButton;

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
        .insert(NonPersistent)
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
            .insert(PlayButton)
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

fn handle_play_button(
    q: Query<&Interaction, (With<PlayButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &q {
        match interaction {
            Interaction::Clicked => {
                println!("<tj> Play button clicked, changing state");
                next_state.set(GameState::Gameplay);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
