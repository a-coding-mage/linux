// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2021-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */
// Dependencies supplied by the surrounding XFS translation.

/*
 * XFS Blob Storage
 * ================
 * Stores and retrieves blobs using an xfile.  Objects are appended to the file
 * and the offset is returned as a magic cookie for retrieval.
 */

pub const XB_KEY_MAGIC: u32 = 0xABAADDAD;

#[repr(C, packed)]
pub struct xb_key {
    pub xb_magic: u32,  // XB_KEY_MAGIC
    pub xb_size: u32,   // size of the blob, in bytes
    pub xb_offset: i64, // byte offset of this key
    // blob comes after here
}

/* Initialize a blob storage object. */
pub unsafe fn xfblob_create(
    description: *const core::ffi::c_char,
    blobp: *mut *mut xfblob,
) -> i32 {
    let mut xfile: *mut xfile = core::ptr::null_mut();
    let mut error: i32;

    error = xfile_create(description, 0, &mut xfile);
    if error != 0 {
        return error;
    }

    let blob = kmalloc_obj::<xfblob>(XCHK_GFP_FLAGS);
    if blob.is_null() {
        error = -ENOMEM;
        xfile_destroy(xfile);
        return error;
    }

    (*blob).xfile = xfile;
    (*blob).last_offset = PAGE_SIZE;

    *blobp = blob;
    0
}

/* Destroy a blob storage object. */
pub unsafe fn xfblob_destroy(blob: *mut xfblob) {
    xfile_destroy((*blob).xfile);
    kfree(blob);
}

/* Retrieve a blob. */
pub unsafe fn xfblob_load(
    blob: *mut xfblob,
    cookie: xfblob_cookie,
    ptr: *mut core::ffi::c_void,
    size: u32,
) -> i32 {
    let mut key = core::mem::MaybeUninit::<xb_key>::uninit();
    let error = xfile_load(
        (*blob).xfile,
        key.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of::<xb_key>(),
        cookie,
    );
    if error != 0 {
        return error;
    }

    let key = key.assume_init();
    if key.xb_magic != XB_KEY_MAGIC || key.xb_offset != cookie {
        ASSERT(0);
        return -ENODATA;
    }
    if size < key.xb_size {
        ASSERT(0);
        return -EFBIG;
    }

    xfile_load(
        (*blob).xfile,
        ptr,
        key.xb_size as usize,
        cookie + core::mem::size_of::<xb_key>() as xfblob_cookie,
    )
}

/* Store a blob. */
pub unsafe fn xfblob_store(
    blob: *mut xfblob,
    cookie: *mut xfblob_cookie,
    ptr: *const core::ffi::c_void,
    size: u32,
) -> i32 {
    let key = xb_key {
        xb_offset: (*blob).last_offset,
        xb_magic: XB_KEY_MAGIC,
        xb_size: size,
    };
    let mut pos = (*blob).last_offset;
    let mut error = xfile_store(
        (*blob).xfile,
        &key as *const xb_key as *const core::ffi::c_void,
        core::mem::size_of::<xb_key>(),
        pos,
    );
    if error != 0 {
        return error;
    }

    pos += core::mem::size_of::<xb_key>() as i64;
    error = xfile_store((*blob).xfile, ptr, size as usize, pos);
    if error != 0 {
        xfile_discard(
            (*blob).xfile,
            (*blob).last_offset,
            core::mem::size_of::<xb_key>(),
        );
        return error;
    }

    *cookie = (*blob).last_offset;
    (*blob).last_offset += core::mem::size_of::<xb_key>() as i64 + size as i64;
    0
}

/* Free a blob. */
pub unsafe fn xfblob_free(blob: *mut xfblob, cookie: xfblob_cookie) -> i32 {
    let mut key = core::mem::MaybeUninit::<xb_key>::uninit();
    let error = xfile_load(
        (*blob).xfile,
        key.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of::<xb_key>(),
        cookie,
    );
    if error != 0 {
        return error;
    }

    let key = key.assume_init();
    if key.xb_magic != XB_KEY_MAGIC || key.xb_offset != cookie {
        ASSERT(0);
        return -ENODATA;
    }

    xfile_discard(
        (*blob).xfile,
        cookie,
        core::mem::size_of::<xb_key>() + key.xb_size as usize,
    );
    0
}

/* How many bytes is this blob storage object consuming? */
pub unsafe fn xfblob_bytes(blob: *mut xfblob) -> u64 {
    xfile_bytes((*blob).xfile)
}

/* Drop all the blobs. */
pub unsafe fn xfblob_truncate(blob: *mut xfblob) {
    xfile_discard((*blob).xfile, PAGE_SIZE, MAX_LFS_FILESIZE - PAGE_SIZE);
    (*blob).last_offset = PAGE_SIZE;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
