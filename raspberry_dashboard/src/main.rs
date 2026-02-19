slint::include_modules!();

use sysinfo::{Components, Disks, System};
use std::time::Duration;

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    let ui_handle = ui.as_weak();

    // Initialize system info
    let mut system = System::new_all();
    let mut disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();

    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_secs(1), move || {
        let ui = ui_handle.unwrap();

        // Refresh system info
        system.refresh_all();
        disks.refresh(true);
        components.refresh(true);
        system.refresh_cpu_usage(); // Specific refresh for CPU usage

        // Memory
        let used_mem = system.used_memory();
        let total_mem = system.total_memory();
        let mem_percent = used_mem as f32 / total_mem as f32;
        ui.set_memory_text(format!("Memory: {} MB / {} MB", used_mem / 1024 / 1024, total_mem / 1024 / 1024).into());
        ui.set_memory_percent(mem_percent);
        
        // CPU Usage
        let mut cpu_list = Vec::new();
        for (i, cpu) in system.cpus().iter().enumerate() {
            let name = format!("CPU {}", i);
            let usage = cpu.cpu_usage() / 100.0; // cpu_usage() returns 0-100
            
            cpu_list.push(CpuInfo {
                name: name.into(),
                usage_percent: usage,
            });
        }
        let cpu_model = std::rc::Rc::new(slint::VecModel::from(cpu_list));
        ui.set_cpus(cpu_model.into());

        // Disks - Dynamic Detection
        let mut disk_list = Vec::new();
        // Simply list all disks found by sysinfo.
        // It's possible users want to see everything including small partitions or loop devices.
        // If too noisy, users can filter later.
        for disk in disks.list() {
            let name = disk.mount_point().to_string_lossy().to_string();
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let used_space = total_space - available_space;
            
            let disk_percent = if total_space > 0 {
                used_space as f32 / total_space as f32
            } else {
                0.0
            };
            
            let usage_text = format!("{:.2} GB / {:.2} GB", used_space as f64 / 1e9, total_space as f64 / 1e9);

            disk_list.push(DiskInfo {
                name: name.into(),
                usage_text: usage_text.into(),
                usage_percent: disk_percent,
            });
        }
        
        let disk_model = std::rc::Rc::new(slint::VecModel::from(disk_list));
        ui.set_disks(disk_model.into());

        // Temperature
        // Try to find CPU or SoC thermal zone. RPi usually has "cpu-thermal" or "soc-thermal".
        // If not specific, take the highest temp found.
        let mut temp: f32 = 0.0;
        let mut found = false;
        
        for component in components.iter() {
            let label = component.label().to_lowercase();
            if label.contains("cpu") || label.contains("soc") {
                temp = component.temperature().unwrap_or(0.0);
                found = true;
                break;
            }
        }
        
        if !found {
            // Fallback to first available if any
            if let Some(comp) = components.iter().next() {
                temp = comp.temperature().unwrap_or(0.0);
            }
        }
        
        ui.set_temp_text(format!("Temperature: {:.1} °C", temp).into());
        // Normalize 0-100C for progress bar
        ui.set_temp_percent(temp / 100.0);
    });

    ui.run()
}
