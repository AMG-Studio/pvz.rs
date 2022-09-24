use bevy_ecs::prelude::*;
use bevy_time::prelude::*;
use crate::marker::*;
use math::vec2::*;

#[derive(Bundle, Debug)]
pub struct ZombieBundle {
  pub zombie: Zombie,
  pub health: Health,
  pub velocity: Velocity,
  pub position: Position,
}

impl ZombieBundle {
  const ZOMBIE_VELOCITY: f32 = -2.0;
  const ZOMBIE_HEALTH: f32 = 20.0;
  const ZOMBIE_STEP: f32 = 1000.0; // millisecond as the unit.

  pub fn new(pos: Position) -> Self {
    ZombieBundle {
      zombie: Zombie,
      health: Health(Self::ZOMBIE_HEALTH),
      velocity: Velocity {
        value: Vec2::new(Self::ZOMBIE_VELOCITY, 0.0),
        step: Self::ZOMBIE_STEP,
      },
      position: pos,
    }
  }
}

pub fn zombies_movement(
  mut query: Query<(&Velocity, &mut Position), With<Zombie>>,
  time: Res<Time>,
) {
  for (v, mut p) in query.iter_mut() {
  }
}
