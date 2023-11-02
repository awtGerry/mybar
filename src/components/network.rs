// use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};
//
use std::process::{Command, Stdio};
use std::io::Read;

use super::component::Component;

/* TODO:
   Implement this using rust sysinfo for network
   right now it is just go for what i already know
*/

pub fn get_network() -> String {
    // Create a command to run the desired shell command
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg("nmcli con | awk 'NR==2' | cut -c 69-");
    cmd.stdout(Stdio::piped());

    // Try to run the command
    let child = cmd.spawn().expect("Failed to start command");

    // Create a string to store the command's output
    let mut output = String::new();

    // Read the command's output into the string
    if let Some(mut stdout) = child.stdout {
        stdout.read_to_string(&mut output).expect("Failed to read command output");
    }

    // Remove the newline character
    let output = output.replace("\n", "");
    // Remove the spaces
    let output = output.replace(" ", "");

    /* TODO: Implement different icons for different networks
       For example, if the network is wired, use a wired icon
       If the network is wireless, use a wireless icon
       If no network is connected, use a disconnected icon
    */

    let icon = String::from(" ");
    Component::new(icon, output)
}
