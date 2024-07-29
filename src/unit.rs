use crate::MIXComputer;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {
    id: u32,
}

impl Unit {

    pub fn new(id: u32) -> Self {
        Unit { id }
    }

    pub fn get_block_size(&self) -> Option<u32> {
        match self.id {
            0..=15 => Some(100),
            16..=17 => Some(16),
            18 => Some(24),
            19 | 20 => Some(14),
            _ => None
        }
    }

    pub fn unit_in(&mut self, _start: usize) {
        println!("unit number {} in", self.id);
        println!("not yet implemented.");
    }

    pub fn unit_out(&self, start: usize, computer: &MIXComputer) {
        println!("unit number {}", self.id);
        let size = self.get_block_size().expect("Unit invalid.") as usize;
        for i in start..(start+size) {
            println!("[INFO] unit {} : {}", self.id, computer.memory[i].0);
        }
    }
}
