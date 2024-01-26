/* 
 * This file contains functions for getting the date and time.
 * The date and time are formatted using the chrono crate.
 * The date and time are returned as strings.

 * You can put icons if you want.

 */

use super::component::Component;

pub fn get_date() -> String {
    let now = chrono::Local::now();
    let date = now.format("%A %d").to_string();
    let icon = String::from("");
    Component::new(icon, date)
}

pub fn get_time() -> String {
    let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
    let time = now.format("%H:%M").to_string();
    let icon = String::from("");
    Component::new(icon, time)
}
