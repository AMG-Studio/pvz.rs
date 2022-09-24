use bevy_ecs::prelude::*;
use bevy_math::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
  pub value: Vec2,
  pub step: f32,
}
#[derive(Component, Debug)]
pub struct Position(pub Vec2);
#[derive(Component, Debug)]
pub struct Health(pub f32);

#[derive(Component, Debug)]
pub struct Plant;
#[derive(Component, Debug)]
pub struct Zombie;

#[derive(Component, Debug)]
pub struct Attack;
#[derive(Component, Debug)]
pub struct Projectile;
#[derive(Component, Debug)]
pub struct Missile;
