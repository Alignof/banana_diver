use std::fs::File;
use std::io::Write;

use super::{DtbMmap, FdtHeader};

pub fn write_dtb(path: Option<&str>, mmap: DtbMmap) -> Result<(), Box<dyn std::error::Error>> {
    let reserve = mmap
        .reserve
        .iter()
        .flat_map(|x| x.to_be_bytes())
        .collect::<Vec<u8>>();

    let structure = mmap
        .structure
        .iter()
        .flat_map(|x| x.to_be_bytes())
        .collect::<Vec<u8>>();

    let mut str_table = mmap.strings.table.iter().collect::<Vec<(&String, &u32)>>();
    str_table.sort_by(|a, b| a.1.cmp(b.1));
    let strings = str_table
        .iter()
        .cloned()
        .flat_map(|(k, _v)| k.clone().into_bytes())
        .collect::<Vec<u8>>();

    let size_dt_header = 0x28;
    let size_dt_reserve = reserve.len() as u32;
    let size_dt_strings = strings.len() as u32;
    let size_dt_struct = structure.len() as u32;
    let totalsize = size_dt_header + size_dt_reserve + size_dt_strings + size_dt_struct;

    let header = FdtHeader {
        magic: 0xd00dfeed,
        totalsize,
        off_dt_struct: size_dt_header + size_dt_reserve,
        off_dt_strings: size_dt_header + size_dt_reserve + size_dt_struct,
        off_mem_rsvmap: size_dt_header,
        version: 17,
        last_comp_version: 16,
        boot_cpuid_phys: 0,
        size_dt_strings,
        size_dt_struct,
    };

    let output_path = path.unwrap_or("./output.dtb");
    let mut file = File::create(output_path)?;

    file.write_all(&header.magic.to_be_bytes())?;
    file.write_all(&header.totalsize.to_be_bytes())?;
    file.write_all(&header.off_dt_struct.to_be_bytes())?;
    file.write_all(&header.off_dt_strings.to_be_bytes())?;
    file.write_all(&header.off_mem_rsvmap.to_be_bytes())?;
    file.write_all(&header.version.to_be_bytes())?;
    file.write_all(&header.last_comp_version.to_be_bytes())?;
    file.write_all(&header.boot_cpuid_phys.to_be_bytes())?;
    file.write_all(&header.size_dt_strings.to_be_bytes())?;
    file.write_all(&header.size_dt_struct.to_be_bytes())?;
    file.write_all(&reserve)?;
    file.write_all(&structure)?;
    file.write_all(&strings)?;

    Ok(())
}
