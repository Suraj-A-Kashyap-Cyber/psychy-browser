use gtk4::prelude::*;
use gtk4::{gdk_pixbuf, gio, glib, Image, Window};
use std::time::Duration;

pub fn show_splash() -> Window {
    // path is already in OUT_DIR; no extra prefix
    gio::resources_register_include!("resources.gresource")
        .expect("resources.gresource not found");

    let window = Window::builder()
        .title("Psychy")
        .default_width(280)
        .default_height(280)
        .build();

    let pixbuf = gdk_pixbuf::Pixbuf::from_resource("/com/psychy/icons/logo.svg")
        .expect("logo asset missing");
    let img = Image::from_pixbuf(Some(&pixbuf));
    window.set_child(Some(&img));
    window.present();

    glib::timeout_add_local_once(Duration::from_millis(2000), move || {
        window.close();
    });

    window
}
