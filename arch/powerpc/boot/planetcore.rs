// SPDX-License-Identifier: GPL-2.0-only
/*
 * PlanetCore configuration data support functions
 *
 * Author: Scott Wood <scottwood@freescale.com>
 *
 * Copyright (c) 2007 Freescale Semiconductor, Inc.
 */

use core::ffi::{c_char, c_void};

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: i32) -> u64;
    fn dt_fixup_mac_address(index: u32, address: *mut u8);
    fn find_node_by_prop_value_str(node: *mut c_void, prop: *const c_char,
                                   value: *const c_char) -> *mut c_void;
    fn get_path(node: *mut c_void, buf: *mut c_char, len: usize) -> *mut c_char;
    fn finddevice(path: *const c_char) -> *mut c_void;
    fn create_node(parent: *mut c_void, name: *const c_char) -> *mut c_void;
    fn setprop_str(node: *mut c_void, name: *const c_char, value: *const c_char);
}

/* PlanetCore passes information to the OS in the form of
 * a table of key=value strings, separated by newlines.
 *
 * The list is terminated by an empty string (i.e. two
 * consecutive newlines).
 *
 * To make it easier to parse, we first convert all the
 * newlines into null bytes.
 */

pub unsafe fn planetcore_prepare_table(mut table: *mut c_char) {
    loop {
        if *table == b'\n' as c_char {
            *table = 0;
        }

        table = table.add(1);
        if !(*table.sub(1) != 0 || *table != b'\n' as c_char) {
            break;
        }
    }

    *table = 0;
}

pub unsafe fn planetcore_get_key(mut table: *const c_char, key: *const c_char) -> *const c_char {
    let keylen = strlen(key);

    loop {
        if strncmp(table, key, keylen) == 0 && *table.add(keylen) == b'=' as c_char {
            return table.add(keylen + 1);
        }

        table = table.add(strlen(table) + 1);
        if strlen(table) == 0 {
            break;
        }
    }

    core::ptr::null()
}

pub unsafe fn planetcore_get_decimal(table: *const c_char, key: *const c_char,
                                     val: *mut u64) -> i32 {
    let string = planetcore_get_key(table, key);
    if string.is_null() {
        return 0;
    }

    *val = strtoull(string, core::ptr::null_mut(), 10);
    1
}

pub unsafe fn planetcore_get_hex(table: *const c_char, key: *const c_char,
                                 val: *mut u64) -> i32 {
    let string = planetcore_get_key(table, key);
    if string.is_null() {
        return 0;
    }

    *val = strtoull(string, core::ptr::null_mut(), 16);
    1
}

static mut mac_table: [u64; 4] = [
    0x000000000000,
    0x000000800000,
    0x000000400000,
    0x000000c00000,
];

pub unsafe fn planetcore_set_mac_addrs(table: *const c_char) {
    let mut addr = [[0u8; 6]; 4];
    let mut int_addr: u64 = 0;
    let mut i: u32;
    let mut j: i32;

    if planetcore_get_hex(table, PLANETCORE_KEY_MAC_ADDR, &mut int_addr) == 0 {
        return;
    }

    i = 0;
    while i < 4 {
        let mut this_dev_addr = (int_addr & !0x000000c00000) | mac_table[i as usize];

        j = 5;
        while j >= 0 {
            addr[i as usize][j as usize] = (this_dev_addr & 0xff) as u8;
            this_dev_addr >>= 8;
            j -= 1;
        }

        dt_fixup_mac_address(i, addr[i as usize].as_mut_ptr());
        i += 1;
    }
}

static mut prop_buf: [c_char; MAX_PROP_LEN] = [0; MAX_PROP_LEN];

pub unsafe fn planetcore_set_stdout_path(table: *const c_char) {
    let mut path: *mut c_char;
    let label: *const c_char;
    let mut node: *mut c_void;
    let mut chosen: *mut c_void;

    label = planetcore_get_key(table, PLANETCORE_KEY_SERIAL_PORT);
    if label.is_null() {
        return;
    }

    node = find_node_by_prop_value_str(core::ptr::null_mut(), b"linux,planetcore-label\0".as_ptr() as *const c_char,
                                       label);
    if node.is_null() {
        return;
    }

    path = get_path(node, prop_buf.as_mut_ptr(), MAX_PROP_LEN);
    if path.is_null() {
        return;
    }

    chosen = finddevice(b"/chosen\0".as_ptr() as *const c_char);
    if chosen.is_null() {
        chosen = create_node(core::ptr::null_mut(), b"chosen\0".as_ptr() as *const c_char);
    }
    if chosen.is_null() {
        return;
    }

    setprop_str(chosen, b"linux,stdout-path\0".as_ptr() as *const c_char, path);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
