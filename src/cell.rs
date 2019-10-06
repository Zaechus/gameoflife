use std::cell::Cell;

pub struct GameCell {
    alive: bool,
    will_live: Cell<bool>,
}

impl GameCell {
    pub fn new(alive: bool) -> Self {
        Self {
            alive,
            will_live: Cell::new(false),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn update(&mut self) {
        self.alive = self.will_live.get()
    }

    pub fn set_will_live(&self, b: bool) {
        self.will_live.set(b);
    }
}
