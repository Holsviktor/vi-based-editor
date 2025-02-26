use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, MoveLeft, MoveDown, MoveUp, MoveRight},
};
use std::io::{stdout, Write};

const QUIT: i8 = -1;
const NORMAL: i8 = 0;
const INSERT: i8 = 1;

fn handle_input_normal(code : KeyCode) -> i8 {
    return match code {
        KeyCode::Char('q') => -1,
        KeyCode::Char('i') => {
            stdout().execute(EnableBlinking).unwrap();
            INSERT
        }
        KeyCode::Char('h') => {
            stdout().execute(MoveLeft(1)).unwrap();
            NORMAL
        },
        KeyCode::Char('j') => {
            stdout().execute(MoveDown(1)).unwrap();
            NORMAL
        },
        KeyCode::Char('k') => {
            stdout().execute(MoveUp(1)).unwrap();
            NORMAL
        },
        KeyCode::Char('l') => {
            stdout().execute(MoveRight(1)).unwrap();
            NORMAL
        },
        _ => NORMAL
    }
}

fn handle_input_insert(code : KeyCode, buffer : &mut String) -> i8 {
    return match code {
        KeyCode::Esc => {
            stdout().execute(DisableBlinking).unwrap();
            NORMAL
        }
        KeyCode::Enter => {
            buffer.push('\n');
            println!();
            stdout().flush().unwrap();
            INSERT

        }
        KeyCode::Char(c) => {
            buffer.push(c);
            print!("{}",c);
            stdout().flush().unwrap();
            INSERT
        },
        _ => INSERT
    }
}

fn main() {
    let mut stdout = stdout();
    //let mut buffer: Vec<char> = Vec::new();
    //let mut line_lengths: Vec<i16> = vec![0];
    let mut buffer: String = String::new();

    terminal::enable_raw_mode().expect("Failed ot enable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    //let (width, height) = terminal::size().unwrap();
    println!("Press q to exit.");
    stdout.execute(MoveTo(0,0)).unwrap();

    // Main loop
    let mut mode = NORMAL;
    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(KeyEvent {code, ..}) = event::read().unwrap() {
                mode = match mode {
                    NORMAL => {
                        handle_input_normal(code)
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

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    MoveTo(0,0);
    println!("Exiting...");
}
