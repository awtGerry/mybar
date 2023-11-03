use sysinfo::{SystemExt, System};

use crate::components;

#[link(name = "X11")]
/* Get the X11 libraries */
extern "C" {
    // https://www.x.org/releases/X11R7.5/doc/man/man3/XOpenDisplay.3.html
    fn XOpenDisplay(display_name: *const u8) -> *mut u8;
    fn XDefaultRootWindow(display: *mut u8) -> *mut u8;
    fn XStoreName(display: *mut u8, w: *mut u8, name: *const u8) -> i32;
    fn XFlush(display: *mut u8) -> i32;
}

/* Display the bar */
pub fn display_bar() {
    let display = unsafe { XOpenDisplay(0 as *const u8) };
    let root = unsafe { XDefaultRootWindow(display) };
    let mut sys = System::new();
    loop {
        /* Components */
        sys.refresh_cpu();
        sys.refresh_memory();
        let date = components::date::get_date();
        let time = components::date::get_time();
        let network = components::network::get_network();
        let cpu = components::resources::get_cpu_usage(&mut sys);
        let ram = components::resources::get_ram_usage(&mut sys);

        let command = format!("{} : {}  {}   {}  {} \0",
                              cpu,
                              ram,
                              network,
                              date,
                              time
                              );
        unsafe { XStoreName(display, root, command.as_ptr()) };
        unsafe { XFlush(display) };

        // Repetition rate
        // TODO: maybe change this later to every component having its own rate
        // sleep for 20 seconds
        std::thread::sleep(std::time::Duration::from_nanos((1e9 * 20.) as u64));
    }
}
