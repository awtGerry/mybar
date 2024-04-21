use battery;
use super::component::Component;

#[allow(unused_assignments)]
pub fn get_charge() -> String {
    let manager = battery::Manager::new().unwrap();
    let mut icon = String::new();
    let mut charge = String::new();
    match manager.batteries().unwrap().next() {
        Some(Ok(battery)) => {
            icon = match battery.state() {
                battery::State::Charging => "".to_string(),
                battery::State::Discharging => change_icon((battery.state_of_charge().value * 100.0) as u32),
                battery::State::Empty => "".to_string(),
                battery::State::Full => "".to_string(),
                battery::State::Unknown => "󰂃".to_string(),
                battery::State::__Nonexhaustive => "󰂃".to_string(),
            };
            charge = format!("{:.0}%", battery.state_of_charge().value * 100.0);
        }
        Some(Err(_e)) => {
            icon = "".to_string();
            charge = "".to_string();
        }
        None => {
            icon = "".to_string();
            charge = "".to_string();
        }
    };
    Component::new(icon, charge)
}

fn change_icon(charge: u32) -> String {
    println!("{}", charge);
    let icon = match charge {
        0..=20 => "".to_string(),
        21..=40 => "".to_string(),
        41..=60 => "".to_string(),
        61..=80 => "".to_string(),
        81..=100 => "".to_string(),
        _ => "󰂃".to_string(),
    };
    icon
}
