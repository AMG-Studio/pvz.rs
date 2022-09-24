use math::vec2::*;
use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
  pub value: Vec2<f32>,
  pub step: f32,
}
#[derive(Component, Debug)]
pub struct Position(pub Vec2<f32>);
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
