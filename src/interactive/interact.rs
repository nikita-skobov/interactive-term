use crossterm::{
    TerminalCursor,
    input,
    InputEvent,
    KeyEvent,
    RawScreen,
    ClearType,
    Color,
    Crossterm,
};
use std::{thread, io, time::Duration};

use super::super::utils::ListItem;
use super::print_list;
use super::get_key_char;
use super::down_or_up;
use super::KeyCharPressed;


pub fn interact(out_vec: &mut Vec<ListItem>) -> io::Result<()> {
    // make sure to enable raw mode, this will make sure key events won't be handled by the terminal it's self and allows crossterm to read the input and pass it back to you.
    if let Ok(_raw) = RawScreen::into_raw_mode() {
        let input = input();

        let mut async_stdin = input.read_async();
        let mut cursor_position = 0;
        let mut cursor_offset = 0;
        let mut current_mode = 0; // scroll mode, 1 is editing

        let crossterm = Crossterm::new();
        let terminal = crossterm.terminal();
        let cursor = crossterm.cursor();
        let (term_x, term_y) = terminal.terminal_size();

        terminal.clear(ClearType::All)?;

        let min_cursor = 0;
        let max_cursor = term_y - (term_y / 10);
        let max_width = term_x;
        let out_vec_len = out_vec.len();

        cursor.hide()?;

        print_list(
            &cursor,
            max_cursor,
            max_width,
            out_vec,
            cursor_offset,
            cursor_position
        )?;
        cursor.goto(0, term_y - 2)?;
        println!("CTRL-g to quit. CTRL-w to edit");

        loop {
            let event = async_stdin.next();
            if let Some(key_event) = event {
                if current_mode == 1 {
                  let editing_item = &mut out_vec[(cursor_position as usize) + (cursor_offset as usize)];
                  match get_key_char(key_event) {
                    KeyCharPressed::Char(c) => {
                      editing_item.answer.push(c);
                    },
                    KeyCharPressed::Backspace => {
                      editing_item.answer.pop();
                    },
                    KeyCharPressed::Exit => {
                      current_mode = 0;
                      terminal.clear(ClearType::All)?;
                      print_list(
                            &cursor,
                            max_cursor,
                            max_width,
                            out_vec,
                            cursor_offset,
                            cursor_position
                      )?;
                      cursor.goto(0, term_y - 2)?;
                      println!("CTRL-g to quit. CTRL-w to edit");
                      continue
                    },
                    KeyCharPressed::Quit => {
                      cursor.show()?;
                      break;
                    },
                    KeyCharPressed::None => {
                      continue
                    },
                  }

                  let my_str = format!(
                    "{}{}{}",
                    editing_item.question_style.get_console_string(&editing_item.question, false),
                    editing_item.delimiter_style.get_console_string(&editing_item.delimiter_style.text, false),
                    editing_item.answer_style.get_console_string(&editing_item.answer, false),
                  );

                  let my_str_highlighted = format!(
                    "{}{}{}{}",
                    editing_item.prefix_style.get_console_string(&editing_item.prefix_style.text, true),
                    editing_item.question_style.get_console_string(&editing_item.question, true),
                    editing_item.delimiter_style.get_console_string(&editing_item.delimiter_style.text, true),
                    editing_item.answer_style.get_console_string(&editing_item.answer, true),
                  );
                  editing_item.regular_output = my_str;
                  editing_item.highlighted_output = my_str_highlighted;

                  terminal.clear(ClearType::All)?;

                  print_list(
                      &cursor,
                      max_cursor,
                      max_width,
                      out_vec,
                      cursor_offset,
                      cursor_position
                  )?;
                  cursor.goto(0, term_y - 2)?;
                  println!("CTRL-g to quit. CTRL-w to stop editing");
                  continue
                }

                let key_dir = down_or_up(key_event);
                if key_dir == 255 {
                    cursor.show()?;
                    break;
                } else if key_dir == 254 {
                  // switch to editing mode.
                  current_mode = 1;
                  terminal.clear(ClearType::All)?;
                  print_list(
                        &cursor,
                        max_cursor,
                        max_width,
                        out_vec,
                        cursor_offset,
                        cursor_position
                  )?;
                  cursor.goto(0, term_y - 2)?;
                  println!("CTRL-g to quit. CTRL-w to stop editing");

                  // out_vec = &mut out_vec.clone();
                  // cursor.goto(0, cursor_position as u16 + cursor_offset as u16);
                  // cursor.show();
                  continue
                } else if cursor_position + key_dir < max_cursor as i32 && cursor_position + key_dir >= min_cursor as i32 {
                    
                    if ((cursor_position + key_dir + cursor_offset as i32) as usize) < out_vec_len {
                        cursor_position += key_dir;
                    }

                    terminal.clear(ClearType::All)?;

                    let highlighted_position = match print_list(
                        &cursor,
                        max_cursor,
                        max_width,
                        out_vec,
                        cursor_offset,
                        cursor_position
                    ) {
                      Ok(p) => p,
                      Err(e) => panic!("ERROR PRINTING LIST: {:?}", e),
                    };

                    cursor.goto(0, term_y - 2)?;
                    println!("CTRL-g to quit. CTRL-w to edit");

                    if highlighted_position >= (max_cursor - 1) {
                        cursor_offset += 1;
                        cursor_position -= key_dir;
                    } else if cursor_position == (min_cursor + 1) as i32 && cursor_offset > 0 {
                        cursor_offset -= 1;
                        cursor_position -= key_dir;
                    }
                }
            }
            thread::sleep(Duration::from_millis(15));
        }
    } // <=== raw modes will be disabled here

    Ok(())
}
