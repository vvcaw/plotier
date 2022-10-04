fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    units_x_axis: u64
}

struct Model {
    egui: nannou_egui::Egui,
    settings: Settings
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
        ui.add(nannou_egui::egui::Slider::new(&mut settings.units_x_axis, 0..=1000))
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

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
