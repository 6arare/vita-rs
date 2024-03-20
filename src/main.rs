use crossterm::{terminal::{disable_raw_mode,enable_raw_mode,LeaveAlternateScreen,EnterAlternateScreen},
event::{self,KeyCode,KeyEventKind},
ExecutableCommand,};

use ratatui::{prelude::{CrosstermBackend,Stylize,Terminal},
widgets::Paragraph, Frame,
};
use std::io::{stdout,Result, Stdout};
fn main()-> Result<()> {
stdout().execute(EnterAlternateScreen)?;
enable_raw_mode()?;
let mut terminal=Terminal::new(CrosstermBackend::new(stdout()))?;
 
 loop {
 terminal.draw(|frame|{let area=frame.size(); 
frame.render_widget(Paragraph::new("vita-rs (press q to quit)").black().on_dark_gray(),area,);})?;    
if event::poll(std::time::Duration::from_millis(16))? {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            break;
        }
    }
}


 }


stdout().execute(EnterAlternateScreen)?;
disable_raw_mode()?;
Ok(())
}
