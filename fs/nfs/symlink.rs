// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/nfs/symlink.c
 *
 *  Copyright (C) 1992  Rick Sladkey
 *
 *  Optimization changes Copyright (C) 1994 Florian La Roche
 *
 *  Jun 7 1999, cache symlink lookups in the page cache.  -DaveM
 *
 *  nfs symlink handling code
 */

// Linux kernel headers and "internal.h" supply the declarations used here.

/* Symlink caching in the page cache is even more simplistic
 * and straight-forward than readdir caching.
 */

unsafe fn nfs_symlink_filler(file: *mut file, folio: *mut folio) -> i32 {
    let inode: *mut inode = (*(*folio).mapping).host;
    let error: i32;

    error = ((*NFS_PROTO(inode)).readlink)(inode, &mut (*folio).page, 0, PAGE_SIZE);
    folio_end_read(folio, error == 0);
    error
}

unsafe fn nfs_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const u8 {
    let folio: *mut folio;
    let mut err: *mut core::ffi::c_void;

    if dentry.is_null() {
        err = ERR_PTR(nfs_revalidate_mapping_rcu(inode));
        if !err.is_null() {
            return err as *const u8;
        }
        folio = filemap_get_folio((*inode).i_mapping, 0);
        if IS_ERR(folio) {
            return ERR_PTR(-ECHILD) as *const u8;
        }
        if !folio_test_uptodate(folio) {
            folio_put(folio);
            return ERR_PTR(-ECHILD) as *const u8;
        }
    } else {
        err = ERR_PTR(nfs_revalidate_mapping(inode, (*inode).i_mapping));
        if !err.is_null() {
            return err as *const u8;
        }
        folio = read_cache_folio(
            &mut (*inode).i_data,
            0,
            Some(nfs_symlink_filler),
            core::ptr::null_mut(),
        );
        if IS_ERR(folio) {
            return ERR_CAST(folio) as *const u8;
        }
    }
    set_delayed_call(done, Some(page_put_link), folio);
    folio_address(folio)
}

/*
 * symlinks can't do much...
 */
#[repr(C)]
pub struct inode_operations {
    pub get_link: Option<unsafe fn(*mut dentry, *mut inode, *mut delayed_call) -> *const u8>,
    pub getattr: Option<unsafe fn()>,
    pub setattr: Option<unsafe fn()>,
    pub fileattr_get: Option<unsafe fn()>,
}

#[no_mangle]
pub static nfs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(nfs_get_link),
    getattr: Some(nfs_getattr),
    setattr: Some(nfs_setattr),
    fileattr_get: Some(nfs_fileattr_get),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
