// SPDX-License-Identifier: GPL-2.0

// External kernel types, constants, globals, and functions are supplied by
// the surrounding translation unit.

#[inline]
unsafe fn data_sinfo_for_inode(inode: *const btrfs_inode) -> *mut btrfs_space_info {
    let fs_info = (*(*inode).root).fs_info;

    if btrfs_is_zoned(fs_info) && btrfs_is_data_reloc_root((*inode).root) {
        ASSERT((*(*fs_info).data_sinfo).sub_group[0].subgroup_id == BTRFS_SUB_GROUP_DATA_RELOC);
        return (*fs_info).data_sinfo.sub_group[0];
    }
    (*fs_info).data_sinfo
}

pub unsafe fn btrfs_alloc_data_chunk_ondemand(inode: *const btrfs_inode, mut bytes: u64) -> i32 {
    let root = (*inode).root;
    let fs_info = (*root).fs_info;
    let mut flush = BTRFS_RESERVE_FLUSH_DATA;

    // Make sure bytes are sectorsize aligned
    bytes = ALIGN(bytes, (*fs_info).sectorsize);

    if btrfs_is_free_space_inode(inode) {
        flush = BTRFS_RESERVE_FLUSH_FREE_SPACE_INODE;
    } else if btrfs_is_zoned(fs_info) && btrfs_is_data_reloc_root(root) {
        flush = BTRFS_RESERVE_FLUSH_ZONED_RELOCATION;
    }

    btrfs_reserve_data_bytes(data_sinfo_for_inode(inode), bytes, flush)
}

pub unsafe fn btrfs_check_data_free_space(
    inode: *mut btrfs_inode,
    reserved: *mut *mut extent_changeset,
    mut start: u64,
    mut len: u64,
    noflush: bool,
) -> i32 {
    let fs_info = (*(*inode).root).fs_info;
    let mut flush = BTRFS_RESERVE_FLUSH_DATA;
    let mut ret: i32;

    // align the range
    len = round_up(start + len, (*fs_info).sectorsize) - round_down(start, (*fs_info).sectorsize);
    start = round_down(start, (*fs_info).sectorsize);

    if noflush {
        flush = BTRFS_RESERVE_NO_FLUSH;
    } else if btrfs_is_free_space_inode(inode) {
        flush = BTRFS_RESERVE_FLUSH_FREE_SPACE_INODE;
    }

    ret = btrfs_reserve_data_bytes(data_sinfo_for_inode(inode), len, flush);
    if ret < 0 {
        return ret;
    }

    // Use new btrfs_qgroup_reserve_data to reserve precious data space.
    ret = btrfs_qgroup_reserve_data(inode, reserved, start, len);
    if ret < 0 {
        btrfs_free_reserved_data_space_noquota(inode, len);
        extent_changeset_free(*reserved);
        *reserved = core::ptr::null_mut();
    } else {
        ret = 0;
    }
    ret
}

// Called if we need to clear a data reservation for this inode.
// Normally in an error case.
//
// This one will *NOT* use accurate qgroup reserved space API, just for case
// which we can't sleep and is sure it won't affect qgroup reserved space.
// Like clear_bit_hook().
pub unsafe fn btrfs_free_reserved_data_space_noquota(inode: *mut btrfs_inode, len: u64) {
    let fs_info = (*(*inode).root).fs_info;
    ASSERT(IS_ALIGNED(len, (*fs_info).sectorsize));
    btrfs_space_info_free_bytes_may_use(data_sinfo_for_inode(inode), len);
}

// Called if we need to clear a data reservation for this inode.
// Normally in an error case.
//
// This one will handle the per-inode data rsv map for accurate reserved space framework.
pub unsafe fn btrfs_free_reserved_data_space(
    inode: *mut btrfs_inode,
    reserved: *mut extent_changeset,
    mut start: u64,
    mut len: u64,
) {
    let fs_info = (*(*inode).root).fs_info;
    // Make sure the range is aligned to sectorsize
    len = round_up(start + len, (*fs_info).sectorsize) - round_down(start, (*fs_info).sectorsize);
    start = round_down(start, (*fs_info).sectorsize);
    btrfs_free_reserved_data_space_noquota(inode, len);
    btrfs_qgroup_free_data(inode, reserved, start, len, core::ptr::null_mut());
}

// Release any excessive reservations for an inode.
//
// @inode:       the inode we need to release from
// @qgroup_free: free or convert qgroup meta. Unlike normal operation, qgroup
//               meta reservation needs to know if we are freeing qgroup
//               reservation or just converting it into per-trans. Normally
//               @qgroup_free is true for error handling, and false for normal
//               release.
//
// This is the same as btrfs_block_rsv_release, except that it handles the
// tracepoint for the reservation.
unsafe fn btrfs_inode_rsv_release(inode: *mut btrfs_inode, qgroup_free: bool) {
    let fs_info = (*(*inode).root).fs_info;
    let block_rsv = &mut (*inode).block_rsv;
    let mut qgroup_to_release: u64 = 0;
    let released = btrfs_block_rsv_release(fs_info, block_rsv, 0, &mut qgroup_to_release);
    if released > 0 {
        trace_btrfs_space_reservation(fs_info, "delalloc", btrfs_ino(inode), released, 0);
    }
    if qgroup_free {
        btrfs_qgroup_free_meta_prealloc((*inode).root, qgroup_to_release);
    } else {
        btrfs_qgroup_convert_reserved_meta((*inode).root, qgroup_to_release);
    }
}

unsafe fn btrfs_calculate_inode_block_rsv_size(fs_info: *mut btrfs_fs_info, inode: *mut btrfs_inode) {
    let block_rsv = &mut (*inode).block_rsv;
    let mut reserve_size: u64 = 0;
    let mut qgroup_rsv_size: u64 = 0;
    let outstanding_extents = (*inode).outstanding_extents;

    lockdep_assert_held(&(*inode).lock);
    if outstanding_extents != 0 {
        reserve_size = btrfs_calc_insert_metadata_size(fs_info, outstanding_extents);
        reserve_size += btrfs_calc_metadata_size(fs_info, 1);
    }
    if ((*inode).flags & BTRFS_INODE_NODATASUM) == 0 {
        let csum_leaves = btrfs_csum_bytes_to_leaves(fs_info, (*inode).csum_bytes);
        reserve_size += btrfs_calc_insert_metadata_size(fs_info, csum_leaves);
    }
    // For qgroup rsv, the calculation is very simple: account one nodesize for each outstanding extent.
    qgroup_rsv_size = (outstanding_extents as u64) << (*fs_info).nodesize_bits;
    spin_lock(&mut block_rsv.lock);
    block_rsv.size = reserve_size;
    block_rsv.qgroup_rsv_size = qgroup_rsv_size;
    spin_unlock(&mut block_rsv.lock);
}

unsafe fn calc_inode_reservations(inode: *mut btrfs_inode, num_bytes: u64, disk_num_bytes: u64, meta_reserve: *mut u64, qgroup_reserve: *mut u64) {
    let fs_info = (*(*inode).root).fs_info;
    let nr_extents = count_max_extents(fs_info, num_bytes);
    let csum_leaves = if ((*inode).flags & BTRFS_INODE_NODATASUM) != 0 { 0 } else { btrfs_csum_bytes_to_leaves(fs_info, disk_num_bytes) };
    *meta_reserve = btrfs_calc_insert_metadata_size(fs_info, nr_extents + csum_leaves);
    *meta_reserve += btrfs_calc_metadata_size(fs_info, 1);
    *qgroup_reserve = nr_extents << (*fs_info).nodesize_bits;
}

pub unsafe fn btrfs_delalloc_reserve_metadata(inode: *mut btrfs_inode, mut num_bytes: u64, mut disk_num_bytes: u64, noflush: bool) -> i32 {
    let root = (*inode).root;
    let fs_info = (*root).fs_info;
    let block_rsv = &mut (*inode).block_rsv;
    let (mut meta_reserve, mut qgroup_reserve) = (0u64, 0u64);
    let mut flush = BTRFS_RESERVE_FLUSH_ALL;
    if noflush || btrfs_is_free_space_inode(inode) { flush = BTRFS_RESERVE_NO_FLUSH; }
    else if (*current).journal_info != core::ptr::null_mut() { flush = BTRFS_RESERVE_FLUSH_LIMIT; }
    num_bytes = ALIGN(num_bytes, (*fs_info).sectorsize);
    disk_num_bytes = ALIGN(disk_num_bytes, (*fs_info).sectorsize);
    calc_inode_reservations(inode, num_bytes, disk_num_bytes, &mut meta_reserve, &mut qgroup_reserve);
    let mut ret = btrfs_qgroup_reserve_meta_prealloc(root, qgroup_reserve, true, noflush);
    if ret != 0 { return ret; }
    ret = btrfs_reserve_metadata_bytes(block_rsv.space_info, meta_reserve, flush);
    if ret != 0 { btrfs_qgroup_free_meta_prealloc(root, qgroup_reserve); return ret; }
    let nr_extents = count_max_extents(fs_info, num_bytes);
    spin_lock(&mut (*inode).lock);
    btrfs_mod_outstanding_extents(inode, nr_extents);
    if ((*inode).flags & BTRFS_INODE_NODATASUM) == 0 { (*inode).csum_bytes += disk_num_bytes; }
    btrfs_calculate_inode_block_rsv_size(fs_info, inode);
    spin_unlock(&mut (*inode).lock);
    btrfs_block_rsv_add_bytes(block_rsv, meta_reserve, false);
    trace_btrfs_space_reservation((*root).fs_info, "delalloc", btrfs_ino(inode), meta_reserve, 1);
    spin_lock(&mut block_rsv.lock);
    block_rsv.qgroup_rsv_reserved += qgroup_reserve;
    spin_unlock(&mut block_rsv.lock);
    0
}

pub unsafe fn btrfs_delalloc_release_metadata(inode: *mut btrfs_inode, mut num_bytes: u64, qgroup_free: bool) {
    let fs_info = (*(*inode).root).fs_info;
    num_bytes = ALIGN(num_bytes, (*fs_info).sectorsize);
    spin_lock(&mut (*inode).lock);
    if ((*inode).flags & BTRFS_INODE_NODATASUM) == 0 { (*inode).csum_bytes -= num_bytes; }
    btrfs_calculate_inode_block_rsv_size(fs_info, inode);
    spin_unlock(&mut (*inode).lock);
    if btrfs_is_testing(fs_info) { return; }
    btrfs_inode_rsv_release(inode, qgroup_free);
}

pub unsafe fn btrfs_delalloc_release_extents(inode: *mut btrfs_inode, num_bytes: u64) {
    let fs_info = (*(*inode).root).fs_info;
    spin_lock(&mut (*inode).lock);
    let num_extents = count_max_extents(fs_info, num_bytes);
    btrfs_mod_outstanding_extents(inode, -(num_extents as i32));
    btrfs_calculate_inode_block_rsv_size(fs_info, inode);
    spin_unlock(&mut (*inode).lock);
    if btrfs_is_testing(fs_info) { return; }
    btrfs_inode_rsv_release(inode, true);
}

pub unsafe fn btrfs_delalloc_shrink_extents(inode: *mut btrfs_inode, reserved_len: u64, new_len: u64) {
    let fs_info = (*(*inode).root).fs_info;
    let reserved_num_extents = count_max_extents(fs_info, reserved_len);
    let new_num_extents = count_max_extents(fs_info, new_len);
    let diff_num_extents = new_num_extents as i32 - reserved_num_extents as i32;
    ASSERT(new_len <= reserved_len);
    if new_num_extents == reserved_num_extents { return; }
    spin_lock(&mut (*inode).lock);
    btrfs_mod_outstanding_extents(inode, diff_num_extents);
    btrfs_calculate_inode_block_rsv_size(fs_info, inode);
    spin_unlock(&mut (*inode).lock);
    if btrfs_is_testing(fs_info) { return; }
    btrfs_inode_rsv_release(inode, true);
}

pub unsafe fn btrfs_delalloc_reserve_space(inode: *mut btrfs_inode, reserved: *mut *mut extent_changeset, start: u64, len: u64) -> i32 {
    let mut ret = btrfs_check_data_free_space(inode, reserved, start, len, false);
    if ret < 0 { return ret; }
    ret = btrfs_delalloc_reserve_metadata(inode, len, len, false);
    if ret < 0 {
        btrfs_free_reserved_data_space(inode, *reserved, start, len);
        extent_changeset_free(*reserved);
        *reserved = core::ptr::null_mut();
    }
    ret
}

pub unsafe fn btrfs_delalloc_release_space(inode: *mut btrfs_inode, reserved: *mut extent_changeset, start: u64, len: u64, qgroup_free: bool) {
    btrfs_delalloc_release_metadata(inode, len, qgroup_free);
    btrfs_free_reserved_data_space(inode, reserved, start, len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
