use std::collections::{HashMap, VecDeque};

use ggez::conf::Conf;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, Drawable};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::seq::SliceRandom;
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

use crate::constants::SOURCE_WORDS;
use crate::word::{Word, WordEffect};

const WORD_SCORE: u32 = 10;

const INITIAL_TIME_UNTIL_NEXT_WORD: f32 = 1.0;
const INITIAL_GAME_SPEED: u32 = 50;

pub fn create_key_codes_map() -> HashMap<keyboard::KeyCode, char> {
    HashMap::from([
        (keyboard::KeyCode::A, 'a'),
        (keyboard::KeyCode::B, 'b'),
        (keyboard::KeyCode::C, 'c'),
        (keyboard::KeyCode::D, 'd'),
        (keyboard::KeyCode::E, 'e'),
        (keyboard::KeyCode::F, 'f'),
        (keyboard::KeyCode::G, 'g'),
        (keyboard::KeyCode::H, 'h'),
        (keyboard::KeyCode::I, 'i'),
        (keyboard::KeyCode::J, 'j'),
        (keyboard::KeyCode::K, 'k'),
        (keyboard::KeyCode::L, 'l'),
        (keyboard::KeyCode::M, 'm'),
        (keyboard::KeyCode::N, 'n'),
        (keyboard::KeyCode::O, 'o'),
        (keyboard::KeyCode::P, 'p'),
        (keyboard::KeyCode::Q, 'q'),
        (keyboard::KeyCode::R, 'r'),
        (keyboard::KeyCode::S, 's'),
        (keyboard::KeyCode::T, 't'),
        (keyboard::KeyCode::U, 'u'),
        (keyboard::KeyCode::V, 'v'),
        (keyboard::KeyCode::W, 'w'),
        (keyboard::KeyCode::X, 'x'),
        (keyboard::KeyCode::Y, 'y'),
        (keyboard::KeyCode::Z, 'z'),
    ])
}

pub struct Game {
    screen_height: f32,
    screen_width: f32,
    key_codes_map: HashMap<keyboard::KeyCode, char>,
    is_game_running: bool,
    words: VecDeque<Word>,
    next_word_loop_length: f32,
    current_score: u32,
    life_points: u32,
    game_speed: u32,
    time_until_next_word: Option<f32>,
    game_speed_before_slow_down: Option<u32>,
    passed_time_since_game_end: Option<f32>,
    slow_down_time_left: Option<f32>,
    spawn_only_short_words_time_left: Option<f32>,
}

impl Game {
    pub fn new(conf: &Conf) -> Game {
        // Load/create resources such as images here.
        Game {
            is_game_running: false,
            key_codes_map: create_key_codes_map(),
            next_word_loop_length: INITIAL_TIME_UNTIL_NEXT_WORD,
            words: VecDeque::new(),
            screen_height: conf.window_mode.height,
            screen_width: conf.window_mode.width,
            current_score: 0,
            life_points: 0,
            game_speed: INITIAL_GAME_SPEED,
            time_until_next_word: None,
            game_speed_before_slow_down: None,
            passed_time_since_game_end: None,
            slow_down_time_left: None,
            spawn_only_short_words_time_left: None,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(first_word) = self.words.front() {
            if first_word.position.y >= self.screen_height {
                if self.life_points > 0 {
                    self.life_points -= 1;
                    self.words.pop_front();
                } else {
                    self.end_game()?
                }
            }
        }

        if !self.is_game_running {
            return Ok(());
        }

        let last_frame_length = ctx.time.delta().as_secs_f32();
        if let Some(slow_down_time_left) = self.slow_down_time_left {
            if slow_down_time_left <= 0.0 {
                self.slow_down_time_left = None;
                if let Some(game_speed_before_slow_down) = self.game_speed_before_slow_down {
                    self.game_speed = game_speed_before_slow_down;
                    self.game_speed_before_slow_down = None
                }
            } else {
                self.slow_down_time_left = Some(slow_down_time_left - last_frame_length);
            }
        }
        self.update_words_positions(self.game_speed as f32 * last_frame_length);
        if let Some(time_until_next_word) = self.time_until_next_word {
            if time_until_next_word <= 0.0 {
                let mut new_word_limit = None;
                if let Some(short_words_time_left) = self.spawn_only_short_words_time_left {
                    if short_words_time_left <= 0.0 {
                        self.spawn_only_short_words_time_left = None
                    } else {
                        new_word_limit = Some(3);
                    }
                }
                self.spawn_new_word(new_word_limit);
                self.time_until_next_word = Some(self.next_word_loop_length);
            } else {
                self.time_until_next_word = Some(time_until_next_word - last_frame_length);
            }
        }

        if let Some(short_words_time_left) = self.spawn_only_short_words_time_left {
            self.spawn_only_short_words_time_left = Some(short_words_time_left - last_frame_length);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.19, 0.2, 0.45, 0.65));
        if let Some(time_passed) = self.passed_time_since_game_end {
            if time_passed < 4.0 {
                self.draw_end_game_message(&mut canvas, ctx);
                self.passed_time_since_game_end =
                    Some(time_passed + ctx.time.delta().as_secs_f32());
            } else {
                self.passed_time_since_game_end = None
            }
        }
        if !self.is_game_running {
            self.draw_home_screen(&mut canvas, ctx);
        } else {
            self.draw_player_stats(&mut canvas);
            self.draw_words(&mut canvas);
        }
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(input_key_code) = input.keycode {
            if let Some(current_word) = self.words.front_mut() {
                if let Some(typed_letter) = self.key_codes_map.get(&input_key_code) {
                    current_word.handle_typed_letter(*typed_letter)
                }

                if current_word.is_completed() {
                    self.complete_word();
                    if self.next_word_loop_length > 0.001 {
                        self.next_word_loop_length -= 0.001;
                    }
                }
            }

            if !self.is_game_running && input_key_code == keyboard::KeyCode::Space {
                self.start_game()?
            }
        }
        return Ok(());
    }
}

impl Game {
    fn draw_home_screen(&self, canvas: &mut Canvas, ctx: &Context) {
        let mut text = graphics::Text::new("PRESS SPACE TO START");
        text.set_font("SecondaryFont");

        text.set_scale(graphics::PxScale::from(50.0));
        let text_width = text.dimensions(ctx).unwrap().w;
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .color(Color::WHITE)
                .dest(Point2 {
                    x: self.screen_width / 2.0 - text_width / 2.0,
                    y: 200.0,
                }),
        )
    }

    fn draw_player_stats(&self, canvas: &mut Canvas) {
        let mut text = graphics::Text::new(format!("SCORE: {}", self.current_score));
        text.set_font("SecondaryFont");
        text.set_scale(graphics::PxScale::from(50.0));
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .color(Color::new(0.8, 0.4, 0.17, 0.88))
                .dest(Point2 {
                    x: 30.0,
                    y: self.screen_height - 100.0,
                }),
        );

        let mut text = graphics::Text::new(format!("LIFES: {}", self.life_points));
        text.set_font("SecondaryFont");
        text.set_scale(graphics::PxScale::from(50.0));
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .color(Color::GREEN)
                .dest(Point2 {
                    x: 30.0,
                    y: self.screen_height - 175.0,
                }),
        );
    }

    fn draw_words(&self, canvas: &mut Canvas) {
        for word in &self.words {
            let mut text = graphics::Text::new(word.get_display_value());
            text.set_font(word.get_font());
            text.set_scale(graphics::PxScale::from(40.0));
            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .color(word.get_color())
                    .dest(word.position),
            );
        }
    }

    fn draw_end_game_message(&self, canvas: &mut Canvas, ctx: &Context) {
        let mut text = graphics::Text::new("YOU LOST");
        text.set_font("ErrorFont");
        text.set_scale(graphics::PxScale::from(100.0));
        let text_width = text.dimensions(ctx).unwrap().w;
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .color(Color::RED)
                .dest(Point2 {
                    x: self.screen_width / 2.0 - text_width / 2.0,
                    y: 100.0,
                }),
        );
    }

    fn start_game(&mut self) -> GameResult {
        self.is_game_running = true;
        self.current_score = 0;
        self.life_points = 0;
        self.time_until_next_word = Some(INITIAL_TIME_UNTIL_NEXT_WORD);
        self.game_speed = INITIAL_GAME_SPEED;
        self.passed_time_since_game_end = None;

        Ok(())
    }

    fn end_game(&mut self) -> GameResult {
        self.is_game_running = false;
        self.words.clear();
        self.passed_time_since_game_end = Some(0.0);
        Ok(())
    }

    fn update_words_positions(&mut self, delta_y: f32) {
        for word in &mut self.words {
            word.update_position(delta_y)
        }
    }

    fn spawn_new_word(&mut self, length_limit: Option<usize>) {
        let source_words: Vec<&str>;
        if let Some(limit) = length_limit {
            source_words = SOURCE_WORDS
                .iter()
                .filter(|&word| word.graphemes(true).count() <= limit)
                .map(|w| *w)
                .collect();
        } else {
            source_words = Vec::from(SOURCE_WORDS)
        }
        let word = source_words.choose(&mut rand::thread_rng()).unwrap();
        let word_position = Point2 {
            x: rand::thread_rng().gen_range(0.0..self.screen_width - 200.0),
            y: 0.0,
        };
        self.words.push_back(Word::new(word, word_position, 0));
    }

    fn complete_word(&mut self) {
        let word = self.words.pop_front().unwrap();
        self.current_score += WORD_SCORE;
        self.game_speed += 10;
        if let Some(effect) = word.effect {
            self.handle_word_effect(effect)
        }
    }

    fn handle_word_effect(&mut self, effect: WordEffect) {
        match effect {
            WordEffect::AddLife => self.life_points += 1,
            WordEffect::SlowDown => {
                self.slow_down_time_left = Some(5.0);
                self.game_speed_before_slow_down = Some(self.game_speed);
                self.game_speed = 25;
            }
            WordEffect::SpawnOnlyShortWords => self.spawn_only_short_words_time_left = Some(5.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use ggez::conf::WindowMode;

    use super::*;

    fn create_game() -> Game {
        let conf = Conf::new().window_mode(WindowMode {
            width: 1200.0,
            height: 1000.0,
            ..Default::default()
        });
        Game::new(&conf)
    }

    #[test]
    fn it_initializes_game_with_defautl_state() {
        let game = create_game();

        assert_eq!(game.screen_height, 1000.0);
        assert_eq!(game.screen_width, 1200.0);
        assert_eq!(game.key_codes_map, create_key_codes_map());
        assert_eq!(game.is_game_running, false);
        assert_eq!(game.words.len(), 0);
        assert_eq!(game.next_word_loop_length, INITIAL_TIME_UNTIL_NEXT_WORD);
        assert_eq!(game.current_score, 0);
        assert_eq!(game.life_points, 0);
        assert_eq!(game.game_speed, INITIAL_GAME_SPEED);
        assert!(game.time_until_next_word.is_none());
        assert!(game.game_speed_before_slow_down.is_none());
        assert!(game.passed_time_since_game_end.is_none());
        assert!(game.slow_down_time_left.is_none());
        assert!(game.spawn_only_short_words_time_left.is_none());
    }

    #[test]
    fn it_starts_the_game_correctly() {
        let mut game = create_game();

        let start_res = game.start_game();
        assert!(start_res.is_ok());

        assert!(game.is_game_running);
        assert_eq!(game.current_score, 0);
        assert_eq!(game.life_points, 0);
        assert_eq!(
            game.time_until_next_word,
            Some(INITIAL_TIME_UNTIL_NEXT_WORD)
        );
        assert_eq!(game.game_speed, INITIAL_GAME_SPEED);
        assert_eq!(game.passed_time_since_game_end, None);
    }

    #[test]
    fn it_ends_the_game_correctly() {
        let mut game = create_game();

        game.is_game_running = true;

        let word = Word::new("word", Point2 { x: 0.0, y: 0.0 }, 0);
        game.words = VecDeque::from([word]);

        let end_res = game.end_game();

        assert!(end_res.is_ok());

        assert_eq!(game.is_game_running, false);
        assert_eq!(game.words.len(), 0);
        assert_eq!(game.passed_time_since_game_end, Some(0.0))
    }

    #[test]
    fn it_updates_words_positions_correctly() {
        let mut game = create_game();

        let word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.update_words_positions(0.2);

        assert_eq!(game.words.front().unwrap().position.y, 1.2);
        assert_eq!(game.words.back().unwrap().position.y, 2.2);
    }

    #[test]
    fn it_spawns_a_new_word_correctly() {
        let mut game = create_game();

        let word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.spawn_new_word(None);
        assert_eq!(game.words.len(), 3);
        assert!(game.words.back().unwrap().position.x < 1000.0)
    }

    #[test]
    fn it_spawns_a_new_word_with_len_limit_correctly() {
        let mut game = create_game();

        let word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.spawn_new_word(Some(3));
        assert_eq!(game.words.len(), 3);
        assert!(game.words.back().unwrap().position.x < 1000.0);
        assert!(game.words.back().unwrap().value.graphemes(true).count() <= 3);
    }

    #[test]
    fn it_completes_a_word_without_effect_correctly() {
        let mut game = create_game();

        game.current_score = 10;
        game.game_speed = INITIAL_GAME_SPEED;
        let word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.complete_word();

        assert_eq!(game.words.len(), 1);
        assert_eq!(game.current_score, 20);
        assert_eq!(game.game_speed, INITIAL_GAME_SPEED + 10);
    }

    #[test]
    fn it_completes_a_word_with_add_life_effect_correctly() {
        let mut game = create_game();

        game.current_score = 10;
        game.game_speed = INITIAL_GAME_SPEED;
        game.life_points = 0;
        let mut word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        word_1.effect = Some(WordEffect::AddLife);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.complete_word();

        assert_eq!(game.words.len(), 1);
        assert_eq!(game.current_score, 20);
        assert_eq!(game.game_speed, INITIAL_GAME_SPEED + 10);
        assert_eq!(game.life_points, 1);
    }

    #[test]
    fn it_completes_a_word_with_slow_down_effect_correctly() {
        let mut game = create_game();

        game.current_score = 10;
        game.game_speed = INITIAL_GAME_SPEED;
        game.life_points = 0;
        let mut word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        word_1.effect = Some(WordEffect::SlowDown);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.complete_word();

        assert_eq!(game.words.len(), 1);
        assert_eq!(game.current_score, 20);
        assert_eq!(game.game_speed, 25);
        assert_eq!(
            game.game_speed_before_slow_down,
            Some(INITIAL_GAME_SPEED + 10)
        );
        assert_eq!(game.slow_down_time_left, Some(5.0));
    }

    #[test]
    fn it_completes_a_word_with_short_words_spawn_effect_correctly() {
        let mut game = create_game();

        game.current_score = 10;
        game.game_speed = INITIAL_GAME_SPEED;
        game.life_points = 0;
        let mut word_1 = Word::new("word1", Point2 { x: 1.0, y: 1.0 }, 0);
        word_1.effect = Some(WordEffect::SpawnOnlyShortWords);
        let word_2 = Word::new("word2", Point2 { x: 0.3, y: 2.0 }, 0);
        game.words = VecDeque::from([word_1, word_2]);

        game.complete_word();

        assert_eq!(game.words.len(), 1);
        assert_eq!(game.current_score, 20);
        assert_eq!(game.game_speed, INITIAL_GAME_SPEED + 10);
        assert_eq!(game.spawn_only_short_words_time_left, Some(5.0));
    }
}
