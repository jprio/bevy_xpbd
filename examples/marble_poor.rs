use bevy::{ time::FixedTimestep, prelude::* };
use bevy_xpbd::*;
use rand::random;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.9)))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(XPBDPlugin::default())
        .add_startup_system(startup)
        .add_system_set(
            SystemSet::new()
                .label(Step::SolvePositions)
                .after(Step::Integrate)
                .with_system(solve_pos)
                .with_system(solve_pos_statics)
        )
        .add_system_set(
            SystemSet::new()
                .label(Step::SolveVelocities)
                .after(Step::UpdateVelocities)
                .with_system(solve_vel)
                .with_system(solve_vel_statics)
        )

        .add_system(spawn_marbles)
        .add_system(despawn_marbles)
        .run();
}
fn solve_vel(
    query: Query<(&mut Vel, &PreSolveVel, &Pos, &Mass, &Restitution)>,
    contacts: Res<Contacts>
) {
    for (entity_a, entity_b) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, pos_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, pos_b, mass_b, restitution_b),
        ) = unsafe {
            // Ensure safety
            assert!(entity_a != entity_b);
            (query.get_unchecked(entity_a).unwrap(), query.get_unchecked(entity_b).unwrap())
        };
        let n = (pos_b.0 - pos_a.0).normalize();
        let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
        let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

        let relative_vel = vel_a.0 - vel_b.0;
        let normal_vel = Vec2::dot(relative_vel, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.0;

        let w_a = 1.0 / mass_a.0;
        let w_b = 1.0 / mass_b.0;
        let w_sum = w_a + w_b;

        vel_a.0 += (n * (-normal_vel - restitution * pre_solve_normal_vel) * w_a) / w_sum;
        vel_b.0 -= (n * (-normal_vel - restitution * pre_solve_normal_vel) * w_b) / w_sum;

        // TODO: make sure velocities are reflected and restitution/friction calculated
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
    Integrate,
    SolveVelocities,
    UpdateVelocities,
    SolvePositions,
}

#[derive(Resource, Debug)]
struct Materials {
    blue: Handle<StandardMaterial>,
}
#[derive(Resource, Debug)]
struct Meshes {
    sphere: Handle<Mesh>,
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let sphere = meshes.add(
        Mesh::from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 8,
        })
    );

    let blue = materials.add(StandardMaterial {
        base_color: Color::rgb(0.4, 0.4, 0.6),
        unlit: true,
        ..Default::default()
    });

    let radius = 13.0;
    commands
        .spawn(PbrBundle {
            mesh: sphere.clone(),
            material: blue.clone(),
            transform: Transform {
                scale: Vec3::splat(radius),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StaticColliderBundle {
            pos: Pos(Vec2::new(0.0, -radius - 2.0)),
            collider: CircleCollider { radius },
            ..Default::default()
        });

    commands.insert_resource(Meshes { sphere });
    commands.insert_resource(Materials { blue });
    commands.insert_resource(Meshes {
        sphere: meshes.add(
            Mesh::from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 4,
            })
        ),
    });

    commands.insert_resource(Materials {
        blue: materials.add(StandardMaterial {
            base_color: Color::rgb(0.4, 0.4, 0.6),
            unlit: true,
            ..Default::default()
        }),
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::Y
        ),
        ..default()
    });
}

fn spawn_marbles(mut commands: Commands, materials: Res<Materials>, meshes: Res<Meshes>) {
    let radius = 0.1;
    let pos = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * 0.5 + Vec2::Y * 3.0;
    let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    commands
        .spawn(PbrBundle {
            mesh: meshes.sphere.clone(),
            material: materials.blue.clone(),
            transform: Transform {
                scale: Vec3::splat(radius),
                translation: pos.extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ParticleBundle {
            collider: CircleCollider { radius },
            ..ParticleBundle::new_with_pos_and_vel(pos, vel)
        });
}

fn despawn_marbles(mut commands: Commands, query: Query<(Entity, &Pos)>) {
    for (entity, pos) in query.iter() {
        if pos.0.y < -20.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn solve_pos_statics(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>, // <-- new
    statics: Query<(Entity, &Pos, &CircleCollider), Without<Mass>>, // <-- new
    mut contacts: ResMut<StaticContacts> // <-- new
) {
    contacts.0.clear(); // <-- new
    for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, circle_b) in statics.iter() {
            let ab = pos_b.0 - pos_a.0;
            let combined_radius = circle_a.radius + circle_b.radius;
            let ab_sqr_len = ab.length_squared();
            if ab_sqr_len < combined_radius * combined_radius {
                let ab_length = ab_sqr_len.sqrt();
                let penetration_depth = combined_radius - ab_length;
                let n = ab / ab_length;
                pos_a.0 -= n * penetration_depth;
                contacts.0.push((entity_a, entity_b)); // <-- new
            }
        }
    }
}
fn solve_vel_statics(
    mut dynamics: Query<(&mut Vel, &PreSolveVel, &Pos, &Restitution), With<Mass>>,
    statics: Query<(&Pos, &Restitution), Without<Mass>>,
    contacts: Res<StaticContacts>
) {
    for (entity_a, entity_b) in contacts.0.iter().cloned() {
        let (mut vel_a, pre_solve_vel_a, pos_a, restitution_a) = dynamics
            .get_mut(entity_a)
            .unwrap();
        let (pos_b, restitution_b) = statics.get(entity_b).unwrap();
        let ba = pos_a.0 - pos_b.0;
        let n = ba.normalize();
        let pre_solve_normal_vel = Vec2::dot(pre_solve_vel_a.0, n);
        let normal_vel = Vec2::dot(vel_a.0, n);
        let restitution = (restitution_a.0 + restitution_b.0) / 2.0;
        vel_a.0 += n * (-normal_vel - restitution * pre_solve_normal_vel);
    }
}
fn solve_pos(
    mut query: Query<(Entity, &mut Pos, &CircleCollider, &Mass)>,
    mut contacts: ResMut<Contacts>
) {
    contacts.0.clear();
    let mut iter = query.iter_combinations_mut();
    while
        let Some(
            [
                (entity_a, mut pos_a, circle_a, mass_a),
                (entity_b, mut pos_b, circle_b, mass_b),
            ],
        ) = iter.fetch_next()
    {
        let ab = pos_b.0 - pos_a.0;
        let combined_radius = circle_a.radius + circle_b.radius;
        let ab_sqr_len = ab.length_squared();
        if ab_sqr_len < combined_radius * combined_radius {
            let ab_length = ab_sqr_len.sqrt();
            let penetration_depth = combined_radius - ab_length;
            let n = ab / ab_length;

            let w_a = 1.0 / mass_a.0;
            let w_b = 1.0 / mass_b.0;
            let w_sum = w_a + w_b;

            pos_a.0 -= (n * penetration_depth * w_a) / w_sum;
            pos_b.0 += (n * penetration_depth * w_b) / w_sum;
            contacts.0.push((entity_a, entity_b));
        }
    }
}
