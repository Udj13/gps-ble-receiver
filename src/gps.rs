use colored::*;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::Hertz;
use esp_idf_hal::sys::EspError;
use esp_idf_hal::uart::UartDriver;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub struct Gps {
    pub rx: Receiver<String>,
}

impl Gps {
    pub fn new() -> Gps {
        let uart = Gps::start_gps_uart().expect("Can't create UART");
        let mut nmea_buffer = String::with_capacity(100);
        let mut buf = [0_u8; 1];

        let (tx, rx) = mpsc::channel::<String>();

        std::thread::spawn(move || loop {
            uart.read(&mut buf, BLOCK).expect("UART GPS can't read");
            nmea_buffer.push(char::from(buf[0]));

            if nmea_buffer.ends_with("\r\n") {
                if nmea_buffer.contains("GNRMC") {
                    println!("{}", nmea_buffer.white().on_green());
                    tx.send(nmea_buffer.clone())
                        .expect("Can't send from GPS thread");
                    nmea_buffer.clear();
                } else {
                    //print!("{}", nmea_buffer.green());
                }
                nmea_buffer.clear();
            }
        });

        Gps { rx }
    }

    fn start_gps_uart() -> Result<UartDriver<'static>, EspError> {
        let peripherals = Peripherals::take().expect("Can't take peripherals for GPS UART");
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
}
