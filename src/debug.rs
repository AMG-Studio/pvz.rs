use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};

pub struct PVZDebugPlugin;

impl Plugin for PVZDebugPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugin(WorldInspectorPlugin::new());
  }
}
