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

use crate::word::{Word, WordEffect};

const WORD_SCORE: u32 = 10;

const INITIAL_TIME_UNTIL_NEXT_WORD: f32 = 1.0;
const INITIAL_GAME_SPEED: u32 = 50;

pub struct Game {
    screen_height: f32,
    screen_width: f32,
    key_codes_map: HashMap<keyboard::KeyCode, char>,
    is_game_running: bool,
    source_words: Vec<String>,
    words: VecDeque<Word>,
    time_until_next_word: Option<f32>,
    next_word_loop_length: f32,
    current_score: u32,
    life_points: u32,
    game_speed: u32,
    game_speed_before_slow_down: u32,
    passed_time_since_game_end: Option<f32>,
    slow_down_time_left: Option<f32>,
    spawn_only_short_words_time_left: Option<f32>,
}

impl Game {
    pub fn new(conf: &Conf) -> Game {
        // Load/create resources such as images here.
        Game {
            is_game_running: false,
            key_codes_map: HashMap::from([
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
            ]),
            source_words: vec![
                "the", "of", "and", "a", "to", "in", "is", "you", "that", "it", "he", "was", "for",
                "on", "are", "with", "as", "his", "they", "be", "at", "one", "have", "this",
                "from", "or", "had", "by", "hot", "word", "but", "what", "some", "we", "can",
                "out", "other", "were", "all", "there", "when", "up", "use", "your", "how", "said",
                "an", "each", "she", "which", "do", "their", "time", "if", "will", "way", "about",
                "many", "then", "them", "write", "would", "like", "so", "these", "her", "long",
                "make", "thing", "see", "him", "two", "has", "look", "more", "day", "could", "go",
                "come", "did", "number", "sound", "no", "most", "people", "my", "over", "know",
                "water", "than", "call", "first", "who", "may", "down", "side", "been", "now",
                "find", "any", "new", "work", "part", "take", "get", "place", "made", "live",
                "where", "after", "back", "little", "only", "round", "man", "year", "came", "show",
                "every", "good", "me", "give", "our", "under", "name", "very", "through", "just",
                "form", "sentence", "great", "think", "say", "help", "low", "line", "differ",
                "turn", "cause", "much", "mean", "before", "move", "right", "boy", "old", "too",
                "same", "tell", "does", "set", "three", "want", "air", "well", "also", "play",
                "small", "end", "put", "home", "read", "hand", "port", "large", "spell", "add",
                "even", "land", "here", "must", "big", "high", "such", "follow", "act", "why",
                "ask", "men", "change", "went", "light", "kind", "off", "need", "house", "picture",
                "try", "us", "again", "animal", "point", "mother", "world", "near", "build",
                "self", "earth", "father", "head", "stand", "own", "page", "should", "country",
                "found", "answer", "school", "grow", "study", "still", "learn", "plant", "cover",
                "food", "sun", "four", "between", "state", "keep", "eye", "never", "last", "let",
                "thought", "city",
            ]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
            time_until_next_word: None,
            next_word_loop_length: INITIAL_TIME_UNTIL_NEXT_WORD,
            words: VecDeque::new(),
            screen_height: conf.window_mode.height,
            screen_width: conf.window_mode.width,
            current_score: 0,
            life_points: 0,
            game_speed: INITIAL_GAME_SPEED,
            game_speed_before_slow_down: INITIAL_GAME_SPEED,
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
                self.game_speed = self.game_speed_before_slow_down;
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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
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
                    if self.next_word_loop_length > 0.01 {
                        self.next_word_loop_length -= 0.01;
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
        self.draw_text(
            &mut text,
            graphics::PxScale::from(50.0),
            Point2 {
                x: 30.0,
                y: self.screen_height - 100.0,
            },
            Color::WHITE,
            canvas,
        );

        let mut text = graphics::Text::new(format!("LIFE POINTS: {}", self.life_points));
        self.draw_text(
            &mut text,
            graphics::PxScale::from(50.0),
            Point2 {
                x: 30.0,
                y: self.screen_height - 150.0,
            },
            Color::WHITE,
            canvas,
        );
    }

    fn draw_words(&self, canvas: &mut Canvas) {
        for word in &self.words {
            let mut text = graphics::Text::new(word.get_display_value());
            self.draw_text(
                &mut text,
                graphics::PxScale::from(40.0),
                word.position,
                word.get_color(),
                canvas,
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

    fn draw_text(
        &self,
        text: &mut graphics::Text,
        scale: graphics::PxScale,
        position: Point2<f32>,
        color: Color,
        canvas: &mut Canvas,
    ) {
        text.set_font("PrimaryFont");

        text.set_scale(scale);
        canvas.draw(
            text,
            graphics::DrawParam::default().color(color).dest(position),
        )
    }

    fn start_game(&mut self) -> GameResult {
        self.is_game_running = true;
        self.current_score = 0;
        self.life_points = 0;
        self.time_until_next_word = Some(INITIAL_TIME_UNTIL_NEXT_WORD);

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
        let source_words: Vec<String>;
        if let Some(limit) = length_limit {
            source_words = self
                .source_words
                .iter()
                .filter(|&word| word.graphemes(true).count() <= limit)
                .map(|w| w.to_string())
                .collect();
        } else {
            source_words = self.source_words.clone();
        }
        let word = source_words.choose(&mut rand::thread_rng()).unwrap();
        let word_position = Point2 {
            x: rand::thread_rng().gen_range(0.0..self.screen_width - 100.0),
            y: 0.0,
        };
        self.words.push_back(Word::new(word, word_position, 0));
    }

    fn complete_word(&mut self) {
        let word = self.words.pop_front().unwrap();
        if let Some(effect) = word.effect {
            self.handle_word_effect(effect)
        }
        self.current_score += WORD_SCORE;
        self.game_speed += 20;
    }

    fn handle_word_effect(&mut self, effect: WordEffect) {
        match effect {
            WordEffect::AddLife => self.life_points += 1,
            WordEffect::SlowDown => {
                self.slow_down_time_left = Some(5.0);
                self.game_speed_before_slow_down = self.game_speed;
                self.game_speed = 25;
            }
            WordEffect::SpawnOnlyShortWords => self.spawn_only_short_words_time_left = Some(5.0),
        }
    }
}
