use std::sync::atomic::{AtomicBool, Ordering};

pub struct GameCell {
    alive: bool,
    will_live: AtomicBool,
}

impl GameCell {
    pub fn new(alive: bool) -> Self {
        Self {
            alive,
            will_live: AtomicBool::new(false),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn update(&mut self) {
        self.alive = *self.will_live.get_mut()
    }

    pub fn set_will_live(&self, b: bool) {
        self.will_live.store(b, Ordering::Relaxed)
    }
}
