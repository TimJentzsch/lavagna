mod debug;
mod drawing;
mod keybinding;
mod local_chalk;

use bevy::{prelude::*, window::Window};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

use crate::debug::DebugPlugin;
use crate::drawing::DrawingPlugin;
use crate::keybinding::KeybindingPlugin;
use crate::local_chalk::LocalPenPlugin;

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (640., 480.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(FramepacePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(LocalPenPlugin)
        .add_plugin(DrawingPlugin)
        .add_plugin(KeybindingPlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut framepace: ResMut<FramepaceSettings>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    clear_color.0 = Color::BLACK;
    
    // Limit frame rate, we dont't want to squeeze that CPU
    framepace.limiter = Limiter::from_framerate(30.0);
}

#[derive(Component, Debug, Clone, Copy)]
struct Chalk {
    pressed: bool,
    updated: bool,
    x: i64,
    y: i64,
    color: Color,
    line_width: u32,
}

impl Chalk {
    fn new() -> Self {
        Self {
            pressed: false,
            updated: false,
            x: 0,
            y: 0,
            color: Color::WHITE,
            line_width: 10,
        }
    }
}
