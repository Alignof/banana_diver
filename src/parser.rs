pub enum FdtNodeKind {
    BeginNode = 0x1,
    EndNode = 0x2,
    Prop = 0x3,
    Nop = 0x4,
    End = 0x9,
}

pub fn make_tree(dts: String) -> dtb_mmap {
    let mut mmap: dtb_mmap = dtb_mmap {
        reserve: vec![0x0, 0x0],
        structure: Vec::new(),
        strings: Strings::new(),
        labels: HashMap::new(),
        current_label: None,
    };
    let mut lines = dts.lines().peekable();

    if lines.next() != Some("/dts-v1/;") {
        panic!("version isn't specified");
    }

    while lines.peek().is_some() {
        parse::parse_line(&mut lines, &mut mmap);
    }
    mmap.write_nodekind(FdtNodeKind::END);

    mmap
}
