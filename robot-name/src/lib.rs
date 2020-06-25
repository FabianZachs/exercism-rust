use rand::prelude::*;
use std::char;

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Robot(Robot::get_random_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Robot::get_random_name()
    }

    fn get_random_name() -> String {
        let mut rng = rand::thread_rng();
        let num = format!("{:03}", rng.gen_range(0, 1000));
        let char_1: u8 = rng.gen_range(b'A', b'Z' + 1);
        let char_2: u8 = rng.gen_range(b'A', b'Z' + 1);

        format!("{}{}{}", char_1 as char, char_2 as char, num)
    }
}
