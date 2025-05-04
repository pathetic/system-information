pub mod bios;
pub mod cpu;
pub mod gpu;
pub mod ram;
pub mod system;
pub mod security;
pub mod drives;
pub mod unique;

use bios::collect_bios_info;
use cpu::collect_cpu_info;
use drives::collect_physical_drives;
use gpu::collect_gpu_info;
use ram::collect_ram_info;
use security::collect_security_info;
use system::collect_system_info;
use unique::collect_unique_info;

pub use bios::BiosInfo;
pub use cpu::CpuInfo;
pub use drives::PhysicalDrive;
pub use gpu::GpuInfo;
pub use ram::RamInfo;
pub use security::SecurityInfo;
pub use system::SystemInfo;
pub use unique::UniqueInfo;

#[derive(Debug)]
pub struct ClientInfo {
    pub system: SystemInfo,
    pub cpu: CpuInfo,
    pub ram: RamInfo,
    pub security: SecurityInfo,
    pub bios: Option<BiosInfo>,
    pub gpus: Vec<GpuInfo>,
    pub drives: Vec<PhysicalDrive>,
    pub unique: UniqueInfo,
}

impl ClientInfo {
    pub async fn gather() -> Self {
        ClientInfo {
            system: collect_system_info().await,
            cpu: collect_cpu_info().await,
            ram: collect_ram_info().await,
            security: collect_security_info().await,
            bios: collect_bios_info().await,
            gpus: collect_gpu_info().await,
            drives: collect_physical_drives().await,
            unique: collect_unique_info().await,
        }
    }
}