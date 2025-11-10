use std::io::{self, Stdout, Write};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rustgame::games::puzzle_game::Puzzle16;

fn main() -> io::Result<()> {
    let mut puz = Puzzle16::new();
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    let result = run_puzzle_loop(&mut puz, &mut stdout);

    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    result
}

fn run_puzzle_loop(puz: &mut Puzzle16, stdout: &mut Stdout) -> io::Result<()> {
    loop {
        stdout.execute(cursor::MoveTo(0, 0))?;
        stdout.execute(terminal::Clear(ClearType::FromCursorDown))?;
        puz.show(stdout)?;
        writeln!(stdout)?;
        writeln!(stdout, "使用 WASD 移動，按 q 或 Esc 離開。")?;
        stdout.flush()?;

        match event::read()? {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Up => puz.action(0),
                KeyCode::Down => puz.action(1),
                KeyCode::Left => puz.action(2),
                KeyCode::Right => puz.action(3),
                KeyCode::Char('w') | KeyCode::Char('W') => puz.action(0),
                KeyCode::Char('s') | KeyCode::Char('S') => puz.action(1),
                KeyCode::Char('a') | KeyCode::Char('A') => puz.action(2),
                KeyCode::Char('d') | KeyCode::Char('D') => puz.action(3),
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break Ok(()),
                _ => {}
            },
            _ => {}
        }

        if puz.iswin() == true {
            break Ok(());
        }
    }
}
