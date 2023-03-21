use bevy::prelude::*;
mod title_plugin;
mod util;
use title_plugin::TitlePlugin;
mod gameplay_plugin;
use gameplay_plugin::GameplayPlugin;
mod debug_plugin;
use debug_plugin::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Janky Pillars".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .insert_resource(GameAssets::default())
        .add_startup_system(load_assets)
        .add_startup_system(setup_cameras)
        .add_plugin(TitlePlugin)
        .add_plugin(GameplayPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    Title,
    Gameplay,
    Gameover,
}

#[derive(Resource, Default)]
struct GameAssets {
    font: Handle<Font>,
}

fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    println!("<tj> Load assets");
    *game_assets = GameAssets {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
    };
}

fn setup_cameras(mut commands: Commands) {
    println!("<tj> Setup camera");
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 500. - 0.1)),
        ..default()
    });
}

#[derive(Component)]
struct NonPersistent;

fn despawn_non_persistent(
    mut commands: Commands,
    q: Query<Entity, (With<NonPersistent>, Without<Parent>)>,
) {
    println!("<tj> Destroy non persistent");
    q.for_each(|entity| commands.entity(entity).despawn_recursive());
}
