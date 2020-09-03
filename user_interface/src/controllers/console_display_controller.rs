extern crate termcolor;
use self::termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use controllers::display_controller_trait::DisplayControllerTrait;

use std::io::Write;

pub struct ConsoleDisplayController {
    buffer_writer: BufferWriter,
}

impl ConsoleDisplayController {
    pub fn new() -> ConsoleDisplayController {
        ConsoleDisplayController {
            buffer_writer: BufferWriter::stdout(ColorChoice::Always),
        }
    }
}

impl Default for ConsoleDisplayController {
    fn default() -> Self {
        Self::new()
    }
}

fn reset_console_output_to_normal(buffer: &mut Buffer) -> Result<&mut Buffer, String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = writeln!(buffer) {
        return Err(error.to_string());
    }

    Ok(buffer)
}

fn write_instruction<'a>(text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
    // Write the instruction "tag".

    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Yellow))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    let instruction_tag = "  [<>]".to_string();

    if let Err(error) = write!(buffer, "{}", instruction_tag) {
        return Err(error.to_string());
    }

    write_regular_text(text, buffer)?;

    reset_console_output_to_normal(buffer)?;

    Ok(buffer)
}

fn write_alert<'a>(text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
    reset_console_output_to_normal(buffer)?;

    // Write the actual text regarding the alert
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Black))
            .set_bg(Some(Color::Red)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, " --{}-- ", text.to_uppercase()) {
        return Err(error.to_string());
    }

    reset_console_output_to_normal(buffer)?;
    reset_console_output_to_normal(buffer)?;

    Ok(buffer)
}

fn set_console_output_to_standard_for_announcement(buffer: &mut Buffer) -> Result<(), String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Black))
            .set_bg(Some(Color::White)),
    ) {
        return Err(error.to_string());
    }

    Ok(())
}

fn write_information<'a>(text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
    // Write the information "tag".

    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Blue))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    let information_tag = "  (*)".to_string();

    if let Err(error) = write!(buffer, "{}", information_tag) {
        return Err(error.to_string());
    }

    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Cyan))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", text) {
        return Err(error.to_string());
    }

    reset_console_output_to_normal(buffer)?;

    Ok(buffer)
}

fn write_announcement<'a>(text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
    // Bookend the announcement with empty lines.
    reset_console_output_to_normal(buffer)?;
    reset_console_output_to_normal(buffer)?;

    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_bg(Some(Color::White)),
    ) {
        return Err(error.to_string());
    }

    let bookend = " *** ".to_string();

    if let Err(error) = write!(buffer, "{}", bookend) {
        return Err(error.to_string());
    }

    set_console_output_to_standard_for_announcement(buffer)?;

    // Gotta change settings to the standard for an announcement, just in case.
    set_console_output_to_standard_for_announcement(buffer)?;

    if let Err(error) = write!(buffer, "{}", text.to_uppercase()) {
        return Err(error.to_string());
    }

    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_bg(Some(Color::White)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", bookend) {
        return Err(error.to_string());
    }

    // Doing it twice because it's nice.
    reset_console_output_to_normal(buffer)?;
    reset_console_output_to_normal(buffer)?;

    Ok(buffer)
}

fn write_regular_text(text: &str, buffer: &mut Buffer) -> Result<(), String> {
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", text) {
        return Err(error.to_string());
    }

    Ok(())
}

fn write_section<'a>(text: &str, buffer: &'a mut Buffer) -> Result<&'a mut Buffer, String> {
    reset_console_output_to_normal(buffer)?;

    // Write the instruction "tag".
    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    let section_tag_start = " /// ".to_string();
    let section_tag_end = " ///".to_string();

    if let Err(error) = write!(buffer, "{}", section_tag_start) {
        return Err(error.to_string());
    }

    // Write the actual text regarding the section
    write_regular_text(&text.to_uppercase(), buffer)?;

    if let Err(error) = buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_bg(Some(Color::Black)),
    ) {
        return Err(error.to_string());
    }

    if let Err(error) = write!(buffer, "{}", section_tag_end) {
        return Err(error.to_string());
    }

    reset_console_output_to_normal(buffer)?;
    reset_console_output_to_normal(buffer)?;

    Ok(buffer)
}

impl DisplayControllerTrait for ConsoleDisplayController {
    fn write_announcement(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self.buffer_writer.print(write_announcement(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    fn write_information(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self.buffer_writer.print(write_information(
            &(" ".to_string() + &text.to_string()),
            buffer,
        )?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    fn write_section(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self.buffer_writer.print(write_section(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    fn write_alert(&self, text: &str) -> Result<(), String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self.buffer_writer.print(write_alert(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }

    fn write_instruction(&self, text: &str) -> std::result::Result<(), std::string::String> {
        let buffer = &mut self.buffer_writer.buffer();

        let buffer_writer_result = self.buffer_writer.print(write_instruction(text, buffer)?);

        if let Err(error) = buffer_writer_result {
            return Err(error.to_string());
        }

        Ok(())
    }
}
