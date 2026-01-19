use crossterm::event;

fn main() -> std::io::Result<()> {
    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("A MI DINA LA AMO INCLUSO MAS", frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}