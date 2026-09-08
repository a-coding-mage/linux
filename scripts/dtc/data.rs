// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation.  2005.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct marker {
    pub offset: u32,
    pub type_: markertype,
    pub ref_: *mut c_char,
    pub next: *mut marker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data {
    pub val: *mut c_char,
    pub len: c_int,
    pub markers: *mut marker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_reserve_entry {
    pub address: u64,
    pub size: u64,
}

pub type cell_t = u32;
pub type fdt16_t = u16;
pub type fdt32_t = u32;
pub type fdt64_t = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum markertype {
    TYPE_NONE = 0,
    TYPE_STRING,
}

extern "C" {
    static empty_data: data;
    fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_char;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xstrdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn get_escape_char(s: *const c_char, i: *mut c_int) -> c_char;
    fn die(fmt: *const c_char, ...);
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut c_void) -> usize;
    fn feof(stream: *mut c_void) -> c_int;
    fn ferror(stream: *mut c_void) -> c_int;
    fn strerror(errnum: c_int) -> *const c_char;
    static mut errno: c_int;
    fn cpu_to_fdt16(value: u64) -> fdt16_t;
    fn cpu_to_fdt32(value: u64) -> fdt32_t;
    fn cpu_to_fdt64(value: u64) -> fdt64_t;
}

#[inline]
pub unsafe fn data_free(mut d: data) {
    let mut m = d.markers;
    while !m.is_null() {
        let nm = (*m).next;
        free((*m).ref_ as *mut c_void);
        free(m as *mut c_void);
        m = nm;
    }
    if !d.val.is_null() {
        free(d.val as *mut c_void);
    }
}

pub unsafe fn data_grow_for(d: data, xlen: u32) -> data {
    if xlen == 0 { return d; }
    let mut nd = d;
    let mut newsize = xlen;
    while (d.len as u32).wrapping_add(xlen) > newsize { newsize = newsize.wrapping_mul(2); }
    nd.val = xrealloc(d.val as *mut c_void, newsize as usize);
    nd
}

pub unsafe fn data_copy_mem(mem: *const c_char, len: c_int) -> data {
    let mut d = data_grow_for(*empty_data, len as u32);
    d.len = len;
    memcpy(d.val as *mut c_void, mem as *const c_void, len as usize);
    d
}

pub unsafe fn data_copy_escape_string(s: *const c_char, len: c_int) -> data {
    let mut i = 0;
    let mut d = data_add_marker(*empty_data, markertype::TYPE_STRING, ptr::null_mut());
    d = data_grow_for(d, (len + 1) as u32);
    let q = d.val;
    while i < len {
        let mut c = *s.add(i as usize);
        i += 1;
        if c == b'\\' as c_char { c = get_escape_char(s, &mut i); }
        *q.add(d.len as usize) = c;
        d.len += 1;
    }
    *q.add(d.len as usize) = 0;
    d.len += 1;
    d
}

pub unsafe fn data_copy_file(f: *mut c_void, maxlen: usize) -> data {
    let mut d = data_add_marker(*empty_data, markertype::TYPE_NONE, ptr::null_mut());
    while feof(f) == 0 && (d.len as usize) < maxlen {
        let chunksize = if maxlen == usize::MAX { 4096 } else { maxlen - d.len as usize };
        d = data_grow_for(d, chunksize as u32);
        let ret = fread(d.val.add(d.len as usize) as *mut c_void, 1, chunksize, f);
        if ferror(f) != 0 { die(b"Error reading file into data: %s\0".as_ptr() as *const c_char, strerror(errno)); }
        if (d.len as usize).wrapping_add(ret) < d.len as usize { die(b"Overflow reading file into data\n\0".as_ptr() as *const c_char); }
        d.len += ret as c_int;
    }
    d
}

pub unsafe fn data_append_data(mut d: data, p: *const c_void, len: c_int) -> data {
    d = data_grow_for(d, len as u32);
    memcpy(d.val.add(d.len as usize) as *mut c_void, p, len as usize);
    d.len += len;
    d
}

pub unsafe fn data_insert_at_marker(mut d: data, mut m: *mut marker, p: *const c_void, len: c_int) -> data {
    d = data_grow_for(d, len as u32);
    memmove(d.val.add((*m).offset as usize + len as usize) as *mut c_void, d.val.add((*m).offset as usize) as *const c_void, (d.len as u32 - (*m).offset) as usize);
    memcpy(d.val.add((*m).offset as usize) as *mut c_void, p, len as usize);
    d.len += len;
    m = (*m).next;
    while !m.is_null() { (*m).offset += len as u32; m = (*m).next; }
    d
}

unsafe fn data_append_markers(mut d: data, m: *mut marker) -> data {
    let mut mp = &mut d.markers;
    while !(*mp).is_null() { mp = &mut (**mp).next; }
    *mp = m;
    d
}

pub unsafe fn data_merge(mut d1: data, mut d2: data) -> data {
    let m2 = d2.markers;
    let mut d = data_append_markers(data_append_data(d1, d2.val as *const c_void, d2.len), m2);
    let mut m = m2;
    while !m.is_null() { (*m).offset += d1.len as u32; m = (*m).next; }
    d2.markers = ptr::null_mut();
    data_free(d2);
    d
}

pub unsafe fn data_append_integer(d: data, value: u64, bits: c_int) -> data {
    match bits {
        8 => { let v = value as u8; data_append_data(d, &v as *const u8 as *const c_void, 1) }
        16 => { let v = cpu_to_fdt16(value); data_append_data(d, &v as *const fdt16_t as *const c_void, 2) }
        32 => { let v = cpu_to_fdt32(value); data_append_data(d, &v as *const fdt32_t as *const c_void, 4) }
        64 => { let v = cpu_to_fdt64(value); data_append_data(d, &v as *const fdt64_t as *const c_void, 8) }
        _ => { die(b"Invalid literal size (%d)\n\0".as_ptr() as *const c_char, bits); d }
    }
}

pub unsafe fn data_append_re(d: data, address: u64, size: u64) -> data {
    let re = fdt_reserve_entry { address: cpu_to_fdt64(address), size: cpu_to_fdt64(size) };
    data_append_data(d, &re as *const fdt_reserve_entry as *const c_void, size_of::<fdt_reserve_entry>() as c_int)
}

pub unsafe fn data_append_cell(d: data, word: cell_t) -> data { data_append_integer(d, word as u64, (size_of::<cell_t>() * 8) as c_int) }
pub unsafe fn data_append_addr(d: data, addr: u64) -> data { data_append_integer(d, addr, (size_of::<u64>() * 8) as c_int) }
pub unsafe fn data_append_byte(d: data, byte: u8) -> data { data_append_data(d, &byte as *const u8 as *const c_void, 1) }

pub unsafe fn data_append_zeroes(mut d: data, len: c_int) -> data {
    d = data_grow_for(d, len as u32);
    memset(d.val.add(d.len as usize) as *mut c_void, 0, len as usize);
    d.len += len;
    d
}

pub unsafe fn data_append_align(d: data, align: c_int) -> data {
    let newlen = ((d.len + align - 1) / align) * align;
    data_append_zeroes(d, newlen - d.len)
}

pub unsafe fn data_add_marker(d: data, type_: markertype, ref_: *mut c_char) -> data {
    data_append_markers(d, alloc_marker(d.len as u32, type_, ref_))
}

pub unsafe fn data_is_one_string(d: data) -> bool {
    if d.len == 0 { return false; }
    for i in 0..d.len - 1 { if *d.val.add(i as usize) == 0 { return false; } }
    *d.val.add((d.len - 1) as usize) == 0
}

pub unsafe fn data_insert_data(mut d: data, mut m: *mut marker, old: data) -> data {
    let offset = (*m).offset;
    let next = (*m).next;
    let mut new_data = data_insert_at_marker(d, m, old.val as *const c_void, old.len);
    let mut marker = old.markers;
    while !marker.is_null() {
        let ref_ = if !(*marker).ref_.is_null() { xstrdup((*marker).ref_) } else { ptr::null_mut() };
        (*m).next = alloc_marker((*marker).offset + offset, (*marker).type_, ref_);
        m = (*m).next;
        marker = (*marker).next;
    }
    (*m).next = next;
    new_data
}

pub unsafe fn alloc_marker(offset: u32, type_: markertype, ref_: *mut c_char) -> *mut marker {
    let m = xmalloc(size_of::<marker>()) as *mut marker;
    (*m).offset = offset;
    (*m).type_ = type_;
    (*m).ref_ = ref_;
    (*m).next = ptr::null_mut();
    m
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
