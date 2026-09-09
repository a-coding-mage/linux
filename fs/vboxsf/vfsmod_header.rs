/* SPDX-License-Identifier: MIT */
/*
 * VirtualBox Guest Shared Folders support: module header.
 *
 * Copyright (C) 2006-2018 Oracle Corporation
 */

// Translated from vfsmod.h. Linux and project-defined types are supplied by
// the surrounding translation unit.

pub const DIR_BUFFER_SIZE: usize = 16 * 1024;

#[inline]
pub unsafe fn VBOXSF_SBI(sb: *mut super_block) -> *mut vboxsf_sbi {
    (*sb).s_fs_info as *mut vboxsf_sbi
}

#[inline]
pub unsafe fn VBOXSF_I(i: *mut inode) -> *mut vboxsf_inode {
    container_of(i, vboxsf_inode, vfs_inode)
}

#[repr(C)]
pub struct vboxsf_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vboxsf_options {
    pub ttl: ::core::ffi::c_ulong,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub dmode_set: bool,
    pub fmode_set: bool,
    pub dmode: umode_t,
    pub fmode: umode_t,
    pub dmask: umode_t,
    pub fmask: umode_t,
}

#[repr(C)]
pub struct vboxsf_fs_context {
    pub o: vboxsf_options,
    pub nls_name: *mut ::core::ffi::c_char,
}

#[repr(C)]
pub struct vboxsf_sbi {
    pub o: vboxsf_options,
    pub root_info: shfl_fsobjinfo,
    pub ino_idr: idr,
    pub ino_idr_lock: spinlock_t,
    pub nls: *mut nls_table,
    pub next_generation: u32,
    pub root: u32,
    pub bdi_id: ::core::ffi::c_int,
    pub case_insensitive: bool,
}

#[repr(C)]
pub struct vboxsf_inode {
    pub force_restat: ::core::ffi::c_int,
    pub handle_list: list_head,
    pub handle_list_mutex: mutex,
    pub vfs_inode: inode,
}

#[repr(C)]
pub struct vboxsf_dir_info {
    pub info_list: list_head,
}

#[repr(C)]
pub struct vboxsf_dir_buf {
    pub entries: usize,
    pub free: usize,
    pub used: usize,
    pub buf: *mut ::core::ffi::c_void,
    pub head: list_head,
}

extern "C" {
    pub static vboxsf_dir_iops: inode_operations;
    pub static vboxsf_lnk_iops: inode_operations;
    pub static vboxsf_reg_iops: inode_operations;
    pub static vboxsf_dir_fops: file_operations;
    pub static vboxsf_reg_fops: file_operations;
    pub static vboxsf_reg_aops: address_space_operations;
    pub static vboxsf_dentry_ops: dentry_operations;

    pub fn vboxsf_create_sf_handle(inode: *mut inode, handle: u64, access_flags: u32) -> *mut vboxsf_handle;
    pub fn vboxsf_release_sf_handle(inode: *mut inode, sf_handle: *mut vboxsf_handle);
    pub fn vboxsf_new_inode(sb: *mut super_block) -> *mut inode;
    pub fn vboxsf_init_inode(sbi: *mut vboxsf_sbi, inode: *mut inode, info: *const shfl_fsobjinfo, reinit: bool) -> ::core::ffi::c_int;
    pub fn vboxsf_create_at_dentry(dentry: *mut dentry, params: *mut shfl_createparms) -> ::core::ffi::c_int;
    pub fn vboxsf_stat(sbi: *mut vboxsf_sbi, path: *mut shfl_string, info: *mut shfl_fsobjinfo) -> ::core::ffi::c_int;
    pub fn vboxsf_stat_dentry(dentry: *mut dentry, info: *mut shfl_fsobjinfo) -> ::core::ffi::c_int;
    pub fn vboxsf_inode_revalidate(dentry: *mut dentry) -> ::core::ffi::c_int;
    pub fn vboxsf_getattr(idmap: *mut mnt_idmap, path: *const path, kstat: *mut kstat, request_mask: u32, query_flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn vboxsf_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> ::core::ffi::c_int;
    pub fn vboxsf_path_from_dentry(sbi: *mut vboxsf_sbi, dentry: *mut dentry) -> *mut shfl_string;
    pub fn vboxsf_nlscpy(sbi: *mut vboxsf_sbi, name: *mut ::core::ffi::c_char, name_bound_len: usize, utf8_name: *const u8, utf8_len: usize) -> ::core::ffi::c_int;
    pub fn vboxsf_dir_info_alloc() -> *mut vboxsf_dir_info;
    pub fn vboxsf_dir_info_free(p: *mut vboxsf_dir_info);
    pub fn vboxsf_dir_read_all(sbi: *mut vboxsf_sbi, sf_d: *mut vboxsf_dir_info, handle: u64) -> ::core::ffi::c_int;
    pub fn vboxsf_query_case_sensitive(sbi: *mut vboxsf_sbi) -> ::core::ffi::c_int;
    pub fn vboxsf_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> ::core::ffi::c_int;

    pub fn vboxsf_connect() -> ::core::ffi::c_int;
    pub fn vboxsf_disconnect();
    pub fn vboxsf_create(root: u32, parsed_path: *mut shfl_string, create_parms: *mut shfl_createparms) -> ::core::ffi::c_int;
    pub fn vboxsf_close(root: u32, handle: u64) -> ::core::ffi::c_int;
    pub fn vboxsf_remove(root: u32, parsed_path: *mut shfl_string, flags: u32) -> ::core::ffi::c_int;
    pub fn vboxsf_rename(root: u32, src_path: *mut shfl_string, dest_path: *mut shfl_string, flags: u32) -> ::core::ffi::c_int;
    pub fn vboxsf_read(root: u32, handle: u64, offset: u64, buf_len: *mut u32, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn vboxsf_write(root: u32, handle: u64, offset: u64, buf_len: *mut u32, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn vboxsf_dirinfo(root: u32, handle: u64, parsed_path: *mut shfl_string, flags: u32, index: u32, buf_len: *mut u32, buf: *mut shfl_dirinfo, file_count: *mut u32) -> ::core::ffi::c_int;
    pub fn vboxsf_fsinfo(root: u32, handle: u64, flags: u32, buf_len: *mut u32, buf: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn vboxsf_map_folder(folder_name: *mut shfl_string, root: *mut u32) -> ::core::ffi::c_int;
    pub fn vboxsf_unmap_folder(root: u32) -> ::core::ffi::c_int;
    pub fn vboxsf_readlink(root: u32, parsed_path: *mut shfl_string, buf_len: u32, buf: *mut u8) -> ::core::ffi::c_int;
    pub fn vboxsf_symlink(root: u32, new_path: *mut shfl_string, old_path: *mut shfl_string, buf: *mut shfl_fsobjinfo) -> ::core::ffi::c_int;
    pub fn vboxsf_set_utf8() -> ::core::ffi::c_int;
    pub fn vboxsf_set_symlinks() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
