use gtk::{prelude::*, Box, Button};
use gtk::{glib, Application, ApplicationWindow};
use std::process::Command;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let vbox = Box::new(gtk::Orientation::Horizontal,10);
    let btn2 = Button::builder()
        .label("os")
        .margin_top(4)
        .margin_start(2)
        .margin_end(2)
        .build();
 
    let btn = Button::builder()
        .label("os")
        .margin_top(4)
        .margin_start(2)
        .margin_end(2)
        .build();

    btn.connect_clicked(move |_|{
         // Open Firefox
        match Command::new("firefox").spawn() {
        Ok(_) => print!("Firefox opened!"),
        Err(e) => {eprintln!("Failed to open Firefox: {}", e); // Print error to console
        }
    }
    }

    );
    vbox.append(&btn);
    vbox.append(&btn2);
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&vbox)
        .build();

    window.set_default_size(-1,20);
    window.set_resizable(false);
    
    // Present window
    window.present();
}
