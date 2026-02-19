use sysinfo::{Disks, System};

fn main() {
    let disks = Disks::new_with_refreshed_list();
    println!("Disks found: {}", disks.list().len());
    for disk in disks.list() {
        println!("Name: {}, Mount Point: {}, Total: {}, Available: {}, Default: {}",
            disk.name().to_string_lossy(),
            disk.mount_point().display(),
            disk.total_space(),
            disk.available_space(),
            disk.is_removable()
        );
    }
}
