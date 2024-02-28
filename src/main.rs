mod ble;
mod gps;
mod wifi;

use ble::Ble;
use gps::Gps;
use wifi::Wifi;

use esp_idf_hal::delay::FreeRtos;

const NMEA_PATTERN: &str = "$GNGGA";

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Start NMEA listener");
    let mut gps = Gps::new();

    log::info!("Start BLE");
    let mut ble = Ble::new("GPS");

    log::info!("Start WI-FI");
    let mut wifi = Wifi::new();

    loop {
        let nmea_filtered_result = gps.filtered_read_gps(NMEA_PATTERN);
        if let Some(nmea) = nmea_filtered_result {
            ble.send(&nmea);
            wifi.send("123456", &nmea);
        }

        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);
    }
}
