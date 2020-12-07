use clap::{App, SubCommand};
use std::io::{self, BufRead};
use term;

fn main() {
    let matches = App::new("printy")
        .version("0.1.0")
        .author("Galen Guyer <galen@galenguyer.com>")
        .about("prints things prettily")
        .subcommand(SubCommand::with_name("info").about("adds the info prefix"))
        .subcommand(SubCommand::with_name("error").about("adds the error prefix"))
        .subcommand(SubCommand::with_name("ok").about("adds the ok prefix"))
        .get_matches();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(_matches) = matches.subcommand_matches("info") {
            info();
        } else if let Some(_matches) = matches.subcommand_matches("ok") {
            ok();
        } else if let Some(_matches) = matches.subcommand_matches("error") {
            error();
        }
        println!("{}", line.unwrap());
    }
}

fn info() {
    let mut terminal = term::stdout().unwrap();
    print!("[");
    terminal.fg(term::color::BRIGHT_BLUE).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    print!("i");
    terminal.reset().unwrap();
    print!("] ");
}
fn ok() {
    let mut terminal = term::stdout().unwrap();
    print!("[");
    terminal.fg(term::color::GREEN).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    print!("+");
    terminal.reset().unwrap();
    print!("] ");
}
fn error() {
    let mut terminal = term::stdout().unwrap();
    print!("[");
    terminal.fg(term::color::RED).unwrap();
    terminal.attr(term::Attr::Bold).unwrap();
    print!("X");
    terminal.reset().unwrap();
    print!("] ");
}
