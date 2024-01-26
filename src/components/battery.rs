fn battery_status() -> battery::Result<String> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => return Err(e),
        None => return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into()),
    };

    if let Ok(battery) = manager.refresh(&mut battery) {
        Ok(format!("{:?}%", battery))
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound).into())
    }
}

pub fn set_battery_charge() -> String {
    let battery = battery_status();
    match battery {
        Ok(battery) => battery,
        Err(_) => "".to_string()
    }
}
