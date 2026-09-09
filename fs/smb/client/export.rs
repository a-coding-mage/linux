// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 *   Common Internet FileSystem (CIFS) client
 *
 *   Operations related to support for exporting files via NFSD
 *
 */

/*
 * See Documentation/filesystems/nfs/exporting.rst
 * and examples in fs/exportfs
 *
 * Since cifs is a network file system, an "fsid" must be included for
 * any nfs exports file entries which refer to cifs paths.  The cifs mount
 * must be mounted with the "serverino" option (ie use stable
 * server inode numbers instead of locally generated temporary ones).
 * Although cifs inodes do not use generation numbers (have generation number
 * of zero) - the inode number alone should be good enough for simple cases
 * in which users want to export cifs shares with NFS. The decode and encode
 * could be improved by using a new routine which expects 64 bit inode numbers
 * instead of the default 32 bit routines in fs/exportfs
 */

use core::ffi::{c_char, c_int, c_uint};

// Declarations supplied by the included kernel headers.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

pub type __u32 = c_uint;

unsafe extern "C" {
    fn cifs_dbg(level: c_int, format: *const c_char, ...);
    fn generic_encode_ino32_fh(
        inode: *mut inode,
        fh: *mut __u32,
        max_len: *mut c_int,
        parent: *mut inode,
    ) -> c_int;
}

// FYI is a cifs debugging level supplied by cifs_debug.h.
const FYI: c_int = 0;

#[repr(C)]
pub struct export_operations {
    pub encode_fh: Option<
        unsafe extern "C" fn(
            inode: *mut inode,
            fh: *mut __u32,
            max_len: *mut c_int,
            parent: *mut inode,
        ) -> c_int,
    >,
    pub get_parent: Option<unsafe extern "C" fn(dentry: *mut dentry) -> *mut dentry>,
}

#[cfg(CONFIG_CIFS_NFSD_EXPORT)]
unsafe extern "C" fn cifs_get_parent(dentry: *mut dentry) -> *mut dentry {
    /* BB need to add code here eventually to enable export via NFSD */
    static MESSAGE: &[u8] = b"get parent for %p\n\0";
    unsafe {
        cifs_dbg(FYI, MESSAGE.as_ptr() as *const c_char, dentry);
    }
    // ERR_PTR(-EACCES)
    (-13isize) as *mut dentry
}

#[cfg(CONFIG_CIFS_NFSD_EXPORT)]
#[no_mangle]
pub static cifs_export_ops: export_operations = export_operations {
    encode_fh: Some(generic_encode_ino32_fh),
    get_parent: Some(cifs_get_parent),
};


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
