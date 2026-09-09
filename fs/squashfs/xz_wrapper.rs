// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * xz_wrapper.c
 */

// Linux and Squashfs headers supply the referenced types, constants, and functions.

#[repr(C)]
struct squashfs_xz {
    state: *mut xz_dec,
    buf: xz_buf,
}

#[repr(C)]
struct disk_comp_opts {
    dictionary_size: __le32,
    flags: __le32,
}

#[repr(C)]
struct comp_opts {
    dict_size: c_int,
}

unsafe fn squashfs_xz_comp_opts(
    msblk: *mut squashfs_sb_info,
    buff: *mut c_void,
    len: c_int,
) -> *mut c_void {
    let comp_opts = buff as *mut disk_comp_opts;
    let opts = kmalloc_obj::<comp_opts>();
    let mut err: c_int = 0;
    let mut n: c_int;

    if opts.is_null() {
        err = -ENOMEM;
        return ERR_PTR(err);
    }

    if !comp_opts.is_null() {
        /* check compressor options are the expected length */
        if len < core::mem::size_of::<disk_comp_opts>() as c_int {
            err = -EIO;
            kfree(opts as *mut c_void);
            return ERR_PTR(err);
        }

        (*opts).dict_size = le32_to_cpu((*comp_opts).dictionary_size) as c_int;

        /* the dictionary size should be 2^n or 2^n+2^(n+1) */
        n = ffs((*opts).dict_size) - 1;
        if (*opts).dict_size != (1_i32 << n)
            && (*opts).dict_size != (1_i32 << n) + (1_i32 << (n + 1))
        {
            err = -EIO;
            kfree(opts as *mut c_void);
            return ERR_PTR(err);
        }
    } else {
        /* use defaults */
        (*opts).dict_size = max_t(
            c_int,
            (*msblk).block_size,
            SQUASHFS_METADATA_SIZE,
        );
    }

    opts as *mut c_void
}

unsafe fn squashfs_xz_init(
    _msblk: *mut squashfs_sb_info,
    buff: *mut c_void,
) -> *mut c_void {
    let comp_opts = buff as *mut comp_opts;
    let stream = kmalloc_obj::<squashfs_xz>();
    let err: c_int;

    if stream.is_null() {
        err = -ENOMEM;
        ERROR(c"Failed to initialise xz decompressor\n");
        return ERR_PTR(err);
    }

    (*stream).state = xz_dec_init(XZ_PREALLOC, (*comp_opts).dict_size);
    if (*stream).state.is_null() {
        kfree(stream as *mut c_void);
        err = -ENOMEM;
        ERROR(c"Failed to initialise xz decompressor\n");
        return ERR_PTR(err);
    }

    stream as *mut c_void
}

unsafe fn squashfs_xz_free(strm: *mut c_void) {
    let stream = strm as *mut squashfs_xz;

    if !stream.is_null() {
        xz_dec_end((*stream).state);
        kfree(stream as *mut c_void);
    }
}

unsafe fn squashfs_xz_uncompress(
    _msblk: *mut squashfs_sb_info,
    strm: *mut c_void,
    bio: *mut bio,
    mut offset: c_int,
    mut length: c_int,
    output: *mut squashfs_page_actor,
) -> c_int {
    let mut iter_all: bvec_iter_all = core::mem::zeroed();
    let bvec = bvec_init_iter_all(&mut iter_all);
    let mut total: c_int = 0;
    let mut error: c_int = 0;
    let stream = strm as *mut squashfs_xz;

    xz_dec_reset((*stream).state);
    (*stream).buf.in_pos = 0;
    (*stream).buf.in_size = 0;
    (*stream).buf.out_pos = 0;
    (*stream).buf.out_size = PAGE_SIZE;
    (*stream).buf.out = squashfs_first_page(output);
    if IS_ERR((*stream).buf.out) {
        error = PTR_ERR((*stream).buf.out);
    } else {
        loop {
            let xz_err: xz_ret;

            if (*stream).buf.in_pos == (*stream).buf.in_size {
                let data: *const c_void;
                let avail: c_int;

                if !bio_next_segment(bio, &mut iter_all) {
                    /* XZ_STREAM_END must be reached. */
                    error = -EIO;
                    break;
                }

                avail = min(length, (*bvec).bv_len as c_int - offset);
                data = bvec_virt(bvec);
                length -= avail;
                (*stream).buf.in = (data as *const u8).add(offset as usize);
                (*stream).buf.in_size = avail as usize;
                (*stream).buf.in_pos = 0;
                offset = 0;
            }

            if (*stream).buf.out_pos == (*stream).buf.out_size {
                (*stream).buf.out = squashfs_next_page(output);
                if IS_ERR((*stream).buf.out) {
                    error = PTR_ERR((*stream).buf.out);
                    break;
                } else if !(*stream).buf.out.is_null() {
                    (*stream).buf.out_pos = 0;
                    total += PAGE_SIZE as c_int;
                }
            }

            xz_err = xz_dec_run((*stream).state, &mut (*stream).buf);
            if xz_err == XZ_STREAM_END {
                break;
            }
            if xz_err != XZ_OK {
                error = -EIO;
                break;
            }
        }
    }

    squashfs_finish_page(output);

    if error != 0 {
        error
    } else {
        total + (*stream).buf.out_pos as c_int
    }
}

const squashfs_xz_comp_ops: squashfs_decompressor = squashfs_decompressor {
    init: Some(squashfs_xz_init),
    comp_opts: Some(squashfs_xz_comp_opts),
    free: Some(squashfs_xz_free),
    decompress: Some(squashfs_xz_uncompress),
    id: XZ_COMPRESSION,
    name: c"xz",
    alloc_buffer: 1,
    supported: 1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
