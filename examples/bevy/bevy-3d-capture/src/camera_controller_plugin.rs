//! A freecam-style camera controller plugin.
//! To use in your own application:
//! - Copy the code for the `CameraControllerPlugin` and add the plugin to your App.
//! - Attach the `CameraController` component to an entity with a `Camera3dBundle`.

use bevy::window::CursorGrabMode;
use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::*;
use std::fmt;

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly 1.0 / 180.0,
/// but I'm guessing it is a misunderstanding between degrees/radians and then sticking with
/// it because it felt nice.
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    // pub key_forward: KeyCode,
    // pub key_back: KeyCode,
    // pub key_left: KeyCode,
    // pub key_right: KeyCode,
    // pub key_up: KeyCode,
    // pub key_down: KeyCode,
    // pub key_run: KeyCode,
    // pub mouse_key_enable_mouse: MouseButton,
    // pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub focus: Vec3,
    pub targets: Vec<Vec3>,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            // key_forward: KeyCode::W,
            // key_back: KeyCode::S,
            // key_left: KeyCode::A,
            // key_right: KeyCode::D,
            // key_up: KeyCode::E,
            // key_down: KeyCode::Q,
            // key_run: KeyCode::LShift,
            // mouse_key_enable_mouse: MouseButton::Left,
            // keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 5.0,
            run_speed: 15.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            focus: Vec3::ZERO,
            targets: vec![Vec3::ZERO],
        }
    }
}

impl fmt::Display for CameraController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO: control")
        //             "
        // Freecam Controls:
        //     MOUSE\t- Move camera orientation
        //     {:?}/{:?}\t- Enable mouse movement
        //     {:?}{:?}\t- forward/backward
        //     {:?}{:?}\t- strafe left/right
        //     {:?}\t- 'run'
        //     {:?}\t- up
        //     {:?}\t- down",
        //             self.mouse_key_enable_mouse,
        //             self.keyboard_key_enable_mouse,
        //             self.key_forward,
        //             self.key_back,
        //             self.key_left,
        //             self.key_right,
        //             self.key_run,
        //             self.key_up,
        //             self.key_down
        //         )
    }
}

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller);
    }
}

#[derive(Component)]
pub struct ExportBundleMarker;

fn camera_controller(
    time: Res<Time>,
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    // mouse_button_input: Res<Input<MouseButton>>,
    // key_input: Res<Input<KeyCode>>,
    mut move_toggled: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mut frame: Local<usize>,
    mut commands: Commands,
    export_bundles: Query<Entity, With<ExportBundleMarker>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        if *frame < options.targets.len() - 1 {
            *frame += 1;
        } else {
            *frame = 0;
            commands.entity(export_bundles.single()).despawn();

            println!("Done!");
            std::process::exit(1);
        }

        let dolly = options.targets[*frame];
        transform.rotate_around(
            options.focus,
            Quat::from_euler(EulerRot::XYZ, dolly.x, dolly.y, dolly.z),
        );
    }
}

use std::f32::consts::PI;

pub fn get_dolly_points(radius: f32, sectors: usize, stacks: usize) -> Vec<Vec3> {
    let sector_step = 2. * PI / sectors as f32;
    let stack_step = PI / stacks as f32;

    let mut point3ds = Vec::with_capacity(stacks * sectors);

    for i in 0..stacks + 1 {
        let stack_angle = PI / 2. - (i as f32) * stack_step;
        let xy = radius * stack_angle.cos();
        let z = radius * stack_angle.sin();

        for j in 0..sectors + 1 {
            let sector_angle = (j as f32) * sector_step;
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            point3ds.push(Vec3::new(x, y, z))
        }
    }

    point3ds
}
