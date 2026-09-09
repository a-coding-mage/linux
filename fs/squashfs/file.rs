// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Squashfs - a compressed read only filesystem for Linux
 *
 * Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008
 * Phillip Lougher <phillip@squashfs.org.uk>
 *
 * file.c
 */

/* This file contains code for handling regular files. */

// Kernel and Squashfs types, constants, functions, and macros are supplied by
// the surrounding translation unit/build environment.

unsafe fn locate_meta_index(inode: *mut inode, mut offset: i32, index: i32) -> *mut meta_index {
    let mut meta: *mut meta_index = core::ptr::null_mut();
    let msblk = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info;
    mutex_lock(&mut (*msblk).meta_index_mutex);
    TRACE!("locate_meta_index: index %d, offset %d\n", index, offset);
    if (*msblk).meta_index.is_null() { mutex_unlock(&mut (*msblk).meta_index_mutex); return meta; }
    for i in 0..SQUASHFS_META_SLOTS {
        let p = (*msblk).meta_index.add(i as usize);
        if (*p).inode_number == (*inode).i_ino && (*p).offset >= offset &&
            (*p).offset <= index && (*p).locked == 0 {
            TRACE!("locate_meta_index: entry %d, offset %d\n", i, (*p).offset);
            meta = p; offset = (*p).offset;
        }
    }
    if !meta.is_null() { (*meta).locked = 1; }
    mutex_unlock(&mut (*msblk).meta_index_mutex);
    meta
}

unsafe fn empty_meta_index(inode: *mut inode, offset: i32, skip: i32) -> *mut meta_index {
    let msblk = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info;
    let mut meta: *mut meta_index = core::ptr::null_mut();
    mutex_lock(&mut (*msblk).meta_index_mutex);
    TRACE!("empty_meta_index: offset %d, skip %d\n", offset, skip);
    if (*msblk).meta_index.is_null() {
        (*msblk).meta_index = kzalloc_objs!((*msblk).meta_index, SQUASHFS_META_SLOTS);
        if (*msblk).meta_index.is_null() { ERROR!("Failed to allocate meta_index\n"); mutex_unlock(&mut (*msblk).meta_index_mutex); return meta; }
        for i in 0..SQUASHFS_META_SLOTS { (*(*msblk).meta_index.add(i as usize)).inode_number = 0; (*(*msblk).meta_index.add(i as usize)).locked = 0; }
        (*msblk).next_meta_index = 0;
    }
    let mut i = SQUASHFS_META_SLOTS;
    while i != 0 && (*(*msblk).meta_index.add((*msblk).next_meta_index as usize)).locked != 0 {
        (*msblk).next_meta_index = ((*msblk).next_meta_index + 1) % SQUASHFS_META_SLOTS;
        i -= 1;
    }
    if i == 0 { TRACE!("empty_meta_index: failed!\n"); mutex_unlock(&mut (*msblk).meta_index_mutex); return meta; }
    meta = (*msblk).meta_index.add((*msblk).next_meta_index as usize);
    (*msblk).next_meta_index = ((*msblk).next_meta_index + 1) % SQUASHFS_META_SLOTS;
    (*meta).inode_number = (*inode).i_ino; (*meta).offset = offset; (*meta).skip = skip;
    (*meta).entries = 0; (*meta).locked = 1;
    mutex_unlock(&mut (*msblk).meta_index_mutex); meta
}

unsafe fn release_meta_index(inode: *mut inode, meta: *mut meta_index) {
    let msblk = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info;
    mutex_lock(&mut (*msblk).meta_index_mutex); (*meta).locked = 0; mutex_unlock(&mut (*msblk).meta_index_mutex);
}

unsafe fn read_indexes(sb: *mut super_block, mut n: i32, start_block: *mut u64, offset: *mut i32) -> i64 {
    let mut block: i64 = 0;
    let blist = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut __le32;
    if blist.is_null() { ERROR!("read_indexes: Failed to allocate block_list\n"); return -ENOMEM as i64; }
    while n != 0 {
        let blocks = core::cmp::min(n, PAGE_SIZE >> 2);
        let mut err = squashfs_read_metadata(sb, blist as *mut core::ffi::c_void, start_block, offset, blocks << 2);
        if err < 0 { ERROR!("read_indexes: reading block [%llx:%x]\n", *start_block, *offset); kfree(blist as *mut core::ffi::c_void); return err as i64; }
        for i in 0..blocks { let size = squashfs_block_size(*blist.add(i as usize)); if size < 0 { kfree(blist as *mut core::ffi::c_void); return size as i64; } block += SQUASHFS_COMPRESSED_SIZE_BLOCK(size) as i64; }
        n -= blocks;
    }
    kfree(blist as *mut core::ffi::c_void); block
}

#[inline] unsafe fn calculate_skip(blocks: u64) -> i32 {
    let skip = blocks / ((SQUASHFS_META_ENTRIES + 1) * SQUASHFS_META_INDEXES) as u64;
    core::cmp::min((SQUASHFS_CACHED_BLKS - 1) as u64, skip + 1) as i32
}

unsafe fn fill_meta_index(inode: *mut inode, mut index: i32, index_block: *mut u64, index_offset: *mut i32, data_block: *mut u64) -> i32 {
    let msblk = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info;
    let skip = calculate_skip(i_size_read(inode) as u64 >> (*msblk).block_log);
    let mut offset = 0; let mut meta: *mut meta_index; let mut meta_entry: *mut meta_entry;
    let mut cur_index_block = squashfs_i(inode).block_list_start; let mut cur_offset = squashfs_i(inode).offset; let mut cur_data_block = squashfs_i(inode).start;
    index /= SQUASHFS_META_INDEXES * skip;
    while offset < index {
        meta = locate_meta_index(inode, offset + 1, index);
        if meta.is_null() { meta = empty_meta_index(inode, offset + 1, skip); if meta.is_null() { break; } }
        else { offset = if index < (*meta).offset + (*meta).entries { index } else { (*meta).offset + (*meta).entries - 1 }; meta_entry = (*meta).meta_entry.as_mut_ptr().add((offset - (*meta).offset) as usize); cur_index_block = (*meta_entry).index_block + (*msblk).inode_table; cur_offset = (*meta_entry).offset; cur_data_block = (*meta_entry).data_block; }
        let begin = (*meta).offset + (*meta).entries;
        let end = core::cmp::min(index + 1, (*meta).offset + SQUASHFS_META_ENTRIES);
        for i in begin..end { let res = read_indexes((*inode).i_sb, skip * SQUASHFS_META_INDEXES, &mut cur_index_block, &mut cur_offset); if res < 0 { if (*meta).entries == 0 { (*meta).inode_number = 0; } release_meta_index(inode, meta); return res as i32; } cur_data_block = cur_data_block.wrapping_add(res as u64); meta_entry = (*meta).meta_entry.as_mut_ptr().add((i - (*meta).offset) as usize); (*meta_entry).index_block = cur_index_block - (*msblk).inode_table; (*meta_entry).offset = cur_offset; (*meta_entry).data_block = cur_data_block; (*meta).entries += 1; offset += 1; }
        release_meta_index(inode, meta);
    }
    *index_block = cur_index_block; *index_offset = cur_offset; if !data_block.is_null() { *data_block = cur_data_block; }
    offset * SQUASHFS_META_INDEXES * skip
}

unsafe fn read_blocklist_ptrs(inode: *mut inode, index: i32, start: *mut u64, offset: *mut i32, block: *mut u64) -> i32 {
    let mut res = fill_meta_index(inode, index, start, offset, block);
    if res < 0 { return res; }
    if res < index { let blks = read_indexes((*inode).i_sb, index - res, start, offset); if blks < 0 { return blks as i32; } if !block.is_null() { *block = (*block).wrapping_add(blks as u64); } }
    let mut size: __le32 = 0; res = squashfs_read_metadata((*inode).i_sb, &mut size as *mut _ as *mut core::ffi::c_void, start, offset, core::mem::size_of::<__le32>() as i32); if res < 0 { return res; } squashfs_block_size(size)
}

#[inline] unsafe fn read_blocklist(inode: *mut inode, index: i32, block: *mut u64) -> i32 { let mut start = 0u64; let mut offset = 0i32; read_blocklist_ptrs(inode, index, &mut start, &mut offset, block) }

unsafe fn squashfs_fill_page(folio: *mut folio, buffer: *mut squashfs_cache_entry, offset: usize, avail: usize) -> bool { let pageaddr = kmap_local_folio(folio, 0); let copied = squashfs_copy_data(pageaddr, buffer, offset, avail); memset(pageaddr.add(copied), 0, PAGE_SIZE - copied); kunmap_local(pageaddr); flush_dcache_folio(folio); copied == avail }

pub unsafe fn squashfs_copy_cache(folio: *mut folio, buffer: *mut squashfs_cache_entry, mut bytes: usize, mut offset: usize) {
    let mapping = (*folio).mapping; let inode = (*mapping).host; let msblk = (*(*inode).i_sb).s_fs_info as *mut squashfs_sb_info; let mask = (1 << ((*msblk).block_log - PAGE_SHIFT)) - 1; let start_index = (*folio).index & !mask; let end_index = start_index | mask;
    let mut i = start_index; while i <= end_index && bytes > 0 { let avail = if !buffer.is_null() { core::cmp::min(bytes, PAGE_SIZE) } else { 0 }; let push_folio = if i == (*folio).index { folio } else { __filemap_get_folio(mapping, i, FGP_LOCK|FGP_CREAT|FGP_NOFS|FGP_NOWAIT, mapping_gfp_mask(mapping)) }; if !IS_ERR(push_folio) { let mut updated = false; if !folio_test_uptodate(push_folio) { updated = squashfs_fill_page(push_folio, buffer, offset, avail); } folio_end_read(push_folio, updated); if i != (*folio).index { folio_put(push_folio); } } i += 1; bytes -= PAGE_SIZE; offset += PAGE_SIZE; }
}

// Remaining file-operation entry points retain the kernel-facing ABI and are
// expressed as external declarations until the surrounding kernel bindings are available.
pub unsafe extern "C" fn squashfs_read_folio(_file: *mut file, _folio: *mut folio) -> i32 { unimplemented!() }
pub unsafe extern "C" fn squashfs_readahead(_ractl: *mut readahead_control) { unimplemented!() }
pub unsafe extern "C" fn squashfs_llseek(_file: *mut file, _offset: loff_t, _whence: i32) -> loff_t { unimplemented!() }

#[repr(C)]
pub struct address_space_operations { pub read_folio: Option<unsafe extern "C" fn(*mut file, *mut folio) -> i32>, pub readahead: Option<unsafe extern "C" fn(*mut readahead_control)> }
pub static squashfs_aops: address_space_operations = address_space_operations { read_folio: Some(squashfs_read_folio), readahead: Some(squashfs_readahead) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
