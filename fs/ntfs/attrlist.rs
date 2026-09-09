// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Attribute list attribute handling code.
 * Part of this file is based on code from the NTFS-3G.
 *
 * Copyright (c) 2004-2005 Anton Altaparmakov
 * Copyright (c) 2004-2005 Yura Pakhuchiy
 * Copyright (c)      2006 Szabolcs Szakacsits
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// Dependencies supplied by the surrounding NTFS implementation are intentionally external.

/*
 * ntfs_attrlist_need - check whether inode need attribute list
 * @ni: opened ntfs inode for which perform check
 *
 * Check whether all are attributes belong to one MFT record, in that case
 * attribute list is not needed.
 *
 * Return 1 if inode need attribute list, 0 if not, or -errno on error.
 */
pub unsafe fn ntfs_attrlist_need(ni: *mut ntfs_inode) -> i32 {
    let mut ale: *mut attr_list_entry;

    if ni.is_null() {
        ntfs_debug(c"Invalid arguments.\n");
        return -EINVAL;
    }
    ntfs_debug(c"Entering for inode 0x%llx.\n", (*ni).mft_no as i64);

    if !NInoAttrList(ni) {
        ntfs_debug(c"Inode haven't got attribute list.\n");
        return -EINVAL;
    }
    if (*ni).attr_list.is_null() {
        ntfs_debug(c"Corrupt in-memory struct.\n");
        return -EINVAL;
    }

    ale = (*ni).attr_list as *mut attr_list_entry;
    while (ale as *mut u8) < (*ni).attr_list.add((*ni).attr_list_size as usize) {
        if MREF_LE((*ale).mft_reference) != (*ni).mft_no {
            return 1;
        }
        ale = (ale as *mut u8).add(le16_to_cpu((*ale).length) as usize) as *mut attr_list_entry;
    }
    0
}

pub unsafe fn ntfs_attrlist_update(base_ni: *mut ntfs_inode) -> i32 {
    let attr_vi: *mut inode;
    let attr_ni: *mut ntfs_inode;
    let mut err: i32;

    /* generic_shutdown_super() clears SB_ACTIVE before evicting cached
     * inodes. Do not look up the attribute-list inode after SB_ACTIVE has
     * been cleared; it may already be I_FREEING, and waiting on it can
     * self-deadlock. */
    if (*VFS_I(base_ni)).i_sb.as_ref().unwrap().s_flags & SB_ACTIVE == 0 {
        return -EIO;
    }
    attr_vi = ntfs_attr_iget(VFS_I(base_ni), AT_ATTRIBUTE_LIST, AT_UNNAMED, 0);
    if IS_ERR(attr_vi) {
        return PTR_ERR(attr_vi);
    }
    attr_ni = NTFS_I(attr_vi);

    err = ntfs_attr_truncate_i(attr_ni, (*base_ni).attr_list_size, HOLES_NO);
    if err == -ENOSPC && (*attr_ni).mft_no == FILE_MFT {
        err = ntfs_attr_truncate(attr_ni, 0);
        if err != 0 || ntfs_attr_truncate_i(attr_ni, (*base_ni).attr_list_size, HOLES_NO) != 0 {
            iput(attr_vi);
            ntfs_error((*base_ni).vol).sb, c"Failed to truncate attribute list of inode %#llx", (*base_ni).mft_no as i64);
            return -EIO;
        }
    } else if err != 0 {
        iput(attr_vi);
        ntfs_error((*base_ni).vol).sb, c"Failed to truncate attribute list of inode %#llx", (*base_ni).mft_no as i64);
        return -EIO;
    }

    i_size_write(attr_vi, (*base_ni).attr_list_size);
    if NInoNonResident(attr_ni) && !NInoAttrListNonResident(base_ni) {
        NInoSetAttrListNonResident(base_ni);
    }
    if ntfs_inode_attr_pwrite(attr_vi, 0, (*base_ni).attr_list_size, (*base_ni).attr_list, false) != (*base_ni).attr_list_size {
        iput(attr_vi);
        ntfs_error((*base_ni).vol).sb, c"Failed to write attribute list of inode %#llx", (*base_ni).mft_no as i64);
        return -EIO;
    }
    NInoSetAttrListDirty(base_ni);
    iput(attr_vi);
    0
}

pub unsafe fn ntfs_attrlist_entry_add(ni: *mut ntfs_inode, attr: *mut attr_record) -> i32 {
    let mut ale: *mut attr_list_entry;
    let mut mref: __le64;
    let ctx: *mut ntfs_attr_search_ctx;
    let new_al: *mut u8;
    let entry_len: i32;
    let entry_offset: usize;
    let mut err: i32;
    let ni_mrec: *mut mft_record;
    let old_al: *mut u8;
    let lowest_vcn: __le64;

    if ni.is_null() || attr.is_null() { ntfs_debug(c"Invalid arguments.\n"); return -EINVAL; }
    ntfs_debug(c"Entering for inode 0x%llx, attr 0x%x.\n", (*ni).mft_no as i64, le32_to_cpu((*attr).type_));
    ni_mrec = map_mft_record(ni);
    if IS_ERR(ni_mrec) { ntfs_debug(c"Invalid arguments.\n"); return -EIO; }
    mref = MK_LE_MREF((*ni).mft_no, le16_to_cpu((*ni_mrec).sequence_number));
    unmap_mft_record(ni);
    let mut work_ni = ni;
    if (*work_ni).nr_extents == -1 { work_ni = (*work_ni).ext.base_ntfs_ino; }
    if !NInoAttrList(work_ni) { ntfs_debug(c"Attribute list isn't present.\n"); return -ENOENT; }
    entry_len = ((core::mem::size_of::<attr_list_entry>() + core::mem::size_of::<__le16>() * (*attr).name_length as usize + 7) & !7) as i32;
    new_al = kvzalloc((*work_ni).attr_list_size as usize + entry_len as usize, GFP_NOFS);
    if new_al.is_null() { return -ENOMEM; }
    ctx = ntfs_attr_get_search_ctx(work_ni, core::ptr::null_mut());
    if ctx.is_null() { ntfs_error((*work_ni).vol).sb, c"Failed to get search context"; kvfree(new_al); return -ENOMEM; }
    lowest_vcn = if (*attr).non_resident { (*attr).data.non_resident.lowest_vcn } else { 0 };
    err = ntfs_attr_lookup((*attr).type_, if (*attr).name_length != 0 { ((*attr as *mut u8).add(le16_to_cpu((*attr).name_offset) as usize)) as *mut __le16 } else { AT_UNNAMED }, (*attr).name_length, CASE_SENSITIVE, le64_to_cpu(lowest_vcn), core::ptr::null(), 0, ctx);
    if err == 0 {
        if (*ctx).al_entry.as_ref().unwrap().lowest_vcn == lowest_vcn { ntfs_attr_put_search_ctx(ctx); kvfree(new_al); return -EEXIST; }
        ale = ((*ctx).al_entry as *mut u8).add(le16_to_cpu((*(*ctx).al_entry).length) as usize) as *mut attr_list_entry;
    } else if err == -ENOENT { ale = (*ctx).al_entry; } else { ntfs_attr_put_search_ctx(ctx); kvfree(new_al); return err; }
    ntfs_attr_put_search_ctx(ctx);
    entry_offset = ale as usize - (*work_ni).attr_list as usize;
    ale = new_al.add(entry_offset) as *mut attr_list_entry;
    core::ptr::write_bytes(ale as *mut u8, 0, entry_len as usize);
    (*ale).type_ = (*attr).type_; (*ale).length = cpu_to_le16(entry_len as u16); (*ale).name_length = (*attr).name_length; (*ale).name_offset = core::mem::offset_of!(attr_list_entry, name) as u8; (*ale).lowest_vcn = lowest_vcn; (*ale).mft_reference = mref; (*ale).instance = (*attr).instance;
    core::ptr::copy_nonoverlapping((attr as *mut u8).add(le16_to_cpu((*attr).name_offset) as usize), (*ale).name.as_mut_ptr() as *mut u8, (*attr).name_length as usize * core::mem::size_of::<__le16>());
    core::ptr::copy_nonoverlapping((*work_ni).attr_list, new_al, entry_offset);
    core::ptr::copy_nonoverlapping((*work_ni).attr_list.add(entry_offset), new_al.add(entry_offset + entry_len as usize), (*work_ni).attr_list_size as usize - entry_offset);
    old_al = (*work_ni).attr_list; (*work_ni).attr_list = new_al; (*work_ni).attr_list_size += entry_len as usize;
    err = ntfs_attrlist_update(work_ni);
    if err != 0 { (*work_ni).attr_list = old_al; (*work_ni).attr_list_size -= entry_len as usize; kvfree(new_al); return err; }
    kvfree(old_al); 0
}

pub unsafe fn ntfs_attrlist_entry_rm(ctx: *mut ntfs_attr_search_ctx) -> i32 {
    if ctx.is_null() || (*ctx).ntfs_ino.is_null() || (*ctx).al_entry.is_null() { ntfs_debug(c"Invalid arguments.\n"); return -EINVAL; }
    let base_ni = if !(*ctx).base_ntfs_ino.is_null() { (*ctx).base_ntfs_ino } else { (*ctx).ntfs_ino };
    let ale = (*ctx).al_entry;
    if !NInoAttrList(base_ni) { ntfs_debug(c"Attribute list isn't present.\n"); return -ENOENT; }
    let new_al_len = (*base_ni).attr_list_size - le16_to_cpu((*ale).length) as usize;
    let new_al = kvzalloc(new_al_len, GFP_NOFS);
    if new_al.is_null() { return -ENOMEM; }
    let offset = ale as usize - (*base_ni).attr_list as usize;
    core::ptr::copy_nonoverlapping((*base_ni).attr_list, new_al, offset);
    core::ptr::copy_nonoverlapping((ale as *mut u8).add(le16_to_cpu((*ale).length) as usize), new_al.add(offset), new_al_len - offset);
    kvfree((*base_ni).attr_list); (*base_ni).attr_list = new_al; (*base_ni).attr_list_size = new_al_len;
    ntfs_attrlist_update(base_ni)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
