use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) struct GameCell {
    alive: bool,
    will_live: AtomicBool,
}

impl GameCell {
    pub(crate) fn new(alive: bool) -> Self {
        Self {
            alive,
            will_live: AtomicBool::new(false),
        }
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.alive
    }

    pub(crate) fn state(&self) -> u8 {
        self.alive as u8
    }

    pub(crate) fn symbol(&self) -> char {
        if self.alive {
            '█'
        } else {
            ' '
        }
    }

    pub(crate) fn update(&mut self) {
        self.alive = *self.will_live.get_mut()
    }

    pub(crate) fn set_will_live(&self, b: bool) {
        self.will_live.store(b, Ordering::Relaxed)
    }
}
