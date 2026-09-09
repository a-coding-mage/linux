// SPDX-License-Identifier: GPL-2.0-or-later
/* CacheFiles security management
 *
 * Copyright (C) 2007, 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the kernel and internal headers are intentionally
// left as external Rust symbols.

/*
 * determine the security context within which we access the cache from within
 * the kernel
 */
pub unsafe fn cachefiles_get_security_ID(cache: *mut cachefiles_cache) -> ::core::ffi::c_int {
    let new: *mut cred;
    let mut ret: ::core::ffi::c_int;

    _enter("{%u}", if (*cache).have_secid { (*cache).secid } else { 0 });

    new = prepare_kernel_cred(current);
    if new.is_null() {
        ret = -ENOMEM;
    } else {
        if (*cache).have_secid {
            ret = set_security_override(new, (*cache).secid);
            if ret < 0 {
                put_cred(new);
                pr_err(
                    "Security denies permission to nominate security context: error %d\n",
                    ret,
                );
            } else {
                (*cache).cache_cred = new;
            }
        } else {
            (*cache).cache_cred = new;
            ret = 0;
        }
    }
    _leave(" = %d", ret);
    ret
}

/*
 * see if mkdir and create can be performed in the root directory
 */
unsafe fn cachefiles_check_cache_dir(
    _cache: *mut cachefiles_cache,
    root: *mut dentry,
) -> ::core::ffi::c_int {
    let mut ret = security_inode_mkdir(d_backing_inode(root), root, 0);
    if ret < 0 {
        pr_err("Security denies permission to make dirs: error %d", ret);
        return ret;
    }

    ret = security_inode_create(d_backing_inode(root), root, 0);
    if ret < 0 {
        pr_err("Security denies permission to create files: error %d", ret);
    }

    ret
}

/*
 * check the security details of the on-disk cache
 * - must be called with security override in force
 * - must return with a security override in force - even in the case of an
 *   error
 */
pub unsafe fn cachefiles_determine_cache_security(
    cache: *mut cachefiles_cache,
    root: *mut dentry,
    _saved_cred: *mut *const cred,
) -> ::core::ffi::c_int {
    _enter("");

    /* duplicate the cache creds for COW (the override is currently in
     * force, so we can use prepare_creds() to do this) */
    let new = prepare_creds();
    if new.is_null() {
        return -ENOMEM;
    }

    cachefiles_end_secure(cache, *_saved_cred);

    /* use the cache root dir's security context as the basis with
     * which create files */
    let mut ret = set_create_files_as(new, d_backing_inode(root));
    if ret < 0 {
        abort_creds(new);
        cachefiles_begin_secure(cache, _saved_cred);
        _leave(" = %d [cfa]", ret);
        return ret;
    }

    put_cred((*cache).cache_cred);
    (*cache).cache_cred = new;

    cachefiles_begin_secure(cache, _saved_cred);
    ret = cachefiles_check_cache_dir(cache, root);

    if ret == -EOPNOTSUPP {
        ret = 0;
    }
    _leave(" = %d", ret);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
