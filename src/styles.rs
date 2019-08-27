use yaml_rust::{Yaml};
use crossterm::{Color, style};

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub text: String,
    pub background: Option<Color>,
    pub color: Option<Color>,
    pub highlighted_color: Option<Color>,
    pub highlighted_background: Option<Color>,
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

pub fn get_color_from_word(word: &str) -> Color {
  match word {
    "black" => Color::Black,
    "red" => Color::Red,
    "dark_red" => Color::DarkRed,
    "green" => Color::Green,
    "dark_green" => Color::DarkGreen,
    "yellow" => Color::Yellow,
    "dark_yellow" => Color::DarkYellow,
    "blue" => Color::Blue,
    "dark_blue" => Color::DarkBlue,
    "magenta" => Color::Magenta,
    "dark_magenta" => Color::DarkMagenta,
    "cyan" => Color::Cyan,
    "dark_cyan" => Color::DarkCyan,
    "gray" => Color::Grey,
    "grey" => Color::Grey,
    "white" => Color::White,
    "dark_white" => Color::Black,
    "reset" => Color::Reset,
    _ => Color::Reset,
  }
}

pub fn parse_yaml_style(yaml_obj: &Yaml) -> TextStyle {
  let mut text = "".to_string();
  let mut color = Some(Color::Reset);
  let mut background = Some(Color::Reset);
  let mut highlighted_background = Some(Color::Reset);
  let mut highlighted_color = Some(Color::Reset);

  if let Some(t) = yaml_obj["text"].as_str() {
    text = String::from(t);
  }
  if let Some(c) = yaml_obj["color"].as_str() {
    color = Some(get_color_from_word(c));
  }
  if let Some(bc) = yaml_obj["background"].as_str() {
    background = Some(get_color_from_word(bc));
  }
  if let Some(hb) = yaml_obj["highlighted_background"].as_str() {
    highlighted_background = Some(get_color_from_word(hb));
  }
  if let Some(hc) = yaml_obj["highlighted_color"].as_str() {
    highlighted_color = Some(get_color_from_word(hc));
  }

  TextStyle {
    text,
    color,
    background,
    highlighted_background,
    highlighted_color,
  }
}

pub fn get_styles_from_yaml(yaml_obj: &Yaml) -> (
    TextStyle,
    TextStyle,
    TextStyle,
    TextStyle,
) {
  let mut prefix_style = TextStyle {
    text: "".to_string(),
    color: None,
    background: None,
    highlighted_background: None,
    highlighted_color: None,
  };

  let mut question_style = TextStyle {
    text: "".to_string(),
    color: None,
    background: None,
    highlighted_background: None,
    highlighted_color: None,
  };

  let mut delimiter_style = TextStyle {
    text: ": ".to_string(),
    color: None,
    background: None,
    highlighted_background: None,
    highlighted_color: None,
  };

  let mut answer_style = TextStyle {
    text: "".to_string(),
    color: None,
    background: None,
    highlighted_background: None,
    highlighted_color: None,
  };

  match yaml_obj["interactive_style"].as_hash() {
    Some(v) => {
      for (key, value) in v {
        if let Some(key_str) = key.as_str() {
          if key_str == "prefix" {
            prefix_style = parse_yaml_style(value);
          } else if key_str == "delimiter" {
            delimiter_style = parse_yaml_style(value);
          } else if key_str == "question" {
            question_style = parse_yaml_style(value);
          } else if key_str == "answer" {
            answer_style = parse_yaml_style(value);
          }
        }
      }
    },
    None => (),
  };

  (
    prefix_style,
    question_style,
    delimiter_style,
    answer_style,
  )
}

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
