// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_void};

// Declarations supplied by the Linux/RISC-V environment.
extern "C" {
    fn fdt_path_offset(fdt: *const c_void, path: *const c_char) -> c_int;
    fn fdt_getprop_w(fdt: *mut c_void, node: c_int, name: *const c_char,
                     lenp: *mut c_int) -> *mut u64;
    fn fdt_getprop(fdt: *const c_void, node: c_int, name: *const c_char,
                   lenp: *mut c_int) -> *const c_void;
    fn fdt_get_name(fdt: *const c_void, node: c_int, lenp: *mut c_int) -> *const c_char;
    fn fdt_stringlist_contains(str: *const c_void, len: c_int, list: *const c_char) -> c_int;
    fn fdt_first_subnode(fdt: *const c_void, offset: c_int) -> c_int;
    fn fdt_next_subnode(fdt: *const c_void, offset: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn memchr(s: *const c_void, c: c_int, n: usize) -> *const c_void;
    fn strchr(s: *const c_char, c: c_int) -> *const c_char;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *const c_char;
    fn tolower(c: c_int) -> c_int;
}

const SATP_MODE_39: u64 = 8 << 60;
const SATP_MODE_48: u64 = 9 << 60;

pub unsafe fn get_kaslr_seed(dtb_pa: usize) -> u64 {
    let node;
    let mut len = 0;
    let prop;
    let ret;

    node = fdt_path_offset(dtb_pa as *const c_void, b"/chosen\0".as_ptr() as *const c_char);
    if node < 0 { return 0; }

    prop = fdt_getprop_w(dtb_pa as *mut c_void, node, b"kaslr-seed\0".as_ptr() as *const c_char, &mut len);
    if prop.is_null() || len as usize != core::mem::size_of::<u64>() { return 0; }

    ret = u64::from_be(prop.read());
    prop.write(0);
    ret
}

/// fdt_device_is_available - check if a device is available for use
unsafe fn fdt_device_is_available(fdt: *const c_void, node: c_int) -> bool {
    let mut statlen = 0;
    let status = fdt_getprop(fdt, node, b"status\0".as_ptr() as *const c_char, &mut statlen) as *const c_char;
    if status.is_null() { return true; }
    if statlen > 0 && (strcmp(status, b"okay\0".as_ptr() as *const c_char) == 0 ||
        statlen > 0 && strcmp(status, b"ok\0".as_ptr() as *const c_char) == 0) { return true; }
    false
}

// Copy of fdt_nodename_eq_.
unsafe fn fdt_node_name_eq(fdt: *const c_void, offset: c_int, s: *const c_char) -> c_int {
    let len = strlen(s);
    let mut olen = 0;
    let p = fdt_get_name(fdt, offset, &mut olen);
    if p.is_null() || olen < len as c_int || memcmp(p as *const c_void, s as *const c_void, len) != 0 { return 0; }
    if *p.add(len) == 0 || (memchr(s as *const c_void, b'@' as c_int, len).is_null() && *p.add(len) == b'@' as c_char) { 1 } else { 0 }
}

unsafe fn isa_string_contains(mut isa_str: *const c_char, ext_name: *const c_char) -> bool {
    let len = strlen(ext_name);
    if strlen(isa_str) < 4 { return false; }
    if len == 1 {
        let single_end = strcspn(isa_str, b"sSxXzZ\0".as_ptr() as *const c_char);
        for i in 4..single_end { if tolower(*isa_str.add(i) as c_int) as c_char == *ext_name { return true; } }
        return false;
    }
    isa_str = strpbrk(isa_str, b"sSxXzZ\0".as_ptr() as *const c_char);
    while !isa_str.is_null() {
        if strncasecmp(isa_str, ext_name, len) == 0 {
            let ext_end = *isa_str.add(len);
            if ext_end == 0 || ext_end == b'_' as c_char { return true; }
        }
        isa_str = strchr(isa_str, b'_' as c_int);
        if !isa_str.is_null() { isa_str = isa_str.add(1); }
    }
    false
}

unsafe fn early_cpu_isa_ext_available(fdt: *const c_void, node: c_int, ext_name: *const c_char) -> bool {
    let mut len = 0;
    let prop = fdt_getprop(fdt, node, b"riscv,isa-extensions\0".as_ptr() as *const c_char, &mut len);
    if !prop.is_null() && fdt_stringlist_contains(prop, len, ext_name) != 0 { return true; }
    let prop = fdt_getprop(fdt, node, b"riscv,isa\0".as_ptr() as *const c_char, &mut len);
    !prop.is_null() && isa_string_contains(prop as *const c_char, ext_name)
}

pub unsafe fn fdt_early_match_extension_isa(fdt: *const c_void, ext_name: *const c_char) -> bool {
    let parent = fdt_path_offset(fdt, b"/cpus\0".as_ptr() as *const c_char);
    if parent < 0 { return false; }
    let mut node = fdt_first_subnode(fdt, parent);
    let mut ret = false;
    while node >= 0 {
        if fdt_node_name_eq(fdt, node, b"cpu\0".as_ptr() as *const c_char) != 0 && fdt_device_is_available(fdt, node) {
            if !early_cpu_isa_ext_available(fdt, node, ext_name) { return false; }
            ret = true;
        }
        node = fdt_next_subnode(fdt, node);
    }
    ret
}

pub unsafe fn set_satp_mode_from_fdt(dtb_pa: usize) -> u64 {
    let fdt = dtb_pa as *const c_void;
    let parent = fdt_path_offset(fdt, b"/cpus\0".as_ptr() as *const c_char);
    if parent < 0 { return 0; }
    let mut node = fdt_first_subnode(fdt, parent);
    while node >= 0 {
        if fdt_node_name_eq(fdt, node, b"cpu\0".as_ptr() as *const c_char) != 0 && fdt_device_is_available(fdt, node) {
            let mmu_type = fdt_getprop(fdt, node, b"mmu-type\0".as_ptr() as *const c_char, core::ptr::null_mut()) as *const c_char;
            if mmu_type.is_null() { break; }
            if strcmp(mmu_type, b"riscv,sv39\0".as_ptr() as *const c_char) == 0 { return SATP_MODE_39; }
            if strcmp(mmu_type, b"riscv,sv48\0".as_ptr() as *const c_char) == 0 { return SATP_MODE_48; }
            break;
        }
        node = fdt_next_subnode(fdt, node);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
