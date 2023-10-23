mod unix;

#[macro_use] extern crate rust_embed;
#[macro_use] extern crate prettytable;

use prettytable::Table;
use prettytable::format;

#[derive(RustEmbed)]
#[folder = "logos/"]
struct Asset;

fn get_distro_logo(distro: &str) -> Result<String, Box<dyn std::error::Error>> {
    let logo_file = Asset::get(distro).ok_or("Logo asset not found")?;
    let logo_vec = logo_file.data.to_owned();
    let logo = String::from_utf8(Vec::from(logo_vec))?;
    Ok(logo)
}


fn main() {

    let mut info_string = String::new();
    let mut logo_string = String::new();

    let test_info = unix::get_info();

    let test = unix::get_os_release();

    for info in &test_info {
        info_string.push_str(&*format!("{} \n", info));
    }


   match get_distro_logo("ubuntu") {
        Ok(logo) => {
            logo_string.push_str(&logo);
        },
        Err(e) => println!("An error occurred: {}", e),
    }

    println!();
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row![&logo_string, &info_string]);
    table.printstd();
    println!();
}
