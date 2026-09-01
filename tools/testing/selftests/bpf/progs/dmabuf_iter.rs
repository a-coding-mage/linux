// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Google LLC */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* Dependencies from vmlinux.h, bpf/bpf_core_read.h, and bpf/bpf_helpers.h. */
type size_t = usize;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_EXIST: u64 = 2;

/* From uapi/linux/dma-buf.h */
const DMA_BUF_NAME_LEN: usize = 32;

#[repr(C)]
pub struct inode {
    pub i_ino: c_ulong,
}

#[repr(C)]
pub struct file {
    pub f_inode: *const inode,
}

#[repr(C)]
pub struct dma_buf {
    pub file: *const file,
    pub size: size_t,
    pub name: *const c_char,
    pub exp_name: *const c_char,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
}

#[repr(C)]
pub struct bpf_iter__dmabuf {
    pub meta: *mut bpf_iter_meta,
    pub dmabuf: *mut dma_buf,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

unsafe extern "C" {
    fn bpf_core_read(dst: *mut c_void, sz: u32, src: *const c_void) -> c_int;
    fn bpf_probe_read_kernel_str(dst: *mut c_void, size: u32, unsafe_ptr: *const c_void) -> c_long;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut bool;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_long;

    /*
     * Rust declarations for BPF C macros/helpers whose definitions are supplied
     * by the BPF build environment.
     */
    fn bpf_for_each_dmabuf_next(d: *mut *mut dma_buf) -> bool;
    fn bpf_seq_printf_dmabuf(
        seq: *mut seq_file,
        inode: c_ulong,
        size: size_t,
        name: *const c_char,
        exporter: *const c_char,
    );
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut testbuf_hash: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: DMA_BUF_NAME_LEN as u32,
    value_size: size_of::<bool>() as u32,
    max_entries: 5,
};

/*
 * Fields output by this iterator are delimited by newlines. Convert any
 * newlines in user-provided printed strings to spaces.
 */
unsafe fn sanitize_string(src: *mut c_char, size: size_t) {
    let mut c = src;

    while (c as usize).wrapping_sub(src as usize) < size && unsafe { *c } != 0 {
        if unsafe { *c } == b'\n' as c_char {
            unsafe {
                *c = b' ' as c_char;
            }
        }
        c = unsafe { c.add(1) };
    }
}

#[unsafe(link_section = "iter/dmabuf")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dmabuf_collector(ctx: *mut bpf_iter__dmabuf) -> c_int {
    let dmabuf = unsafe { (*ctx).dmabuf };
    let seq = unsafe { (*(*ctx).meta).seq };
    let mut inode: c_ulong = 0;
    let mut size: size_t = 0;
    let mut pname: *const c_char = ptr::null();
    let mut exporter: *const c_char = ptr::null();
    let mut name: [c_char; DMA_BUF_NAME_LEN] = [0; DMA_BUF_NAME_LEN];

    if dmabuf.is_null() {
        return 0;
    }

    /*
     * Equivalent to:
     * BPF_CORE_READ_INTO(&inode, dmabuf, file, f_inode, i_ino)
     */
    let file_ptr = unsafe { (*dmabuf).file };
    let inode_ptr = if file_ptr.is_null() {
        ptr::null()
    } else {
        unsafe { (*file_ptr).f_inode }
    };

    if inode_ptr.is_null()
        || unsafe {
            bpf_core_read(
                &mut inode as *mut _ as *mut c_void,
                size_of::<c_ulong>() as u32,
                &(*inode_ptr).i_ino as *const _ as *const c_void,
            ) != 0
        }
        || unsafe {
            bpf_core_read(
                &mut size as *mut _ as *mut c_void,
                size_of::<size_t>() as u32,
                &(*dmabuf).size as *const _ as *const c_void,
            ) != 0
        }
        || unsafe {
            bpf_core_read(
                &mut pname as *mut _ as *mut c_void,
                size_of::<*const c_char>() as u32,
                &(*dmabuf).name as *const _ as *const c_void,
            ) != 0
        }
        || unsafe {
            bpf_core_read(
                &mut exporter as *mut _ as *mut c_void,
                size_of::<*const c_char>() as u32,
                &(*dmabuf).exp_name as *const _ as *const c_void,
            ) != 0
        }
    {
        return 1;
    }

    /* Buffers are not required to be named */
    if !pname.is_null() {
        if unsafe {
            bpf_probe_read_kernel_str(
                name.as_mut_ptr() as *mut c_void,
                size_of::<[c_char; DMA_BUF_NAME_LEN]>() as u32,
                pname as *const c_void,
            )
        } < 0
        {
            return 1;
        }

        /* Name strings can be provided by userspace */
        unsafe {
            sanitize_string(name.as_mut_ptr(), size_of::<[c_char; DMA_BUF_NAME_LEN]>());
        }
    }

    unsafe {
        bpf_seq_printf_dmabuf(seq, inode, size, name.as_ptr(), exporter);
    }
    0
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iter_dmabuf_for_each(_ctx: *const c_void) -> c_int {
    let mut d: *mut dma_buf = ptr::null_mut();

    /* bpf_for_each(dmabuf, d) */
    while unsafe { bpf_for_each_dmabuf_next(&mut d) } {
        let mut name: [c_char; DMA_BUF_NAME_LEN] = [0; DMA_BUF_NAME_LEN];
        let mut pname: *const c_char = ptr::null();
        let mut found: *mut bool;
        let len: c_long;
        let mut i: c_int;

        if unsafe {
            bpf_core_read(
                &mut pname as *mut _ as *mut c_void,
                size_of::<*const c_char>() as u32,
                &(*d).name as *const _ as *const c_void,
            ) != 0
        } {
            return 1;
        }

        /* Buffers are not required to be named */
        if pname.is_null() {
            continue;
        }

        len = unsafe {
            bpf_probe_read_kernel_str(
                name.as_mut_ptr() as *mut c_void,
                size_of::<[c_char; DMA_BUF_NAME_LEN]>() as u32,
                pname as *const c_void,
            )
        };
        if len < 0 {
            return 1;
        }

        /*
         * The entire name buffer is used as a map key.
         * Zeroize any uninitialized trailing bytes after the NUL.
         */
        i = len as c_int;
        while i < DMA_BUF_NAME_LEN as c_int {
            name[i as usize] = 0;
            i += 1;
        }

        found = unsafe {
            bpf_map_lookup_elem(
                &raw mut testbuf_hash as *mut _ as *mut c_void,
                name.as_ptr() as *const c_void,
            )
        };
        if !found.is_null() {
            let t: bool = true;

            unsafe {
                bpf_map_update_elem(
                    &raw mut testbuf_hash as *mut _ as *mut c_void,
                    name.as_ptr() as *const c_void,
                    &t as *const _ as *const c_void,
                    BPF_EXIST,
                );
            }
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
