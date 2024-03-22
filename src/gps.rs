use colored::*;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{AnyIOPin};
use esp_idf_hal::prelude::Hertz;
use esp_idf_hal::uart::{UART1, UartDriver};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub struct Gps {
    pub rx: Receiver<String>,
}

impl Gps {
    pub fn new(uart1: UART1, tx:AnyIOPin, rx: AnyIOPin) -> Gps {


        println!("Starting UART");
        let config = esp_idf_hal::uart::config::Config::new().baudrate(Hertz(115_200));
        let uart = UartDriver::new(
            uart1,
            tx,
            rx,
            Option::<gpio::Gpio0>::None,
            Option::<gpio::Gpio1>::None,
            &config,
        ).expect("GPS - uart driver error");


        let mut nmea_buffer = String::with_capacity(100);
        let mut buf = [0_u8; 1];

        let (tx, rx) = mpsc::channel::<String>();

        std::thread::spawn(move || loop {
            uart.read(&mut buf, BLOCK).expect("UART GPS can't read");
            nmea_buffer.push(char::from(buf[0]));

            if nmea_buffer.ends_with("\r\n") {
                if nmea_buffer.contains("GNRMC") {
                    println!("{}", nmea_buffer.yellow());
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

}
