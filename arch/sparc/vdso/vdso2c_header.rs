// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
//! Checked big-endian ELF views and SPARC vDSO image generation.

use std::io::{self, Write};

struct Elf<'a> {
    bytes: &'a [u8],
    wide: bool,
}

impl Elf<'_> {
    fn integer(&self, offset: u64, size: usize) -> Result<u64, &'static str> {
        let start = usize::try_from(offset).map_err(|_| "ELF offset out of range\n")?;
        let end = start.checked_add(size).ok_or("ELF offset out of range\n")?;
        let bytes = self.bytes.get(start..end).ok_or("truncated ELF input\n")?;
        Ok(bytes
            .iter()
            .fold(0, |value, byte| (value << 8) | u64::from(*byte)))
    }

    fn word(&self, offset: u64) -> Result<u64, &'static str> {
        self.integer(offset, if self.wide { 8 } else { 4 })
    }

    fn field(&self, base: u64, offset: u64) -> Result<u64, &'static str> {
        self.word(
            base.checked_add(offset)
                .ok_or("ELF offset out of range\n")?,
        )
    }
}

pub(crate) fn validate(raw: &[u8], stripped_len: usize) -> Result<(), &'static str> {
    let wide = match raw.get(4) {
        Some(1) => false,
        Some(2) => true,
        _ => return Err("unknown ELF class\n"),
    };
    // Read all fields as big endian, independent of EI_DATA, like the C tool.
    let elf = Elf { bytes: raw, wide };
    let phoff = elf.word(if wide { 32 } else { 28 })?;
    let phnum = elf.integer(if wide { 56 } else { 44 }, 2)?;
    let mut load_size = None;
    let mut dynamic = None;
    for index in 0..phnum {
        // C advances an Elf*_Phdr pointer; e_phentsize is deliberately unused.
        let base = phoff
            .checked_add(index * if wide { 56 } else { 32 })
            .ok_or("ELF offset out of range\n")?;
        match elf.integer(base, 4)? {
            1 => {
                if load_size.is_some() {
                    return Err("multiple PT_LOAD segs\n");
                }
                if elf.field(base, if wide { 8 } else { 4 })? != 0
                    || elf.field(base, if wide { 16 } else { 8 })? != 0
                {
                    return Err("PT_LOAD in wrong place\n");
                }
                let memsz = elf.field(base, if wide { 40 } else { 20 })?;
                if memsz != elf.field(base, if wide { 32 } else { 16 })? {
                    return Err("cannot handle memsz != filesz\n");
                }
                load_size = Some(memsz);
            }
            2 => {
                let start = elf.field(base, if wide { 8 } else { 4 })?;
                let size = elf.field(base, if wide { 40 } else { 20 })?;
                dynamic = Some((
                    start,
                    start.checked_add(size).ok_or("ELF offset out of range\n")?,
                ));
            }
            _ => {}
        }
    }
    if (stripped_len as u64) < load_size.ok_or("no PT_LOAD seg\n")? {
        return Err("stripped input is too short\n");
    }
    if let Some((mut offset, end)) = dynamic {
        while offset < end {
            let tag = elf.word(offset)?;
            if tag == 0 {
                break;
            }
            let value = elf.field(offset, if wide { 8 } else { 4 })?;
            if matches!(tag, 8 | 18) && value != 0 {
                return Err("vdso image contains dynamic relocations\n");
            }
            offset = offset
                .checked_add(if wide { 16 } else { 8 })
                .ok_or("ELF offset out of range\n")?;
        }
    }
    let shoff = elf.word(if wide { 40 } else { 32 })?;
    let shnum = elf.integer(if wide { 60 } else { 48 }, 2)?;
    let stride = elf.integer(if wide { 58 } else { 46 }, 2)?;
    let mut symtab = false;
    for index in 0..shnum {
        let offset = shoff
            .checked_add(index * stride)
            .and_then(|base| base.checked_add(4))
            .ok_or("ELF offset out of range\n")?;
        symtab |= elf.integer(offset, 4)? == 2;
    }
    if !symtab {
        return Err("no symbol table\n");
    }
    Ok(())
}

pub(crate) fn emit(
    output: &mut impl Write,
    stripped: &[u8],
    name: Option<&[u8]>,
) -> io::Result<()> {
    let Some(name) = name else {
        return output.write_all(stripped);
    };
    let mapping_size = stripped
        .len()
        .checked_add(8191)
        .ok_or_else(|| io::Error::other("vDSO image too large"))?
        / 8192
        * 8192;
    output.write_all(b"/* AUTOMATICALLY GENERATED -- DO NOT EDIT */\n\n#include <linux/cache.h>\n#include <asm/vdso.h>\n\n")?;
    write!(
        output,
        "static unsigned char raw_data[{mapping_size}] __ro_after_init __aligned(8192)= {{"
    )?;
    for (index, byte) in stripped.iter().enumerate() {
        if index % 10 == 0 {
            output.write_all(b"\n\t")?;
        }
        write!(output, "0x{byte:02X}, ")?;
    }
    output.write_all(b"\n};\n\nconst struct vdso_image ")?;
    output.write_all(name)?;
    write!(
        output,
        "_builtin = {{\n\t.data = raw_data,\n\t.size = {mapping_size},\n}};\n"
    )
}
