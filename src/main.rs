use crossterm::event::{Event, read};


fn main() -> crossterm::Result<()> {
    loop {
        match read()? {
            Event::Key(key_event: key_event) => {},
            Event::Mouse() => {},
            Event::Resize(_, _) => {}
        }
    }
}
