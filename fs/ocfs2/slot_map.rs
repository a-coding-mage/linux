// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * slot_map.c
 *
 * Copyright (C) 2002, 2004 Oracle.  All rights reserved.
 */

// Linux and OCFS2 headers provide the types, constants, macros, and external
// functions referenced below.

#[repr(C)]
pub struct ocfs2_slot {
    sl_valid: ::core::ffi::c_int,
    sl_node_num: u32,
}

#[repr(C)]
pub struct ocfs2_slot_info {
    si_extended: ::core::ffi::c_int,
    si_slots_per_block: ::core::ffi::c_int,
    si_inode: *mut inode,
    si_blocks: u32,
    si_bh: *mut *mut buffer_head,
    si_num_slots: u32,
    si_slots: [ocfs2_slot; 0],
}

unsafe fn __ocfs2_node_num_to_slot(si: *mut ocfs2_slot_info, node_num: u32) -> ::core::ffi::c_int {
    for i in 0..(*si).si_num_slots { let s = (*si).si_slots.add(i as usize).read(); if s.sl_valid != 0 && s.sl_node_num == node_num { return i as _; } } -ENOENT
}

unsafe fn ocfs2_validate_slot_map_block(sb: *mut super_block, bh: *mut buffer_head) -> ::core::ffi::c_int {
    BUG_ON(!buffer_uptodate(bh));
    if (*bh).b_blocknr < OCFS2_SUPER_BLOCK_BLKNO { return ocfs2_error(sb, "Invalid Slot Map Buffer Head Block Number : %llu, Should be >= %d", (*bh).b_blocknr, OCFS2_SUPER_BLOCK_BLKNO); } 0
}

unsafe fn ocfs2_invalidate_slot(si: *mut ocfs2_slot_info, slot_num: ::core::ffi::c_int) {
    BUG_ON(slot_num < 0 || slot_num as u32 >= (*si).si_num_slots);
    (*si).si_slots.add(slot_num as usize).as_mut().unwrap().sl_valid = 0;
}

unsafe fn ocfs2_set_slot(si: *mut ocfs2_slot_info, slot_num: ::core::ffi::c_int, node_num: u32) {
    BUG_ON(slot_num < 0 || slot_num as u32 >= (*si).si_num_slots);
    let slot = (*si).si_slots.add(slot_num as usize).as_mut().unwrap();
    slot.sl_valid = 1;
    slot.sl_node_num = node_num;
}

/* This version is for the extended slot map */
unsafe fn ocfs2_update_slot_info_extended(si: *mut ocfs2_slot_info) {
    let mut slotno = 0;
    for b in 0..(*si).si_blocks {
        let se = (*si).si_bh.add(b as usize).read().cast::<ocfs2_slot_map_extended>();
        for i in 0..(*si).si_slots_per_block {
            if slotno >= (*si).si_num_slots as i32 { break; }
            let s = (*se).se_slots.add(i as usize).read();
            if s.es_valid != 0 { ocfs2_set_slot(si, slotno, le32_to_cpu(s.es_node_num)); }
            else { ocfs2_invalidate_slot(si, slotno); }
            slotno += 1;
        }
    }
}

/* Post the slot information on disk into our slot_info struct. Must be protected by osb_lock. */
unsafe fn ocfs2_update_slot_info_old(si: *mut ocfs2_slot_info) {
    let sm = (*si).si_bh.read().cast::<ocfs2_slot_map>();
    for i in 0..(*si).si_num_slots {
        let v = le16_to_cpu((*sm).sm_slots.add(i as usize).read());
        if v == OCFS2_INVALID_SLOT as u16 { ocfs2_invalidate_slot(si, i as i32); }
        else { ocfs2_set_slot(si, i as i32, v as u32); }
    }
}

unsafe fn ocfs2_update_slot_info(si: *mut ocfs2_slot_info) {
    if (*si).si_extended != 0 { ocfs2_update_slot_info_extended(si); }
    else { ocfs2_update_slot_info_old(si); }
}

pub unsafe fn ocfs2_refresh_slot_info(osb: *mut ocfs2_super) -> ::core::ffi::c_int {
    let si = (*osb).slot_info;
    if si.is_null() { return 0; }
    BUG_ON((*si).si_blocks == 0); BUG_ON((*si).si_bh.is_null());
    trace_ocfs2_refresh_slot_info((*si).si_blocks);
    let ret = ocfs2_read_blocks(INODE_CACHE((*si).si_inode), -1, (*si).si_blocks, (*si).si_bh,
        OCFS2_BH_IGNORE_CACHE, Some(ocfs2_validate_slot_map_block));
    if ret == 0 { spin_lock(&mut (*osb).osb_lock); ocfs2_update_slot_info(si); spin_unlock(&mut (*osb).osb_lock); }
    ret
}

unsafe fn ocfs2_update_disk_slot_extended(si: *mut ocfs2_slot_info, slot_num: ::core::ffi::c_int, bh: *mut *mut buffer_head) {
    let blkind = slot_num / (*si).si_slots_per_block;
    let slotno = slot_num % (*si).si_slots_per_block;
    BUG_ON(blkind as u32 >= (*si).si_blocks);
    let se = (*si).si_bh.add(blkind as usize).read().cast::<ocfs2_slot_map_extended>();
    let slot = (*si).si_slots.add(slot_num as usize).read();
    (*se).se_slots.add(slotno as usize).as_mut().unwrap().es_valid = slot.sl_valid as _;
    if slot.sl_valid != 0 { (*se).se_slots.add(slotno as usize).as_mut().unwrap().es_node_num = cpu_to_le32(slot.sl_node_num); }
    *bh = (*si).si_bh.add(blkind as usize).read();
}

unsafe fn ocfs2_update_disk_slot_old(si: *mut ocfs2_slot_info, _slot_num: ::core::ffi::c_int, bh: *mut *mut buffer_head) {
    let sm = (*si).si_bh.read().cast::<ocfs2_slot_map>();
    for i in 0..(*si).si_num_slots {
        let slot = (*si).si_slots.add(i as usize).read();
        (*sm).sm_slots.add(i as usize).as_mut().unwrap().write(if slot.sl_valid != 0 { cpu_to_le16(slot.sl_node_num as _) } else { cpu_to_le16(OCFS2_INVALID_SLOT as _) });
    }
    *bh = (*si).si_bh.read();
}

unsafe fn ocfs2_update_disk_slot(osb: *mut ocfs2_super, si: *mut ocfs2_slot_info, slot_num: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut bh = core::ptr::null_mut();
    spin_lock(&mut (*osb).osb_lock);
    if (*si).si_extended != 0 { ocfs2_update_disk_slot_extended(si, slot_num, &mut bh); }
    else { ocfs2_update_disk_slot_old(si, slot_num, &mut bh); }
    spin_unlock(&mut (*osb).osb_lock);
    let status = ocfs2_write_block(osb, bh, INODE_CACHE((*si).si_inode));
    if status < 0 { mlog_errno(status); }
    status
}

unsafe fn ocfs2_slot_map_physical_size(osb: *mut ocfs2_super, inode: *mut inode, bytes: *mut u64) -> ::core::ffi::c_int {
    let bytes_needed = if ocfs2_uses_extended_slot_map(osb) { (*osb).max_slots as u64 * size_of::<ocfs2_extended_slot>() as u64 } else { (*osb).max_slots as u64 * size_of::<u16>() as u64 };
    if bytes_needed > i_size_read(inode) { mlog!(ML_ERROR, "Slot map file is too small!  (size %llu, needed %llu)\n", i_size_read(inode), bytes_needed); return -ENOSPC; }
    *bytes = bytes_needed; 0
}

unsafe fn __ocfs2_find_empty_slot(si: *mut ocfs2_slot_info, preferred: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if preferred >= 0 && preferred as u32 < (*si).si_num_slots && (*si).si_slots.add(preferred as usize).read().sl_valid == 0 { return preferred; }
    for i in 0..(*si).si_num_slots { if (*si).si_slots.add(i as usize).read().sl_valid == 0 { return i as i32; } }
    -ENOSPC
}

pub unsafe fn ocfs2_node_num_to_slot(osb: *mut ocfs2_super, node_num: u32) -> ::core::ffi::c_int { spin_lock(&mut (*osb).osb_lock); let slot = __ocfs2_node_num_to_slot((*osb).slot_info, node_num); spin_unlock(&mut (*osb).osb_lock); slot }

pub unsafe fn ocfs2_slot_to_node_num_locked(osb: *mut ocfs2_super, slot_num: ::core::ffi::c_int, node_num: *mut u32) -> ::core::ffi::c_int {
    let si = (*osb).slot_info; assert_spin_locked(&(*osb).osb_lock); BUG_ON(slot_num < 0); BUG_ON(slot_num as u32 >= (*osb).max_slots);
    if (*si).si_slots.add(slot_num as usize).read().sl_valid == 0 { return -ENOENT; } *node_num = (*si).si_slots.add(slot_num as usize).read().sl_node_num; 0
}

unsafe fn __ocfs2_free_slot_info(si: *mut ocfs2_slot_info) {
    if si.is_null() { return; } iput((*si).si_inode);
    if !(*si).si_bh.is_null() { for i in 0..(*si).si_blocks { let p = (*si).si_bh.add(i as usize); if !p.read().is_null() { brelse(p.read()); p.write(core::ptr::null_mut()); } } kfree((*si).si_bh as *mut _); }
    kfree(si as *mut _);
}

pub unsafe fn ocfs2_clear_slot(osb: *mut ocfs2_super, slot_num: ::core::ffi::c_int) -> ::core::ffi::c_int { let si = (*osb).slot_info; if si.is_null() { return 0; } spin_lock(&mut (*osb).osb_lock); ocfs2_invalidate_slot(si, slot_num); spin_unlock(&mut (*osb).osb_lock); ocfs2_update_disk_slot(osb, si, slot_num) }

unsafe fn ocfs2_map_slot_buffers(osb: *mut ocfs2_super, si: *mut ocfs2_slot_info) -> ::core::ffi::c_int {
    let mut bytes = 0; let status = ocfs2_slot_map_physical_size(osb, (*si).si_inode, &mut bytes); if status != 0 { return status; }
    (*si).si_blocks = ocfs2_blocks_for_bytes((*si).si_inode, bytes) as _; if (*si).si_blocks == 0 { return 0; }
    (*si).si_slots_per_block = if (*si).si_extended != 0 { (*osb).sb.s_blocksize / size_of::<ocfs2_extended_slot>() } else { (*osb).sb.s_blocksize / size_of::<u16>() } as _;
    (*si).si_bh = kzalloc_objs::<*mut buffer_head>((*si).si_blocks as _); if (*si).si_bh.is_null() { return -ENOMEM; }
    for i in 0..(*si).si_blocks { let mut blkno = 0; let mut bh = core::ptr::null_mut(); let r = ocfs2_extent_map_get_blocks((*si).si_inode, i, &mut blkno, core::ptr::null_mut(), core::ptr::null_mut()); if r < 0 { return r; } let r = ocfs2_read_blocks(INODE_CACHE((*si).si_inode), blkno, 1, &mut bh, OCFS2_BH_IGNORE_CACHE, Some(ocfs2_validate_slot_map_block)); if r < 0 { return r; } (*si).si_bh.add(i as usize).write(bh); } 0
}

pub unsafe fn ocfs2_init_slot_info(osb: *mut ocfs2_super) -> ::core::ffi::c_int {
    let si = kzalloc_flex::<ocfs2_slot_info>(size_of::<ocfs2_slot>(), (*osb).max_slots);
    if si.is_null() { mlog_errno(-ENOMEM); return -ENOMEM; }
    (*si).si_extended = ocfs2_uses_extended_slot_map(osb) as _;
    (*si).si_num_slots = (*osb).max_slots;
    let inode = ocfs2_get_system_file_inode(osb, SLOT_MAP_SYSTEM_INODE, OCFS2_INVALID_SLOT);
    if inode.is_null() { __ocfs2_free_slot_info(si); mlog_errno(-EINVAL); return -EINVAL; }
    (*si).si_inode = inode;
    let status = ocfs2_map_slot_buffers(osb, si);
    if status < 0 { __ocfs2_free_slot_info(si); return status; }
    (*osb).slot_info = si; 0
}

pub unsafe fn ocfs2_free_slot_info(osb: *mut ocfs2_super) { let si = (*osb).slot_info; (*osb).slot_info = core::ptr::null_mut(); __ocfs2_free_slot_info(si); }

pub unsafe fn ocfs2_find_slot(osb: *mut ocfs2_super) -> ::core::ffi::c_int {
    let si = (*osb).slot_info; spin_lock(&mut (*osb).osb_lock); ocfs2_update_slot_info(si);
    let mut slot = __ocfs2_node_num_to_slot(si, (*osb).node_num);
    if slot < 0 { slot = __ocfs2_find_empty_slot(si, (*osb).preferred_slot); if slot < 0 { spin_unlock(&mut (*osb).osb_lock); mlog!(ML_ERROR, "no free slots available!\n"); return -EINVAL; } }
    else { printk!(KERN_INFO, "ocfs2: Slot %d on device (%s) was already allocated to this node!\n", slot, (*osb).dev_str); }
    ocfs2_set_slot(si, slot, (*osb).node_num); (*osb).slot_num = slot; spin_unlock(&mut (*osb).osb_lock);
    trace_ocfs2_find_slot((*osb).slot_num); let status = ocfs2_update_disk_slot(osb, si, (*osb).slot_num);
    if status < 0 { mlog_errno(status); spin_lock(&mut (*osb).osb_lock); ocfs2_invalidate_slot(si, (*osb).slot_num); (*osb).slot_num = OCFS2_INVALID_SLOT; spin_unlock(&mut (*osb).osb_lock); } status
}

pub unsafe fn ocfs2_put_slot(osb: *mut ocfs2_super) {
    let si = (*osb).slot_info; if si.is_null() { return; }
    spin_lock(&mut (*osb).osb_lock); ocfs2_update_slot_info(si); let slot_num = (*osb).slot_num; ocfs2_invalidate_slot(si, slot_num); (*osb).slot_num = OCFS2_INVALID_SLOT; spin_unlock(&mut (*osb).osb_lock);
    let status = ocfs2_update_disk_slot(osb, si, slot_num); if status < 0 { mlog_errno(status); } ocfs2_free_slot_info(osb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
