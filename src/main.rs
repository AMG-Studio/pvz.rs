use bevy::prelude::*;
use pvz::camera::PVZCameraPlugin;
use pvz::cursor::PVZCursorPlugin;
use pvz::debug::PVZDebugPlugin;

const SIZE: f32 = 50.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "PVZ".to_string(),
            width: SIZE * 16.,
            height: SIZE * 9.,
            cursor_visible: false,
            ..Default::default()
        })
        .insert_resource(ClearColor::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(PVZDebugPlugin)
        .add_plugin(PVZCameraPlugin)
        .add_plugin(PVZCursorPlugin)
        .run();
}
