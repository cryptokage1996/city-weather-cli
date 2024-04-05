mod my_commands;
mod process;
use clap::command;

#[tokio::main]
async fn main() {
    let match_result = command!()
        .about("Weather cli")
        .subcommand(my_commands::city_command::build_command())
        .get_matches();

    (process::the_process::process_commands(&match_result).await).unwrap();
}
