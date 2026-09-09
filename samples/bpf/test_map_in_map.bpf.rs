/*
 * Copyright (c) 2017 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// KBUILD_MODNAME is "foo".
// The following names are supplied by vmlinux.h and the BPF helper headers.

const MAX_NR_PORTS: u32 = 65536;
const EINVAL: i32 = 22;
const ENOENT: i32 = 2;

#[repr(C)]
pub struct inner_a {
    pub _map_definition: [u8; 0],
}

#[repr(C)]
pub struct inner_h {
    pub _map_definition: [u8; 0],
}

// map #0
#[link_section = ".maps"]
pub static mut port_a: inner_a = inner_a { _map_definition: [] };

// map #1
#[link_section = ".maps"]
pub static mut port_h: inner_h = inner_h { _map_definition: [] };

// map #2
#[link_section = ".maps"]
pub static mut reg_result_h: inner_h = inner_h { _map_definition: [] };

// map #3
#[link_section = ".maps"]
pub static mut inline_result_h: inner_h = inner_h { _map_definition: [] };

// map #4 // Test case #0
#[link_section = ".maps"]
pub static mut a_of_port_a: inner_a = inner_a { _map_definition: [] };

// map #5 // Test case #1
#[link_section = ".maps"]
pub static mut h_of_port_a: inner_a = inner_a { _map_definition: [] };

// map #6 // Test case #2
#[link_section = ".maps"]
pub static mut h_of_port_h: inner_h = inner_h { _map_definition: [] };

extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const u32)
        -> *mut i32;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const i32,
        value: *const i32,
        flags: u64,
    ) -> i64;
    fn bpf_probe_read_user(
        dst: *mut core::ffi::c_void,
        size: usize,
        src: *const core::ffi::c_void,
    ) -> i64;
}

const BPF_ANY: u64 = 0;

#[inline(always)]
unsafe fn do_reg_lookup(inner_map: *mut core::ffi::c_void, port: u32) -> i32 {
    let result = bpf_map_lookup_elem(inner_map, &port);
    if !result.is_null() { *result } else { -ENOENT }
}

#[inline(always)]
unsafe fn do_inline_array_lookup(inner_map: *mut core::ffi::c_void, port: u32) -> i32 {
    if inner_map != core::ptr::addr_of_mut!(port_a).cast() {
        return -EINVAL;
    }

    let result = bpf_map_lookup_elem(core::ptr::addr_of_mut!(port_a).cast(), &port);
    if !result.is_null() { *result } else { -ENOENT }
}

#[inline(always)]
unsafe fn do_inline_hash_lookup(inner_map: *mut core::ffi::c_void, port: u32) -> i32 {
    if inner_map != core::ptr::addr_of_mut!(port_h).cast() {
        return -EINVAL;
    }

    let result = bpf_map_lookup_elem(core::ptr::addr_of_mut!(port_h).cast(), &port);
    if !result.is_null() { *result } else { -ENOENT }
}

#[link_section = "ksyscall/connect"]
pub unsafe extern "C" fn trace_sys_connect(
    _fd: u32,
    in6: *mut crate::sockaddr_in6,
    addrlen: i32,
) -> i32 {
    let mut test_case: u16;
    let mut port: u16 = 0;
    let mut dst6 = [0u16; 8];
    let mut ret: i32;
    let mut inline_ret: i32;
    let ret_key: i32 = 0;
    let port_key: u32;
    let outer_map: *mut core::ffi::c_void;
    let inner_map: *mut core::ffi::c_void;
    let _inline_hash = false;

    if addrlen != core::mem::size_of::<crate::sockaddr_in6>() as i32 {
        return 0;
    }

    ret = bpf_probe_read_user(
        dst6.as_mut_ptr().cast(),
        core::mem::size_of_val(&dst6),
        core::ptr::addr_of!((*in6).sin6_addr).cast(),
    ) as i32;
    if ret != 0 {
        inline_ret = ret;
        goto_done(ret_key, ret, inline_ret);
        return 0;
    }

    if dst6[0] != 0xdead || dst6[1] != 0xbeef { return 0; }
    test_case = dst6[7];

    ret = bpf_probe_read_user(
        core::ptr::addr_of_mut!(port).cast(),
        core::mem::size_of::<u16>(),
        core::ptr::addr_of!((*in6).sin6_port).cast(),
    ) as i32;
    if ret != 0 { inline_ret = ret; goto_done(ret_key, ret, inline_ret); return 0; }
    port_key = port as u32;

    ret = -ENOENT;
    outer_map = if test_case == 0 { core::ptr::addr_of_mut!(a_of_port_a).cast() }
        else if test_case == 1 { core::ptr::addr_of_mut!(h_of_port_a).cast() }
        else if test_case == 2 { core::ptr::addr_of_mut!(h_of_port_h).cast() }
        else { ret = line!() as i32; inline_ret = ret; goto_done(ret_key, ret, inline_ret); return 0; };

    inner_map = bpf_map_lookup_elem(outer_map, &port_key).cast();
    if inner_map.is_null() { ret = line!() as i32; inline_ret = ret; goto_done(ret_key, ret, inline_ret); return 0; }
    ret = do_reg_lookup(inner_map, port_key);
    inline_ret = if test_case == 0 || test_case == 1 { do_inline_array_lookup(inner_map, port_key) } else { do_inline_hash_lookup(inner_map, port_key) };
    goto_done(ret_key, ret, inline_ret);
    0
}

unsafe fn goto_done(ret_key: i32, ret: i32, inline_ret: i32) {
    bpf_map_update_elem(core::ptr::addr_of_mut!(reg_result_h).cast(), &ret_key, &ret, BPF_ANY);
    bpf_map_update_elem(core::ptr::addr_of_mut!(inline_result_h).cast(), &ret_key, &inline_ret, BPF_ANY);
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
#[link_section = "version"]
pub static mut _version: u32 = crate::LINUX_VERSION_CODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
