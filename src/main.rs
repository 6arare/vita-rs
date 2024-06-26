// use fakeit::{last_taken, contact, vitamin};
use std::path::Path;
use std::io::Write;
use std::fs::File;
use serde::{Serialize,Deserialize};
use crossterm::{terminal::{disable_raw_mode,enable_raw_mode,LeaveAlternateScreen,EnterAlternateScreen},
event::{self,Event,KeyEvent,KeyCode,KeyModifiers,KeyEventKind,EnableMouseCapture,DisableMouseCapture},
execute,};
use ratatui::{prelude::*,widgets::*};
// use std::error::Error;
use itertools::Itertools;
use std::{io::{stdout, Result}, path};
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;
const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move down | (→) next color | (←) previous color";
const ITEM_HEIGHT: usize = 4;
struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}
impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
}}
#[derive(Debug,Serialize,Deserialize)]
struct Vitamin {
    vitamin: String,
    last_taken: String,
    next_time: String,
}

impl Vitamin {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.vitamin, &self.last_taken, &self.next_time]
    }

    fn vitamin(&self) -> &str {
        &self.vitamin
    }

    fn last_taken(&self) -> &str {
        &self.last_taken
    }

    fn next_time(&self) -> &str {
            &self.next_time
    }
}
struct App {
    state: TableState,
    items: Vec<Vitamin>,
    longest_item_lens: (u16, u16, u16), // order is (vitamin, last_taken, next_time)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    fn new() -> Self {
        let data_vec = generate_fake_vitamins();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }
}

    fn generate_fake_vitamins() -> Vec<Vitamin> {
    
        (0..20)
            .map(|_| {
                let vitamin = "Vitamin C".to_string();
                let last_taken = format!(
                    "{}",
                    // "ahn".to_string(),
                    // "ahn".to_string(),
                    // "ahn".to_string(),
                     "ahn".to_string()
                );
                let next_time = "7 days".to_string();
    
                Vitamin {
                    vitamin,
                    last_taken,
                    next_time,
                }
            })
            .sorted_by(|a, b| a.vitamin.cmp(&b.vitamin))
            .collect_vec()
    }
    
        
fn main()-> Result<()> {
    let mut vitamin_list:Vec<String>;
    enable_raw_mode()?;
    let mut stdout = stdout();
execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
let backend=CrosstermBackend::new(stdout);
let mut terminal=Terminal::new(backend)?;
let app=App::new();
let res=run_app(&mut terminal,app);
disable_raw_mode()?;
execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
)?;
terminal.show_cursor()?;

if let Err(err) = res {
    println!("{err:?}");
}
Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>{
    let path =Path::new("data.txt");
    let mut data_file=File::create(&path)?;
    let serialized = serde_json::to_string(&data_file).unwrap();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        let key = event::read()?;
        // if let Event::Key(key) = event::read()? {
            // if key.kind == KeyEventKind::Press {
                match key{
                    Event::Key(KeyEvent{
                        modifiers:KeyModifiers::CONTROL,
                        code:KeyCode::Char('s'),
                        ..})=>
                        data_file.write_all(serialized.as_bytes())?,
                        Event::Key(KeyEvent{
                            modifiers:KeyModifiers::CONTROL,
                            code:KeyCode::Char('q'),
                            ..})  => return Ok(()),
                        Event::Key(KeyEvent{code:KeyCode::Down,..}) => app.next(),
                        Event::Key(KeyEvent{code:KeyCode::Up,..}) => app.previous(),
                        Event::Key(KeyEvent{code:KeyCode::Right,..}) => app.next_color(),
                        Event::Key(KeyEvent{code:KeyCode::Left,..}) => app.previous_color(),
                        _ => {}
                    // }}

            }
            // match key.code {
            }
        // }
    // }


fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(f.size());

    app.set_colors();

    render_table(f, app, rects[0]);

    render_scrollbar(f, app, rects[0]);

    render_footer(f, app, rects[1]);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = ["pill", "last_taken", "next_one"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Length(app.longest_item_lens.0 + 1),
            Constraint::Min(app.longest_item_lens.1 + 1),
            Constraint::Min(app.longest_item_lens.2),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(app.colors.buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn constraint_len_calculator(items: &[Vitamin]) -> (u16, u16, u16) {
    let vitamin_len = items
        .iter()
        .map(Vitamin::vitamin)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let address_len = items
        .iter()
        .map(Vitamin::last_taken)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let email_len = items
        .iter()
        .map(Vitamin::next_time)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (vitamin_len as u16, address_len as u16, email_len as u16)
}

fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

fn render_footer(f: &mut Frame, app: &App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .centered()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Double),
        );
    f.render_widget(info_footer, area);
}}




