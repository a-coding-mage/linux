// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ufs/util.c
 *
 * Copyright (C) 1998
 * Daniel Pirkl <daniel.pirkl@email.cz>
 * Charles University, Faculty of Mathematics and Physics
 */

// Dependencies are supplied by the surrounding kernel translation.

pub unsafe fn _ubh_bread_(
    uspi: *mut ufs_sb_private_info,
    sb: *mut super_block,
    fragment: u64,
    size: u64,
) -> *mut ufs_buffer_head {
    let mut ubh: *mut ufs_buffer_head;
    let mut i: u32;
    let mut j: u32;
    let count: u64;
    if (size & !(*uspi).s_fmask) != 0 {
        return core::ptr::null_mut();
    }
    count = size >> (*uspi).s_fshift;
    if count > UFS_MAXFRAG {
        return core::ptr::null_mut();
    }
    ubh = kmalloc_obj::<ufs_buffer_head>(GFP_NOFS);
    if ubh.is_null() {
        return core::ptr::null_mut();
    }
    (*ubh).fragment = fragment;
    (*ubh).count = count;
    i = 0;
    while (i as u64) < count {
        (*ubh).bh[i as usize] = sb_bread(sb, fragment + i as u64);
        if (*ubh).bh[i as usize].is_null() {
            j = 0;
            while j < i {
                brelse((*ubh).bh[j as usize]);
                j += 1;
            }
            kfree(ubh);
            return core::ptr::null_mut();
        }
        i += 1;
    }
    while (i as u64) < UFS_MAXFRAG {
        (*ubh).bh[i as usize] = core::ptr::null_mut();
        i += 1;
    }
    ubh
}

pub unsafe fn ubh_bread_uspi(
    uspi: *mut ufs_sb_private_info,
    sb: *mut super_block,
    fragment: u64,
    size: u64,
) -> *mut ufs_buffer_head {
    if (size & !(*uspi).s_fmask) != 0 {
        return core::ptr::null_mut();
    }
    let count = size >> (*uspi).s_fshift;
    if count == 0 || count > UFS_MAXFRAG {
        return core::ptr::null_mut();
    }
    let ubh = USPI_UBH(uspi);
    (*ubh).fragment = fragment;
    (*ubh).count = count;
    let mut i = 0u32;
    while (i as u64) < count {
        (*ubh).bh[i as usize] = sb_bread(sb, fragment + i as u64);
        if (*ubh).bh[i as usize].is_null() {
            let mut j = 0u32;
            while j < i {
                brelse((*ubh).bh[j as usize]);
                j += 1;
            }
            return core::ptr::null_mut();
        }
        i += 1;
    }
    while (i as u64) < UFS_MAXFRAG {
        (*ubh).bh[i as usize] = core::ptr::null_mut();
        i += 1;
    }
    ubh
}

pub unsafe fn ubh_brelse(ubh: *mut ufs_buffer_head) {
    if ubh.is_null() { return; }
    let mut i = 0u64;
    while i < (*ubh).count { brelse((*ubh).bh[i as usize]); i += 1; }
    kfree(ubh);
}

pub unsafe fn ubh_brelse_uspi(uspi: *mut ufs_sb_private_info) {
    let ubh = USPI_UBH(uspi);
    if ubh.is_null() { return; }
    let mut i = 0u64;
    while i < (*ubh).count {
        brelse((*ubh).bh[i as usize]);
        (*ubh).bh[i as usize] = core::ptr::null_mut();
        i += 1;
    }
}

pub unsafe fn ubh_mark_buffer_dirty(ubh: *mut ufs_buffer_head) {
    if ubh.is_null() { return; }
    let mut i = 0u64;
    while i < (*ubh).count { mark_buffer_dirty((*ubh).bh[i as usize]); i += 1; }
}

pub unsafe fn ubh_sync_block(ubh: *mut ufs_buffer_head) {
    if !ubh.is_null() {
        let mut i = 0u64;
        while i < (*ubh).count { write_dirty_buffer((*ubh).bh[i as usize], 0); i += 1; }
        i = 0;
        while i < (*ubh).count { wait_on_buffer((*ubh).bh[i as usize]); i += 1; }
    }
}

pub unsafe fn ubh_bforget(ubh: *mut ufs_buffer_head) {
    if ubh.is_null() { return; }
    let mut i = 0u64;
    while i < (*ubh).count {
        if !(*ubh).bh[i as usize].is_null() { bforget((*ubh).bh[i as usize]); }
        i += 1;
    }
}

pub unsafe fn ubh_buffer_dirty(ubh: *mut ufs_buffer_head) -> u32 {
    if ubh.is_null() { return 0; }
    let mut result = 0u32;
    let mut i = 0u64;
    while i < (*ubh).count { result |= buffer_dirty((*ubh).bh[i as usize]); i += 1; }
    result
}

pub unsafe fn ufs_get_inode_dev(sb: *mut super_block, ufsi: *mut ufs_inode_info) -> dev_t {
    let fs32: u32;
    if (UFS_SB(sb).s_flags & UFS_ST_MASK) == UFS_ST_SUNx86 {
        fs32 = fs32_to_cpu(sb, (*ufsi).i_u1.i_data[1]);
    } else { fs32 = fs32_to_cpu(sb, (*ufsi).i_u1.i_data[0]); }
    match UFS_SB(sb).s_flags & UFS_ST_MASK {
        UFS_ST_SUNx86 | UFS_ST_SUN => {
            if (fs32 & 0xffff0000) == 0 || (fs32 & 0xffff0000) == 0xffff0000 {
                old_decode_dev(fs32 & 0x7fff)
            } else { MKDEV(sysv_major(fs32), sysv_minor(fs32)) }
        }
        _ => old_decode_dev(fs32),
    }
}

pub unsafe fn ufs_set_inode_dev(sb: *mut super_block, ufsi: *mut ufs_inode_info, dev: dev_t) {
    let mut fs32;
    match UFS_SB(sb).s_flags & UFS_ST_MASK {
        UFS_ST_SUNx86 | UFS_ST_SUN => {
            fs32 = sysv_encode_dev(dev);
            if (fs32 & 0xffff8000) == 0 { fs32 = old_encode_dev(dev); }
        }
        _ => { fs32 = old_encode_dev(dev); }
    }
    if (UFS_SB(sb).s_flags & UFS_ST_MASK) == UFS_ST_SUNx86 {
        (*ufsi).i_u1.i_data[1] = cpu_to_fs32(sb, fs32);
    } else { (*ufsi).i_u1.i_data[0] = cpu_to_fs32(sb, fs32); }
}

pub unsafe fn ufs_get_locked_folio(mapping: *mut address_space, index: pgoff_t) -> *mut folio {
    let inode = (*mapping).host;
    let mut folio = filemap_lock_folio(mapping, index);
    if IS_ERR(folio) {
        folio = read_mapping_folio(mapping, index, core::ptr::null_mut());
        if IS_ERR(folio) {
            printk(KERN_ERR, "ufs_change_blocknr: read_mapping_folio error: ino %llu, index: %lu\n", (*(*mapping).host).i_ino, index);
            return folio;
        }
        folio_lock(folio);
        if (*folio).mapping.is_null() {
            folio_unlock(folio);
            folio_put(folio);
            return core::ptr::null_mut();
        }
    }
    if !folio_buffers(folio) { create_empty_buffers(folio, 1 << (*inode).i_blkbits, 0); }
    folio
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
