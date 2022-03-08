use clap::{AppSettings, arg};

fn main() {
    let app = clap::command!()
        .arg(arg!(<inputfile> ... "dts file filename"))
        .arg(arg!(-o <outputfile> ... "dtb file name").required(false))
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();

    dbg!(app);
}
