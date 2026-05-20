use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub main : Main,
    pub weather : Weather: Vec<<Weather>,
    pub name :String,
    
    
}
pub struct Main{
    pub temp: f32,
    pub pressure: u8,
    pub humidity: u8,
    pub feels_like : f64,
}
pub struct Weather{
    pub main : String,
    pub description: String,
}