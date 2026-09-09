// SPDX-License-Identifier: GPL-2.0-or-later
/* NTFS kernel journal handling. */

/* Linux/kernel and local NTFS dependencies are supplied by the surrounding crate. */

unsafe fn ntfs_check_restart_page_header(vi: *mut inode, rp: *mut restart_page_header, pos: i64) -> bool {
    let system = le32_to_cpu((*rp).system_page_size);
    let log = le32_to_cpu((*rp).log_page_size);
    let mut usa_end: u16 = 0;
    let mut have_usa = true;
    ntfs_debug!("Entering.");
    if system < NTFS_BLOCK_SIZE || log < NTFS_BLOCK_SIZE || (system & (system - 1)) != 0 || !is_power_of_2(log) {
        ntfs_error!((*vi).i_sb, "LogFile uses unsupported page size."); return false;
    }
    if pos != 0 && pos != system as i64 { ntfs_error!((*vi).i_sb, "Found restart area in incorrect position in LogFile."); return false; }
    if le16_to_cpu((*rp).major_ver) != 1 || le16_to_cpu((*rp).minor_ver) != 1 {
        ntfs_error!((*vi).i_sb, "LogFile version %i.%i is not supported.  (This driver supports version 1.1 only.)", le16_to_cpu((*rp).major_ver) as i32, le16_to_cpu((*rp).minor_ver) as i32); return false;
    }
    if ntfs_is_chkd_record((*rp).magic) && le16_to_cpu((*rp).usa_count) == 0 { have_usa = false; } else {
        let usa_count = 1 + (system >> NTFS_BLOCK_SIZE_BITS);
        if usa_count != le16_to_cpu((*rp).usa_count) as u32 { ntfs_error!((*vi).i_sb, "LogFile restart page specifies inconsistent update sequence array count."); return false; }
        let usa_ofs = le16_to_cpu((*rp).usa_ofs);
        usa_end = usa_ofs + (usa_count as u16) * core::mem::size_of::<u16>() as u16;
        if usa_ofs < core::mem::size_of::<restart_page_header>() as u16 || usa_end as u32 > NTFS_BLOCK_SIZE - core::mem::size_of::<u16>() as u32 { ntfs_error!((*vi).i_sb, "LogFile restart page specifies inconsistent update sequence array offset."); return false; }
    }
    let ra_ofs = le16_to_cpu((*rp).restart_area_offset);
    if (ra_ofs & 7) != 0 || (if have_usa { ra_ofs < usa_end } else { ra_ofs < core::mem::size_of::<restart_page_header>() as u16 }) || ra_ofs as u32 > system { ntfs_error!((*vi).i_sb, "LogFile restart page specifies inconsistent restart area offset."); return false; }
    if !ntfs_is_chkd_record((*rp).magic) && le64_to_cpu((*rp).chkdsk_lsn) != 0 { ntfs_error!((*vi).i_sb, "LogFile restart page is not modified by chkdsk but a chkdsk LSN is specified."); return false; }
    ntfs_debug!("Done."); true
}

unsafe fn ntfs_check_restart_area(vi: *mut inode, rp: *mut restart_page_header) -> bool {
    let ra_ofs = le16_to_cpu((*rp).restart_area_offset) as usize;
    let ra = (rp as *mut u8).add(ra_ofs) as *mut restart_area;
    ntfs_debug!("Entering.");
    if ra_ofs + core::mem::offset_of!(restart_area, file_size) > NTFS_BLOCK_SIZE as usize - 2 { ntfs_error!((*vi).i_sb, "LogFile restart area specifies inconsistent file offset."); return false; }
    let ca_ofs = le16_to_cpu((*ra).client_array_offset) as usize;
    if ((ca_ofs + 7) & !7) != ca_ofs || ra_ofs + ca_ofs > NTFS_BLOCK_SIZE as usize - 2 { ntfs_error!((*vi).i_sb, "LogFile restart area specifies inconsistent client array offset."); return false; }
    let ra_len = ca_ofs + le16_to_cpu((*ra).log_clients) as usize * core::mem::size_of::<log_client_record>();
    let system = le32_to_cpu((*rp).system_page_size) as usize;
    if ra_ofs + ra_len > system || ra_ofs + le16_to_cpu((*ra).restart_area_length) as usize > system || ra_len > le16_to_cpu((*ra).restart_area_length) as usize { ntfs_error!((*vi).i_sb, "LogFile restart area is out of bounds of the system page size specified by the restart page header and/or the specified restart area length is inconsistent."); return false; }
    if ((*ra).client_free_list != LOGFILE_NO_CLIENT && le16_to_cpu((*ra).client_free_list) >= le16_to_cpu((*ra).log_clients)) || ((*ra).client_in_use_list != LOGFILE_NO_CLIENT && le16_to_cpu((*ra).client_in_use_list) >= le16_to_cpu((*ra).log_clients)) { ntfs_error!((*vi).i_sb, "LogFile restart area specifies overflowing client free and/or in use lists."); return false; }
    let mut file_size = le64_to_cpu((*ra).file_size); let mut fs_bits = 0u8; while file_size != 0 { file_size >>= 1; fs_bits += 1; }
    if le32_to_cpu((*ra).seq_number_bits) != 67 - fs_bits as u32 { ntfs_error!((*vi).i_sb, "LogFile restart area specifies inconsistent sequence number bits."); return false; }
    if ((le16_to_cpu((*ra).log_record_header_length) + 7) & !7) != le16_to_cpu((*ra).log_record_header_length) || ((le16_to_cpu((*ra).log_page_data_offset) + 7) & !7) != le16_to_cpu((*ra).log_page_data_offset) { ntfs_error!((*vi).i_sb, "LogFile restart area specifies inconsistent log record header length or log page data offset."); return false; }
    ntfs_debug!("Done."); true
}

unsafe fn ntfs_check_log_client_array(vi: *mut inode, rp: *mut restart_page_header) -> bool {
    let ra = (rp as *mut u8).add(le16_to_cpu((*rp).restart_area_offset) as usize) as *mut restart_area;
    let ca = (ra as *mut u8).add(le16_to_cpu((*ra).client_array_offset) as usize) as *mut log_client_record;
    let total = le16_to_cpu((*ra).log_clients); let mut nr = total; let mut idx = le16_to_cpu((*ra).client_free_list); let mut free = true;
    ntfs_debug!("Entering.");
    loop { let mut first = true; while idx != LOGFILE_NO_CLIENT_CPU { if nr == 0 || idx >= total { ntfs_error!((*vi).i_sb, "LogFile log client array is corrupt."); return false; } let cr = ca.add(idx as usize); if first { if (*cr).prev_client != LOGFILE_NO_CLIENT { ntfs_error!((*vi).i_sb, "LogFile log client array is corrupt."); return false; } first = false; } nr -= 1; idx = le16_to_cpu((*cr).next_client); } if !free { break; } free = false; idx = le16_to_cpu((*ra).client_in_use_list); }
    ntfs_debug!("Done."); true
}

unsafe fn ntfs_check_and_load_restart_page(vi: *mut inode, rp: *mut restart_page_header, pos: i64, wrp: *mut *mut restart_page_header, lsn: *mut i64) -> i32 {
    if !ntfs_check_restart_page_header(vi, rp, pos) || !ntfs_check_restart_area(vi, rp) { return -EINVAL; }
    let ra = (rp as *mut u8).add(le16_to_cpu((*rp).restart_area_offset) as usize) as *mut restart_area;
    let page_size = le32_to_cpu((*rp).system_page_size) as usize; let trp = kvzalloc(page_size, GFP_NOFS) as *mut restart_page_header; if trp.is_null() { ntfs_error!((*vi).i_sb, "Failed to allocate memory for LogFile restart page buffer."); return -ENOMEM; }
    let mut size = PAGE_SIZE - ((pos as usize) & !PAGE_MASK); let mut err = 0;
    if size >= page_size { core::ptr::copy_nonoverlapping(rp as *const u8, trp as *mut u8, page_size); } else { core::ptr::copy_nonoverlapping(rp as *const u8, trp as *mut u8, size); let mut have = size; let mut left = page_size - size; let mut idx = ((pos as usize + size) >> PAGE_SHIFT) as pgoff_t; while left > 0 { let folio = read_mapping_folio((*vi).i_mapping, idx, core::ptr::null_mut()); if IS_ERR(folio) { err = PTR_ERR(folio); if err != -EIO && err != -ENOMEM { err = -EIO; } kvfree(trp); return err; } size = core::cmp::min(left, PAGE_SIZE); core::ptr::copy_nonoverlapping(folio_address(folio) as *const u8, (trp as *mut u8).add(have), size); folio_put(folio); have += size; left -= size; idx += 1; } }
    if (!ntfs_is_chkd_record((*trp).magic) || le16_to_cpu((*trp).usa_count) != 0) && post_read_mst_fixup(trp as *mut ntfs_record, page_size as u32) != 0 && le16_to_cpu((*rp).restart_area_offset) as usize + le16_to_cpu((*ra).restart_area_length) as usize > NTFS_BLOCK_SIZE as usize - 2 { kvfree(trp); return -EINVAL; }
    if ntfs_is_rstr_record((*rp).magic) && (*ra).client_in_use_list != LOGFILE_NO_CLIENT && !ntfs_check_log_client_array(vi, trp) { kvfree(trp); return -EINVAL; }
    if !lsn.is_null() { *lsn = if ntfs_is_rstr_record((*rp).magic) { le64_to_cpu((*ra).current_lsn) as i64 } else { le64_to_cpu((*rp).chkdsk_lsn) as i64 }; }
    if !wrp.is_null() { *wrp = trp; } else { kvfree(trp); } err
}

pub unsafe fn ntfs_check_logfile(log_vi: *mut inode, rp: *mut *mut restart_page_header) -> bool {
    let vol = NTFS_SB((*log_vi).i_sb); let mapping = (*log_vi).i_mapping; let mut r1: *mut restart_page_header = core::ptr::null_mut(); let mut r2: *mut restart_page_header = core::ptr::null_mut(); let mut l1 = 0i64; let mut l2 = 0i64; let mut empty = true;
    if NVolLogFileEmpty(vol) { return true; } let mut size = i_size_read(log_vi); if size > MaxLogFileSize { size = MaxLogFileSize; }
    let page = if DefaultLogPageSize <= PAGE_SIZE && DefaultLogPageSize * 2 <= PAGE_SIZE { DefaultLogPageSize } else { PAGE_SIZE }; let bits = ntfs_ffs(page) - 1; size &= !((page - 1) as i64); if size < (page * 2) as i64 || ((size - (page * 2) as i64) >> bits) < MinLogRecordPages { ntfs_error!((*vol).sb, "LogFile is too small."); return false; }
    let mut pos = 0i64; while pos < size { let idx = (pos >> PAGE_SHIFT) as pgoff_t; let folio = read_mapping_folio(mapping, idx, core::ptr::null_mut()); if IS_ERR(folio) { return false; } let kaddr = (kmap_local_folio(folio, 0) as *mut u8).add((pos as usize) & !PAGE_MASK); if !ntfs_is_empty_recordp(kaddr as *mut __le32) { empty = false; } else if !empty { kunmap_local(kaddr as *mut _); folio_put(folio); break; } if ntfs_is_rcrd_recordp(kaddr as *mut __le32) { kunmap_local(kaddr as *mut _); folio_put(folio); break; } if ntfs_is_rstr_recordp(kaddr as *mut __le32) || ntfs_is_chkd_recordp(kaddr as *mut __le32) { let e = ntfs_check_and_load_restart_page(log_vi, kaddr as *mut restart_page_header, pos, if r1.is_null() { &mut r1 } else { &mut r2 }, if r1.is_null() { &mut l1 } else { &mut l2 }); if e == 0 && pos != 0 { kunmap_local(kaddr as *mut _); folio_put(folio); break; } if e != -EINVAL { kunmap_local(kaddr as *mut _); folio_put(folio); return false; } } kunmap_local(kaddr as *mut _); folio_put(folio); if pos == 0 { pos = (NTFS_BLOCK_SIZE >> 1) as i64; } else { pos <<= 1; } }
    if empty { NVolSetLogFileEmpty(vol); return true; } if r1.is_null() { return false; } if !r2.is_null() { if l2 > l1 { kvfree(r1); r1 = r2; } else { kvfree(r2); } } if !rp.is_null() { *rp = r1; } else { kvfree(r1); } true
}

pub unsafe fn ntfs_empty_logfile(log_vi: *mut inode) -> bool {
    let log_ni = NTFS_I(log_vi); let vol = (*log_ni).vol; let sb = (*vol).sb; if NVolLogFileEmpty(vol) { return true; }
    let mut vcn = 0i64; let end_vcn = ((*log_ni).initialized_size + (*vol).cluster_size_mask) >> (*vol).cluster_size_bits; truncate_inode_pages((*log_vi).i_mapping, 0); down_write(&mut (*log_ni).runlist.lock); let mut rl = (*log_ni).runlist.rl; if rl.is_null() || vcn < (*rl).vcn || (*rl).length == 0 { let e = ntfs_map_runlist_nolock(log_ni, vcn, core::ptr::null_mut()); if e != 0 { up_write(&mut (*log_ni).runlist.lock); return false; } rl = (*log_ni).runlist.rl; }
    while (*rl).length != 0 && vcn >= (*rl.add(1)).vcn { rl = rl.add(1); } let buf = kvzalloc((*vol).cluster_size, GFP_NOFS) as *mut u8; if buf.is_null() { up_write(&mut (*log_ni).runlist.lock); return false; } core::ptr::write_bytes(buf, 0xff, (*vol).cluster_size); let mut cur = rl; while (*cur).vcn < end_vcn { if (*cur).lcn == LCN_RL_NOT_MAPPED { kvfree(buf); up_write(&mut (*log_ni).runlist.lock); return false; } if (*cur).length == 0 || (*cur).lcn < LCN_HOLE { kvfree(buf); up_write(&mut (*log_ni).runlist.lock); return false; } if (*cur).lcn != LCN_HOLE { let start = NTFS_CLU_TO_B(vol, (*cur).lcn); let len = core::cmp::min((*cur).length, end_vcn - (*cur).vcn); let mut p = start; let end = NTFS_CLU_TO_B(vol, (*cur).lcn + len); while p < end { if ntfs_bdev_write(sb, buf, p, (*vol).cluster_size) != 0 { kvfree(buf); up_write(&mut (*log_ni).runlist.lock); return false; } p += (*vol).cluster_size; } } cur = cur.add(1); } up_write(&mut (*log_ni).runlist.lock); kvfree(buf); truncate_inode_pages((*log_vi).i_mapping, 0); NVolSetLogFileEmpty(vol); true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
