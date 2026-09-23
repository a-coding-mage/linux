// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2015 Imagination Technologies
// Author: Alex Smith <alex.smith@imgtec.com>
//! Repair MIPS vDSO ABI sections and emit its kernel image description.

#[path = "../../../scripts/elf-parse.rs"]
mod elf;
mod genvdso_header;

use std::ffi::OsStr;
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

fn errno_text(error: &io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error ").next().unwrap_or(&text).into()
}

fn file_error(action: &str, path: &Path, error: &io::Error) -> Vec<u8> {
    let mut result = format!("Failed to {action} '").into_bytes();
    result.extend_from_slice(path.as_os_str().as_encoded_bytes());
    result.extend_from_slice(format!("': {}", errno_text(error)).as_bytes());
    result
}

fn invalid(path: &Path, message: &str) -> Vec<u8> {
    let mut result = b"'".to_vec();
    result.extend_from_slice(path.as_os_str().as_encoded_bytes());
    result.extend_from_slice(b"' ");
    result.extend_from_slice(message.as_bytes());
    result
}

fn same_file(left: &Metadata, right: &Metadata) -> bool {
    left.dev() == right.dev() && left.ino() == right.ino()
}

struct Image {
    file: File,
    path: PathBuf,
    metadata: Metadata,
    data: Vec<u8>,
    format: (u8, u8),
}

impl Image {
    fn open(path: &Path) -> Result<Self, Vec<u8>> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|error| file_error("open", path, &error))?;
        let metadata = file
            .metadata()
            .map_err(|error| file_error("stat", path, &error))?;
        if metadata.len() == 0 {
            return Err(file_error("map", path, &io::Error::from_raw_os_error(22)));
        }
        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|error| file_error("read", path, &error))?;
        let format = genvdso_header::validate(&data).map_err(|message| invalid(path, &message))?;
        Ok(Self {
            file,
            path: path.to_owned(),
            metadata,
            data,
            format,
        })
    }

    fn patch(&mut self) -> Result<(), Vec<u8>> {
        let before = self.data.clone();
        let result = genvdso_header::patch(&mut self.data);
        // Preserve the C tool's ordered partial repairs on semantic errors.
        // Write only changed spans, retaining inode, length and other bytes.
        let mut index = 0;
        while index < self.data.len() {
            if before[index] == self.data[index] {
                index += 1;
                continue;
            }
            let start = index;
            while index < self.data.len() && before[index] != self.data[index] {
                index += 1;
            }
            self.file
                .seek(SeekFrom::Start(start as u64))
                .and_then(|_| self.file.write_all(&self.data[start..index]))
                .map_err(|error| file_error("sync", &self.path, &error))?;
        }
        result.map_err(|message| invalid(&self.path, &message))
    }

    fn sync(&self) -> Result<(), Vec<u8>> {
        self.file
            .sync_data()
            .map_err(|error| file_error("sync", &self.path, &error))
    }
}

fn emit(data: &[u8], name: &OsStr, out: &mut impl Write) -> io::Result<()> {
    out.write_all(b"/* Automatically generated - do not edit */\n#include <linux/linkage.h>\n#include <linux/mm.h>\n#include <asm/vdso.h>\nstatic int vdso_mremap(\n\tconst struct vm_special_mapping *sm,\n\tstruct vm_area_struct *new_vma)\n{\n\tcurrent->mm->context.vdso =\n\t(void *)(new_vma->vm_start);\n\treturn 0;\n}\n")?;
    write!(
        out,
        "static unsigned char vdso_image_data[PAGE_ALIGN({})] __page_aligned_data = {{\n\t",
        data.len()
    )?;
    for (index, byte) in data.iter().enumerate() {
        if index % 10 == 0 {
            out.write_all(b"\n\t")?;
        }
        write!(out, "0x{byte:02x}, ")?;
    }
    writeln!(out, "\n}};")?;
    writeln!(
        out,
        "static struct page *vdso_pages[PAGE_ALIGN({}) / PAGE_SIZE];",
        data.len()
    )?;
    out.write_all(b"struct mips_vdso_image vdso_image")?;
    if !name.is_empty() {
        out.write_all(b"_")?;
    }
    out.write_all(name.as_encoded_bytes())?;
    writeln!(out, " = {{")?;
    writeln!(
        out,
        "\t.data = vdso_image_data,\n\t.size = PAGE_ALIGN({}),",
        data.len()
    )?;
    out.write_all(b"\t.mapping = {\n\t\t.name = \"[vdso]\",\n\t\t.pages = vdso_pages,\n\t\t.mremap = vdso_mremap,\n\t},\n")
}

fn run(debug: &Path, stripped: &Path, output: &Path, name: &OsStr) -> Result<(), Vec<u8>> {
    let mut debug = Image::open(debug)?;
    let mut stripped = Image::open(stripped)?;
    if debug.format != stripped.format {
        return Err(invalid(
            &stripped.path,
            "does not match debug VDSO class and byte order",
        ));
    }
    // Truncating an input's inode through the output path gives the mmap-based
    // reference a SIGBUS. Reject this before modifying either input.
    if let Ok(metadata) = fs::metadata(output) {
        if same_file(&metadata, &debug.metadata) || same_file(&metadata, &stripped.metadata) {
            return Err(invalid(output, "aliases an input VDSO"));
        }
    }
    debug.patch()?;
    if same_file(&debug.metadata, &stripped.metadata) {
        stripped.data.clone_from(&debug.data);
    }
    stripped.patch()?;
    debug.sync()?;
    stripped.sync()?;
    let file = File::create(output).map_err(|error| file_error("open", output, &error))?;
    let mut out = BufWriter::new(file);
    emit(&stripped.data, name, &mut out).map_err(|error| file_error("write", output, &error))?;
    let offsets = match genvdso_header::symbol_offsets(&debug.data) {
        Ok(offsets) => offsets,
        Err(message) => {
            let _ = out.flush();
            drop(out);
            // Match output cleanup for files/symlinks, but never unlink a
            // device, FIFO or other special node used as the output stream.
            if fs::symlink_metadata(output).is_ok_and(|metadata| {
                metadata.file_type().is_file() || metadata.file_type().is_symlink()
            }) {
                let _ = fs::remove_file(output);
            }
            return Err(invalid(&debug.path, &message));
        }
    };
    for (field, value) in offsets {
        writeln!(out, "\t.{field} = 0x{value:x},")
            .map_err(|error| file_error("write", output, &error))?;
    }
    out.write_all(b"};\n")
        .and_then(|_| out.flush())
        .map_err(|error| file_error("write", output, &error))
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if !(4..=5).contains(&args.len()) {
        let mut error = b"Usage: ".to_vec();
        error.extend_from_slice(args[0].as_encoded_bytes());
        error.extend_from_slice(b" <debug VDSO> <stripped VDSO> <output file> [<name>]\n");
        let _ = io::stderr().write_all(&error);
        std::process::exit(1);
    }
    let name = args.get(4).map_or(OsStr::new(""), |arg| arg.as_os_str());
    if let Err(message) = run(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
        name,
    ) {
        let mut error = args[0].as_encoded_bytes().to_vec();
        error.extend_from_slice(b": ");
        error.extend_from_slice(&message);
        error.push(b'\n');
        let _ = io::stderr().write_all(&error);
        std::process::exit(1);
    }
}
