mod dtb;

use std::fs;
use clap::{AppSettings, arg};

fn main() {
    let app = clap::command!()
        .arg(arg!(<inputfile> ... "dts file filename"))
        .arg(arg!(-o <outputfile> ... "dtb file name").required(false))
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();

    let input_path = match app.value_of("inputfile") {
        Some(f) => f.to_string(),
        None => panic!("please specify target ELF file."),
    };

    let dts = fs::read_to_string(input_path)
        .expect("opening file failed.")
        .replace("  ", "");
    let dtb = dtb::make_dtb(dts);

    dbg!(dtb);
}
