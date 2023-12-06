use crate::index::SigCoord;

pub(crate) fn coord_to_screen(coord: impl Into<SigCoord>) -> (u32, u32) {
    const CENTER_X: i32 = 1216;
    const CENTER_Y: i32 = 504;
    const TILE_WIDTH: i32 = 66;
    const TILE_HEIGHT: i32 = 57;

    let coord = coord.into();
    let row = coord.row() as i32;
    let col = coord.col() as i32;
    (
        (CENTER_X + col * TILE_WIDTH - row * TILE_WIDTH / 2) as u32,
        (CENTER_Y + row * TILE_HEIGHT) as u32,
    )
}
