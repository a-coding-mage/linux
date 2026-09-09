// SPDX-License-Identifier: GPL-2.0
/*
 * FUSE passthrough to backing file.
 *
 * Copyright (c) 2023 CTERA Networks.
 */

// Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn fuse_backing_get(fb: *mut fuse_backing) -> *mut fuse_backing {
    if !fb.is_null() && refcount_inc_not_zero(unsafe { &mut (*fb).count }) {
        fb
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn fuse_backing_free(fb: *mut fuse_backing) {
    pr_debug!("%s: fb=0x%p\n", c"fuse_backing_free", fb);

    if !(*fb).file.is_null() {
        fput((*fb).file);
    }
    put_cred((*fb).cred);
    kfree_rcu(fb, rcu);
}

pub unsafe fn fuse_backing_put(fb: *mut fuse_backing) {
    if !fb.is_null() && refcount_dec_and_test(unsafe { &mut (*fb).count }) {
        fuse_backing_free(fb);
    }
}

pub unsafe fn fuse_backing_files_init(fc: *mut fuse_conn) {
    idr_init(unsafe { &mut (*fc).backing_files_map });
}

unsafe fn fuse_backing_id_alloc(fc: *mut fuse_conn, fb: *mut fuse_backing) -> i32 {
    let id: i32;

    idr_preload(GFP_KERNEL);
    spin_lock(&mut (*fc).lock);
    /* FIXME: xarray might be space inefficient */
    id = idr_alloc_cyclic(
        &mut (*fc).backing_files_map,
        fb,
        1,
        0,
        GFP_ATOMIC,
    );
    spin_unlock(&mut (*fc).lock);
    idr_preload_end();

    WARN_ON_ONCE(id == 0);
    id
}

unsafe fn fuse_backing_id_remove(fc: *mut fuse_conn, id: i32) -> *mut fuse_backing {
    let fb: *mut fuse_backing;

    spin_lock(&mut (*fc).lock);
    fb = idr_remove(&mut (*fc).backing_files_map, id);
    spin_unlock(&mut (*fc).lock);

    fb
}

unsafe extern "C" fn fuse_backing_id_free(id: i32, p: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    let fb = p as *mut fuse_backing;

    WARN_ON_ONCE(refcount_read(&(*fb).count) != 1);
    fuse_backing_free(fb);
    0
}

pub unsafe fn fuse_backing_files_free(fc: *mut fuse_conn) {
    idr_for_each(&mut (*fc).backing_files_map, fuse_backing_id_free, core::ptr::null_mut());
    idr_destroy(&mut (*fc).backing_files_map);
}

pub unsafe fn fuse_backing_open(fc: *mut fuse_conn, map: *mut fuse_backing_map) -> i32 {
    let file: *mut file;
    let backing_sb: *mut super_block;
    let mut fb: *mut fuse_backing = core::ptr::null_mut();
    let mut res: i32;

    pr_debug!("%s: fd=%d flags=0x%x\n", c"fuse_backing_open", (*map).fd, (*map).flags);

    /* TODO: relax CAP_SYS_ADMIN once backing files are visible to lsof */
    res = -EPERM;
    if !(*fc).passthrough || !capable(CAP_SYS_ADMIN) {
        return res;
    }

    res = -EINVAL;
    if (*map).flags != 0 || (*map).padding != 0 {
        return res;
    }

    file = fget_raw((*map).fd);
    res = -EBADF;
    if file.is_null() {
        return res;
    }

    /* read/write/splice/mmap passthrough only relevant for regular files */
    res = if d_is_dir((*file).f_path.dentry) { -EISDIR } else { -EINVAL };
    if !d_is_reg((*file).f_path.dentry) {
        fput(file);
        return res;
    }

    backing_sb = (*file_inode(file)).i_sb;
    res = -ELOOP;
    if (*backing_sb).s_stack_depth >= (*fc).max_stack_depth {
        fput(file);
        return res;
    }

    fb = kmalloc_obj::<fuse_backing>();
    res = -ENOMEM;
    if fb.is_null() {
        fput(file);
        return res;
    }

    (*fb).file = file;
    (*fb).cred = get_current_cred();
    refcount_set(&mut (*fb).count, 1);

    res = fuse_backing_id_alloc(fc, fb);
    if res < 0 {
        fuse_backing_free(fb);
        fb = core::ptr::null_mut();
    }

    pr_debug!("%s: fb=0x%p, ret=%i\n", c"fuse_backing_open", fb, res);
    res
}

pub unsafe fn fuse_backing_close(fc: *mut fuse_conn, backing_id: i32) -> i32 {
    let mut fb: *mut fuse_backing = core::ptr::null_mut();
    let mut err: i32;

    pr_debug!("%s: backing_id=%d\n", c"fuse_backing_close", backing_id);

    /* TODO: relax CAP_SYS_ADMIN once backing files are visible to lsof */
    err = -EPERM;
    if !(*fc).passthrough || !capable(CAP_SYS_ADMIN) {
        return err;
    }

    err = -EINVAL;
    if backing_id <= 0 {
        return err;
    }

    err = -ENOENT;
    fb = fuse_backing_id_remove(fc, backing_id);
    if fb.is_null() {
        return err;
    }

    fuse_backing_put(fb);
    err = 0;
    pr_debug!("%s: fb=0x%p, err=%i\n", c"fuse_backing_close", fb, err);
    err
}

pub unsafe fn fuse_backing_lookup(fc: *mut fuse_conn, backing_id: i32) -> *mut fuse_backing {
    let mut fb: *mut fuse_backing;

    rcu_read_lock();
    fb = idr_find(&(*fc).backing_files_map, backing_id);
    fb = fuse_backing_get(fb);
    rcu_read_unlock();

    fb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
