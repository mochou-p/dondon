mod audio;
mod ui;


fn main() {
    nannou::app(ui::model)
        .event(ui::event)
        .simple_window(ui::view)
        .run();
}

