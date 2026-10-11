mod app
use app::App;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};

fn main() -> Result<()> {
    let mut app = App::new();
    while app.running {
        if let Event::Key(k) = event::read()? {
            if k.code == KeyCode::Char('q') { app.quit();}
        }   
     }
     Ok(())
}