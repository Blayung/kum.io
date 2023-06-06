struct Point {
    x: f64,
    y: f64,
}

fn predict_next_frame(current_position: Point, speed: f64, direction: f64, time_interval: f64) -> Point {
    let displacement_x = speed * direction.cos() * time_interval;
    let displacement_y = speed * direction.sin() * time_interval;

    let next_x = current_position.x + displacement_x;
    let next_y = current_position.y + displacement_y;

    Point { x: next_x, y: next_y }
}

fn main() {
    let current_position = Point { x: 1.0, y: 2.0 };

    let speed = 10.0;

    let direction = 0 as f64 * std::f64::consts::PI/180.0;

    let time_interval = 0.5; // Assume time interval between frames is 0.5 seconds

    let next_frame_position = predict_next_frame(current_position, speed, direction, time_interval);

    println!("Next frame position: ({}, {})", next_frame_position.x, next_frame_position.y);
}
