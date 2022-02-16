use crate::{map::Map, map::TileType, player::Player, vector2::Vector2};
use raylib::prelude::*;
use std::mem::swap;

const TILE_SIZE: (i32, i32) = (10, 10);
const PLAYER_RADIUS: f32 = 5.0;

struct Ray {
    position: Vector2,
    direction: Vector2,
}

impl Ray {
    fn new(position: Vector2, direction: Vector2) -> Ray {
        Ray {
            position,
            direction: direction
                .normalized()
                .expect("Direction must not be a null vector"),
        }
    }
}

pub fn render(map: &Map, player: &Player, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    render_ground(draw_handle);
    render_map(map, player, draw_handle);
    render_minimap(map, player, draw_handle);
}

fn collide_vertically(ray: &Ray, x: f64) -> f64 {
    (x - ray.position.x) / ray.direction.x
}

fn collide_horizontally(ray: &Ray, y: f64) -> f64 {
    (y - ray.position.y) / ray.direction.y
}

fn collision_distance(ray: &Ray, coordinates: (usize, usize)) -> Option<f64> {
    if ray
        .direction
        .dot(Vector2::new(coordinates.0 as f64, coordinates.1 as f64) - ray.position)
        < 0.0
    {
        return None;
    }

    let x_min = coordinates.0 as f64;
    let x_max = x_min + 1.0;
    let y_min = coordinates.1 as f64;
    let y_max = y_min + 1.0;
    let mut r_x_min = collide_vertically(ray, x_min);
    let mut r_x_max = collide_vertically(ray, x_max);
    if r_x_min > r_x_max {
        swap(&mut r_x_min, &mut r_x_max);
    }
    let mut r_y_min = collide_horizontally(ray, y_min);
    let mut r_y_max = collide_horizontally(ray, y_max);
    if r_y_min > r_y_max {
        swap(&mut r_y_min, &mut r_y_max);
    }
    if r_x_min < 0.0 && r_x_max < 0.0 && r_y_min < 0.0 && r_y_max < 0.0 {
        return None;
    }
    let is_overlapping = (r_y_min >= r_x_min && r_y_min <= r_x_max)
        || (r_y_max >= r_x_min && r_y_max <= r_x_max)
        || (r_x_min >= r_y_min && r_x_min <= r_y_max)
        || (r_x_max >= r_y_min && r_x_max <= r_y_max);
    if !is_overlapping {
        return None;
    }
    Some(f64::max(r_x_min, r_y_min))
}

fn cast_ray(ray: &Ray, map: &Map) -> Option<f64> {
    let mut min_distance = None;
    for y in 0..map.size().1 {
        for x in 0..map.size().0 {
            if map[(x, y)] == TileType::Free {
                continue;
            }
            if let Some(current_distance) = collision_distance(ray, (x, y)) {
                min_distance = match min_distance {
                    Some(old_minimum) => Some(f64::min(old_minimum, current_distance)),
                    None => Some(current_distance),
                }
            }
        }
    }
    return min_distance;
}

fn create_ray(player: &Player, normalized_x: f64) -> Ray {
    let near_plane = 1.6;
    let normal = Vector2::new(-player.direction.y, player.direction.x);
    let ray_direction = near_plane * player.direction + (-0.5 + normalized_x) * normal;
    Ray::new(player.position, ray_direction)
}

fn render_map(map: &Map, player: &Player, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    use crate::SCREEN_SIZE;
    let min_height = SCREEN_SIZE.height / 5;
    //let far_plane = 25.0;
    for x in 0..SCREEN_SIZE.width {
        let normalized_x = x as f64 / (SCREEN_SIZE.width - 1) as f64;
        let ray = create_ray(player, normalized_x);
        let distance = cast_ray(&ray, map);
        let height = match distance {
            None => continue,
            // Some(distance) if distance > far_plane => continue,
            Some(distance) if distance <= 0.0 => SCREEN_SIZE.height,
            Some(distance) => (2000.0 / distance) as i32,
        };
        let brightness = ((height.clamp(min_height, SCREEN_SIZE.height) - min_height) as f64
            / (SCREEN_SIZE.height - min_height) as f64
            * 255.0) as u8;
        let color = Color::new(brightness, brightness, brightness, 255);
        draw_handle.draw_line(
            x - SCREEN_SIZE.width / 2 + 1,
            -height / 2,
            x - SCREEN_SIZE.width / 2 + 1,
            height / 2,
            color,
        );
    }
}

fn render_ground(draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    use crate::SCREEN_SIZE;
    draw_handle.draw_rectangle(
        -SCREEN_SIZE.width / 2,
        0,
        SCREEN_SIZE.width,
        SCREEN_SIZE.height / 2,
        Color::GRAY,
    );
}

fn render_minimap(map: &Map, player: &Player, draw_handle: &mut RaylibMode2D<RaylibDrawHandle>) {
    use crate::map::TileType::*;
    for y in 0..map.size().1 {
        for x in 0..map.size().0 {
            let draw_function = match map[(x, y)] {
                Occupied => <_>::draw_rectangle,
                Free => <_>::draw_rectangle_lines,
            };
            let ray = Ray::new(player.position, player.direction);
            let distance = collision_distance(&ray, (x, y));
            draw_function(
                draw_handle,
                (x as i32) * TILE_SIZE.0,
                (y as i32) * TILE_SIZE.1,
                TILE_SIZE.0,
                TILE_SIZE.1,
                match distance {
                    Some(_) if map[(x, y)] == Occupied => Color::RED,
                    _ => Color::BLACK,
                },
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
