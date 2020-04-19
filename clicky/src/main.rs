
use gtk::prelude::*;

use gtk::{Button, Window, WindowType};
use std::io::Write;
use std::cell::RefCell;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    gtk::init()?;

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("Click me!");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let file = RefCell::new(std::fs::File::create("log.txt")?);
    button.connect_clicked(move |_| {
        match file.borrow_mut().write_all(b"I was clicked!\n") {
            Err(e) => eprintln!("Error: {}", e),
            Ok(_) => println!("Clicked!"),
        };

    });

    gtk::main();
    Ok(())
}
