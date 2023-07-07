extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
use std::fs

fn dialog(message) {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    match message {
        "Success" => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "File deleted successfully").run();
        }
        "NoFile" => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "File not found").run();
        }
        _ => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "Internal error").run();
        }
    }
}

fn main(path, file_to_remove) {
    if (!file_to_remove) {
        dialog("NoFile")
    }
    fs::remove_file(file_to_remove)
        .expect(dialog());
    success();
}
