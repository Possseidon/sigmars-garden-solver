use std::{thread::sleep, time::Duration};

use mouse_rs::{types::keys::Keys, Mouse};

use crate::{index::SigCoord, screen::coord_to_screen};

pub(crate) fn click_next_game(mouse: &Mouse) {
    click_at(mouse, 870, 886);
}

pub(crate) fn click_at_coord(mouse: &Mouse, coord: impl Into<SigCoord>) {
    let (x, y) = coord_to_screen(coord);
    click_at(mouse, x, y);
}

pub(crate) fn click_at(mouse: &Mouse, x: u32, y: u32) {
    mouse.move_to(x as i32, y as i32).unwrap();
    sleep(Duration::from_millis(42));
    mouse.press(&Keys::LEFT).unwrap();
    sleep(Duration::from_millis(42));
    mouse.release(&Keys::LEFT).unwrap();
}
