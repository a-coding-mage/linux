/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (c) International Business Machines  Corp., 2002, 2007
 *   Author(s): Steve French (sfrench@us.ibm.com)
 */

// Translated from cifsfs.h. Linux kernel dependencies are supplied externally.

pub const ROOT_I: ::core::ffi::c_uint = 2;

unsafe extern "C" {
    pub static mut cifs_sillycounter: atomic_t;
    pub static mut cifs_tmpcounter: atomic_t;
}

/*
 * ino_t is 32-bits on 32-bit arch. We have to squash the 64-bit value down
 * so that it will fit. We use hash_64 to convert the value to 31 bits, and
 * then add 1, to ensure that we don't end up with a 0 as the value.
 */
#[inline]
pub unsafe fn cifs_uniqueid_to_ino_t(fileid: u64) -> ino_t {
    if core::mem::size_of::<ino_t>() < core::mem::size_of::<u64>() {
        return (hash_64(fileid, (core::mem::size_of::<ino_t>() * 8 - 1) as _) + 1) as ino_t;
    }
    fileid as ino_t
}

#[inline]
pub unsafe fn cifs_set_time(dentry: *mut dentry, time: ::core::ffi::c_ulong) {
    (*dentry).d_fsdata = time as *mut ::core::ffi::c_void;
}

#[inline]
pub unsafe fn cifs_get_time(dentry: *mut dentry) -> ::core::ffi::c_ulong {
    (*dentry).d_fsdata as ::core::ffi::c_ulong
}

unsafe extern "C" {
    pub static mut cifs_fs_type: file_system_type;
    pub static mut smb3_fs_type: file_system_type;
    pub static cifs_addr_ops: address_space_operations;
    pub static cifs_addr_ops_smallbuf: address_space_operations;

    pub fn cifs_sb_active(sb: *mut super_block);
    pub fn cifs_sb_deactive(sb: *mut super_block);

    pub static cifs_dir_inode_ops: inode_operations;
    pub fn cifs_root_iget(sb: *mut super_block) -> *mut inode;
    pub fn cifs_create(idmap: *mut mnt_idmap, dir: *mut inode, direntry: *mut dentry, mode: umode_t) -> ::core::ffi::c_int;
    pub fn cifs_atomic_open(dir: *mut inode, direntry: *mut dentry, file: *mut file, oflags: ::core::ffi::c_uint, mode: umode_t) -> ::core::ffi::c_int;
    pub fn cifs_tmpfile(idmap: *mut mnt_idmap, dir: *mut inode, file: *mut file, mode: umode_t) -> ::core::ffi::c_int;
    pub fn cifs_lookup(parent_dir_inode: *mut inode, direntry: *mut dentry, flags: ::core::ffi::c_uint) -> *mut dentry;
    pub fn cifs_unlink(dir: *mut inode, dentry: *mut dentry) -> ::core::ffi::c_int;
    pub fn cifs_hardlink(old_file: *mut dentry, inode: *mut inode, direntry: *mut dentry) -> ::core::ffi::c_int;
    pub fn cifs_mknod(idmap: *mut mnt_idmap, inode: *mut inode, direntry: *mut dentry, mode: umode_t, device_number: dev_t) -> ::core::ffi::c_int;
    pub fn cifs_mkdir(idmap: *mut mnt_idmap, inode: *mut inode, direntry: *mut dentry, mode: umode_t) -> *mut dentry;
    pub fn cifs_rmdir(inode: *mut inode, direntry: *mut dentry) -> ::core::ffi::c_int;
    pub fn cifs_rename2(idmap: *mut mnt_idmap, source_dir: *mut inode, source_dentry: *mut dentry, target_dir: *mut inode, target_dentry: *mut dentry, flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn cifs_revalidate_file_attr(filp: *mut file) -> ::core::ffi::c_int;
    pub fn cifs_revalidate_dentry_attr(dentry: *mut dentry) -> ::core::ffi::c_int;
    pub fn cifs_revalidate_file(filp: *mut file) -> ::core::ffi::c_int;
    pub fn cifs_revalidate_dentry(dentry: *mut dentry) -> ::core::ffi::c_int;
    pub fn cifs_revalidate_mapping(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn cifs_zap_mapping(inode: *mut inode) -> ::core::ffi::c_int;
    pub fn cifs_getattr(idmap: *mut mnt_idmap, path: *const path, stat: *mut kstat, request_mask: u32, flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn cifs_setattr(idmap: *mut mnt_idmap, direntry: *mut dentry, attrs: *mut iattr) -> ::core::ffi::c_int;
    pub fn cifs_fiemap(inode: *mut inode, fei: *mut fiemap_extent_info, start: u64, len: u64) -> ::core::ffi::c_int;

    pub static cifs_file_inode_ops: inode_operations;
    pub static cifs_symlink_inode_ops: inode_operations;
    pub static cifs_namespace_inode_operations: inode_operations;
    pub fn cifs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> ::core::ffi::c_int;

    pub static cifs_req_ops: netfs_request_ops;
    pub static cifs_file_ops: file_operations;
    pub static cifs_file_direct_ops: file_operations;
    pub static cifs_file_strict_ops: file_operations;
    pub static cifs_file_nobrl_ops: file_operations;
    pub static cifs_file_direct_nobrl_ops: file_operations;
    pub static cifs_file_strict_nobrl_ops: file_operations;
    pub fn cifs_open(inode: *mut inode, file: *mut file) -> ::core::ffi::c_int;
    pub fn cifs_close(inode: *mut inode, file: *mut file) -> ::core::ffi::c_int;
    pub fn cifs_closedir(inode: *mut inode, file: *mut file) -> ::core::ffi::c_int;
    pub fn cifs_strict_readv(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t;
    pub fn cifs_strict_writev(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t;
    pub fn cifs_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t;
    pub fn cifs_direct_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t;
    pub fn cifs_loose_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t;
    pub fn cifs_flock(file: *mut file, cmd: ::core::ffi::c_int, fl: *mut file_lock) -> ::core::ffi::c_int;
    pub fn cifs_lock(file: *mut file, cmd: ::core::ffi::c_int, flock: *mut file_lock) -> ::core::ffi::c_int;
    pub fn cifs_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cifs_strict_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn cifs_flush(file: *mut file, id: fl_owner_t) -> ::core::ffi::c_int;
    pub fn cifs_file_mmap_prepare(desc: *mut vm_area_desc) -> ::core::ffi::c_int;
    pub fn cifs_file_strict_mmap_prepare(desc: *mut vm_area_desc) -> ::core::ffi::c_int;
    pub static cifs_dir_ops: file_operations;
    pub fn cifs_readdir(file: *mut file, ctx: *mut dir_context) -> ::core::ffi::c_int;

    pub static cifs_dentry_ops: dentry_operations;
    pub static cifs_ci_dentry_ops: dentry_operations;
    pub fn cifs_d_automount(path: *mut path) -> *mut vfsmount;
    pub fn cifs_get_link(dentry: *mut dentry, inode: *mut inode, done: *mut delayed_call) -> *const ::core::ffi::c_char;
    pub fn cifs_symlink(idmap: *mut mnt_idmap, inode: *mut inode, direntry: *mut dentry, symname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}

// CONFIG_CIFS_XATTR controls these declarations in the original header.
#[cfg(CONFIG_CIFS_XATTR)]
unsafe extern "C" {
    pub static cifs_xattr_handlers: *const *const xattr_handler;
    pub fn cifs_listxattr(direntry: *mut dentry, data: *mut ::core::ffi::c_char, buf_size: usize) -> ssize_t;
}
#[cfg(not(CONFIG_CIFS_XATTR))]
pub const cifs_xattr_handlers: *const *const xattr_handler = core::ptr::null();
#[cfg(not(CONFIG_CIFS_XATTR))]
pub const cifs_listxattr: Option<unsafe extern "C" fn(*mut dentry, *mut ::core::ffi::c_char, usize) -> ssize_t> = None;

unsafe extern "C" {
    pub fn cifs_file_copychunk_range(xid: ::core::ffi::c_uint, src_file: *mut file, off: loff_t, dst_file: *mut file, destoff: loff_t, len: usize, flags: ::core::ffi::c_uint) -> ssize_t;
    pub fn cifs_ioctl(filep: *mut file, command: ::core::ffi::c_uint, arg: ::core::ffi::c_ulong) -> ::core::ffi::c_long;
    pub fn cifs_setsize(inode: *mut inode, offset: loff_t);
    pub fn cifs_resize_file_locked(inode: *mut inode, offset: loff_t);
    pub fn cifs_smb3_do_mount(fc: *mut fs_context, old_ctx: *mut smb3_fs_context) -> *mut dentry;
    pub fn cifs_silly_fullpath(dentry: *mut dentry) -> *mut ::core::ffi::c_char;
}

pub const CIFS_TMPNAME_PREFIX: &str = ".__smbfile_tmp";
pub const CIFS_TMPNAME_LEN: usize = DNAME_INLINE_LEN - 1;
pub const CIFS_SILLYNAME_PREFIX: &str = ".__smbfile_silly";
pub const CIFS_SILLYNAME_LEN: usize = DNAME_INLINE_LEN - 1;

// CONFIG_CIFS_NFSD_EXPORT controls this declaration in the original header.
#[cfg(CONFIG_CIFS_NFSD_EXPORT)]
unsafe extern "C" {
    pub static cifs_export_ops: export_operations;
}

pub const SMB3_PRODUCT_BUILD: ::core::ffi::c_uint = 61;
pub const CIFS_VERSION: &str = "2.61";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
