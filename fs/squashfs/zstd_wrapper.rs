// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2016-present, Facebook, Inc.
 * All rights reserved.
 *
 * zstd_wrapper.c
 */

// Linux and Squashfs dependencies supplied by the surrounding kernel source.

use core::ffi::c_void;

#[repr(C)]
struct Workspace {
    mem: *mut c_void,
    mem_size: usize,
    window_size: usize,
}

#[repr(C)]
struct ZstdInBuffer {
    src: *const c_void,
    size: usize,
    pos: usize,
}

#[repr(C)]
struct ZstdOutBuffer {
    dst: *mut c_void,
    size: usize,
    pos: usize,
}

// These declarations are provided by the kernel/Squashfs sources.
#[repr(C)]
struct SquashfsSbInfo;
#[repr(C)]
struct Bio;
#[repr(C)]
struct SquashfsPageActor;
#[repr(C)]
struct BvecIterAll;
#[repr(C)]
struct BioVec {
    bv_len: usize,
}
#[repr(C)]
struct ZstdDstream;

extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn zstd_dstream_workspace_bound(window_size: usize) -> usize;
    fn zstd_init_dstream(window_size: usize, workspace: *mut c_void, workspace_size: usize)
        -> *mut ZstdDstream;
    fn zstd_decompress_stream(
        stream: *mut ZstdDstream,
        output: *mut ZstdOutBuffer,
        input: *mut ZstdInBuffer,
    ) -> usize;
    fn zstd_is_error(code: usize) -> bool;
    fn zstd_get_error_code(code: usize) -> usize;
    fn bio_next_segment(bio: *mut Bio, iter: *mut BvecIterAll) -> bool;
    fn bvec_init_iter_all(iter: *mut BvecIterAll) -> *mut BioVec;
    fn bvec_virt(bvec: *mut BioVec) -> *const u8;
    fn squashfs_first_page(output: *mut SquashfsPageActor) -> *mut c_void;
    fn squashfs_next_page(output: *mut SquashfsPageActor) -> *mut c_void;
    fn squashfs_finish_page(output: *mut SquashfsPageActor);
    fn squashfs_error(message: *const u8, ...);
}

unsafe fn zstd_init(msblk: *mut SquashfsSbInfo, _buff: *mut c_void) -> *mut c_void {
    let wksp = kmalloc(core::mem::size_of::<Workspace>(), 0) as *mut Workspace;

    if wksp.is_null() {
        goto_failed(wksp);
    }
    (*wksp).window_size = core::cmp::max((*msblk).block_size, SQUASHFS_METADATA_SIZE);
    (*wksp).mem_size = zstd_dstream_workspace_bound((*wksp).window_size);
    (*wksp).mem = vmalloc((*wksp).mem_size);
    if (*wksp).mem.is_null() {
        goto_failed(wksp);
    }

    wksp as *mut c_void
}

unsafe fn goto_failed(wksp: *mut Workspace) -> ! {
    squashfs_error(b"Failed to allocate zstd workspace\0".as_ptr());
    kfree(wksp as *mut c_void);
    core::ptr::null_mut()
}

unsafe fn zstd_free(strm: *mut c_void) {
    let wksp = strm as *mut Workspace;
    if !wksp.is_null() {
        vfree((*wksp).mem);
    }
    kfree(wksp as *mut c_void);
}

unsafe fn zstd_uncompress(
    msblk: *mut SquashfsSbInfo,
    strm: *mut c_void,
    bio: *mut Bio,
    mut offset: i32,
    mut length: i32,
    output: *mut SquashfsPageActor,
) -> i32 {
    let wksp = strm as *mut Workspace;
    let mut total_out: usize = 0;
    let mut error: i32 = 0;
    let mut in_buf = ZstdInBuffer { src: core::ptr::null(), size: 0, pos: 0 };
    let mut out_buf = ZstdOutBuffer { dst: core::ptr::null_mut(), size: 0, pos: 0 };
    let mut iter_all = BvecIterAll {};
    let bvec = bvec_init_iter_all(&mut iter_all);
    let stream = zstd_init_dstream((*wksp).window_size, (*wksp).mem, (*wksp).mem_size);

    if stream.is_null() {
        squashfs_error(b"Failed to initialize zstd decompressor\0".as_ptr());
        return -5;
    }

    out_buf.size = PAGE_SIZE;
    out_buf.dst = squashfs_first_page(output);
    if out_buf.dst.is_null() {
        error = -5;
        goto_finish(output);
    }

    loop {
        if in_buf.pos == in_buf.size {
            if !bio_next_segment(bio, &mut iter_all) {
                error = -5;
                break;
            }
            let avail = core::cmp::min(length, (*bvec).bv_len as i32 - offset);
            length -= avail;
            in_buf.src = bvec_virt(bvec).add(offset as usize) as *const c_void;
            in_buf.size = avail as usize;
            in_buf.pos = 0;
            offset = 0;
        }
        if out_buf.pos == out_buf.size {
            out_buf.dst = squashfs_next_page(output);
            if out_buf.dst.is_null() {
                error = -5;
                break;
            }
            out_buf.pos = 0;
            out_buf.size = PAGE_SIZE;
        }
        total_out = total_out.wrapping_sub(out_buf.pos);
        let zstd_err = zstd_decompress_stream(stream, &mut out_buf, &mut in_buf);
        total_out = total_out.wrapping_add(out_buf.pos);
        if zstd_err == 0 { break; }
        if zstd_is_error(zstd_err) {
            squashfs_error(b"zstd decompression error\0".as_ptr(), zstd_get_error_code(zstd_err) as i32);
            error = -5;
            break;
        }
    }
    goto_finish(output);
    if error != 0 { error } else { total_out as i32 }
}

unsafe fn goto_finish(output: *mut SquashfsPageActor) {
    squashfs_finish_page(output);
}

const PAGE_SIZE: usize = 4096;
const SQUASHFS_METADATA_SIZE: usize = 8192;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
