use nannou::prelude::*;

#[derive(Default)]
struct Model {
    lines: Vec<Vec<Point2>>,
    mouse: Point2,
    pmouse: Point2,
    angle: f32,
    mouse_down: bool,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().event(event).view(view).build().unwrap();
    app.window(window_id)
        .unwrap()
        .set_cursor_icon(nannou::winit::window::CursorIcon::Crosshair);

    Model::default()
}

fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    if m.mouse_down {
        m.lines.last_mut().unwrap().push(m.mouse);
    }

    // the target angle the pencil should be in if it were following the mouse
    let target = match (m.pmouse - m.mouse).angle() {
        f if f.is_nan() => PI / 3.0,
        f => f,
    };
    let theta = (target - m.angle + PI) % TAU - PI;
    m.angle += theta.clamp(-PI / 3.0, PI / 3.0) / 10.0;

    // always update previous mouse
    m.pmouse = m.mouse;
}

fn event(_app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        ReceivedCharacter(_) => (),
        MouseMoved(p) => {
            m.mouse = p;
        }
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
        let draw = draw
            .translate(model.mouse.extend(0.0))
            .translate(pt3(
                0.0,
                if model.mouse_down {
                    -TIPHEIGHT / 4.0 / 2.0
                } else {
                    30.0
                },
                1.0,
            ))
            .scale(0.4)
            .rotate(model.angle);

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
    // rotate draw to make it easier
    let draw = draw.rotate(-PI / 2.0);
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
