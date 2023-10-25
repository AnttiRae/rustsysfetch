mod unix;

#[macro_use] extern crate rust_embed;
#[macro_use] extern crate prettytable;

use prettytable::Table;
use prettytable::format;
use std::io::{Error};
use std::io::ErrorKind;
use rust_embed::EmbeddedFile;
use colored::*;

#[derive(RustEmbed)]
#[folder = "logos/"]
struct Asset;

fn open_asset(name: &str) -> Result<EmbeddedFile, Error> {
    Asset::get(name).ok_or(std::io::Error::new(ErrorKind::NotFound, "Asset not found"))
}


fn main() {

    let mut info_string = String::new();
    let mut logo_string = String::new();

    let sys_info = unix::get_info();

    for info in &sys_info {
        info_string.push_str(&*format!("{} \n", info));
    }


    let distro = unix::get_os_release();

    match open_asset(&distro) {
        Ok(logo) => {
            let logo_vec = logo.data.to_owned();
            let logo = String::from_utf8(Vec::from(logo_vec)).unwrap();
            logo_string.push_str(&logo);
        },
        Err(e) => {
            let logo = open_asset("penguin").unwrap();
            let logo_vec = logo.data.to_owned();
            let logo = String::from_utf8(Vec::from(logo_vec)).unwrap();
            logo_string.push_str(&logo);
        }
    }

    let colored_logo_string = logo_string
        .lines()
        .map(|line| format!("{}", line.blue()))
        .collect::<Vec<_>>()
        .join("\n");

    println!();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row![&colored_logo_string, &info_string]);
    table.printstd();
    println!();
}
