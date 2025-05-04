use crate::collectors::*;

impl ClientInfo {
    pub fn display(&self) {
        display_system_info(&self.system);
        display_cpu_info(&self.cpu);
        display_ram_info(&self.ram);
        display_security_info(&self.security);
        display_gpu_info(&self.gpus);
        display_unique_info(&self.unique);
        display_physical_drives(&self.drives);
        display_bios_info(self.bios.as_ref());
    }
}

pub fn display_system_info(info: &SystemInfo) {
    println!("COMPUTER - OS");
    println!("🧑 Username              {}", info.username);
    println!("🖥️  Machine Name         {}", info.machine_name);
    println!("💻 System Model          {}", info.system_model.as_deref().unwrap_or("Unknown"));
    println!("🏢 System Manufacturer   {}", info.system_manufacturer.as_deref().unwrap_or("Unknown"));
    println!("🪟 OS Full Name          {}", info.os_full_name.as_deref().unwrap_or("Unknown"));
    println!("📦 OS Version            {}", info.os_version.as_deref().unwrap_or("Unknown"));
    println!("🔑 OS Serial Number      {}", info.os_serial_number.as_deref().unwrap_or("Unknown"));
}

pub fn display_cpu_info(info: &CpuInfo) {
    println!("CPU");
    println!("🧠 CPU                   {}", info.cpu_name);
    println!("🧮 N. Processor/s        {}", info.logical_processors);
    println!("🏷️  Processor Family      {}", info.processor_family.as_deref().unwrap_or("Unknown"));
    println!("⚡ Manufacturer          {}", info.manufacturer.as_deref().unwrap_or("Unknown"));
    println!("⏱️  Current Clock Speed   {} MHZ", info.clock_speed_mhz);
    println!("📝 Description           {}", info.description.as_deref().unwrap_or("Unknown"));
}

pub fn display_ram_info(info: &RamInfo) {
    println!("RAM");
    println!("💾 Total RAM            {:.2} GB", info.total_gb);
    println!("📊 Used RAM             {:.2} GB", info.used_gb);
}

pub fn display_security_info(info: &SecurityInfo) {
    println!("SECURITY");
    println!("🧱 Firewall             {}", if info.firewall_enabled { "Enabled" } else { "Disabled" });
    if info.antivirus_names.is_empty() {
        println!("🛡️  Antivirus            None detected");
    } else {
        for av in &info.antivirus_names {
            println!("🛡️  Antivirus            {}", av);
        }
    }
}

pub fn display_gpu_info(gpus: &[GpuInfo]) {
    println!("SCREEN");
    if gpus.is_empty() {
        println!("❌ No GPU information available");
        return;
    }
    for (i, gpu) in gpus.iter().enumerate() {
        println!("🎮 Video Card {}         {}", i + 1, gpu.name);
        println!("🧾 Driver Version        {}", gpu.driver_version.as_deref().unwrap_or("Unknown"));
    }
}

pub fn display_physical_drives(drives: &[PhysicalDrive]) {
    println!("PHYSICAL DRIVES");
    if drives.is_empty() {
        println!("❌ No physical drives found");
        return;
    }
    for (i, drive) in drives.iter().enumerate() {
        println!("🖴 Drive {}               {}", i + 1, drive.model);
        println!("📦 Size                  {:.2} GB", drive.size_gb);
    }
}

pub fn display_bios_info(info: Option<&BiosInfo>) {
    match info {
        Some(bios) => {
            println!("BIOS");
            println!("🧬 Bios Manufacturer     {}", bios.manufacturer);
            println!("🧾 Bios Description      {}", bios.description);
            println!("🔑 Bios Serial Number    {}", bios.serial_number);
            println!("🧮 Bios Version          {}", bios.version);
        }
        None => {
            println!("❌ BIOS info not available");
        }
    }
}

pub fn display_unique_info(info: &UniqueInfo) {
    println!("UNIQUE");
    println!("🌐 Mac Address          {}", info.mac_address);
    println!("💽 Volume Serial        {}", info.volume_serial);
}
