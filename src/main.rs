use serde_derive::{Serialize, Deserialize};
use cli_table::{print_stdout, WithTitle, Table, Color};

#[derive(Table, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WeatherTable {
    city: String,
    country: String,
    #[table(color = "Color::Rgb(0, 128, 128)")]
    temp: f64,
    #[table(color = "Color::Rgb(0, 255, 0)")]
    feels_like: f64,
    #[table(color = "Color::Rgb(0, 255, 255)")]
    temp_min: f64,
    #[table(color = "Color::Rgb(255, 204, 203)")]
    temp_max: f64,
}

const URL: &str = "https://weather-server.takashiidobe.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client.get(URL).send().await?;

    let weather_table = res.json::<WeatherTable>().await?;

    print_stdout(vec![weather_table].with_title())?;
    Ok(())
}
