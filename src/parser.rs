mod tree;
mod util;

use crate::LabelManager;

pub enum FdtTokenKind {
    BeginNode = 0x1,
    EndNode = 0x2,
    Prop = 0x3,
    Nop = 0x4,
    End = 0x9,
}

pub struct Token {
    pub kind: FdtTokenKind,
    name: String,
    data: Option<Vec<u32>>,
    label: Option<String>,
    child: Option<Vec<Token>>,
}

impl Token {
    pub fn from_kind(kind: FdtTokenKind) -> Self {
        Token {
            kind,
            name: String::new(),
            data: None,
            label: None,
            child: None,
        }
    }
}

pub fn make_tree(dts: String, label_mgr: &mut LabelManager) -> Token {
    let mut lines = dts.lines().peekable();

    if lines.next() != Some("/dts-v1/;") {
        panic!("version isn't specified");
    }

    tree::parse_node(&mut lines, label_mgr)
}
