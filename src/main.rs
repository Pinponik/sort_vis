mod sorting;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::DefaultTerminal;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::ops::Range;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));
    println!("Starting Ratatui app");
    if let Err(e) = enable_raw_mode() {
        println!("Enable raw mode error: {}", e);
        return Ok(());
    }
    println!("Raw mode enabled");
    if let Err(e) = execute!(io::stdout(), EnterAlternateScreen) {
        println!("Enter alternate screen error: {}", e);
        return Ok(());
    }
    println!("Alternate screen entered");
    let result = ratatui::run(|terminal| App::default().run(terminal));
    println!("After ratatui run");
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
    println!("App finished");
    result
}

#[derive(Debug, Clone)]
struct SortingVec {
    data: Arc<RwLock<Vec<u16>>>,
    change: Arc<Mutex<Option<ChangeRecord>>>,
    read: Arc<Mutex<Option<Vec<usize>>>>,
    start: usize,
    end: usize,
}

impl Default for SortingVec {
    fn default() -> Self {
        SortingVec {
            data: Arc::new(RwLock::new(vec![])),
            change: Arc::new(Mutex::new(None)),
            read: Arc::new(Mutex::new(None)),
            start: 0,
            end: 0,
        }
    }
}

impl SortingVec {
    fn len(&self) -> usize {
        self.end - self.start
    }

    fn swap(&mut self, i: usize, j: usize) {
        *self.change.lock().unwrap() = Some(ChangeRecord {
            nums: (self.start + i, self.start + j),
            state: 0,
        });

        std::thread::sleep(std::time::Duration::from_millis(500));

        self.data
            .write()
            .unwrap()
            .swap(self.start + i, self.start + j);

        std::thread::sleep(std::time::Duration::from_millis(500));

        *self.change.lock().unwrap() = None;
    }

    fn sub(&self, idx: Range<usize>) -> SortingVec {
        if idx.end - idx.start == 1 {
            return SortingVec {
                data: self.data.clone(),
                change: Arc::new(Mutex::new(None)),
                read: Arc::new(Mutex::new(Some(vec![self.start]))),
                start: self.start + idx.start,
                end: self.start + idx.end,
            };
        } else {
            SortingVec {
                data: self.data.clone(),
                change: Arc::new(Mutex::new(None)),
                read: Arc::new(Mutex::new(None)),
                start: self.start + idx.start,
                end: self.start + idx.end,
            }
        }
    }

    fn push(&mut self, value: u16) {
        self.data.write().unwrap().insert(self.end, value);
        self.end += 1;
    }

    fn pop(&mut self) -> Option<u16> {
        if self.end > self.start {
            self.end -= 1;
            Some(self.data.write().unwrap().remove(self.end))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    list: SortingVec,
    exit: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChangeRecord {
    nums: (usize, usize),
    state: u8,
}

impl App {}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        eprintln!("Rendering app");
        let borrowed = self.list.data.read().unwrap();
        let slice = &borrowed[self.list.start..self.list.end];

        let change_lock = self.list.change.lock().unwrap();

        let title = if change_lock.is_some() {
            Line::from(" Sorting in progress... ".bold())
        } else {
            Line::from(" Sorting Visualization App ".bold())
        };
        let instructions = Line::from(vec![
            " Quicksort ".into(),
            "<q>".blue().into(),
            " Exit ".into(),
            " <e> ".red().into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut v = vec![];
        let change_lock = self.list.change.lock().unwrap();
        for i in 0..slice.len() {
            let num_str = format!(
                "{}{}{} ",
                if slice[i] < 100 { " " } else { "" },
                if slice[i] < 10 { " " } else { "" },
                slice[i]
            );

            let style = if let Some(r @ ChangeRecord { .. }) = change_lock.as_ref() {
                if r.nums.0 == i || r.nums.1 == i {
                    Style::default().bg(Color::Red).fg(Color::White).bold()
                } else {
                    Style::default()
                }
            } else {
                Style::default()
            };

            v.push(Span::styled(num_str, style));
        }

        let nums_text = Text::from(vec![
            Line::from(v),
            Line::from(format!("Length: {}", slice.len())),
        ]);

        Paragraph::new(nums_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl App {
    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('e') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char('q') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::quicksort::sort(&mut list);
                });
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.list.push(rand::random_range(0..=999));
    }

    fn decrement_counter(&mut self) {
        self.list.pop();
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        eprintln!("Starting run loop");
        while !self.exit {
            eprintln!("In run loop");
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        eprintln!("Exiting run loop");
        Ok(())
    }
}
