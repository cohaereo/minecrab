use cgmath::{Point3, Vector3};
use collision::{Aabb, Aabb3};

use crate::world::ChunkManager;

pub fn is_solid(block: u8) -> bool {
    match block {
        0 | 6 | 8 | 9 | 10 | 11 | 27 | 28 | 30 | 31 | 32 | 36 | 37 | 38 | 39 | 40 | 50 | 51
        | 55 | 59 | 63 | 66 | 68 | 69 | 70 | 72 | 75 | 76 | 77 | 83 | 90 | 104 | 105 | 106
        | 115 | 119 | 131 | 132 | 141 | 142 | 143 | 147 | 148 | 157 | 175 => false,
        _ => true,
    }
}

pub fn calculate_next_player_pos(
    world: &ChunkManager,
    position: Point3<f32>,
    velocity: Vector3<f32>,
) -> Point3<f32> {
    let position = position + velocity;
    let mut bounds = Aabb3::<f32>::new(position, position + Vector3::new(0.6, 1.8, 0.6));

    let min = Point3::new(
        bounds.min.x as i32 - 1,
        bounds.min.y as i32 - 1,
        bounds.min.z as i32 - 1,
    );

    let max = Point3::new(
        bounds.max.x as i32 + 1,
        bounds.max.y as i32 + 1,
        bounds.max.z as i32 + 1,
    );

    for y in min.y..max.y {
        for z in min.z..max.z {
            for x in min.x..max.x {
                let block = world.get_block(x, y, z);

                if is_solid(block) {
                    let bb = Aabb3::new(Point3::new(0., 0., 0.), Point3::new(1., 1., 1.));
                    let bb = bb.add_v(Vector3::new(x as f32, y as f32, z as f32));
                    if collides(&bb, &bounds) {
                        move_out_of(&mut bounds, &bb, velocity);
                    }
                }
            }
        }
    }

    bounds.min
}

fn collides(a: &Aabb3<f32>, b: &Aabb3<f32>) -> bool {
    !(b.min.x >= a.max.x
        || b.max.x <= a.min.x
        || b.min.y >= a.max.y
        || b.max.y <= a.min.y
        || b.min.z >= a.max.z
        || b.max.z <= a.min.z)
}

fn move_out_of(this: &mut Aabb3<f32>, other: &Aabb3<f32>, dir: Vector3<f32>) {
    if dir.x != 0.0 {
        if dir.x > 0.0 {
            let ox = this.max.x;
            this.max.x = other.min.x - 0.0001;
            this.min.x += this.max.x - ox;
        } else {
            let ox = this.min.x;
            this.min.x = other.max.x + 0.0001;
            this.max.x += this.min.x - ox;
        }
    }

    if dir.y != 0.0 {
        if dir.y > 0.0 {
            let oy = this.max.y;
            this.max.y = other.min.y - 0.0001;
            this.min.y += this.max.y - oy;
        } else {
            let oy = this.min.y;
            this.min.y = other.max.y + 0.0001;
            this.max.y += this.min.y - oy;
        }
    }
    if dir.z != 0.0 {
        if dir.z > 0.0 {
            let oz = this.max.z;
            this.max.z = other.min.z - 0.0001;
            this.min.z += this.max.z - oz;
        } else {
            let oz = this.min.z;
            this.min.z = other.max.z + 0.0001;
            this.max.z += this.min.z - oz;
        }
    }
}
