use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use webkit6::prelude::*;
use webkit6::{Settings, WebView};
use psy_ui::*;

const APP_ID: &str = "com.psychy.Browser";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("resources.gresource").unwrap();
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| { let _ = show_splash(); });
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Psychy")
        .default_width(1024)
        .default_height(768)
        .build();

    let settings = Settings::builder()
        .enable_javascript(true)
        .enable_webgl(false)
        .enable_webrtc(false)
        .enable_developer_extras(false)
        .build();

    let webview = WebView::builder()
        .settings(&settings)
        .build();
    webview.load_uri("https://check.torproject.org");
    window.set_child(Some(&webview));
    window.present();
}
