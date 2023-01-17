use super::DtbMmap;
use crate::parser::{FdtTokenKind, Token};

pub fn create_mmap(tree: &Token, mut mmap: DtbMmap) -> DtbMmap {
    match tree.kind {
        FdtTokenKind::BeginNode => {
            mmap.write_nodekind(FdtTokenKind::BeginNode);
            mmap.write_nodename(&tree.name);

            if let Some(children) = &tree.child {
                for tk in children.iter() {
                    mmap = create_mmap(tk, mmap);
                }
            }

            if let Some(phandle) = mmap.is_phandle_needed(&tree.name) {
                mmap.write_property("phandle", &mut vec![phandle], 4);
            }
            mmap.write_nodekind(FdtTokenKind::EndNode);
        }
        FdtTokenKind::Prop => match &tree.data {
            Some(data) => mmap.write_property(&tree.name, &data, data.len() as u32),
            None => {
                let label = tree.label.as_ref().unwrap();
                let data = mmap
                    .label_lookup(label)
                    .into_bytes()
                    .chunks(4)
                    .map(|bs| {
                        // &[u8] -> [u8; 4]
                        let mut s = [0; 4];
                        s[..bs.len()].clone_from_slice(bs);
                        u32::from_be_bytes(s)
                    })
                    .collect::<Vec<u32>>();
                mmap.write_property(&tree.name, &data, data.len() as u32);
            }
        },
        _ => (),
    }

    mmap
}
