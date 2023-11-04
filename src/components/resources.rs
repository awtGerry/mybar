use sysinfo::{SystemExt, CpuExt};

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
    let used_ram = sys.used_memory();
    // convert to GB
    let used_ram = used_ram as f32 / 1024.0 / 1024.0 / 1024.0;
    let used_ram = format!("{:.2} GB", used_ram);
    let icon = String::from(" ");
    Component::new(icon, used_ram)
}
