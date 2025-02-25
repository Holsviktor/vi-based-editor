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
    match code {
        KeyCode::Char('q') => -1,
        KeyCode::Char('i') => return INSERT,

        KeyCode::Char('h') => {
            stdout().execute(MoveLeft(1)).unwrap();
            return NORMAL;
        },
        KeyCode::Char('j') => {
            stdout().execute(MoveDown(1)).unwrap();
            return NORMAL;
        },
        KeyCode::Char('k') => {
            stdout().execute(MoveUp(1)).unwrap();
            return NORMAL;
        },
        KeyCode::Char('l') => {
            stdout().execute(MoveRight(1)).unwrap();
            return NORMAL;
        },
        _ => return NORMAL,
    }
}

fn handle_input_insert(code : KeyCode, buffer : &mut String) -> i8 {
    match code {
        KeyCode::Esc => return NORMAL,
        KeyCode::Char(c) => {
            buffer.push(c);
            print!("{}",c);
            //stdout.flush.unwrap();
            stdout().execute(MoveRight(1)).unwrap();
        },
        _ => return INSERT,


    }
    return INSERT;
}

fn main() {
    let mut stdout = stdout();
    let mut buffer = String::new();

    terminal::enable_raw_mode().expect("Failed ot enable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    MoveTo(0,0);
    println!("Press q to exit.");

    // Main loop
    let mut mode = NORMAL;
    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(KeyEvent {code, ..}) = event::read().unwrap() {
                mode = match mode {
                    NORMAL => handle_input_normal(code),
                    INSERT => handle_input_insert(code, &mut buffer),
                    
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
