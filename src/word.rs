use ggez::{graphics::Color, mint::Point2};
use rand::{seq::SliceRandom, Rng};
use unicode_segmentation::UnicodeSegmentation;

pub struct Word {
    pub value: String,
    pub position: Point2<f32>,
    pub progress_index: usize,
    pub effect: Option<WordEffect>,
}
#[derive(Copy, Clone)]
pub enum WordEffect {
    SlowDown,
    AddLife,
    SpawnOnlyShortWords,
}

impl Word {
    pub fn new(value: &str, position: Point2<f32>, progress_index: usize) -> Self {
        let mut rng = rand::thread_rng();
        let rand_number = rng.gen_range(1..=100);

        let word_effect;
        if rand_number > 80 {
            word_effect = Some(
                *vec![
                    WordEffect::SlowDown,
                    WordEffect::AddLife,
                    WordEffect::SpawnOnlyShortWords,
                ]
                .choose(&mut rand::thread_rng())
                .unwrap(),
            )
        } else {
            word_effect = None
        }
        Self {
            value: String::from(value),
            position,
            progress_index,
            effect: word_effect,
        }
    }

    pub fn get_display_value(&self) -> String {
        self.value
            .graphemes(true)
            .skip(self.progress_index)
            .collect()
    }

    pub fn update_position(&mut self, delta_y: f32) {
        self.position.y += delta_y
    }

    pub fn get_color(&self) -> Color {
        match self.effect {
            Some(WordEffect::SlowDown) => Color::new(0.06, 0.9, 0.92, 0.65),
            Some(WordEffect::AddLife) => Color::GREEN,
            Some(WordEffect::SpawnOnlyShortWords) => Color::YELLOW,
            None => Color::WHITE,
        }
    }

    pub fn get_font(&self) -> String {
        if let Some(_effect) = self.effect {
            String::from("SecondaryFont")
        } else {
            String::from("PrimaryFont")
        }
    }

    pub fn handle_typed_letter(&mut self, letter: char) {
        let current_letter = self.value.chars().nth(self.progress_index).unwrap();

        if current_letter == letter {
            self.progress_index += 1;
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.progress_index == self.value.graphemes(true).count();
    }
}
