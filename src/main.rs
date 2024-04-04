use reqwest;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
}

#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    wind: Wind,
}

#[derive(Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize)]
struct Wind {
    speed: f32,
}

#[derive(Deserialize)]
struct CityResponse {
    name: String,
    lat: f32,
    lon: f32,
}

struct City {
    name: String,
    lat: f32,
    lon: f32,
}

fn print_table_cli (city: &City, weather_response: &WeatherResponse) {
    println!("---------------------------------");
    println!("| City  ");
    println!("---------------------------------");
    println!("| {:<10}", city.name);
    println!("---------------------------------");
    println!("| Temperature| Feels Like | Wind Speed |");
    println!("---------------------------------");
    println!("| {:<11} | {:<10} | {:<10} |", weather_response.main.temp, weather_response.main.feels_like, weather_response.wind.speed);
    println!("---------------------------------");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    dotenv::dotenv().ok();
    let api_key = std::env::
    var("API_KEY").expect("API_KEY must be set");
    let city_url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&limit={}&appid={}", args.city, 1, api_key);
    let city_response = reqwest::get(&city_url).await?.json::<Vec<CityResponse>>().await?;
    let cities = city_response.iter().map(|city| City {
        name: city.name.clone(),
        lat: city.lat,
        lon: city.lon,
    }).collect::<Vec<City>>();

    // For each city, get the weather
    for city in cities {
        let weather_url = format!("http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", city.lat, city.lon, api_key);
        let weather_response = reqwest::get(&weather_url).await?.json::<WeatherResponse>().await?;
        print_table_cli(&city, &weather_response);
    }

    Ok(())
}