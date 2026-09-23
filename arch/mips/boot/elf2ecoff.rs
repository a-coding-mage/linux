// SPDX-License-Identifier: BSD-3-Clause
/*
 * Copyright (c) 1995
 * Ted Lemon (hereinafter referred to as the author)
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
//! Convert ELF32 load segments into ECOFF images for MIPS boot firmware.

mod host_tool;

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

#[derive(Clone, Copy, Default)]
struct Section {
    address: u32,
    size: u32,
}

struct Program {
    kind: u32,
    offset: u32,
    address: u32,
    file_size: u32,
    memory_size: u32,
    flags: u32,
}

fn number(data: &[u8], offset: usize, width: usize, big: bool) -> u32 {
    let bytes = &data[offset..offset + width];
    if big {
        bytes.iter().fold(0, |n, &b| n << 8 | u32::from(b))
    } else {
        bytes.iter().rev().fold(0, |n, &b| n << 8 | u32::from(b))
    }
}

fn put(data: &mut [u8], offset: usize, width: usize, value: u32, big: bool) {
    for index in 0..width {
        let shift = if big { width - index - 1 } else { index };
        data[offset + index] = (value >> (shift * 8)) as u8;
    }
}

fn path_message(prefix: &[u8], path: &Path, suffix: &[u8]) -> Vec<u8> {
    let mut message = prefix.to_vec();
    message.extend_from_slice(path.as_os_str().as_encoded_bytes());
    message.extend_from_slice(suffix);
    message
}

fn read_table(
    input: &mut File,
    offset: u32,
    length: usize,
    name: &str,
) -> Result<Vec<u8>, Vec<u8>> {
    input
        .seek(SeekFrom::Start(u64::from(offset)))
        .map_err(|error| format!("{name}: fseek: {}\n", host_tool::errno(&error)).into_bytes())?;
    // ELF32 counts bound each allocation to fewer than 3 MiB.
    let mut bytes = vec![0; length];
    let count = input
        .read(&mut bytes)
        .map_err(|error| format!("{name}: read: {}.\n", host_tool::errno(&error)).into_bytes())?;
    if count != length {
        return Err(format!(
            "{name}: read: {}.\n",
            if count == 0 {
                "End of file reached"
            } else {
                "Success"
            }
        )
        .into_bytes());
    }
    Ok(bytes)
}

fn combine(base: &mut Section, new: Section, pad: bool) -> Result<(), Vec<u8>> {
    if base.size == 0 {
        *base = new;
    } else if new.size != 0 {
        if base.address.wrapping_add(base.size) != new.address {
            if pad {
                base.size = new.address.wrapping_sub(base.address);
            } else {
                return Err(b"Non-contiguous data can't be converted.\n".to_vec());
            }
        }
        base.size = base.size.wrapping_add(new.size);
    }
    Ok(())
}

fn write(output: &mut File, bytes: &[u8], name: &str) -> Result<(), Vec<u8>> {
    output
        .write_all(bytes)
        .map_err(|error| host_tool::perror(name, error))
}

fn copy(output: &mut File, input: &mut File, offset: u32, size: u32) -> Result<(), Vec<u8>> {
    input
        .seek(SeekFrom::Start(u64::from(offset)))
        .map_err(|error| host_tool::perror("copy: lseek", error))?;
    let mut left = size;
    let mut bytes = [0u8; 4096];
    while left != 0 {
        let length = (left as usize).min(bytes.len());
        let count = input
            .read(&mut bytes[..length])
            .map_err(|error| host_tool::perror("copy: read", error))?;
        if count != length {
            return Err(format!(
                "copy: read: {}\n",
                if count == 0 {
                    "premature end of file"
                } else {
                    "Success"
                }
            )
            .into_bytes());
        }
        write(output, &bytes[..length], "copy: write")?;
        left -= length as u32;
    }
    Ok(())
}

fn run() -> Result<(), Vec<u8>> {
    let arguments: Vec<_> = std::env::args_os().collect();
    if !(3..=4).contains(&arguments.len()) || arguments.len() == 4 && arguments[3] != "-a" {
        return Err(b"usage: elf2ecoff <elf executable> <ecoff executable> [-a]\n".to_vec());
    }
    let input_path = Path::new(&arguments[1]);
    let output_path = Path::new(&arguments[2]);
    let add = arguments.len() == 4;
    let mut input = File::open(input_path).map_err(|error| {
        path_message(
            b"Can't open ",
            input_path,
            format!(" for read: {}\n", host_tool::errno(&error)).as_bytes(),
        )
    })?;
    let mut header = [0u8; 52];
    let count = input.read(&mut header).map_err(|error| {
        path_message(
            b"ex: ",
            input_path,
            format!(": {}.\n", host_tool::errno(&error)).as_bytes(),
        )
    })?;
    if count != header.len() {
        return Err(path_message(
            b"ex: ",
            input_path,
            if count == 0 {
                b": End of file reached.\n"
            } else {
                b": Success.\n"
            },
        ));
    }
    if &header[..4] != b"\x7fELF" || header[4] != 1 || !matches!(header[5], 1 | 2) {
        return Err(b"Input is not a valid ELF32 image.\n".to_vec());
    }
    let big = header[5] == 2;
    let count = number(&header, 44, 2, big) as usize;
    let program_bytes = read_table(&mut input, number(&header, 28, 4, big), count * 32, "ph")?;
    // The original reads even the unused section headers, and rejects a
    // truncated table before creating the output.
    read_table(
        &mut input,
        number(&header, 32, 4, big),
        number(&header, 48, 2, big) as usize * 40,
        "sh",
    )?;
    let mut programs: Vec<_> = program_bytes
        .chunks_exact(32)
        .map(|p| Program {
            kind: number(p, 0, 4, big),
            offset: number(p, 4, 4, big),
            address: number(p, 8, 4, big),
            file_size: number(p, 16, 4, big),
            memory_size: number(p, 20, 4, big),
            flags: number(p, 24, 4, big),
        })
        .collect();
    programs.sort_by_key(|p| p.address);
    let (mut text, mut data, mut bss) =
        (Section::default(), Section::default(), Section::default());
    let mut current = u32::MAX;
    for p in &programs {
        match p.kind {
            0 | 4 | 6 | 0x70000000 | 0x70000003 => continue,
            1 => {
                let new = Section {
                    address: p.address,
                    size: p.file_size,
                };
                if p.flags & 2 != 0 {
                    combine(&mut data, new, false)?;
                    combine(
                        &mut bss,
                        Section {
                            address: p.address.wrapping_add(p.file_size),
                            size: p.memory_size.wrapping_sub(p.file_size),
                        },
                        true,
                    )?;
                } else {
                    combine(&mut text, new, false)?;
                }
                current = current.min(p.address);
            }
            _ => {
                return Err(format!(
                    "Program header {count} type {} can't be converted.\n",
                    p.kind as i32
                )
                .into_bytes())
            }
        }
    }
    if text.address > data.address
        || data.address > bss.address
        || text.address.wrapping_add(text.size) > data.address
        || data.address.wrapping_add(data.size) > bss.address
    {
        return Err(b"Sections ordering prevents a.out conversion.\n".to_vec());
    }
    if data.size != 0 && text.size == 0 {
        text = data;
        data.address = text.address.wrapping_add(text.size);
        data.size = 0;
    }
    if text.address.wrapping_add(text.size) < data.address {
        text.size = data.address.wrapping_sub(text.address);
    }
    let sections = if add { 6 } else { 3 };
    let text_offset = (20 + 56 + sections * 40 + 15) & !15;
    let data_offset = (text_offset as u32).wrapping_add(text.size);
    let bss_offset = data_offset.wrapping_add(data.size.wrapping_add(15) & !15);
    let mut file_header = [0u8; 20];
    put(&mut file_header, 0, 2, if big { 0x160 } else { 0x162 }, big);
    put(&mut file_header, 2, 2, sections as u32, big);
    put(&mut file_header, 16, 2, 56, big);
    put(&mut file_header, 18, 2, 0x100f, big);
    let mut optional = [0u8; 56];
    put(&mut optional, 0, 2, 0o407, big);
    put(&mut optional, 2, 2, 200, big);
    for (offset, value) in [
        (4, text.size),
        (8, data.size),
        (12, bss.size),
        (16, number(&header, 24, 4, big)),
        (20, text.address),
        (24, data.address),
        (28, bss.address),
        (32, 0xf3fffffe),
    ] {
        put(&mut optional, offset, 4, value, big);
    }
    let mut section_headers = vec![0u8; sections * 40];
    for (i, (name, address, size, offset, flags)) in [
        (
            b".text".as_slice(),
            text.address,
            text.size,
            text_offset as u32,
            0x20,
        ),
        (
            b".data".as_slice(),
            data.address,
            data.size,
            data_offset,
            0x40,
        ),
        (b".bss".as_slice(), bss.address, bss.size, bss_offset, 0x82),
        (b".rdata".as_slice(), 0, 0, 0, 0x100),
        (b".sdata".as_slice(), 0, 0, 0, 0x200),
        (b".sbss".as_slice(), 0, 0, 0, 0x400),
    ]
    .into_iter()
    .take(sections)
    .enumerate()
    {
        let entry = &mut section_headers[i * 40..(i + 1) * 40];
        entry[..name.len()].copy_from_slice(name);
        for (field, value) in [
            (8, address),
            (12, address),
            (16, size),
            (20, offset),
            (36, flags),
        ] {
            put(entry, field, 4, value, big);
        }
    }
    // No O_TRUNC: an existing output retains bytes beyond the newly written
    // image, and existing permissions/inode and symbolic links are preserved.
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o777)
        .open(output_path)
        .map_err(|error| {
            path_message(
                b"Unable to create ",
                output_path,
                format!(": {}\n", host_tool::errno(&error)).as_bytes(),
            )
        })?;
    write(&mut output, &file_header, "efh: write")?;
    host_tool::diagnostic("wrote 20 byte file header.\n");
    write(&mut output, &optional, "eah: write")?;
    host_tool::diagnostic("wrote 56 byte a.out header.\n");
    write(&mut output, &section_headers, "esecs: write")?;
    host_tool::diagnostic(format!(
        "wrote {} bytes of section headers.\n",
        section_headers.len()
    ));
    let pad = text_offset - (20 + 56 + section_headers.len());
    if pad != 0 {
        write(&mut output, &[0u8; 16][..pad], "ipad: write")?;
        host_tool::diagnostic(format!("wrote {pad} byte pad.\n"));
    }
    for p in &programs {
        if p.kind != 1 || p.file_size == 0 {
            continue;
        }
        if current != p.address {
            let mut gap = p.address.wrapping_sub(current);
            if gap > 65536 {
                return Err(
                    format!("Intersegment gap ({} bytes) too large.\n", gap as i32).into_bytes(),
                );
            }
            host_tool::diagnostic(format!("Warning: {gap} byte intersegment gap.\n"));
            while gap != 0 {
                let size = gap.min(1024) as usize;
                output.write_all(&[0u8; 1024][..size]).map_err(|error| {
                    format!("Error writing gap: {}\n", host_tool::errno(&error)).into_bytes()
                })?;
                gap -= size as u32;
            }
        }
        host_tool::diagnostic(format!("writing {} bytes...\n", p.file_size as i32));
        copy(&mut output, &mut input, p.offset, p.file_size)?;
        current = p.address.wrapping_add(p.file_size);
    }
    output.write_all(&[0u8; 4096]).map_err(|error| {
        format!("Error writing PROM padding: {}\n", host_tool::errno(&error)).into_bytes()
    })?;
    Ok(())
}

fn main() {
    host_tool::finish(run());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
