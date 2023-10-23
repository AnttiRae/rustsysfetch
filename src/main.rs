mod unix;

#[macro_use] extern crate prettytable;

use std::fs;
use prettytable::Table;
use prettytable::format;

fn get_distro_logo(file_path: &str) -> String {

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}

fn main() {

    let mut info_string = String::new();
    let mut logo_string = String::new();

    let test_info = unix::get_info();

    for info in &test_info {
        info_string.push_str(&*format!("{} \n", info));
    }


    let logo = get_distro_logo("logos/fedora.txt");
    logo_string.push_str(&logo);

    println!();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row![&logo_string, &info_string]);
    table.printstd();
    println!();
}
