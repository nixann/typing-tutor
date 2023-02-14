use std::{env, path};

use ggez::conf::{Conf, WindowMode};
use ggez::{ContextBuilder};
use ggez::event::{self};
use typing_tutor::game::Game;



// load font
// let font_data = graphics::FontData::from_path(&ctx, "/DejaVuSerif.ttf").unwrap();
// ctx.gfx.add_font("MainFont", font_data);

fn main() {
    
     // Конфигурация:
     let conf = Conf::new().
     window_mode(WindowMode {
        width: 1200.0,
        height: 1000.0,
        ..Default::default()
    });

 // Контекст и event loop
 let (mut ctx, event_loop) = ContextBuilder::new("shooter", "FMI").
     default_conf(conf.clone()).
     build().
     unwrap();

    // prepare resources
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.fs.mount(&path, true);
}
    // Пускане на главния loop
    let state = Game::new(&mut ctx);
    event::run(ctx, event_loop, state);
}