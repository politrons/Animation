use bevy::prelude::*;

// Solo importamos wasm-bindgen en WebAssembly
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Cuando NO compilamos para wasm, necesitamos `main`.
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    start();
}

// Cuando SÍ estamos en wasm, definimos `start` como el punto de entrada.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Añadimos una cámara 2D
    commands.spawn(Camera2dBundle::default());

    // Cargamos la imagen (asegúrate de ponerla en tu carpeta assets)
    let texture = asset_server.load("sprites/foo.png");

    // Creas el sprite
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            scale: Vec3::splat(0.5), // Ajusta el tamaño del sprite
            ..Default::default()
        },
        ..Default::default()
    });
}

fn rotate_system(time: Res<Time>, mut query: Query<&mut Transform, With<Sprite>>) {
    // Para cada sprite encontrado, lo rotamos sobre el eje Z
    for mut transform in &mut query {
        transform.rotation *= Quat::from_rotation_z(1.0 * time.delta_seconds());
    }
}
