// src/collectors/bios.rs
use std::collections::HashMap;
use wmi::{COMLibrary, Variant, WMIConnection};
use tokio::task;

#[derive(Debug)]
pub struct BiosInfo {
    pub manufacturer: String,
    pub description: String,
    pub serial_number: String,
    pub version: String,
}

pub async fn collect_bios_info() -> Option<BiosInfo> {
    task::spawn_blocking(|| {
        let com = COMLibrary::new().ok()?;
        let wmi = WMIConnection::with_namespace_path("root\\cimv2", com).ok()?;

        let results: Vec<HashMap<String, Variant>> = wmi
            .raw_query("SELECT Manufacturer, Description, SerialNumber, SMBIOSBIOSVersion FROM Win32_BIOS")
            .ok()?;

        let first = results.into_iter().next()?;

        Some(BiosInfo {
            manufacturer: first.get("Manufacturer").and_then(|v| v.clone().try_into().ok()).unwrap_or_else(|| "Unknown".to_string()),
            description: first.get("Description").and_then(|v| v.clone().try_into().ok()).unwrap_or_else(|| "Unknown".to_string()),
            serial_number: first.get("SerialNumber").and_then(|v| v.clone().try_into().ok()).unwrap_or_else(|| "Unknown".to_string()),
            version: first.get("SMBIOSBIOSVersion").and_then(|v| v.clone().try_into().ok()).unwrap_or_else(|| "Unknown".to_string()),
        })
    }).await.ok().flatten()
}
