use bevy::prelude::*;

use crate::*;

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pos: Pos,
    prev_pos: PrevPos,
    mass: Mass,
    collider: CircleCollider,
    vel: Vel,
    pre_solve_vel: PreSolveVel,
    restitution: Restitution,
}

impl ParticleBundle {
    pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos: Pos(pos),
            prev_pos: PrevPos(pos - vel * DELTA_TIME),
            vel: Vel(vel),
            ..Default::default()
        }
    }
}
