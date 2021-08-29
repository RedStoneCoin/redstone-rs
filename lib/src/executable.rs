use crate::state::GlobalState;

pub trait Executable {
    fn execute(&self, context: &String, state: Option<&mut GlobalState>) -> Result<String, Box<dyn std::error::Error>>;
    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>>; // is this valid executable
    fn cost(&self, context: &String) -> u64; // returns the expected cost in microstones
}