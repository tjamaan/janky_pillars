use bevy::{prelude::*, utils::HashMap};

use crate::{despawn_non_persistent, GameState, NonPersistent};

const WELL_COLS: u32 = 6;
const WELL_ROWS: u32 = 13;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            // on enter
            setup.in_schedule(OnEnter(GameState::Gameplay)),
            // on exit
            teardown.in_schedule(OnExit(GameState::Gameplay)),
            despawn_non_persistent
                .in_schedule(OnExit(GameState::Gameplay))
                .after(teardown),
            // on update
            update_floating_piece_position.in_set(OnUpdate(GameState::Gameplay)),
        ));
    }
}

#[derive(Component)]
struct FloatingGem;

struct Gem {
    gem_type: GemType,
    entity: Entity,
}

enum GemType {
    Red,
    Pink,
    Blue,
    Green,
    Yellow,
    Orange,
}

#[derive(Component)]
struct Well {
    floating_piece: (Gem, Gem, Gem),
    floating_piece_loc: (i32, i32),
    floating_piece_progress: f32,
    well: HashMap<(i32, i32), Gem>,
}

fn setup(mut commands: Commands) {
    println!("<tj> Gameplay: Setup");
    let (mut gem1, mut gem2, mut gem3) = (None, None, None);
    commands
        .spawn(SpatialBundle::default())
        .with_children(|well| {
            gem1 = well
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::splat(20.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(FloatingGem)
                .id()
                .into();
            gem2 = well
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLUE,
                        custom_size: Some(Vec2::splat(20.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(FloatingGem)
                .id()
                .into();
            gem3 = well
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::splat(20.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(FloatingGem)
                .id()
                .into();
            let _background = well
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GRAY,
                        custom_size: Some(Vec2::new(
                            20. * WELL_COLS as f32,
                            20. * WELL_ROWS as f32,
                        )),
                        ..default()
                    },
                    ..default()
                })
                .id();
        })
        .insert(Well {
            floating_piece: (
                Gem {
                    entity: gem1.unwrap(),
                    gem_type: GemType::Red,
                },
                Gem {
                    entity: gem2.unwrap(),
                    gem_type: GemType::Blue,
                },
                Gem {
                    entity: gem3.unwrap(),
                    gem_type: GemType::Green,
                },
            ),
            floating_piece_loc: (-2, 3),
            floating_piece_progress: 0.,
            well: HashMap::new(),
        })
        .insert(NonPersistent);
}

fn teardown() {
    println!("<tj> Title: Teardown");
}

fn update_floating_piece_position(
    q: Query<&mut Well>,
    q2: Query<&mut Transform, With<FloatingGem>>,
) {
    for mut well in &q {
        // todo: move the entities with the floating piece
    }
}
