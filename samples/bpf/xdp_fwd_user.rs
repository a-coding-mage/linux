// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2017-18 David Ahern <dsahern@gmail.com>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

// Definitions and declarations supplied by the Linux, libc, and libbpf headers.
const XDP_FLAGS_UPDATE_IF_NOEXIST: u32 = 1 << 0;
const XDP_FLAGS_SKB_MODE: u32 = 1 << 1;
const XDP_FLAGS_DRV_MODE: u32 = 1 << 2;
const BPF_PROG_TYPE_XDP: u32 = 6;
const BPF_OBJ_NAME_LEN: usize = 16;
const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;

#[repr(C)]
pub struct bpf_program { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_object { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_map { _private: [u8; 0] }
#[repr(C)]
pub struct bpf_xdp_attach_opts {
    pub sz: usize,
    pub old_prog_fd: c_int,
    pub _reserved: [u64; 8],
}
#[repr(C)]
pub struct bpf_prog_info {
    pub _data: [u8; 256],
}

unsafe extern "C" {
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: u32, opts: *mut c_void) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_xdp_query_id(ifindex: c_int, flags: u32, prog_id: *mut u32) -> c_int;
    fn bpf_prog_get_fd_by_id(id: u32) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut u32) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: u32, opts: *mut bpf_xdp_attach_opts) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn bpf_object__next_program(obj: *mut bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    fn bpf_program__set_type(prog: *mut bpf_program, ty: u32) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__section_name(prog: *mut bpf_program) -> *const c_char;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn if_nametoindex(name: *const c_char) -> u32;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> u64;
    fn basename(path: *mut c_char) -> *mut c_char;
    fn strerror(err: c_int) -> *const c_char;
    fn printf(fmt: *const c_char, ...);
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ... ) -> c_int;
}

static mut XDP_FLAGS: u32 = XDP_FLAGS_UPDATE_IF_NOEXIST;

unsafe fn do_attach(idx: c_int, prog_fd: c_int, map_fd: c_int, name: *const c_char) -> c_int {
    let mut err = bpf_xdp_attach(idx, prog_fd, XDP_FLAGS, ptr::null_mut());
    if err < 0 {
        printf(b"ERROR: failed to attach program to %s\0".as_ptr() as *const c_char, name);
        return err;
    }
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &idx as *const _ as *const c_void, 0);
    if err != 0 { printf(b"ERROR: failed using device %s as TX-port\n\0".as_ptr() as *const c_char, name); }
    err
}

unsafe fn do_detach(ifindex: c_int, ifname: *const c_char, app_name: *const c_char) -> c_int {
    let mut opts = bpf_xdp_attach_opts { sz: std::mem::size_of::<bpf_xdp_attach_opts>(), old_prog_fd: 0, _reserved: [0; 8] };
    let mut prog_info = bpf_prog_info { _data: [0; 256] };
    let mut prog_name = [0 as c_char; BPF_OBJ_NAME_LEN];
    let mut info_len = std::mem::size_of::<bpf_prog_info>() as u32;
    let mut curr_prog_id = 0u32;
    let mut err = 1;
    if bpf_xdp_query_id(ifindex, XDP_FLAGS, &mut curr_prog_id) != 0 { printf(b"ERROR: bpf_xdp_query_id failed (%s)\n\0".as_ptr() as _, strerror(*libc_errno())); return err; }
    if curr_prog_id == 0 { printf(b"ERROR: flags(0x%x) xdp prog is not attached to %s\n\0".as_ptr() as _, XDP_FLAGS, ifname); return err; }
    let prog_fd = bpf_prog_get_fd_by_id(curr_prog_id);
    if prog_fd < 0 { printf(b"ERROR: bpf_prog_get_fd_by_id failed (%s)\n\0".as_ptr() as _, strerror(*libc_errno())); return prog_fd; }
    err = bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut info_len);
    if err != 0 { printf(b"ERROR: bpf_prog_get_info_by_fd failed (%s)\n\0".as_ptr() as _, strerror(*libc_errno())); close(prog_fd); return err; }
    snprintf(prog_name.as_mut_ptr(), prog_name.len(), b"%s_prog\0".as_ptr() as _, app_name);
    prog_name[BPF_OBJ_NAME_LEN - 1] = 0;
    // The C source compares prog_info.name with prog_name; preserve that FFI operation.
    if libc_strcmp(prog_info._data.as_ptr() as *const c_char, prog_name.as_ptr()) != 0 { printf(b"ERROR: %s isn't attached to %s\n\0".as_ptr() as _, app_name, ifname); close(prog_fd); return 1; }
    opts.old_prog_fd = prog_fd;
    err = bpf_xdp_detach(ifindex, XDP_FLAGS, &mut opts);
    if err < 0 { printf(b"ERROR: failed to detach program from %s (%s)\n\0".as_ptr() as _, ifname, strerror(*libc_errno())); }
    close(prog_fd); err
}

unsafe extern "C" { fn libc_errno() -> *mut c_int; fn libc_strcmp(a: *const c_char, b: *const c_char) -> c_int; }

unsafe fn usage(prog: *const c_char) {
    fprintf(ptr::null_mut(), b"usage: %s [OPTS] interface-list\n\nOPTS:\n    -d    detach program\n    -S    use skb-mode\n    -F    force loading prog\n    -D    direct table lookups (skip fib rules)\n\0".as_ptr() as _, prog);
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut prog_name = b"xdp_fwd\0".as_ptr() as *const c_char;
    let mut prog: *mut bpf_program = ptr::null_mut();
    let mut pos: *mut bpf_program = ptr::null_mut();
    let mut sec_name: *const c_char;
    let mut prog_fd = -1; let mut map_fd = -1;
    let mut filename = [0 as c_char; PATH_MAX];
    let mut attach = 1; let mut ret = 0;
    loop {
        let opt = getopt(argc, argv, b":dDSF\0".as_ptr() as _);
        if opt == -1 { break; }
        match opt as u8 { b'd' => attach = 0, b'S' => XDP_FLAGS |= XDP_FLAGS_SKB_MODE, b'F' => XDP_FLAGS &= !XDP_FLAGS_UPDATE_IF_NOEXIST, b'D' => prog_name = b"xdp_fwd_direct\0".as_ptr() as _, _ => { usage(basename(*argv)); return 1; } }
    }
    if XDP_FLAGS & XDP_FLAGS_SKB_MODE == 0 { XDP_FLAGS |= XDP_FLAGS_DRV_MODE; }
    if optind() == argc { usage(basename(*argv)); return 1; }
    if attach != 0 {
        snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as _, *argv);
        if access(filename.as_ptr(), O_RDONLY) < 0 { printf(b"error accessing file %s: %s\n\0".as_ptr() as _, filename.as_ptr(), strerror(*libc_errno())); return 1; }
        let obj = bpf_object__open_file(filename.as_ptr(), ptr::null());
        if libbpf_get_error(obj as *const c_void) != 0 { return 1; }
        prog = bpf_object__next_program(obj, ptr::null_mut());
        bpf_program__set_type(prog, BPF_PROG_TYPE_XDP);
        let err = bpf_object__load(obj);
        if err != 0 { printf(b"Does kernel support devmap lookup?\n\0".as_ptr() as _); return 1; }
        // bpf_object__for_each_program(pos, obj)
        pos = bpf_object__next_program(obj, ptr::null_mut());
        while !pos.is_null() {
            sec_name = bpf_program__section_name(pos);
            if !sec_name.is_null() && libc_strcmp(sec_name, prog_name) == 0 { prog = pos; break; }
            pos = bpf_object__next_program(obj, pos);
        }
        prog_fd = bpf_program__fd(prog);
        if prog_fd < 0 { printf(b"program not found: %s\n\0".as_ptr() as _, strerror(prog_fd)); return 1; }
        map_fd = bpf_map__fd(bpf_object__find_map_by_name(obj, b"xdp_tx_ports\0".as_ptr() as _));
        if map_fd < 0 { printf(b"map not found: %s\n\0".as_ptr() as _, strerror(map_fd)); return 1; }
    }
    let mut i = optind();
    while i < argc {
        let arg = *argv.add(i as usize);
        let mut idx = if_nametoindex(arg) as c_int;
        if idx == 0 { idx = strtoul(arg, ptr::null_mut(), 0) as c_int; }
        if idx == 0 { fprintf(ptr::null_mut(), b"Invalid arg\n\0".as_ptr() as _); return 1; }
        let err = if attach == 0 { do_detach(idx, arg, prog_name) } else { do_attach(idx, prog_fd, map_fd, arg) };
        if err != 0 { ret = err; }
        i += 1;
    }
    ret
}

unsafe extern "C" { fn optind() -> c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
