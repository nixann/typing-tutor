#[cfg(test)]
mod tests {
    use ggez::{graphics::Color, mint::Point2};
    use typing_tutor::word::{Word, WordEffect};

    #[test]
    fn it_returns_the_correct_color_when_the_word_has_no_effect() {
        let mut word = Word::new("val", Point2 { x: 1.0, y: 1.0 }, 0);
        word.effect = None;

        assert_eq!(word.get_color(), Color::WHITE);
    }

    #[test]
    fn it_returns_the_correct_color_when_the_word_has_an_effect() {
        let mut word = Word::new("val", Point2 { x: 1.0, y: 1.0 }, 0);
        word.effect = Some(WordEffect::AddLife);

        assert_eq!(word.get_color(), Color::GREEN);

        word.effect = Some(WordEffect::SlowDown);

        assert_eq!(word.get_color(), Color::new(0.06, 0.9, 0.92, 0.65));

        word.effect = Some(WordEffect::SpawnOnlyShortWords);

        assert_eq!(word.get_color(), Color::YELLOW);
    }

    #[test]
    fn it_returns_the_correct_display_value_depending_on_the_progress() {
        let mut word = Word::new("val", Point2 { x: 1.0, y: 1.0 }, 0);

        assert_eq!(word.get_display_value(), "val");
        word.progress_index = 1;
        assert_eq!(word.get_display_value(), "al");
    }

    #[test]
    fn it_updates_the_display_value_and_progress_when_a_letter_is_typed() {
        let mut word = Word::new("climate", Point2 { x: 1.0, y: 1.0 }, 0);

        // does not update when the letter does not match the first letter of the display value
        word.handle_typed_letter('a');
        assert_eq!(word.get_display_value(), "climate");
        assert_eq!(word.progress_index, 0);

        // updates the display value
        word.handle_typed_letter('c');
        word.handle_typed_letter('l');
        assert_eq!(word.get_display_value(), "imate");
        assert_eq!(word.progress_index, 2);
    }

    #[test]
    fn it_completes_a_word_when_all_its_leters_are_typed() {
        let mut word = Word::new("climate", Point2 { x: 1.0, y: 1.0 }, 0);

        word.handle_typed_letter('c');
        word.handle_typed_letter('l');
        word.handle_typed_letter('i');
        word.handle_typed_letter('m');
        word.handle_typed_letter('a');
        word.handle_typed_letter('t');
        word.handle_typed_letter('e');
        assert!(word.is_completed());
    }

    #[test]
    fn it_updates_word_position() {
        let mut word = Word::new("val", Point2 { x: 1.0, y: 1.0 }, 0);
        word.update_position(1.0);
        assert_eq!(word.position.y, 2.0);
    }
}
