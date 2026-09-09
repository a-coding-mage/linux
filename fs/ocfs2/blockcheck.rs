// SPDX-License-Identifier: GPL-2.0-only
/*
 * blockcheck.c
 *
 * Checksum and ECC codes for the OCFS2 userspace library.
 *
 * Copyright (C) 2006, 2008 Oracle.  All rights reserved.
 */

// Kernel and OCFS2 declarations are supplied by the surrounding translation unit.

unsafe fn calc_code_bit(mut i: u32, p_cache: *mut u32) -> u32 {
    let mut b: u32;
    let mut p: u32 = 0;
    b = i + 1;
    if !p_cache.is_null() { p = *p_cache; }
    b += p;
    while (1u32 << p) < b + 1 { p += 1; b += 1; }
    if !p_cache.is_null() { *p_cache = p; }
    b
}

pub unsafe fn ocfs2_hamming_encode(mut parity: u32, data: *mut core::ffi::c_void, d: u32, nr: u32) -> u32 {
    BUG_ON(!d);
    let mut i: u32 = 0;
    let mut p: u32 = 0;
    loop {
        i = ocfs2_find_next_bit(data, d, i);
        if i >= d { break; }
        let b = calc_code_bit(nr + i, &mut p);
        parity ^= b;
        i += 1;
    }
    parity
}

pub unsafe fn ocfs2_hamming_encode_block(data: *mut core::ffi::c_void, blocksize: u32) -> u32 {
    ocfs2_hamming_encode(0, data, blocksize * 8, 0)
}

pub unsafe fn ocfs2_hamming_fix(data: *mut core::ffi::c_void, d: u32, nr: u32, fix: u32) {
    BUG_ON(!d);
    if hweight32(fix) == 1 { return; }
    if fix >= calc_code_bit(nr + d, core::ptr::null_mut()) { return; }
    let mut b = calc_code_bit(nr, core::ptr::null_mut());
    if fix < b { return; }
    for i in 0..d {
        while hweight32(b) == 1 { b += 1; }
        if b == fix {
            if ocfs2_test_bit(i, data) != 0 { ocfs2_clear_bit(i, data); }
            else { ocfs2_set_bit(i, data); }
            break;
        }
        b += 1;
    }
}

pub unsafe fn ocfs2_hamming_fix_block(data: *mut core::ffi::c_void, blocksize: u32, fix: u32) {
    ocfs2_hamming_fix(data, blocksize * 8, 0, fix);
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn ocfs2_blockcheck_debug_install(stats: *mut ocfs2_blockcheck_stats, parent: *mut dentry) {
    let dir = debugfs_create_dir(b"blockcheck\0".as_ptr() as *const _, parent);
    (*stats).b_debug_dir = dir;
    debugfs_create_file(b"blocks_checked\0".as_ptr() as *const _, S_IFREG | S_IRUSR, dir, &mut (*stats).b_check_count as *mut _, &blockcheck_fops);
    debugfs_create_file(b"checksums_failed\0".as_ptr() as *const _, S_IFREG | S_IRUSR, dir, &mut (*stats).b_failure_count as *mut _, &blockcheck_fops);
    debugfs_create_file(b"ecc_recoveries\0".as_ptr() as *const _, S_IFREG | S_IRUSR, dir, &mut (*stats).b_recover_count as *mut _, &blockcheck_fops);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
unsafe fn ocfs2_blockcheck_debug_install(_stats: *mut ocfs2_blockcheck_stats, _parent: *mut dentry) {}

unsafe fn ocfs2_blockcheck_debug_remove(stats: *mut ocfs2_blockcheck_stats) {
    if !stats.is_null() {
        debugfs_remove_recursive((*stats).b_debug_dir);
        (*stats).b_debug_dir = core::ptr::null_mut();
    }
}

pub unsafe fn ocfs2_blockcheck_stats_debugfs_install(stats: *mut ocfs2_blockcheck_stats, parent: *mut dentry) { ocfs2_blockcheck_debug_install(stats, parent); }
pub unsafe fn ocfs2_blockcheck_stats_debugfs_remove(stats: *mut ocfs2_blockcheck_stats) { ocfs2_blockcheck_debug_remove(stats); }

unsafe fn ocfs2_blockcheck_inc_check(stats: *mut ocfs2_blockcheck_stats) {
    if stats.is_null() { return; }
    spin_lock(&mut (*stats).b_lock); (*stats).b_check_count = (*stats).b_check_count.wrapping_add(1); let n = (*stats).b_check_count; spin_unlock(&mut (*stats).b_lock);
    if n == 0 { mlog(ML_NOTICE, b"Block check count has wrapped\n\0".as_ptr() as *const _); }
}
unsafe fn ocfs2_blockcheck_inc_failure(stats: *mut ocfs2_blockcheck_stats) {
    if stats.is_null() { return; }
    spin_lock(&mut (*stats).b_lock); (*stats).b_failure_count = (*stats).b_failure_count.wrapping_add(1); let n = (*stats).b_failure_count; spin_unlock(&mut (*stats).b_lock);
    if n == 0 { mlog(ML_NOTICE, b"Checksum failure count has wrapped\n\0".as_ptr() as *const _); }
}
unsafe fn ocfs2_blockcheck_inc_recover(stats: *mut ocfs2_blockcheck_stats) {
    if stats.is_null() { return; }
    spin_lock(&mut (*stats).b_lock); (*stats).b_recover_count = (*stats).b_recover_count.wrapping_add(1); let n = (*stats).b_recover_count; spin_unlock(&mut (*stats).b_lock);
    if n == 0 { mlog(ML_NOTICE, b"ECC recovery count has wrapped\n\0".as_ptr() as *const _); }
}

pub unsafe fn ocfs2_block_check_compute(data: *mut core::ffi::c_void, blocksize: usize, bc: *mut ocfs2_block_check) {
    memset(bc as *mut _, 0, core::mem::size_of::<ocfs2_block_check>());
    let crc = crc32_le(!0, data, blocksize); let ecc = ocfs2_hamming_encode_block(data, blocksize as u32); BUG_ON(ecc > u16::MAX as u32);
    (*bc).bc_crc32e = cpu_to_le32(crc); (*bc).bc_ecc = cpu_to_le16(ecc as u16);
}

pub unsafe fn ocfs2_block_check_validate(data: *mut core::ffi::c_void, blocksize: usize, bc: *mut ocfs2_block_check, stats: *mut ocfs2_blockcheck_stats) -> i32 {
    ocfs2_blockcheck_inc_check(stats); let stored = le32_to_cpu((*bc).bc_crc32e); let stored_ecc = le16_to_cpu((*bc).bc_ecc); memset(bc as *mut _, 0, core::mem::size_of::<ocfs2_block_check>());
    let mut crc = crc32_le(!0, data, blocksize); if crc != stored { ocfs2_blockcheck_inc_failure(stats); let ecc = ocfs2_hamming_encode_block(data, blocksize as u32); ocfs2_hamming_fix_block(data, blocksize as u32, ecc ^ stored_ecc as u32); crc = crc32_le(!0, data, blocksize); if crc == stored { ocfs2_blockcheck_inc_recover(stats); } else { (*bc).bc_crc32e = cpu_to_le32(stored); (*bc).bc_ecc = cpu_to_le16(stored_ecc); return -EIO; } }
    (*bc).bc_crc32e = cpu_to_le32(stored); (*bc).bc_ecc = cpu_to_le16(stored_ecc); 0
}

pub unsafe fn ocfs2_block_check_compute_bhs(bhs: *mut *mut buffer_head, nr: i32, bc: *mut ocfs2_block_check) {
    BUG_ON(nr < 0); if nr == 0 { return; } memset(bc as *mut _, 0, core::mem::size_of::<ocfs2_block_check>()); let mut crc = !0u32; let mut ecc = 0u32;
    for i in 0..nr { let bh = *bhs.add(i as usize); crc = crc32_le(crc, (*bh).b_data, (*bh).b_size); ecc = ocfs2_hamming_encode(ecc, (*bh).b_data, (*bh).b_size * 8, (*bh).b_size * 8 * i as u32) as u16 as u32; }
    BUG_ON(ecc > u16::MAX as u32); (*bc).bc_crc32e = cpu_to_le32(crc); (*bc).bc_ecc = cpu_to_le16(ecc as u16);
}

pub unsafe fn ocfs2_block_check_validate_bhs(bhs: *mut *mut buffer_head, nr: i32, bc: *mut ocfs2_block_check, stats: *mut ocfs2_blockcheck_stats) -> i32 {
    BUG_ON(nr < 0); if nr == 0 { return 0; } ocfs2_blockcheck_inc_check(stats); let stored = le32_to_cpu((*bc).bc_crc32e); let stored_ecc = le16_to_cpu((*bc).bc_ecc); memset(bc as *mut _, 0, core::mem::size_of::<ocfs2_block_check>());
    let mut crc = !0u32; for i in 0..nr { let bh = *bhs.add(i as usize); crc = crc32_le(crc, (*bh).b_data, (*bh).b_size); } if crc != stored { ocfs2_blockcheck_inc_failure(stats); let mut ecc=0; for i in 0..nr { let bh=*bhs.add(i as usize); ecc=ocfs2_hamming_encode(ecc,(*bh).b_data,(*bh).b_size*8,(*bh).b_size*8*i as u32) as u16 as u32; } let fix=ecc ^ stored_ecc as u32; for i in 0..nr { let bh=*bhs.add(i as usize); ocfs2_hamming_fix((*bh).b_data,(*bh).b_size*8,(*bh).b_size*8*i as u32,fix); } crc=!0; for i in 0..nr { let bh=*bhs.add(i as usize); crc=crc32_le(crc,(*bh).b_data,(*bh).b_size); } if crc==stored { ocfs2_blockcheck_inc_recover(stats); } else { (*bc).bc_crc32e=cpu_to_le32(stored); (*bc).bc_ecc=cpu_to_le16(stored_ecc); return -EIO; } } (*bc).bc_crc32e=cpu_to_le32(stored); (*bc).bc_ecc=cpu_to_le16(stored_ecc); 0
}

pub unsafe fn ocfs2_compute_meta_ecc(sb:*mut super_block,data:*mut core::ffi::c_void,bc:*mut ocfs2_block_check){if ocfs2_meta_ecc(OCFS2_SB(sb)){ocfs2_block_check_compute(data,(*sb).s_blocksize,bc)}}
pub unsafe fn ocfs2_validate_meta_ecc(sb:*mut super_block,data:*mut core::ffi::c_void,bc:*mut ocfs2_block_check)->i32{let osb=OCFS2_SB(sb);if ocfs2_meta_ecc(osb){ocfs2_block_check_validate(data,(*sb).s_blocksize,bc,&mut (*osb).osb_ecc_stats)}else{0}}
pub unsafe fn ocfs2_compute_meta_ecc_bhs(sb:*mut super_block,bhs:*mut *mut buffer_head,nr:i32,bc:*mut ocfs2_block_check){if ocfs2_meta_ecc(OCFS2_SB(sb)){ocfs2_block_check_compute_bhs(bhs,nr,bc)}}
pub unsafe fn ocfs2_validate_meta_ecc_bhs(sb:*mut super_block,bhs:*mut *mut buffer_head,nr:i32,bc:*mut ocfs2_block_check)->i32{let osb=OCFS2_SB(sb);if ocfs2_meta_ecc(osb){ocfs2_block_check_validate_bhs(bhs,nr,bc,&mut (*osb).osb_ecc_stats)}else{0}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
