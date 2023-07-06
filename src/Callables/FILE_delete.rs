extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
use std::fs

fn dialog(message) {
    if gtk::init().is_err() {
        println!("VIR-055: Failed to initialize GTK.");
        return;
    }
    match message {
        "success" => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "File deleted successfully").run();
        }
        "VIR-054" => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "VIR-054: File not found").run();
        }
        _ => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "VIR-057: Internal error").run();
        }
    }
}

fn main(file_to_remove) {
    if (!file_to_remove) {
        dialog("VIR-054")
    }
    fs::remove_file(file_to_remove)
        .expect(dialog());
    success();
}
