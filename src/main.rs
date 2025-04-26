use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Entry};

const APP_ID: &str = "no.brunhenriksen.ordKode";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // lag text input field
    let entry = Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .placeholder_text("Skriv noe drit her...") // placeholder text
        .build();

    // lag vinduet
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ordKode")
        .child(&entry)
        .build();

    // vis vinduet
    window.present();
}
