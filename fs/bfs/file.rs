// SPDX-License-Identifier: GPL-2.0
/*
 * fs/bfs/file.c
 * BFS file operations.
 * Copyright (C) 1999-2018 Tigran Aivazian <aivazian.tigran@gmail.com>
 *
 * Make the file block allocation algorithm understand the size
 * of the underlying block device.
 * Copyright (C) 2007 Dmitri Vorobiev <dmitri.vorobiev@gmail.com>
 */

// Linux kernel and bfs.h declarations are supplied by other translation units.

use core::ffi::c_void;

type CInt = i32;
type CUint = u32;
type CULong = usize;
type SectorT = u64;
type LoffT = i64;

#[repr(C)]
pub struct FileOperations {
    pub llseek: Option<unsafe extern "C" fn()>,
    pub read_iter: Option<unsafe extern "C" fn()>,
    pub write_iter: Option<unsafe extern "C" fn()>,
    pub mmap_prepare: Option<unsafe extern "C" fn()>,
    pub splice_read: Option<unsafe extern "C" fn()>,
}

#[repr(C)]
pub struct AddressSpaceOperations {
    pub dirty_folio: Option<unsafe extern "C" fn()>,
    pub invalidate_folio: Option<unsafe extern "C" fn()>,
    pub read_folio: Option<unsafe extern "C" fn()>,
    pub writepages: Option<unsafe extern "C" fn()>,
    pub write_begin: Option<unsafe extern "C" fn()>,
    pub write_end: Option<unsafe extern "C" fn()>,
    pub migrate_folio: Option<unsafe extern "C" fn()>,
    pub bmap: Option<unsafe extern "C" fn()>,
}

#[repr(C)] pub struct File { _private: [u8; 0] }
#[repr(C)] pub struct Folio { _private: [u8; 0] }
#[repr(C)] pub struct Kiocb { _private: [u8; 0] }
#[repr(C)] pub struct BufferHead { pub b_data: *mut u8, pub b_size: CUint }
#[repr(C)] pub struct SuperBlock { _private: [u8; 0] }
#[repr(C)] pub struct AddressSpace { pub host: *mut Inode }
#[repr(C)] pub struct WritebackControl { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct BfsSbInfo { pub bfs_lock: Mutex, pub si_blocks: CULong, pub si_lf_eblk: CULong, pub si_freeb: CULong }
#[repr(C)] pub struct BfsInodeInfo { pub i_sblock: CULong, pub i_eblock: CULong }
#[repr(C)] pub struct Inode { pub i_sb: *mut SuperBlock, pub i_ino: CULong, pub i_blocks: CULong, pub i_size: LoffT }

extern "C" {
    fn generic_file_llseek(); fn generic_file_read_iter(); fn generic_file_write_iter();
    fn generic_file_mmap_prepare(); fn filemap_splice_read();
    fn sb_bread(sb: *mut SuperBlock, block: CULong) -> *mut BufferHead;
    fn sb_getblk(sb: *mut SuperBlock, block: CULong) -> *mut BufferHead;
    fn mark_buffer_dirty(bh: *mut BufferHead); fn bforget(bh: *mut BufferHead); fn brelse(bh: *mut BufferHead);
    fn map_bh(result: *mut BufferHead, sb: *mut SuperBlock, block: CULong);
    fn mutex_lock(lock: *mut Mutex); fn mutex_unlock(lock: *mut Mutex);
    fn mark_inode_dirty(inode: *mut Inode);
    fn mpage_writepages(mapping: *mut AddressSpace, wbc: *mut WritebackControl, get_block: unsafe extern "C" fn(*mut Inode, SectorT, *mut BufferHead, CInt) -> CInt) -> CInt;
    fn block_read_full_folio(folio: *mut Folio, get_block: unsafe extern "C" fn(*mut Inode, SectorT, *mut BufferHead, CInt) -> CInt) -> CInt;
    fn truncate_pagecache(inode: *mut Inode, size: LoffT);
    fn block_write_begin(mapping: *mut AddressSpace, pos: LoffT, len: CUint, foliop: *mut *mut Folio, get_block: unsafe extern "C" fn(*mut Inode, SectorT, *mut BufferHead, CInt) -> CInt) -> CInt;
    fn generic_block_bmap(mapping: *mut AddressSpace, block: SectorT, get_block: unsafe extern "C" fn(*mut Inode, SectorT, *mut BufferHead, CInt) -> CInt) -> SectorT;
    fn block_dirty_folio(); fn block_invalidate_folio(); fn generic_write_end(); fn buffer_migrate_folio();
    fn BFS_SB(sb: *mut SuperBlock) -> *mut BfsSbInfo;
    fn BFS_I(inode: *mut Inode) -> *mut BfsInodeInfo;
}

pub static bfs_file_operations: FileOperations = FileOperations {
    llseek: Some(generic_file_llseek), read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter), mmap_prepare: Some(generic_file_mmap_prepare),
    splice_read: Some(filemap_splice_read),
};

unsafe extern "C" fn bfs_move_block(from: CULong, to: CULong, sb: *mut SuperBlock) -> CInt {
    let bh = sb_bread(sb, from); if bh.is_null() { return -5; }
    let new = sb_getblk(sb, to);
    core::ptr::copy_nonoverlapping((*bh).b_data, (*new).b_data, (*bh).b_size as usize);
    mark_buffer_dirty(new); bforget(bh); brelse(new); 0
}

unsafe extern "C" fn bfs_move_blocks(sb: *mut SuperBlock, start: CULong, end: CULong, where_: CULong) -> CInt {
    let mut i = start; while i <= end { if bfs_move_block(i, where_ + i, sb) != 0 { return -5; } i += 1; } 0
}

unsafe extern "C" fn bfs_get_block(inode: *mut Inode, block: SectorT, bh_result: *mut BufferHead, create: CInt) -> CInt {
    let sb = (*inode).i_sb; let info = BFS_SB(sb); let bi = BFS_I(inode);
    let mut phys = (*bi).i_sblock + block as usize;
    if create == 0 { if phys <= (*bi).i_eblock { map_bh(bh_result, sb, phys); } return 0; }
    if (*bi).i_sblock != 0 && phys <= (*bi).i_eblock { map_bh(bh_result, sb, phys); return 0; }
    if phys >= (*info).si_blocks { return -28; }
    mutex_lock(&mut (*info).bfs_lock);
    let err: CInt;
    if (*bi).i_eblock == (*info).si_lf_eblk {
        map_bh(bh_result, sb, phys); (*info).si_freeb -= phys - (*bi).i_eblock;
        (*info).si_lf_eblk = phys; (*bi).i_eblock = phys; mark_inode_dirty(inode); err = 0;
    } else {
        phys = (*info).si_lf_eblk + 1;
        if phys + block as usize >= (*info).si_blocks { mutex_unlock(&mut (*info).bfs_lock); return -28; }
        if (*bi).i_sblock != 0 { let e = bfs_move_blocks(sb, (*bi).i_sblock, (*bi).i_eblock, phys); if e != 0 { mutex_unlock(&mut (*info).bfs_lock); return e; } }
        (*bi).i_sblock = phys; phys += block as usize; (*info).si_lf_eblk = phys; (*bi).i_eblock = phys;
        (*info).si_freeb -= (*bi).i_eblock - (*bi).i_sblock + 1 - (*inode).i_blocks;
        mark_inode_dirty(inode); map_bh(bh_result, sb, phys); err = 0;
    }
    mutex_unlock(&mut (*info).bfs_lock); err
}

unsafe extern "C" fn bfs_writepages(mapping: *mut AddressSpace, wbc: *mut WritebackControl) -> CInt { mpage_writepages(mapping, wbc, bfs_get_block) }
unsafe extern "C" fn bfs_read_folio(_file: *mut File, folio: *mut Folio) -> CInt { block_read_full_folio(folio, bfs_get_block) }
unsafe extern "C" fn bfs_write_failed(mapping: *mut AddressSpace, to: LoffT) { let inode = (*mapping).host; if to > (*inode).i_size { truncate_pagecache(inode, (*inode).i_size); } }
unsafe extern "C" fn bfs_write_begin(_iocb: *const Kiocb, mapping: *mut AddressSpace, pos: LoffT, len: CUint, foliop: *mut *mut Folio, _fsdata: *mut *mut c_void) -> CInt { let ret = block_write_begin(mapping, pos, len, foliop, bfs_get_block); if ret != 0 { bfs_write_failed(mapping, pos + len as i64); } ret }
unsafe extern "C" fn bfs_bmap(mapping: *mut AddressSpace, block: SectorT) -> SectorT { generic_block_bmap(mapping, block, bfs_get_block) }

pub static bfs_aops: AddressSpaceOperations = AddressSpaceOperations { dirty_folio: Some(block_dirty_folio), invalidate_folio: Some(block_invalidate_folio), read_folio: Some(bfs_read_folio), writepages: Some(bfs_writepages), write_begin: Some(bfs_write_begin), write_end: Some(generic_write_end), migrate_folio: Some(buffer_migrate_folio), bmap: Some(bfs_bmap) };
pub static bfs_file_inops: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
