use std::fs::File;
use std::process;
use std::io::prelude::*;
use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

pub enum Mode {
    Normal,
    Insert,
}

pub fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_name);
    let mut file_string = String::new();
    if let Ok(mut file) = file {
        file.read_to_string(&mut file_string)?;
    } 

    Ok(file_string)
}

pub fn edit_file(file: &mut File, file_string: &mut String) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    write!(stdout, "{}{}{file_string}",
           // clear terminal
           clear::All,
           // move cursor to 1,1
           cursor::Goto(1,0)).unwrap();

           stdout.flush().unwrap();

           let mut i: u16 = file_string.len() as u16;
           let mut j: u16 = 0;

           let mode = Mode::Insert;

           for c in stdin.keys() {
               match c.unwrap() {
                   Key::Ctrl(_c) => break,
                   Key::Left => {
                       i -= 1; 
                       write!(stdout, "{}", cursor::Goto(i,j)).unwrap();
                   }
                   Key::Right => {
                       i += 1; 
                       write!(stdout, "{}", cursor::Goto(i,j)).unwrap();
                   }
                   Key::Up => {
                       if j <= 0 {
                           continue;
                       }
                       j -= 1; 
                       write!(stdout, "{}", cursor::Goto(i,j)).unwrap();
                   }
                   Key::Backspace => {
                       if i > 0 {
                           i -= 1;
                           file_string.remove(i as usize);
                       }
                   },
                   // Key::Enter => {
                   //     file_string.insert(i as usize, '\n');
                   //     i += 1;
                   // },
                   Key::Char(x) => {
                       match mode {
                           Mode::Normal => println!("Visual"),
                           Mode::Insert => {
                               file_string.insert(i as usize, x);
                               i += 1;
                           },
                       }
                   },
                   _ => { 
                       // eprintln!("unknown character");
                       // process::exit(2);
                   },
               }
               write!(stdout, "{}{}", clear::All, cursor::Goto(1,1)).unwrap();
               write!(stdout, "{}",file_string).unwrap();
               write!(stdout, "{}", cursor::Goto(i+1,j)).unwrap();
               stdout.flush().unwrap();
           }
           
           fn normal_key(key: &Key, s: &mut String, i: usize) {
               match key {
                   Key::Char('h') => {

                   },
                   _ => {}
               }
           }

           write!(file, "{}", file_string).unwrap();
           write!(stdout, "{}", termion::cursor::Show).unwrap();
}
