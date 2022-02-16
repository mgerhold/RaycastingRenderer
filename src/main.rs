mod map;
mod player;
mod renderer;
mod vector2;
use map::{Map, TileType};
use player::Player;
use rand::prelude::*;
use raylib::core::math::Vector2 as RVector2;
use raylib::prelude::*;
use renderer::render;

pub struct Size {
    width: i32,
    height: i32,
}

pub const SCREEN_SIZE: Size = Size {
    width: 800,
    height: 600,
};

const MOVEMENT_SPEED: f64 = 4.0;
const ROTATION_SPEED: f64 = std::f64::consts::PI;

fn map_keys(
    raylib_handle: &raylib::RaylibHandle,
    keys: [raylib::consts::KeyboardKey; 2],
    values: [f64; 2],
) -> f64 {
    match (
        raylib_handle.is_key_down(keys[0]),
        raylib_handle.is_key_down(keys[1]),
    ) {
        (true, false) => values[0],
        (false, true) => values[1],
        _ => 0.0,
    }
}

fn create_map() -> Map {
    let mut map = Map::new((20, 15));
    let map_size = map.size();
    for x in 0..map_size.0 {
        map[(x, 0)] = TileType::Occupied;
        map[(x, map_size.1 - 1)] = TileType::Occupied;
    }
    for y in 1..map_size.1 - 1 {
        map[(0, y)] = TileType::Occupied;
        map[(map_size.0 - 1, y)] = TileType::Occupied;
    }
    let mut rng = rand::thread_rng();
    for x in 1..map_size.0 - 1 {
        for y in 1..map_size.1 - 1 {
            if x == 1 && y == 1 {
                continue;
            }
            if rng.gen::<f64>() >= 0.8 {
                map[(x, y)] = TileType::Occupied;
            }
        }
    }

    map
}

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .size(SCREEN_SIZE.width, SCREEN_SIZE.height)
        .title("Hello, World")
        .build();
    let camera = raylib::camera::Camera2D {
        offset: RVector2 {
            x: (SCREEN_SIZE.width as f32) / 2.0,
            y: (SCREEN_SIZE.height as f32) / 2.0,
        },
        target: RVector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    let map = create_map();
    let mut player = Player::default();
    player.position = vector2::Vector2::new(1.5, 1.5);

    use raylib::consts::KeyboardKey::*;
    while !raylib_handle.window_should_close() {
        let delta_time: f64 = raylib_handle.get_frame_time().into();

        let movement = map_keys(
            &raylib_handle,
            [KEY_W, KEY_S],
            [MOVEMENT_SPEED, -MOVEMENT_SPEED],
        );
        player.position += player.direction * movement * delta_time;

        let rotation = map_keys(
            &raylib_handle,
            [KEY_A, KEY_D],
            [-ROTATION_SPEED, ROTATION_SPEED],
        );
        player.direction = player.direction.rotated(rotation * delta_time);
        if raylib_handle.is_key_pressed(KEY_Q) {
            player.direction = player.direction.rotated(std::f64::consts::PI / 2.0);
        }
        if raylib_handle.is_key_pressed(KEY_E) {
            player.direction = player.direction.rotated(-std::f64::consts::PI / 2.0);
        }

        let mut draw_handle = raylib_handle.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);

        let mut mode2d_handle = draw_handle.begin_mode2D(camera);
        render(&map, &player, &mut mode2d_handle);
        drop(mode2d_handle);

        draw_handle.draw_fps(10, 10);
    }
}
