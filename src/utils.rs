use clap::{ArgMatches};

use super::styles::TextStyle;


#[derive(Debug, Clone)]
pub struct ListItem {
  regular_output: String,
  highlighted_output: String,
  question: String,
  answer: String,
  prefix_style: TextStyle,
  question_style: TextStyle,
  delimiter_style: TextStyle,
  answer_style: TextStyle,
}

impl ListItem {
  pub fn new(
    s: &str,
    s2: &str,
    s3: &str,
    s4: &str,
    ps: TextStyle,
    qs: TextStyle,
    ds: TextStyle,
    ans: TextStyle,
  ) -> ListItem {
    ListItem {
      regular_output: s.to_string(),
      highlighted_output: s2.to_string(),
      question: s3.to_string(),
      answer: s4.to_string(),
      prefix_style: ps,
      question_style: qs,
      delimiter_style: ds,
      answer_style: ans,
    }
  }

  pub fn get_output(&self, is_highlighted: bool) -> &str {
    if is_highlighted {
      &self.highlighted_output
    } else {
      &self.regular_output
    }
  }
}

pub fn get_list_items_from_matches(
    matches: &ArgMatches,
    style_objs: &(
        TextStyle,
        TextStyle,
        TextStyle,
        TextStyle,
    ),
) -> Vec<ListItem> {

    let (
        prefix_style,
        question_style,
        delimiter_style,
        answer_style,
    ) = style_objs;

    let mut my_vec: Vec<ListItem> = vec![];
    for item in matches.args.iter() {
        let question = item.0;
        let default = item.1.vals[0].clone();
        let default_answer = &default.into_string().unwrap();

        let my_str = format!(
            "{}{}{}",
            question_style.get_console_string(question, false),
            delimiter_style.get_console_string(&delimiter_style.text, false),
            answer_style.get_console_string(default_answer, false),
        );

        let my_str_highlighted = format!(
            "{}{}{}{}",
            prefix_style.get_console_string(&prefix_style.text, true),
            question_style.get_console_string(question, true),
            delimiter_style.get_console_string(&delimiter_style.text, true),
            answer_style.get_console_string(default_answer, true),
        );

        let my_question = ListItem::new(
            &my_str,
            &my_str_highlighted,
            question,
            default_answer,
            prefix_style.clone(),
            question_style.clone(),
            delimiter_style.clone(),
            answer_style.clone(),
        );
        my_vec.push(my_question);
    }


    my_vec
}