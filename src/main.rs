use bevy::{
    input::mouse::MouseMotion,
    window::CursorGrabMode,
    prelude::*
};

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct Camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am the Window!".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, camera_movement, mouse))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>
) {
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb_u8(255, 0, 0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
                .with_rotation(Quat::from_rotation_y(f32::to_radians(45.0))),
            ..default()
        },
        Cube
    ));

    // platform
    commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(50.0, 0.2, 50.0)),
            // material: materials.add(Color::rgb_u8(90, 90, 90)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(server.load("grass.png")),
                perceptual_roughness: 1.0,
                reflectance: 0.2,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.8, 0.0),
            ..default()
        }
    );

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-5.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
    
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Camera
    ));
}

fn rotate_cube(
    mut query: Query<&mut Transform, With<Cube>>,
    timer: Res<Time>,
) {
    // rotate cube
    let mut cube_transform = query.single_mut();
    cube_transform.rotate_y(0.2 * std::f32::consts::TAU * timer.delta_seconds())
}

fn mouse(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut window_query: Query<&mut Window>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    let mut window = window_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    if mouse_button.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }

    // if cursor is locked then allow mouse looking
    if window.cursor.grab_mode == CursorGrabMode::Locked {
        for event in mouse_motion_events.read() {
            // because the movement and looking is done by the camera, looking both
            // vertically and horizontally causes a lot of problems, only use horizontal
            // looking for now until the camera and body is seperate.
            let rotation_x = -event.delta.x / 10.0;
            // let rotation_y = -event.delta.y / 10.0;

            camera_transform.rotate_y(f32::to_radians(rotation_x));
            // camera_transform.rotate_z(f32::to_radians(rotation_y));
        }
    }
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    timer: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut camera_transform = query.single_mut();

    /*
    let rotation ;

    if keyboard.pressed(KeyCode::KeyA) {
        rotation = 1.0;
    } else if keyboard.pressed(KeyCode::KeyD) {
        rotation = -1.0;
    } else {
        rotation = 0.0;
    }

    camera_transform.rotate_y(f32::to_radians(rotation));
    */

    let forward_direction = camera_transform.rotation * Vec3::Z;
    let sideways_direction = camera_transform.rotation * Vec3::X;

    if keyboard.pressed(KeyCode::KeyW) {
        camera_transform.translation -= forward_direction * 5.0 * timer.delta_seconds();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera_transform.translation += forward_direction * 5.0 * timer.delta_seconds();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        camera_transform.translation -= sideways_direction * 5.0 * timer.delta_seconds();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera_transform.translation += sideways_direction * 5.0 * timer.delta_seconds();
    }
}