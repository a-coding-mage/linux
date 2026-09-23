// SPDX-License-Identifier: GPL-2.0
//! Checked extraction and inspection of i386 and x86-64 kernel relocations.

#[path = "../../../scripts/elf-parse.rs"]
mod elf;
mod relocs_32;
mod relocs_64;
mod relocs_common;
mod relocs_header;

use elf::{ElfFile, Relocation, Section, Symbol};
use relocs_header::{matches, relocation_type, Options, SymbolClass, Width};
use std::io::Write;

pub(crate) struct Failure(Vec<u8>);
pub(crate) type Result<T> = std::result::Result<T, Failure>;

impl Failure {
    fn named(prefix: &str, name: &[u8], suffix: &str) -> Self {
        let mut message = prefix.as_bytes().to_vec();
        message.extend_from_slice(name);
        message.extend_from_slice(suffix.as_bytes());
        Self(message)
    }
}

impl From<&str> for Failure {
    fn from(message: &str) -> Self {
        Self(message.as_bytes().to_vec())
    }
}

impl From<String> for Failure {
    fn from(message: String) -> Self {
        Self(message.into_bytes())
    }
}

fn main() {
    if let Err(error) = relocs_common::run() {
        let _ = std::io::stderr().lock().write_all(&error.0);
        std::process::exit(1);
    }
}

fn integer(bytes: &[u8], offset: usize, width: usize) -> u64 {
    bytes[offset..offset + width]
        .iter()
        .rev()
        .fold(0, |value, byte| value << 8 | u64::from(*byte))
}

fn check_header(bytes: &[u8]) -> Result<bool> {
    let is_64 = bytes[4] == 2;
    let (header, program, section, sizes, word, shoff_at) = if is_64 {
        (64, 56, 64, 52, 8, 40)
    } else {
        (52, 32, 40, 40, 4, 32)
    };
    if bytes.len() < header {
        return Err("Cannot read ELF header: Success\n".into());
    }
    if &bytes[..4] != b"\x7fELF" {
        return Err("No ELF magic\n".into());
    }
    if bytes[4] != if is_64 { 2 } else { 1 } {
        return Err("Not a 32 bit executable\n".into());
    }
    if bytes[5] != 1 {
        return Err("Not a LSB ELF executable\n".into());
    }
    if bytes[6] != 1 {
        return Err("Unknown ELF version\n".into());
    }
    if !matches!(integer(bytes, 16, 2), 2 | 3) {
        return Err("Unsupported ELF header type\n".into());
    }
    if integer(bytes, 18, 2) != if is_64 { 62 } else { 3 } {
        return Err(format!("Not for {}\n", if is_64 { "x86_64" } else { "i386" }).into());
    }
    if integer(bytes, 20, 4) != 1 {
        return Err("Unknown ELF version\n".into());
    }
    if integer(bytes, sizes, 2) != header as u64 {
        return Err("Bad ELF header size\n".into());
    }
    if integer(bytes, sizes + 2, 2) != program {
        return Err("Bad program header entry\n".into());
    }
    if integer(bytes, sizes + 6, 2) != section {
        return Err("Bad section header entry\n".into());
    }
    let offset = integer(bytes, shoff_at, word);
    let mut count = integer(bytes, sizes + 8, 2);
    let mut strings = integer(bytes, sizes + 10, 2);
    let extended = count == 0 || strings == 0xffff;
    if offset > i64::MAX as u64 {
        return Err(format!("Seek to {offset} failed: Invalid argument\n").into());
    }
    let available = (bytes.len() as u64).saturating_sub(offset);
    if extended {
        if available < section {
            return Err("Cannot read initial ELF section header: Success\n".into());
        }
        let start = offset as usize;
        if count == 0 {
            count = integer(bytes, start + if is_64 { 32 } else { 20 }, word);
        }
        if strings == 0xffff {
            strings = integer(bytes, start + if is_64 { 40 } else { 24 }, 4);
        }
    }
    if strings >= count {
        return Err("String table index out of bounds\n".into());
    }
    if count > available / section {
        return Err(format!(
            "Cannot read ELF section headers {}/{count}: Success\n",
            available / section
        )
        .into());
    }
    Ok(is_64)
}

struct Image<'a> {
    elf: ElfFile<'a>,
    sections: Vec<Section>,
    symbols: Vec<Vec<Symbol>>,
    relocations: Vec<Vec<Relocation>>,
    is_64: bool,
}

impl<'a> Image<'a> {
    fn read(bytes: &'a [u8]) -> Result<Self> {
        let is_64 = check_header(bytes)?;
        let elf = ElfFile::parse(bytes, (1 << 2) | (1 << 3))?;
        let sections = elf.sections()?;
        let mut symbols = vec![Vec::new(); sections.len()];
        let mut relocations = vec![Vec::new(); sections.len()];
        // Match the original reader's phase ordering, including errors in
        // tables unused by the requested output mode.
        for section in &sections {
            if section.kind == 3 {
                elf.section_data(section)
                    .map_err(|_| Failure::from("Cannot read symbol table: Success\n"))?;
            }
        }
        for section in &sections {
            if matches!(section.kind, 2 | 18) {
                elf.section_data(section).map_err(|_| {
                    Failure::from(if section.kind == 18 {
                        "Cannot read extended symbol table: Success\n"
                    } else {
                        "Cannot read symbol table: Success\n"
                    })
                })?;
                if section.kind == 2 {
                    symbols[section.index] = elf.symbols(section)?;
                }
            }
        }
        for section in &sections {
            if section.kind == if is_64 { 4 } else { 9 } {
                elf.section_data(section)
                    .map_err(|_| Failure::from("Cannot read symbol table: Success\n"))?;
                relocations[section.index] = elf.relocations(section)?;
            }
        }
        Ok(Self {
            elf,
            sections,
            symbols,
            relocations,
            is_64,
        })
    }

    fn section_name(&self, index: u32) -> Result<&[u8]> {
        if let Some(section) = self.sections.get(index as usize) {
            return Ok(self.elf.section_name(section)?);
        }
        Ok(match index {
            0xfff1 => b"ABSOLUTE",
            0xfff2 => b"COMMON",
            _ => b"<noname>",
        })
    }

    fn symbol_section(&self, table: &Section, index: usize, symbol: &Symbol) -> Result<u32> {
        Ok(if symbol.section == 0xffff {
            self.elf.symbol_section_index(table, index, symbol)?
        } else {
            u32::from(symbol.section)
        })
    }

    fn symbol_name(&self, table: &Section, index: usize, symbol: &Symbol) -> Result<&[u8]> {
        if symbol.name == 0 {
            self.section_name(self.symbol_section(table, index, symbol)?)
        } else {
            Ok(self
                .elf
                .string(&self.elf.section(table.link as usize)?, symbol.name)?)
        }
    }

    fn walk(
        &self,
        mut callback: impl FnMut(&Section, &Section, &Relocation, &Symbol, &[u8], usize) -> Result<()>,
    ) -> Result<()> {
        for section in &self.sections {
            if section.kind != if self.is_64 { 4 } else { 9 } {
                continue;
            }
            let applies = self.elf.section(section.info as usize)?;
            if applies.flags & 2 == 0 || applies.kind == 7 {
                continue;
            }
            let table = self.elf.section(section.link as usize)?;
            if table.kind != 2 {
                return Err("Relocation section does not link to a symbol table\n".into());
            }
            for relocation in &self.relocations[section.index] {
                let index = relocation.symbol as usize;
                let symbol = self.symbols[table.index]
                    .get(index)
                    .ok_or("Relocation symbol index out of bounds\n")?;
                callback(
                    &applies,
                    &table,
                    relocation,
                    symbol,
                    self.symbol_name(&table, index, symbol)?,
                    index,
                )?;
            }
        }
        Ok(())
    }

    fn absolute_symbols(&self, output: &mut Vec<u8>) -> Result<()> {
        output.extend_from_slice(
            b"Absolute symbols\n Num:    Value Size  Type       Bind        Visibility  Name\n",
        );
        for section in &self.sections {
            if section.kind != 2 {
                continue;
            }
            for (index, symbol) in self.symbols[section.index].iter().enumerate() {
                if symbol.section != 0xfff1 {
                    continue;
                }
                let size = if self.is_64 {
                    symbol.size as i64
                } else {
                    i64::from(symbol.size as i32)
                };
                let gap = if self.is_64 { " " } else { "  " };
                let _ = write!(
                    output,
                    "{index:5} {:0digits$x}{gap}{size:5} {:>10} {:>10} {:>12} ",
                    symbol.value,
                    relocs_header::symbol_type(symbol.kind),
                    relocs_header::symbol_binding(symbol.binding),
                    relocs_header::visibility(symbol.visibility),
                    digits = if self.is_64 { 16 } else { 8 }
                );
                output.extend_from_slice(self.symbol_name(section, index, symbol)?);
                output.push(b'\n');
            }
        }
        output.push(b'\n');
        Ok(())
    }

    fn absolute_relocations(&self, output: &mut Vec<u8>, options: &Options) -> Result<()> {
        let mut printed = false;
        self.walk(|_,_,rel,sym,name,_| {
            if sym.section != 0xfff1 || matches(SymbolClass::Absolute,name,self.is_64,options.real_mode)
                || matches(SymbolClass::Relative,name,self.is_64,options.real_mode) { return Ok(()); }
            if !printed {
                output.extend_from_slice(b"WARNING: Absolute relocations present\nOffset     Info     Type     Sym.Value Sym.Name\n");
                printed = true;
            }
            let info = if self.is_64 { u64::from(rel.symbol)<<32 | u64::from(rel.kind) }
                else { u64::from(rel.symbol)<<8 | u64::from(rel.kind) };
            let _ = write!(output,"{:0digits$x} {info:0digits$x} {:>10} {:0digits$x}  ",
                rel.offset,relocation_type(rel.kind,self.is_64),sym.value,digits=if self.is_64 {16} else {8});
            output.extend_from_slice(name);
            output.push(b'\n');
            Ok(())
        })?;
        if printed {
            output.push(b'\n');
        }
        Ok(())
    }

    fn relocation_info(&self, output: &mut Vec<u8>) -> Result<()> {
        output.extend_from_slice(b"reloc section\treloc type\tsymbol\tsymbol section\n");
        self.walk(|applies, table, rel, sym, name, index| {
            output.extend_from_slice(self.elf.section_name(applies)?);
            let _ = write!(output, "\t{}\t", relocation_type(rel.kind, self.is_64));
            output.extend_from_slice(name);
            output.push(b'\t');
            output.extend_from_slice(self.section_name(self.symbol_section(table, index, sym)?)?);
            output.push(b'\n');
            Ok(())
        })
    }

    fn emit(&self, output: &mut Vec<u8>, options: &Options) -> Result<()> {
        if self.is_64 && options.real_mode {
            return Err("--realmode not valid for a 64-bit ELF file".into());
        }
        let (mut offsets16, mut offsets32, mut offsets64) = (Vec::new(), Vec::new(), Vec::new());
        self.walk(|_, _, rel, sym, name, _| {
            let bucket = if self.is_64 {
                relocs_64::classify(rel, sym, name)?
            } else {
                relocs_32::classify(rel, sym, name, options.real_mode)?
            };
            if let Some(width) = bucket {
                match width {
                    Width::Bits16 => offsets16.push(rel.offset as u32),
                    Width::Bits32 => offsets32.push(rel.offset as u32),
                    Width::Bits64 => offsets64.push(rel.offset as u32),
                }
            }
            Ok(())
        })?;
        offsets16.sort_unstable();
        offsets32.sort_unstable();
        offsets64.sort_unstable();
        if options.text {
            output.extend_from_slice(b".section \".data.reloc\",\"a\"\n.balign 4\n");
        }
        let mut word = |value: u32| {
            if options.text {
                let _ = writeln!(output, "\t.long 0x{value:08x}");
            } else {
                output.extend_from_slice(&value.to_le_bytes());
            }
        };
        if options.real_mode {
            word(
                offsets16
                    .len()
                    .try_into()
                    .map_err(|_| "Too many segment relocations\n")?,
            );
            for offset in offsets16 {
                word(offset);
            }
            word(
                offsets32
                    .len()
                    .try_into()
                    .map_err(|_| "Too many relocations\n")?,
            );
        } else {
            if self.is_64 {
                word(0);
                for offset in offsets64 {
                    word(offset);
                }
            }
            word(0);
        }
        for offset in offsets32 {
            word(offset);
        }
        Ok(())
    }
}

fn process(bytes: &[u8], options: &Options) -> Result<Vec<u8>> {
    let image = Image::read(bytes)?;
    let mut output = Vec::new();
    if options.absolute_symbols {
        image.absolute_symbols(&mut output)?;
    } else if options.absolute_relocations {
        image.absolute_relocations(&mut output, options)?;
    } else if options.relocation_info {
        image.relocation_info(&mut output)?;
    } else {
        image.emit(&mut output, options)?;
    }
    Ok(output)
}
