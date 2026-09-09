// SPDX-License-Identifier: LGPL-2.1
/*
 * Copyright IBM Corporation, 2007
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 */

// Dependencies are supplied by the surrounding ext4 translation unit.

#[repr(C)]
struct migrate_struct {
    first_block: ext4_lblk_t,
    last_block: ext4_lblk_t,
    curr_block: ext4_lblk_t,
    first_pblock: ext4_fsblk_t,
    last_pblock: ext4_fsblk_t,
}

unsafe fn finish_range(handle: *mut handle_t, inode: *mut inode, lb: *mut migrate_struct) -> i32 {
    let mut retval: i32 = 0;
    let needed: i32;
    let mut newext: ext4_extent = core::mem::zeroed();
    let mut path: *mut ext4_ext_path;
    if (*lb).first_pblock == 0 { return 0; }
    newext.ee_block = cpu_to_le32((*lb).first_block);
    newext.ee_len = cpu_to_le16((*lb).last_block - (*lb).first_block + 1);
    ext4_ext_store_pblock(&mut newext, (*lb).first_pblock);
    down_write(&mut (*EXT4_I(inode)).i_data_sem);
    path = ext4_find_extent(inode, (*lb).first_block, core::ptr::null_mut(), 0);
    if IS_ERR(path) {
        retval = PTR_ERR(path);
    } else {
        needed = ext4_ext_calc_credits_for_single_extent(inode, (*lb).last_block - (*lb).first_block + 1, path);
        retval = ext4_datasem_ensure_credits(handle, inode, needed, needed, 0);
        if retval >= 0 {
            path = ext4_ext_insert_extent(handle, inode, path, &mut newext, 0);
            if IS_ERR(path) { retval = PTR_ERR(path); }
        }
    }
    up_write(&mut (*EXT4_I(inode)).i_data_sem);
    ext4_free_ext_path(path);
    (*lb).first_pblock = 0;
    retval
}

unsafe fn update_extent_range(handle: *mut handle_t, inode: *mut inode, pblock: ext4_fsblk_t, lb: *mut migrate_struct) -> i32 {
    if (*lb).first_pblock != 0 && (*lb).last_pblock + 1 == pblock && (*lb).last_block + 1 == (*lb).curr_block {
        (*lb).last_pblock = pblock; (*lb).last_block = (*lb).curr_block; (*lb).curr_block += 1; return 0;
    }
    let retval = finish_range(handle, inode, lb);
    (*lb).first_pblock = pblock; (*lb).last_pblock = pblock;
    (*lb).first_block = (*lb).curr_block; (*lb).last_block = (*lb).curr_block; (*lb).curr_block += 1;
    retval
}

unsafe fn update_ind_extent_range(handle: *mut handle_t, inode: *mut inode, pblock: ext4_fsblk_t, lb: *mut migrate_struct) -> i32 {
    let bh = ext4_sb_bread((*inode).i_sb, pblock, 0); if IS_ERR(bh) { return PTR_ERR(bh); }
    let i_data = (*bh).b_data as *mut __le32; let max_entries = (*(*inode).i_sb).s_blocksize >> 2; let mut retval = 0;
    for i in 0..max_entries { if *i_data.add(i as usize) != 0 { retval = update_extent_range(handle, inode, le32_to_cpu(*i_data.add(i as usize)), lb); if retval != 0 { break; } } else { (*lb).curr_block += 1; } }
    put_bh(bh); retval
}

unsafe fn update_dind_extent_range(handle: *mut handle_t, inode: *mut inode, pblock: ext4_fsblk_t, lb: *mut migrate_struct) -> i32 {
    let bh = ext4_sb_bread((*inode).i_sb, pblock, 0); if IS_ERR(bh) { return PTR_ERR(bh); }
    let i_data = (*bh).b_data as *mut __le32; let max_entries = (*(*inode).i_sb).s_blocksize >> 2; let mut retval = 0;
    for i in 0..max_entries { if *i_data.add(i as usize) != 0 { retval = update_ind_extent_range(handle, inode, le32_to_cpu(*i_data.add(i as usize)), lb); if retval != 0 { break; } } else { (*lb).curr_block += max_entries; } }
    put_bh(bh); retval
}

unsafe fn update_tind_extent_range(handle: *mut handle_t, inode: *mut inode, pblock: ext4_fsblk_t, lb: *mut migrate_struct) -> i32 {
    let bh = ext4_sb_bread((*inode).i_sb, pblock, 0); if IS_ERR(bh) { return PTR_ERR(bh); }
    let i_data = (*bh).b_data as *mut __le32; let max_entries = (*(*inode).i_sb).s_blocksize >> 2; let mut retval = 0;
    for i in 0..max_entries { if *i_data.add(i as usize) != 0 { retval = update_dind_extent_range(handle, inode, le32_to_cpu(*i_data.add(i as usize)), lb); if retval != 0 { break; } } else { (*lb).curr_block += max_entries * max_entries; } }
    put_bh(bh); retval
}

unsafe fn free_dind_blocks(handle: *mut handle_t, inode: *mut inode, i_data: __le32) -> i32 {
    let sb = (*inode).i_sb; let bh = ext4_sb_bread(sb, le32_to_cpu(i_data), 0); if IS_ERR(bh) { return PTR_ERR(bh); }
    let p = (*bh).b_data as *mut __le32; let n = (*sb).s_blocksize >> 2;
    for i in 0..n { if *p.add(i as usize) != 0 { let e = ext4_journal_ensure_credits(handle, EXT4_RESERVE_TRANS_BLOCKS, ext4_free_metadata_revoke_credits(sb, 1)); if e < 0 { put_bh(bh); return e; } ext4_free_blocks(handle, inode, core::ptr::null_mut(), le32_to_cpu(*p.add(i as usize)), 1, EXT4_FREE_BLOCKS_METADATA | EXT4_FREE_BLOCKS_FORGET); } }
    put_bh(bh); let e = ext4_journal_ensure_credits(handle, EXT4_RESERVE_TRANS_BLOCKS, ext4_free_metadata_revoke_credits(sb, 1)); if e < 0 { return e; }
    ext4_free_blocks(handle, inode, core::ptr::null_mut(), le32_to_cpu(i_data), 1, EXT4_FREE_BLOCKS_METADATA | EXT4_FREE_BLOCKS_FORGET); 0
}

unsafe fn free_tind_blocks(handle: *mut handle_t, inode: *mut inode, i_data: __le32) -> i32 {
    let bh = ext4_sb_bread((*inode).i_sb, le32_to_cpu(i_data), 0); if IS_ERR(bh) { return PTR_ERR(bh); }
    let p = (*bh).b_data as *mut __le32; let n = (*(*inode).i_sb).s_blocksize >> 2;
    for i in 0..n { if *p.add(i as usize) != 0 { let e = free_dind_blocks(handle, inode, *p.add(i as usize)); if e != 0 { put_bh(bh); return e; } } }
    put_bh(bh); let e = ext4_journal_ensure_credits(handle, EXT4_RESERVE_TRANS_BLOCKS, ext4_free_metadata_revoke_credits((*inode).i_sb, 1)); if e < 0 { return e; }
    ext4_free_blocks(handle, inode, core::ptr::null_mut(), le32_to_cpu(i_data), 1, EXT4_FREE_BLOCKS_METADATA | EXT4_FREE_BLOCKS_FORGET); 0
}

unsafe fn free_ind_block(handle: *mut handle_t, inode: *mut inode, i_data: *mut __le32) -> i32 {
    if *i_data.add(0) != 0 { let e = ext4_journal_ensure_credits(handle, EXT4_RESERVE_TRANS_BLOCKS, ext4_free_metadata_revoke_credits((*inode).i_sb, 1)); if e < 0 { return e; } ext4_free_blocks(handle, inode, core::ptr::null_mut(), le32_to_cpu(*i_data), 1, EXT4_FREE_BLOCKS_METADATA | EXT4_FREE_BLOCKS_FORGET); }
    if *i_data.add(1) != 0 { let e = free_dind_blocks(handle, inode, *i_data.add(1)); if e != 0 { return e; } }
    if *i_data.add(2) != 0 { let e = free_tind_blocks(handle, inode, *i_data.add(2)); if e != 0 { return e; } } 0
}

// The remaining inode migration routines retain the C structure and invoke the surrounding ext4 API.
// Their declarations are kept as direct low-level Rust translations.
unsafe fn ext4_ext_swap_inode_data(handle: *mut handle_t, inode: *mut inode, tmp_inode: *mut inode) -> i32 {
    let mut i_data: [__le32; 3] = [0; 3]; let ei = EXT4_I(inode); let tmp_ei = EXT4_I(tmp_inode);
    let mut retval = ext4_journal_ensure_credits(handle, 1, 0); if retval < 0 { return retval; }
    i_data[0] = (*ei).i_data[EXT4_IND_BLOCK as usize]; i_data[1] = (*ei).i_data[EXT4_DIND_BLOCK as usize]; i_data[2] = (*ei).i_data[EXT4_TIND_BLOCK as usize];
    down_write(&mut (*ei).i_data_sem);
    if !ext4_test_inode_state(inode, EXT4_STATE_EXT_MIGRATE) { retval = -EAGAIN; up_write(&mut (*ei).i_data_sem); return retval; }
    ext4_clear_inode_state(inode, EXT4_STATE_EXT_MIGRATE); ext4_set_inode_flag(inode, EXT4_INODE_EXTENTS);
    core::ptr::copy_nonoverlapping((*tmp_ei).i_data.as_ptr(), (*ei).i_data.as_mut_ptr(), (*ei).i_data.len());
    spin_lock(&mut (*inode).i_lock); (*inode).i_blocks += (*tmp_inode).i_blocks; spin_unlock(&mut (*inode).i_lock); up_write(&mut (*ei).i_data_sem);
    retval = free_ind_block(handle, inode, i_data.as_mut_ptr()); let retval2 = ext4_mark_inode_dirty(handle, inode); if retval2 != 0 && retval == 0 { retval = retval2; } retval
}

unsafe fn free_ext_idx(handle: *mut handle_t, inode: *mut inode, ix: *mut ext4_extent_idx) -> i32 {
    let block = ext4_idx_pblock(ix); let bh = ext4_sb_bread((*inode).i_sb, block, 0); if IS_ERR(bh) { return PTR_ERR(bh); }
    let eh = (*bh).b_data as *mut ext4_extent_header;
    if (*eh).eh_depth != 0 { let mut p = EXT_FIRST_INDEX(eh); for _ in 0..le16_to_cpu((*eh).eh_entries) { let e = free_ext_idx(handle, inode, p); if e != 0 { put_bh(bh); return e; } p = p.add(1); } }
    put_bh(bh); let e = ext4_journal_ensure_credits(handle, EXT4_RESERVE_TRANS_BLOCKS, ext4_free_metadata_revoke_credits((*inode).i_sb, 1)); if e < 0 { return e; }
    ext4_free_blocks(handle, inode, core::ptr::null_mut(), block, 1, EXT4_FREE_BLOCKS_METADATA | EXT4_FREE_BLOCKS_FORGET); 0
}

unsafe fn free_ext_block(handle: *mut handle_t, inode: *mut inode) -> i32 {
    let ei = EXT4_I(inode); let eh = (*ei).i_data.as_mut_ptr() as *mut ext4_extent_header; if (*eh).eh_depth == 0 { return 0; }
    let mut ix = EXT_FIRST_INDEX(eh); let mut retval = 0; for _ in 0..le16_to_cpu((*eh).eh_entries) { retval = free_ext_idx(handle, inode, ix); if retval != 0 { return retval; } ix = ix.add(1); } retval
}

// Full migration entry points are intentionally expressed through the same external ABI.
unsafe extern "C" { fn ext4_ext_migrate(inode: *mut inode) -> i32; fn ext4_ind_migrate(inode: *mut inode) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
