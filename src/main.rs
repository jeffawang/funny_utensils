use nannou::{color::Gradient, color::Mix, prelude::*};
use rand;

struct Model {
    lines: Vec<Vec<Point2>>,
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
        lines: vec![],
        mouse: pt2(0.0, 0.0),
        mouse_down: false,
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    if m.mouse_down {
        m.lines.last_mut().unwrap().push(m.mouse);
    }
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        ReceivedCharacter(_) => (),
        MouseMoved(p) => m.mouse = p,
        MousePressed(MouseButton::Left) => {
            m.mouse_down = true;
            m.lines.push(vec![])
        }
        MouseReleased(MouseButton::Left) => m.mouse_down = false,
        _ => return,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(IVORY);

    // Begin drawing
    let draw = app.draw();

    {
        let draw = draw.translate(pt3(0.0, 300.0, 0.0));
        draw.text("Why shouldn't you write with a dull pencil?")
            .color(BLACK)
            .font_size(30)
            .w(400.0);
    }

    {
        let draw = draw.translate(model.mouse.extend(0.0)).translate(pt3(
            0.0,
            if model.mouse_down {
                -TIPHEIGHT / 4.0 / 2.0
            } else {
                30.0
            },
            1.0,
        ));
        pencil(&draw);
    }

    for line in model.lines.iter() {
        let c = rgba(0.0, 0.0, 0.0, 0.6);
        draw.polyline()
            .stroke_weight(3.0)
            .points_colored(line.iter().map(|p| (p.clone(), c)));
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

const PENCILHEIGHT: f32 = 500.0;
const WIDTH: f32 = 50.0;
const TIPHEIGHT: f32 = 100.0;
fn pencil(draw: &Draw) {
    let draw = draw.scale(1.0).rotate(deg_to_rad(-30.0));

    // pencil tip
    draw.quad().color(BLANCHEDALMOND).points(
        pt2(-WIDTH / 2.0, TIPHEIGHT),
        pt2(WIDTH / 2.0, TIPHEIGHT),
        pt2(WIDTH / 2.0 / 4.0, TIPHEIGHT / 4.0),
        pt2(-WIDTH / 2.0 / 4.0, TIPHEIGHT / 4.0),
    );

    // graphite tip
    draw.quad().color(BLACK).points(
        pt2(-WIDTH / 2.0 / 4.0, TIPHEIGHT / 4.0),
        pt2(WIDTH / 2.0 / 4.0, TIPHEIGHT / 4.0),
        pt2(WIDTH / 2.0 / 4.0 / 2.0, TIPHEIGHT / 4.0 / 2.0),
        pt2(-WIDTH / 2.0 / 4.0 / 2.0, TIPHEIGHT / 4.0 / 2.0),
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
