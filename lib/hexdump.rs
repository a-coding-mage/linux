// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/hexdump.c
 */

type SizeT = usize;
type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;

pub static hex_asc: [u8; 17] = *b"0123456789abcdef\0";
pub static hex_asc_upper: [u8; 17] = *b"0123456789ABCDEF\0";

const EINVAL: i32 = 22;

extern "C" {
    fn is_power_of_2(n: i32) -> bool;
    fn isascii(c: U8) -> bool;
    fn isprint(c: U8) -> bool;
    fn snprintf(s: *mut u8, n: SizeT, format: *const u8, ...) -> i32;
    fn printk(format: *const u8, ...);
    fn get_unaligned<T>(ptr: *const T) -> T;
    fn hex_asc_hi(x: U8) -> u8;
    fn hex_asc_lo(x: U8) -> u8;
}

/**
 * hex_to_bin - convert a hex digit to its real value
 * @ch: ascii character represents hex digit
 */
#[no_mangle]
pub unsafe extern "C" fn hex_to_bin(ch: U8) -> i32 {
    let cu = ch & 0xdf;
    -1 + ((((ch as i32) - b'0' as i32 + 1)
        & (((((ch as i32) - b'9' as i32 - 1)
            & (b'0' as i32 - 1 - ch as i32)) as u32) >> 8) as i32))
        + ((((cu as i32) - b'A' as i32 + 11)
            & (((((cu as i32) - b'F' as i32 - 1)
                & (b'A' as i32 - 1 - cu as i32)) as u32) >> 8) as i32))
}

/** Convert an ascii hexadecimal string to its binary representation. */
#[no_mangle]
pub unsafe extern "C" fn hex2bin(mut dst: *mut U8, mut src: *const u8, mut count: SizeT) -> i32 {
    while count != 0 {
        count -= 1;
        let hi = hex_to_bin(*src);
        src = src.add(1);
        if hi < 0 { return -EINVAL; }
        let lo = hex_to_bin(*src);
        src = src.add(1);
        if lo < 0 { return -EINVAL; }
        *dst = ((hi << 4) | lo) as U8;
        dst = dst.add(1);
    }
    0
}

/** Convert binary data to an ascii hexadecimal string. */
#[no_mangle]
pub unsafe extern "C" fn bin2hex(mut dst: *mut u8, src: *const core::ffi::c_void, mut count: SizeT) -> *mut u8 {
    let mut src = src as *const U8;
    while count != 0 {
        count -= 1;
        let ch = *src;
        src = src.add(1);
        *dst = hex_asc_hi(ch); dst = dst.add(1);
        *dst = hex_asc_lo(ch); dst = dst.add(1);
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn hex_dump_to_buffer(
    buf: *const core::ffi::c_void, mut len: SizeT, mut rowsize: i32, mut groupsize: i32,
    linebuf: *mut u8, linebuflen: SizeT, ascii: bool,
) -> i32 {
    let ptr = buf as *const U8;
    let mut lx: SizeT = 0;
    let mut ngroups: i32;
    let mut ascii_column: SizeT;
    let mut ret: i32;
    if rowsize != 16 && rowsize != 32 { rowsize = 16; }
    if len > rowsize as SizeT { len = rowsize as SizeT; }
    if !is_power_of_2(groupsize) || groupsize > 8 { groupsize = 1; }
    if len % groupsize as SizeT != 0 { groupsize = 1; }
    ngroups = (len / groupsize as SizeT) as i32;
    ascii_column = (rowsize * 2 + rowsize / groupsize + 1) as SizeT;
    if linebuflen == 0 { return if ascii { ascii_column as i32 + len as i32 } else { (groupsize * 2 + 1) * ngroups - 1 }; }
    if len == 0 { *linebuf = 0; return 0; }
    if groupsize == 8 {
        let p = buf as *const U64;
        for j in 0..ngroups {
            let fmt = b"%s%16.16llx\0";
            ret = snprintf(linebuf.add(lx), linebuflen - lx, fmt.as_ptr(), if j != 0 { b" \0".as_ptr() } else { b"\0".as_ptr() }, get_unaligned(p.add(j as usize)));
            if ret >= (linebuflen - lx) as i32 { return if ascii { ascii_column as i32 + len as i32 } else { (groupsize * 2 + 1) * ngroups - 1 }; }
            lx += ret as SizeT;
        }
    } else if groupsize == 4 || groupsize == 2 {
        let width = if groupsize == 4 { b"%s%8.8x\0" } else { b"%s%4.4x\0" };
        let step = groupsize as usize;
        for j in 0..ngroups as usize {
            let value = if groupsize == 4 { get_unaligned((buf as *const U32).add(j)) as u64 } else { get_unaligned((buf as *const U16).add(j)) as u64 };
            ret = snprintf(linebuf.add(lx), linebuflen - lx, width.as_ptr(), if j != 0 { b" \0".as_ptr() } else { b"\0".as_ptr() }, value);
            if ret >= (linebuflen - lx) as i32 { return if ascii { ascii_column as i32 + len as i32 } else { (groupsize * 2 + 1) * ngroups - 1 }; }
            lx += ret as SizeT;
            let _ = step;
        }
    } else {
        for j in 0..len {
            if linebuflen < lx + 2 { *linebuf.add(lx) = 0; return if ascii { ascii_column as i32 + len as i32 } else { (groupsize * 2 + 1) * ngroups - 1 }; }
            let ch = *ptr.add(j); *linebuf.add(lx) = hex_asc_hi(ch); lx += 1;
            if linebuflen < lx + 2 { *linebuf.add(lx) = 0; return if ascii { ascii_column as i32 + len as i32 } else { (groupsize * 2 + 1) * ngroups - 1 }; }
            *linebuf.add(lx) = hex_asc_lo(ch); lx += 1;
            if linebuflen < lx + 2 { *linebuf.add(lx) = 0; return if ascii { ascii_column as i32 + len as i32 } else { (groupsize * 2 + 1) * ngroups - 1 }; }
            *linebuf.add(lx) = b' '; lx += 1;
        }
        if len != 0 { lx -= 1; }
    }
    if ascii {
        while lx < ascii_column { if linebuflen < lx + 2 { *linebuf.add(lx) = 0; return ascii_column as i32 + len as i32; } *linebuf.add(lx) = b' '; lx += 1; }
        for j in 0..len { if linebuflen < lx + 2 { *linebuf.add(lx) = 0; return ascii_column as i32 + len as i32; } let ch = *ptr.add(j); *linebuf.add(lx) = if isascii(ch) && isprint(ch) { ch } else { b'.' }; lx += 1; }
    }
    *linebuf.add(lx) = 0; lx as i32
}

#[cfg(CONFIG_PRINTK)]
#[no_mangle]
pub unsafe extern "C" fn print_hex_dump(level: *const u8, prefix_str: *const u8, prefix_type: i32, mut rowsize: i32, groupsize: i32, buf: *const core::ffi::c_void, len: SizeT, ascii: bool) {
    let ptr = buf as *const U8;
    let mut remaining = len as i32;
    let mut linebuf = [0u8; 32 * 3 + 2 + 32 + 1];
    if rowsize != 16 && rowsize != 32 { rowsize = 16; }
    let mut i = 0;
    while i < len { let linelen = core::cmp::min(remaining, rowsize); remaining -= rowsize; hex_dump_to_buffer(ptr.add(i), linelen as SizeT, rowsize, groupsize, linebuf.as_mut_ptr(), linebuf.len(), ascii); match prefix_type { 0 => printk(b"%s%s%p: %s\n\0".as_ptr(), level, prefix_str, ptr.add(i), linebuf.as_ptr()), 1 => printk(b"%s%s%.8x: %s\n\0".as_ptr(), level, prefix_str, i, linebuf.as_ptr()), _ => printk(b"%s%s%s\n\0".as_ptr(), level, prefix_str, linebuf.as_ptr()) }; i += rowsize as usize; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
