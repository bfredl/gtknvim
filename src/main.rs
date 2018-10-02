#![feature(generators, async_await, futures_api, await_macro)]

//extern crate futures;

//use futures::future::Future; // Note: It's not `futures_preview`
//use futures::prelude::*;
use futures::executor::block_on;
//use futures::io::block_on;
use futures::io::AsyncRead;
use futures::io::{AsyncReadExt, Error};

use glib;
//use glib::main_context_futures::MainContext;
//use gio::prelude::*;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

use rmp;
use rmp::Marker;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    //let c = glib::MainContext::default();

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

    //c.spawn_local(goff());
    block_on(goff());
    gtk::main();
}

async fn goff() {
    println!("Hello, goff!");
}

pub async fn read_marker<R: AsyncRead>(rd: &mut R) -> Result<Marker, Error> {
    let mut bytes = [0u8;1];
    let read = await!(rd.read_exact(&mut bytes))?;
    Ok(Marker::from_u8(bytes[0]))
}
