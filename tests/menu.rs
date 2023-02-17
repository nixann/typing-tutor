#[cfg(test)]
mod tests {
    use typing_tutor::menu::{Menu, MenuMove};

    #[test]
    fn it_returns_the_selected_option() {
        let menu = Menu {
            options: Vec::from(["opt1".to_string(), "opt2".to_string()]),
            selected_option_index: 0,
        };

        assert_eq!(*menu.get_selected_option(), "opt1".to_string());
    }

    #[test]
    fn it_navigates_correctly() {
        let mut menu = Menu {
            options: Vec::from(["opt1".to_string(), "opt2".to_string()]),
            selected_option_index: 0,
        };

        menu.navigate(MenuMove::Up);
        assert_eq!(*menu.get_selected_option(), "opt1".to_string());

        menu.navigate(MenuMove::Down);
        assert_eq!(*menu.get_selected_option(), "opt2".to_string());

        menu.navigate(MenuMove::Down);
        assert_eq!(*menu.get_selected_option(), "opt2".to_string());

        menu.navigate(MenuMove::Up);
        assert_eq!(*menu.get_selected_option(), "opt1".to_string());
    }
}
