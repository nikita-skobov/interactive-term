use crossterm::TerminalCursor;
use std::io;

use super::super::utils::ListItem;

pub fn print_list(
    cursor: &TerminalCursor,
    max_cursor: u16,
    max_width: u16,
    out_vec: &Vec<ListItem>,
    cursor_offset: usize,
    cursor_position: i32,
) -> io::Result<u16> {
    let mut internal_offset = 0;
    let mut num = 0;
    let mut highlighted_position = 0;

    while num + internal_offset < max_cursor {
        cursor.goto(0, internal_offset + num as u16)?;

        // let mut item: Option<&ListItem> = None;
        let num_is_cursor_position = num as i32 == cursor_position;
        let mut out_str_len = 0;
        let mut num_newlines = 0;

        if ((num as usize) + cursor_offset) < out_vec.len() {
            // string exists at this cursor, print
            let item = &out_vec[(num as usize) + cursor_offset];
            let out_str = item.get_output(num_is_cursor_position);
            out_str_len = out_str.len();
            num_newlines = out_str.matches("\n").count();
            println!("{}", out_str);
        }

        if out_str_len > max_width as usize {
          internal_offset += out_str_len as u16 / max_width;
        }
        internal_offset += num_newlines as u16;

        if num_is_cursor_position {
          highlighted_position = num + internal_offset;
        }

        num += 1;
    }
    // if cursor_position == 11 {
    //   panic!("POSITION: {}, real_pos: {}, max_cursor: {}, internal_offset: {}, offset: {}, vec_len: {}", cursor_position, highlighted_position, max_cursor, internal_offset, cursor_offset, out_vec.len());
    // }

    Ok(highlighted_position)
}
