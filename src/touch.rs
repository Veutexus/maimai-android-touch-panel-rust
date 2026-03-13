use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::serial_manager::SerialManager;
use crate::zone::ZoneLookup;

struct TouchSlot {
    pressed: bool,
    x: i32,
    y: i32,
}

impl Default for TouchSlot {
    fn default() -> Self {
        Self {
            pressed: false,
            x: 0,
            y: 0,
        }
    }
}

/// Spawns `adb shell getevent -l` and parses multi-touch Type B events.
/// Converts touch positions to zones and sends updates to the serial manager.
pub fn run_getevent(
    serial_manager: &SerialManager,
    zone_lookup: &ZoneLookup,
    max_slot: usize,
    monitor_size: [u32; 2],
    input_size: [u32; 2],
    reverse_monitor: Arc<AtomicBool>,
    specified_device: &str,
) {
    let abs_multi_x = monitor_size[0] as f64 / input_size[0] as f64;
    let abs_multi_y = monitor_size[1] as f64 / input_size[1] as f64;

    let mut touch_data: Vec<TouchSlot> = (0..max_slot).map(|_| TouchSlot::default()).collect();
    let mut touch_index: usize = 0;
    let mut key_is_changed = false;

    let mut adb_args: Vec<&str> = vec![];
    if !specified_device.is_empty() {
        adb_args.extend_from_slice(&["-s", specified_device]);
    }
    adb_args.extend_from_slice(&["shell", "getevent", "-l"]);

    let process = Command::new("adb")
        .args(&adb_args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn();

    let mut child = match process {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to start adb: {}", e);
            return;
        }
    };

    let stdout = match child.stdout.take() {
        Some(s) => s,
        None => {
            eprintln!("Failed to capture adb stdout");
            return;
        }
    };

    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let event_type = parts[2];
        let event_value_hex = parts[3];

        let event_value = match i64::from_str_radix(event_value_hex, 16) {
            Ok(v) => v,
            Err(_) => continue,
        };

        match event_type {
            "ABS_MT_POSITION_X" => {
                key_is_changed = true;
                let reverse = reverse_monitor.load(Ordering::Relaxed);
                if touch_index < touch_data.len() {
                    touch_data[touch_index].x = if reverse {
                        monitor_size[0] as i32 - (event_value as f64 * abs_multi_x) as i32
                    } else {
                        (event_value as f64 * abs_multi_x) as i32
                    };
                }
            }
            "ABS_MT_POSITION_Y" => {
                key_is_changed = true;
                let reverse = reverse_monitor.load(Ordering::Relaxed);
                if touch_index < touch_data.len() {
                    touch_data[touch_index].y = if reverse {
                        monitor_size[1] as i32 - (event_value as f64 * abs_multi_y) as i32
                    } else {
                        (event_value as f64 * abs_multi_y) as i32
                    };
                }
            }
            "SYN_REPORT" => {
                if key_is_changed {
                    convert(&touch_data, serial_manager, zone_lookup);
                    key_is_changed = false;
                }
            }
            "ABS_MT_SLOT" => {
                key_is_changed = true;
                touch_index = event_value as usize;
            }
            "ABS_MT_TRACKING_ID" => {
                key_is_changed = true;
                if touch_index < touch_data.len() {
                    if event_value_hex == "ffffffff" {
                        touch_data[touch_index].pressed = false;
                    } else {
                        touch_data[touch_index].pressed = true;
                    }
                }
            }
            _ => {}
        }
    }
}

/// Maps current touch positions to zones and sends the packet to serial manager.
fn convert(touch_data: &[TouchSlot], serial_manager: &SerialManager, zone_lookup: &ZoneLookup) {
    let mut all_zones = std::collections::HashSet::new();

    for slot in touch_data {
        if !slot.pressed {
            continue;
        }
        let zones = zone_lookup.lookup_zones(slot.x, slot.y);
        all_zones.extend(zones);
    }

    let touch_keys: Vec<String> = all_zones.iter().cloned().collect();
    let grid = ZoneLookup::zones_to_grid(&all_zones);
    serial_manager.change_touch(&grid, touch_keys);
}
