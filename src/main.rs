use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "no.brunhenriksen.ordKode";

fn main() -> glib::ExitCode {
    // lag ny applikasjon
    let app = Application::builder().application_id(APP_ID).build();

    // connect til "activate" signalet av 'app'
    app.connect_activate(build_ui);

    // kjør applikasjonen
    app.run()
}

fn build_ui(app: &Application) {
    // lag en knapp med label og margins
    let button = Button::builder()
        .label("Trykk på meg!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|button| {
        button.set_label("Heisan sveisan, verden!");
    });

    // lag vindu og sett tittel på han
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ordKode")
        .child(&button)
        .build();

    // vis vinduet
    window.present()
}
