use crate::my_commands::weather_struct::WeatherData;
use clap::{error::ErrorKind, Arg, ArgMatches, Command, Error};
use flatten_json_object::Flattener;
use reqwest::Url;

const API_KEY: &str = "b5995f80856bc3401cd7dca81357bd81";

fn processor(obj: String) -> Result<(), Error> {
    let json = serde_json::from_str(&obj).map_err(|_err| Error::new(ErrorKind::InvalidValue))?;
    let data = Flattener::new()
        .flatten(&json)
        .map_err(|_err| Error::new(ErrorKind::InvalidValue))?;
    for i in data.as_object().unwrap().into_iter() {
        println!(
            "{}: {}",
            i.0.replace("main.", "")
                .replace("sys.", "")
                .replace("0.", "")
                .replace(".", " "),
            i.1
        );
    }

    Ok(())
}

pub fn build_command() -> Command {
    Command::new("city").about("Enter a City Name").arg(
        Arg::new("city")
            .short('c')
            .long("city")
            .required(true)
            .help("City Name"),
    )
}

impl WeatherData {
    async fn get(city_name: &String) -> Result<String, Error> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city_name, API_KEY
        );

        let url = Url::parse(&*url).map_err(|_err| Error::new(ErrorKind::InvalidValue))?;
        let res = reqwest::get(url)
            .await
            .map_err(|_err| Error::new(ErrorKind::InvalidValue))?
            .text()
            .await
            .map_err(|_err| Error::new(ErrorKind::InvalidValue))?;
        Ok(res)
    }
}

pub async fn process_command(match_result: &ArgMatches) -> Result<(), Error> {
    if let Some(city_matches) = match_result.subcommand_matches("city") {
        let city = city_matches.get_one::<String>("city").unwrap();
        let res = WeatherData::get(city)
            .await
            .map_err(|_err| Error::new(clap::error::ErrorKind::InvalidValue))?;
        processor(res).map_err(|_err| Error::new(clap::error::ErrorKind::Format))?;
    }
    Ok(())
}
