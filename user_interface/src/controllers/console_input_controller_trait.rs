pub trait ConsoleInputControllerTrait {
    fn get_console_argument_number(&self, argument_number: usize) -> String;
    fn does_console_argument_exist(&self, argument_number: usize) -> bool;
    fn parse_console_argument_number_as_type<T: std::str::FromStr>(
        &self,
        argument_number: usize,
    ) -> Result<T, <T as std::str::FromStr>::Err>;
}
