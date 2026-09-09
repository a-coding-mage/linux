// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Translated from yamon-dt.c. Kernel and firmware dependencies are supplied externally.

const MAX_MEM_ARRAY_ENTRIES: usize = 2;

extern "C" {
    fn fdt_path_offset(fdt: *mut core::ffi::c_void, path: *const core::ffi::c_char) -> i32;
    fn fdt_add_subnode(fdt: *mut core::ffi::c_void, parentoffset: i32, name: *const core::ffi::c_char) -> i32;
    fn fdt_setprop_string(fdt: *mut core::ffi::c_void, nodeoffset: i32, name: *const core::ffi::c_char, value: *const core::ffi::c_char) -> i32;
    fn fdt_setprop(fdt: *mut core::ffi::c_void, nodeoffset: i32, name: *const core::ffi::c_char, val: *const core::ffi::c_void, len: i32) -> i32;
    fn fw_getcmdline() -> *mut core::ffi::c_char;
    fn fw_getenv(name: *const core::ffi::c_char) -> *mut core::ffi::c_char;
    fn kstrtoul(s: *const core::ffi::c_char, base: u32, res: *mut usize) -> i32;
    fn memparse(ptr: *const core::ffi::c_char, retptr: *mut *mut core::ffi::c_char) -> usize;
    static mut arcs_cmdline: *mut core::ffi::c_char;
}

#[repr(C)]
pub struct yamon_mem_region {
    pub start: u32,
    pub size: usize,
    pub discard: usize,
}

const FDT_ERR_NOTFOUND: i32 = 1; // Supplied by libfdt; value is resolved by the kernel build.

unsafe fn gen_fdt_mem_array(
    regions: *const yamon_mem_region,
    mut mem_array: *mut u32,
    max_entries: u32,
    mut memsize: usize,
) -> u32 {
    let mut mr = regions;
    let mut entries = 0;
    while (*mr).size != 0 && memsize != 0 {
        if entries >= max_entries { break; }
        let size = core::cmp::min(memsize, (*mr).size);
        memsize -= size;
        *mem_array = (*mr).start.to_be(); mem_array = mem_array.add(1);
        *mem_array = (size as u32).to_be(); mem_array = mem_array.add(1);
        entries += 1;
        memsize -= core::cmp::min(memsize, (*mr).discard);
        mr = mr.add(1);
    }
    entries
}

pub unsafe fn yamon_dt_append_cmdline(fdt: *mut core::ffi::c_void) -> i32 {
    let mut chosen_off = fdt_path_offset(fdt, b"/chosen\0".as_ptr() as _);
    if chosen_off == -FDT_ERR_NOTFOUND { chosen_off = fdt_add_subnode(fdt, 0, b"chosen\0".as_ptr() as _); }
    if chosen_off < 0 { return chosen_off; }
    let err = fdt_setprop_string(fdt, chosen_off, b"bootargs\0".as_ptr() as _, fw_getcmdline());
    if err != 0 { return err; }
    0
}

pub unsafe fn yamon_dt_append_memory(fdt: *mut core::ffi::c_void, regions: *const yamon_mem_region) -> i32 {
    let mut phys_memsize = 0usize;
    let mut memsize;
    let mut mem_array = [0u32; 2 * MAX_MEM_ARRAY_ENTRIES];
    let names = [b"ememsize\0".as_ptr() as *const i8, b"memsize\0".as_ptr() as *const i8];
    for name in names { let var = fw_getenv(name); if !var.is_null() && kstrtoul(var, 0, &mut phys_memsize) == 0 { break; } }
    if phys_memsize == 0 { phys_memsize = 32usize << 20; }
    memsize = phys_memsize;
    for name in names {
        let mut found = arcs_cmdline;
        while !found.is_null() { if *found == *name { memsize = memparse(found.add(9), core::ptr::null_mut()); break; } found = found.add(1); }
    }
    phys_memsize = core::cmp::max(phys_memsize, memsize);
    let mut mem_off = fdt_path_offset(fdt, b"/memory\0".as_ptr() as _);
    if mem_off == -FDT_ERR_NOTFOUND { mem_off = fdt_add_subnode(fdt, 0, b"memory\0".as_ptr() as _); }
    if mem_off < 0 { return mem_off; }
    let mut entries = gen_fdt_mem_array(regions, mem_array.as_mut_ptr(), MAX_MEM_ARRAY_ENTRIES as u32, phys_memsize);
    let mut err = fdt_setprop(fdt, mem_off, b"reg\0".as_ptr() as _, mem_array.as_ptr() as _, (entries * 8) as i32);
    if err != 0 { return err; }
    entries = gen_fdt_mem_array(regions, mem_array.as_mut_ptr(), MAX_MEM_ARRAY_ENTRIES as u32, memsize);
    err = fdt_setprop(fdt, mem_off, b"linux,usable-memory\0".as_ptr() as _, mem_array.as_ptr() as _, (entries * 8) as i32);
    err
}

pub unsafe fn yamon_dt_serial_config(fdt: *mut core::ffi::c_void) -> i32 {
    let tty = fw_getenv(b"yamontty\0".as_ptr() as _);
    let uart = if tty.is_null() || core::ffi::CStr::from_ptr(tty).to_bytes() == b"tty0" { 0 } else { 1 };
    let mut baud = 38400u32; let parity = b'n'; let stop_bits = 8u8;
    let mut path = [0i8; 20];
    let s = format!("serial{}:{}{}{}", uart, baud, parity as char, stop_bits);
    for (d, b) in path.iter_mut().zip(s.bytes()) { *d = b as i8; }
    let mut chosen_off = fdt_path_offset(fdt, b"/chosen\0".as_ptr() as _);
    if chosen_off == -FDT_ERR_NOTFOUND { chosen_off = fdt_add_subnode(fdt, 0, b"chosen\0".as_ptr() as _); }
    if chosen_off < 0 { return chosen_off; }
    fdt_setprop_string(fdt, chosen_off, b"stdout-path\0".as_ptr() as _, path.as_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
