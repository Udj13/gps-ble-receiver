use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::Hertz;
use esp_idf_hal::sys::EspError;
use esp_idf_hal::uart::UartDriver;

pub struct Gps {
    uart: UartDriver<'static>,
    nmea_buffer: String,
}

impl Gps {
    pub fn new() -> Gps {
        let uart = Gps::start_gps_uart().expect("Can't create UART");
        let nmea_buffer = String::with_capacity(1024);
        Gps { uart, nmea_buffer }
    }

    pub fn filtered_read_gps(&mut self, pattern: &str) -> Option<String> {
        let read_result = Gps::read_gps(&self.uart);
        match read_result {
            Ok(new_char) => self.nmea_buffer.push(new_char),
            Err(e) => println!("Error uart reading: {}", e),
        }

        Gps::filter_nmea(&mut self.nmea_buffer, pattern)
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

    fn filter_nmea(nmea_buffer: &mut String, pattern: &str) -> Option<String> {
        if nmea_buffer.ends_with("\r\n") {
            if nmea_buffer.contains(pattern) {
                let result = nmea_buffer.clone();
                nmea_buffer.clear();
                return Some(result);
            }
            nmea_buffer.clear();
        }
        None
    }
}
