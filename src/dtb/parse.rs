use std::iter::Peekable;
use super::util;
use super::{dtb_mmap, FdtNodeKind};

pub fn parse_data(data: &str, mmap: &mut dtb_mmap) -> (Vec<u32>, u32) {
    dbg!(data);
    let data_ch = &mut data.chars();
    match data_ch.next().unwrap() {
        '"' => {
            let str_data = format!("{}\0", data_ch
                .take_while(|c| *c != '"')
                .collect::<String>());
            let size = str_data.len() as u32;
            let bin = str_data
                .into_bytes()
                .chunks(4)
                .map(|bs| {
                    // &[u8] -> [u8; 4]
                    let mut s = [0; 4];
                    s[.. bs.len()].clone_from_slice(bs);
                    u32::from_be_bytes(s)
                })
                .collect();

            if data_ch.last() != Some(';') {
                panic!("{} <-- ';' expected.", data);
            }

            (bin, size)
        },
        '<' => {
            let bin = data_ch
                .take_while(|c| *c != '>')
                .collect::<String>()
                .split(' ')
                .map(|num| {
                    if let Some(hex) = num.strip_prefix("0x") {
                        u32::from_str_radix(hex, 16).expect("parsing hex error.")
                    } else {
                        num.parse::<u32>().unwrap_or_else(|_| {
                            mmap.labels
                                .get(num.trim_start_matches('&'))
                                .expect("label referencing error.")
                                .clone()
                        })
                    }
                })
                .collect::<Vec<u32>>();
            let size = bin.len() as u32 * 4;

            if data_ch.last() != Some(';') {
                panic!("{} <-- ';' expected.", data);
            }

            (bin, size)
        },
        _ => panic!("prop data is invalid"),
    }
}

pub fn parse_property(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let tokens = &mut util::tokenize(lines, "property is invalid").peekable();
    let prop_name = tokens.next().expect("prop name not found");

    mmap.write_nodekind(FdtNodeKind::PROP);
    if util::consume(tokens, "=") {
        let raw_data = tokens.collect::<Vec<_>>().join(" ");
        let (mut data_map, data_size) = parse_data(&raw_data, mmap);
        mmap.write_property(prop_name, &mut data_map, data_size);

        if prop_name == "#address-cells" {
            if let Some(addr_cells) = mmap.current_label.clone() {
                mmap.regist_label(addr_cells, data_map[0]);
            }
        }
    } else {
        // only property's name
        mmap.write_property(prop_name, &mut Vec::new(), 0);
    }
}

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    // expect node's name and "{"
    let tokens = &mut util::tokenize(lines, "node is invalid").peekable();

    let first = tokens.next().expect("node name not found");
    mmap.write_nodekind(FdtNodeKind::BEGIN_NODE);
    if util::consume(tokens, "{") {
        let node_name = first;
        mmap.write_nodename(node_name);
        mmap.current_label = None;
    } else {
        let node_name = tokens.next().expect("node name not found");
        mmap.write_nodename(node_name);
        mmap.current_label = Some(first.trim_end_matches(':').to_string());
        util::expect(tokens.next(), "{");
    }

    loop {
        parse_line(lines, mmap);

        if util::consume(lines, "};") {
            mmap.write_nodekind(FdtNodeKind::END_NODE);
            break;
        }
    }
}

pub fn parse_line(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    dbg!(&lines.peek());

    if !util::consume(lines, "") {
        if lines.peek().unwrap().chars().last() == Some('{') {
            parse_node(lines, mmap);
        } else {
            parse_property(lines, mmap);
        }
    }
}


