// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/hexdump.c
 */
//! Allocation-free hexadecimal conversion and byte-exact kernel dump formatting.
//!
//! C entry points retain forward, possibly overlapping buffer operations. Raw
//! accesses are confined to these wrappers; the formatter uses checked indices
//! and byte-access callbacks, without manufacturing aliased Rust references.

use core::ffi::{c_char, c_int, c_void};

/// Lower-case digits, including the C string terminator.
#[export_name = "hex_asc"]
pub static HEX_ASC: [u8; 17] = *b"0123456789abcdef\0";
/// Upper-case digits, including the C string terminator.
#[export_name = "hex_asc_upper"]
pub static HEX_ASC_UPPER: [u8; 17] = *b"0123456789ABCDEF\0";

/// Convert an ASCII hexadecimal digit, returning -1 for every other byte.
///
/// Cryptographic callers require that neither branches nor memory addresses
/// depend on the input. Keep the original unsigned-sign-mask arithmetic; do not
/// replace it with a lookup table or a conditional character classification.
#[no_mangle]
pub extern "C" fn hex_to_bin(ch: u8) -> c_int {
    let ch = i32::from(ch);
    let upper = ch & 0xdf;
    let digit = ch.wrapping_sub(i32::from(b'0') - 1)
        & (((ch.wrapping_sub(i32::from(b'9') + 1) & (i32::from(b'0') - 1).wrapping_sub(ch)) as u32)
            >> 8) as i32;
    let letter = upper.wrapping_sub(i32::from(b'A') - 11)
        & (((upper.wrapping_sub(i32::from(b'F') + 1) & (i32::from(b'A') - 1).wrapping_sub(upper))
            as u32)
            >> 8) as i32;
    (-1i32).wrapping_add(digit).wrapping_add(letter)
}

/// Decode count hexadecimal pairs, stopping before the first invalid pair.
///
/// # Safety
///
/// Every source byte read before success or the first invalid digit must be
/// readable, and every successfully decoded destination byte must be writable.
/// Buffers may overlap: writes affect subsequent reads, as in the C function.
/// Null pointers are permitted when count is zero.
#[no_mangle]
pub unsafe extern "C" fn hex2bin(mut dst: *mut u8, mut src: *const c_char, count: usize) -> c_int {
    for _ in 0..count {
        // SAFETY: the caller supplies the next readable digit. Do not read the
        // low digit when the high digit is invalid.
        let high = hex_to_bin(unsafe { src.read() } as u8);
        if high < 0 {
            return -22; // EINVAL
        }
        // SAFETY: this pair's second source byte is readable by the contract.
        let low = hex_to_bin(unsafe { src.add(1).read() } as u8);
        if low < 0 {
            return -22;
        }
        // SAFETY: only complete valid pairs write output. No slices exist, so
        // overlapping source and destination pointers remain valid.
        unsafe {
            dst.write(((high << 4) | low) as u8);
            dst = dst.add(1);
            src = src.add(2);
        }
    }
    0
}

/// Encode count bytes as lower-case digits, without a terminating NUL.
///
/// Returns the pointer immediately following the output.
///
/// # Safety
///
/// Each of count source bytes must be readable and 2 * count destination
/// bytes writable. Overlap follows the C function's forward read/write order.
/// Null pointers are permitted when count is zero.
#[no_mangle]
pub unsafe extern "C" fn bin2hex(
    mut dst: *mut c_char,
    src: *const c_void,
    count: usize,
) -> *mut c_char {
    let mut src = src.cast::<u8>();
    for _ in 0..count {
        // SAFETY: read this byte before its two output writes, including when
        // those writes overlap the next input byte.
        let byte = unsafe { src.read() };
        unsafe {
            dst.write(HEX_ASC[usize::from(byte >> 4)] as c_char);
            dst.add(1).write(HEX_ASC[usize::from(byte & 15)] as c_char);
            src = src.add(1);
            dst = dst.add(2);
        }
    }
    dst
}

struct Layout {
    length: usize,
    group: usize,
    ascii_column: usize,
    ascii: bool,
}

impl Layout {
    fn new(length: usize, rowsize: c_int, groupsize: c_int, ascii: bool) -> Self {
        let row = if rowsize == 32 { 32 } else { 16 };
        let length = length.min(row);
        let mut group = match groupsize {
            1 | 2 | 4 | 8 => groupsize as usize,
            _ => 1,
        };
        if length % group != 0 {
            group = 1;
        }
        Self {
            length,
            group,
            // On a 32-bit C unsigned long, INT_MIN is a power of two.
            // The original signed comparison then leaves it unchanged only
            // for empty input; integer row / INT_MIN is zero. Preserve this
            // defined zero-capacity ASCII return even on 32-bit targets.
            ascii_column: if cfg!(target_pointer_width = "32")
                && groupsize == c_int::MIN
                && length == 0
            {
                row * 2 + 1
            } else {
                row * 2 + row / group + 1
            },
            ascii,
        }
    }

    fn required(&self) -> c_int {
        if self.ascii {
            (self.ascii_column + self.length) as c_int
        } else {
            ((self.group * 2 + 1) * (self.length / self.group)) as c_int - 1
        }
    }
}

struct Output<W> {
    write: W,
    capacity: usize,
    position: usize,
}

impl<W: FnMut(usize, u8)> Output<W> {
    fn room(&self) -> bool {
        self.position + 1 < self.capacity
    }

    fn finish(&mut self) {
        (self.write)(self.position, 0);
    }

    fn put(&mut self, byte: u8) -> bool {
        if !self.room() {
            self.finish();
            return false;
        }
        (self.write)(self.position, byte);
        self.position += 1;
        true
    }
}

// Keeping reads and writes separate preserves the C API's overlap semantics,
// including ASCII's second pass over input already modified by earlier output.
// Safe slice callers can provide indexed closures; the C wrapper uses raw byte
// accesses rather than constructing simultaneously aliased slice references.
fn format_line(
    layout: Layout,
    capacity: usize,
    mut read: impl FnMut(usize) -> u8,
    write: impl FnMut(usize, u8),
) -> c_int {
    if capacity == 0 {
        return layout.required();
    }
    let mut output = Output {
        write,
        capacity,
        position: 0,
    };
    if layout.length == 0 {
        output.finish();
        return 0;
    }
    if layout.group == 1 {
        for index in 0..layout.length {
            if !output.room() {
                output.finish();
                return layout.required();
            }
            let byte = read(index);
            if !output.put(HEX_ASC[usize::from(byte >> 4)])
                || !output.put(HEX_ASC[usize::from(byte & 15)])
                || !output.put(b' ')
            {
                return layout.required();
            }
        }
        output.position -= 1; // Remove the final space, just like the byte loop.
    } else {
        for index in (0..layout.length).step_by(layout.group) {
            // snprintf receives a complete native-endian integer before it
            // starts writing. Snapshot only this group, not the whole input.
            let mut group = [0u8; 8];
            for (offset, byte) in group[..layout.group].iter_mut().enumerate() {
                *byte = read(index + offset);
            }
            if index != 0 && !output.put(b' ') {
                return layout.required();
            }
            for offset in 0..layout.group {
                let offset = if cfg!(target_endian = "little") {
                    layout.group - 1 - offset
                } else {
                    offset
                };
                let byte = group[offset];
                if !output.put(HEX_ASC[usize::from(byte >> 4)])
                    || !output.put(HEX_ASC[usize::from(byte & 15)])
                {
                    return layout.required();
                }
            }
            // Even an intermediate successful snprintf writes a NUL.
            output.finish();
        }
    }
    if layout.ascii {
        while output.position < layout.ascii_column {
            if !output.put(b' ') {
                return layout.required();
            }
        }
        for index in 0..layout.length {
            if !output.room() {
                output.finish();
                return layout.required();
            }
            let byte = read(index);
            let printable = if (b' '..=b'~').contains(&byte) {
                byte
            } else {
                b'.'
            };
            if !output.put(printable) {
                return layout.required();
            }
        }
    }
    output.finish();
    output.position as c_int
}

/// Format one 16- or 32-byte row, returning its length or required capacity.
///
/// The return value excludes the NUL. A nonempty output buffer is always
/// terminated; a zero-capacity buffer is untouched. The original empty-input,
/// zero-capacity return values (including -1 without ASCII) are retained.
///
/// # Safety
///
/// Input bytes actually visited by the selected row/group must be readable;
/// output bytes written within linebuflen must be writable. Buffers may
/// overlap. No input is read when linebuflen is zero, and no input is needed
/// when len is zero. A zero-capacity output pointer may be null.
#[no_mangle]
pub unsafe extern "C" fn hex_dump_to_buffer(
    buf: *const c_void,
    len: usize,
    rowsize: c_int,
    groupsize: c_int,
    linebuf: *mut c_char,
    linebuflen: usize,
    ascii: bool,
) -> c_int {
    let input = buf.cast::<u8>();
    format_line(
        Layout::new(len, rowsize, groupsize, ascii),
        linebuflen,
        // SAFETY: the formatter only requests input bytes in the bounded row,
        // and only output bytes within the supplied capacity.
        |index| unsafe { input.add(index).read() },
        |index, byte| unsafe { linebuf.add(index).write(byte as c_char) },
    )
}

#[cfg(all(CONFIG_PRINTK, CONFIG_RUST))]
use kernel::bindings::_printk;

#[cfg(all(CONFIG_PRINTK, not(CONFIG_RUST)))]
unsafe extern "C" {
    fn _printk(format: *const c_char, ...) -> c_int;
}

#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX, CONFIG_RUST))]
#[repr(transparent)]
struct PrintkIndexEntry(kernel::bindings::pi_entry);

// SAFETY: every field points at a static immutable C string or is null. The
// record is read-only for its entire lifetime, just like the C printk entries.
#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX, CONFIG_RUST))]
unsafe impl Sync for PrintkIndexEntry {}

#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX, CONFIG_RUST))]
#[repr(transparent)]
struct PrintkIndexPointer(*const kernel::bindings::pi_entry);

// SAFETY: the pointer names an immutable static PrintkIndexEntry.
#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX, CONFIG_RUST))]
unsafe impl Sync for PrintkIndexPointer {}

#[cfg(CONFIG_PRINTK)]
macro_rules! dump_printk {
    ($format:expr, $($argument:expr),* $(,)?) => {{
        #[cfg(all(CONFIG_PRINTK_INDEX, CONFIG_RUST))]
        {
            static ENTRY: PrintkIndexEntry = PrintkIndexEntry(kernel::bindings::pi_entry {
                fmt: $format.as_ptr().cast(),
                func: c"print_hex_dump".as_ptr().cast(),
                file: concat!(file!(), "\0").as_ptr().cast(),
                line: line!(),
                level: core::ptr::null(),
                subsys_fmt_prefix: core::ptr::null(),
            });
            #[used]
            #[link_section = ".printk_index"]
            static POINTER: PrintkIndexPointer = PrintkIndexPointer(&ENTRY.0);
        }
        // The caller supplies a literal format and the matching C arguments
        // in an unsafe block. Preserve the native variadic printk boundary.
        // Kernel bindings map char to u8 (-funsigned-char), whereas host C
        // may use i8. Pointer casts retain the same C string and machine ABI.
        _printk($format.as_ptr().cast(), $($argument),*)
    }};
}

/// Print all rows through the real kernel printk C ABI.
///
/// Prefix kinds are the linux/printk.h values: none = 0, address = 1,
/// offset = 2. Other values select no address/offset prefix.
///
/// # Safety
///
/// buf must supply len readable bytes. level and prefix_str must satisfy
/// the kernel printk %s contract. A zero length does not access any pointer.
#[cfg(CONFIG_PRINTK)]
#[no_mangle]
pub unsafe extern "C" fn print_hex_dump(
    level: *const c_char,
    prefix_str: *const c_char,
    prefix_type: c_int,
    rowsize: c_int,
    groupsize: c_int,
    buf: *const c_void,
    len: usize,
    ascii: bool,
) {
    let row = if rowsize == 32 { 32 } else { 16 };
    let mut offset = 0;
    let mut line = [0u8; 32 * 3 + 2 + 32 + 1];
    while offset < len {
        let length = (len - offset).min(row);
        // SAFETY: this bounded chunk lies in the caller's input allocation.
        let input = unsafe { buf.cast::<u8>().add(offset) };
        // Safe slice-based byte access for the nonoverlapping stack line.
        let bytes = unsafe { core::slice::from_raw_parts(input, length) };
        let capacity = line.len();
        format_line(
            Layout::new(length, row as c_int, groupsize, ascii),
            capacity,
            |index| bytes[index],
            |index, byte| line[index] = byte,
        );
        // SAFETY: each format string is terminated and all variadic arguments
        // have its exact C types. In particular, %.8x takes an unsigned int,
        // not a pointer-sized integer. printk owns address formatting/policy.
        unsafe {
            match prefix_type {
                1 => dump_printk!(
                    c"%s%s%p: %s\n",
                    level,
                    prefix_str,
                    input.cast::<c_void>(),
                    line.as_ptr().cast::<c_char>(),
                ),
                2 => dump_printk!(
                    c"%s%s%.8x: %s\n",
                    level,
                    prefix_str,
                    offset as core::ffi::c_uint,
                    line.as_ptr().cast::<c_char>(),
                ),
                _ => dump_printk!(
                    c"%s%s%s\n",
                    level,
                    prefix_str,
                    line.as_ptr().cast::<c_char>(),
                ),
            };
        }
        offset += length;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
