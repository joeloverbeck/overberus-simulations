extern crate termcolor;

pub trait DisplayControllerTrait {
    fn write_announcement(&self, text: &str) -> Result<(), String>;
    fn write_information(&self, text: &str) -> Result<(), String>;
    fn write_section(&self, text: &str) -> Result<(), String>;
    fn write_alert(&self, text: &str) -> Result<(), String>;
}
