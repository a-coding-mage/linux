// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2006 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding NFS implementation.

const NFSDBG_FACILITY: u32 = NFSDBG_CLIENT;

extern "C" {
    fn nfs_alloc_fattr() -> *mut nfs_fattr;
    fn nfs4_proc_get_rootfh(
        server: *mut nfs_server,
        mntfh: *mut nfs_fh,
        fattr: *mut nfs_fattr,
        auth_probe: bool,
    ) -> i32;
    fn nfs_free_fattr(fattr: *mut nfs_fattr);
    fn dprintk(fmt: *const core::ffi::c_char, ...);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
}

extern "C" {
    static NFSDBG_CLIENT: u32;
    static NFS_ATTR_FATTR_TYPE: u32;
    static ENOMEM: i32;
    static ENOTDIR: i32;
}

#[repr(C)]
pub struct nfs_server {
    pub fsid: nfs_fsid,
}

#[repr(C)]
pub struct nfs_fattr {
    pub valid: u32,
    pub mode: u32,
    pub fsid: nfs_fsid,
}

#[repr(C)]
pub struct nfs_fh {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfs_fsid {
    pub data: [u64; 2],
}

#[inline]
unsafe fn s_isdir(mode: u32) -> bool {
    (mode & 0o170000) == 0o040000
}

pub unsafe fn nfs4_get_rootfh(
    server: *mut nfs_server,
    mntfh: *mut nfs_fh,
    auth_probe: bool,
) -> i32 {
    let fattr = nfs_alloc_fattr();
    let mut ret: i32 = -ENOMEM;

    if fattr.is_null() {
        nfs_free_fattr(fattr);
        return ret;
    }

    // Start by getting the root filehandle from the server
    ret = nfs4_proc_get_rootfh(server, mntfh, fattr, auth_probe);
    if ret < 0 {
        dprintk(
            b"nfs4_get_rootfh: getroot error = %d\n\0".as_ptr() as *const core::ffi::c_char,
            -ret,
        );
        nfs_free_fattr(fattr);
        return ret;
    }

    if ((*fattr).valid & NFS_ATTR_FATTR_TYPE) == 0 || !s_isdir((*fattr).mode) {
        printk(
            b"nfs4_get_rootfh: getroot encountered non-directory\n\0".as_ptr()
                as *const core::ffi::c_char,
        );
        ret = -ENOTDIR;
        nfs_free_fattr(fattr);
        return ret;
    }

    memcpy(
        core::ptr::addr_of_mut!((*server).fsid) as *mut core::ffi::c_void,
        core::ptr::addr_of!((*fattr).fsid) as *const core::ffi::c_void,
        core::mem::size_of::<nfs_fsid>(),
    );
    nfs_free_fattr(fattr);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
