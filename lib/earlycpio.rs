// SPDX-License-Identifier: GPL-2.0-only
//! Locate an uncompressed early-boot archive member without allocating.
// Copyright 2012 Intel Corporation; author H. Peter Anvin.

use kernel::bindings::{self, cpio_data, MAX_CPIO_FILE_NAME};
use kernel::ffi::{c_char, c_long, c_void};
#[cfg(CONFIG_PRINTK)]
use kernel::ffi::c_int;

const FIELDS: usize = 14;
const HEADER_LEN: usize = 8 * FIELDS - 2;
const MAGIC: usize = 0;
const MODE: usize = 2;
const FILESIZE: usize = 7;
const NAMESIZE: usize = 12;

#[cfg(CONFIG_PRINTK)]
const WARNING_MESSAGE: &[u8] = b"File %s exceeding MAX_CPIO_FILE_NAME [%d]\n\0";

#[cfg(CONFIG_PRINTK)]
const WARNING_FORMAT: [u8; bindings::KERN_WARNING.len() - 1 + WARNING_MESSAGE.len()] = {
    let mut format = [0; bindings::KERN_WARNING.len() - 1 + WARNING_MESSAGE.len()];
    let mut i = 0;
    while i + 1 < bindings::KERN_WARNING.len() {
        format[i] = bindings::KERN_WARNING[i];
        i += 1;
    }
    let mut j = 0;
    while j < WARNING_MESSAGE.len() {
        format[i + j] = WARNING_MESSAGE[j];
        j += 1;
    }
    format
};

#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
#[repr(transparent)]
struct PrintkIndexEntry(bindings::pi_entry);

// SAFETY: each pointer names a permanent immutable string or is null.
#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
unsafe impl Sync for PrintkIndexEntry {}

#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
#[repr(transparent)]
struct PrintkIndexPointer(*const bindings::pi_entry);

// SAFETY: the pointer names one permanent immutable index record.
#[cfg(all(CONFIG_PRINTK, CONFIG_PRINTK_INDEX))]
unsafe impl Sync for PrintkIndexPointer {}

#[cfg(CONFIG_PRINTK)]
unsafe fn warn_name(name: *const c_char) {
    #[cfg(CONFIG_PRINTK_INDEX)]
    {
        static ENTRY: PrintkIndexEntry = PrintkIndexEntry(bindings::pi_entry {
            fmt: WARNING_FORMAT.as_ptr().cast(),
            func: c"find_cpio_data".as_ptr().cast(),
            file: concat!(file!(), "\0").as_ptr().cast(),
            line: line!(),
            level: core::ptr::null(),
            subsys_fmt_prefix: core::ptr::null(),
        });
        #[used]
        #[link_section = ".printk_index"]
        static POINTER: PrintkIndexPointer = PrintkIndexPointer(&ENTRY.0);
    }
    // SAFETY: the fixed C format takes this readable C string and promoted int.
    unsafe { bindings::_printk(WARNING_FORMAT.as_ptr(), name, MAX_CPIO_FILE_NAME as c_int) };
}

/// Find the first regular archive member whose name starts with `path`.
///
/// The returned name omits the prefix and uses the original header's capacity.
/// A successful match updates a nonnull `nextoff`; failure leaves it untouched.
/// Parsing, diagnostics and the name copy perform no allocation.
///
/// # Safety
///
/// `path` must be a terminated C string. `data` must remain readable for `len`
/// bytes throughout the call. As in the original C interface, a matching name
/// must additionally satisfy the C string operations: up to the name capacity
/// bytes at its suffix must be readable, and a warned name must be terminated
/// for printk's `%s`. These obligations can extend beyond a malformed declared
/// name length. A nonnull `nextoff` must be writable and exclusively accessible.
#[no_mangle]
pub unsafe extern "C" fn find_cpio_data(
    path: *const c_char,
    data: *mut c_void,
    mut len: usize,
    nextoff: *mut c_long,
) -> cpio_data {
    let empty = cpio_data {
        data: core::ptr::null_mut(),
        size: 0,
        name: [0; MAX_CPIO_FILE_NAME as usize],
    };
    // SAFETY: the caller supplies a terminated, readable path.
    let path_size = unsafe { bindings::strlen(path) };
    let mut cursor = data.cast::<c_char>();

    while len > HEADER_LEN {
        // SAFETY: at least a complete header plus one byte remains readable.
        if unsafe { cursor.read() } == 0 {
            cursor = cursor.wrapping_add(4);
            len -= 4;
            continue;
        }
        let mut fields = [0u32; FIELDS];
        for (index, field) in fields.iter_mut().enumerate() {
            let digits = if index == MAGIC { 6 } else { 8 };
            for _ in 0..digits {
                // SAFETY: the complete fixed-size header is inside the buffer.
                let byte = unsafe { cursor.read() } as u8;
                cursor = cursor.wrapping_add(1);
                let decimal = byte.wrapping_sub(b'0');
                let hex = (byte | 0x20).wrapping_sub(b'a');
                let digit = if decimal < 10 {
                    decimal
                } else if hex < 6 {
                    hex + 10
                } else {
                    return empty;
                };
                *field = (*field << 4).wrapping_add(u32::from(digit));
            }
        }
        if fields[MAGIC].wrapping_sub(0x070701) > 1 {
            return empty;
        }
        len -= HEADER_LEN;

        // PTR_ALIGN aligns the absolute address, even for an unaligned input.
        // Wrapping operations permit inspection of overflow before any access;
        // `.add` would assert an in-allocation offset before the C checks run.
        let payload = cursor.wrapping_add(fields[NAMESIZE] as usize)
            .map_addr(|address| address.wrapping_add(3) & !3);
        let next = payload.wrapping_add(fields[FILESIZE] as usize)
            .map_addr(|address| address.wrapping_add(3) & !3);
        if next.addr() > cursor.wrapping_add(len).addr()
            || payload.addr() < cursor.addr()
            || next.addr() < payload.addr()
        {
            return empty;
        }

        if fields[MODE] & 0o170000 == 0o100000
            && fields[NAMESIZE] as usize >= path_size
            // SAFETY: validated name bounds include every compared prefix byte.
            && unsafe { bindings::memcmp(cursor.cast(), path.cast(), path_size) } == 0
        {
            if !nextoff.is_null() {
                // SAFETY: the caller provides exclusive writable long storage.
                unsafe { nextoff.write((next.addr() as c_long).wrapping_sub(data.addr() as c_long)) };
            }
            #[cfg(CONFIG_PRINTK)]
            if fields[NAMESIZE] as usize - path_size >= MAX_CPIO_FILE_NAME as usize {
                // SAFETY: a warned member satisfies printk's C string contract.
                unsafe { warn_name(cursor) };
            }
            let mut result = empty;
            // SAFETY: the destination is the actual header's name array. The
            // caller supplies the same readable suffix required by strscpy.
            // sized_strscpy is its real function, not an invented macro symbol.
            unsafe {
                bindings::sized_strscpy(result.name.as_mut_ptr(), cursor.wrapping_add(path_size),
                                       MAX_CPIO_FILE_NAME as usize);
            }
            result.data = payload.cast();
            result.size = fields[FILESIZE] as usize;
            return result;
        }
        len -= next.addr() - cursor.addr();
        cursor = next;
    }
    empty
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
