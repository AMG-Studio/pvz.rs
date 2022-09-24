use bevy::prelude::*;

pub struct PVZCameraPlugin;

impl Plugin for PVZCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
