use chrono::{DateTime, NaiveDateTime, Utc};

pub fn convert_ts_milis_to_datetime(ms: u64) -> DateTime<Utc> {
    let seconds = (ms / 1000) as i64;
    let nanoseconds = ((seconds % 1000) * 1_000_000) as u32;

    let naive_datetime = NaiveDateTime::from_timestamp_opt(seconds, nanoseconds).unwrap();

    // let timestamp = NaiveDateTime::from_timestamp_millis(unix as i64).unwrap();
    let date = DateTime::from_utc(naive_datetime, Utc);
    date
}

pub fn process_api_val<T: std::str::FromStr>(res: &serde_json::Value, key: &str) -> T {
    // removes double qoutes from a value and pipes it into <T>
    let val = res.get(key);

    match val {
        Some(v) => {
            let unpacked_str = v.to_string().replace("\"", "");
            let parsed = unpacked_str.parse::<T>();

            if let Ok(target) = parsed {
                target
            } else {
                panic!("Failed to parse {}", key);
            }
        }
        None => panic!("There's no {} in API Reponse keys!", key),
    }
}
