mod osmand;
mod secret;
// wifi/secret.rs:
// pub const SSID: &str = "WIFI_SSID";
// pub const PASSWORD: &str = "WIFI_PASS";

use osmand::gngga_to_osmand;

use embedded_svc::http::{client::Client as HttpClient};
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::modem::Modem;
use embedded_svc::{io::Write, utils::io};

use esp_idf_svc::wifi::{BlockingWifi, EspWifi, };
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_svc::http::client::EspHttpConnection;
use log::{error, info};
use secret::{PASSWORD, SSID};


pub struct Wifi {
    client: HttpClient<EspHttpConnection>,
    connected : bool,
}

impl Wifi {
    pub fn new(modem: Modem) -> Wifi {
        let sys_loop = EspSystemEventLoop::take().expect("WIFI - can't take system event loop");
        let nvs = EspDefaultNvsPartition::take().expect("WIFI - can't take default nvs partition");

        let mut wifi = BlockingWifi::wrap(
            EspWifi::new(modem, sys_loop.clone(), Some(nvs)).expect("WIFI - error EspWifi::new"),
            sys_loop,
        ).expect("WIFI - can't get wifi driver");


        Self::connect_wifi(&mut wifi).expect("WIFI - can't connect");

        let ip_info = wifi.wifi().sta_netif().get_ip_info().expect("WIFI - can't get ip");
        info!("Wifi DHCP info: {:?}", ip_info);

        // Create HTTP(S) client
        let client = HttpClient::wrap(EspHttpConnection::new(&Default::default()).expect("WIFI - Can't create client"));

        let connected = false;
        Wifi { client, connected }

    }


    fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {

        let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
            ssid: SSID.parse().expect("SSID parsing error"),
            bssid: None,
            auth_method: AuthMethod::WPA2Personal,
            password: PASSWORD.parse().expect("PASSWORD parsing error"),
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


    pub fn send(&mut self, url: &str, name: &str, nmea: &str){
        if let Some(payload) = gngga_to_osmand(nmea, name) {
            println!("URL: {}", url);
            println!("Wifi out: {}", payload);
            let result = self.post_request(url, payload);
            match result {
                Err(..) => println!("Can't POST request"),
                _ => {}
            }

        } else {
            println!("Wifi out: Invalid GNGGA sentence parsing");
        }
    }


    /// Send an HTTP POST request.
    fn post_request(&mut self, url: &str, payload: String) -> anyhow::Result<()> {
        // Prepare headers and URL
        let content_length_header = format!("{}", payload.len());
        let headers = [
            ("content-type", "text/plain"),
            ("content-length", &*content_length_header),
        ];
        // Send request
        let mut request = self.client.post(url, &headers)?;
        request.write_all(payload.as_bytes())?;
        request.flush()?;
        info!("-> POST {}", url);
        let mut response = request.submit()?;

        // Process response
        let status = response.status();
        info!("<- {}", status);
        let mut buf = [0u8; 1024];
        let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
        info!("Read {} bytes", bytes_read);
        match std::str::from_utf8(&buf[0..bytes_read]) {
            Ok(body_string) => info!(
            "Response body (truncated to {} bytes): {:?}",
            buf.len(),
            body_string
        ),
            Err(e) => error!("Error decoding response body: {}", e),
        };

        // Drain the remaining response bytes
        while response.read(&mut buf)? > 0 {}

        Ok(())
    }

}
