use bevy::prelude::*;
const DEFAULT_CURSOR_POS: Vec3 = Vec3::new(0., 0., 5.);

pub struct PVZCursorPlugin;

impl Plugin for PVZCursorPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_startup_system(load_cursor_asset)
      .add_system(move_cursor);
  }
}

#[derive(Component)]
struct Cursor;

fn move_cursor(windows: Res<Windows>, mut cursor_query: Query<(&Cursor, &mut Transform)>) {
  let window = windows.get_primary().unwrap();
  if let Some(position) = window.cursor_position() {
    let (_, mut transform) = cursor_query.single_mut();
    transform.translation = Vec3::new(
      position.x - window.width() / 2.,
      position.y - window.height() / 2.,
      100.,
    );
  }
}

fn load_cursor_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
  let cursor_texture = asset_server.load("cursor.png");
  commands.spawn().insert(Cursor).insert_bundle(SpriteBundle {
    transform: Transform {
      translation: DEFAULT_CURSOR_POS,
      ..Default::default()
    },
    sprite: Sprite {
      custom_size: Some(Vec2::new(24., 24.)),
      ..Default::default()
    },
    texture: cursor_texture,
    ..Default::default()
  });
}
