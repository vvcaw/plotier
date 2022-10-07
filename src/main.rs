fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    units_x_axis: u64,
}

struct Model {
    egui: nannou_egui::Egui,
    settings: Settings,
}

fn model(app: &nannou::App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = nannou_egui::Egui::from_window(&window);

    let settings = Settings { units_x_axis: 20 };

    Model { egui, settings }
}

fn update(_app: &nannou::App, model: &mut Model, update: nannou::prelude::Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    nannou_egui::egui::Window::new("Plot Settings").show(&ctx, |ui| {
        // Coordinate system slider for x-axis
        ui.label("Units in x direction:");
        ui.add(nannou_egui::egui::Slider::new(
            &mut settings.units_x_axis,
            1..=100,
        ))
    });
}

fn raw_window_event(
    _app: &nannou::App,
    model: &mut Model,
    event: &nannou::winit::event::WindowEvent,
) {
    // Let egui handle things like keyboard events and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &nannou::App, model: &Model, frame: nannou::prelude::Frame) {
    let draw = app.draw();
    draw.background().color(nannou::prelude::BLACK);

    // Draw x axis
    draw.line()
        .weight(3.0)
        .caps_round()
        .color(nannou::prelude::WHEAT)
        .points(
            nannou::prelude::Vec2::new(-(app.window_rect().w() / 2.0) + 10.0, 0.0),
            nannou::prelude::Vec2::new((app.window_rect().w() / 2.0) - 10.0, 0.0),
        );

    // Draw y axis
    draw.line()
        .weight(3.0)
        .caps_round()
        .color(nannou::prelude::WHEAT)
        .points(
            nannou::prelude::Vec2::new(0.0, -(app.window_rect().h() / 2.0) + 10.0),
            nannou::prelude::Vec2::new(0.0, (app.window_rect().h() / 2.0) - 10.0),
        );

    let rescaled_unit = app.window_rect().w() / model.settings.units_x_axis as f32;

    // Render bezier curve to approximate function
    draw.path()
        .stroke()
        .weight(3.0)
        .color(nannou::prelude::WHITE)
        .events(
            approximate_function_splice_as_bezier(rescaled_unit, -5.0, 5.0, 1.0, 3.0, -6.0, 0.0)
                .iter(),
        );

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn approximate_function_splice_as_bezier(
    rescaled_unit: f32,
    approximation_start: f32,
    approximation_end: f32,
    c0: f32,
    c1: f32,
    c2: f32,
    c3: f32,
) -> nannou::geom::Path {
    // For approximating the part of the polynomial of degree 3, we need to create a blossom
    // (symettric + multi-affine) function that satisfies G(t, t, t) = F(t) for all t.

    // Create function t
    fn t(u: f32, v: f32, w: f32) -> f32 {
        (u + v + w) / 3.0
    }

    // Create blossom function
    // Replace t^3 with u * v * w
    // Replace t^2 with (1.0 / 3.0) * (vw + wu + uv)
    // Replace t with (1.0 / 3.0) * (u + v + w)
    let blossom = |u: f32, v: f32, w: f32| {
        c0 * (u * v * w)
            + c1 * (((v * w) + (w * u) + (u * v)) / 3.0)
            + c2 * ((u + v + w) / 3.0)
            + c3
    };

    // The control points of the bezier curve for the segment [approximation_start, approximation_end] are evaluated as follows:
    // blossom(approximation_start, approximation_start, approximation_start)
    // blossom(approximation_start, approximation_start, approximation_end)
    // blossom(approximation_start, approximation_end, approximation_end)
    // blossom(approximation_end, approximation_end, approximation_end)
    //
    // The order does not matter as blossom is symmetric.
    //
    // Note that approximation here is important! Therefore we use real values for x values at the
    // first & last control point.
    nannou::geom::path()
        .begin(nannou::geom::Point2::new(
            rescaled_unit * approximation_start, // TODO: This does not work for 0.0, 0.0 for some
            // reason
            rescaled_unit
                * blossom(
                    approximation_start,
                    approximation_start,
                    approximation_start,
                ),
        ))
        .cubic_bezier_to(
            nannou::geom::Point2::new(
                rescaled_unit * t(approximation_start, approximation_start, approximation_end),
                rescaled_unit
                    * blossom(approximation_start, approximation_start, approximation_end),
            ),
            nannou::geom::Point2::new(
                rescaled_unit * t(approximation_start, approximation_end, approximation_end),
                rescaled_unit * blossom(approximation_start, approximation_end, approximation_end),
            ),
            nannou::geom::Point2::new(
                rescaled_unit * approximation_end,
                rescaled_unit * blossom(approximation_end, approximation_end, approximation_end),
            ),
        )
        .build()
}
