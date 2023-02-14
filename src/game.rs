use std::collections::VecDeque;

use ggez::event::{EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::seq::SliceRandom;

use crate::word::Word;

const DESIRED_FPS: u32 = 60;

pub struct Game {
    is_game_over: bool, // Your state here...
    source_words: Vec<String>,
    words: VecDeque<Word>,
    time_until_next_word: f32,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load/create resources such as images here.
        Game {
            is_game_over: false,
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
            time_until_next_word: 1.0,
            words: VecDeque::new(), // ...
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_game_over {
            return Ok(());
        }

        while ctx.time.check_update_time(DESIRED_FPS) {
            let last_frame_length = ctx.time.delta().as_secs_f32();

            for mut word in &mut self.words {
                word.position = Point2 {
                    x: word.position.x,
                    y: word.position.y + 1.0,
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
                self.time_until_next_word = 1.0;
            }
        }

        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        for word in &self.words {
            let text = graphics::Text::new(word.get_display_value());
            canvas.draw(&text, graphics::DrawParam::default().dest(word.position));
        }
        // Draw code here...
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(current_word) = self.words.front_mut() {
            match input.keycode {
                Some(keyboard::KeyCode::A) => {
                    current_word.handle_typed_letter('a');
                }
                Some(keyboard::KeyCode::B) => {
                    current_word.handle_typed_letter('b');
                }
                Some(keyboard::KeyCode::C) => {
                    current_word.handle_typed_letter('c');
                }
                Some(keyboard::KeyCode::D) => {
                    current_word.handle_typed_letter('d');
                }
                Some(keyboard::KeyCode::E) => {
                    current_word.handle_typed_letter('e');
                }
                Some(keyboard::KeyCode::F) => {
                    current_word.handle_typed_letter('f');
                }
                Some(keyboard::KeyCode::G) => {
                    current_word.handle_typed_letter('g');
                }
                Some(keyboard::KeyCode::H) => {
                    current_word.handle_typed_letter('h');
                }
                Some(keyboard::KeyCode::I) => {
                    current_word.handle_typed_letter('i');
                }
                Some(keyboard::KeyCode::J) => {
                    current_word.handle_typed_letter('j');
                }
                Some(keyboard::KeyCode::K) => {
                    current_word.handle_typed_letter('k');
                }
                Some(keyboard::KeyCode::L) => {
                    current_word.handle_typed_letter('l');
                }
                Some(keyboard::KeyCode::M) => {
                    current_word.handle_typed_letter('m');
                }
                Some(keyboard::KeyCode::N) => {
                    current_word.handle_typed_letter('n');
                }
                Some(keyboard::KeyCode::O) => {
                    current_word.handle_typed_letter('o');
                }
                Some(keyboard::KeyCode::P) => {
                    current_word.handle_typed_letter('p');
                }
                Some(keyboard::KeyCode::Q) => {
                    current_word.handle_typed_letter('q');
                }
                Some(keyboard::KeyCode::R) => {
                    current_word.handle_typed_letter('r');
                }
                Some(keyboard::KeyCode::S) => {
                    current_word.handle_typed_letter('s');
                }
                Some(keyboard::KeyCode::T) => {
                    current_word.handle_typed_letter('t');
                }
                Some(keyboard::KeyCode::U) => {
                    current_word.handle_typed_letter('u');
                }
                Some(keyboard::KeyCode::V) => {
                    current_word.handle_typed_letter('v');
                }
                Some(keyboard::KeyCode::W) => {
                    current_word.handle_typed_letter('w');
                }
                Some(keyboard::KeyCode::X) => {
                    current_word.handle_typed_letter('x');
                }
                Some(keyboard::KeyCode::Y) => {
                    current_word.handle_typed_letter('y');
                }
                Some(keyboard::KeyCode::Z) => {
                    current_word.handle_typed_letter('z');
                }
                _ => (), // handle all other cases
            }

            if current_word.is_completed() {
                self.words.pop_front();
            }
        }
        return Ok(());
    }
}
