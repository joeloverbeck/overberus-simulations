use controllers::console_input_controller_trait::ConsoleInputControllerTrait;

pub struct ConsoleInputController {}

impl Default for ConsoleInputController {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleInputController {
    pub fn new() -> ConsoleInputController {
        ConsoleInputController {}
    }
}

impl ConsoleInputControllerTrait for ConsoleInputController {
    fn does_console_argument_exist(&self, argument_number: usize) -> bool {
        let possible_argument = std::env::args().nth(argument_number);
        possible_argument.is_some()
    }

    fn parse_console_argument_number_as_type<T: std::str::FromStr>(
        &self,
        argument_number: usize,
    ) -> std::result::Result<T, <T as std::str::FromStr>::Err> {
        std::env::args().nth(argument_number).unwrap().parse()
    }

    fn get_console_argument_number(&self, argument_number: usize) -> std::string::String {
        std::env::args().nth(argument_number).unwrap()
    }
}
