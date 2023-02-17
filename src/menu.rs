use ggez::{graphics::{Canvas, self, Drawable, Color}, Context, mint::Point2};

pub struct MenuOption {
    pub label: String,
}

pub struct Menu {
    pub options: Vec<MenuOption>,
    pub selected_option_index: usize,
}

pub enum MenuMove {
    Up,
    Down
}

impl Menu {
    pub fn draw(&self, canvas: &mut Canvas, ctx: &Context, screen_width: f32) {
        for (idx, opt) in self.options.iter().enumerate() {
            let mut text = graphics::Text::new(&opt.label);
            text.set_font("BungeeShade");

            text.set_scale(graphics::PxScale::from(50.0));
            let text_width = text.dimensions(ctx).unwrap().w;
            let color = if idx == self.selected_option_index { Color::YELLOW } else { Color::WHITE };
            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .color(color)
                    .dest(Point2 {
                        x: screen_width / 2.0 - text_width / 2.0,
                        y: 200.0 + (70 * idx) as f32,
                    }),
            )
        }
    }

    pub fn get_selected_option(&self) -> &MenuOption {
        &self.options[self.selected_option_index]
    }

    pub fn navigate(&mut self, menu_move: MenuMove) {
        match menu_move {
            MenuMove::Up => self.handle_move_up(),
            MenuMove::Down => self.handle_move_down()
        }
    }

    pub fn handle_move_up(&mut self) {
        if self.selected_option_index > 0 {
            self.selected_option_index -= 1;
        }
    }

    pub fn handle_move_down(&mut self) {
        if self.selected_option_index < self.options.len() - 1 {
            self.selected_option_index += 1;
        }
    }
}