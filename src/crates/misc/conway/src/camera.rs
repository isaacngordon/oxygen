use bevy::prelude::*;

pub const CAMERA_MOVE_SPEED: f32 = 15.0;
pub const CAMERA_ZOOM_SPEED: f32 = 0.01;

#[derive(Resource)]
pub struct GlobalDefaults{
    pub window_width: f32,
    pub window_height: f32,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraMovement {
    plane_speed: Vec3,
    zoom_speed: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_movement_system, camera_zoom_system));
    }
}

fn camera_setup(mut commands: Commands, global_defaults: Res<GlobalDefaults>) {
    commands
        .spawn(Camera2dBundle{
            // custom transform to center the camera on the screen
            transform: Transform{
                translation: Vec3::new(global_defaults.window_width / 2.0, global_defaults.window_height / 2.0, 0.0),
                scale: Vec3::splat(10.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera)
        .insert(CameraMovement {
            plane_speed: Vec3::ZERO,
            zoom_speed: 0.0,
        });
}

fn camera_movement_system(
    mut camera: Query<(&mut Transform, &mut CameraMovement), With<MainCamera>>,
    global_defaults: Res<GlobalDefaults>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut move_direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        move_direction.y = CAMERA_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::A) {
        move_direction.x = -CAMERA_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::S) {
        move_direction.y = -CAMERA_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) {
        move_direction.x = CAMERA_MOVE_SPEED;
    }

    let move_direction = move_direction.normalize_or_zero();
    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform found on camera MainCamera");

    movement.plane_speed = (move_direction);

    transform.translation += movement.plane_speed;

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.plane_speed = Vec3::ZERO;
        transform.translation = Vec3::new(global_defaults.window_width / 2.0, global_defaults.window_height / 2.0, 0.0)
    }
}

fn camera_zoom_system(
    mut camera: Query<(&mut Transform, &mut CameraMovement), With<MainCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut zoom_direction = 0.0;
    if keyboard_input.pressed(KeyCode::Q) {
        zoom_direction = CAMERA_ZOOM_SPEED;
    }
    if keyboard_input.pressed(KeyCode::E) {
        zoom_direction = -CAMERA_ZOOM_SPEED;
    }

    let (mut transform, mut movement) = camera
        .iter_mut()
        .next()
        .expect("No transform found on camera MainCamera");

    movement.zoom_speed = zoom_direction;

    transform.scale += Vec3::splat(movement.zoom_speed);

    if keyboard_input.just_pressed(KeyCode::Space) {
        movement.zoom_speed = 0.0;
        transform.scale = Vec3::splat(10.0);
    }
}
