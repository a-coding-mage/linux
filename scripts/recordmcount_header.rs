// SPDX-License-Identifier: GPL-2.0-only

//! Checked, width-independent ftrace relocation and ELF-section construction.
// Copyright 2009 John F. Reiser <jreiser@BitWagon.com>
// Copyright 2010 Steven Rostedt <srostedt@redhat.com>, Red Hat Inc.

use crate::elf::{ElfFile, Relocation, Section, Symbol};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Processed {
    pub(crate) output: Option<Vec<u8>>,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
    pub(crate) failed: bool,
}

fn message(output: &mut Vec<u8>, prefix: &str, value: &[u8], suffix: &str) {
    output.extend_from_slice(prefix.as_bytes());
    output.extend_from_slice(value);
    output.extend_from_slice(suffix.as_bytes());
}

fn write_integer(
    data: &mut [u8],
    offset: usize,
    width: usize,
    value: u64,
    little: bool,
) -> Result<(), String> {
    let end = offset.checked_add(width).ok_or("ELF offset overflow")?;
    let output = data.get_mut(offset..end).ok_or("ELF write outside file")?;
    let bytes = if little {
        value.to_le_bytes()
    } else {
        value.to_be_bytes()
    };
    output.copy_from_slice(if little {
        &bytes[..width]
    } else {
        &bytes[8 - width..]
    });
    Ok(())
}

fn append_integer(data: &mut Vec<u8>, width: usize, value: u64, little: bool) {
    let bytes = if little {
        value.to_le_bytes()
    } else {
        value.to_be_bytes()
    };
    data.extend_from_slice(if little {
        &bytes[..width]
    } else {
        &bytes[8 - width..]
    });
}

fn range(data: &[u8], offset: usize, length: usize) -> Option<&[u8]> {
    data.get(offset..offset.checked_add(length)?)
}

#[derive(Clone, Copy)]
enum Nop {
    X86(bool),
    Arm,
    Arm64,
}

struct Architecture {
    machine: u16,
    word: usize,
    little: bool,
    relocation: u32,
    adjust: i64,
    prefix: bool,
    nop: Option<Nop>,
    old_mips_offset: Option<u64>,
}

impl Architecture {
    fn new(machine: u16, word: usize, little: bool) -> Self {
        let relocation = match machine {
            3 | 20 | 42 => 1,
            40 => 2,
            183 => 257,
            8 => {
                if word == 8 {
                    18
                } else {
                    2
                }
            }
            258 => {
                if word == 8 {
                    2
                } else {
                    1
                }
            }
            21 => 38,
            22 => {
                if word == 8 {
                    22
                } else {
                    0
                }
            }
            43 => 32,
            62 => 1,
            _ => unreachable!(),
        };
        Self {
            machine,
            word,
            little,
            relocation,
            adjust: if (machine == 3 && word == 4) || (machine == 62 && word == 8) {
                -1
            } else if machine == 22 && word == 8 {
                -14
            } else {
                0
            },
            prefix: !matches!(machine, 3 | 40 | 42 | 62),
            nop: match machine {
                3 => Some(Nop::X86(false)),
                62 => Some(Nop::X86(true)),
                40 => Some(Nop::Arm),
                183 => Some(Nop::Arm64),
                _ => None,
            },
            old_mips_offset: None,
        }
    }

    fn mask(&self, value: u64) -> u64 {
        if self.word == 4 {
            value & 0xffff_ffff
        } else {
            value
        }
    }

    fn is_mcount(&self, mut name: &[u8]) -> bool {
        if name.first() == Some(&b'.') {
            name = &name[1..];
        }
        name == if self.prefix {
            &b"_mcount"[..]
        } else {
            &b"mcount"[..]
        } || name == b"__fentry__"
            || (self.machine == 40 && name == b"__gnu_mcount_nc")
    }

    fn fake(&mut self, relocation: &Relocation) -> bool {
        match self.machine {
            8 => {
                let fake = self.old_mips_offset.is_some_and(|old| {
                    old != self.mask(u64::MAX)
                        && self.mask(relocation.offset.wrapping_sub(old)) == 4
                });
                self.old_mips_offset = Some(relocation.offset);
                fake
            }
            40 if self.word == 4 => !matches!(relocation.kind, 1 | 10 | 28),
            183 if self.word == 8 => relocation.kind != 283,
            258 => {
                // Preserve the original LoongArch filter's 32-bit r_info view.
                let value = if self.word == 4 {
                    relocation.symbol.wrapping_shl(8) | relocation.kind
                } else if self.little == cfg!(target_endian = "little") {
                    relocation.kind
                } else {
                    relocation.symbol
                };
                !matches!(value, 20 | 29)
            }
            _ => false,
        }
    }

    fn info(&self, output: &mut Vec<u8>, symbol: u32, kind: u32) {
        if self.machine == 8 && self.word == 8 {
            append_integer(output, 4, u64::from(symbol), self.little);
            output.extend_from_slice(&[0, 0, 0, kind as u8]);
        } else {
            let info = (u64::from(symbol) << if self.word == 8 { 32 } else { 8 }) | u64::from(kind);
            append_integer(output, self.word, info, self.little);
        }
    }

    fn make_nop(&self, data: &mut [u8], offset: usize) -> bool {
        let Some(nop) = self.nop else {
            return false;
        };
        let (start, replacement): (usize, &[u8]) = match nop {
            Nop::X86(is64) => {
                let Some(start) = offset.checked_sub(1) else {
                    return false;
                };
                if range(data, start, 5) != Some(&[0xe8, 0, 0, 0, 0]) {
                    return false;
                }
                (
                    start,
                    if is64 {
                        &[0x0f, 0x1f, 0x44, 0, 0]
                    } else {
                        &[0x3e, 0x8d, 0x74, 0x26, 0]
                    },
                )
            }
            Nop::Arm64 => {
                if range(data, offset, 4) != Some(&[0, 0, 0, 0x94]) {
                    return false;
                }
                (offset, &[0x1f, 0x20, 0x03, 0xd5])
            }
            Nop::Arm => {
                let (call, push, thumb): (&[u8], &[u8], &[u8]) = if self.little {
                    (
                        &[0xfe, 0xff, 0xff, 0xeb],
                        &[4, 0xe0, 0x2d, 0xe5],
                        &[0, 0xb5, 0xff, 0xf7, 0xfe, 0xff],
                    )
                } else {
                    (
                        &[0xeb, 0xff, 0xff, 0xfe],
                        &[0xe5, 0x2d, 0xe0, 4],
                        &[0xb5, 0, 0xf7, 0xff, 0xff, 0xfe],
                    )
                };
                if range(data, offset, 4) == Some(call) {
                    let pushed = offset
                        .checked_sub(4)
                        .filter(|&start| data.get(start..offset) == Some(push));
                    match (self.little, pushed) {
                        (true, Some(start)) => (start, &[0, 0, 0xa0, 0xe1, 0, 0, 0xa0, 0xe1]),
                        (false, Some(start)) => (start, &[0xe1, 0xa0, 0, 0, 0xe1, 0xa0, 0, 0]),
                        (true, None) => (offset, &[0, 0, 0xa0, 0xe1]),
                        (false, None) => (offset, &[0xe1, 0xa0, 0, 0]),
                    }
                } else if let Some(start) = offset
                    .checked_sub(2)
                    .filter(|&start| range(data, start, 6) == Some(thumb))
                {
                    (
                        start,
                        if self.little {
                            &[0, 0xbf, 0, 0xbf, 0, 0xbf]
                        } else {
                            &[0xbf, 0, 0xbf, 0, 0xbf, 0]
                        },
                    )
                } else {
                    return false;
                }
            }
        };
        data[start..start + replacement.len()].copy_from_slice(replacement);
        true
    }
}

fn counted(name: &[u8]) -> bool {
    name.starts_with(b".text")
        || matches!(
            name,
            b".init.text"
                | b".ref.text"
                | b".sched.text"
                | b".spinlock.text"
                | b".irqentry.text"
                | b".softirqentry.text"
                | b".kprobes.text"
                | b".cpuidle.text"
        )
}

struct SymbolTable {
    symbols: Vec<Symbol>,
    indices: Vec<u32>,
    names: Vec<Vec<u8>>,
}

impl SymbolTable {
    fn load(elf: &ElfFile<'_>, sections: &[Section], table: &Section) -> Result<Self, String> {
        let symbols = elf.symbols(table)?;
        let strings = elf.section(table.link as usize)?;
        let extended = sections
            .iter()
            .find(|section| section.kind == 18 && section.link as usize == table.index);
        let mut indices = Vec::new();
        let mut names = Vec::new();
        for (index, symbol) in symbols.iter().enumerate() {
            let section = if symbol.section == 0xffff {
                let extended = extended.ok_or("missing ELF extended symbol indices")?;
                let offset = (index as u64)
                    .checked_mul(4)
                    .and_then(|offset| offset.checked_add(extended.offset))
                    .ok_or("ELF symbol index overflow")?;
                if index as u64 * 4 + 4 > extended.size {
                    return Err("truncated ELF extended symbol indices".into());
                }
                elf.read_integer(offset, 4)? as u32
            } else if symbol.section < 0xff00 {
                u32::from(symbol.section)
            } else {
                0
            };
            indices.push(section);
            names.push(elf.string(&strings, symbol.name)?.to_vec());
        }
        Ok(Self {
            symbols,
            indices,
            names,
        })
    }
}

pub(crate) fn process(data: &[u8], filename: &[u8], warn: bool) -> Processed {
    let mut result = Processed::default();
    match run(data, filename, warn, &mut result) {
        Ok(output) => result.output = output,
        Err(error) => {
            result.failed = true;
            result.stderr.extend_from_slice(error.as_bytes());
            if !error.is_empty() && !error.ends_with('\n') {
                result.stderr.push(b'\n');
            }
        }
    }
    result
}

fn run(
    data: &[u8],
    filename: &[u8],
    warn: bool,
    result: &mut Processed,
) -> Result<Option<Vec<u8>>, String> {
    if data.len() < 16 {
        return Err("truncated ELF file".into());
    }
    if !matches!(data[5], 1 | 2) {
        message(
            &mut result.stderr,
            &format!("unrecognized ELF data encoding {}: ", data[5]),
            filename,
            "\n",
        );
        return Err(String::new());
    }
    let little = data[5] == 1;
    let read16 = |offset| -> Result<u16, String> {
        let bytes: [u8; 2] = data
            .get(offset..offset + 2)
            .ok_or("truncated ELF header")?
            .try_into()
            .unwrap();
        Ok(if little {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        })
    };
    if &data[..4] != b"\x7fELF" || read16(16)? != 1 || data[6] != 1 {
        message(
            &mut result.stderr,
            "unrecognized ET_REL file ",
            filename,
            "\n",
        );
        return Err(String::new());
    }
    let machine = read16(18)?;
    if !matches!(
        machine,
        3 | 8 | 20 | 21 | 22 | 40 | 42 | 43 | 62 | 183 | 258
    ) {
        message(
            &mut result.stderr,
            &format!("unrecognized e_machine {machine} "),
            filename,
            "\n",
        );
        return Err(String::new());
    }
    let (word, header_size, section_size, sizes_offset, shoff_offset) = match data[4] {
        1 => (4, 52, 40, 40, 32),
        2 => (8, 64, 64, 52, 40),
        class => {
            message(
                &mut result.stderr,
                &format!("unrecognized ELF class {class} "),
                filename,
                "\n",
            );
            return Err(String::new());
        }
    };
    if read16(sizes_offset)? as usize != header_size
        || read16(sizes_offset + 6)? as usize != section_size
    {
        message(
            &mut result.stderr,
            "unrecognized ET_REL file: ",
            filename,
            "\n",
        );
        return Err(String::new());
    }
    let elf = ElfFile::parse(data, 1 << 1)?;
    let sections = elf.sections()?;
    let names = sections
        .iter()
        .map(|section| elf.section_name(section).map(<[u8]>::to_vec))
        .collect::<Result<Vec<_>, _>>()?;
    let mut total = 0u64;
    let mut applicable = Vec::new();
    for section in &sections {
        if !matches!(section.kind, 4 | 9) {
            continue;
        }
        let target = sections
            .get(section.info as usize)
            .ok_or("invalid ELF relocation target section")?;
        let name = &names[target.index];
        if name == b"__mcount_loc" {
            message(
                &mut result.stderr,
                "warning: __mcount_loc already exists: ",
                filename,
                "\n",
            );
            return Ok(None);
        }
        if target.kind != 1 || target.flags & 4 == 0 {
            continue;
        }
        elf.section_data(target)?;
        if counted(name) {
            total = total
                .checked_add(section.size)
                .ok_or("ELF relocation size overflow")?;
        }
        applicable.push(section);
    }
    if total == 0 {
        return Ok(None);
    }
    let mut architecture = Architecture::new(machine, word, little);
    let mut output = data.to_vec();
    let mut changed = false;
    let mut locations = Vec::new();
    let mut relocations = Vec::new();
    let mut relocation_size = 0;
    let mut symbol_link = 0;
    let mut tables = HashMap::new();
    for section in applicable {
        let is_counted = counted(&names[section.info as usize]);
        if !is_counted && !warn && architecture.nop.is_none() {
            continue;
        }
        let table_section = elf.section(section.link as usize)?;
        if let std::collections::hash_map::Entry::Vacant(entry) = tables.entry(section.link) {
            entry.insert(SymbolTable::load(&elf, &sections, &table_section)?);
        }
        let table = &tables[&section.link];
        let entries = elf.relocations(section)?;
        if entries
            .iter()
            .any(|relocation| relocation.offset >= sections[section.info as usize].size)
        {
            return Err("ELF relocation location outside section".into());
        }
        let stride = word * if section.kind == 4 { 3 } else { 2 };
        if section.entry_size != stride as u64 {
            return Err("invalid ELF relocation entry size".into());
        }
        let mut mcount_symbol = 0;
        if is_counted {
            let base = table.symbols.iter().enumerate().find(|&(index, symbol)| {
                table.indices[index] == section.info
                    && matches!(symbol.binding, 0 | 1)
                    && !(machine == 40 && symbol.kind == 2)
            });
            let Some((base_index, base)) = base else {
                message(
                    &mut result.stderr,
                    &format!("Cannot find symbol for section {}: ", section.info),
                    &names[section.info as usize],
                    ".\n",
                );
                return Err(String::new());
            };
            if relocation_size != 0 && relocation_size != stride {
                return Err("mixed REL/RELA ftrace sections are unsupported".into());
            }
            relocation_size = stride;
            symbol_link = section.link;
            for relocation in entries {
                let name = table
                    .names
                    .get(relocation.symbol as usize)
                    .ok_or("invalid ELF relocation symbol index")?;
                if mcount_symbol == 0 && architecture.is_mcount(name) {
                    mcount_symbol = relocation.symbol;
                }
                if mcount_symbol == 0
                    || mcount_symbol != relocation.symbol
                    || architecture.fake(&relocation)
                {
                    continue;
                }
                let addend = architecture.mask(
                    relocation
                        .offset
                        .wrapping_sub(base.value)
                        .wrapping_add(architecture.adjust as u64),
                );
                append_integer(&mut relocations, word, locations.len() as u64, little);
                architecture.info(&mut relocations, base_index as u32, architecture.relocation);
                if section.kind == 4 {
                    append_integer(&mut relocations, word, addend, little);
                    append_integer(&mut locations, word, 0, little);
                } else {
                    append_integer(&mut locations, word, addend, little);
                }
            }
        } else {
            let mut warned = false;
            let target = &sections[section.info as usize];
            for (index, relocation) in entries.iter().enumerate() {
                let name = table
                    .names
                    .get(relocation.symbol as usize)
                    .ok_or("invalid ELF relocation symbol index")?;
                if mcount_symbol == 0 && architecture.is_mcount(name) {
                    mcount_symbol = relocation.symbol;
                }
                if mcount_symbol != relocation.symbol || architecture.fake(relocation) {
                    continue;
                }
                let offset = target
                    .offset
                    .checked_add(relocation.offset)
                    .ok_or("ELF relocation offset overflow")?;
                let offset = usize::try_from(offset)
                    .map_err(|_| "ELF relocation offset exceeds host address space")?;
                let removed = architecture.make_nop(&mut output, offset);
                if warn && !warned {
                    message(
                        &mut result.stdout,
                        "Section ",
                        &names[section.info as usize],
                        " has mcount callers being ignored\n",
                    );
                    warned = true;
                    if architecture.nop.is_none() {
                        break;
                    }
                }
                if removed {
                    let mut info = Vec::new();
                    architecture.info(&mut info, relocation.symbol, 0);
                    let position = usize::try_from(section.offset)
                        .map_err(|_| "ELF offset exceeds host address space")?
                        .checked_add(index * stride + word)
                        .ok_or("ELF relocation offset overflow")?;
                    output
                        .get_mut(position..position + word)
                        .ok_or("ELF relocation outside file")?
                        .copy_from_slice(&info);
                    changed = true;
                }
            }
        }
    }
    if !locations.is_empty() {
        append_sections(
            &mut output,
            &elf,
            &sections,
            word,
            section_size,
            sizes_offset,
            shoff_offset,
            &locations,
            &relocations,
            relocation_size,
            symbol_link,
        )?;
        changed = true;
    }
    Ok(changed.then_some(output))
}

#[allow(clippy::too_many_arguments)]
fn append_sections(
    output: &mut Vec<u8>,
    elf: &ElfFile<'_>,
    sections: &[Section],
    word: usize,
    section_size: usize,
    sizes_offset: usize,
    shoff_offset: usize,
    locations: &[u8],
    relocations: &[u8],
    relocation_size: usize,
    symbol_link: u32,
) -> Result<(), String> {
    let little = elf.little_endian();
    let old_size = output.len();
    let old_shoff = usize::try_from(elf.read_integer(shoff_offset as u64, word)?)
        .map_err(|_| "ELF offset exceeds host address space")?;
    let string_index = elf.read_integer((sizes_offset + 10) as u64, 2)? as usize;
    let string_index = if string_index == 0xffff {
        sections[0].link as usize
    } else {
        string_index
    };
    let strings = &sections[string_index];
    let old_strings = elf.section_data(strings)?;
    let name: &[u8] = if relocation_size == word * 3 {
        b".rela__mcount_loc\0"
    } else {
        b".rel__mcount_loc\0"
    };
    let new_count = sections
        .len()
        .checked_add(2)
        .ok_or("ELF section count overflow")?;
    if new_count > u32::MAX as usize || old_strings.len() > u32::MAX as usize - name.len() {
        return Err("ELF section metadata exceeds its field width".into());
    }
    let final_size = old_size
        .checked_add(old_strings.len())
        .and_then(|size| size.checked_add(name.len() + word - 1))
        .map(|size| size & !(word - 1))
        .and_then(|size| {
            new_count
                .checked_mul(section_size)
                .and_then(|headers| size.checked_add(headers))
        })
        .and_then(|size| size.checked_add(locations.len()))
        .and_then(|size| size.checked_add(relocations.len()))
        .ok_or("ELF output size overflow")?;
    if word == 4 && final_size > u32::MAX as usize {
        return Err("ELF32 output exceeds its offset field width".into());
    }
    if new_count >= 0xff00 {
        write_integer(output, sizes_offset + 8, 2, 0, little)?;
        write_integer(
            output,
            old_shoff + if word == 8 { 32 } else { 20 },
            word,
            new_count as u64,
            little,
        )?;
    } else {
        write_integer(output, sizes_offset + 8, 2, new_count as u64, little)?;
    }
    let string_header = old_shoff + string_index * section_size;
    write_integer(
        output,
        string_header + if word == 8 { 24 } else { 16 },
        word,
        old_size as u64,
        little,
    )?;
    write_integer(
        output,
        string_header + if word == 8 { 32 } else { 20 },
        word,
        (old_strings.len() + name.len()) as u64,
        little,
    )?;
    output.extend_from_slice(old_strings);
    output.extend_from_slice(name);
    output.resize(
        output
            .len()
            .checked_add(word - 1)
            .ok_or("ELF size overflow")?
            & !(word - 1),
        0,
    );
    let new_shoff = output.len();
    output.extend_from_within(old_shoff..old_shoff + sections.len() * section_size);
    let location_offset = new_shoff
        .checked_add(
            new_count
                .checked_mul(section_size)
                .ok_or("ELF section table overflow")?,
        )
        .ok_or("ELF offset overflow")?;
    for (is_relocation, contents) in [(false, locations), (true, relocations)] {
        let name_offset = old_strings.len()
            + if is_relocation {
                0
            } else {
                if relocation_size == word * 3 {
                    5
                } else {
                    4
                }
            };
        append_integer(output, 4, name_offset as u64, little);
        append_integer(
            output,
            4,
            if is_relocation {
                if relocation_size == word * 3 {
                    4
                } else {
                    9
                }
            } else {
                1
            },
            little,
        );
        append_integer(output, word, if is_relocation { 0 } else { 2 }, little);
        append_integer(output, word, 0, little);
        append_integer(
            output,
            word,
            (location_offset + if is_relocation { locations.len() } else { 0 }) as u64,
            little,
        );
        append_integer(output, word, contents.len() as u64, little);
        append_integer(
            output,
            4,
            if is_relocation {
                u64::from(symbol_link)
            } else {
                0
            },
            little,
        );
        append_integer(
            output,
            4,
            if is_relocation {
                sections.len() as u64
            } else {
                0
            },
            little,
        );
        append_integer(output, word, word as u64, little);
        append_integer(
            output,
            word,
            if is_relocation {
                relocation_size as u64
            } else {
                word as u64
            },
            little,
        );
    }
    output.extend_from_slice(locations);
    output.extend_from_slice(relocations);
    write_integer(output, shoff_offset, word, new_shoff as u64, little)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
