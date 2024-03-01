mod osmand;
mod secret;
// wifi/secret.rs:
// pub const SSID: &str = "WIFI_SSID";
// pub const PASSWORD: &str = "WIFI_PASS";

use osmand::gngga_to_osmand;

use esp_idf_svc::hal::prelude::Peripherals;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::info;
use secret::{PASSWORD, SSID};


pub struct Wifi {}

impl Wifi {
    pub fn new() -> Wifi {

        let peripherals = Peripherals::take().expect("WIFI - can't take peripherals");
        let sys_loop = EspSystemEventLoop::take().expect("WIFI - can't take system event loop");
        let nvs = EspDefaultNvsPartition::take().expect("WIFI - can't take default nvs partition");

        let mut wifi = BlockingWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
            sys_loop,
        )?;

        Self::connect_wifi(&mut wifi).unwrap();

        let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();

        info!("Wifi DHCP info: {:?}", ip_info);

        info!("Shutting down in 5s...");

        Wifi {}
    }


    fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {

        let wifi_configuration : embedded_svc::wifi::Configuration = Configuration::Client(ClientConfiguration {
            ssid: SSID.into(),
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            password: PASSWORD.into(),
            channel: None,
        });

        wifi.set_configuration(&wifi_configuration)?;

        wifi.start()?;
        info!("Wifi started");

        wifi.connect()?;
        info!("Wifi connected");

        wifi.wait_netif_up()?;
        info!("Wifi netif up");

        Ok(())
    }


    pub fn send(&mut self, name: &str, nmea: &str) {
        if let Some(url) = gngga_to_osmand(nmea, name) {
            println!("URL: {}", url);
        } else {
            println!("Wifi out: Invalid GNGGA sentence parsing");
        }

        println!("Wifi out: {}", nmea);
    }
}
