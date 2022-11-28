#[derive(Clone, Copy)]
pub struct Key {
    pub d: KeyState,
    pub q: KeyState,
    pub z: KeyState,
}

impl Key {
    pub fn new() -> Self {
        Self {
            d: KeyState { pressed: false },
            q: KeyState { pressed: false },
            z: KeyState { pressed: false },
        }
    }
}

#[derive(Clone, Copy)]
pub struct KeyState {
    pub pressed: bool,
}
