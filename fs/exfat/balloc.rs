// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2012-2013 Samsung Electronics Co., Ltd.
 */

// Linux and exFAT headers supply the referenced types, constants, macros, and functions.
// The BITS_PER_LONG conditional selects the native little-endian bitmap word type.

/*
 *  Allocation Bitmap Management Functions
 */
unsafe fn exfat_test_bitmap_range(
    sb: *mut super_block,
    clu: ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) -> bool {
    let sbi = EXFAT_SB(sb);
    let mut start = clu;
    let end = clu.wrapping_add(count);
    let mut ent_idx: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint;
    let mut b: ::core::ffi::c_uint;
    let mut bit_offset: ::core::ffi::c_uint;
    let mut bits_to_check: ::core::ffi::c_uint;
    let bitmap_le: *mut __le_long;
    let mut mask: ::core::ffi::c_ulong;
    let mut word: ::core::ffi::c_ulong;

    if !is_valid_cluster(sbi, start) || !is_valid_cluster(sbi, end.wrapping_sub(1)) {
        return false;
    }

    while start < end {
        ent_idx = CLUSTER_TO_BITMAP_ENT(start);
        i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
        b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);

        bitmap_le = (*sbi).vol_amap[i as usize].b_data as *mut __le_long;

        /* Calculate how many bits we can check in the current word */
        bit_offset = b % BITS_PER_LONG;
        bits_to_check = core::cmp::min(
            end.wrapping_sub(start),
            (BITS_PER_LONG - bit_offset) as ::core::ffi::c_uint,
        );

        /* Create a bitmask for the range of bits to check */
        if bits_to_check >= BITS_PER_LONG {
            mask = !0;
        } else {
            mask = ((1 as ::core::ffi::c_ulong).wrapping_shl(bits_to_check) - 1)
                .wrapping_shl(bit_offset);
        }
        word = lel_to_cpu(*bitmap_le.add((b / BITS_PER_LONG) as usize));

        /* Check if all bits in the mask are set */
        if (word & mask) != mask {
            return false;
        }

        start = start.wrapping_add(bits_to_check);
    }

    true
}

unsafe fn exfat_allocate_bitmap(sb: *mut super_block, ep: *mut exfat_dentry) -> ::core::ffi::c_int {
    let sbi = EXFAT_SB(sb);
    let map_size: ::core::ffi::c_longlong;
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint;
    let need_map_size: ::core::ffi::c_uint;
    let mut sector: sector_t;
    let mut end: sector_t;
    let mut ra: sector_t;
    let mut ra_cnt: blkcnt_t = 0;

    (*sbi).map_clu = le32_to_cpu((*ep).dentry.bitmap.start_clu);
    map_size = le64_to_cpu((*ep).dentry.bitmap.size);
    need_map_size = ((EXFAT_DATA_CLUSTER_COUNT(sbi) - 1) / BITS_PER_BYTE) + 1;
    if need_map_size as ::core::ffi::c_longlong != map_size {
        exfat_err(sb, "bogus allocation bitmap size(need : %u, cur : %lld)", need_map_size, map_size);
        /* Only allowed when bogus allocation bitmap size is large */
        if need_map_size as ::core::ffi::c_longlong > map_size {
            return -EIO;
        }
    }
    (*sbi).map_sectors = ((need_map_size - 1) >> (*sb).s_blocksize_bits) + 1;
    (*sbi).vol_amap = kvmalloc_objs::<*mut buffer_head>((*sbi).map_sectors);
    if (*sbi).vol_amap.is_null() {
        return -ENOMEM;
    }

    sector = exfat_cluster_to_sector(sbi, (*sbi).map_clu);
    ra = sector;
    end = sector + (*sbi).map_sectors - 1;

    while i < (*sbi).map_sectors {
        /* Trigger the next readahead in advance. */
        exfat_blk_readahead(sb, sector + i, &mut ra, &mut ra_cnt, end);

        (*sbi).vol_amap[i as usize] = sb_bread(sb, sector + i);
        if (*sbi).vol_amap[i as usize].is_null() {
            break;
        }
        i += 1;
    }

    if i == (*sbi).map_sectors
        && exfat_test_bitmap_range(sb, (*sbi).map_clu, exfat_bytes_to_cluster_round_up(sbi, map_size))
    {
        return 0;
    }

    j = 0;
    /* release all buffers and free vol_amap */
    while j < i {
        brelse((*sbi).vol_amap[j as usize]);
        j += 1;
    }
    kvfree((*sbi).vol_amap as *mut ::core::ffi::c_void);
    (*sbi).vol_amap = core::ptr::null_mut();
    -EIO
}

pub unsafe fn exfat_load_bitmap(sb: *mut super_block) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint;
    let mut type_: ::core::ffi::c_uint;
    let mut clu: exfat_chain = core::mem::zeroed();
    let sbi = EXFAT_SB(sb);

    exfat_chain_set(&mut clu, (*sbi).root_dir, 0, ALLOC_FAT_CHAIN);
    while clu.dir != EXFAT_EOF_CLUSTER {
        i = 0;
        while i < (*sbi).dentries_per_clu {
            let mut bh: *mut buffer_head = core::ptr::null_mut();
            let ep = exfat_get_dentry(sb, &mut clu, i, &mut bh);
            if ep.is_null() {
                return -EIO;
            }

            type_ = exfat_get_entry_type(ep);
            if type_ == TYPE_BITMAP && (*ep).dentry.bitmap.flags == 0x0 {
                let err = exfat_allocate_bitmap(sb, ep);
                brelse(bh);
                return err;
            }
            brelse(bh);
            if type_ == TYPE_UNUSED {
                return -EINVAL;
            }
            i += 1;
        }
        if exfat_get_next_cluster(sb, &mut clu.dir) != 0 {
            return -EIO;
        }
    }
    -EINVAL
}

pub unsafe fn exfat_free_bitmap(sbi: *mut exfat_sb_info) {
    let mut i = 0;
    while i < (*sbi).map_sectors {
        __brelse((*sbi).vol_amap[i as usize]);
        i += 1;
    }
    kvfree((*sbi).vol_amap as *mut ::core::ffi::c_void);
}

pub unsafe fn exfat_set_bitmap(sb: *mut super_block, clu: ::core::ffi::c_uint, sync: bool) -> ::core::ffi::c_int {
    let sbi = EXFAT_SB(sb);
    if !is_valid_cluster(sbi, clu) { return -EINVAL; }
    let ent_idx = CLUSTER_TO_BITMAP_ENT(clu);
    let i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    let b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    set_bit_le(b, (*sbi).vol_amap[i as usize].b_data);
    exfat_update_bh((*sbi).vol_amap[i as usize], sync);
    0
}

pub unsafe fn exfat_clear_bitmap(sb: *mut super_block, clu: ::core::ffi::c_uint, sync: bool) -> ::core::ffi::c_int {
    let sbi = EXFAT_SB(sb);
    if !is_valid_cluster(sbi, clu) { return -EIO; }
    let ent_idx = CLUSTER_TO_BITMAP_ENT(clu);
    let i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    let b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    if !test_bit_le(b, (*sbi).vol_amap[i as usize].b_data) { return -EIO; }
    clear_bit_le(b, (*sbi).vol_amap[i as usize].b_data);
    exfat_update_bh((*sbi).vol_amap[i as usize], sync);
    0
}

pub unsafe fn exfat_test_bitmap(sb: *mut super_block, clu: ::core::ffi::c_uint) -> bool {
    let sbi = EXFAT_SB(sb);
    if (*sbi).vol_amap.is_null() { return true; }
    if !is_valid_cluster(sbi, clu) { return false; }
    let ent_idx = CLUSTER_TO_BITMAP_ENT(clu);
    let i = BITMAP_OFFSET_SECTOR_INDEX(sb, ent_idx);
    let b = BITMAP_OFFSET_BIT_IN_SECTOR(sb, ent_idx);
    test_bit_le(b, (*sbi).vol_amap[i as usize].b_data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
