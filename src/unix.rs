use std::fs;
use regex::Regex;

pub fn get_info() -> Vec<String>{
    let mut info = Vec::new();

    info.push(get_username());
    info.push(get_desktop());
    info.push(get_hostname());
    info.push(get_uptime());
    info.push(get_memory());
    info.push(get_os_release());
    info.push(get_shell());

    info
}

fn get_hostname() -> String {
    let mut hostname = String::new();
    hostname.push_str("hostname");

    hostname
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

fn get_uptime() -> String {
    let mut uptime = String::new();
    uptime.push_str("uptime");

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