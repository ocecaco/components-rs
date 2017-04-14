use std::sync::Mutex;

pub struct RefCount {
    counter: Mutex<u32>,
}

impl RefCount {
    pub fn new(initial: u32) -> Self {
        RefCount { counter: Mutex::new(initial) }
    }

    pub fn up(&self) -> u32 {
        let mut guard = self.counter.lock().unwrap();
        let field: &mut u32 = &mut guard;
        let new_value = *field + 1;
        *field = new_value;
        new_value
    }

    pub fn down(&self) -> u32 {
        let mut guard = self.counter.lock().unwrap();
        let field: &mut u32 = &mut guard;
        let new_value = *field - 1;
        *field = new_value;
        new_value
    }
}
