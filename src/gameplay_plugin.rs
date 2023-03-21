use bevy::ecs::system::EntityCommands;
use bevy::{prelude::*, sprite::Anchor, utils::HashMap};
use rand::distributions::{Distribution, Standard};
use rand::random;

use crate::util::Spawner;
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
            drop_floating_piece.in_set(OnUpdate(GameState::Gameplay)),
            update_floating_piece_position
                .in_set(OnUpdate(GameState::Gameplay))
                .after(drop_floating_piece),
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

impl Distribution<GemType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> GemType {
        match rng.gen_range(0..6) {
            0 => GemType::Red,
            1 => GemType::Pink,
            2 => GemType::Blue,
            3 => GemType::Green,
            4 => GemType::Yellow,
            5 => GemType::Orange,
            _ => unreachable!(),
        }
    }
}

#[derive(Component)]
struct Well {
    floating_piece: [Gem; 3],
    floating_piece_loc: (u32, u32),
    floating_piece_progress: f32,
    speed: f32,
    well: HashMap<(u32, u32), Gem>,
}

impl Well {
    fn pick_next_piece(&mut self) {
        for mut gem in self.floating_piece.iter_mut() {
            gem.gem_type = random();
        }
    }
}

trait SpawnerExt<'w, 's> {
    fn spawn_gem(&mut self, gem_type: GemType) -> EntityCommands<'w, 's, '_>;
}

impl<'w, 's, T: Spawner<'w, 's>> SpawnerExt<'w, 's> for T {
    fn spawn_gem(&mut self, gem_type: GemType) -> EntityCommands<'w, 's, '_> {
        let color = match gem_type {
            GemType::Red => Color::RED,
            GemType::Pink => Color::PINK,
            GemType::Blue => Color::BLUE,
            GemType::Green => Color::GREEN,
            GemType::Yellow => Color::YELLOW,
            GemType::Orange => Color::ORANGE,
        };

        self.spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(20.)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        })
    }
}

fn setup(mut commands: Commands) {
    println!("<tj> Gameplay: Setup");
    let (mut gem1, mut gem2, mut gem3) = (None, None, None);
    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Well".to_string()))
        .with_children(|well| {
            gem1 = well
                .spawn_gem(GemType::Red)
                .insert(Name::new("Gem1".to_string()))
                .insert(FloatingGem)
                .id()
                .into();
            gem2 = well
                .spawn_gem(GemType::Orange)
                .insert(Name::new("Gem2".to_string()))
                .insert(FloatingGem)
                .id()
                .into();
            gem3 = well
                .spawn_gem(GemType::Green)
                .insert(Name::new("Gem3".to_string()))
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
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                    ..default()
                })
                .insert(Name::new("WellBackground".to_string()))
                .id();
        })
        .insert(Well {
            floating_piece: [
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
            ],
            floating_piece_loc: (WELL_ROWS, 3),
            floating_piece_progress: 0.,
            speed: 5.,
            well: HashMap::new(),
        })
        .insert(NonPersistent);
}

fn teardown() {
    println!("<tj> Title: Teardown");
}

fn drop_floating_piece(time: Res<Time>, mut q: Query<&mut Well>) {
    for mut well in &mut q {
        well.floating_piece_progress += time.delta_seconds() * well.speed;
        if well.floating_piece_progress > 1. {
            well.floating_piece_progress = 0.;
            well.floating_piece_loc.0 = if well.floating_piece_loc.0 > 0 {
                well.floating_piece_loc.0 - 1
            } else {
                well.pick_next_piece();
                WELL_ROWS
            }
        }
    }
}

fn update_floating_piece_position(
    mut q: Query<&mut Well>,
    mut q2: Query<&mut Transform, With<FloatingGem>>,
) {
    for mut well in &mut q {
        for (i, gem) in well.floating_piece.iter().enumerate() {
            let mut transform = q2.get_mut(gem.entity).unwrap();
            transform.translation.x = well.floating_piece_loc.1 as f32 * 20.;
            transform.translation.y = ((well.floating_piece_loc.0 + i as u32) as f32 * 20.)
                - (20. * (WELL_ROWS as f32 - 2.))
                - (20. * well.floating_piece_progress.clamp(0., 1.));
        }
        well.floating_piece_progress += 0.00001;
    }
}
