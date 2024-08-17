use colored::*;
use serde::Deserialize;
use std::{env, io};

// Struct to desierialise the JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind parameters
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to get the weather information from OpenWeatherMap API
fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    // https://docs.openweather.co.uk/current
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    println!("Debugging: {}", &url);

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response
        .json::<WeatherResponse>()
        .expect("Unable to decode message from OpenWeatherAPI");

    Ok(response_json)
}

// Function to display the weather information
fn display_weather_info(response: &WeatherResponse) -> () {
    // Extract the weather information from the response
    let description: &String = &response.weather[0].description;
    let temprature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;
    // Formatting weather information into a string
    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Temprature: {:.1}
        > Humidity: {:.1}%
        > Pressure {:.1} hPa
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temprature),
        temprature,
        humidity,
        pressure,
        wind_speed
    );

    // Function to get emoji based on temprature
    fn get_temp_emoji(temprature: f64) -> &'static str {
        if temprature < 0.0 {
            "❄️"
        } else if temprature >= 0.0 && temprature < 10.0 {
            "☁️"
        } else if temprature >= 10.0 && temprature < 20.0 {
            "⛅"
        } else if temprature >= 20.0 && temprature < 30.0 {
            "🌤️"
        } else {
            "🔥"
        }
    }

    // Coloring the weather text based on weather conditions
    let weather_text_colored: ColoredString = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);
}

fn main() {
    println!("{}", "Welcome to Weather Station!".bright_yellow());
    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Uh oh, failed to read city!");
        let city: &str = city.trim();

        println!("{}", "Please enter the name of the country:".bright_green());
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Uh oh, failed to read city!");
        let country_code: &str = country_code.trim();

        // OpenWeatherMap API key
        let api_key = env::var("OPEN_WEATHER_API_KEY").unwrap_or("NO_API_KEY".to_string());

        // Calling the function to fetch weather information
        match get_weather_info(city, country_code, &api_key) {
            Ok(response) => display_weather_info(&response),
            Err(err) => eprintln!("Error: {}", err),
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        ); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
