// SPDX-License-Identifier: GPL-2.0-or-later
/* Volume handling.
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/fscache implementation are
// intentionally referenced here rather than reimplemented in this translation.

/*
 * Allocate and set up a volume representation.  We make sure all the fanout
 * directories are created and pinned.
 */
pub unsafe fn cachefiles_acquire_volume(vcookie: *mut fscache_volume) {
    let mut volume: *mut cachefiles_volume;
    let cache: *mut cachefiles_cache = (*(*vcookie).cache).cache_priv;
    let mut saved_cred: *const cred = core::ptr::null();
    let mut vdentry: *mut dentry;
    let mut fan: *mut dentry;
    let len: usize;
    let name: *mut core::ffi::c_char;
    let mut is_new = false;
    let mut ret: i32;
    let mut n_accesses: i32;
    let mut i: i32;

    _enter("");

    volume = kzalloc_obj::<cachefiles_volume>();
    if volume.is_null() {
        return;
    }
    (*volume).vcookie = vcookie;
    (*volume).cache = cache;
    INIT_LIST_HEAD(&mut (*volume).cache_link);

    cachefiles_begin_secure(cache, &mut saved_cred);

    len = (*vcookie).key[0] as usize;
    name = kmalloc(len + 3, GFP_NOFS) as *mut core::ffi::c_char;
    if name.is_null() {
        goto_error_vol(cache, saved_cred, volume);
        return;
    }
    *name = b'I' as core::ffi::c_char;
    core::ptr::copy_nonoverlapping(
        (*vcookie).key.add(1),
        name.add(1) as *mut u8,
        len,
    );
    *name.add(len + 1) = 0;

    'retry: loop {
        vdentry = cachefiles_get_directory(cache, (*cache).store, name, &mut is_new);
        if IS_ERR(vdentry) {
            goto_error_name(cache, saved_cred, volume, name);
            return;
        }
        (*volume).dentry = vdentry;

        if is_new {
            if !cachefiles_set_volume_xattr(volume) {
                goto_error_dir(cache, saved_cred, volume, name);
                return;
            }
        } else {
            ret = cachefiles_check_volume_xattr(volume);
            if ret < 0 {
                if ret != -ESTALE {
                    goto_error_dir(cache, saved_cred, volume, name);
                    return;
                }
                vdentry = start_removing_dentry((*cache).store, vdentry);
                if !IS_ERR(vdentry) {
                    cachefiles_bury_object(
                        cache,
                        core::ptr::null_mut(),
                        (*cache).store,
                        vdentry,
                        FSCACHE_VOLUME_IS_WEIRD,
                    );
                }
                cachefiles_put_directory((*volume).dentry);
                cond_resched();
                continue 'retry;
            }
        }

        i = 0;
        while i < 256 {
            sprintf(name, b"@%02x\0".as_ptr() as *const core::ffi::c_char, i);
            fan = cachefiles_get_directory(cache, vdentry, name, core::ptr::null_mut());
            if IS_ERR(fan) {
                for j in 0..256 {
                    cachefiles_put_directory((*volume).fanout[j as usize]);
                }
                cachefiles_put_directory((*volume).dentry);
                goto_error_name(cache, saved_cred, volume, name);
                return;
            }
            (*volume).fanout[i as usize] = fan;
            i += 1;
        }

        cachefiles_end_secure(cache, saved_cred);

        (*vcookie).cache_priv = volume;
        n_accesses = atomic_inc_return(&mut (*vcookie).n_accesses); /* Stop wakeups on dec-to-0 */
        trace_fscache_access_volume(
            (*vcookie).debug_id,
            0,
            refcount_read(&(*vcookie).ref_()),
            n_accesses,
            fscache_access_cache_pin,
        );

        spin_lock(&mut (*cache).object_list_lock);
        list_add(&mut (*volume).cache_link, &mut (*volume).cache.volumes);
        spin_unlock(&mut (*cache).object_list_lock);

        kfree(name as *mut core::ffi::c_void);
        return;
    }
}

/* Release a volume representation. */
unsafe fn __cachefiles_free_volume(volume: *mut cachefiles_volume) {
    _enter("");

    (*(*volume).vcookie).cache_priv = core::ptr::null_mut();

    for i in 0..256 {
        cachefiles_put_directory((*volume).fanout[i]);
    }
    cachefiles_put_directory((*volume).dentry);
    kfree(volume as *mut core::ffi::c_void);
}

pub unsafe fn cachefiles_free_volume(vcookie: *mut fscache_volume) {
    let volume: *mut cachefiles_volume = (*vcookie).cache_priv;

    if !volume.is_null() {
        spin_lock(&mut (*(*volume).cache).object_list_lock);
        list_del_init(&mut (*volume).cache_link);
        spin_unlock(&mut (*(*volume).cache).object_list_lock);
        __cachefiles_free_volume(volume);
    }
}

pub unsafe fn cachefiles_withdraw_volume(volume: *mut cachefiles_volume) {
    cachefiles_set_volume_xattr(volume);
    __cachefiles_free_volume(volume);
}

// The following cleanup helper calls correspond to the C goto cleanup paths.
unsafe fn goto_error_vol(cache: *mut cachefiles_cache, saved_cred: *const cred, volume: *mut cachefiles_volume) {
    kfree(volume as *mut core::ffi::c_void);
    cachefiles_end_secure(cache, saved_cred);
}

unsafe fn goto_error_name(cache: *mut cachefiles_cache, saved_cred: *const cred, volume: *mut cachefiles_volume, name: *mut core::ffi::c_char) {
    kfree(name as *mut core::ffi::c_void);
    goto_error_vol(cache, saved_cred, volume);
}

unsafe fn goto_error_dir(cache: *mut cachefiles_cache, saved_cred: *const cred, volume: *mut cachefiles_volume, name: *mut core::ffi::c_char) {
    cachefiles_put_directory((*volume).dentry);
    goto_error_name(cache, saved_cred, volume, name);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
