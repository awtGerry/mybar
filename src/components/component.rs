#[allow(unused)]
pub struct Component {
    icon: String,
    command: String,
}

impl Component {
    pub fn new(icon: String, command: String) -> String {
        format!("{}{}", icon, command)
    }
}
