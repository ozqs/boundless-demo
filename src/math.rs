use macroquad::prelude::*;
pub fn calculate_intersect(rotation: f32, screen_width: f32, screen_height: f32) -> Vec2 {
    let center = Vec2::new(screen_width / 2., screen_height / 2.);
    let screen_rect = Rect::new(0., 0., screen_width, screen_height);
    let ray_direction = Vec2::from_angle(rotation);
    let mut intersections = vec![];
    if ray_direction.x > 0. || ray_direction.x < 0. {
        intersections.push(center + ray_direction * (center.x / ray_direction.x).abs());
    }

    if ray_direction.y > 0. || ray_direction.y < 0. {
        intersections.push(center + ray_direction * (center.y / ray_direction.y).abs());
    }

    for intersection in intersections {
        if intersection.x >= screen_rect.left()
            && intersection.x <= screen_rect.right()
            && intersection.y <= screen_rect.bottom()
            && intersection.y >= screen_rect.top()
        {
            return intersection;
        }
    }

    (0., 0.).into()
}
