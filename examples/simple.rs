use bevy::prelude::*;
use bevy_xpbd::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(startup)
        .add_plugins(DefaultPlugins)
        .add_plugin(XPBDPlugin::default())
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let sphere = meshes.add(shape::Icosphere::default().try_into().unwrap());

    let white = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: sphere.clone(),
            material: white.clone(),
            ..Default::default()
        })
        .insert(ParticleBundle::new_with_pos_and_vel(Vec2::ZERO, Vec2::new(2.0, 0.0)));
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::Y
        ),
        ..default()
    });
}
