extern crate chrono;
use self::chrono::prelude::*;

pub fn generate_time_tag_as_string() -> String {
    let dt = Local::now();

    format!(
        "{}{}{}_{}{}{}",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    )
}
