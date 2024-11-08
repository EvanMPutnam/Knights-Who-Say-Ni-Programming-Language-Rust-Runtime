use log::warn;

pub struct Storage {
    memory: Vec<u8>,
    total_bits: usize,
    data_pointer: usize,
    debug: bool,
}

impl Storage {
    pub fn new(size: usize, debug: bool) -> Storage {
        Storage {
            memory: (0..size).map(|_| 0).collect(),
            total_bits: size,
            data_pointer: 0,
            debug,
        }
    }

    pub fn output_byte_at_pointer(&self) {
        let current_value = self.memory.get(self.data_pointer);
        if current_value.is_none() {
            self.write_debug(
                "An unexpected error occurred in outputting current index pointer byte",
            );
        }
        let value_reference = current_value.unwrap();
        print!("{}", *value_reference as char);
    }

    pub fn increment_pointer(&mut self) {
        if self.data_pointer + 1 == self.total_bits {
            self.write_debug("Tried to increment data pointer beyond memory bounds, skipping");
            return;
        }
        self.data_pointer += 1;
    }

    pub fn decrement_pointer(&mut self) {
        if self.data_pointer == 0 {
            self.write_debug("Tried to decrement pointer from empty memory, skipping");
            return;
        }
        self.data_pointer -= 1;
    }

    pub fn set(&mut self, byte_index: usize, value: u8) {
        let current_value = self.memory.get(byte_index);
        if current_value.is_none() {
            self.write_debug("Invalid set operation, skipping");
            return;
        }
        self.memory[byte_index] = value;
    }

    pub fn get(&mut self, byte_index: usize) -> u8 {
        let current_value = self.memory.get(byte_index);
        if current_value.is_none() {
            self.write_debug("Invalid get operation, exceeds byte");
            return 0;
        }
        *current_value.unwrap()
    }

    pub fn get_current_data_pointer(&self) -> usize {
        self.data_pointer
    }

    fn write_debug(&self, val: &str) {
        if self.debug {
            warn!("{}", val);
        }
    }
}
