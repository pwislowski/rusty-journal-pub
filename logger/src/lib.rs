use std::fs;
use std::io::{Result as ioResult, Write};

pub fn create_directory() -> ioResult<()> {
    fs::create_dir_all("logs")?;
    Ok(())
}

pub fn create_logfile(exchange: &str, pair: &str, content: &str) -> ioResult<()> {
    let file_name = "logs/".to_string() + exchange + "_" + pair + ".json";
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn read_logfile(exchange: &str, pair: &str) -> ioResult<String> {
    let file_name = "logs/".to_string() + exchange + "_" + pair + ".json";
    let content = fs::read_to_string(file_name)?;

    Ok(content)
}

#[allow(dead_code)]
fn remove_test_files() -> ioResult<()> {
    fs::remove_dir_all("logs")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_dir() {
        let test = create_directory();
        assert!(test.is_ok());
    }

    #[test]
    fn test_create_logfile() {
        let test = create_logfile("Bybit", "BTCUSDT", "Hello world");
        assert!(test.is_ok());
    }

    #[test]
    fn test_read_logfile() {
        let test = read_logfile("Bybit", "BTCUSDT");
        assert_eq!("Hello world".to_string(), test.unwrap());
    }

    #[test]
    #[ignore = "Causes other tests to fail"]
    fn test_remove_test_files() {
        let test = remove_test_files();
        assert!(test.is_ok());
    }
}
