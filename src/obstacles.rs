use raylib::prelude::*;

use crate::{
    math::{degrees_to_radians, radians_to_degrees},
    State,
};

#[derive(Debug, Clone)]
pub enum Obstacle {
    Planet {
        position: Vector2,
        radius: f32,
        color: Color,
        mass: f32,
    },
    Line {
        start_pos: Vector2,
        end_pos: Vector2,
        color: Color,
    },
}

pub fn draw_obstacle(
    obstacle: &Obstacle,
    d: &mut RaylibDrawHandle,
    camera: &mut Camera2D,
    state: &State,
) {
    match obstacle {
        Obstacle::Planet {
            position,
            radius,
            color,
            mass,
        } => {
            let mut d = d.begin_mode2D(camera as &Camera2D);

            d.draw_circle_v(position, *radius, color);
            d.draw_circle_lines(
                position.x as i32,
                position.y as i32,
                (mass + 1.0) * 200.0,
                Color::GRAY,
            );
        }
        Obstacle::Line {
            start_pos,
            end_pos,
            color,
        } => {
            let mut d = d.begin_mode2D(camera as &Camera2D);

            d.draw_line_v(start_pos, end_pos, color);
        }
    }
}

pub fn gravity_obstacle(obstacle: &Obstacle, state: &mut State) {
    match obstacle {
        Obstacle::Planet {
            position,
            radius: _,
            color: _,
            mass,
        } => {
            let distance = position.distance_to(state.ship_pos);

            // Check if gravity should be in effect
            if distance > ((mass + 1.0) * 200.0) {
                return;
            }

            let grav_angle = position.angle_to(state.ship_pos);
            let mut grav = Vector2::new(0.0, *mass);
            grav.rotate(grav_angle);
            grav.rotate(degrees_to_radians(90.0));
            grav /= distance.sqrt();

            state.ship_vel -= grav;
        }
        Obstacle::Line {
            start_pos: _,
            end_pos: _,
            color: _,
        } => {}
    }
}

pub fn collide_obstacle(obstacle: &Obstacle, state: &mut State) {
    match obstacle {
        Obstacle::Planet {
            position,
            radius,
            color: _,
            mass: _,
        } => {
            if check_collision_circles(position, *radius, state.ship_pos, 10.0) {
                // state.ship_vel.rotate((
                //     state.ship_pos.angle_to(*position).tan()
                //     // + Vector2::one().angle_to(state.ship_vel)
                // ) / 1.0);

                let distance_from_center = state.ship_pos.distance_to(*position);

                state
                    .ship_vel
                    .rotate(-state.ship_pos.angle_to(*position).tanh());

                // while check_collision_circles(position, *radius, state.ship_pos, 10.0) {
                state.ship_pos.x -= state.ship_vel.x;
                state.ship_pos.y -= state.ship_vel.y;
                // }
            }
        }
        Obstacle::Line {
            start_pos,
            end_pos,
            color: _,
        } => {
            let after_mov = state.ship_pos - state.ship_vel;

            let point_of_intersection = check_collision_lines(start_pos, end_pos, state.ship_pos, after_mov);

            if let Some(point_of_intersection) = point_of_intersection {
                let angle_to = state.ship_pos.angle_to(point_of_intersection);

                println!("{}", radians_to_degrees(angle_to).round());

                state.ship_vel.rotate(angle_to.tan());
                state.ship_vel.rotate(degrees_to_radians(90.0));
                state.ship_pos -= state.ship_vel;
            }
        }
    }
}
