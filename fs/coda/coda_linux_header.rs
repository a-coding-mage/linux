/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Coda File System, Linux Kernel module
 *
 * Original version, adapted from cfs_mach.c, (C) Carnegie Mellon University
 * Linux modifications (C) 1996, Peter J. Braam
 * Rewritten for Linux 2.1 (C) 1997 Carnegie Mellon University
 *
 * Carnegie Mellon University encourages users of this software to
 * contribute improvements to the Coda project.
 */

/* C header dependencies: linux/kernel.h, linux/param.h, linux/mm.h,
 * linux/vmalloc.h, linux/slab.h, linux/wait.h, linux/types.h, linux/fs.h,
 * and coda_fs_i.h supply the referenced types and symbols. */

/* #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt */

/* operations */
extern "C" {
    pub static coda_dir_inode_operations: inode_operations;
    pub static coda_file_inode_operations: inode_operations;
    pub static coda_ioctl_inode_operations: inode_operations;

    pub static coda_dentry_operations: dentry_operations;

    pub static coda_file_aops: address_space_operations;
    pub static coda_symlink_aops: address_space_operations;

    pub static coda_dir_operations: file_operations;
    pub static coda_file_operations: file_operations;
    pub static coda_ioctl_operations: file_operations;

    /* operations shared over more than one file */
    pub fn coda_open(i: *mut inode, f: *mut file) -> i32;
    pub fn coda_release(i: *mut inode, f: *mut file) -> i32;
    pub fn coda_permission(idmap: *mut mnt_idmap, inode: *mut inode, mask: i32) -> i32;
    pub fn coda_revalidate_inode(inode: *mut inode) -> i32;
    pub fn coda_getattr(
        idmap: *mut mnt_idmap,
        path: *const path,
        stat: *mut kstat,
        request_mask: u32,
        query_flags: u32,
    ) -> i32;
    pub fn coda_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> i32;

    /* this file: helpers */
    pub fn coda_f2s(f: *mut CodaFid) -> *mut c_char;
    pub fn coda_iscontrol(name: *const c_char, length: usize) -> i32;

    pub fn coda_inode_type(attr: *mut coda_vattr) -> umode_t;
    pub fn coda_vattr_to_iattr(inode: *mut inode, attr: *mut coda_vattr);
    pub fn coda_iattr_to_vattr(iattr: *mut iattr, attr: *mut coda_vattr);
    pub fn coda_flags_to_cflags(flags: u16) -> u16;
}

/* inode to cnode access functions */

#[inline]
pub unsafe fn ITOC(inode: *mut inode) -> *mut coda_inode_info {
    /* Equivalent to Linux's container_of(inode, coda_inode_info, vfs_inode). */
    crate::container_of!(inode, coda_inode_info, vfs_inode)
}

#[inline]
pub unsafe fn coda_i2f(inode: *mut inode) -> *mut CodaFid {
    &mut (*ITOC(inode)).c_fid
}

#[inline]
pub unsafe fn coda_i2s(inode: *mut inode) -> *mut c_char {
    coda_f2s(&mut (*ITOC(inode)).c_fid)
}

/* this will not zap the inode away */
#[inline]
pub unsafe fn coda_flag_inode(inode: *mut inode, flag: i32) {
    let cii: *mut coda_inode_info = ITOC(inode);

    if inode.is_null() {
        return;
    }

    spin_lock(&mut (*cii).c_lock);
    (*cii).c_flags |= flag;
    spin_unlock(&mut (*cii).c_lock);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
