pub fn gngga_to_osmand(nmea_sentence: &str, id: &str) -> Option<String> {
    let fields: Vec<&str> = nmea_sentence.split(',').collect();
    if fields.len() < 6 {
        return None;
    }

    let lat = match fields[2].parse::<f64>() {
        Ok(value) => value / 100.0,
        Err(_) => return None,
    };

    let lon = match fields[4].parse::<f64>() {
        Ok(value) => value / 100.0,
        Err(_) => return None,
    };

    let time = fields[1];
    let hh = &time[0..2];
    let mm = &time[2..4];
    let ss = &time[4..6];

    let timestamp = format!("{:0}:{:0}:{:0}", hh, mm, ss);

    let url = format!(
        "http://demo.traccar.org:5055/?id={}&lat={}&lon={}&timestamp={}",
        id, lat, lon, timestamp,
    );

    Some(url)
}
