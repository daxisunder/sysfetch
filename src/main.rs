use colored::*;
use sysinfo::{Disks, System};

// ── Distro detection ────────────────────────────────────────────────────────

fn read_os_id() -> String {
    std::fs::read_to_string("/etc/os-release")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("ID="))
        .map(|l| l.trim_start_matches("ID=").trim_matches('"').to_lowercase())
        .unwrap_or_else(|| "linux".into())
}

// ── ASCII logos ─────────────────────────────────────────────────────────────

fn get_logo(distro: &str) -> (Vec<&'static str>, Color) {
    match distro {
        "arch" | "archlinux" => (
            vec![
                "      /\\      ",
                "     /  \\     ",
                "    /\\   \\    ",
                "   /  __  \\   ",
                "  /  (  )  \\  ",
                " / __|  |__ \\ ",
                "/_-''    ''-_\\",
            ],
            Color::Cyan,
        ),

        "debian" => (
            vec![
                "  _____  ",
                " /  __ \\ ",
                "|  /    |",
                "|  \\___- ",
                "-_        ",
                "  --_     ",
            ],
            Color::Red,
        ),

        "ubuntu" => (
            vec![
                "          _    ",
                "      ---(_)   ",
                "  _/  ---  \\   ",
                " (_) |   |     ",
                "   \\  --- _/   ",
                "      ---(_)   ",
            ],
            Color::BrightRed,
        ),

        "fedora" => (
            vec![
                "      _____   ",
                "     /   __\\ ",
                "     |  /  / ",
                "  ___|  / /_ ",
                " / __   ___ \\",
                " \\___/  |   /",
                "     |__|\\_ \\",
                "        __\\ \\",
            ],
            Color::Blue,
        ),

        "nixos" => (
            vec![
                "  \\\\  \\\\ //  ",
                " ==\\\\=\\\\/ /  ",
                "   //   \\\\   ",
                "  //     \\\\  ",
                " //   /\\  \\\\ ",
                "//   /  \\  \\\\",
                "\\\\  /    \\  /",
            ],
            Color::BrightBlue,
        ),

        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => (
            vec![
                "  _______   ",
                "__|   __ \\  ",
                "     / .\\ \\ ",
                "     \\__/ | ",
                "   _____/ / ",
                "   \\______/ ",
            ],
            Color::Green,
        ),

        "gentoo" => (
            vec![
                "  _-----_  ",
                " (       \\ ",
                " \\    0   \\",
                "  \\        )",
                "  /      _/ ",
                " (     _-   ",
                " \\____-     ",
            ],
            Color::BrightMagenta,
        ),

        "manjaro" => (
            vec![
                "||||||||| || ",
                "||||||||| || ",
                "||||      || ",
                "||||  ||| || ",
                "||||  ||| || ",
                "||||  ||| || ",
            ],
            Color::Green,
        ),

        "endeavouros" => (
            vec![
                "      /\\      ",
                "     /  \\     ",
                "    / /\\ \\    ",
                "   / / /\\ \\   ",
                "  / / /  \\ \\  ",
                " /_/ /    \\_\\ ",
            ],
            Color::BrightRed,
        ),

        _ => (
            vec![
                "    .--.    ",
                "   |o_o |   ",
                "   |:_/ |   ",
                "  //   \\ \\  ",
                " (|     | ) ",
                "/'\\_   _/`\\ ",
                "\\___)=(___/ ",
            ],
            Color::White,
        ),
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn bytes_to_gib(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

fn format_uptime(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{}h {}m {}s", hours, minutes, secs)
}

fn make_info_lines(sys: &System) -> Vec<String> {
    let user = std::env::var("USER").unwrap_or("user".into());
    let hostname = System::host_name().unwrap_or("unknown".into());
    let os_name = System::name().unwrap_or("unknown".into());
    let os_version = System::os_version().unwrap_or("unknown".into());
    let kernel = System::kernel_version().unwrap_or("unknown".into());
    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or("unknown".into());
    let cpu_count = sys.cpus().len();
    let cpu_usage = sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / cpu_count as f32;
    let total_ram = bytes_to_gib(sys.total_memory());
    let used_ram = bytes_to_gib(sys.used_memory());
    let uptime = format_uptime(System::uptime());

    let sep = "─".repeat(28);

    let mut lines = vec![
        format!(
            "{}@{}",
            user.bright_green().bold(),
            hostname.bright_green().bold()
        ),
        sep.bright_black().to_string(),
        format!(
            "{} {}",
            "OS     ".cyan().bold(),
            format!("{} {}", os_name, os_version)
        ),
        format!("{} {}", "Kernel ".cyan().bold(), kernel),
        format!("{} {}", "Uptime ".cyan().bold(), uptime),
        format!(
            "{} {}",
            "CPU    ".cyan().bold(),
            format!("{} ({} cores)", cpu_name, cpu_count)
        ),
        format!(
            "{} {}",
            "Usage  ".cyan().bold(),
            format!("{:.1}%", cpu_usage)
        ),
        format!(
            "{} {}",
            "RAM    ".cyan().bold(),
            format!("{:.2} / {:.2} GiB", used_ram, total_ram)
        ),
        String::new(),
        "Disks:".yellow().bold().to_string(),
    ];

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total = bytes_to_gib(disk.total_space());
        let used = total - bytes_to_gib(disk.available_space());
        let mount = disk.mount_point().to_string_lossy();
        lines.push(format!(
            "  {} {}  {:.1}/{:.1} GiB",
            "▸".bright_black(),
            mount.cyan(),
            used,
            total
        ));
    }

    lines.push(String::new());

    // Color palette
    let palette = format!(
        "{}{}{}{}{}{}{}{}",
        "███".red(),
        "███".yellow(),
        "███".green(),
        "███".cyan(),
        "███".blue(),
        "███".magenta(),
        "███".white(),
        "███".bright_black()
    );
    lines.push(palette);

    lines
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();

    let distro_id = read_os_id();
    let (logo_lines, logo_color) = get_logo(&distro_id);
    let info_lines = make_info_lines(&sys);

    println!();

    let max_rows = logo_lines.len().max(info_lines.len());
    for i in 0..max_rows {
        let logo_col = logo_lines
            .get(i)
            .map(|l| format!("{}", l.color(logo_color).bold()))
            .unwrap_or_else(|| " ".repeat(14));

        let info_col = info_lines.get(i).cloned().unwrap_or_default();

        println!("  {}   {}", logo_col, info_col);
    }

    println!();
}
