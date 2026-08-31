// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies: <test_progs.h>, <network_helpers.h>, "map_ptr_kern.lskel.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct bpf_map {
    pub max_entries: c_int,
}

#[repr(C)]
pub struct map_ptr_kern_lskel_maps {
    pub m_ringbuf: bpf_map,
}

#[repr(C)]
pub struct bpf_program {
    pub prog_fd: c_int,
}

#[repr(C)]
pub struct map_ptr_kern_lskel_progs {
    pub cg_skb: bpf_program,
}

#[repr(C)]
pub struct map_ptr_kern_lskel_bss {
    pub page_size: c_int,
}

#[repr(C)]
pub struct map_ptr_kern_lskel {
    pub maps: map_ptr_kern_lskel_maps,
    pub progs: map_ptr_kern_lskel_progs,
    pub bss: *mut map_ptr_kern_lskel_bss,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub data_out: *mut c_void,
    pub data_size_out: u32,
    pub repeat: u32,
    pub retval: u32,
}

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn getpagesize() -> c_int;
    fn map_ptr_kern_lskel__open() -> *mut map_ptr_kern_lskel;
    fn map_ptr_kern_lskel__load(skel: *mut map_ptr_kern_lskel) -> c_int;
    fn map_ptr_kern_lskel__destroy(skel: *mut map_ptr_kern_lskel);
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: u32, expected: u32, name: *const c_char) -> bool;
}

pub unsafe fn test_map_ptr() {
    let mut skel: *mut map_ptr_kern_lskel;
    let mut buf: [c_char; 128] = [0; 128];
    let mut err: c_int;
    let page_size: c_int = unsafe { getpagesize() };
    let mut topts = bpf_test_run_opts {
        data_in: unsafe { pkt_v4.as_ptr() as *const c_void },
        data_size_in: core::mem::size_of_val(unsafe { &pkt_v4 }) as u32,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: core::mem::size_of_val(&buf) as u32,
        repeat: 1,
        retval: 0,
    };

    skel = unsafe { map_ptr_kern_lskel__open() };
    if !unsafe { ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) } {
        return;
    }

    unsafe {
        (*skel).maps.m_ringbuf.max_entries = page_size;
    }

    err = unsafe { map_ptr_kern_lskel__load(skel) };
    if !unsafe { ASSERT_OK(err, c"skel_load".as_ptr()) } {
        unsafe {
            map_ptr_kern_lskel__destroy(skel);
        }
        return;
    }

    unsafe {
        (*(*skel).bss).page_size = page_size;
    }

    err = unsafe { bpf_prog_test_run_opts((*skel).progs.cg_skb.prog_fd, &mut topts) };

    if !unsafe { ASSERT_OK(err, c"test_run".as_ptr()) } {
        unsafe {
            map_ptr_kern_lskel__destroy(skel);
        }
        return;
    }

    if !unsafe { ASSERT_NEQ(topts.retval, 0, c"test_run retval".as_ptr()) } {
        unsafe {
            map_ptr_kern_lskel__destroy(skel);
        }
        return;
    }

    unsafe {
        map_ptr_kern_lskel__destroy(skel);
    }
}
