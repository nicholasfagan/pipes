
extern crate structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Copy, Clone)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// A terminal screensaver written in rust.
///
/// This is really a re-write of pipes.sh. I encourage you to take a look at that origional
/// project. https://github.com/pipeseroni/pipes.sh
///
/// Start the program when you are bored, and exit with Ctrl-c at any time.
pub struct Config {

    #[structopt(short = "d", long = "delay", default_value = "15")]
    /// The delay in milliseconds between drawing each frame.
    pub delay : u64,

    #[structopt(short = "n", long = "num-pipes", default_value = "1")]
    /// The number of pipes to be drawing at once on the screen.
    pub num_pipes : u32,

    #[structopt(short = "c", long = "color-chance", default_value = "200")]
    /// The odds of changing colors on each frame.
    ///
    /// Odds are 1 out of <color_chance>
    pub color_chance : u32,

    #[structopt(short = "t", long = "turn-chance", default_value = "10")]
    /// The odds of changing direction on each frame.
    ///
    /// Odds are 1 out of <turn_chance>
    pub turn_chance : u32,

    #[structopt(short = "r", long = "reset-chance", default_value = "1000")]
    /// The odds of reseting the screen on each frame.
    ///
    /// Odds are 1 out of <reset_chance>, or set to 0 to never reset.
    pub reset_chance : u32,

    #[structopt(short = "p", long = "pipe-set", default_value = "0")]
    /// Which set of characters to use to draw the pipes.
    pub pipe_set : u32,
}
