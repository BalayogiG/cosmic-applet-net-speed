use std::fs;

const IGNORED_PREFIXES: &[&str] = &["lo", "docker", "veth", "br-", "virbr", "tun", "tap"];

/// Picks the "active" interface: up, not in the ignore list, with the highest
/// combined rx+tx byte count so far.
pub fn active_interface() -> Option<String> {
    let entries = fs::read_dir("/sys/class/net").ok()?;
    let mut best: Option<(String, u64)> = None;

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if IGNORED_PREFIXES.iter().any(|p| name.starts_with(p)) {
            continue;
        }

        let operstate = fs::read_to_string(format!("/sys/class/net/{name}/operstate"))
            .unwrap_or_default();
        if operstate.trim() != "up" {
            continue;
        }

        let Some((rx, tx)) = rx_tx_bytes(&name) else {
            continue;
        };
        let total = rx + tx;

        if best.as_ref().is_none_or(|(_, best_total)| total > *best_total) {
            best = Some((name, total));
        }
    }

    best.map(|(name, _)| name)
}

pub fn rx_tx_bytes(iface: &str) -> Option<(u64, u64)> {
    let rx = read_counter(&format!("/sys/class/net/{iface}/statistics/rx_bytes"))?;
    let tx = read_counter(&format!("/sys/class/net/{iface}/statistics/tx_bytes"))?;
    Some((rx, tx))
}

fn read_counter(path: &str) -> Option<u64> {
    fs::read_to_string(path).ok()?.trim().parse().ok()
}

pub fn format_speed(bytes_per_sec: u64) -> String {
    let kb = bytes_per_sec as f64 / 1024.0;
    if kb < 1024.0 {
        format!("{kb:.0} KB/s")
    } else {
        format!("{:.1} MB/s", kb / 1024.0)
    }
}
