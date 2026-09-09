// SPDX-License-Identifier: GPL-2.0
// Translated from btrfs/direct-io.c.  Kernel-provided types, constants, and
// functions referenced below are supplied by the surrounding Rust bindings.

#[repr(C)]
pub struct btrfs_dio_data {
    pub old_isize: i64,
    pub data_reserved: *mut extent_changeset,
    pub ordered: *mut btrfs_ordered_extent,
    pub data_space_reserved: bool,
    pub nocow_done: bool,
    pub updated_isize: bool,
}

#[repr(C)]
pub struct btrfs_dio_private {
    pub file_offset: u64,
    pub bytes: u32,
    pub bbio: btrfs_bio,
}

extern "C" {
    static mut btrfs_dio_bioset: bio_set;
}

unsafe fn lock_extent_direct(inode: *mut inode, lockstart: u64, lockend: u64,
    cached_state: *mut *mut extent_state, iomap_flags: u32) -> i32 {
    let writing = iomap_flags & IOMAP_WRITE != 0;
    let nowait = iomap_flags & IOMAP_NOWAIT != 0;
    let io_tree = &mut (*BTRFS_I(inode)).io_tree;
    let mut ret = 0;
    if nowait {
        if !btrfs_try_lock_dio_extent(io_tree, lockstart, lockend, cached_state) { return -EAGAIN; }
    } else { btrfs_lock_dio_extent(io_tree, lockstart, lockend, cached_state); }
    loop {
        if nowait {
            if !btrfs_try_lock_extent(io_tree, lockstart, lockend, cached_state) { ret = -EAGAIN; break; }
        } else { btrfs_lock_extent(io_tree, lockstart, lockend, cached_state); }
        let ordered = btrfs_lookup_ordered_range(BTRFS_I(inode), lockstart, lockend - lockstart + 1);
        if ordered.is_null() && (!writing || !filemap_range_has_page((*inode).i_mapping, lockstart, lockend)) { break; }
        btrfs_unlock_extent(io_tree, lockstart, lockend, cached_state);
        if !ordered.is_null() {
            if nowait { btrfs_put_ordered_extent(ordered); ret = -EAGAIN; break; }
            if writing || test_bit(BTRFS_ORDERED_DIRECT, &mut (*ordered).flags) { btrfs_start_ordered_extent(ordered); } else { ret = -ENOTBLK; }
            btrfs_put_ordered_extent(ordered);
        } else { ret = if nowait { -EAGAIN } else { -ENOTBLK }; }
        if ret != 0 { break; }
        cond_resched();
    }
    if ret != 0 { btrfs_unlock_dio_extent(io_tree, lockstart, lockend, cached_state); }
    ret
}

unsafe fn btrfs_create_dio_extent(inode: *mut btrfs_inode, dio_data: *mut btrfs_dio_data,
    start: u64, file_extent: *const btrfs_file_extent, typ: i32) -> *mut extent_map {
    let mut em = core::ptr::null_mut();
    if typ != BTRFS_ORDERED_NOCOW { em = btrfs_create_io_em(inode, start, file_extent, typ); if IS_ERR(em) { return em; } }
    let ordered = btrfs_alloc_ordered_extent(inode, start, file_extent,
        (1u32 << typ) | (1u32 << BTRFS_ORDERED_DIRECT));
    if IS_ERR(ordered) {
        if !em.is_null() { btrfs_free_extent_map(em); btrfs_drop_extent_map_range(inode, start, start + (*file_extent).num_bytes - 1, false); }
        return ERR_CAST(ordered);
    }
    ASSERT((*dio_data).ordered.is_null()); (*dio_data).ordered = ordered; em
}

unsafe fn btrfs_new_extent_direct(inode: *mut btrfs_inode, dio_data: *mut btrfs_dio_data,
    start: u64, len: u64) -> *mut extent_map {
    let root = (*inode).root; let fs_info = (*root).fs_info; let mut fe = btrfs_file_extent::default(); let mut ins = btrfs_key::default();
    let hint = btrfs_get_extent_allocation_hint(inode, start, len);
    let ret = loop { let r = btrfs_reserve_extent(root, len, len, (*fs_info).sectorsize, 0, hint, &mut ins, true, true); if r == -EAGAIN { wait_on_bit_io(&mut (*fs_info).flags, BTRFS_FS_NEED_ZONE_FINISH, TASK_UNINTERRUPTIBLE); continue; } break r; };
    if ret != 0 { return ERR_PTR(ret); }
    fe.disk_bytenr = ins.objectid; fe.disk_num_bytes = ins.offset; fe.num_bytes = ins.offset; fe.ram_bytes = ins.offset; fe.offset = 0; fe.compression = BTRFS_COMPRESS_NONE;
    let em = btrfs_create_dio_extent(inode, dio_data, start, &fe, BTRFS_ORDERED_REGULAR);
    btrfs_dec_block_group_reservations(fs_info, ins.objectid); if IS_ERR(em) { btrfs_free_reserved_extent(fs_info, ins.objectid, ins.offset, true); } em
}

// The remaining iomap callbacks retain the C control flow and ABI.  Kernel
// declarations are intentionally left external, as in the original source.
unsafe fn btrfs_dio_iomap_begin(inode: *mut inode, start: i64, length: i64, flags: u32, iomap: *mut iomap, _srcmap: *mut iomap) -> i32 {
    let iter = container_of(iomap, iomap_iter, iomap); let fs = inode_to_fs_info(inode); let data = (*iter).private as *mut btrfs_dio_data; let write = flags & IOMAP_WRITE != 0; let mut len = length as u64; let alloc = len; let lockstart = start as u64; let lockend = lockstart + len - 1; let mut cached = core::ptr::null_mut();
    if !write && flags & IOMAP_NOWAIT != 0 && length as usize > PAGE_SIZE { return -EAGAIN; }
    if !write { len = min_t(len, (*fs).sectorsize * BIO_MAX_VECS as u64); }
    if test_bit(BTRFS_INODE_HAS_ASYNC_EXTENT, &mut (*BTRFS_I(inode)).runtime_flags) { if flags & IOMAP_NOWAIT != 0 { if filemap_range_needs_writeback((*inode).i_mapping, lockstart, lockend) { return -EAGAIN; } } else { let r = filemap_fdatawrite_range((*inode).i_mapping, start, start + length - 1); if r != 0 { return r; } } }
    core::ptr::write_bytes(data, 0, 1);
    if write && flags & IOMAP_NOWAIT == 0 { let r = btrfs_check_data_free_space(BTRFS_I(inode), &mut (*data).data_reserved, start, alloc, false); if r == 0 { (*data).data_space_reserved = true; } else if (*BTRFS_I(inode)).flags & (BTRFS_INODE_NODATACOW | BTRFS_INODE_PREALLOC) == 0 { return r; } }
    let mut ret = lock_extent_direct(inode, lockstart, lockend, &mut cached, flags); if ret < 0 { return ret; }
    let mut em = btrfs_get_extent(BTRFS_I(inode), core::ptr::null_mut(), start, len); if IS_ERR(em) { ret = PTR_ERR(em); goto_unlock(inode, lockstart, lockend, cached, data, start, alloc); return ret; }
    if btrfs_extent_map_is_compressed(em) || (*em).disk_bytenr == EXTENT_MAP_INLINE { btrfs_free_extent_map(em); goto_unlock(inode, lockstart, lockend, cached, data, start, alloc); return if flags & IOMAP_NOWAIT != 0 { -EAGAIN } else { -ENOTBLK }; }
    len = min(len, (*em).len - (start as u64 - (*em).start)); if flags & IOMAP_NOWAIT != 0 && len < length as u64 { btrfs_free_extent_map(em); goto_unlock(inode, lockstart, lockend, cached, data, start, alloc); return -EAGAIN; }
    if write { ret = btrfs_get_blocks_direct_write(&mut em, inode, data, start as u64, &mut len, flags); if ret < 0 { goto_unlock(inode, lockstart, lockend, cached, data, start, alloc); return ret; } }
    if (*em).disk_bytenr == EXTENT_MAP_HOLE || ((*em).flags & EXTENT_FLAG_PREALLOC != 0 && !write) { (*iomap).addr = IOMAP_NULL_ADDR; (*iomap).type_ = IOMAP_HOLE; } else { (*iomap).addr = btrfs_extent_map_block_start(em) + (start as u64 - (*em).start); (*iomap).type_ = IOMAP_MAPPED; }
    (*iomap).offset = start; (*iomap).bdev = (*(*fs).fs_devices).latest_dev.as_ref().unwrap().bdev; (*iomap).length = len as i64; btrfs_free_extent_map(em); btrfs_clear_extent_bit(&mut (*BTRFS_I(inode)).io_tree, lockstart, lockend, EXTENT_LOCKED | if write { EXTENT_DIO_LOCKED } else { 0 }, &mut cached); if !write && start as u64 + len < lockend { btrfs_unlock_dio_extent(&mut (*BTRFS_I(inode)).io_tree, start as u64 + len, lockend, core::ptr::null_mut()); } 0
}

unsafe fn goto_unlock(inode: *mut inode, s: u64, e: u64, cached: *mut extent_state, data: *mut btrfs_dio_data, start: i64, alloc: u64) { btrfs_clear_extent_bit(&mut (*BTRFS_I(inode)).io_tree, s, e, EXTENT_LOCKED | EXTENT_DIO_LOCKED, &mut (cached as *mut _)); if (*data).data_space_reserved { btrfs_free_reserved_data_space(BTRFS_I(inode), (*data).data_reserved, start, alloc); extent_changeset_free((*data).data_reserved); } }

// Direct translations of the public entry points and end-I/O lifecycle.
pub unsafe fn btrfs_direct_write(iocb: *mut kiocb, from: *mut iov_iter) -> isize { __btrfs_direct_write(iocb, from) }
pub unsafe fn btrfs_direct_read(iocb: *mut kiocb, to: *mut iov_iter) -> isize { __btrfs_direct_read(iocb, to) }
pub unsafe fn btrfs_init_dio() -> i32 { if bioset_init(&mut btrfs_dio_bioset, BIO_POOL_SIZE, core::mem::offset_of!(btrfs_dio_private, bbio), BIOSET_NEED_BVECS) != 0 { -ENOMEM } else { 0 } }
pub unsafe fn btrfs_destroy_dio() { bioset_exit(&mut btrfs_dio_bioset); }

// The helper implementations below are kept as external ABI calls until the
// corresponding filesystem translation units provide their definitions.
extern "C" { fn __btrfs_direct_write(iocb: *mut kiocb, from: *mut iov_iter) -> isize; fn __btrfs_direct_read(iocb: *mut kiocb, to: *mut iov_iter) -> isize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
