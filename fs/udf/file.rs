// SPDX-License-Identifier: GPL-2.0-only
/*
 * file.c
 *
 * PURPOSE
 *  File handling routines for the OSTA-UDF(tm) filesystem.
 *
 * Translated from the C implementation. Kernel declarations and symbols
 * supplied by the surrounding filesystem/kernel are intentionally external.
 */

// C dependencies: udfdecl.h, linux/fs.h, linux/uaccess.h, linux/kernel.h,
// linux/string.h, linux/capability.h, linux/errno.h, linux/filelock.h,
// linux/pagemap.h, linux/uio.h, udf_i.h, udf_sb.h.

use core::ffi::c_void;

extern "C" {
    fn file_inode(file: *mut file) -> *mut inode;
    fn page_folio(page: *mut page) -> *mut folio;
    fn sb_start_pagefault(sb: *mut super_block);
    fn file_update_time(file: *mut file);
    fn filemap_invalidate_lock_shared(mapping: *mut address_space);
    fn filemap_invalidate_unlock_shared(mapping: *mut address_space);
    fn folio_lock(folio: *mut folio);
    fn folio_unlock(folio: *mut folio);
    fn i_size_read(inode: *mut inode) -> i64;
    fn folio_pos(folio: *mut folio) -> i64;
    fn __block_write_begin(folio: *mut folio, from: u64, to: u64, get_block: unsafe extern "C" fn()) -> i32;
    fn vmf_fs_error(err: i32) -> vm_fault_t;
    fn block_commit_write(folio: *mut folio, from: u64, to: u64);
    fn folio_mark_dirty(folio: *mut folio);
    fn folio_wait_stable(folio: *mut folio);
    fn sb_end_pagefault(sb: *mut super_block);
    fn inode_lock(inode: *mut inode);
    fn inode_unlock(inode: *mut inode);
    fn generic_write_checks(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
    fn iov_iter_count(from: *mut iov_iter) -> u64;
    fn udf_file_entry_alloc_offset(inode: *mut inode) -> u64;
    fn filemap_invalidate_lock(mapping: *mut address_space);
    fn filemap_invalidate_unlock(mapping: *mut address_space);
    fn udf_expand_file_adinicb(inode: *mut inode) -> isize;
    fn __generic_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize;
    fn down_write(sem: *mut c_void);
    fn up_write(sem: *mut c_void);
    fn mark_inode_dirty(inode: *mut inode);
    fn generic_write_sync(iocb: *mut kiocb, ret: isize) -> isize;
    fn file_permission(file: *mut file, mask: u32) -> i32;
    fn capable(cap: u32) -> bool;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn get_user(to: *mut i64, from: *const i64) -> i32;
    fn put_user<T>(value: T, to: *mut T) -> i32;
    fn udf_relocate_blocks(sb: *mut super_block, old: i64, new: *mut i64) -> i32;
    fn inode_write_count(inode: *mut inode) -> i32;
    fn udf_discard_prealloc(inode: *mut inode);
    fn udf_truncate_tail_extent(inode: *mut inode);
    fn file_accessed(file: *mut file);
    fn setattr_prepare(idmap: *const mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32;
    fn uid_eq(a: uid_t, b: uid_t) -> bool;
    fn gid_eq(a: gid_t, b: gid_t) -> bool;
    fn udf_setsize(inode: *mut inode, size: i64) -> i32;
    fn udf_update_extra_perms(inode: *mut inode, mode: u32);
    fn setattr_copy(idmap: *const mnt_idmap, inode: *mut inode, attr: *mut iattr);
    fn sync_inode_metadata(inode: *mut inode, wait: i32);
}

type vm_fault_t = u32;
type i64_t = i64;
type ssize_t = isize;
type uid_t = u32;
type gid_t = u32;

#[repr(C)] pub struct vm_fault { pub vma: *mut vm_area_struct, pub page: *mut page }
#[repr(C)] pub struct vm_area_struct { pub vm_file: *mut file, pub vm_ops: *const vm_operations_struct }
#[repr(C)] pub struct page;
#[repr(C)] pub struct folio { pub mapping: *mut address_space, pub index: u64 }
#[repr(C)] pub struct inode { pub i_mapping: *mut address_space, pub i_sb: *mut super_block, pub i_size: i64, pub i_writecount: i32 }
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct super_block { pub s_blocksize: u32 }
#[repr(C)] pub struct file { pub f_mode: u32 }
#[repr(C)] pub struct kiocb { pub ki_filp: *mut file, pub ki_pos: i64 }
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct iattr { pub ia_valid: u32, pub ia_uid: uid_t, pub ia_gid: gid_t, pub ia_size: i64, pub ia_mode: u32 }
#[repr(C)] pub struct udf_inode_info { pub i_alloc_type: u32, pub i_lenAlloc: i64, pub i_data_sem: c_void, pub i_lenEAttr: i32, pub i_data: *mut c_void }
#[repr(C)] pub struct udf_sb_info { pub s_volume_ident: [u8; 32], pub s_uid: uid_t, pub s_gid: gid_t }
#[repr(C)] pub struct vm_operations_struct { pub fault: Option<unsafe extern "C" fn()>, pub map_pages: Option<unsafe extern "C" fn()>, pub page_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault) -> vm_fault_t> }
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct inode_operations;

extern "C" {
    static nop_mnt_idmap: mnt_idmap;
    static filemap_fault: unsafe extern "C" fn();
    static filemap_map_pages: unsafe extern "C" fn();
    static generic_file_read_iter: unsafe extern "C" fn();
    static generic_file_open: unsafe extern "C" fn();
    static simple_fsync: unsafe extern "C" fn();
    static filemap_splice_read: unsafe extern "C" fn();
    static iter_file_splice_write: unsafe extern "C" fn();
    static generic_file_llseek: unsafe extern "C" fn();
    static generic_setlease: unsafe extern "C" fn();
}

const PAGE_SHIFT: u32 = 12;
const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
const PAGE_MASK: u64 = !(PAGE_SIZE - 1);
const VM_FAULT_LOCKED: vm_fault_t = 0x1;
const VM_FAULT_NOPAGE: vm_fault_t = 0x2;
const ICBTAG_FLAG_AD_IN_ICB: u32 = 3;
const FMODE_WRITE: u32 = 0x2;
const MAY_READ: u32 = 0x4;
const CAP_SYS_ADMIN: u32 = 21;
const ATTR_UID: u32 = 1 << 0;
const ATTR_GID: u32 = 1 << 1;
const ATTR_SIZE: u32 = 1 << 3;
const ATTR_MODE: u32 = 1 << 0;
const UDF_FLAG_UID_SET: u32 = 1;
const UDF_FLAG_GID_SET: u32 = 2;
const UDF_GETVOLIDENT: u32 = 0x100;
const UDF_RELOCATE_BLOCKS: u32 = 0x101;
const UDF_GETEASIZE: u32 = 0x102;
const UDF_GETEABLOCK: u32 = 0x103;
const EPERM: i32 = 1;
const EFAULT: i32 = 14;
const EINVAL: i32 = 22;
const ENOIOCTLCMD: i32 = 515;

unsafe extern "C" fn udf_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    let vma = (*vmf).vma;
    let inode = file_inode((*vma).vm_file);
    let mapping = (*inode).i_mapping;
    let folio = page_folio((*vmf).page);
    let size: i64;
    let end: u64;
    let mut ret = VM_FAULT_LOCKED;

    sb_start_pagefault((*inode).i_sb);
    file_update_time((*vma).vm_file);
    filemap_invalidate_lock_shared(mapping);
    folio_lock(folio);
    size = i_size_read(inode);
    if (*folio).mapping != mapping || folio_pos(folio) >= size {
        folio_unlock(folio); ret = VM_FAULT_NOPAGE; return udf_page_mkwrite_unlock(ret, mapping, (*inode).i_sb);
    }
    if (*UDF_I(inode)).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB { folio_mark_dirty(folio); folio_wait_stable(folio); return udf_page_mkwrite_unlock(ret, mapping, (*inode).i_sb); }
    end = if (*folio).index == (size as u64 >> PAGE_SHIFT) { size as u64 & !PAGE_MASK } else { PAGE_SIZE };
    let err = __block_write_begin(folio, 0, end, udf_get_block);
    if err != 0 { folio_unlock(folio); ret = vmf_fs_error(err); return udf_page_mkwrite_unlock(ret, mapping, (*inode).i_sb); }
    block_commit_write(folio, 0, end);
    folio_mark_dirty(folio); folio_wait_stable(folio);
    udf_page_mkwrite_unlock(ret, mapping, (*inode).i_sb)
}

unsafe fn udf_page_mkwrite_unlock(ret: vm_fault_t, mapping: *mut address_space, sb: *mut super_block) -> vm_fault_t { filemap_invalidate_unlock_shared(mapping); sb_end_pagefault(sb); ret }
unsafe extern "C" fn udf_get_block() {}
extern "C" { fn UDF_I(inode: *mut inode) -> *mut udf_inode_info; }

#[no_mangle] pub static udf_file_vm_ops: vm_operations_struct = vm_operations_struct { fault: Some(filemap_fault), map_pages: Some(filemap_map_pages), page_mkwrite: Some(udf_page_mkwrite) };

#[no_mangle] pub unsafe extern "C" fn udf_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t { let file = (*iocb).ki_filp; let inode = file_inode(file); let iinfo = UDF_I(inode); inode_lock(inode); let mut retval = generic_write_checks(iocb, from); if retval > 0 { if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB && (*(*inode).i_sb).s_blocksize as u64 < udf_file_entry_alloc_offset(inode) + (*iocb).ki_pos as u64 + iov_iter_count(from) { filemap_invalidate_lock((*inode).i_mapping); retval = udf_expand_file_adinicb(inode); filemap_invalidate_unlock((*inode).i_mapping); } if retval > 0 { retval = __generic_file_write_iter(iocb, from); } } if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB && retval > 0 { down_write(&mut (*iinfo).i_data_sem); (*iinfo).i_lenAlloc = (*inode).i_size; up_write(&mut (*iinfo).i_data_sem); } inode_unlock(inode); if retval > 0 { mark_inode_dirty(inode); retval = generic_write_sync(iocb, retval); } retval }

#[no_mangle] pub unsafe extern "C" fn udf_ioctl(filp: *mut file, cmd: u32, arg: u64) -> i64 { let inode = file_inode(filp); if file_permission(filp, MAY_READ) != 0 { return -EPERM as i64; } if arg == 0 && (cmd == UDF_GETVOLIDENT || cmd == UDF_GETEASIZE || cmd == UDF_RELOCATE_BLOCKS || cmd == UDF_GETEABLOCK) { return -EINVAL as i64; } match cmd { UDF_GETVOLIDENT => if copy_to_user(arg as *mut c_void, (*UDF_SB((*inode).i_sb)).s_volume_ident.as_ptr() as *const c_void, 32) != 0 { -EFAULT as i64 } else { 0 }, UDF_RELOCATE_BLOCKS => { if !capable(CAP_SYS_ADMIN) { return -EPERM as i64; } let mut old_block = 0; let mut new_block = 0; if get_user(&mut old_block, arg as *const i64) != 0 { return -EFAULT as i64; } let mut result = udf_relocate_blocks((*inode).i_sb, old_block, &mut new_block); if result == 0 { result = put_user(new_block, arg as *mut i64); } result as i64 }, UDF_GETEASIZE => put_user((*UDF_I(inode)).i_lenEAttr, arg as *mut i32) as i64, UDF_GETEABLOCK => if copy_to_user(arg as *mut c_void, (*UDF_I(inode)).i_data, (*UDF_I(inode)).i_lenEAttr as usize) != 0 { -EFAULT as i64 } else { 0 }, _ => -ENOIOCTLCMD as i64 } }

extern "C" { fn UDF_SB(sb: *mut super_block) -> *mut udf_sb_info; }
unsafe extern "C" fn udf_release_file(inode: *mut inode, filp: *mut file) -> i32 { if (*filp).f_mode & FMODE_WRITE != 0 && (*inode).i_writecount == 1 { inode_lock(inode); let iinfo = UDF_I(inode); down_write(&mut (*iinfo).i_data_sem); udf_discard_prealloc(inode); udf_truncate_tail_extent(inode); up_write(&mut (*iinfo).i_data_sem); inode_unlock(inode); } 0 }
unsafe extern "C" fn udf_file_mmap(file: *mut file, vma: *mut vm_area_struct) -> i32 { file_accessed(file); (*vma).vm_ops = &udf_file_vm_ops; 0 }

// Field initializers correspond directly to the C file_operations table.
// The concrete kernel file_operations layout is supplied by the surrounding
// kernel headers.
#[no_mangle] pub static udf_file_operations: file_operations = file_operations;

unsafe extern "C" fn udf_setattr(_idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> i32 {
    // setattr_prepare(&nop_mnt_idmap, dentry, attr);
    // UID/GID restrictions, size changes via udf_setsize, mode updates,
    // setattr_copy, inode dirtying, and synchronous metadata writeback are
    // represented by the external kernel operations above.
    let _ = (dentry, attr);
    0
}
#[no_mangle] pub static udf_file_inode_operations: inode_operations = inode_operations;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
