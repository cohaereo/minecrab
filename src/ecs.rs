use std::{fmt::Debug, num::NonZeroU32};

use glam::Vec3;
use hecs::{Entity, World};

// In units (aka blocks) per second
pub const VELOCITY_UNIT: f32 = (1. / 8000.) * 20.;
// pub const VELOCITY_UNIT: f32 = (1. / 8000.);

pub struct Position(pub Vec3);
pub struct Velocity(pub Vec3);

pub fn update_velocity(world: &mut World, delta: f32) {
    for (e, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
        // FIXME: We're not moving in Y, because the lack of block collision makes mobs phase through the ground
        let temp_vel = Vec3::new(vel.0.x, 0., vel.0.z);
        pos.0 += temp_vel * delta;
    }
}

pub fn get_or_insert(world: &mut World, eid: i32) -> Entity {
    // println!("{:?}", NonZeroU32::new(eid as u32));
    let ent = Entity::from_bits(eid as u64 | (1 << 32)).unwrap();
    if world.get::<(&Position)>(ent).is_ok() {
        ent
    } else {
        world.spawn_at(ent, (Position(Vec3::default()), Velocity(Vec3::default())));
        ent
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<{:.2},{:.2},{:.2}>",
            self.0.x, self.0.y, self.0.z
        ))
    }
}
