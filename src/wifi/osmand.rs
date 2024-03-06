pub fn gngga_to_osmand(nmea_sentence: &str, id: &str) -> Option<String> {
    let fields: Vec<&str> = nmea_sentence.split(',').collect();
    if fields.len() < 6 {
        return None;
    }

    let lat = match fields[3].parse::<f64>() {
        Ok(value) => value / 100.0,
        Err(_) => return None,
    };

    let lon = match fields[5].parse::<f64>() {
        Ok(value) => value / 100.0,
        Err(_) => return None,
    };

    let time = fields[1];
    let hh = &time[0..2];
    let mm = &time[2..4];
    let ss = &time[4..6];

    let date = fields[9];
    let day = &date[0..2];
    let month = &date[2..4];
    let year = &date[4..6];

    let timestamp = format!("20{}-{}-{}T{}:{}:{}Z", year, month, day, hh, mm, ss);

    let url = format!(
        "?id={}&lat={:.6}&lon={:.6}&timestamp={}",
        id, lat, lon, timestamp,
    );

    Some(url)
}
