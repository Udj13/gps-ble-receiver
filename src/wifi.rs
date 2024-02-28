mod osmand;

use osmand::gngga_to_osmand;

pub struct Wifi {}

impl Wifi {
    pub fn new() -> Wifi {
        Wifi {}
    }

    pub fn send(&mut self, name: &str, nmea: &str) {
        if let Some(url) = gngga_to_osmand(nmea, name) {
            println!("URL: {}", url);
        } else {
            println!("Invalid GNGGA sentence");
        }

        println!("Wifi out: {}", nmea);
    }
}
