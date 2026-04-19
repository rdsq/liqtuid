use crate::object::Liqtuid;
use std::cmp::min;

const FULL_MIDDLE: &str = "███";
const FULL_BOTTOM: &str = "◥█◤";
const EMPT_MIDDLE: &str = "⎸ ⎹";
const EMPT_BOTTOM: &str =r"\⎽/";

pub fn render_game(game: &Liqtuid, columns: usize) -> String {
    let mut result = String::new();
    let bottles_num = game.bottles.len();
    for row_index in 0..(bottles_num as f32 / columns as f32).ceil() as usize {
        // this splits all the bottles into rows
        let min_index = row_index * columns;
        let range = min_index..min_index + min(columns, bottles_num - min_index);
        // show the labels above the bottles
        for bottle_index in range.clone() {
            if bottle_index != min_index {
                // add indent except for first in the row
                result.push_str("   ");
            }
            let key = *game.keys.as_bytes().get(bottle_index).unwrap_or(&45 /* ASCII for `-` */) as char;
            if game.selected.as_ref() == Some(&bottle_index) {
                // for selected bottles
                result.push_str("\x1b[36;1m[");
            } else {
                result.push_str("\x1b[2m[");
            }
            result.push(key);
            result.push_str("]\x1b[0m");
        }
        result.push('\n');
        // drawing the bottles
        for depth_i in 0..game.depth as usize {
            // this goes down all the bottles in this row
            // also this code sucks
            let is_last_layer = game.depth - 1 == depth_i;
            for bottle_index in range.clone() {
                // this goes through bottles in the row
                if bottle_index != min_index {
                    result.push_str("   ");
                }
                let bottle = &game.bottles[bottle_index];
                if let Some(element) = bottle.get(game.depth - depth_i - 1) {
                    // then this cell of the bottle is not empty
                    let col = game.colours[*element];
                    result.push_str(&format!("\x1b[38;5;{}m{}\x1b[0m", col, if is_last_layer { FULL_BOTTOM } else { FULL_MIDDLE }));
                } else {
                    // then it is empty
                    result.push_str(if is_last_layer { EMPT_BOTTOM } else { EMPT_MIDDLE });
                }
            }
            result.push('\n');
        }
        // bottom row with the cursor
        for bottle_index in range.clone() {
            if bottle_index != min_index { result.push_str("   ") };
            if game.cursor.as_ref() == Some(&bottle_index) {
                result.push_str(" \x1b[36;1m^\x1b[0m ");
            } else {
                result.push_str("   ");
            }
        }
        result.push('\n');
    }
    result
}
