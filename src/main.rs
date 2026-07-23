mod app;
mod renderer;

use app::App;

fn main() -> Result<(), String> {
    let mut app = App::new()?;
    app.run();
    Ok(())
}
