// SPDX-License-Identifier: GPL-2.0
//! Shared ELF32/ELF64 vDSO validation and C-image generation.

use crate::elf::{ElfFile, Section};
use std::io::Write;

const SYMBOLS: [&str; 10] = [
    "__kernel_vsyscall",
    "__kernel_sigreturn",
    "__kernel_rt_sigreturn",
    "int80_landing_pad",
    "vdso32_rt_sigreturn_landing_pad",
    "vdso32_sigreturn_landing_pad",
    "__futex_list64_try_unlock_cs_start",
    "__futex_list64_try_unlock_cs_end",
    "__futex_list32_try_unlock_cs_start",
    "__futex_list32_try_unlock_cs_end",
];

fn extent(data: &[u8], offset: u64, length: u64) -> Result<&[u8], String> {
    let start = usize::try_from(offset).map_err(|_| "ELF offset exceeds host address space")?;
    let end = offset
        .checked_add(length)
        .and_then(|end| usize::try_from(end).ok())
        .ok_or("ELF extent overflow")?;
    data.get(start..end)
        .ok_or_else(|| "truncated ELF data".into())
}

fn segments(elf: &ElfFile<'_>, raw: &[u8], stripped: &[u8]) -> Result<(), String> {
    let word = elf.word_size();
    let wide = word == 8;
    let phoff = elf.read_integer(if wide { 32 } else { 28 }, word)?;
    let stride = elf.read_integer(if wide { 54 } else { 42 }, 2)?;
    let count = elf.read_integer(if wide { 56 } else { 44 }, 2)?;
    if stride != if wide { 56 } else { 32 } {
        return Err("invalid program header size".into());
    }
    extent(
        raw,
        phoff,
        count
            .checked_mul(stride)
            .ok_or("program header size overflow")?,
    )?;
    let mut load = None;
    let mut dynamic = None;
    for index in 0..count {
        let base = phoff + index * stride;
        let kind = elf.read_integer(base, 4)?;
        let offset = elf.read_integer(base + if wide { 8 } else { 4 }, word)?;
        let size = elf.read_integer(base + if wide { 40 } else { 20 }, word)?;
        match kind {
            1 => {
                if load.is_some() {
                    return Err("multiple PT_LOAD segs".into());
                }
                let address = elf.read_integer(base + if wide { 16 } else { 8 }, word)?;
                if offset != 0 || address != 0 {
                    return Err("PT_LOAD in wrong place".into());
                }
                if size != elf.read_integer(base + if wide { 32 } else { 16 }, word)? {
                    return Err("cannot handle memsz != filesz".into());
                }
                load = Some(size);
            }
            2 => dynamic = Some((offset, size)),
            _ => {}
        }
    }
    let load = load.ok_or("no PT_LOAD seg")?;
    if (stripped.len() as u64) < load {
        return Err("stripped input is too short".into());
    }
    let (offset, size) =
        dynamic.ok_or("input has no PT_DYNAMIC section -- your toolchain is buggy")?;
    extent(raw, offset, size)?;
    let stride = 2 * word as u64;
    if size % stride != 0 {
        return Err("invalid dynamic table size".into());
    }
    for index in 0..size / stride {
        let tag = elf.read_integer(offset + index * stride, word)?;
        if tag == 0 {
            break;
        }
        if matches!(tag, 7 | 17 | 18 | 19 | 22) {
            return Err("vdso image contains dynamic relocations".into());
        }
    }
    Ok(())
}

fn copy(output: &mut Vec<u8>, bytes: &[u8]) {
    for (index, byte) in bytes.iter().enumerate() {
        if index % 10 == 0 {
            output.extend_from_slice(b"\n\t");
        }
        // A Vec writer cannot fail other than allocation failure.
        let _ = write!(output, "0x{byte:02X}, ");
    }
}

fn extracted<'a>(elf: &ElfFile<'a>, section: &Section) -> Result<&'a [u8], String> {
    elf.section_data(section)
        .map_err(|_| "section to extract overruns input data".into())
}

pub(crate) fn generate(
    raw: &[u8],
    stripped: &[u8],
    name: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    if !matches!(raw.get(4), Some(1 | 2)) {
        return Err("unknown ELF class".into());
    }
    if raw.get(16..18) != Some(&[3, 0]) {
        return Err("input is not a shared object".into());
    }
    let elf = ElfFile::parse(raw, 1 << 3)?;
    if !elf.little_endian() {
        return Err("x86 vDSO must use little-endian encoding".into());
    }
    segments(&elf, raw, stripped)?;
    let sections = elf.sections()?;
    let table = sections
        .iter()
        .rev()
        .find(|section| section.kind == 2)
        .ok_or("no symbol table")?;
    let strings = elf.section(table.link as usize)?;
    let alternatives = elf.find_section(b".altinstructions")?;
    let exceptions = elf.find_section(b"__ex_table")?;
    let mut values = [0i64; SYMBOLS.len()];
    for symbol in elf.symbols(table)? {
        let symbol_name = elf.string(&strings, symbol.name)?;
        if let Some(index) = SYMBOLS
            .iter()
            .position(|name| name.as_bytes() == symbol_name)
        {
            if values[index] != 0 {
                return Err(format!("duplicate symbol {}", SYMBOLS[index]));
            }
            values[index] = if elf.word_size() == 8 {
                symbol.value as i64
            } else {
                symbol.value as u32 as i32 as i64
            };
        }
    }
    let Some(name) = name else {
        return Ok(stripped.to_vec());
    };
    let mapping_size = stripped
        .len()
        .checked_add(4095)
        .ok_or("vDSO mapping size overflow")?
        / 4096
        * 4096;
    let mut output = Vec::new();
    output.extend_from_slice(b"/* AUTOMATICALLY GENERATED -- DO NOT EDIT */\n\n#include <linux/linkage.h>\n#include <linux/init.h>\n#include <asm/page_types.h>\n#include <asm/vdso.h>\n\n");
    let _ = write!(
        output,
        "static unsigned char raw_data[{mapping_size}] __ro_after_init __aligned(PAGE_SIZE) = {{"
    );
    copy(&mut output, stripped);
    output.extend_from_slice(b"\n};\n\n");
    if let Some(section) = exceptions {
        let data = extracted(&elf, &section)?;
        let _ = write!(
            output,
            "static const unsigned char extable[{}] = {{",
            data.len()
        );
        copy(&mut output, data);
        output.extend_from_slice(b"\n};\n\n");
    }
    output.extend_from_slice(b"const struct vdso_image ");
    output.extend_from_slice(name);
    let _ = write!(
        output,
        " = {{\n\t.data = raw_data,\n\t.size = {mapping_size},\n"
    );
    if let Some(section) = alternatives {
        let _ = write!(
            output,
            "\t.alt = {},\n\t.alt_len = {},\n",
            section.offset, section.size
        );
    }
    if let Some(section) = exceptions {
        let _ = write!(
            output,
            "\t.extable_base = {},\n\t.extable_len = {},\n\t.extable = extable,\n",
            section.offset, section.size
        );
    }
    for (symbol, value) in SYMBOLS.iter().zip(values) {
        if value != 0 {
            let _ = writeln!(output, "\t.sym_{symbol} = {value},");
        }
    }
    output.extend_from_slice(b"};\n\nstatic __init int init_");
    output.extend_from_slice(name);
    output.extend_from_slice(b"(void) {\n\treturn init_vdso_image(&");
    output.extend_from_slice(name);
    output.extend_from_slice(b");\n};\nsubsys_initcall(init_");
    output.extend_from_slice(name);
    output.extend_from_slice(b");\n");
    Ok(output)
}
