#[allow(dead_code)]
// use dotenv::dotenv;
// use std::env;
// use std::collections::HashMap;

pub mod webhook;
pub mod embed;

mod sample;

fn main() {
    sample::run();
}