use sysinfo::{ Components, Disks, Networks, System };

#[derive(Debug)]
pub struct SystemInfo {
    pub title: String,
    pub info: String,
}

pub fn system_info() -> Vec<SystemInfo> {
    let mut sys = System::new_all();

    sys.refresh_all();
    // 
    // println!("System: ");
    // 
    // // RAM and swap information:
    // println!("total memory: {} bytes", sys.total_memory());
    // println!("used memory : {} bytes", sys.used_memory());
    // println!("total swap  : {} bytes", sys.total_swap());
    // println!("used swap   : {} bytes", sys.used_swap());
    // 
    // // Display system information:
    // println!("System name:             {:?}", System::name().unwrap());
    // println!("kernel version:          {:?}", System::kernel_long_version());
    // println!("OS version:              {:?}", System::long_os_version().unwrap());
    // println!("CPU Architecture:        {:?}", System::cpu_arch());
    // 
    // 
    // // Number of CPUs:
    // println!("Physical CPUs Core :  {:?}" ,System::physical_core_count().unwrap());
    // println!("CPUs: {:?}", sys.cpus().len());
    
    let mut info_list: Vec<SystemInfo> = Vec::new();
    info_list.push(
        SystemInfo{
            title: "System name:".to_string(),
            info: System::name().unwrap().to_string(),
        }
    );
    info_list.push(
        SystemInfo{
            title: "kernel version:".to_string(),
            info: System::kernel_long_version().to_string(),
        }
    );
    info_list.push(
        SystemInfo{
            title: "OS version:".to_string(),
            info: System::long_os_version().unwrap().to_string(),
        }
    );
    info_list.push(
        SystemInfo{
            title: "CPU Architecture:".to_string(),
            info: System::cpu_arch().to_string(),
        }
    );
    info_list.push(
        SystemInfo{
            title: "Physical CPUs Core :".to_string(),
            info: System::physical_core_count().unwrap().to_string(),
        }
    );
    info_list.push(
        SystemInfo{
            title: "CPUs:".to_string(),
            info: sys.cpus().len().to_string(),
        }
    );

    info_list
}
