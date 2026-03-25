use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        let list: Vec<i64> = vec![0, 3, 25, 1, 237, 2];
        terminal.draw(|frame| {
            let block = Block::bordered().title("Sorting...");
            let greeting = Paragraph::new(
                list.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .centered()
            .yellow()
            .block(block);
            frame.render_widget(greeting, frame.area());
        })?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        Ok(())
    })
}
