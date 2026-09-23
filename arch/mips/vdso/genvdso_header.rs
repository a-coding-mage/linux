// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2015 Imagination Technologies
// Author: Alex Smith <alex.smith@imgtec.com>
//! Checked MIPS vDSO section repair and ABI-specific symbol extraction.

use crate::elf::ElfFile;

fn image(data: &[u8]) -> Result<ElfFile<'_>, String> {
    ElfFile::parse(data, 1 << 3).map_err(|error| format!("has malformed ELF data: {error}"))
}

pub(crate) fn validate(data: &[u8]) -> Result<(u8, u8), String> {
    if data.get(..4) != Some(b"\x7fELF") {
        return Err("is not an ELF file".into());
    }
    let class = *data.get(4).ok_or("has a truncated ELF header")?;
    if !matches!(class, 1 | 2) {
        return Err("has invalid ELF class".into());
    }
    let order = *data.get(5).ok_or("has a truncated ELF header")?;
    if !matches!(order, 1 | 2) {
        return Err("has invalid ELF data order".into());
    }
    let half = |offset: usize| -> Result<u16, String> {
        let bytes: [u8; 2] = data
            .get(offset..offset + 2)
            .ok_or("has a truncated ELF header")?
            .try_into()
            .unwrap();
        Ok(if order == 1 {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        })
    };
    if half(18)? != 8 {
        return Err("has invalid ELF machine (expected EM_MIPS)".into());
    }
    if half(16)? != 3 {
        return Err("has invalid ELF type (expected ET_DYN)".into());
    }
    image(data)?;
    Ok((class, order))
}

fn store(data: &mut [u8], offset: u64, bytes: &[u8]) -> Result<(), String> {
    let offset: usize = offset
        .try_into()
        .map_err(|_| "has an oversized ELF offset")?;
    let end = offset
        .checked_add(bytes.len())
        .ok_or("has an overflowing ELF offset")?;
    data.get_mut(offset..end)
        .ok_or("has a truncated ELF section")?
        .copy_from_slice(bytes);
    Ok(())
}

pub(crate) fn patch(data: &mut [u8]) -> Result<(), String> {
    let elf = image(data)?;
    let count = elf.sections()?.len();
    let width = elf.word_size();
    let little = elf.little_endian();
    let table = elf.read_integer(if width == 8 { 40 } else { 32 }, width)?;
    let stride = if width == 8 { 64 } else { 40 };
    let string_index = elf.read_integer(if width == 8 { 62 } else { 50 }, 2)?;
    let string_index = if string_index == 0xffff {
        elf.section(0)?.link as usize
    } else {
        string_index as usize
    };
    let strings = elf.section(string_index)?;
    for index in 0..count {
        // Re-read names after each repair: two sections may share a string
        // offset, and the second must see the first one's renamed value.
        let elf = image(data)?;
        let section = elf.section(index)?;
        if matches!(section.kind, 4 | 9) {
            return Err("contains relocation sections".into());
        }
        let name = elf.section_name(&section)?;
        if name == b".MIPS.abiflags" {
            return Err("already contains a '.MIPS.abiflags' section".into());
        }
        if name != b".mips_abiflags" {
            continue;
        }
        let header = table
            .checked_add(index as u64 * stride)
            .ok_or("has an overflowing ELF section table")?;
        let name_offset = strings
            .offset
            .checked_add(elf.read_integer(header, 4)?)
            .ok_or("has an overflowing ELF string offset")?;
        let kind: u32 = 0x7000_002a;
        let kind = if little {
            kind.to_le_bytes()
        } else {
            kind.to_be_bytes()
        };
        let size = if little {
            section.size.to_le_bytes()
        } else {
            section.size.to_be_bytes()
        };
        store(data, name_offset, b".MIPS.abiflags")?;
        store(data, header + 4, &kind)?;
        let encoded_size = if little {
            &size[..width]
        } else {
            &size[8 - width..]
        };
        store(
            data,
            header + if width == 8 { 56 } else { 36 },
            encoded_size,
        )?;
    }
    Ok(())
}

pub(crate) fn symbol_offsets(data: &[u8]) -> Result<Vec<(&'static str, u64)>, String> {
    let elf = image(data)?;
    let table = elf
        .sections()?
        .into_iter()
        .find(|section| section.kind == 2)
        .ok_or("has no symbol table")?;
    let symbols = elf.symbols(&table)?;
    let strings = elf.section(table.link as usize)?;
    let flags = elf.read_integer(if elf.word_size() == 8 { 48 } else { 36 }, 4)?;
    let o32 = elf.word_size() == 4 && flags & 0x20 == 0;
    let mut offsets = Vec::new();
    for (name, field, required) in [
        (b"__vdso_sigreturn".as_slice(), "off_sigreturn", o32),
        (b"__vdso_rt_sigreturn".as_slice(), "off_rt_sigreturn", true),
    ] {
        if !required {
            continue;
        }
        let mut found = None;
        for symbol in &symbols {
            if elf.string(&strings, symbol.name)? == name {
                found = Some(symbol.value);
                break;
            }
        }
        let value = found.ok_or_else(|| {
            format!(
                "is missing required symbol '{}'",
                String::from_utf8_lossy(name)
            )
        })?;
        offsets.push((field, value));
    }
    Ok(offsets)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
