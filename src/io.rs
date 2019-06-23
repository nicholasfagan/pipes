extern crate terminal_size;
extern crate rand;
use rand::Rng;
use terminal_size::{Width, Height, terminal_size};
use std::io::Write;
pub fn flush() {
    std::io::stdout().flush().unwrap();
}
pub fn cup(x : u32, y : u32) {
    print!("\x1b[{};{}H", y, x);
}
pub fn smcup() {
    print!("\x1b[?1049h");
}
pub fn rmcup() {
    print!("\x1b[?1049l");
}
// Hide cursor
pub fn civis() {
    print!("\x1b[?25l");
}
// show cursor
pub fn cnorm() {
    print!("\x1b[?25h");
}
pub fn clear() {
    print!("\x1b[2J");
}
pub fn width() -> u32 {
    let size = terminal_size();
    if let Some((Width(w), Height(_h))) = size {
        w as u32
    } else {
        panic!("Could not get window size.");
    }
}
pub fn height() -> u32 {
    let size = terminal_size();
    if let Some((Width(_w), Height(h))) = size {
        h as u32
    } else {
        panic!("Could not get window size.");
    }
}

#[derive(Copy, Clone)]
pub enum COLOR {
    BLACK = 0,
    RED = 1,
    GREEN = 2,
    YELLOW = 3,
    BLUE = 4,
    PURPLE = 5,
    TURQUOISE = 6,
    WHITE = 7,
}
impl COLOR {
    pub fn random() -> COLOR {
        COLOR::from(rand::thread_rng().gen_range(1, 7))
    }
    pub fn from(code : u16) -> COLOR {
        match code {
            0 => COLOR::BLACK,
            1 => COLOR::RED,
            2 => COLOR::GREEN,
            3 => COLOR::YELLOW,
            4 => COLOR::BLUE,
            5 => COLOR::PURPLE,
            6 => COLOR::TURQUOISE,
            _ => COLOR::WHITE,
        }
    }
}
pub fn setaf(color : COLOR) {
    let code : u8 = 30 + color as u8;
    print!("\x1b[{}m", code);
}
pub fn nocolor() {
    print!("\x1b[0m");
}
