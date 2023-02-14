use ggez::mint::Point2;
use unicode_segmentation::UnicodeSegmentation;


pub struct Word {
    pub value: String,
    pub position: Point2<f32>,
    pub progress_index: usize,
}

impl Word {
    pub fn new(value: &str, position: Point2<f32>, progress_index: usize) -> Self {
        Self { value: String::from(value), position, progress_index }
    }

    pub fn get_display_value(&self) -> String {
        self.value.graphemes(true).skip(self.progress_index).collect()
        // self.value.chars().take(self.value.len() - self.progress_index).collect()
    }

    pub fn handle_typed_letter(&mut self, letter: char) {
        let current_letter = self.value.chars().nth(self.progress_index).unwrap();

        if current_letter == letter {
            self.progress_index += 1;
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.progress_index == self.value.graphemes(true).count() - 1
    }
}
