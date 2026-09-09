// SPDX-License-Identifier: GPL-2.0

// Dependencies are supplied by the surrounding translation unit.

unsafe fn block_rsv_release_bytes(
    fs_info: *mut btrfs_fs_info, block_rsv: *mut btrfs_block_rsv,
    dest: *mut btrfs_block_rsv, mut num_bytes: u64,
    qgroup_to_release_ret: *mut u64,
) -> u64 {
    let space_info = (*block_rsv).space_info;
    let mut qgroup_to_release = 0u64;
    let ret;
    spin_lock(&mut (*block_rsv).lock);
    if num_bytes == u64::MAX { num_bytes = (*block_rsv).size; qgroup_to_release = (*block_rsv).qgroup_rsv_size; }
    (*block_rsv).size -= num_bytes;
    if (*block_rsv).reserved >= (*block_rsv).size {
        num_bytes = (*block_rsv).reserved - (*block_rsv).size;
        (*block_rsv).reserved = (*block_rsv).size; (*block_rsv).full = true;
    } else { num_bytes = 0; }
    if !qgroup_to_release_ret.is_null() && (*block_rsv).qgroup_rsv_reserved >= (*block_rsv).qgroup_rsv_size {
        qgroup_to_release = (*block_rsv).qgroup_rsv_reserved - (*block_rsv).qgroup_rsv_size;
        (*block_rsv).qgroup_rsv_reserved = (*block_rsv).qgroup_rsv_size;
    } else { qgroup_to_release = 0; }
    spin_unlock(&mut (*block_rsv).lock);
    ret = num_bytes;
    if num_bytes > 0 {
        if !dest.is_null() {
            spin_lock(&mut (*dest).lock);
            if !(*dest).full {
                let mut bytes_to_add = (*dest).size - (*dest).reserved;
                bytes_to_add = min(num_bytes, bytes_to_add);
                (*dest).reserved += bytes_to_add;
                if (*dest).reserved >= (*dest).size { (*dest).full = true; }
                num_bytes -= bytes_to_add;
            }
            spin_unlock(&mut (*dest).lock);
        }
        if num_bytes != 0 { btrfs_space_info_free_bytes_may_use(space_info, num_bytes); }
    }
    if !qgroup_to_release_ret.is_null() { *qgroup_to_release_ret = qgroup_to_release; }
    ret
}

pub unsafe fn btrfs_block_rsv_migrate(src: *mut btrfs_block_rsv, dst: *mut btrfs_block_rsv, num_bytes: u64, update_size: bool) -> i32 {
    let ret = btrfs_block_rsv_use_bytes(src, num_bytes); if ret != 0 { return ret; }
    btrfs_block_rsv_add_bytes(dst, num_bytes, update_size); 0
}

pub unsafe fn btrfs_init_block_rsv(rsv: *mut btrfs_block_rsv, type_: btrfs_rsv_type) {
    memset(rsv as *mut _, 0, core::mem::size_of::<btrfs_block_rsv>());
    spin_lock_init(&mut (*rsv).lock); (*rsv).type_ = type_;
}

pub unsafe fn btrfs_init_metadata_block_rsv(fs_info: *mut btrfs_fs_info, rsv: *mut btrfs_block_rsv, type_: btrfs_rsv_type) {
    btrfs_init_block_rsv(rsv, type_); (*rsv).space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_METADATA);
}

pub unsafe fn btrfs_alloc_block_rsv(fs_info: *mut btrfs_fs_info, type_: btrfs_rsv_type) -> *mut btrfs_block_rsv {
    let block_rsv = kmalloc_obj::<btrfs_block_rsv>(GFP_NOFS); if block_rsv.is_null() { return core::ptr::null_mut(); }
    btrfs_init_metadata_block_rsv(fs_info, block_rsv, type_); block_rsv
}

pub unsafe fn btrfs_free_block_rsv(fs_info: *mut btrfs_fs_info, rsv: *mut btrfs_block_rsv) {
    if rsv.is_null() { return; } btrfs_block_rsv_release(fs_info, rsv, u64::MAX, core::ptr::null_mut()); kfree(rsv);
}

pub unsafe fn btrfs_block_rsv_add(fs_info: *mut btrfs_fs_info, block_rsv: *mut btrfs_block_rsv, num_bytes: u64, flush: btrfs_reserve_flush_enum) -> i32 {
    if num_bytes == 0 { return 0; }
    let ret = btrfs_reserve_metadata_bytes((*block_rsv).space_info, num_bytes, flush);
    if ret == 0 { btrfs_block_rsv_add_bytes(block_rsv, num_bytes, true); } ret
}

pub unsafe fn btrfs_block_rsv_check(block_rsv: *mut btrfs_block_rsv, min_percent: i32) -> i32 {
    let mut ret = -ENOSPC; spin_lock(&mut (*block_rsv).lock);
    let num_bytes = mult_perc((*block_rsv).size, min_percent);
    if (*block_rsv).reserved >= num_bytes { ret = 0; } spin_unlock(&mut (*block_rsv).lock); ret
}

pub unsafe fn btrfs_block_rsv_refill(fs_info: *mut btrfs_fs_info, block_rsv: *mut btrfs_block_rsv, mut num_bytes: u64, flush: btrfs_reserve_flush_enum) -> i32 {
    let mut ret = -ENOSPC; if block_rsv.is_null() { return 0; }
    spin_lock(&mut (*block_rsv).lock); if (*block_rsv).reserved >= num_bytes { ret = 0; } else { num_bytes -= (*block_rsv).reserved; } spin_unlock(&mut (*block_rsv).lock);
    if ret == 0 { return 0; } ret = btrfs_reserve_metadata_bytes((*block_rsv).space_info, num_bytes, flush);
    if ret == 0 { btrfs_block_rsv_add_bytes(block_rsv, num_bytes, false); 0 } else { ret }
}

pub unsafe fn btrfs_block_rsv_release(fs_info: *mut btrfs_fs_info, block_rsv: *mut btrfs_block_rsv, num_bytes: u64, qgroup: *mut u64) -> u64 {
    let global_rsv = &mut (*fs_info).global_block_rsv as *mut _; let delayed_rsv = &mut (*fs_info).delayed_refs_rsv as *mut _; let mut target = core::ptr::null_mut();
    if (*block_rsv).type_ == BTRFS_BLOCK_RSV_DELREFS { target = global_rsv; } else if block_rsv != global_rsv && !btrfs_block_rsv_full(delayed_rsv) { target = delayed_rsv; }
    if !target.is_null() && (*block_rsv).space_info != (*target).space_info { target = core::ptr::null_mut(); }
    block_rsv_release_bytes(fs_info, block_rsv, target, num_bytes, qgroup)
}

pub unsafe fn btrfs_block_rsv_use_bytes(block_rsv: *mut btrfs_block_rsv, num_bytes: u64) -> i32 {
    let mut ret = -ENOSPC; spin_lock(&mut (*block_rsv).lock);
    if (*block_rsv).reserved >= num_bytes { (*block_rsv).reserved -= num_bytes; if (*block_rsv).reserved < (*block_rsv).size { (*block_rsv).full = false; } ret = 0; }
    spin_unlock(&mut (*block_rsv).lock); ret
}

pub unsafe fn btrfs_block_rsv_add_bytes(block_rsv: *mut btrfs_block_rsv, num_bytes: u64, update_size: bool) {
    spin_lock(&mut (*block_rsv).lock); (*block_rsv).reserved += num_bytes;
    if update_size { (*block_rsv).size += num_bytes; } else if (*block_rsv).reserved >= (*block_rsv).size { (*block_rsv).full = true; }
    spin_unlock(&mut (*block_rsv).lock);
}

pub unsafe fn btrfs_update_global_block_rsv(fs_info: *mut btrfs_fs_info) {
    let block_rsv = &mut (*fs_info).global_block_rsv as *mut _;
    if btrfs_is_full_ro(fs_info) { spin_lock(&mut (*block_rsv).lock); (*block_rsv).full = true; spin_unlock(&mut (*block_rsv).lock); return; }
    let sinfo = (*block_rsv).space_info; let mut num_bytes = btrfs_root_used(&(*(*fs_info).tree_root).root_item); let mut min_items = 1u32;
    read_lock(&mut (*fs_info).global_root_lock);
    rbtree_postorder_for_each_entry_safe(|root: *mut btrfs_root| { let id = btrfs_root_id(root); if id == BTRFS_EXTENT_TREE_OBJECTID || id == BTRFS_CSUM_TREE_OBJECTID || id == BTRFS_FREE_SPACE_TREE_OBJECTID { num_bytes += btrfs_root_used(&(*root).root_item); min_items += 1; } }, &(*fs_info).global_root_tree);
    read_unlock(&mut (*fs_info).global_root_lock);
    if btrfs_fs_compat_ro(fs_info, BLOCK_GROUP_TREE) { num_bytes += btrfs_root_used(&(*(*fs_info).block_group_root).root_item); min_items += 1; }
    if btrfs_fs_incompat(fs_info, RAID_STRIPE_TREE) { num_bytes += btrfs_root_used(&(*(*fs_info).stripe_root).root_item); min_items += 1; }
    min_items += BTRFS_UNLINK_METADATA_UNITS;
    num_bytes = max(num_bytes, btrfs_calc_insert_metadata_size(fs_info, min_items) + btrfs_calc_delayed_ref_bytes(fs_info, BTRFS_UNLINK_METADATA_UNITS));
    spin_lock(&mut (*sinfo).lock); spin_lock(&mut (*block_rsv).lock);
    (*block_rsv).size = min(num_bytes, SZ_512M);
    if (*block_rsv).reserved < (*block_rsv).size { num_bytes = (*block_rsv).size - (*block_rsv).reserved; btrfs_space_info_update_bytes_may_use(sinfo, num_bytes); (*block_rsv).reserved = (*block_rsv).size; }
    else if (*block_rsv).reserved > (*block_rsv).size { num_bytes = (*block_rsv).reserved - (*block_rsv).size; btrfs_space_info_update_bytes_may_use(sinfo, -(num_bytes as i64)); (*block_rsv).reserved = (*block_rsv).size; btrfs_try_granting_tickets(sinfo); }
    (*block_rsv).full = (*block_rsv).reserved == (*block_rsv).size; if (*block_rsv).size >= (*sinfo).total_bytes { (*sinfo).force_alloc = CHUNK_ALLOC_FORCE; }
    spin_unlock(&mut (*block_rsv).lock); spin_unlock(&mut (*sinfo).lock);
}

pub unsafe fn btrfs_init_root_block_rsv(root: *mut btrfs_root) {
    let fs_info = (*root).fs_info; (*root).block_rsv = match btrfs_root_id(root) {
        BTRFS_CSUM_TREE_OBJECTID | BTRFS_EXTENT_TREE_OBJECTID | BTRFS_FREE_SPACE_TREE_OBJECTID | BTRFS_BLOCK_GROUP_TREE_OBJECTID | BTRFS_RAID_STRIPE_TREE_OBJECTID => &mut (*fs_info).delayed_refs_rsv,
        BTRFS_ROOT_TREE_OBJECTID | BTRFS_DEV_TREE_OBJECTID | BTRFS_QUOTA_TREE_OBJECTID => &mut (*fs_info).global_block_rsv,
        BTRFS_CHUNK_TREE_OBJECTID => &mut (*fs_info).chunk_block_rsv, BTRFS_TREE_LOG_OBJECTID => &mut (*fs_info).treelog_rsv,
        BTRFS_REMAP_TREE_OBJECTID => &mut (*fs_info).remap_block_rsv, _ => core::ptr::null_mut(),
    };
}

pub unsafe fn btrfs_init_global_block_rsv(fs_info: *mut btrfs_fs_info) {
    (*fs_info).chunk_block_rsv.space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_SYSTEM);
    (*fs_info).remap_block_rsv.space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_METADATA_REMAP);
    let space_info = btrfs_find_space_info(fs_info, BTRFS_BLOCK_GROUP_METADATA);
    (*fs_info).global_block_rsv.space_info = space_info; (*fs_info).trans_block_rsv.space_info = space_info; (*fs_info).empty_block_rsv.space_info = space_info; (*fs_info).delayed_block_rsv.space_info = space_info; (*fs_info).delayed_refs_rsv.space_info = space_info;
    if !btrfs_is_zoned(fs_info) { (*fs_info).treelog_rsv.space_info = space_info; } else { ASSERT((*space_info).sub_group[0].subgroup_id == BTRFS_SUB_GROUP_TREELOG); (*fs_info).treelog_rsv.space_info = (*space_info).sub_group[0]; }
    btrfs_update_global_block_rsv(fs_info);
}

pub unsafe fn btrfs_release_global_block_rsv(fs_info: *mut btrfs_fs_info) {
    btrfs_block_rsv_release(fs_info, &mut (*fs_info).global_block_rsv, u64::MAX, core::ptr::null_mut());
    WARN_ON((*fs_info).trans_block_rsv.size > 0); WARN_ON((*fs_info).trans_block_rsv.reserved > 0); WARN_ON((*fs_info).chunk_block_rsv.size > 0); WARN_ON((*fs_info).chunk_block_rsv.reserved > 0); WARN_ON((*fs_info).remap_block_rsv.size > 0); WARN_ON((*fs_info).remap_block_rsv.reserved > 0); WARN_ON((*fs_info).delayed_block_rsv.size > 0); WARN_ON((*fs_info).delayed_block_rsv.reserved > 0); WARN_ON((*fs_info).delayed_refs_rsv.reserved > 0); WARN_ON((*fs_info).delayed_refs_rsv.size > 0);
}

unsafe fn get_block_rsv(trans: *const btrfs_trans_handle, root: *const btrfs_root) -> *mut btrfs_block_rsv {
    let fs_info = (*root).fs_info; let mut block_rsv = core::ptr::null_mut();
    if test_bit(BTRFS_ROOT_SHAREABLE, &(*root).state) || root == (*fs_info).uuid_root || ((*trans).adding_csums && btrfs_root_id(root) == BTRFS_CSUM_TREE_OBJECTID) { block_rsv = (*trans).block_rsv; }
    if block_rsv.is_null() { block_rsv = (*root).block_rsv; } if block_rsv.is_null() { block_rsv = &mut (*fs_info).empty_block_rsv; } block_rsv
}

pub unsafe fn btrfs_use_block_rsv(trans: *mut btrfs_trans_handle, root: *mut btrfs_root, blocksize: u32) -> *mut btrfs_block_rsv {
    let fs_info = (*root).fs_info; let block_rsv = get_block_rsv(trans, root); let global_rsv = &mut (*fs_info).global_block_rsv as *mut _; let mut global_updated = false;
    if btrfs_block_rsv_size(block_rsv) == 0 { return try_reserve(fs_info, root, block_rsv, global_rsv, blocksize); }
    loop { let ret = btrfs_block_rsv_use_bytes(block_rsv, blocksize); if ret == 0 { return block_rsv; }
        if (*block_rsv).failfast { return ERR_PTR(ret); }
        if (*block_rsv).type_ == BTRFS_BLOCK_RSV_GLOBAL && !global_updated { global_updated = true; btrfs_update_global_block_rsv(fs_info); continue; }
        break;
    }
    try_reserve(fs_info, root, block_rsv, global_rsv, blocksize)
}

unsafe fn try_reserve(fs_info: *mut btrfs_fs_info, root: *mut btrfs_root, block_rsv: *mut btrfs_block_rsv, global_rsv: *mut btrfs_block_rsv, blocksize: u32) -> *mut btrfs_block_rsv {
    let mut ret = btrfs_reserve_metadata_bytes((*block_rsv).space_info, blocksize as u64, BTRFS_RESERVE_NO_FLUSH); if ret == 0 { return block_rsv; }
    if btrfs_root_id(root) == BTRFS_TREE_LOG_OBJECTID { return ERR_PTR(ret); }
    if (*block_rsv).type_ != BTRFS_BLOCK_RSV_GLOBAL && (*block_rsv).space_info == (*global_rsv).space_info { ret = btrfs_block_rsv_use_bytes(global_rsv, blocksize as u64); if ret == 0 { return global_rsv; } }
    ret = btrfs_reserve_metadata_bytes((*block_rsv).space_info, blocksize as u64, BTRFS_RESERVE_FLUSH_EMERGENCY); if ret == 0 { block_rsv } else { ERR_PTR(ret) }
}

pub unsafe fn btrfs_check_trunc_cache_free_space(fs_info: *const btrfs_fs_info, rsv: *mut btrfs_block_rsv) -> i32 {
    let needed_bytes = btrfs_calc_insert_metadata_size(fs_info, 1) + btrfs_calc_metadata_size(fs_info, 1); spin_lock(&mut (*rsv).lock); let ret = if (*rsv).reserved < needed_bytes { -ENOSPC } else { 0 }; spin_unlock(&mut (*rsv).lock); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
