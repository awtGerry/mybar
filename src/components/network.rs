use super::component::Component;

use interfaces::Interface;

pub fn get_network() -> String {
    let mut ifs = Interface::get_all().expect("Failed to get interfaces");
    let mut iname = String::new();
    let mut icon = String::new();
    // Loop through interfaces
    for i in &mut ifs {
        // Remove loopback interface
        if !i.is_loopback() {
            if i.is_running() {
                iname = i.name.clone();
                // Check if iname is wifi or ethernet
                if iname.contains("enp") || iname.contains("eth") {
                    icon = "󰈀 ".to_string();
                } else {
                    icon = "󰤨 ".to_string();
                }
            } else {
                iname = "".to_string();
                icon = "󰤭 ".to_string();
            }
        }
    }
    Component::new(icon, "".to_string())
}
