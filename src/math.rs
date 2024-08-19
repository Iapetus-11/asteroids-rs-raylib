use raylib::math::Vector2;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    (degrees * ::core::f32::consts::PI) / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * (180.0 / ::core::f32::consts::PI)
}

pub fn avg_vec2(v: Vector2) -> f32 {
    (v.x + v.y) / 2.0
}
