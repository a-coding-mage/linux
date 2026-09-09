// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS kernel bitmap handling.
 *
 * Copyright (c) 2004-2005 Anton Altaparmakov
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// Linux headers and local headers from the C implementation provide the
// types, constants, macros, and external functions referenced below.

pub unsafe fn ntfs_trim_fs(
    vol: *mut ntfs_volume,
    range: *mut fstrim_range,
) -> i32 {
    let mut buf_clusters: usize;
    let mut index: pgoff_t;
    let mut start_index: pgoff_t;
    let mut end_index: pgoff_t;
    let mut ra: *mut file_ra_state;
    let mut folio: *mut folio;
    let mut bitmap: *mut c_ulong;
    let mut kaddr: *mut c_char;
    let mut end: u64;
    let mut trimmed: u64 = 0;
    let mut start_buf: u64;
    let mut end_buf: u64;
    let mut end_cluster: u64;
    let start_cluster: u64 = ntfs_bytes_to_cluster(vol, (*range).start);
    let dq: u32 = {
        let mut v = bdev_discard_granularity((*(*vol).sb).s_bdev);
        if v == 0 { v = (*vol).cluster_size; }
        v
    };
    let mut ret: i32 = 0;

    if start_cluster >= (*vol).nr_clusters { return -EINVAL; }

    if (*range).len == u64::MAX {
        end_cluster = (*vol).nr_clusters;
    } else {
        end_cluster = ntfs_bytes_to_cluster(vol,
            (*range).start + (*range).len + (*vol).cluster_size as u64 - 1);
        if end_cluster > (*vol).nr_clusters { end_cluster = (*vol).nr_clusters; }
    }

    ra = kzalloc(core::mem::size_of::<file_ra_state>(), GFP_NOFS);
    if ra.is_null() { return -ENOMEM; }

    buf_clusters = PAGE_SIZE * 8;
    start_index = (start_cluster >> 15) as pgoff_t;
    end_index = ((end_cluster + buf_clusters as u64 - 1) >> 15) as pgoff_t;

    index = start_index;
    while index < end_index {
        folio = ntfs_get_locked_folio((*(*vol).lcnbmp_ino).i_mapping,
            index, end_index, ra);
        if IS_ERR(folio) {
            ret = PTR_ERR(folio);
            goto out_free;
        }
        kaddr = kmap_local_folio(folio, 0);
        bitmap = kaddr as *mut c_ulong;
        start_buf = core::cmp::max(index as u64 * buf_clusters as u64, start_cluster);
        end_buf = core::cmp::min((index as u64 + 1) * buf_clusters as u64, end_cluster);
        end = start_buf;
        while end < end_buf {
            let start = find_next_zero_bit(bitmap, (end_buf - start_buf) as usize,
                (end - start_buf) as usize) as u64 + start_buf;
            if start >= end_buf { break; }
            end = find_next_bit(bitmap, (end_buf - start_buf) as usize,
                (start - start_buf) as usize) as u64 + start_buf;
            let aligned_start = ALIGN(ntfs_cluster_to_bytes(vol, start), dq);
            let aligned_count = ALIGN_DOWN(ntfs_cluster_to_bytes(vol, end - start), dq);
            if aligned_count >= (*range).minlen {
                ret = blkdev_issue_discard((*(*vol).sb).s_bdev,
                    aligned_start >> 9, aligned_count >> 9, GFP_NOFS);
                if ret != 0 { goto out_unmap; }
                trimmed += aligned_count;
            }
        }
out_unmap:
        kunmap_local(kaddr);
        folio_unlock(folio);
        folio_put(folio);
        if ret != 0 { goto out_free; }
        index += 1;
    }
    (*range).len = trimmed;
out_free:
    kfree(ra);
    ret
}

/* Set a run of bits in a bitmap to a value; rollback is for internal use. */
pub unsafe fn __ntfs_bitmap_set_bits_in_run(
    vi: *mut inode, start_bit: i64, count: i64, value: u8, is_rollback: bool,
) -> i32 {
    let mut cnt = count;
    let mut index: pgoff_t;
    let end_index: pgoff_t;
    let mapping: *mut address_space;
    let mut folio: *mut folio;
    let mut kaddr: *mut u8;
    let mut pos: i32;
    let mut len: i64;
    let mut err: i32;
    let mut bit: u8;
    let ni: *mut ntfs_inode = NTFS_I(vi);
    let vol: *mut ntfs_volume = (*ni).vol;

    ntfs_debug("Entering for i_ino 0x%llx, start_bit 0x%llx, count 0x%llx, value %u.%s",
        (*ni).mft_no, start_bit as u64, cnt as u64, value,
        if is_rollback { " (rollback)" } else { "" });
    if start_bit < 0 || cnt < 0 || value > 1 { return -EINVAL; }
    index = (start_bit >> (3 + PAGE_SHIFT)) as pgoff_t;
    end_index = ((start_bit + cnt - 1) >> (3 + PAGE_SHIFT)) as pgoff_t;
    mapping = (*vi).i_mapping;
    folio = read_mapping_folio(mapping, index, core::ptr::null_mut());
    if IS_ERR(folio) {
        if !is_rollback { ntfs_error((*vi).i_sb, "Failed to map first page (error %li), aborting.", PTR_ERR(folio)); }
        return PTR_ERR(folio);
    }
    folio_lock(folio);
    kaddr = kmap_local_folio(folio, 0) as *mut u8;
    pos = ((start_bit >> 3) & !(PAGE_MASK as i64)) as i32;
    bit = (start_bit & 7) as u8;
    if bit != 0 {
        let byte = kaddr.add(pos as usize);
        if (*ni).mft_no == FILE_Bitmap { ntfs_set_lcn_empty_bits(vol, index, value, core::cmp::min(8 - bit as i64, cnt) as u64); }
        while (bit & 7) != 0 && cnt != 0 {
            cnt -= 1;
            if value != 0 { *byte |= 1 << bit; } else { *byte &= !(1 << bit); }
            bit += 1;
        }
        if cnt == 0 { goto done; }
        pos += 1;
    }
    len = core::cmp::min(cnt >> 3, PAGE_SIZE as i64 - pos as i64);
    core::ptr::write_bytes(kaddr.add(pos as usize), if value != 0 { 0xff } else { 0 }, len as usize);
    cnt -= len << 3;
    if (*ni).mft_no == FILE_Bitmap { ntfs_set_lcn_empty_bits(vol, index, value, (len << 3) as u64); }
    if cnt < 8 { len += pos as i64; }
    while index < end_index {
        if cnt <= 0 { err = -EIO; goto rollback; }
        folio_mark_dirty(folio); folio_unlock(folio); kunmap_local(kaddr); folio_put(folio);
        index += 1;
        folio = read_mapping_folio(mapping, index, core::ptr::null_mut());
        if IS_ERR(folio) { ntfs_error((*vi).i_sb, "Failed to map subsequent page (error %li), aborting.", PTR_ERR(folio)); err = PTR_ERR(folio); goto rollback; }
        folio_lock(folio); kaddr = kmap_local_folio(folio, 0) as *mut u8;
        len = core::cmp::min(cnt >> 3, PAGE_SIZE as i64);
        core::ptr::write_bytes(kaddr, if value != 0 { 0xff } else { 0 }, len as usize);
        cnt -= len << 3;
        if (*ni).mft_no == FILE_Bitmap { ntfs_set_lcn_empty_bits(vol, index, value, (len << 3) as u64); }
    }
    if cnt != 0 {
        WARN_ON(cnt > 7); bit = cnt as u8; let byte = kaddr.add(len as usize);
        if (*ni).mft_no == FILE_Bitmap { ntfs_set_lcn_empty_bits(vol, index, value, bit as u64); }
        while bit != 0 { bit -= 1; if value != 0 { *byte |= 1 << bit; } else { *byte &= !(1 << bit); } }
    }
done:
    folio_mark_dirty(folio); folio_unlock(folio); kunmap_local(kaddr); folio_put(folio); ntfs_debug("Done."); return 0;
rollback:
    if is_rollback { return err; }
    pos = if count != cnt { __ntfs_bitmap_set_bits_in_run(vi, start_bit, count - cnt, if value != 0 { 0 } else { 1 }, true) } else { 0 };
    if pos == 0 { ntfs_error((*vi).i_sb, "Failed to map subsequent page (error %i), aborting.", err); }
    else { ntfs_error((*vi).i_sb, "Failed to map subsequent page (error %i) and rollback failed (error %i). Aborting and leaving inconsistent metadata. Unmount and run chkdsk.", err, pos); NVolSetErrors(NTFS_SB((*vi).i_sb)); }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
