// use sysinfo::{SystemExt, CpuExt};
// use sysinfo::System

use super::component::Component;

pub fn get_cpu_usage(sys: &mut sysinfo::System) -> String {
    let mut cpu_usage: u16 = 0;
    /* First sum the cpu usage of all the cores */
    for cpu in sys.cpus() {
        cpu_usage += cpu.cpu_usage() as u16;
    }
    /* Then divide by the number of cores to get the average */
    if sys.cpus().len() > 0 {
        cpu_usage /= sys.cpus().len() as u16;
    }

    let icon = String::from(" ");
    let cpu_usage = format!("{}%", cpu_usage);
    Component::new(icon, cpu_usage)
}

pub fn get_ram_usage(sys: &mut sysinfo::System) -> String {
    // Get free memory in bytes
    // let available_ram = sys.free_memory();
    // Get total memory in bytes
    let total_ram = sys.total_memory();

    // Calculate percentage of used memory
    // let ram_usage = (total_ram - available_ram) as f32 / total_ram as f32 * 100.0;
    // let ram_usage = format!("{:.1}%", ram_usage);

    let icon = String::from(" ");
    Component::new(icon, total_ram.to_string())
}
