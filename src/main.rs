#![feature(generators, async_await, futures_api)]

//extern crate futures;

//use futures::future::Future; // Note: It's not `futures_preview`
use futures::executor::block_on;

fn main() {
    println!("Hello, world!");
    block_on(goff())
}

async fn goff() {
    println!("Hello, goff!");
}
