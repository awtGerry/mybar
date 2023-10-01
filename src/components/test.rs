struct Commands {
    commands: Vec<String>,
}

struct Command {
    icon: String,
    command: String,
}

impl Commands {
    fn new() -> Commands {
        Commands {
            commands: Vec::new(),
        }
    }

    fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    fn get_commands(&self) -> Vec<String> {
        self.commands.clone()
    }
}

impl Command {
    fn new(icon: String, command: String) -> Command {
        Command {
            icon,
            command,
        }
    }

    fn get_command(&self) -> String {
        format!("{}{}", self.icon, self.command)
    }
}

fn get_date() -> String {
    let now = chrono::Local::now();
    let date = now.format("%a %b %-d").to_string();
    Command::new("".to_string(), date).get_command()
}

fn get_time() -> String {
    let now = chrono::Local::now();
    let time = now.format("%H:%M").to_string();
    Command::new("󰥔 ".to_string(), time).get_command()
}

fn get_battery() -> String {
    let battery = battery_status();
    match battery {
        Ok(battery) => battery,
        Err(_) => "".to_string()
    }
}

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

pub fn get_commands() -> Vec<String> {
    let mut commands = Commands::new();
    commands.add_command(get_battery());
    commands.add_command(get_date());
    commands.add_command(get_time());
    commands.get_commands()
}
