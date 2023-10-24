use std::fs;
use std::fs::File;
use std::io::Read;
use regex::Regex;

pub fn get_info() -> Vec<String>{
    let mut info = Vec::new();

    info.push(format!("Username: {}", get_username()));
    info.push(format!("Distro: {}", get_os_release()));
    info.push(format!("Desktop: {}", get_desktop()));
    info.push(format!("Hostname: {}", get_hostname()));
    info.push(format!("Uptime: {}", get_uptime()));
    info.push(format!("Memory: {}", get_memory()));
    info.push(format!("Shell: {}", get_shell()));

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
    username.push_str("username");

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

    format!("{} days, {} hours, {} minutes", days, hours % 24.0, minutes % 60.0)
}

fn get_uptime() -> String {
    let file_path = "/proc/uptime";
    let mut uptime: String = String::new();

    match File::open(file_path) {
        Ok(mut file) => {
            file.read_to_string(&mut uptime).unwrap();
            let uptime_seconds = uptime.split(".").next().unwrap();
            uptime = String::from(uptime_seconds);

        }
        Err(e) => {
            uptime.push_str("0");
        }
    }
    let uptime_as_f64: f64 = uptime.parse().expect("Failed to convert uptime to f64");
    let uptime: String = String::from("1234.56"); // your string

    let uptime_as_f64: f64 = uptime.parse().expect("Failed to convert uptime to f64");

    println!("{}", uptime_as_f64);
    uptime
}

fn get_memory() -> String {
    let mut memory = String::new();
    memory.push_str("memory");

    memory
}

fn get_shell() -> String {
    let mut shell = String::new();
    shell.push_str("shell");

    shell
}

fn get_desktop() -> String {
    let mut desktop = String::new();
    desktop.push_str("desktop");

    desktop
}