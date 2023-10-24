use std::{env, f64, fs};
use std::fs::File;
use std::io::Read;
use regex::Regex;
use colored::*;
use colored::Styles::Strikethrough;


fn build_lines_row(length: usize) -> String {
    let mut lines = String::new();
    for _ in 0..length {
        lines.push_str("-")
    }
    lines
}

pub fn get_info() -> Vec<String>{
    let mut info = Vec::new();

    let mut host_user_string = format!("{}@{}", get_hostname(), get_username());
    let length = host_user_string.chars().count();

    info.push(host_user_string.blue().to_string());
    info.push(build_lines_row(length));
    info.push(format!("{} {}", "OS:".blue().to_string(), get_os_release()));
    info.push(format!("{} {}", "Kernel:".blue().to_string(), get_kernel_version()));
    info.push(format!("{} {}", "Desktop:".blue().to_string(), get_desktop()));

    info.push(format!("{} {}", "Uptime:".blue().to_string(), get_uptime()));
    info.push(format!("{} {}", "Board:".blue().to_string(), get_host_board()));
    info.push(format!("{} {}", "Memory:".blue().to_string(), get_memory()));
    info.push(format!("{} {}", "CPU:".blue().to_string(), get_cpu()));
    info.push(format!("{} {}", "Shell:".blue().to_string(), get_shell()));

    info
}

fn get_hostname() -> String {
    let file_path = "/proc/sys/kernel/hostname";

    let contents = fs::read_to_string(file_path)
        .unwrap_or_else(|_| String::new());
    contents.trim_end_matches('\n').to_string()
}

fn get_username() -> String {
    let mut username = String::new();

    match env::var("USER") {
        Ok(mut desktop_environment) => {
            username.push_str(&*desktop_environment);
        }
        Err(e) => {
            username.push_str("No Info")
        }
    }

    username
}

pub fn get_os_release() -> String {
    let file_path = "/etc/os-release";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r#"ID=(\w+)"#).unwrap();
    let distro_id = re.captures(&contents).unwrap().get(1).map_or("", |m| m.as_str());

    distro_id.to_string()
}

fn format_uptime(seconds: f64) -> String {
    let minutes = (seconds / 60.0).round();
    let hours = (minutes / 60.0).round();
    let days = (hours / 24.0).round();

    if days == 0.0 {
        format!("{} hours, {} minutes", hours, minutes % 60.0)
    } else {
        format!("{} days, {} hours, {} minutes", days, hours % 24.0, minutes % 60.0)
    }
}

fn get_uptime() -> String {
    let file_path = "/proc/uptime";
    let mut uptime: String = String::new();
    let mut uptime_seconds: f64 = 0.0;

    match File::open(file_path) {
        Ok(mut file) => {
            file.read_to_string(&mut uptime).unwrap();
            let uptime_string = uptime.split(".").next().unwrap();
            uptime_seconds = uptime_string.parse().unwrap();
        }
        Err(e) => {
            uptime.push_str("0");
        }
    }

    let uptime= format_uptime(uptime_seconds);

    uptime
}

/// Converts KB to MB
fn kb_to_mb(memory_kb: i32) -> f64{
    (memory_kb as f64) / 1024.0
}

fn extract_memory_from_content(pattern: &str, content: &str) -> Option<i32>{
    let re = Regex::new(pattern).unwrap();
    match re.captures(content) {
        Some(cap) => {
            cap[1].parse().ok()
        },
        None => None
    }
}

fn get_memory() -> String {
    let file_path = "/proc/meminfo";
    let file_content = fs::read_to_string(file_path).unwrap_or_default();

    let mem_total = extract_memory_from_content(r"MemTotal:\s+(\d+)", &file_content)
                    .unwrap_or(0);
    let mem_available = extract_memory_from_content(r"MemAvailable:\s+(\d+)", &file_content)
                        .unwrap_or(0);

    let memory = format!(
        "{:.0}MB / {:.0}MB",
        kb_to_mb(mem_total - mem_available),
        kb_to_mb(mem_total)
    );

    memory
}

fn get_shell() -> String {
    let mut shell = String::new();

    match env::var("SHELL") {
        Ok(mut desktop_environment) => {
            shell.push_str(&*desktop_environment);
        }
        Err(e) => {
            shell.push_str("No Info")
        }
    }

    shell
}

fn get_desktop() -> String {
    let mut desktop = String::new();


    match env::var("XDG_CURRENT_DESKTOP") {
        Ok(mut desktop_environment) => {
            desktop.push_str(&*desktop_environment);
        }
        Err(e) => {
            desktop.push_str("No Info")
        }
    }

    desktop
}

fn extract_line_from_text(pattern: &str, content: &str) -> Option<String>{
    let re = Regex::new(pattern).unwrap();
    match re.captures(content) {
        Some(cap) => {
            cap[1].parse().ok()
        },
        None => None
    }
}

fn get_cpu() -> String {
    let file_path = "/proc/cpuinfo";

    let file_content = fs::read_to_string(file_path).unwrap_or_default();

    let cpu_info_string = extract_line_from_text(r"model name\s*:\s*(.*?@\s*\d+(\.\d+)?GHz)", &file_content)
                    .unwrap_or("No info".to_string());

    cpu_info_string
}

fn get_host_board() -> String {
    let file_path = "/sys/devices/virtual/dmi/id/board_name";
    let mut boardname = String::new();

    match File::open(file_path) {
        Ok(mut file) => {
            file.read_to_string(&mut boardname).unwrap();
        }
        Err(e) => {
            boardname.push_str("0");
        }
    }
    boardname.trim_end_matches('\n').to_string()
}

fn get_kernel_version() -> String {
    let file_path = "/proc/version";

    let file_content = fs::read_to_string(file_path).unwrap_or_default();

    let cpu_info_string = extract_line_from_text(r"^Linux version ([\d\.]+-\d+\.\w+\.\w+)", &file_content)
        .unwrap_or("No info".to_string());

    cpu_info_string
}

