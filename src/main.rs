use nannou::{color::Gradient, color::Mix, prelude::*};
use rand;

struct Model {
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    if app.window_count() == 0 {
        app.new_window().event(event).view(view).build().unwrap();
    }
    Model { points: vec![] }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        ReceivedCharacter(_) => (),
        _ => return,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    // Begin drawing
    let draw = app.draw();

    pencil(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn pencil(draw: &Draw) {
    const PENCILHEIGHT: f32 = 500.0;
    const WIDTH: f32 = 50.0;
    const TIPHEIGHT: f32 = 100.0;

    let draw = draw.scale(0.4).rotate(deg_to_rad(-30.0));

    // pencil tip
    draw.tri().color(BLANCHEDALMOND).points(
        pt2(-WIDTH / 2.0, TIPHEIGHT),
        pt2(WIDTH / 2.0, TIPHEIGHT),
        pt2(0.0, 0.0),
    );

    // graphite tip
    draw.tri().color(BLACK).points(
        pt2(-WIDTH / 2.0 / 4.0, TIPHEIGHT / 4.0),
        pt2(WIDTH / 2.0 / 4.0, TIPHEIGHT / 4.0),
        pt2(0.0, 0.0),
    );

    // pencil body
    let draw = draw.translate(pt3(0.0, TIPHEIGHT, 0.0));
    draw.quad().color(GOLDENROD).points(
        pt2(-WIDTH / 2.0, 0.0),
        pt2(WIDTH / 2.0, 0.0),
        pt2(WIDTH / 2.0, PENCILHEIGHT),
        pt2(-WIDTH / 2.0, PENCILHEIGHT),
    );

    // eraser holder
    let draw = draw.translate(pt3(0.0, PENCILHEIGHT, 0.0));
    draw.quad().color(GREY).points(
        pt2(-WIDTH / 2.0, 0.0),
        pt2(WIDTH / 2.0, 0.0),
        pt2(WIDTH / 2.0, WIDTH),
        pt2(-WIDTH / 2.0, WIDTH),
    );

    // eraser
    let draw = draw.translate(pt3(0.0, WIDTH, 0.0));
    draw.quad().color(PINK).points(
        pt2(-WIDTH / 2.0, 0.0),
        pt2(WIDTH / 2.0, 0.0),
        pt2(WIDTH / 2.0, WIDTH),
        pt2(-WIDTH / 2.0, WIDTH),
    );
}
