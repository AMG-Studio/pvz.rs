pub mod marker;
pub mod plants;
pub mod zombies;

use bevy_ecs::prelude::*;
use bevy_time::{prelude::*, FixedTimesteps};

pub struct Game {
  pub world: World,
  pub schedule: Schedule,
}

impl Game {
  pub fn setup() -> Self {
    let mut world = World::new();
    world.insert_resource(Time::default());
    world.insert_resource(FixedTimesteps::default());

    let schedule = Schedule::default();

    Game { world, schedule }
  }
}
