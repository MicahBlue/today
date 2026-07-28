use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::env::args;
use std::fs;
use rand::RngExt;

fn main(){
    println!("Try to click the down arrow 78 times without looking");
    println!("Press control c when you are done");
    print!("{}", termion::clear::All);
    let mut count: u64 = 0;
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    for c in stdin.keys(){
        match c.unwrap(){
            Key::Ctrl ('c') => {
                break;
            }
            Key::Down => {
                count += 1;
            }
            _=>{}
        }
        
        termion::cursor::Goto(5,10);
        print!("{}", termion::clear::All);
    }
    stdout.flush().unwrap();
    print!("{}", termion::clear::All);
    if count == 78 {
       // color::Bg(color::Cyan)
        println!("You won!!!");
    }
    else {
        println!("You lost!!!");
    }
}
