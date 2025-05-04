// src/collectors/gpu.rs
use std::collections::HashMap;
use wmi::{COMLibrary, Variant, WMIConnection};
use tokio::task;

#[derive(Debug)]
pub struct GpuInfo {
    pub name: String,
    pub driver_version: Option<String>,
}

pub async fn collect_gpu_info() -> Vec<GpuInfo> {
    task::spawn_blocking(|| {
        let com = match COMLibrary::new() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ COM init failed: {:?}", e);
                return Vec::new();
            }
        };

        let wmi = match WMIConnection::with_namespace_path("root\\cimv2", com) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("❌ WMI connection failed: {:?}", e);
                return Vec::new();
            }
        };

        let results: Vec<HashMap<String, Variant>> = match wmi.raw_query("SELECT Name, DriverVersion FROM Win32_VideoController") {
            Ok(r) => r,
            Err(e) => {
                eprintln!("❌ WMI GPU query failed: {:?}", e);
                return Vec::new();
            }
        };

        results
            .into_iter()
            .map(|mut gpu| {
                let name = gpu
                    .remove("Name")
                    .and_then(|v| v.try_into().ok())
                    .unwrap_or_else(|| "Unknown".to_string());

                let driver_version = gpu
                    .remove("DriverVersion")
                    .and_then(|v| v.try_into().ok());

                GpuInfo {
                    name,
                    driver_version,
                }
            })
            .collect()
    }).await.unwrap_or_default()
}