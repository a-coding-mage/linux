// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct region {
    pub start: u32,
    pub off: u32,
    pub group_len: u32,
    pub end: u32,
    pub nbits: u32,
}

extern "C" {
    fn memdup_user_nul(ubuf: *const core::ffi::c_char, ulen: u32) -> *mut core::ffi::c_char;
    fn bitmap_zero(maskp: *mut usize, nbits: u32);
    fn bitmap_set(maskp: *mut usize, start: u32, nbits: u32);
    fn bitmap_clear(maskp: *mut usize, start: i32, nbits: i32);
    fn find_next_bit(addr: *const usize, size: i32, offset: i32) -> i32;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn bitmap_format_to_buf(list: bool, buf: *mut core::ffi::c_char, maskp: *const usize, nmaskbits: i32, off: i64, count: usize) -> isize;
}

pub unsafe fn bitmap_parse_user(ubuf: *const core::ffi::c_char, ulen: u32, maskp: *mut usize, nmaskbits: i32) -> i32 {
    let buf = memdup_user_nul(ubuf, ulen);
    if buf.is_null() { return -12; }
    let ret = bitmap_parse(buf, u32::MAX, maskp, nmaskbits);
    kfree(buf as *mut core::ffi::c_void);
    ret
}

unsafe fn bitmap_print_to_buf(list: bool, buf: *mut core::ffi::c_char, maskp: *const usize, nmaskbits: i32, off: i64, count: usize) -> i32 {
    bitmap_format_to_buf(list, buf, maskp, nmaskbits, off, count) as i32
}

pub unsafe fn bitmap_print_bitmask_to_buf(buf: *mut core::ffi::c_char, maskp: *const usize, nmaskbits: i32, off: i64, count: usize) -> i32 {
    bitmap_print_to_buf(false, buf, maskp, nmaskbits, off, count)
}

pub unsafe fn bitmap_print_list_to_buf(buf: *mut core::ffi::c_char, maskp: *const usize, nmaskbits: i32, off: i64, count: usize) -> i32 {
    bitmap_print_to_buf(true, buf, maskp, nmaskbits, off, count)
}

unsafe fn bitmap_set_region(r: *const region, bitmap: *mut usize) {
    let mut start = (*r).start;
    while start <= (*r).end {
        bitmap_set(bitmap, start, core::cmp::min((*r).end - start + 1, (*r).off));
        start = start.wrapping_add((*r).group_len);
    }
}

unsafe fn bitmap_check_region(r: *const region) -> i32 {
    if (*r).start > (*r).end || (*r).group_len == 0 || (*r).off > (*r).group_len { return -22; }
    if (*r).end >= (*r).nbits { return -34; }
    0
}

pub unsafe fn bitmap_parselist(_buf: *const core::ffi::c_char, maskp: *mut usize, nmaskbits: i32) -> i32 {
    bitmap_zero(maskp, nmaskbits as u32);
    let mut p = _buf;
    while !p.is_null() && *p != 0 && *p != b'\n' as i8 {
        while *p == b',' as i8 || (*p as u8).is_ascii_whitespace() { p = p.add(1); }
        if *p == 0 || *p == b'\n' as i8 { break; }
        let mut a = 0u32;
        while (*p as u8).is_ascii_digit() { a = a * 10 + (*p as u32 - b'0' as u32); p = p.add(1); }
        let mut b = a;
        if *p == b'-' as i8 { p = p.add(1); b = 0; while (*p as u8).is_ascii_digit() { b = b * 10 + (*p as u32 - b'0' as u32); p = p.add(1); } }
        let mut off = b - a + 1; let mut group = off;
        if *p == b':' as i8 { p = p.add(1); off = 0; while (*p as u8).is_ascii_digit() { off = off * 10 + (*p as u32 - b'0' as u32); p = p.add(1); } if *p != b'/' as i8 { return -22; } p = p.add(1); group = 0; while (*p as u8).is_ascii_digit() { group = group * 10 + (*p as u32 - b'0' as u32); p = p.add(1); } }
        let r = region { start: a, off, group_len: group, end: b, nbits: nmaskbits as u32 };
        let ret = bitmap_check_region(&r); if ret != 0 { return ret; } bitmap_set_region(&r, maskp);
    }
    0
}

pub unsafe fn bitmap_parselist_user(ubuf: *const core::ffi::c_char, ulen: u32, maskp: *mut usize, nmaskbits: i32) -> i32 {
    let buf = memdup_user_nul(ubuf, ulen);
    if buf.is_null() { return -12; }
    let ret = bitmap_parselist(buf, maskp, nmaskbits);
    kfree(buf as *mut core::ffi::c_void);
    ret
}

pub unsafe fn bitmap_parse(start: *const core::ffi::c_char, buflen: u32, maskp: *mut usize, nmaskbits: i32) -> i32 {
    bitmap_zero(maskp, nmaskbits as u32); let mut end = start; while (end.offset_from(start) as u32) < buflen && *end != 0 && *end != b'\n' as i8 { end = end.add(1); }
    let mut chunk = 0usize; while end > start { while end > start && (*end.offset(-1) == b',' as i8 || (*end.offset(-1) as u8).is_ascii_whitespace()) { end = end.offset(-1); } if end == start { break; }
        let mut value = 0u32; let mut n = 0; while end > start && n < 8 { let c = *end.offset(-1) as u8; let d = match c { b'0'..=b'9' => c-b'0', b'a'..=b'f' => c-b'a'+10, b'A'..=b'F' => c-b'A'+10, _ => break }; value |= (d as u32) << (n*4); n += 1; end = end.offset(-1); } if n == 0 { return -22; } *(maskp as *mut u32).add(chunk) = value; chunk += 1; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
