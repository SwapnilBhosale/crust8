const MEMORY_SIZE : usize = 4 * 1024;
use std::fmt;
use std::fmt::Debug;

pub struct InterConnect{
   memory : [usize;MEMORY_SIZE] 
}

impl Default for InterConnect {
    fn default() -> InterConnect{
        InterConnect{
            memory : [0;MEMORY_SIZE]
        }
    }
}

impl Debug for InterConnect {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.memory[..].fmt(formatter)
    }
}
