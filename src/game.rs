use std::collections::{HashMap, VecDeque};

use ggez::conf::Conf;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::seq::SliceRandom;

use crate::word::Word;

const DESIRED_FPS: u32 = 60;

const WORD_SCORE: u32 = 10;

const INITIAL_TIME_UNTIL_NEXT_WORD: f32 = 1.0;
const INITIAL_GAME_SPEED: u32 = 50;

pub struct Game {
    screen_height: f32,
    key_codes_map: HashMap<keyboard::KeyCode, char>,
    is_game_running: bool,
    source_words: Vec<String>,
    words: VecDeque<Word>,
    time_until_next_word: f32,
    next_word_loop_length: f32,
    current_score: u32,
    game_speed: u32,
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
                "on", "are", "with", "as", "I", "his", "they", "be", "at", "one", "have", "this",
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
            time_until_next_word: INITIAL_TIME_UNTIL_NEXT_WORD,
            next_word_loop_length: INITIAL_TIME_UNTIL_NEXT_WORD,
            words: VecDeque::new(),
            screen_height: conf.window_mode.height,
            current_score: 0,
            game_speed: INITIAL_GAME_SPEED
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(first_word) = self.words.front() {
            if first_word.position.y >= self.screen_height {
                self.is_game_running = false;
                self.words.clear();
                self.current_score = 0;
                return Ok(());
            }
        }

        if !self.is_game_running {
            return Ok(());
        }

        while ctx.time.check_update_time(DESIRED_FPS) {
            let last_frame_length = ctx.time.delta().as_secs_f32();
            for mut word in &mut self.words {
                word.position = Point2 {
                    x: word.position.x,
                    y: word.position.y + (self.game_speed as f32 * last_frame_length),
                }
            }

            self.time_until_next_word -= last_frame_length;
            if self.time_until_next_word <= 0.0 {
                let word = self.source_words.choose(&mut rand::thread_rng()).unwrap();
                let top_left = Point2 { x: 600.0, y: 0.0 };
                self.words.push_back(Word {
                    value: word.clone(),
                    position: top_left,
                    progress_index: 0,
                });
                self.time_until_next_word = self.next_word_loop_length;
            }
        }

        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        if !self.is_game_running {
            let mut text = graphics::Text::new("PRESS SPACE TO START");
            text.set_scale(graphics::PxScale::from(50.0));

            // TODO: do not use static values for the position
            canvas.draw(
                &text,
                graphics::DrawParam::default().dest(Point2 { x: 350.0, y: 200.0 }),
            );
        }
        else {
            let mut current_score_text = graphics::Text::new(format!("SCORE: {}", self.current_score));
            current_score_text.set_scale(graphics::PxScale::from(50.0));
            canvas.draw(&current_score_text, graphics::DrawParam::default().dest(Point2 { x: 30.0, y: self.screen_height - 100.0 }));
            for word in &self.words {
                let mut text = graphics::Text::new(word.get_display_value());
                text.set_scale(graphics::PxScale::from(40.0));
                // text.set_layout(layout)

                canvas.draw(&text, graphics::DrawParam::default().dest(word.position));
            }
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
                    self.words.pop_front();
                    self.current_score += WORD_SCORE;
                    self.game_speed += 20;
                    if self.next_word_loop_length > 0.01 {
                        self.next_word_loop_length -= 0.01;
                    }
                }
            }

            if !self.is_game_running && input_key_code == keyboard::KeyCode::Space {
                self.is_game_running = true;
            }
        }
        return Ok(());
    }
}
