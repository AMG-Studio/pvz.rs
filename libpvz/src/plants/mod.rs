use crate::marker::*;
use bevy_ecs::prelude::*;

#[derive(Debug, Bundle)]
pub struct PeaShooterBundle {
  pub plant: Plant,
  pub health: Health,
  pub position: Position,
}

impl PeaShooterBundle {
  const PEA_SHOOTER_HEALTH: f32 = 20.0;

  pub fn new(pos: Position) -> Self {
    PeaShooterBundle {
      plant: Plant,
      health: Health(Self::PEA_SHOOTER_HEALTH),
      position: pos,
    }
  }
}

