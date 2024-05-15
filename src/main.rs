mod ble;
mod gps;

use ble::Ble;
use gps::Gps;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();


    let peripherals = Peripherals::take().expect("GPS - can't take peripherals");


    println!("Start NMEA listener");
    let uart1 = peripherals.uart1;
    let tx = peripherals.pins.gpio16;
    let rx = peripherals.pins.gpio17;
    let gps = Gps::new(uart1, AnyIOPin::from(tx), AnyIOPin::from(rx));
    println!("----------------------------------------------------");


    println!("Start BLE");
    let mut ble = Ble::new("AGRO-GPS");

    loop {
        if let Ok(nmea) = gps.rx.recv() {
            println!("{}", nmea);

            ble.send(&nmea);
        }



        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);
    }
}
