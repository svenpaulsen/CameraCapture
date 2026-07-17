//! Print device identities per Windows backend and show how they match
//! across backends — the resolution a caller performs to open a device
//! stored as a DirectShow DevicePath through Media Foundation instead.
//!
//! On non-Windows platforms both calls enumerate the platform's single
//! backend, so the listings simply repeat.

use ccap::{Provider, Result};

/// Cross-backend id matching as documented on
/// `find_device_identities_for_backend`: lowercase, cut at the `#{` of
/// the trailing interface-class GUID.
fn normalize_interface_path(id: &str) -> String {
    let lower = id.to_ascii_lowercase();
    match lower.find("#{") {
        Some(pos) => lower[..pos].to_string(),
        None => lower,
    }
}

fn main() -> Result<()> {
    Provider::set_error_callback(|error_code, description| {
        eprintln!(
            "Camera Error - Code: {}, Description: {}",
            error_code, description
        );
    });

    let dshow = Provider::find_device_identities_for_backend(Some("dshow"))?;
    let msmf = Provider::find_device_identities_for_backend(Some("msmf"))?;

    println!("## DirectShow ({} device(s)):", dshow.len());
    for (index, dev) in dshow.iter().enumerate() {
        println!("    {}: {} id={:?}", index, dev.name, dev.id);
    }

    println!("## Media Foundation ({} device(s)):", msmf.len());
    for (index, dev) in msmf.iter().enumerate() {
        println!("    {}: {} id={:?}", index, dev.name, dev.id);
    }

    println!("## Cross-backend matches (DShow DevicePath -> MSMF index):");
    for dev in &dshow {
        if dev.id.is_empty() {
            println!(
                "    {}: virtual cam (no DevicePath) -> DirectShow only",
                dev.name
            );
            continue;
        }
        let stem = normalize_interface_path(&dev.id);
        match msmf
            .iter()
            .position(|m| !m.id.is_empty() && normalize_interface_path(&m.id) == stem)
        {
            Some(index) => println!("    {}: -> MSMF index {}", dev.name, index),
            None => println!("    {}: -> no MSMF match (DirectShow only)", dev.name),
        }
    }

    Ok(())
}
