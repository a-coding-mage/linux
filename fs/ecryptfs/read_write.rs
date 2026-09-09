// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 2007 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
 */

// Linux kernel dependencies supplied by the surrounding translation.

use core::ffi::c_void;

pub type LoffT = i64;
pub type SizeT = usize;
pub type PgoffT = usize;

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Mapping {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Inode {
    pub i_mapping: *mut Mapping,
}

#[repr(C)]
pub struct Folio {
    pub index: PgoffT,
}

#[repr(C)]
pub struct EcryptfsCryptStat {
    pub flags: u32,
}

#[repr(C)]
pub struct EcryptfsInodePrivate {
    pub lower_file: *mut File,
    pub crypt_stat: EcryptfsCryptStat,
}

pub const EIO: i32 = 5;
pub const EINTR: i32 = 4;
pub const ECRYPTFS_ENCRYPTED: u32 = 1 << 0;
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

extern "C" {
    fn ecryptfs_inode_to_private(inode: *mut Inode) -> *mut EcryptfsInodePrivate;
    fn kernel_write(file: *mut File, data: *mut u8, size: SizeT, offset: *mut LoffT) -> isize;
    fn kernel_read(file: *mut File, data: *mut u8, size: SizeT, offset: *mut LoffT) -> isize;
    fn mark_inode_dirty_sync(inode: *mut Inode);
    fn i_size_read(inode: *mut Inode) -> LoffT;
    fn i_size_write(inode: *mut Inode, size: LoffT);
    fn fatal_signal_pending(task: *mut c_void) -> bool;
    static mut current: *mut c_void;
    fn read_mapping_folio(mapping: *mut Mapping, index: PgoffT, fgp: *mut c_void) -> *mut Folio;
    fn is_err(ptr: *mut Folio) -> bool;
    fn ptr_err(ptr: *mut Folio) -> i32;
    fn folio_lock(folio: *mut Folio);
    fn kmap_local_folio(folio: *mut Folio, offset: usize) -> *mut u8;
    fn kunmap_local(addr: *mut u8);
    fn memset(dst: *mut u8, value: i32, len: SizeT) -> *mut c_void;
    fn memcpy(dst: *mut u8, src: *const u8, len: SizeT) -> *mut c_void;
    fn flush_dcache_folio(folio: *mut Folio);
    fn folio_mark_uptodate(folio: *mut Folio);
    fn folio_unlock(folio: *mut Folio);
    fn ecryptfs_encrypt_page(folio: *mut Folio) -> i32;
    fn folio_put(folio: *mut Folio);
    fn ecryptfs_write_inode_size_to_metadata(inode: *mut Inode) -> i32;
    fn printk(fmt: *const u8, ...);
}

pub unsafe fn ecryptfs_write_lower(
    ecryptfs_inode: *mut Inode,
    data: *mut u8,
    mut offset: LoffT,
    size: SizeT,
) -> i32 {
    let lower_file = (*ecryptfs_inode_to_private(ecryptfs_inode)).lower_file;
    if lower_file.is_null() {
        return -EIO;
    }
    let rc = kernel_write(lower_file, data, size, &mut offset) as i32;
    mark_inode_dirty_sync(ecryptfs_inode);
    rc
}

pub unsafe fn ecryptfs_write_lower_page_segment(
    ecryptfs_inode: *mut Inode,
    folio_for_lower: *mut Folio,
    offset_in_page: SizeT,
    size: SizeT,
) -> i32 {
    let offset = (*folio_for_lower).index as LoffT * PAGE_SIZE as LoffT + offset_in_page as LoffT;
    let virt = kmap_local_folio(folio_for_lower, 0);
    let mut rc = ecryptfs_write_lower(ecryptfs_inode, virt, offset, size);
    if rc > 0 { rc = 0; }
    kunmap_local(virt);
    rc
}

pub unsafe fn ecryptfs_write(
    ecryptfs_inode: *mut Inode, data: *mut u8, offset: LoffT, size: SizeT,
) -> i32 {
    let crypt_stat = &mut (*ecryptfs_inode_to_private(ecryptfs_inode)).crypt_stat;
    let ecryptfs_file_size = i_size_read(ecryptfs_inode);
    let mut data_offset: LoffT = 0;
    let mut pos = if offset > ecryptfs_file_size { ecryptfs_file_size } else { offset };
    let mut rc = 0;
    while pos < offset + size as LoffT {
        let ecryptfs_page_idx = (pos >> PAGE_SHIFT) as PgoffT;
        let start_offset_in_page = (pos as usize) & !PAGE_MASK;
        let mut num_bytes = PAGE_SIZE - start_offset_in_page;
        let total_remaining_bytes = offset + size as LoffT - pos;
        if fatal_signal_pending(current) { rc = -EINTR; break; }
        if num_bytes as LoffT > total_remaining_bytes { num_bytes = total_remaining_bytes as SizeT; }
        if pos < offset {
            let total_remaining_zeros = offset - pos;
            if num_bytes as LoffT > total_remaining_zeros { num_bytes = total_remaining_zeros as SizeT; }
        }
        let folio = read_mapping_folio((*ecryptfs_inode).i_mapping, ecryptfs_page_idx, core::ptr::null_mut());
        if is_err(folio) { rc = ptr_err(folio); break; }
        folio_lock(folio);
        let virt = kmap_local_folio(folio, 0);
        if pos < offset || start_offset_in_page == 0 {
            memset(virt.add(start_offset_in_page), 0, PAGE_SIZE - start_offset_in_page);
        }
        if pos >= offset {
            memcpy(virt.add(start_offset_in_page), data.add(data_offset as usize), num_bytes);
            data_offset += num_bytes as LoffT;
        }
        kunmap_local(virt);
        flush_dcache_folio(folio);
        folio_mark_uptodate(folio);
        folio_unlock(folio);
        if crypt_stat.flags & ECRYPTFS_ENCRYPTED != 0 {
            rc = ecryptfs_encrypt_page(folio);
        } else {
            rc = ecryptfs_write_lower_page_segment(ecryptfs_inode, folio, start_offset_in_page, data_offset as SizeT);
        }
        folio_put(folio);
        if rc != 0 { break; }
        pos += num_bytes as LoffT;
    }
    if pos > ecryptfs_file_size {
        i_size_write(ecryptfs_inode, pos);
        if crypt_stat.flags & ECRYPTFS_ENCRYPTED != 0 {
            let rc2 = ecryptfs_write_inode_size_to_metadata(ecryptfs_inode);
            if rc2 != 0 && rc == 0 { rc = rc2; }
        }
    }
    rc
}

pub unsafe fn ecryptfs_read_lower(
    data: *mut u8, mut offset: LoffT, size: SizeT, ecryptfs_inode: *mut Inode,
) -> i32 {
    let lower_file = (*ecryptfs_inode_to_private(ecryptfs_inode)).lower_file;
    if lower_file.is_null() { return -EIO; }
    kernel_read(lower_file, data, size, &mut offset) as i32
}

pub unsafe fn ecryptfs_read_lower_page_segment(
    folio_for_ecryptfs: *mut Folio, page_index: PgoffT,
    offset_in_page: SizeT, size: SizeT, ecryptfs_inode: *mut Inode,
) -> i32 {
    let offset = page_index as LoffT * PAGE_SIZE as LoffT + offset_in_page as LoffT;
    let virt = kmap_local_folio(folio_for_ecryptfs, 0);
    let mut rc = ecryptfs_read_lower(virt, offset, size, ecryptfs_inode);
    if rc > 0 { rc = 0; }
    kunmap_local(virt);
    flush_dcache_folio(folio_for_ecryptfs);
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
