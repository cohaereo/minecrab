use std::fmt::Debug;

use cgmath::{Point3, Vector3, Zero};
use hecs::{Entity, World};

// In units (aka blocks) per second
pub const VELOCITY_UNIT: f32 = (1. / 8000.) * 20.;
// pub const VELOCITY_UNIT: f32 = (1. / 8000.);

pub const TICK_DELTA: f32 = 1. / 20.;

/// Used for interpolation
pub struct InterpolatedPosition {
    pub position: Point3<f32>,
    pub delta: f32, // The amount of time until we reach the target
}

pub struct Position(pub Point3<f32>);
pub struct Velocity(pub Vector3<f32>);

pub fn update_velocity(_world: &mut World, _delta: f32) {
    // for (e, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
    //     // FIXME: We're not moving in Y, because the lack of block collision makes mobs phase through the ground
    //     let temp_vel = Vector3::new(vel.0.x, 0., vel.0.z);
    //     pos.0 += temp_vel * delta;
    // }
}

pub fn update_interpolation(world: &mut World, delta: f32) {
    for (_e, (pos, interp)) in world
        .query::<(&mut Position, &mut InterpolatedPosition)>()
        .iter()
    {
        // interp.position = pos.0;
        if interp.delta <= 0. {
            continue;
        }

        let offset_mul = delta / interp.delta;
        if offset_mul <= 0. {
            continue;
        }

        let offset = (pos.0 - interp.position) * offset_mul;
        interp.position += offset;
        interp.delta -= delta;
    }
}

pub fn get_or_insert(world: &mut World, eid: i32) -> Entity {
    // println!("{:?}", NonZeroU32::new(eid as u32));
    let ent = Entity::from_bits(eid as u64 | (1 << 32)).unwrap();
    if world.get::<&Position>(ent).is_ok() {
        ent
    } else {
        world.spawn_at(
            ent,
            (
                Position(Point3::new(0., 0., 0.)),
                Velocity(Vector3::zero()),
                InterpolatedPosition {
                    position: Point3::new(0., 0., 0.),
                    delta: TICK_DELTA,
                },
            ),
        );
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
