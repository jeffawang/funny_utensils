use nannou::{color::Gradient, color::Mix, prelude::*};
use rand;

struct Model {
    points: Vec<Point2>,
    mouse: Point2,
    mouse_down: bool,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().event(event).view(view).build().unwrap();
    let window = app
        .window(window_id)
        .unwrap()
        .set_cursor_icon(nannou::winit::window::CursorIcon::Crosshair);

    Model {
        points: vec![],
        mouse: pt2(0.0, 0.0),
        mouse_down: false,
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        ReceivedCharacter(_) => (),
        MouseMoved(p) => m.mouse = p,
        MousePressed(MouseButton::Left) => m.mouse_down = true,
        MouseReleased(MouseButton::Left) => m.mouse_down = false,
        _ => return,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(IVORY);

    // Begin drawing
    let draw = app.draw();

    let draw = draw.translate(model.mouse.extend(0.0)).translate(pt3(
        0.0,
        if model.mouse_down { 0.0 } else { 30.0 },
        0.0,
    ));
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
