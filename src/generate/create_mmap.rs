use super::DtbMmap;
use crate::parser::{FdtTokenKind, Token};

pub fn create_mmap(tree: Token, mut mmap: DtbMmap) -> DtbMmap {
    match tree.kind {
        FdtTokenKind::BeginNode => {
            mmap.write_nodekind(FdtTokenKind::BeginNode);
            mmap.write_nodekind(FdtTokenKind::EndNode);
        }
        FdtTokenKind::Prop => {}
        _ => (),
    }

    mmap
}
