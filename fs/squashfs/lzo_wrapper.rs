// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2010 LG Electronics
 * Chan Jeong <chan.jeong@lge.com>
 *
 * lzo_wrapper.c
 */

// Kernel and Squashfs headers are supplied by the surrounding translation unit.

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct squashfs_sb_info {
    pub block_size: c_int,
}

#[repr(C)]
pub struct squashfs_page_actor {
    pub length: usize,
}

#[repr(C)]
pub struct bio;

#[repr(C)]
pub struct bvec_iter_all {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bio_vec {
    pub bv_len: usize,
}

#[repr(C)]
pub struct squashfs_decompressor {
    pub init: Option<unsafe extern "C" fn(*mut squashfs_sb_info, *mut c_void) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub decompress: Option<unsafe extern "C" fn(
        *mut squashfs_sb_info,
        *mut c_void,
        *mut bio,
        c_int,
        c_int,
        *mut squashfs_page_actor,
    ) -> c_int>,
    pub id: c_int,
    pub name: *const u8,
    pub alloc_buffer: c_int,
    pub supported: c_int,
}

extern "C" {
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn bvec_init_iter_all(iter: *mut bvec_iter_all) -> *mut bio_vec;
    fn bio_next_segment(bio: *mut bio, iter: *mut bvec_iter_all) -> bool;
    fn bvec_virt(bvec: *mut bio_vec) -> *mut u8;
    fn lzo1x_decompress_safe(
        input: *const c_void,
        input_len: usize,
        output: *mut c_void,
        output_len: *mut usize,
    ) -> c_int;
    fn squashfs_first_page(output: *mut squashfs_page_actor) -> *mut c_void;
    fn squashfs_next_page(output: *mut squashfs_page_actor) -> *mut c_void;
    fn squashfs_finish_page(output: *mut squashfs_page_actor);
}

const SQUASHFS_METADATA_SIZE: c_int = 8192;
const PAGE_SIZE: usize = 4096;
const LZO_COMPRESSION: c_int = 3;
const LZO_E_OK: c_int = 0;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;

#[repr(C)]
struct squashfs_lzo {
    input: *mut c_void,
    output: *mut c_void,
}

unsafe fn lzo_init(msblk: *mut squashfs_sb_info, _buff: *mut c_void) -> *mut c_void {
    let block_size = (*msblk).block_size.max(SQUASHFS_METADATA_SIZE) as usize;
    let stream = libc::calloc(1, core::mem::size_of::<squashfs_lzo>()) as *mut squashfs_lzo;
    if stream.is_null() {
        return core::ptr::null_mut();
    }
    (*stream).input = vmalloc(block_size);
    if (*stream).input.is_null() {
        kfree(stream as *mut c_void);
        return core::ptr::null_mut();
    }
    (*stream).output = vmalloc(block_size);
    if (*stream).output.is_null() {
        vfree((*stream).input);
        kfree(stream as *mut c_void);
        return core::ptr::null_mut();
    }
    stream as *mut c_void
}

unsafe fn lzo_free(strm: *mut c_void) {
    let stream = strm as *mut squashfs_lzo;
    if !stream.is_null() {
        vfree((*stream).input);
        vfree((*stream).output);
    }
    kfree(strm);
}

unsafe fn lzo_uncompress(
    _msblk: *mut squashfs_sb_info,
    strm: *mut c_void,
    bio: *mut bio,
    mut offset: c_int,
    length: c_int,
    output: *mut squashfs_page_actor,
) -> c_int {
    let mut iter_all = core::mem::zeroed::<bvec_iter_all>();
    let bvec = bvec_init_iter_all(&mut iter_all);
    let stream = strm as *mut squashfs_lzo;
    let mut buff = (*stream).input as *mut u8;
    let mut data: *mut u8;
    let mut bytes = length;
    let mut out_len = (*output).length;

    while bio_next_segment(bio, &mut iter_all) {
        let avail = bytes.min((*bvec).bv_len as c_int - offset);
        data = bvec_virt(bvec);
        core::ptr::copy_nonoverlapping(data.add(offset as usize), buff, avail as usize);
        buff = buff.add(avail as usize);
        bytes -= avail;
        offset = 0;
    }

    let mut res = lzo1x_decompress_safe(
        (*stream).input,
        length as usize,
        (*stream).output,
        &mut out_len,
    );
    if res != LZO_E_OK {
        return -EIO;
    }

    bytes = out_len as c_int;
    data = squashfs_first_page(output) as *mut u8;
    buff = (*stream).output as *mut u8;
    while !data.is_null() {
        if bytes <= PAGE_SIZE as c_int {
            core::ptr::copy_nonoverlapping(buff, data, bytes as usize);
            break;
        } else {
            core::ptr::copy_nonoverlapping(buff, data, PAGE_SIZE);
            buff = buff.add(PAGE_SIZE);
            bytes -= PAGE_SIZE as c_int;
            data = squashfs_next_page(output) as *mut u8;
        }
    }
    squashfs_finish_page(output);
    res = out_len as c_int;
    res
}

#[no_mangle]
pub static mut squashfs_lzo_comp_ops: squashfs_decompressor = squashfs_decompressor {
    init: Some(lzo_init),
    free: Some(lzo_free),
    decompress: Some(lzo_uncompress),
    id: LZO_COMPRESSION,
    name: b"lzo\0".as_ptr(),
    alloc_buffer: 0,
    supported: 1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
