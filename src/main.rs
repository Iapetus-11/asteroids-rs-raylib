use math::{degrees_to_radians, radians_to_degrees};
use obstacles::{collide_obstacle, draw_obstacle, gravity_obstacle, Obstacle};
use raylib::prelude::*;

mod math;
mod obstacles;

const MAP_WIDTH: f32 = 8192.0;
const MAP_HEIGHT: f32 = 8192.0;

#[derive(Debug)]
struct State {
    ship_vel: Vector2,
    ship_pos: Vector2,
    ship_rot: f32,
    ship_rot_vel: f32,
    ship_mov_pressed: bool,
    obstacles: Vec<Obstacle>,
}

impl State {
    pub fn new() -> Self {
        State {
            ship_vel: Vector2 { x: 0.0, y: 0.0 },
            ship_pos: Vector2 { x: 0.0, y: 0.0 },
            ship_rot: 0.0,
            ship_rot_vel: 0.0,
            ship_mov_pressed: false,
            obstacles: vec![],
        }
    }
}

fn controls(rl: &mut RaylibHandle, thread: &RaylibThread, state: &mut State) {
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
        state.ship_mov_pressed = true;
        state.ship_vel += Vector2::new(0.0, 0.5).rotated(state.ship_rot);
    } else {
        state.ship_mov_pressed = false;
    }

    state.ship_vel.x = state.ship_vel.x.abs().min(10.0).copysign(state.ship_vel.x);
    state.ship_vel.y = state.ship_vel.y.abs().min(10.0).copysign(state.ship_vel.y);

    state.ship_vel /= 1.0005;

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
        state.ship_rot_vel -= 0.02125;
    }

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
        state.ship_rot_vel += 0.02125;
    }

    state.ship_rot_vel = state
        .ship_rot_vel
        .abs()
        .min(0.25)
        .copysign(state.ship_rot_vel)
        / 1.05;
}

fn calculate(rl: &mut RaylibHandle, state: &mut State) {
    let obstacles_clone = state.obstacles.to_vec();

    state.ship_pos.x -= state.ship_vel.x;
    state.ship_pos.y -= state.ship_vel.y;

    let avg_velocity = {
        // let rotated_velocity = state.ship_vel.rotated(state.ship_rot);
        (state.ship_vel.x.abs() + state.ship_vel.y.abs()) / 2.0
    };

    let ship_rot_cos = state.ship_rot.cos();
    let ship_rot_sin = state.ship_rot.sin();

    if state.ship_pos.x + 10.0 >= MAP_WIDTH {
        state.ship_pos.x = MAP_WIDTH - 10.0;
        state.ship_vel.x = -(state.ship_vel.x / 2.0);

        state.ship_rot_vel -= ship_rot_cos / 2.0 * (avg_velocity / 20.0);
    }

    if state.ship_pos.x - 10.0 <= 0.0 {
        state.ship_pos.x = 10.0;
        state.ship_vel.x = -(state.ship_vel.x / 2.0);

        state.ship_rot_vel += ship_rot_cos / 2.0 * (avg_velocity / 20.0);
    }

    if state.ship_pos.y + 10.0 >= MAP_HEIGHT {
        state.ship_pos.y = MAP_HEIGHT - 10.0;
        state.ship_vel.y = -(state.ship_vel.y / 2.0);

        state.ship_rot_vel -= ship_rot_sin / 2.0 * (avg_velocity / 12.0);
    }

    if state.ship_pos.y - 10.0 <= 0.0 {
        state.ship_pos.y = 10.0;
        state.ship_vel.y = -(state.ship_vel.y / 2.0);

        state.ship_rot_vel += ship_rot_sin / 2.0 * (avg_velocity / 20.0);
    }

    state.ship_rot += state.ship_rot_vel;

    for obstacle in &obstacles_clone {
        gravity_obstacle(obstacle, state);
    }

    for obstacle in &obstacles_clone {
        collide_obstacle(obstacle, state);
    }
}

fn draw_ship(
    d: &mut RaylibDrawHandle,
    camera: &mut Camera2D,
    screen_center: Vector2,
    state: &mut State,
) {
    // let mut d = d.begin_mode2D(camera as &Camera2D);

    let mut v1: Vector2;
    let mut v2: Vector2;
    let mut v3: Vector2;

    if state.ship_mov_pressed {
        let fire_scale = 1.125 + ((rand::random::<f32>() - 0.25) / 3.0);

        v1 = Vector2::new(8.0, -8.0);
        v1.y += 14.0;
        v1 /= fire_scale;
        v1.rotate(state.ship_rot);
        v1 += screen_center;

        v2 = Vector2::new(-8.0, -8.0);
        v2.y += 14.0;
        v2 /= fire_scale;
        v2.rotate(state.ship_rot);
        v2 += screen_center;

        v3 = Vector2::new(0.0, 10.0);
        v3.y += 14.0;
        v3 /= fire_scale;
        v3.rotate(state.ship_rot);
        v3 += screen_center;

        d.draw_triangle(v1, v2, v3, Color::ORANGE);
    }

    let rotation = degrees_to_radians(180.0);
    v1 = Vector2::new(7.0, -7.0);
    v1.rotate(rotation + state.ship_rot);
    v1 += screen_center;

    v2 = Vector2::new(-7.0, -7.0);
    v2.rotate(rotation + state.ship_rot);
    v2 += screen_center;

    v3 = Vector2::new(0.0, 14.0);
    v3.rotate(rotation + state.ship_rot);
    v3 += screen_center;

    d.draw_triangle(v1, v2, v3, Color::WHITE);

    camera.target = state.ship_pos;
    // camera.rotation = radians_to_degrees(state.ship_rot);
}

fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, camera: &mut Camera2D, state: &mut State) {
    let screen_width = rl.get_screen_width() as f32;
    let screen_height = rl.get_screen_height() as f32;
    let screen_center = Vector2::new(screen_width, screen_height) / 2.0;
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLACK);

    // d.draw_line(0 + camera.target.x as i32, 0 + camera.target.y as i32, screen_width + camera.target.x as i32, 0 + camera.target.y as i32, Color::WHITESMOKE);
    // d.draw_line(0, screen_height, screen_width, screen_height, Color::WHITESMOKE);
    // d.draw_line(0, 0, 0, screen_height, Color::WHITESMOKE);
    // d.draw_line(0, 0, 0, screen_height, Color::WHITESMOKE);
    {
        let mut d = d.begin_mode2D(camera as &Camera2D);

        // Draw border
        d.draw_circle_v(Vector2::new(100.0, 100.0), 30.0, Color::RED);
        d.draw_line_v(Vector2::zero(), Vector2::new(0.0, MAP_HEIGHT), Color::RED);
        d.draw_line_v(Vector2::zero(), Vector2::new(MAP_WIDTH, 0.0), Color::RED);
        d.draw_line_v(
            Vector2::new(MAP_WIDTH, MAP_HEIGHT),
            Vector2::new(0.0, MAP_HEIGHT),
            Color::RED,
        );
        d.draw_line_v(
            Vector2::new(MAP_WIDTH, MAP_HEIGHT),
            Vector2::new(MAP_WIDTH, 0.0),
            Color::RED,
        );
    }

    draw_ship(&mut d, camera, screen_center, state);

    for obstacle in &state.obstacles {
        draw_obstacle(obstacle, &mut d, camera, &state);
    }
}

fn main() {
    let mut state = State::new();

    state.obstacles.push(Obstacle::Planet {
        position: Vector2::new(100.0, 0.0) + Vector2::new(800.0, 700.0),
        radius: 200.0,
        color: Color::GRAY,
        mass: 3.0,
    });
    state.obstacles.push(Obstacle::Planet {
        position: Vector2::new(800.0, 0.0) + Vector2::new(800.0, 700.0),
        radius: 50.0,
        color: Color::RED,
        mass: 3.0,
    });
    state.obstacles.push(Obstacle::Planet {
        position: Vector2::new(400.0, 500.0) + Vector2::new(800.0, 700.0),
        radius: 75.0,
        color: Color::BLUEVIOLET,
        mass: 3.0,
    });
    state.obstacles.push(Obstacle::Line {
        start_pos: Vector2::new(200.0, 200.0),
        end_pos: Vector2::new(400.0, 400.0),
        color: Color::PERU,
    });

    let (mut rl, thread) = raylib::init()
        .title("Hello, World")
        .resizable()
        .size(1024, 740)
        .build();

    rl.set_target_fps(30);

    let center_of_screen =
        Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32) / 2.0;

    let mut camera = Camera2D {
        offset: center_of_screen.clone(),
        target: state.ship_pos.clone() + center_of_screen,
        rotation: degrees_to_radians(0.0),
        zoom: 1.0,
    };

    state.ship_pos = center_of_screen.clone();

    while !rl.window_should_close() {
        controls(&mut rl, &thread, &mut state);
        calculate(&mut rl, &mut state);
        draw(&mut rl, &thread, &mut camera, &mut state);
    }
}
