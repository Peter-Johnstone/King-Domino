#[derive(Default, Copy, Clone)]
//#[allow(dead_code)]
pub(crate) struct GridDomino {
    x: u8,
    y: u8,
    domino_id: usize,
    rotation: f64, // In radians
}

impl GridDomino {
    pub(crate) fn new(x: u8, y:u8, id: usize, rotation: f64) -> Self {
        Self {
            x: x,
            y: y,
            domino_id: id,
            rotation: rotation,
        }
    }

    pub(crate) fn x(&self)->&u8{&self.x}
    pub(crate) fn y(&self)->&u8{&self.y}
    pub(crate) fn domino_id(&self)->&usize{&self.domino_id}
    pub(crate) fn rotation(&self)->&f64{&self.rotation}


}