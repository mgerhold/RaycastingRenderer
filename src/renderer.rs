use crate::{map::Map, player::Player, vector2::Vector2};
use raylib::prelude::*;

const TILE_SIZE: (i32, i32) = (10, 10);
const PLAYER_RADIUS: f32 = 5.0;

pub fn render(map: &Map, player: &Player, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    use crate::map::TileType::*;
    for y in 0..map.size().1 {
        for x in 0..map.size().0 {
            let draw_function = match map[(x, y)] {
                Occupied => <_>::draw_rectangle,
                Free => <_>::draw_rectangle_lines,
            };
            draw_function(
                draw_handle,
                (x as i32) * TILE_SIZE.0,
                (y as i32) * TILE_SIZE.1,
                TILE_SIZE.0,
                TILE_SIZE.1,
                Color::BLACK,
            );
        }
    }
    let player_position = (
        (player.position.x * (TILE_SIZE.0 as f64)).round() as i32,
        (player.position.y * (TILE_SIZE.1 as f64)).round() as i32,
    );
    let player_view_target = (
        ((player.position.x + player.direction.x) * (TILE_SIZE.0 as f64)).round() as i32,
        ((player.position.y + player.direction.y) * (TILE_SIZE.1 as f64)).round() as i32,
    );
    draw_handle.draw_line(
        player_position.0,
        player_position.1,
        player_view_target.0,
        player_view_target.1,
        Color::RED,
    );
    draw_handle.draw_circle(
        player_position.0,
        player_position.1,
        PLAYER_RADIUS,
        Color::RED,
    );
}
