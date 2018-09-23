#![feature(generators, async_await, futures_api)]

//extern crate futures;

//use futures::future::Future; // Note: It's not `futures_preview`
use futures::executor::block_on;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

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

    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    block_on(goff());
    gtk::main();
}

async fn goff() {
    println!("Hello, goff!");
}
