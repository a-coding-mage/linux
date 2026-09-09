// SPDX-License-Identifier: GPL-2.0-only
/*
 * V9FS cache definitions.
 *
 *  Copyright (C) 2009 by Abhishek Kulkarni <adkulkar@umail.iu.edu>
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn v9fs_cache_session_get_cookie(
    v9ses: *mut v9fs_session_info,
    dev_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut vcookie: *mut fscache_volume;
    let name: *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char;

    name = kasprintf(
        GFP_KERNEL,
        b"9p,%s,%s\0".as_ptr() as *const ::core::ffi::c_char,
        dev_name,
        if (*v9ses).cachetag.is_null() {
            (*v9ses).aname
        } else {
            (*v9ses).cachetag
        },
    );
    if name.is_null() {
        return -ENOMEM;
    }

    p = name;
    while *p != 0 {
        if *p == b'/' as ::core::ffi::c_char {
            *p = b';' as ::core::ffi::c_char;
        }
        p = p.add(1);
    }

    vcookie = fscache_acquire_volume(name, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    p9_debug(
        P9_DEBUG_FSC,
        b"session %p get volume %p (%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
        v9ses,
        vcookie,
        name,
    );
    if IS_ERR(vcookie) {
        if vcookie != ERR_PTR(-EBUSY) {
            kfree(name as *mut ::core::ffi::c_void);
            return PTR_ERR(vcookie);
        }
        pr_err(
            b"Cache volume key already in use (%s)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            name,
        );
        vcookie = core::ptr::null_mut();
    }
    (*v9ses).fscache = vcookie;
    kfree(name as *mut ::core::ffi::c_void);
    0
}

pub unsafe fn v9fs_cache_inode_get_cookie(inode: *mut inode) {
    let v9inode: *mut v9fs_inode = V9FS_I(inode);
    let v9ses: *mut v9fs_session_info;
    let version: __le32;
    let path: __le64;

    if !S_ISREG((*inode).i_mode) {
        return;
    }
    if WARN_ON(v9fs_inode_cookie(v9inode)) {
        return;
    }

    version = cpu_to_le32((*v9inode).qid.version);
    path = cpu_to_le64((*v9inode).qid.path);
    v9ses = v9fs_inode2v9ses(inode);
    (*v9inode).netfs.cache = fscache_acquire_cookie(
        v9fs_session_cache(v9ses),
        0,
        &path as *const __le64 as *const ::core::ffi::c_void,
        core::mem::size_of::<__le64>(),
        &version as *const __le32 as *const ::core::ffi::c_void,
        core::mem::size_of::<__le32>(),
        i_size_read(&mut (*v9inode).netfs.inode),
    );
    if !(*v9inode).netfs.cache.is_null() {
        mapping_set_release_always((*inode).i_mapping);
    }

    p9_debug(
        P9_DEBUG_FSC,
        b"inode %p get cookie %p\n\0".as_ptr() as *const ::core::ffi::c_char,
        inode,
        v9fs_inode_cookie(v9inode),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
