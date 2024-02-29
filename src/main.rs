mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    //spawn ground
    commands.spawn((SpriteBundle {
        texture: asset_server.load("sprites/ground.png"),
        transform: Transform::from_translation(Vec3::new(0., -248., 1.)).with_scale(Vec3::new(64., 4., 1.)),
        ..default()
    },
    ImageScaleMode::Tiled {
        tile_x: true,
        tile_y: false,
        stretch_value: 0.0625,
    }));
}
