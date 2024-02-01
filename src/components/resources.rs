// use sysinfo::{SystemExt, CpuExt};
// use sysinfo::System

use super::component::Component;

pub fn get_cpu_usage(sys: &mut sysinfo::System) -> String {
    let mut cpu_usage: u16 = 0;
    sys.refresh_cpu_usage();
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
    sys.refresh_memory();
    let total_ram = sys.total_memory();
    let used_ram = sys.used_memory();
    /* let ram_usage = format!("{} / {}",
                           used_ram / 1024 / 1024,
                           total_ram / 1024 / 1024); */
    let ram_percentage = format!("{}", (used_ram as f32 / total_ram as f32) * 100.);
    let icon = String::from(" ");
    Component::new(icon, ram_percentage)
}
