mod osmand;
mod secret;
use colored::*;


use core::convert::TryInto;

use embedded_svc::{
    http::{client::Client as HttpClient},
    io::Write,
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};
use esp_idf_hal::modem::Modem;
use esp_idf_svc::http::client::EspHttpConnection;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::eventloop::{EspSystemEventLoop};
use esp_idf_svc::nvs::{EspDefaultNvsPartition};

use log::{error, info};

pub fn start_wifi(modem: Modem) -> anyhow::Result<(BlockingWifi<EspWifi<'static>>, HttpClient<EspHttpConnection>)> {

    // Setup Wifi
    let sys_loop = EspSystemEventLoop::take().expect("WIFI - Error ESP event loop");
    let nvs = EspDefaultNvsPartition::take().expect("WIFI - Error NVS partition");

    let mut wifi:BlockingWifi<EspWifi<'static>> = BlockingWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi)?;

    // Create HTTP(S) client
    let client = HttpClient::wrap(EspHttpConnection::new(&Default::default())?);

    Ok((wifi, client))
}


pub fn send(client: &mut HttpClient<EspHttpConnection>) {

    let post_result = post_request(client);

    match post_result {
        Ok(..) => println!("POST success"),
        Err(e) => println!("{} {}", "POST error: ".on_red(), e),
    }

}


/// Send an HTTP POST request.
fn post_request(client: &mut HttpClient<EspHttpConnection>) -> anyhow::Result<()> {
    // Prepare payload
    let payload = b"";

    // Prepare headers and URL
    let content_length_header = format!("{}", payload.len());
    let headers = [
        ("content-type", "text/plain"),
        ("content-length", &*content_length_header),
    ];
    let url = "http://free-gps.ru:5055/?id=12345&lat=48.8566&lon=2.3522&timestamp=2021-01-02T00:00:00Z&speed=15";

    // Send request
    let mut request = client.post(url, &headers)?;
    request.write_all(payload)?;
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

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: secret::SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: secret::PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
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



