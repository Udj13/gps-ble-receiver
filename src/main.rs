mod gps;
use gps::Gps;

use esp_idf_hal::delay::FreeRtos;

const NMEA_PATTERN: &str = "$GNGGA";

fn send(nmea_buffer: String) {
    print!("{}", nmea_buffer);
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Start NMEA listener");
    let mut gps = Gps::new();

    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);

        let nmea_filtered_result = gps.filtered_read_gps(NMEA_PATTERN);
        if let Some(nmea) = nmea_filtered_result {
            send(nmea)
        }
    }
}
