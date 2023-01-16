mod memmap;
mod parser;

use clap::{arg, AppSettings};
use std::collections::HashMap;
use std::fs;

pub struct LabelManager {
    labels: HashMap<String, String>,
    phandles: HashMap<String, u32>,
    current_phandle: u32,
}

impl LabelManager {
    pub fn new() -> Self {
        LabelManager {
            labels: HashMap::new(),
            phandles: HashMap::new(),
            current_phandle: 0,
        }
    }

    pub fn regist_label(&mut self, label: String, data: String) {
        self.labels.insert(label, data);
    }

    pub fn regist_phandle(&mut self, label: &str) -> u32 {
        *self.phandles.entry(label.to_string()).or_insert_with(|| {
            self.current_phandle += 1;
            self.current_phandle
        })
    }
}

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

    let mut label_mgr: LabelManager = LabelManager::new();
    let tree = parser::make_tree(dts, &mut label_mgr);

    memmap::write_dtb(output_path)?;

    Ok(())
}
