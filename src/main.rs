mod example;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;

const NMEA_PATTERN: &str = "$GNGGA";

mod gps {
    use esp_idf_hal::delay::BLOCK;
    use esp_idf_hal::gpio;
    use esp_idf_hal::peripherals::Peripherals;
    use esp_idf_hal::prelude::Hertz;
    use esp_idf_hal::sys::EspError;
    use esp_idf_hal::uart::UartDriver;

    pub struct GPS {
        _uart: UartDriver<'static>,
        //mut nmea_buffer : String::new(),
    }

    impl GPS {
        pub fn new() -> GPS {
            let _uart = start_gps_uart().expect("Can't create UART");
            //nmea_buffer = ;
            GPS { _uart }
        }
    }

    fn start_gps_uart() -> Result<UartDriver<'static>, EspError> {
        let peripherals = Peripherals::take()?;
        let tx = peripherals.pins.gpio16;
        let rx = peripherals.pins.gpio17;

        println!("Starting UART");
        let config = esp_idf_hal::uart::config::Config::new().baudrate(Hertz(115_200));
        let uart_result = UartDriver::new(
            peripherals.uart1,
            tx,
            rx,
            Option::<gpio::Gpio0>::None,
            Option::<gpio::Gpio1>::None,
            &config,
        );

        uart_result
    }


    fn read_gps(uart: &UartDriver) -> Result<char, anyhow::Error> {
        let mut buf = [0_u8; 1];
        uart.read(&mut buf, BLOCK)?;

        Ok(char::from(buf[0]))
    }


    fn filter_nmea(nmea_buffer: &mut String, pattern: &str) -> Result<String, anyhow::Error> {
        if nmea_buffer.ends_with("\r\n") {
            if nmea_buffer.contains(pattern) {
                let result = nmea_buffer.clone();
                nmea_buffer.clear();
                return Ok(result);
            }
            nmea_buffer.clear();
        }
        Err(anyhow::anyhow!("Error: NMEA buffer does not contain NMEA_PATTERN"))
    }

//     pub fn filtered_read_gps(uart: &UartDriver, nmea_buffer: &mut String, pattern: &str) -> Result<String, anyhow::Error> {
// let read_result = read_gps(&uart);
// match read_result {
// Ok(new_char) => nmea_buffer.push(new_char),
// Err(e) => println!("Error uart reading: {}", e),
// }
//
//         let filtered_result = filter_nmea(&mut nmea_buffer);
//         match filtered_result {
//             Ok(nmea) => Ok(nmea),
//             Err(e) => Err(e),
//         }
// }
}

fn send(nmea_buffer: String) {
    print!("{}", nmea_buffer);
}


fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_hal::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello!");
    let gps = gps::GPS::new();

    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);

        // let nmea_filtered_result = gps::filtered_read_gps(NMEA_PATTERN);
        // match nmea_filtered_result {
        //     Ok(nmea) => send(nmea),
        //     _ => {},
        // }
    }
}
