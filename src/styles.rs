use yaml_rust::{Yaml};
use crossterm::{Color, style};

pub struct TextStyle {
    text: String,
    background: Option<Color>,
    color: Option<Color>,
    highlighted_color: Option<Color>,
    highlighted_background: Option<Color>,
}

impl TextStyle {
    pub fn get_console_string(
        &self,
        txt: &str,
        is_highlighted: bool,
    ) -> String {
        let use_background;
        let use_color;

        if is_highlighted {
            use_background = self.highlighted_background;
            use_color = self.highlighted_color;
        } else {
            use_background = self.background;
            use_color = self.color;
        }

        let mut out_str;

        if let (Some(bc), Some(tc)) = (use_background, use_color) {
            // both text and background color provided
            out_str = style(txt)
                .on(bc)
                .with(tc);
        } else if let Some(bc) = use_background {
            // only background color provided
            out_str = style(txt)
                .on(bc);
        } else if let Some(tc) = use_color {
            // only text color provided
            out_str = style(txt)
                .with(tc);
        } else {
            // none exist
            return String::from(txt)
        }


        format!(
            "{}{}",
            out_str,
            style("")
                .on(Color::Reset)
                .with(Color::Reset)
        )
    }
}

// pub fn get_styles_from_yaml(yaml_obj: &Yaml) -> (

// ) {

// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_console_string_works() {
        let test_string = "12345";
        let mut my_text_style = TextStyle {
            text: String::from("doesnt matter"),
            color: None,
            background: None,
            highlighted_background: None,
            highlighted_color: None,
        };

        // if there are no highlighted colors, it should
        // return string as is.
        assert_eq!(
            my_text_style.get_console_string(test_string, true),
            String::from(test_string)
        );

        // if there are no regular colors, it should return
        // string as is.
        assert_eq!(
            my_text_style.get_console_string(test_string, false),
            String::from(test_string)
        );

        // if there are colors, it should use them.
        my_text_style.color = Some(Color::Red);
        my_text_style.background = Some(Color::Yellow);
        assert_ne!(
            my_text_style.get_console_string(test_string, false),
            String::from(test_string)
        );
        // same for highlighted settings
        my_text_style.highlighted_background = Some(Color::Red);
        my_text_style.highlighted_color = Some(Color::Black);
        assert_ne!(
            my_text_style.get_console_string(test_string, true),
            String::from(test_string)
        );

        // if it uses colors, it should end with a color reset.
        let color_reset = format!("{}", style("")
            .on(Color::Reset)
            .with(Color::Reset));
        let my_console_str = my_text_style.get_console_string(test_string, false);
        let my_console_str_highlighted = my_text_style.get_console_string(test_string, true);
        assert_eq!(
            my_console_str_highlighted.contains(color_reset.as_str()),
            true,
        );
        assert_eq!(
            my_console_str.contains(color_reset.as_str()),
            true,
        );
    }
}
