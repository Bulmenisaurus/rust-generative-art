use nannou::prelude::*;
use rand::Rng;

// https://docs.rs/nannou/0.17.1/nannou/

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

struct Model {
    points: Vec<Circle>,
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    let mut circles: Vec<Circle> = Vec::new();
    for _ in 0..100 {
        let mut rng = rand::thread_rng();
        // just generate 1-100 now, since you can't (or at least I think you can't) access with width and height of the window here
        let rand_x = rng.gen_range(0..100);
        let rand_y = rng.gen_range(0..100);
        let radius = rng.gen_range(0..50);

        circles.push(Circle {
            x: rand_x as f32,
            y: rand_y as f32,
            radius: radius as f32,
        })
    }

    Model {
        _window,
        points: circles,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.time;
    let win = app.main_window().rect();

    frame.clear(BLACK);

    for i in model.points.iter() {
        draw.ellipse()
            .rgba(0., 0., 0., 0.)
            .radius(((time + i.radius).sin() * i.radius + i.radius) / 2.)
            .stroke(WHITE)
            .stroke_weight(1.00)
            // normalize coordinates and make the origin in the middle, not top left
            // also makes it responsive
            .x_y(
                (i.x / 100.) * (win.w()) - (win.w() / 2.),
                (i.y / 100.) * (win.h()) - (win.h() / 2.),
            );
    }

    draw.to_frame(app, &frame).unwrap();
}
