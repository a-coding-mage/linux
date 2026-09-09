// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

// Linux and SquashFS dependencies are supplied by the surrounding translation.

/*
 * This file implements single-threaded decompression in the
 * decompressor framework
 */

#[repr(C)]
struct squashfs_stream {
    stream: *mut core::ffi::c_void,
    mutex: mutex,
}

unsafe fn squashfs_decompressor_create(
    msblk: *mut squashfs_sb_info,
    comp_opts: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut stream: *mut squashfs_stream;
    let mut err: i32 = -ENOMEM;

    stream = kmalloc_obj::<squashfs_stream>();
    if stream.is_null() {
        goto_out!();
    }

    (*stream).stream = ((*(*msblk).decompressor).init)(msblk, comp_opts);
    if IS_ERR((*stream).stream) {
        err = PTR_ERR((*stream).stream);
        goto_out!();
    }

    kfree(comp_opts);
    mutex_init(&mut (*stream).mutex);
    return stream as *mut core::ffi::c_void;

out:
    kfree(stream);
    return ERR_PTR(err);
}

unsafe fn squashfs_decompressor_destroy(msblk: *mut squashfs_sb_info) {
    let stream: *mut squashfs_stream = (*msblk).stream as *mut squashfs_stream;

    if !stream.is_null() {
        ((*(*msblk).decompressor).free)((*stream).stream);
        kfree(stream);
    }
}

unsafe fn squashfs_decompress(
    msblk: *mut squashfs_sb_info,
    bio: *mut bio,
    offset: i32,
    length: i32,
    output: *mut squashfs_page_actor,
) -> i32 {
    let res: i32;
    let stream: *mut squashfs_stream = (*msblk).stream as *mut squashfs_stream;

    mutex_lock(&mut (*stream).mutex);
    res = ((*(*msblk).decompressor).decompress)(
        msblk,
        (*stream).stream,
        bio,
        offset,
        length,
        output,
    );
    mutex_unlock(&mut (*stream).mutex);

    if res < 0 {
        ERROR!(
            "{} decompression failed, data probably corrupt\n",
            (*(*msblk).decompressor).name
        );
    }

    res
}

unsafe fn squashfs_max_decompressors() -> i32 {
    1
}

const squashfs_decompressor_single: squashfs_decompressor_thread_ops =
    squashfs_decompressor_thread_ops {
        create: Some(squashfs_decompressor_create),
        destroy: Some(squashfs_decompressor_destroy),
        decompress: Some(squashfs_decompress),
        max_decompressors: Some(squashfs_max_decompressors),
    };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
