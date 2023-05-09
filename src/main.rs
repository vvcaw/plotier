mod eval_function;
mod linear_solver;

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    units_x_axis: u32,
    units_y_axis: u32,
    sample_point_count: u32,
    function_str: String,
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

    let settings = Settings {
        units_x_axis: 20,
        units_y_axis: 20,
        sample_point_count: 10,
        function_str: "sin(x)".to_string(),
    };

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
        ));

        // Coordinate system slider for y-axis
        ui.label("Units in y direction:");
        ui.add(nannou_egui::egui::Slider::new(
            &mut settings.units_y_axis,
            1..=100,
        ));

        // Slider for sample point count
        ui.label("Sample point count:");
        ui.add(nannou_egui::egui::Slider::new(
            &mut settings.sample_point_count,
            2..=19,
        ));

        // Input for function
        ui.label("Function:");
        ui.add(nannou_egui::egui::TextEdit::singleline(
            &mut settings.function_str,
        ));
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

    let rescaled_x_unit = app.window_rect().w() / model.settings.units_x_axis as f32;
    let rescaled_y_unit = app.window_rect().h() / model.settings.units_y_axis as f32;

    let points = eval_function::sample_points_for_function(
        &model.settings.function_str.as_str(),
        model.settings.sample_point_count,
        model.settings.units_x_axis,
    );

    // Don't draw if no points are evaluted
    if !points.is_empty() {
        let matrix = linear_solver::generate_linear_system(&points);

        let gaussian_elimination = linear_solver::gaussian_elimination(matrix);

        // Get four coefficients each and draw bezier curve.
        for (polynomial_num, chunk) in gaussian_elimination.chunks(4).enumerate() {
            // Render bezier curve to approximate function
            draw.path()
                .stroke()
                .weight(3.0)
                .color(nannou::prelude::WHITE)
                .tolerance(1.0 / app.window_rect().w() * 0.01) // Adjust tolerance to fix path not rendering in certain instances.
                .events(
                    approximate_function_splice_as_bezier(
                        rescaled_x_unit as f64,
                        rescaled_y_unit as f64,
                        points[polynomial_num].0,
                        points[polynomial_num + 1].0,
                        chunk[0],
                        chunk[1],
                        chunk[2],
                        chunk[3],
                    )
                    .iter(),
                );
        }

        // Render the points
        for (x, y) in &points {
            draw.ellipse()
                .color(nannou::prelude::GREEN)
                .radius(10.0)
                .x(rescaled_x_unit * ((*x) as f32))
                .y(rescaled_y_unit * ((*y) as f32));
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn approximate_function_splice_as_bezier(
    rescaled_x_unit: f64,
    rescaled_y_unit: f64,
    approximation_start: f64,
    approximation_end: f64,
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
) -> nannou::geom::Path {
    // For approximating the part of the polynomial of degree 3, we need to create a blossom
    // (symettric + multi-affine) function that satisfies G(t, t, t) = F(t) for all t.

    // Create function t
    fn t(u: f64, v: f64, w: f64) -> f64 {
        (u + v + w) / 3.0
    }

    // Create blossom function
    // Replace t^3 with u * v * w
    // Replace t^2 with (1.0 / 3.0) * (vw + wu + uv)
    // Replace t with (1.0 / 3.0) * (u + v + w)
    let blossom = |u: f64, v: f64, w: f64| {
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
            (rescaled_x_unit * approximation_start) as f32, // TODO: This does not work for 0.0, 0.0 for some
            // reason
            (rescaled_y_unit
                * blossom(
                    approximation_start,
                    approximation_start,
                    approximation_start,
                )) as f32,
        ))
        .cubic_bezier_to(
            nannou::geom::Point2::new(
                (rescaled_x_unit * t(approximation_start, approximation_start, approximation_end)) as f32,
                (rescaled_y_unit
                    * blossom(approximation_start, approximation_start, approximation_end)) as f32,
            ),
            nannou::geom::Point2::new(
                (rescaled_x_unit * t(approximation_start, approximation_end, approximation_end)) as f32,
                (rescaled_y_unit
                    * blossom(approximation_start, approximation_end, approximation_end)) as f32,
            ),
            nannou::geom::Point2::new(
                (rescaled_x_unit * approximation_end) as f32,
                (rescaled_y_unit * blossom(approximation_end, approximation_end, approximation_end)) as f32,
            ),
        )
        .build()
}
