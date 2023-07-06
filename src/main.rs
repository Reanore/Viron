use gtk::prelude::{ApplicationExtManual, ApplicationExt};
use gtk::traits::GtkWindowExt;
use gtk::{Application, glib, ApplicationWindow};

const APP_ID: &str = "temp.temp.viron";
const TITLE: &str = "Viron";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder().application(app).title(TITLE).build();

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
