/*

# Tutorials:

 * https://guide.nannou.cc/tutorials/basics/anatomy-of-a-nannou-app.html
 * https://docs.rs/nannou/0.17.1/nannou/
 * https://jacobrosenthal.github.io/rust-training-aimldevfest-2019/iterators.html
 * https://doc.rust-lang.org/cargo/guide/project-layout.html

*/

/* Nannou stuff: */

use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Point>,
}

struct Point {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let mut points: Vec<Point> = Vec::new();

    for _ in 0..100 {
        let mut rng = rand::thread_rng();

        points.push(Point {
            x: rng.gen_range(0..100) as f32,
            y: rng.gen_range(0..100) as f32,
        });
    }

    Model { _window, points }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);

    for point in &model.points {
        let mut closest_points = &model.points;
        closest_points.sort_by_key(|a| coordinate_distance(a.x, a.y, point.x, point.y));
    }

    draw.to_frame(app, &frame).unwrap();
}

fn coordinate_distance(x1: f32, y1: f32, x2: f32, y2: f32) {
    let a: f32 = (x2 - x1).pow(2) + (y2 - y1).pow(2);

    (a).sqrt();
}
