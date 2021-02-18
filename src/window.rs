extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

pub fn init() {
    let application = Application::new(Some("com.github.arts.example"), Default::default())
        .expect("failed to initialize GTK Application");
    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First Arts Program.");
        window.set_default_size(350, 70);
        let button = Button::with_label("Click me");
        button.connect_clicked(|button| {
            println!("Clicked");
        });
        window.add(&button);
        window.show_all();
    });
    application.run(&[]);
}
