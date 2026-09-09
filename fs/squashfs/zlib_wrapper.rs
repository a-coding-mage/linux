// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * zlib_wrapper.c
 */

// The declarations referenced below are supplied by the corresponding kernel
// and Squashfs translation units.

unsafe fn zlib_init(_dummy: *mut squashfs_sb_info, _buff: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let stream: *mut z_stream = kmalloc_obj::<z_stream>();
    if stream.is_null() {
        return failed_zlib_init(stream);
    }
    (*stream).workspace = vmalloc(zlib_inflate_workspacesize());
    if (*stream).workspace.is_null() {
        return failed_zlib_init(stream);
    }

    stream as *mut core::ffi::c_void
}

unsafe fn failed_zlib_init(stream: *mut z_stream) -> *mut core::ffi::c_void {
    ERROR!("Failed to allocate zlib workspace\n");
    kfree(stream as *mut core::ffi::c_void);
    ERR_PTR(-ENOMEM)
}

unsafe fn zlib_free(strm: *mut core::ffi::c_void) {
    let stream = strm as *mut z_stream;

    if !stream.is_null() {
        vfree((*stream).workspace);
    }
    kfree(strm);
}

unsafe fn zlib_uncompress(
    _msblk: *mut squashfs_sb_info,
    strm: *mut core::ffi::c_void,
    bio: *mut bio,
    mut offset: i32,
    mut length: i32,
    output: *mut squashfs_page_actor,
) -> i32 {
    let mut iter_all: bvec_iter_all = core::mem::zeroed();
    let bvec: *mut bio_vec = bvec_init_iter_all(&mut iter_all);
    let mut zlib_init: i32 = 0;
    let mut error: i32 = 0;
    let stream = strm as *mut z_stream;

    (*stream).avail_out = PAGE_SIZE;
    (*stream).next_out = squashfs_first_page(output);
    (*stream).avail_in = 0;

    if IS_ERR((*stream).next_out) {
        error = PTR_ERR((*stream).next_out);
        return zlib_finish(output, stream, error);
    }

    loop {
        let zlib_err: i32;

        if (*stream).avail_in == 0 {
            let data: *const core::ffi::c_void;
            let avail: i32;

            if !bio_next_segment(bio, &mut iter_all) {
                // Z_STREAM_END must be reached.
                error = -EIO;
                break;
            }

            avail = core::cmp::min(length, (*bvec).bv_len as i32 - offset);
            data = bvec_virt(bvec);
            length -= avail;
            (*stream).next_in = data.add(offset as usize) as *mut _;
            (*stream).avail_in = avail as _;
            offset = 0;
        }

        if (*stream).avail_out == 0 {
            (*stream).next_out = squashfs_next_page(output);
            if IS_ERR((*stream).next_out) {
                error = PTR_ERR((*stream).next_out);
                break;
            } else if !(*stream).next_out.is_null() {
                (*stream).avail_out = PAGE_SIZE;
            }
        }

        if zlib_init == 0 {
            zlib_err = zlib_inflateInit(stream);
            if zlib_err != Z_OK {
                error = -EIO;
                break;
            }
            zlib_init = 1;
        }

        zlib_err = zlib_inflate(stream, Z_SYNC_FLUSH);
        if zlib_err == Z_STREAM_END {
            break;
        }
        if zlib_err != Z_OK {
            error = -EIO;
            break;
        }
    }

    zlib_finish(output, stream, error)
}

unsafe fn zlib_finish(output: *mut squashfs_page_actor, stream: *mut z_stream, mut error: i32) -> i32 {
    squashfs_finish_page(output);

    if error == 0 && zlib_inflateEnd(stream) != Z_OK {
        error = -EIO;
    }

    if error != 0 { error } else { (*stream).total_out as i32 }
}

pub static squashfs_zlib_comp_ops: squashfs_decompressor = squashfs_decompressor {
    init: Some(zlib_init),
    free: Some(zlib_free),
    decompress: Some(zlib_uncompress),
    id: ZLIB_COMPRESSION,
    name: b"zlib\0".as_ptr() as *const _,
    alloc_buffer: 1,
    supported: 1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
