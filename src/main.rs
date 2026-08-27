use std::thread;
use sysinfo::{
    CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, MemoryRefreshKind, RefreshKind, System,
};

const BG: &str = "#313244";
const BG_END: &str = "#1e1e2e";
const FG_DARK: &str = "#11111b";
const GREEN: &str = "#a6e3a1";
const YELLOW: &str = "#f9e2af";
const RED: &str = "#f38ba8";

fn main() {
    match collect_metrics() {
        Ok(metrics) => print!("{}", metrics.render()),
        Err(_) => print!(""),
    }
}

fn collect_metrics() -> Result<Metrics, String> {
    let mut system = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    system.refresh_cpu_usage();
    thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    system.refresh_cpu_usage();
    system.refresh_memory();

    let total_memory = system.total_memory();
    if total_memory == 0 {
        return Err("total memory is zero".to_string());
    }

    let cpu_percent = system.global_cpu_usage().round() as u8;
    let ram_percent = ((system.used_memory() as f64 / total_memory as f64) * 100.0).round() as u8;

    Ok(Metrics {
        cpu_percent,
        ram_percent,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Metrics {
    cpu_percent: u8,
    ram_percent: u8,
}

impl Metrics {
    fn render(self) -> String {
        format!(
            "{}{}",
            render_pill("CPU", self.cpu_percent, cpu_color(self.cpu_percent)),
            render_pill("RAM", self.ram_percent, ram_color(self.ram_percent)),
        )
    }
}

fn render_pill(label: &str, value: u8, color: &str) -> String {
    format!(
        "#[bg={BG},fg={color}]#[bg={color},fg={FG_DARK},bold]{label} {value}%#[bg={BG_END},fg={color}] "
    )
}

fn cpu_color(value: u8) -> &'static str {
    match value {
        0..=39 => GREEN,
        40..=74 => YELLOW,
        _ => RED,
    }
}

fn ram_color(value: u8) -> &'static str {
    match value {
        0..=49 => GREEN,
        50..=79 => YELLOW,
        _ => RED,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_thresholds_match_expected_colors() {
        assert_eq!(cpu_color(0), GREEN);
        assert_eq!(cpu_color(39), GREEN);
        assert_eq!(cpu_color(40), YELLOW);
        assert_eq!(cpu_color(74), YELLOW);
        assert_eq!(cpu_color(75), RED);
        assert_eq!(cpu_color(100), RED);
    }

    #[test]
    fn ram_thresholds_match_expected_colors() {
        assert_eq!(ram_color(0), GREEN);
        assert_eq!(ram_color(49), GREEN);
        assert_eq!(ram_color(50), YELLOW);
        assert_eq!(ram_color(79), YELLOW);
        assert_eq!(ram_color(80), RED);
        assert_eq!(ram_color(100), RED);
    }

    #[test]
    fn render_includes_both_metrics() {
        let rendered = Metrics {
            cpu_percent: 12,
            ram_percent: 63,
        }
        .render();

        assert!(rendered.contains("CPU 12%"));
        assert!(rendered.contains("RAM 63%"));
        assert!(rendered.contains("#[bg=#313244,fg=#a6e3a1]"));
        assert!(rendered.contains("#[bg=#313244,fg=#f9e2af]"));
    }
}
