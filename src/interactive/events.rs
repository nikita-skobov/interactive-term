use crossterm::{InputEvent, KeyEvent};

pub enum KeyCharPressed {
  Backspace,
  Char(char),
  Exit,
  None,
  Quit,
}


pub fn get_key_char(key_event: InputEvent) -> KeyCharPressed {
  match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Char(c) => { return KeyCharPressed::Char(c) },
                KeyEvent::Backspace => { return KeyCharPressed::Backspace },
                KeyEvent::Ctrl(c) => match c {
                  'w' => {
                    return KeyCharPressed::Exit;
                  },
                  'g' => {
                    return KeyCharPressed::Quit;
                  }
                  _ => (),
                }
                _ => (),
            }
        }
        _ => (),
    }

    return KeyCharPressed::None;
}

pub fn down_or_up(key_event: InputEvent) -> i32 {
    match key_event {
        InputEvent::Keyboard(k) => {
            match k {
                KeyEvent::Ctrl(c) => match c {
                  'w' => {
                    return 254;
                  },
                  'g' => {
                    return 255;
                  },
                  _ => ()
                }
                KeyEvent::Up => {
                    return -1;
                }
                KeyEvent::Down => {
                    return 1;
                }
                _ => ()
            }
        }
        _ => ()
    }

    return 0;
}