// SPDX-License-Identifier: GPL-2.0-or-later
/* CacheFiles extended attribute management
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies supplied by the surrounding translation unit.

pub const CACHEFILES_COOKIE_TYPE_DATA: u8 = 1;

#[repr(C, packed)]
pub struct cachefiles_xattr {
    pub object_size: u64,
    pub zero_point: u64,
    pub r#type: u8,
    pub content: u8,
    pub data: [u8; 0],
}

pub static cachefiles_xattr_cache: &[u8] = b"user.CacheFiles.cache\0";

#[repr(C, packed)]
pub struct cachefiles_vol_xattr {
    pub reserved: u32,
    pub data: [u8; 0],
}

/* set the state xattr on a cache file */
pub unsafe fn cachefiles_set_object_xattr(object: *mut cachefiles_object) -> i32 {
    let mut buf: *mut cachefiles_xattr;
    let dentry: *mut dentry;
    let file: *mut file = (*object).file;
    let len: usize = (*(*object).cookie).aux_len as usize;
    let mut ret: i32;

    if file.is_null() {
        return -ESTALE;
    }
    dentry = (*file).f_path.dentry;

    _enter!("%x,#%d", (*object).debug_id, len);

    buf = kmalloc(core::mem::size_of::<cachefiles_xattr>() + len, GFP_KERNEL)
        as *mut cachefiles_xattr;
    if buf.is_null() {
        return -ENOMEM;
    }

    (*buf).object_size = cpu_to_be64((*(*object).cookie).object_size);
    (*buf).zero_point = 0;
    (*buf).r#type = CACHEFILES_COOKIE_TYPE_DATA;
    (*buf).content = (*object).content_info;
    if test_bit(FSCACHE_COOKIE_LOCAL_WRITE, &(*(*object).cookie).flags) {
        (*buf).content = CACHEFILES_CONTENT_DIRTY;
    }
    if len > 0 {
        memcpy((*buf).data.as_mut_ptr(), fscache_get_aux((*object).cookie), len);
    }

    ret = cachefiles_inject_write_error();
    if ret == 0 {
        ret = mnt_want_write_file(file);
        if ret == 0 {
            ret = vfs_setxattr(
                &nop_mnt_idmap,
                dentry,
                cachefiles_xattr_cache,
                buf as *const core::ffi::c_void,
                core::mem::size_of::<cachefiles_xattr>() + len,
                0,
            );
            mnt_drop_write_file(file);
        }
    }
    if ret < 0 {
        trace_cachefiles_vfs_error(object, file_inode(file), ret, cachefiles_trace_setxattr_error);
        trace_cachefiles_coherency(
            object,
            (*file_inode(file)).i_ino,
            be64_to_cpup((*buf).data.as_ptr() as *const u64),
            (*buf).content,
            cachefiles_coherency_set_fail,
        );
        if ret != -ENOMEM {
            cachefiles_io_error_obj(object, "Failed to set xattr with error %d", ret);
        }
    } else {
        trace_cachefiles_coherency(
            object,
            (*file_inode(file)).i_ino,
            be64_to_cpup((*buf).data.as_ptr() as *const u64),
            (*buf).content,
            cachefiles_coherency_set_ok,
        );
    }

    kfree(buf as *mut core::ffi::c_void);
    _leave!(" = %d", ret);
    ret
}

/* check the consistency between the backing cache and the FS-Cache cookie */
pub unsafe fn cachefiles_check_auxdata(object: *mut cachefiles_object, file: *mut file) -> i32 {
    let dentry: *mut dentry = (*file).f_path.dentry;
    let len: usize = (*(*object).cookie).aux_len as usize;
    let p: *const core::ffi::c_void = fscache_get_aux((*object).cookie);
    let mut why: enum_cachefiles_coherency_trace;
    let mut xlen: isize;
    let mut ret: i32 = -ESTALE;
    let tlen = core::mem::size_of::<cachefiles_xattr>() + len;
    let buf = kmalloc(tlen, GFP_KERNEL) as *mut cachefiles_xattr;
    if buf.is_null() { return -ENOMEM; }

    xlen = cachefiles_inject_read_error() as isize;
    if xlen == 0 { xlen = vfs_getxattr(&nop_mnt_idmap, dentry, cachefiles_xattr_cache, buf as *mut _, tlen) as isize; }
    if xlen != tlen as isize {
        if xlen < 0 { ret = xlen as i32; trace_cachefiles_vfs_error(object, file_inode(file), xlen as i32, cachefiles_trace_getxattr_error); }
        if xlen == -EIO as isize { cachefiles_io_error_obj(object, "Failed to read aux with error %zd", xlen as i32); }
        why = cachefiles_coherency_check_xattr;
    } else if (*buf).r#type != CACHEFILES_COOKIE_TYPE_DATA {
        why = cachefiles_coherency_check_type;
    } else if memcmp((*buf).data.as_ptr(), p, len) != 0 {
        why = cachefiles_coherency_check_aux;
    } else if be64_to_cpu((*buf).object_size) != (*(*object).cookie).object_size {
        why = cachefiles_coherency_check_objsize;
    } else if (*buf).content == CACHEFILES_CONTENT_DIRTY {
        // TODO: Begin conflict resolution
        pr_warn!("Dirty object in cache\n");
        why = cachefiles_coherency_check_dirty;
    } else { why = cachefiles_coherency_check_ok; ret = 0; }

    trace_cachefiles_coherency(object, (*file_inode(file)).i_ino, be64_to_cpup((*buf).data.as_ptr() as *const u64), (*buf).content, why);
    kfree(buf as *mut core::ffi::c_void);
    ret
}

/* remove the object's xattr to mark it stale */
pub unsafe fn cachefiles_remove_object_xattr(cache: *mut cachefiles_cache, object: *mut cachefiles_object, dentry: *mut dentry) -> i32 {
    let mut ret = cachefiles_inject_remove_error();
    if ret == 0 { ret = mnt_want_write((*cache).mnt); if ret == 0 { ret = vfs_removexattr(&nop_mnt_idmap, dentry, cachefiles_xattr_cache); mnt_drop_write((*cache).mnt); } }
    if ret < 0 {
        trace_cachefiles_vfs_error(object, d_inode(dentry), ret, cachefiles_trace_remxattr_error);
        if ret == -ENOENT || ret == -ENODATA { ret = 0; }
        else if ret != -ENOMEM { cachefiles_io_error(cache, "Can't remove xattr from %llu (error %d)", d_backing_inode(dentry).i_ino, -ret); }
    }
    _leave!(" = %d", ret); ret
}

/* Stick a marker on the cache object to indicate that it's dirty. */
pub unsafe fn cachefiles_prepare_to_write(cookie: *mut fscache_cookie) {
    let mut saved_cred: *const cred = core::ptr::null();
    let object = (*cookie).cache_priv;
    let cache = (*(*object).volume).cache;
    _enter!("c=%08x", (*(*object).cookie).debug_id);
    if !test_bit(CACHEFILES_OBJECT_USING_TMPFILE, &(*object).flags) { cachefiles_begin_secure(cache, &mut saved_cred); cachefiles_set_object_xattr(object); cachefiles_end_secure(cache, saved_cred); }
}

/* Set the state xattr on a volume directory. */
pub unsafe fn cachefiles_set_volume_xattr(volume: *mut cachefiles_volume) -> bool {
    let mut len = (*(*volume).vcookie).coherency_len as usize;
    let p = (*(*volume).vcookie).coherency;
    let dentry = (*volume).dentry;
    _enter!("%x,#%d", (*(*volume).vcookie).debug_id, len);
    len += core::mem::size_of::<cachefiles_vol_xattr>();
    let buf = kmalloc(len, GFP_KERNEL) as *mut cachefiles_vol_xattr;
    if buf.is_null() { return false; }
    (*buf).reserved = cpu_to_be32(0); memcpy((*buf).data.as_mut_ptr(), p, (*(*volume).vcookie).coherency_len as usize);
    let mut ret = cachefiles_inject_write_error();
    if ret == 0 { ret = mnt_want_write((*(*volume).cache).mnt); if ret == 0 { ret = vfs_setxattr(&nop_mnt_idmap, dentry, cachefiles_xattr_cache, buf as *const _, len, 0); mnt_drop_write((*(*volume).cache).mnt); } }
    if ret < 0 { trace_cachefiles_vfs_error(core::ptr::null_mut(), d_inode(dentry), ret, cachefiles_trace_setxattr_error); trace_cachefiles_vol_coherency(volume, (*d_inode(dentry)).i_ino, cachefiles_coherency_vol_set_fail); if ret != -ENOMEM { cachefiles_io_error((*volume).cache, "Failed to set xattr with error %d", ret); } }
    else { trace_cachefiles_vol_coherency(volume, (*d_inode(dentry)).i_ino, cachefiles_coherency_vol_set_ok); }
    kfree(buf as *mut _); _leave!(" = %d", ret); ret == 0
}

/* Check the consistency between the backing cache and the volume cookie. */
pub unsafe fn cachefiles_check_volume_xattr(volume: *mut cachefiles_volume) -> i32 {
    let mut len = (*(*volume).vcookie).coherency_len as usize;
    let p = (*(*volume).vcookie).coherency;
    let dentry = (*volume).dentry;
    let mut ret = -ESTALE;
    _enter!("");
    len += core::mem::size_of::<cachefiles_vol_xattr>();
    let buf = kmalloc(len, GFP_KERNEL) as *mut cachefiles_vol_xattr;
    if buf.is_null() { return -ENOMEM; }
    let mut xlen = cachefiles_inject_read_error() as isize;
    if xlen == 0 { xlen = vfs_getxattr(&nop_mnt_idmap, dentry, cachefiles_xattr_cache, buf as *mut _, len) as isize; }
    let why: enum_cachefiles_coherency_trace;
    if xlen != len as isize { if xlen < 0 { ret = xlen as i32; trace_cachefiles_vfs_error(core::ptr::null_mut(), d_inode(dentry), xlen as i32, cachefiles_trace_getxattr_error); if xlen == -EIO as isize { cachefiles_io_error((*volume).cache, "Failed to read xattr with error %zd", xlen as i32); } } why = cachefiles_coherency_vol_check_xattr; }
    else if (*buf).reserved != cpu_to_be32(0) { why = cachefiles_coherency_vol_check_resv; }
    else if memcmp((*buf).data.as_ptr(), p, len - core::mem::size_of::<cachefiles_vol_xattr>()) != 0 { why = cachefiles_coherency_vol_check_cmp; }
    else { why = cachefiles_coherency_vol_check_ok; ret = 0; }
    trace_cachefiles_vol_coherency(volume, (*d_inode(dentry)).i_ino, why); kfree(buf as *mut _); _leave!(" = %d", ret); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
