use serde_derive::{Serialize, Deserialize};
use cli_table::{print_stdout, WithTitle, Table, Color};
use std::env;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
 struct IpResponse {
     ip: String,
     version: String,
     city: String,
     region: String,
    #[serde(rename = "region_code")]
     region_code: String,
     country: String,
    #[serde(rename = "country_name")]
     country_name: String,
    #[serde(rename = "country_code")]
     country_code: String,
    #[serde(rename = "country_code_iso3")]
     country_code_iso3: String,
    #[serde(rename = "country_capital")]
     country_capital: String,
    #[serde(rename = "country_tld")]
     country_tld: String,
    #[serde(rename = "continent_code")]
     continent_code: String,
    #[serde(rename = "in_eu")]
     in_eu: bool,
     postal: String,
     latitude: f64,
     longitude: f64,
     timezone: String,
    #[serde(rename = "utc_offset")]
     utc_offset: String,
    #[serde(rename = "country_calling_code")]
     country_calling_code: String,
     currency: String,
    #[serde(rename = "currency_name")]
     currency_name: String,
     languages: String,
    #[serde(rename = "country_area")]
     country_area: f64,
    #[serde(rename = "country_population")]
     country_population: i64,
     asn: String,
     org: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Response {
     coord: Coord,
     weather: Vec<Weather>,
     base: String,
     main: Main,
     visibility: i64,
     wind: Wind,
     clouds: Clouds,
     dt: i64,
     sys: Sys,
     timezone: i64,
     id: i64,
     name: String,
     cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Coord {
     lon: f64,
     lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Weather {
     id: i64,
     main: String,
     description: String,
     icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Main {
     temp: f64,
     feels_like: f64,
     temp_min: f64,
     temp_max: f64,
     pressure: i64,
     humidity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Wind {
     speed: f64,
     deg: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Clouds {
     all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 struct Sys {
    #[serde(rename = "type")]
     type_field: i64,
     id: i64,
     country: String,
     sunrise: i64,
     sunset: i64,
}

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

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 1.8 + 32f64
}

const URL: &str = "https://api.openweathermap.org/data/2.5/weather";
const IP_URL: &str = "http://ipapi.co/json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let app_id = env::var("APP_ID").unwrap();

    let ip_res = client.get(IP_URL).send().await?;
    let ip_body = ip_res.json::<IpResponse>().await?;

    let res = client.get(URL)
    .query(&[("q", &format!("{},{}", ip_body.city, ip_body.country)), ("appid", &app_id)])
    .send().await?;

    let body = res.json::<Response>().await?;
    let weather_table = WeatherTable {
        city: body.name.to_string(),
        country: body.sys.country.to_string(),
        temp: kelvin_to_fahrenheit(body.main.temp).round(),
        feels_like: kelvin_to_fahrenheit(body.main.feels_like).round(),
        temp_min: kelvin_to_fahrenheit(body.main.temp_min).round(),
        temp_max: kelvin_to_fahrenheit(body.main.temp_max).round(),
    };

    print_stdout(vec![weather_table].with_title())?;
    Ok(())
}
