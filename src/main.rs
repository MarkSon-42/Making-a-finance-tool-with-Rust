use crossterm::event::{Event, read};

fn main() -> crossterm::Result<()> {
    loop {
        match read()? {
            Event::Key() =>
        }
    }
}
