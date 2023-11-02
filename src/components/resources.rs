use sysinfo::{SystemExt, CpuExt};

use super::component::Component;

pub fn get_cpu_usage(sys: &mut sysinfo::System) -> String {
    let mut cpu_usage: u16 = 0;
    /* First sum the cpu usage of all the cores */
    for cpu in sys.cpus() {
        cpu_usage += cpu.cpu_usage() as u16;
    }
    /* Then divide by the number of cores */
    cpu_usage /= sys.cpus().len() as u16;

    let icon = String::from(" ");
    let cpu_usage = format!("{}%", cpu_usage);
    Component::new(icon, cpu_usage)
}

pub fn get_ram_usage(sys: &mut sysinfo::System) -> String {
    let total_ram = sys.total_memory();
    let used_ram = sys.used_memory();
    /* Calculate the percentage of ram used */
    let ram_usage = (used_ram as f32 / total_ram as f32 * 100.) as u16;
    let ram_usage = format!("{}%", ram_usage);
    let icon = String::from(" ");
    Component::new(icon, ram_usage)
}
