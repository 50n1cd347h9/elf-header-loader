use std::fs::File;
use std::io::Read;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

#[allow(non_snake_case)]
#[derive(Debug)]
struct ElfHeader
{
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16, 
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}

#[allow(non_snake_case)]
impl ElfHeader
{
    fn parse_file_header(buf: &[u8]) -> std::io::Result<ElfHeader>
    {
        let mut cur = Cursor::new(buf);
        let mut e_ident = [0_u8; 16];
        for cc in &mut e_ident
        {
            *cc = cur.read_u8()?;
        }

        let e_type = cur.read_u16::<LittleEndian>()?;
        let e_machine = cur.read_u16::<LittleEndian>()?;
        let e_version = cur.read_u32::<LittleEndian>()?;
        let e_entry = cur.read_u64::<LittleEndian>()?;
        let e_phoff = cur.read_u64::<LittleEndian>()?;
        let e_shoff = cur.read_u64::<LittleEndian>()?;
        let e_flags = cur.read_u32::<LittleEndian>()?;
        let e_ehsize = cur.read_u16::<LittleEndian>()?;
        let e_phentsize = cur.read_u16::<LittleEndian>()?;
        let e_phnum = cur.read_u16::<LittleEndian>()?;
        let e_shentsize = cur.read_u16::<LittleEndian>()?;
        let e_shnum = cur.read_u16::<LittleEndian>()?;
        let e_shstrndx = cur.read_u16::<LittleEndian>()?;

        Ok(ElfHeader 
           {
               e_ident,
               e_type,
               e_machine,
               e_version,
               e_entry,
               e_phoff,
               e_shoff,
               e_flags,
               e_ehsize,
               e_phentsize,
               e_phnum, 
               e_shentsize,
               e_shnum,
               e_shstrndx
           })
    }
}

fn main() -> std::io::Result<()>
{
    let mut f = File::open("chall")?;
    let mut buf_file_header = [0_u8; 64];

    let _ = f.read(&mut buf_file_header)?;

    let file_header = ElfHeader::parse_file_header(&buf_file_header)?;

    println!("{:?}", file_header);
    
    Ok(())
}
