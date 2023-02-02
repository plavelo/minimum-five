#[derive(Default)]
pub struct IntegerRegister {
    x: [u64; 32],
}

impl IntegerRegister {
    pub fn read(&self, register: usize) -> u64 {
        self.x[register]
    }

    pub fn write(&mut self, register: usize, value: u64) {
        if register != 0 {
            self.x[register] = value;
        }
    }
}
