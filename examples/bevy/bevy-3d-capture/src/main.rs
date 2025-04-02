//! A simple glTF scene viewer made with Bevy.
//!
//! Just run `cargo run --release --example scene_viewer /path/to/model.gltf`,
//! replacing the path as appropriate.
//! In case of multiple scenes, you can select which to display by adapting the file path: `/path/to/model.gltf#Scene1`.
//! With no arguments it will load the `FlightHelmet` glTF model from the repository assets subdirectory.

use bevy::{
    math::Vec3A,
    prelude::*,
    render::{
        camera::RenderTarget,
        primitives::{Aabb, Sphere},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        RenderPlugin,
    },
    window::WindowPlugin,
};

mod camera_controller_plugin;
mod scene_viewer_plugin;

use camera_controller_plugin::{CameraController, CameraControllerPlugin};
use scene_viewer_plugin::{SceneHandle, SceneViewerPlugin};

use bevy_image_export::{ImageExportPlugin, ImageExportSettings, ImageExportSource};

use crate::camera_controller_plugin::get_dolly_points;

const WIDTH_HEIGHT_512: f32 = 512f32;

fn main() {
    let export_plugin = ImageExportPlugin::default();
    let export_threads = export_plugin.threads.clone();

    let mut app = App::new();
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    })
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "glb2png".to_string(),
                    resolution: (WIDTH_HEIGHT_512, WIDTH_HEIGHT_512).into(),
                    transparent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
                ..default()
            })
            // Capture
            .set(RenderPlugin {
                synchronous_pipeline_compilation: true,
                ..default()
            }),
        export_plugin,
    ))
    .add_plugins(CameraControllerPlugin)
    .add_plugins(SceneViewerPlugin)
    .add_systems(Startup, setup)
    .add_systems(Startup, setup_scene_after_load)
    .run();

    // This line is optional but recommended.
    // It blocks the main thread until all image files have been saved successfully.
    export_threads.finish();
}

fn parse_scene(scene_path: String) -> (String, usize) {
    if scene_path.contains('#') {
        let gltf_and_scene = scene_path.split('#').collect::<Vec<_>>();
        if let Some((last, path)) = gltf_and_scene.split_last() {
            if let Some(index) = last
                .strip_prefix("Scene")
                .and_then(|index| index.parse::<usize>().ok())
            {
                return (path.join("#"), index);
            }
        }
    }
    (scene_path, 0)
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "assets/models/pistol.glb".to_string());
    info!("Loading {}", scene_path);
    let (file_path, scene_index) = parse_scene(scene_path);

    commands.insert_resource(SceneHandle::new(asset_server.load(file_path), scene_index));
}

struct CenterAndSize {
    size: f32,
    center: Vec3A,
}

fn get_center(
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
) -> Option<CenterAndSize> {
    if meshes.iter().any(|(_, maybe_aabb)| maybe_aabb.is_none()) {
        return None;
    }

    let mut min = Vec3A::splat(f32::MAX);
    let mut max = Vec3A::splat(f32::MIN);
    for (transform, maybe_aabb) in &meshes {
        let aabb = maybe_aabb.unwrap();
        // If the Aabb had not been rotated, applying the non-uniform scale would produce the
        // correct bounds. However, it could very well be rotated and so we first convert to
        // a Sphere, and then back to an Aabb to find the conservative min and max points.
        let sphere = Sphere {
            center: Vec3A::from(transform.transform_point(Vec3::from(aabb.center))),
            radius: transform.radius_vec3a(aabb.half_extents),
        };
        let aabb = Aabb::from(sphere);
        min = min.min(aabb.min());
        max = max.max(aabb.max());
    }

    let size = (max - min).length();
    let aabb = Aabb::from_min_max(Vec3::from(min), Vec3::from(max));

    Some(CenterAndSize {
        size,
        center: aabb.center,
    })
}

#[derive(Component)]
struct ExportBundleMarker;

fn setup_scene_after_load(
    mut commands: Commands,
    mut setup: Local<bool>,
    mut scene_handle: ResMut<SceneHandle>,
    asset_server: Res<AssetServer>,
    meshes: Query<(&GlobalTransform, Option<&Aabb>), With<Handle<Mesh>>>,
    mut images: ResMut<Assets<Image>>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    if scene_handle.is_loaded && !*setup {
        *setup = true;
        // Find an approximate bounding box of the scene from its meshes
        if meshes.iter().any(|(_, maybe_aabb)| maybe_aabb.is_none()) {
            return;
        }

        let aabb_info = match get_center(meshes) {
            Some(aabb_info) => aabb_info,
            None => return,
        };

        info!("Spawning a controllable 3D perspective camera");
        let mut projection = PerspectiveProjection::default();
        projection.far = projection.far.max(aabb_info.size * 10.0);

        let camera_controller = CameraController {
            focus: Vec3::from(aabb_info.center),
            targets: get_dolly_points(1.0, 8, 8),
            ..default()
        };

        // Display the controls of the scene viewer
        info!("{}", camera_controller);
        info!("{}", *scene_handle);

        // Create an output texture.
        let output_texture_handle = {
            let size = Extent3d {
                width: WIDTH_HEIGHT_512 as u32,
                height: WIDTH_HEIGHT_512 as u32,
                ..default()
            };
            let mut export_texture = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Rgba8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::COPY_DST
                        | TextureUsages::COPY_SRC
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            };
            export_texture.resize(size);

            images.add(export_texture)
        };

        commands
            .spawn((
                Camera3d {
                    projection: projection.into(),
                    transform: Transform::from_translation(
                        Vec3::from(aabb_info.center) + aabb_info.size * Vec3::new(1.0, 0.0, 1.0),
                    )
                    .looking_at(Vec3::from(aabb_info.center), Vec3::Y),
                    camera: Camera {
                        is_active: false,
                        ..default()
                    },

                    ..default()
                },
                camera_controller,
            ))
            .with_children(|parent| {
                parent.spawn(Camera3d {
                    camera: Camera {
                        // Connect the output texture to a camera as a RenderTarget.
                        target: RenderTarget::Image(output_texture_handle.clone()),
                        ..default()
                    },
                    ..default()
                });
            });

        // Spawn the ImageExportBundle to initiate the export of the output texture.
        commands.spawn((
            ImageExportBundle {
                source: export_sources.add(output_texture_handle),
                settings: ImageExportSettings {
                    // Frames will be saved to "./out/[#####].png".
                    output_dir: "out".into(),
                    // Choose "exr" for HDR renders.
                    extension: "png".into(),
                },
            },
            ExportBundleMarker,
        ));

        // Spawn a default light if the scene does not have one
        if !scene_handle.has_light {
            info!("Spawning a directional light");
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(
                    Vec3::from(aabb_info.center) + aabb_info.size * Vec3::new(1.0, 0.0, 1.0),
                )
                .looking_at(Vec3::from(aabb_info.center), Vec3::Y),
                ..default()
            });
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(
                    Vec3::from(aabb_info.center) + aabb_info.size * Vec3::new(-1.0, 0.0, 1.0),
                )
                .looking_at(Vec3::from(aabb_info.center), Vec3::Y),
                ..default()
            });
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(
                    Vec3::from(aabb_info.center) + aabb_info.size * Vec3::new(1.0, 0.0, -1.0),
                )
                .looking_at(Vec3::from(aabb_info.center), Vec3::Y),
                ..default()
            });
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(
                    Vec3::from(aabb_info.center) + aabb_info.size * Vec3::new(-1.0, 0.0, -1.0),
                )
                .looking_at(Vec3::from(aabb_info.center), Vec3::Y),
                ..default()
            });

            scene_handle.has_light = true;
        }
    }
}

// fn setup_dolly(
//     mut commands: Commands,
//     mut wireframe_config: ResMut<WireframeConfig>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(
//                 shape::UVSphere {
//                     radius: 1.0,
//                     sectors: 8,
//                     stacks: 8,
//                 }
//                 .into(),
//             ),
//             material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
//             ..default()
//         },
//         // This enables wireframe drawing on this entity
//         Wireframe,
//     ));
// }

// fn setup_dolly(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     const COUNT: usize = 6;
//     let position_range = -1.0..1.0;
//     let radius_range = 0.0..0.5;
//     let pos_len = position_range.end - position_range.start;
//     let radius_len = radius_range.end - radius_range.start;
//     let mesh = meshes.add(Mesh::from(shape::UVSphere {
//         sectors: 128,
//         stacks: 64,
//         ..default()
//     }));

//     for i in 0..COUNT {
//         let percent = i as f32 / COUNT as f32;
//         let radius = radius_range.start + percent * radius_len;

//         // sphere light
//         commands
//             .spawn(PbrBundle {
//                 mesh: mesh.clone(),
//                 material: materials.add(StandardMaterial {
//                     base_color: Color::rgb(0.5, 0.5, 1.0),
//                     unlit: true,
//                     ..default()
//                 }),
//                 transform: Transform::from_xyz(position_range.start + percent * pos_len, 0.6, 0.0)
//                     .with_scale(Vec3::splat(radius)),
//                 ..default()
//             })
//             .with_children(|children| {
//                 children.spawn(PointLightBundle {
//                     point_light: PointLight {
//                         intensity: 1500.0,
//                         radius,
//                         color: Color::rgb(0.2, 0.2, 1.0),
//                         ..default()
//                     },
//                     ..default()
//                 });
//             });
//     }
// }
