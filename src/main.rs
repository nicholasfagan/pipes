
mod config;
use pipes::config::Config;
use structopt::StructOpt;

fn main() {
    pipes::run(Config::from_args());
}

