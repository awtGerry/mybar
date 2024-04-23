use super::component::Component;

use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::io;
use std::path::PathBuf;

pub struct Brightness {
    backend: String,
    max_brightness: i32,
}

impl std::default::Default for Brightness {
    fn default() -> Brightness {
        return Brightness {
            backend: "intel_backlight".to_string(),
            max_brightness: 0,
        }
    }
}

impl Brightness {
    fn get(&self, filename: &str) -> Result<i32, io::Error> {
        let mut path_buffer = PathBuf::from("/sys/class/backlight");
        path_buffer.push(self.backend.clone());
        path_buffer.push(filename);

        let path = path_buffer.as_path();
        let mut file = File::open(path)?;

        let mut content = String::new();
        file.read_to_string(&mut content)?;

        match content.trim().parse::<i32>() {
            Ok(value) => Ok(value),
            Err(_) => {
                Ok(-1)
            }
        }
    }

    pub fn get_max_brightness(&self) -> Result<i32, io::Error> {
        if self.max_brightness > 0 {
            return Ok(self.max_brightness);
        }
        return self.get("max_brightness");
    }

    pub fn get_brightness(&self) -> Result<i32, io::Error> {
        return self.get("brightness");
    }

    pub fn get_percent(&self) -> Result<i32, io::Error> {
        let value = self.get_brightness()? as f32;
        let max = self.get_max_brightness()? as f32;
        let result = (100 as f32) * (value + 0.5) / max;
        return Ok(result as i32);
    }

}

pub fn get_backlight() -> String {
    let brightness = Brightness::default();
    let percent = brightness.get_percent().unwrap();
    let icon = "".to_string();
    let value = format!("{}%", percent);
    Component::new(icon, value).to_string()
}
