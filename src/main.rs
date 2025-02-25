use crossterm::{
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, RestorePosition, SavePosition, MoveLeft, MoveDown, MoveUp, MoveRight},
};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();

    terminal::enable_raw_mode().expect("Failed ot enable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    MoveTo(0,0);
    println!("Press q to exit.");

    loop {
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(KeyEvent {code, ..}) = event::read().unwrap() {
                match code {
                    KeyCode::Char('q') => break,

                    KeyCode::Char('h') => {
                        stdout.execute(MoveLeft(1)).unwrap();
                    },
                    KeyCode::Char('j') => {
                        stdout.execute(MoveDown(1)).unwrap();
                    },
                    KeyCode::Char('k') => {
                        stdout.execute(MoveUp(1)).unwrap();
                    },
                    KeyCode::Char('l') => {
                        stdout.execute(MoveRight(1)).unwrap();
                    },
                    _ => continue,
                }
            }
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    MoveTo(0,0);
    println!("Exiting...");
}
