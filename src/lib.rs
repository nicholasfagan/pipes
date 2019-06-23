extern crate ctrlc;
extern crate rand;
use rand::Rng;

mod io;
mod pipe;
use pipe::Pipe;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::vec;

pub mod config;
use config::Config;

pub fn run(cfg : Config) {
    init();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut parr : vec::Vec<Pipe> = vec![];
    for _i in 0..(cfg.num_pipes)  {
        parr.push(Pipe::new(cfg));
    }
    while running.load(Ordering::SeqCst) {
        for pipe in parr.iter_mut() {
            pipe.update();
        }
        if cfg.reset_chance != 0 && maybe(cfg.reset_chance as i32) {
            io::clear();
        }
        std::thread::sleep(std::time::Duration::from_millis(cfg.delay));
    }

    cleanup();
}


fn init() {
    io::smcup();
    io::clear();
    io::civis();
}
fn cleanup() {
    io::cup(0,0);
    io::nocolor();
    io::clear();
    io::cnorm();
    io::rmcup();
}

pub fn maybe(one_in_odds : i32) -> bool {
    rand::thread_rng().gen_range(0, one_in_odds) == 0
}

