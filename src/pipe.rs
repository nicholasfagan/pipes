
extern crate rand;
use rand::Rng;
use crate::io;
use crate::maybe;

use crate::config::Config;

fn random_direction() -> i32 {
    rand::thread_rng().gen_range(0, 2) * 2 - 1
}

#[derive(Copy, Clone)]
pub struct Pipe {
    x : i32,
    y : i32,
    color : io::COLOR,
    dx : i32,
    dy : i32,
    color_chance : i32,
    turn_chance : i32,
    pipe_set : PipeSet,
}


impl Pipe {
    pub fn new(cfg : Config) -> Pipe {
        if maybe(2) {
            Pipe { 
                x : rand::thread_rng().gen_range(1,io::width() as i32),
                y : rand::thread_rng().gen_range(1,io::height() as i32),
                color : io::COLOR::random(),
                dx : random_direction(),
                dy : 0,
                color_chance : cfg.color_chance as i32,
                turn_chance : cfg.turn_chance as i32,
                pipe_set : PipeSet::from(cfg.pipe_set),
            }
        } else {
            Pipe { 
                x : rand::thread_rng().gen_range(1,io::width() as i32),
                y : rand::thread_rng().gen_range(1,io::height() as i32),
                color : io::COLOR::random(),
                dx : 0,
                dy : random_direction(),
                color_chance : cfg.color_chance as i32,
                turn_chance : cfg.turn_chance as i32,
                pipe_set : PipeSet::from(cfg.pipe_set),
            }
        }
    }
    fn cornerh(&self) {
        if self.dx == 1 {
            if self.dy == 1 {
                self.draw_with(self.pipe_set.dl); // dl
            } else {
                self.draw_with(self.pipe_set.ul); // ul
            }
        } else {
            if self.dy == 1 {
                self.draw_with(self.pipe_set.dr); // dr
            } else {
                self.draw_with(self.pipe_set.ur); // ur
            }
        }
    }
    fn cornerv(&self) {
        if self.dx == 1 {
            if self.dy == 1 {
                self.draw_with(self.pipe_set.ur); // ur
            } else {
                self.draw_with(self.pipe_set.dr); // dr
            }
        } else {
            if self.dy == 1 {
                self.draw_with(self.pipe_set.ul); // ul
            } else {
                self.draw_with(self.pipe_set.dl); // dl
            }
        }
    }
    fn bounds(&mut self) {
        if self.x == 0 {
            self.x = io::width() as i32 ;
            self.color = io::COLOR::random();
        }
        if self.x == io::width() as i32 + 1 {
            self.x = 1;
            self.color = io::COLOR::random();
        }
        if self.y == 0 {
            self.y = io::height() as i32 ;
            self.color = io::COLOR::random();
        }
        if self.y == io::height() as i32 + 1 {
            self.y = 1;
            self.color = io::COLOR::random();
        }
    }
    fn change_direction(&mut self) {
        if self.dx == 0 {
            self.dx = random_direction();
            self.cornerv();
            self.dy = 0;
        } else {
            self.dy = random_direction();
            self.cornerh();
            self.dx = 0;
        }
    }
    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.bounds();
        self.draw();
        if self.dx == 0 && maybe(self.turn_chance) || maybe(self.turn_chance * 2) {
            self.change_direction();
        }
        if maybe(self.color_chance) {
            self.color = io::COLOR::random();
        }
    }
    fn draw(&self) {
        if self.dx == 0 {
            self.draw_with(self.pipe_set.v);
        } else {
            self.draw_with(self.pipe_set.h);
        }
    }
    fn draw_with(&self, s : char) {
        io::cup(self.x as u32, self.y as u32);
        io::setaf(self.color);
        print!("{}",s);
        io::flush();
    }
}


//const CDL : &str = "\u{256e}";//"\u{2557}";
//const CUL : &str = "\u{256f}";//"\u{255D}";
//const CDR : &str = "\u{256d}";//"\u{2554}";
//const CUR : &str = "\u{2570}";//"\u{255A}";
//const CV  : &str = "\u{2502}";//"\u{2551}";
//const CH  : &str = "\u{2500}";//"\u{2550}";

#[derive(Copy, Clone)]
struct PipeSet {
    pub dl : char,
    pub ul : char,
    pub dr : char,
    pub ur : char,
    pub v  : char,
    pub h  : char,
}

impl PipeSet {
    pub fn default() -> PipeSet {
        PipeSet::from(0)
    }
    pub fn from(n : u32) -> PipeSet {
        match n {
            0 => PipeSet::new("\u{2503}\u{2501}\u{250f}\u{2513}\u{251b}\u{2517}"), // Thick Solid Pipe
            1 => PipeSet::new("\u{2551}\u{2550}\u{2554}\u{2557}\u{255D}\u{255A}"), // Two-walled pipe
            2 => PipeSet::new("\u{2502}\u{2500}\u{256d}\u{256e}\u{256f}\u{2570}"), // Thin Solid Pipe
            3 => PipeSet::new("|-++++"),   // Star
            4 => PipeSet::new("|-/\\/\\"), // Slash
            _ => PipeSet::default(),
        }
    }
    pub fn new(s : &str) -> PipeSet {
        //string must be: {vertical}{horizontal}{dr}{dl}{ul}{ur}
        if s.len() < 6 {
            use crate::cleanup;
            cleanup();
            panic!("invalid pipeset string: '{}'", s);
        }
        PipeSet {
            v : s.chars().nth(0).unwrap(),
            h : s.chars().nth(1).unwrap(),
            dr : s.chars().nth(2).unwrap(),
            dl : s.chars().nth(3).unwrap(),
            ul : s.chars().nth(4).unwrap(),
            ur : s.chars().nth(5).unwrap(),
        }
    }
}
