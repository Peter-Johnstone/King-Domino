#[derive(Default, Copy, Clone)]
//#[allow(dead_code)]
pub(crate) struct GridDomino {
    x: u8,
    y: u8,
    domino_id: usize,
    rotation: f32, // In radians
}

impl GridDomino {
    pub(crate) fn new(x: u8, y:u8, id: usize, rotation: f32) -> Self {
        Self {
            x: x,
            y: y,
            domino_id: id,
            rotation: rotation,
        }
    }
}