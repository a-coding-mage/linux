// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext4/bitmap.c
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 */

pub unsafe fn ext4_count_free(bitmap: *mut i8, numchars: u32) -> u32 {
    numchars
        .wrapping_mul(BITS_PER_BYTE as u32)
        .wrapping_sub(memweight(bitmap, numchars as usize) as u32)
}

pub unsafe fn ext4_inode_bitmap_csum_verify(
    sb: *mut super_block,
    gdp: *mut ext4_group_desc,
    bh: *mut buffer_head,
) -> i32 {
    let mut hi: u32;
    let provided: u32;
    let mut calculated: u32;
    let sbi: *mut ext4_sb_info = EXT4_SB(sb);
    let sz: i32;

    if !ext4_has_feature_metadata_csum(sb) {
        return 1;
    }

    sz = (EXT4_INODES_PER_GROUP(sb) >> 3) as i32;
    provided = le16_to_cpu((*gdp).bg_inode_bitmap_csum_lo) as u32;
    calculated = ext4_chksum(
        (*sbi).s_csum_seed,
        (*bh).b_data as *const u8,
        sz as usize,
    );
    if (*sbi).s_desc_size >= EXT4_BG_INODE_BITMAP_CSUM_HI_END {
        hi = le16_to_cpu((*gdp).bg_inode_bitmap_csum_hi) as u32;
        calculated = calculated;
        let provided = provided | hi.wrapping_shl(16);
        return (provided == calculated) as i32;
    } else {
        calculated &= 0xFFFF;
    }

    (provided == calculated) as i32
}

pub unsafe fn ext4_inode_bitmap_csum_set(
    sb: *mut super_block,
    gdp: *mut ext4_group_desc,
    bh: *mut buffer_head,
) {
    let csum: u32;
    let sbi: *mut ext4_sb_info = EXT4_SB(sb);
    let sz: i32;

    if !ext4_has_feature_metadata_csum(sb) {
        return;
    }

    sz = (EXT4_INODES_PER_GROUP(sb) >> 3) as i32;
    csum = ext4_chksum((*sbi).s_csum_seed, (*bh).b_data as *const u8, sz as usize);
    (*gdp).bg_inode_bitmap_csum_lo = cpu_to_le16(csum & 0xFFFF);
    if (*sbi).s_desc_size >= EXT4_BG_INODE_BITMAP_CSUM_HI_END {
        (*gdp).bg_inode_bitmap_csum_hi = cpu_to_le16(csum >> 16);
    }
}

pub unsafe fn ext4_block_bitmap_csum_verify(
    sb: *mut super_block,
    gdp: *mut ext4_group_desc,
    bh: *mut buffer_head,
) -> i32 {
    let mut hi: u32;
    let provided: u32;
    let mut calculated: u32;
    let sbi: *mut ext4_sb_info = EXT4_SB(sb);
    let sz: i32 = (EXT4_CLUSTERS_PER_GROUP(sb) / 8) as i32;

    if !ext4_has_feature_metadata_csum(sb) {
        return 1;
    }

    provided = le16_to_cpu((*gdp).bg_block_bitmap_csum_lo) as u32;
    calculated = ext4_chksum((*sbi).s_csum_seed, (*bh).b_data as *const u8, sz as usize);
    if (*sbi).s_desc_size >= EXT4_BG_BLOCK_BITMAP_CSUM_HI_END {
        hi = le16_to_cpu((*gdp).bg_block_bitmap_csum_hi) as u32;
        let provided = provided | hi.wrapping_shl(16);
        return (provided == calculated) as i32;
    } else {
        calculated &= 0xFFFF;
    }

    (provided == calculated) as i32
}

pub unsafe fn ext4_block_bitmap_csum_set(
    sb: *mut super_block,
    gdp: *mut ext4_group_desc,
    bh: *mut buffer_head,
) {
    let sz: i32 = (EXT4_CLUSTERS_PER_GROUP(sb) / 8) as i32;
    let csum: u32;
    let sbi: *mut ext4_sb_info = EXT4_SB(sb);

    if !ext4_has_feature_metadata_csum(sb) {
        return;
    }

    csum = ext4_chksum((*sbi).s_csum_seed, (*bh).b_data as *const u8, sz as usize);
    (*gdp).bg_block_bitmap_csum_lo = cpu_to_le16(csum & 0xFFFF);
    if (*sbi).s_desc_size >= EXT4_BG_BLOCK_BITMAP_CSUM_HI_END {
        (*gdp).bg_block_bitmap_csum_hi = cpu_to_le16(csum >> 16);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
