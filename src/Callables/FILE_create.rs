extern crate gtk;
use std::fs::File;
use std::io::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

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
                "File created successfully").run();
        }
        "FileExists" => {
            MessageDialog::new(None::<&Window>,
                DialogFlags::empty(),
                MessageType::Info,
                ButtonsType::Ok,
                "File already exists").run();
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

fn main(file_to_create) -> std::io::Result<()> {
    let b = std::path::Path::new(fp).exists();
    if b {
        let mut file = File::create(file_to_create)?;
        file.write_all(b"")?;
        Ok(())
    }
}
