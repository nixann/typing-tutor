use std::{env, path};

use ggez::conf::{Conf, WindowMode};
use ggez::event::{self};
use ggez::{graphics, ContextBuilder};
use typing_tutor::game::Game;

fn main() {
    let conf = Conf::new().window_mode(WindowMode {
        width: 1200.0,
        height: 1000.0,
        ..Default::default()
    });

    let (mut ctx, event_loop) = ContextBuilder::new("shooter", "FMI")
        .default_conf(conf.clone())
        .build()
        .unwrap();

    // prepare resources
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }

    // load fonts
    let font_data = graphics::FontData::from_path(&ctx, "/GravitasOne.ttf").unwrap();
    ctx.gfx.add_font("GravitasOne", font_data);
    let font_data = graphics::FontData::from_path(&ctx, "/BungeeShade.ttf").unwrap();
    ctx.gfx.add_font("BungeeShade", font_data);
    let font_data = graphics::FontData::from_path(&ctx, "/Creepster.ttf").unwrap();
    ctx.gfx.add_font("Creepster", font_data);
    let state = Game::new(&conf);
    event::run(ctx, event_loop, state);
}
