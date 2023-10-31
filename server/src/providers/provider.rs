use std::error::Error;


pub trait Provider {
    fn instruct(&self) -> Result<String, Box<dyn Error>>;

    fn get_params(&self) -> Result<String, Box<dyn Error>>;
}