use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Pos(pub Vec2);

#[derive(Component, Debug, Default)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0) // Default to 1 kg
    }
}
#[derive(Component, Debug, Default)]
pub struct PreSolveVel(pub Vec2);

#[derive(Resource, Debug, Default)]
pub struct Contacts(pub Vec<(Entity, Entity)>);
#[derive(Component, Debug, Default)]
pub struct Vel(pub Vec2);

#[derive(Debug, Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}
#[derive(Bundle, Default)]
pub struct StaticColliderBundle {
    pub pos: Pos,
    pub collider: CircleCollider,
    pub restitution: Restitution,
}
#[derive(Default, Debug, Resource)]
pub struct StaticContacts(pub Vec<(Entity, Entity)>);
#[derive(Component, Debug)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.3)
    }
}
