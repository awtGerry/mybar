#[allow(unused)]
pub struct Component {
    icon: String,
    command: String
}

impl Component {
    fn new(icon: String, command: String) -> String {
        format!("{}{}", icon, command)
    }
}

pub fn get_date() -> String {
    let now = chrono::Local::now();
    let date = now.format("%a %b %-d").to_string();
    Component::new("".to_string(), date)
}

pub fn get_time() -> String {
    let now = chrono::Local::now();
    let time = now.format("%H:%M").to_string();
    Component::new("󰥔 ".to_string(), time)
}
