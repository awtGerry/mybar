use super::component::Component;

use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::io;
use std::path::PathBuf;

pub struct Volume {
    backend: String,
    max_volume: i32,
}

impl std::default::Default for Volume {
    fn default() -> Volume {
        return Volume {
            backend: "pulse".to_string(),
            max_volume: 0,
        }
    }
}
