mod dtb;

use clap::{arg, AppSettings};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap::command!()
        .arg(arg!(<inputfile> ... "dts file name"))
        .arg(arg!(-o <outputfile> ... "dtb file name").required(false))
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();

    let input_path = match app.value_of("inputfile") {
        Some(f) => f.to_string(),
        None => panic!("please specify target file."),
    };
    let output_path = app.value_of("outputfile");

    let dts = fs::read_to_string(input_path)
        .expect("opening file failed.")
        .replace("  ", "");
    let dtb = dtb::make_dtb(dts);

    dtb::write_dtb(dtb, output_path)?;

    Ok(())
}
