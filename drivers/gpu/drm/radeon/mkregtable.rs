// SPDX-License-Identifier: MIT
/*
 * Copyright 2009 Jerome Glisse
 * Copyright 2009 Red Hat Inc.
 *
 * Authors: Jerome Glisse, Dave Airlie
 */

//! Generate Radeon register permission bitmaps from the register source files.

// The native entry point retains the C program's inherited SIGPIPE disposition.
// C stdio also preserves fgets chunking, ftell behavior on unseekable inputs,
// buffering, and the original policy of ignoring stdout errors.
#![no_main]

use std::ffi::{c_char, c_int, c_long};

#[repr(C)]
struct File {
    _private: [u8; 0],
}

extern "C" {
    fn fopen(name: *const c_char, mode: *const c_char) -> *mut File;
    fn fclose(file: *mut File) -> c_int;
    fn fgets(buffer: *mut c_char, size: c_int, file: *mut File) -> *mut c_char;
    fn fseek(file: *mut File, offset: c_long, whence: c_int) -> c_int;
    fn ftell(file: *mut File) -> c_long;
    fn strtol(text: *const c_char, end: *mut *mut c_char, base: c_int) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(file: *mut File, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut File;
}

struct Input(*mut File);

impl Drop for Input {
    fn drop(&mut self) {
        // SAFETY: Input owns a successful fopen result, closed exactly once.
        unsafe { fclose(self.0) };
    }
}

fn field(line: &[u8], cursor: &mut usize) -> Option<[u8; 10]> {
    while *cursor < line.len() && matches!(line[*cursor], b' ' | b'\t' | b'\n' | b'\r' | 11 | 12) {
        *cursor += 1;
    }
    let start = *cursor;
    while *cursor < line.len()
        && *cursor - start < 9
        && !matches!(line[*cursor], b' ' | b'\t' | b'\n' | b'\r' | 11 | 12)
    {
        *cursor += 1;
    }
    if start == *cursor {
        return None;
    }
    let mut word = [0; 10];
    word[..*cursor - start].copy_from_slice(&line[start..*cursor]);
    Some(word)
}

fn length(buffer: &[u8; 1024]) -> usize {
    // fgets always terminates successful reads within the buffer.
    buffer.iter().position(|&byte| byte == 0).unwrap_or(buffer.len())
}

unsafe fn parse(filename: *const c_char) -> Result<([u8; 10], Vec<u32>), ()> {
    // SAFETY: filename is a NUL-terminated argument supplied by the C runtime.
    let file = unsafe { fopen(filename, c"r".as_ptr()) };
    if file.is_null() {
        // SAFETY: stderr is the C runtime stream and filename is a C string.
        unsafe { fprintf(stderr, c"Failed to open: %s\n".as_ptr(), filename) };
        return Err(());
    }
    let input = Input(file);
    // Deliberately retain the unchecked seeks and signed-to-size_t conversion.
    // For a pipe, both ftell calls return -1 and only the first body chunk is read.
    let end = unsafe {
        fseek(input.0, 0, 2);
        let end = ftell(input.0) as usize;
        fseek(input.0, 0, 0);
        end
    };
    let mut buffer = [0u8; 1024];
    // SAFETY: the live input and writable buffer satisfy fgets's contract.
    if unsafe { fgets(buffer.as_mut_ptr().cast(), 1024, input.0) }.is_null() {
        return Err(());
    }
    let mut cursor = 0;
    let header = &buffer[..length(&buffer)];
    let prefix = field(header, &mut cursor).ok_or(())?;
    let last = field(header, &mut cursor).ok_or(())?;
    // Keep the host's C long overflow behavior and the original unsigned
    // comparison after strtol's result is assigned to an int.
    let last = unsafe { strtol(last.as_ptr().cast(), std::ptr::null_mut(), 16) } as u32;
    let mut offsets = Vec::new();
    let mut maximum = 0u32;
    loop {
        // SAFETY: the input remains open and fgets writes at most 1024 bytes.
        if unsafe { fgets(buffer.as_mut_ptr().cast(), 1024, input.0) }.is_null() {
            return Err(());
        }
        let len = length(&buffer);
        let done = unsafe { ftell(input.0) as usize } == end;
        // The C-locale ERE (0x[0-9a-fA-F]*) *([_a-zA-Z0-9]*) is
        // unanchored. Its optional trailing groups cannot affect capture 1.
        if let Some(start) = buffer[..len].windows(2).position(|pair| pair == b"0x") {
            let mut stop = start + 2;
            while stop < len && buffer[stop].is_ascii_hexdigit() {
                stop += 1;
            }
            buffer[stop] = 0;
            // SAFETY: this capture is a bounded, NUL-terminated substring.
            let offset = unsafe {
                strtol(buffer.as_ptr().add(start).cast(), std::ptr::null_mut(), 16)
            } as u32;
            offsets.try_reserve(1).map_err(|_| ())?;
            offsets.push(offset);
            maximum = maximum.max(offset);
        }
        if done {
            break;
        }
    }
    drop(input);
    maximum = maximum.max(last);
    let entries = ((maximum >> 2) + 31) / 32;
    let mut table = Vec::new();
    table.try_reserve_exact(entries as usize).map_err(|_| ())?;
    table.resize(entries as usize, u32::MAX);
    for offset in offsets {
        let index = (offset >> 2) / 32;
        let mask = 1u32 << ((offset >> 2) & 31);
        // A source whose final register falls beyond this allocation causes
        // an out-of-bounds C write. Reject that undefined input in Rust.
        *table.get_mut(index as usize).ok_or(())? ^= mask;
    }
    Ok((prefix, table))
}

#[no_mangle]
extern "C" fn main(argc: c_int, argv: *const *const c_char) -> c_int {
    // SAFETY: the C runtime supplies argc pointers followed by a null pointer,
    // and each actual argument is a NUL-terminated string valid through main.
    unsafe {
        if argc != 2 {
            fprintf(stderr, c"Usage: %s <authfile>\n".as_ptr(), *argv);
            return 1;
        }
        let filename = *argv.add(1);
        let Ok((prefix, table)) = parse(filename) else {
            fprintf(stderr, c"Failed to parse file %s\n".as_ptr(), filename);
            return -1;
        };
        printf(
            c"static const unsigned %s_reg_safe_bm[%d] = {\n".as_ptr(),
            prefix.as_ptr().cast::<c_char>(),
            table.len() as c_int,
        );
        for row in table.chunks(4) {
            for (index, word) in row.iter().enumerate() {
                printf(if index == 0 { c"\t" } else { c" " }.as_ptr());
                printf(c"0x%08X,".as_ptr(), *word);
            }
            printf(c"\n".as_ptr());
        }
        printf(c"};\n".as_ptr());
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
