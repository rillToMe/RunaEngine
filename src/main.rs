mod engine;

use engine::App;

fn main() {
    let app = App::new("My 2D Engine", 1280, 720);

    app.run();
}