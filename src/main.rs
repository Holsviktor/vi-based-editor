use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, MoveLeft, MoveDown, MoveUp, MoveRight, SetCursorStyle},
};
use std::io::{stdout, Write};
use std::fs::{self,File};
use std::io::prelude::*;
use std::io;
mod text;
use text::Text;
use std::env;

const QUIT: i8 = -1;
const NORMAL: i8 = 0;
const INSERT: i8 = 1;

struct RawModeGuard;
impl RawModeGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        terminal::disable_raw_mode();
    }
}

fn refresh_text(buffer : &Text) {
    stdout().execute(SavePosition).unwrap();
    stdout().execute(terminal::Clear(ClearType::All)).unwrap();
    stdout().execute(MoveTo(0,0)).unwrap();
    for line in buffer.get_text().lines() {
        print!("{}", line);
        stdout().execute(MoveDown(1)).unwrap();
        stdout().execute(MoveLeft(line.chars().count() as u16)).unwrap();
    }
    //let (width, height) = terminal::size().ok().unwrap();
    //stdout().execute(MoveTo(0, height)).unwrap();
    //println!("Press q to exit.");
    stdout().execute(RestorePosition).unwrap();
}

fn handle_input_normal(code : KeyCode, buffer : &mut Text) -> i8 {
    let x : u16; let y : u16;
    match crossterm::cursor::position() {
        Ok((col, row)) => {
            x = col; y = row;
        }
        Err(_) => {
            eprintln!("Error getting cursor position");
            return NORMAL;
        }
    }

    return match code {
        KeyCode::Char('q') => -1,
        KeyCode::Char('a') => {
            stdout().execute(MoveRight(1));
            stdout().execute(SetCursorStyle::BlinkingBar).unwrap();
            INSERT
        }

        KeyCode::Char('i') => {
            stdout().execute(SetCursorStyle::BlinkingBar).unwrap();
            INSERT
        }
        KeyCode::Char('h') => {
            if x >= 1 {
                stdout().execute(MoveLeft(1)).unwrap();
            }
            NORMAL
        },
        KeyCode::Char('j') => {
            if buffer.line_count() - 1 > y {
                stdout().execute(MoveDown(1)).unwrap();
                if x >= buffer.get_line_length(1+y as usize) as u16 && x >= 1 {
                    stdout().execute(MoveTo(buffer.get_line_length(1+y as usize) as u16 - 1, y+1)).unwrap();
                }
            }
            NORMAL
        },
        KeyCode::Char('k') => {
            if y > 0 {
                stdout().execute(MoveUp(1)).unwrap();
                if x >= buffer.get_line_length(y as usize - 1) as u16 {
                    stdout().execute(MoveTo(buffer.get_line_length(y as usize - 1)as u16 - 1, y-1)).unwrap();
                }
            }
            NORMAL
        },
        KeyCode::Char('l') => {
            if x + 1 < buffer.get_line_length(y as usize) as u16 {
                stdout().execute(MoveRight(1)).unwrap();
            }
            NORMAL
        },
        KeyCode::Char('r') => {
            refresh_text(&buffer);
            NORMAL
        },
        _ => NORMAL
    }
}

fn handle_input_insert(code : KeyCode, buffer : &mut Text) -> i8 {
    let x : u16; let y : u16;
    match crossterm::cursor::position() {
        Ok((col, row)) => {
            x = col; y = row;
        }
        Err(_) => {
            eprintln!("Error getting cursor position");
            return INSERT;
        }
    }
    let idx : usize = buffer.get_string_index(y as usize, x as usize);

    return match code {
        KeyCode::Esc => {
            if x >= buffer.get_line_length(y as usize) as u16 {
                if buffer.get_line_length(y as usize) > 0 {
                    stdout().execute(MoveTo(buffer.get_line_length(y as usize) as u16 - 1, y)).unwrap();
                }
            }
            stdout().execute(SetCursorStyle::SteadyBlock).unwrap();
            NORMAL
        }
        KeyCode::Enter => {
            if buffer.size() < idx {
                return INSERT;
            }
            buffer.write_char("\n", idx);
            refresh_text(&buffer);
            //print!("\n");
            stdout().execute(MoveTo(0, y + 1)).unwrap();
            stdout().flush().unwrap();
            INSERT

        }
        KeyCode::Char(c) => {
            if buffer.size() < idx {
                return INSERT;
            }
            buffer.write_char(&c.to_string(), idx);
            print!("{}",c);
            stdout().flush().unwrap();
            INSERT
        },
        _ => INSERT
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    let _guard = RawModeGuard::new();
    let args : Vec<String> = env::args().collect();
    let mut buffer: Text;

    if args.len() > 1 {
        let mut file = File::open(args.get(1).unwrap())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            buffer = Text::new(&contents);
    }
    else {
        buffer = Text::new("Usage: editor {filename}");
    }

    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    //let (width, height) = terminal::size().unwrap();
    stdout.execute(MoveTo(0,0)).unwrap();
    refresh_text(&buffer);

    // Main loop
    let mut mode = NORMAL;
    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(KeyEvent {code, ..}) = event::read().unwrap() {
                mode = match mode {
                    NORMAL => {
                        handle_input_normal(code, &mut buffer)
                    },
                    INSERT => {
                        handle_input_insert(code, &mut buffer)
                    },
                    
                    _ => continue,
                }
            }
        }
        if mode == QUIT {
            break;
        }
    }
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    MoveTo(0,0);
    println!("Exiting...");
    if args.len() > 1 {
        //let mut file = File::create(format!("{}{}",args.get(1).unwrap(), ".new"))?;
        //file.write_all(buffer.get_text());
        fs::write(format!("{}{}", args.get(1).unwrap(), ".new"), buffer.get_text());
    }
    return Ok(());
}
