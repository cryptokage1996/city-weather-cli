use crate::my_commands::city_command::process_command as fetch_city;
use clap::{error::Error, ArgMatches};

pub async fn process_commands(match_result: &ArgMatches) -> Result<(), Error> {
    fetch_city(match_result).await
}
