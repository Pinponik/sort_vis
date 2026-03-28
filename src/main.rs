mod sorting;

use std::io;
use std::ops::Range;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Clone)]
struct SortingVec {
    data: Arc<RwLock<Vec<u16>>>,
    start: usize,
    end: usize,
}

impl Default for SortingVec {
    fn default() -> Self {
        SortingVec {
            data: Arc::new(RwLock::new(vec![])),
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
        thread::sleep(Duration::from_secs(2));

        self.data
            .write()
            .unwrap()
            .swap(self.start + i, self.start + j);
    }

    fn sub(&self, idx: Range<usize>) -> SortingVec {
        SortingVec {
            data: self.data.clone(),
            start: self.start + idx.start,
            end: self.start + idx.end,
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
    change: ChangeRecord,
    read: Option<Vec<usize>>,
    exit: bool,
}

#[derive(Debug, Default)]
pub struct ChangeRecord {
    nums: (usize, usize),
    state: u8,
}

impl App {}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
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

        let borrowed = self.list.data.read().unwrap();
        let slice = &borrowed[self.list.start..self.list.end];
        let nums_text = Text::from(vec![Line::from(
            slice
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )]);

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
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
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

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
}
