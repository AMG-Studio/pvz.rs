use bevy::prelude::*;
use pvz::camera::PVZCameraPlugin;
use pvz::cursor::PVZCursorPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "PVZ".to_string(),
            width: 600.,
            height: 400.,
            cursor_visible: false,
            ..Default::default()
        })
        .insert_resource(ClearColor::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(PVZCameraPlugin)
        .add_plugin(PVZCursorPlugin)
        .run();
}
