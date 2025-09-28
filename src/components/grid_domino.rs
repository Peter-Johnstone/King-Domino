#[derive(Default, Copy, Clone)]
pub(crate) struct Grid_domino {
    x: u8,
    y: u8,
    domino_id: usize,
    rotation: f32, // In radians
}

impl Grid_domino {
    pub(crate) fn new(x: u8, y:u8, id: usize, rotation: f32) -> Self {
        Self {
            x: x,
            y: y,
            domino_id: id,
            rotation: rotation,
        }
    }
}