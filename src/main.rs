mod sorting;

use std::io;
use std::ops::Range;
use std::sync::{Arc, RwLock};
use std::thread;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug, Clone)]
struct SortingVec {
    data: Arc<RwLock<Vec<u16>>>,
    change: Arc<RwLock<Option<ChangeRecord>>>,
    read: Arc<RwLock<Option<Vec<usize>>>>,
    start: usize,
    end: usize,
}

impl Default for SortingVec {
    fn default() -> Self {
        SortingVec {
            data: Arc::new(RwLock::new(vec![])),
            change: Arc::new(RwLock::new(None)),
            read: Arc::new(RwLock::new(None)),
            start: 0,
            end: 0,
        }
    }
}

impl SortingVec {
    fn len(&self) -> usize {
        self.end - self.start
    }

    fn get(&self, idx: usize) -> u16 {
        *self.read.write().unwrap() = Some(vec![self.start + idx]);
        std::thread::sleep(std::time::Duration::from_millis(50));
        let val = self.data.read().unwrap()[self.start + idx];
        *self.read.write().unwrap() = None;
        val
    }

    fn swap(&mut self, i: usize, j: usize) {
        *self.change.write().unwrap() = Some(ChangeRecord {
            nums: (i + self.start, j + self.start),
        });

        std::thread::sleep(std::time::Duration::from_millis(500));

        self.data
            .write()
            .unwrap()
            .swap(self.start + i, self.start + j);

        std::thread::sleep(std::time::Duration::from_millis(500));

        *self.change.write().unwrap() = None;
    }

    fn sub(&self, idx: Range<usize>, color: bool) -> SortingVec {
        if color {
            self.read
                .write() // if we sub a single element, we want to read it for the visualization
                .unwrap()
                .replace(vec![self.start + idx.start]);
            std::thread::sleep(std::time::Duration::from_millis(500));
            let r = SortingVec {
                data: self.data.clone(),
                change: Arc::new(RwLock::new(None)),
                read: Arc::new(RwLock::new(Some(vec![self.start + idx.start]))),
                start: self.start + idx.start,
                end: self.start + idx.end,
            };
            std::thread::sleep(std::time::Duration::from_millis(500));
            self.read.write().unwrap().replace(vec![]);
            r
        } else {
            SortingVec {
                data: self.data.clone(),
                change: self.change.clone(),
                read: self.read.clone(),
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
}

impl App {}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Sorting Visualization App ".bold());
        let instructions = Line::from(vec![
            " ".into(),
            "<q>".blue().into(),
            ": Quicksort ".into(),
            "<i>".blue().into(),
            ": Insertion ".into(),
            "<b>".blue().into(),
            ": Binary Insertion ".into(),
            "<s>".blue().into(),
            ": Selection ".into(),
            "<u>".blue().into(),
            ": Bubble ".into(),
            "<o>".blue().into(),
            ": Bubble Opt ".into(),
            "<c>".blue().into(),
            ": Cocktail ".into(),
            "<h>".blue().into(),
            ": Shell ".into(),
            "<p>".blue().into(),
            ": Heap ".into(),
            "<m>".blue().into(),
            ": Merge ".into(),
            "<k>".blue().into(),
            ": Counting ".into(),
            "<e>".red().into(),
            ": Exit ".into(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let borrowed = self.list.data.read().unwrap();
        let slice = &borrowed[self.list.start..self.list.end];

        let mut v = vec![];
        let change_lock = self.list.change.read().unwrap();
        for i in 0..slice.len() {
            let num_str = format!(
                "{}{}{} ",
                if slice[i] < 100 { " " } else { "" },
                if slice[i] < 10 { " " } else { "" },
                slice[i]
            );

            let style = if let Some(r @ ChangeRecord { .. }) = change_lock.as_ref() {
                if r.nums.0 == i || r.nums.1 == i {
                    Style::default().fg(Color::Red).bold()
                } else {
                    Style::default()
                }
            } else {
                if let Some(read_lock) = self.list.read.read().unwrap().as_ref() {
                    if read_lock.contains(&(self.list.start + i)) {
                        Style::default().fg(Color::Yellow).bold()
                    } else {
                        Style::default()
                    }
                } else {
                    Style::default()
                }
            };

            v.push(Span::styled(num_str, style));
        }

        let nums_text = Text::from(vec![Line::from(v)]);

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
            KeyCode::Char('i') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::insertion_sort::sort(&mut list);
                });
            }
            KeyCode::Char('b') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::binary_insertion_sort::sort(&mut list);
                });
            }
            KeyCode::Char('s') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::selection_sort::sort(&mut list);
                });
            }
            KeyCode::Char('u') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::bubble_sort::sort(&mut list);
                });
            }
            KeyCode::Char('o') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::bubble_sort_optimized::sort(&mut list);
                });
            }
            KeyCode::Char('c') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::cocktail_sort::sort(&mut list);
                });
            }
            KeyCode::Char('h') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::shell_sort::sort(&mut list);
                });
            }
            KeyCode::Char('p') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::heap_sort::sort(&mut list);
                });
            }
            KeyCode::Char('m') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::merge_sort::sort(&mut list);
                });
            }
            KeyCode::Char('k') => {
                let mut list = self.list.clone();
                thread::spawn(move || {
                    sorting::counting_sort::sort(&mut list);
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
