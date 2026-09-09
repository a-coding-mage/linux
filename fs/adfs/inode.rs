// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/adfs/inode.c
 *
 *  Copyright (C) 1997-1999 Russell King
 */
// Linux kernel dependencies are supplied by other translated files.

/*
 * Lookup/Create a block at offset 'block' into 'inode'.  We currently do
 * not support creation of new blocks, so we return -EIO for this case.
 */
unsafe fn adfs_get_block(inode: *mut inode, mut block: sector_t,
                         bh: *mut buffer_head, create: c_int) -> c_int {
    if create == 0 {
        if block >= (*inode).i_blocks {
            return 0;
        }

        block = __adfs_block_map((*inode).i_sb, (*ADFS_I(inode)).indaddr, block);
        if block != 0 {
            map_bh(bh, (*inode).i_sb, block);
        }
        return 0;
    }
    /* don't support allocation of blocks yet */
    -EIO
}

unsafe fn adfs_writepages(mapping: *mut address_space,
                          wbc: *mut writeback_control) -> c_int {
    mpage_writepages(mapping, wbc, adfs_get_block)
}

unsafe fn adfs_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    block_read_full_folio(folio, adfs_get_block)
}

unsafe fn adfs_write_failed(mapping: *mut address_space, to: loff_t) {
    let inode = (*mapping).host;
    if to > (*inode).i_size {
        truncate_pagecache(inode, (*inode).i_size);
    }
}

unsafe fn adfs_write_begin(iocb: *const kiocb, mapping: *mut address_space,
                           pos: loff_t, len: c_uint,
                           foliop: *mut *mut folio, fsdata: *mut *mut c_void) -> c_int {
    let ret = cont_write_begin(iocb, mapping, pos, len, foliop, fsdata,
                               adfs_get_block,
                               &mut (*ADFS_I((*mapping).host)).mmu_private);
    if unlikely(ret != 0) {
        adfs_write_failed(mapping, pos + len as loff_t);
    }
    ret
}

unsafe fn _adfs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t {
    generic_block_bmap(mapping, block, adfs_get_block)
}

static adfs_aops: address_space_operations = address_space_operations {
    dirty_folio: Some(block_dirty_folio),
    invalidate_folio: Some(block_invalidate_folio),
    read_folio: Some(adfs_read_folio),
    writepages: Some(adfs_writepages),
    write_begin: Some(adfs_write_begin),
    write_end: Some(generic_write_end),
    migrate_folio: Some(buffer_migrate_folio),
    bmap: Some(_adfs_bmap),
};

/* Convert ADFS attributes and filetype to Linux permission. */
unsafe fn adfs_atts2mode(sb: *mut super_block, inode: *mut inode) -> umode_t {
    let attr = (*ADFS_I(inode)).attr;
    let asb = ADFS_SB(sb);
    if attr & ADFS_NDA_DIRECTORY != 0 {
        let mode = S_IRUGO & (*asb).s_owner_mask;
        return S_IFDIR | S_IXUGO | mode;
    }
    let rmask = match adfs_filetype((*ADFS_I(inode)).loadaddr) {
        0xfc0 => return S_IFLNK | S_IRWXUGO,
        0xfe6 => S_IRUGO | S_IXUGO,
        _ => S_IRUGO,
    };
    let mut mode = S_IFREG;
    if attr & ADFS_NDA_OWNER_READ != 0 { mode |= rmask & (*asb).s_owner_mask; }
    if attr & ADFS_NDA_OWNER_WRITE != 0 { mode |= S_IWUGO & (*asb).s_owner_mask; }
    if attr & ADFS_NDA_PUBLIC_READ != 0 { mode |= rmask & (*asb).s_other_mask; }
    if attr & ADFS_NDA_PUBLIC_WRITE != 0 { mode |= S_IWUGO & (*asb).s_other_mask; }
    mode
}

/* Convert Linux permission to ADFS attribute. */
unsafe fn adfs_mode2atts(sb: *mut super_block, inode: *mut inode, ia_mode: umode_t) -> c_int {
    let asb = ADFS_SB(sb);
    if S_ISLNK((*inode).i_mode) { return (*ADFS_I(inode)).attr; }
    if S_ISDIR((*inode).i_mode) { return ADFS_NDA_DIRECTORY; }
    let mut attr = 0;
    let mut mode = ia_mode & (*asb).s_owner_mask;
    if mode & S_IRUGO != 0 { attr |= ADFS_NDA_OWNER_READ; }
    if mode & S_IWUGO != 0 { attr |= ADFS_NDA_OWNER_WRITE; }
    mode = ia_mode & (*asb).s_other_mask;
    mode &= !(*asb).s_owner_mask;
    if mode & S_IRUGO != 0 { attr |= ADFS_NDA_PUBLIC_READ; }
    if mode & S_IWUGO != 0 { attr |= ADFS_NDA_PUBLIC_WRITE; }
    attr
}

static nsec_unix_epoch_diff_risc_os_epoch: s64 = 2208988800000000000;

unsafe fn adfs_adfs2unix_time(tv: *mut timespec64, inode: *mut inode) {
    if !adfs_inode_is_stamped(inode) { *tv = current_time(inode); return; }
    let high = (*ADFS_I(inode)).loadaddr & 0xFF;
    let low = (*ADFS_I(inode)).execaddr;
    let mut nsec = ((((high as s64) << 32) | low as s64) * 10000000);
    if nsec < nsec_unix_epoch_diff_risc_os_epoch { (*tv).tv_sec = 0; (*tv).tv_nsec = 0; return; }
    nsec -= nsec_unix_epoch_diff_risc_os_epoch;
    *tv = ns_to_timespec64(nsec);
}

unsafe fn adfs_unix2adfs_time(inode: *mut inode, ts: *const timespec64) {
    let mut nsec = timespec64_to_ns(ts) + nsec_unix_epoch_diff_risc_os_epoch;
    let mut cs = div_s64(nsec, 10000000);
    cs = clamp_t(cs, 0, 0xffffffffff);
    (*ADFS_I(inode)).loadaddr &= !0xff;
    (*ADFS_I(inode)).loadaddr |= (cs >> 32) & 0xff;
    (*ADFS_I(inode)).execaddr = cs;
}

unsafe fn adfs_iget(sb: *mut super_block, obj: *mut object_info) -> *mut inode {
    let inode = new_inode(sb);
    if inode.is_null() { return core::ptr::null_mut(); }
    (*inode).i_uid = (*ADFS_SB(sb)).s_uid; (*inode).i_gid = (*ADFS_SB(sb)).s_gid;
    (*inode).i_ino = (*obj).indaddr; (*inode).i_size = (*obj).size; set_nlink(inode, 2);
    (*inode).i_blocks = ((*inode).i_size + (*sb).s_blocksize - 1) >> (*sb).s_blocksize_bits;
    (*ADFS_I(inode)).parent_id = (*obj).parent_id; (*ADFS_I(inode)).indaddr = (*obj).indaddr;
    (*ADFS_I(inode)).loadaddr = (*obj).loadaddr; (*ADFS_I(inode)).execaddr = (*obj).execaddr; (*ADFS_I(inode)).attr = (*obj).attr;
    (*inode).i_mode = adfs_atts2mode(sb, inode); let mut ts = timespec64 { tv_sec: 0, tv_nsec: 0 }; adfs_adfs2unix_time(&mut ts, inode);
    inode_set_atime_to_ts(inode, ts); inode_set_mtime_to_ts(inode, ts); inode_set_ctime_to_ts(inode, ts);
    if S_ISDIR((*inode).i_mode) { (*inode).i_op = &adfs_dir_inode_operations; (*inode).i_fop = &adfs_dir_operations; }
    else if S_ISREG((*inode).i_mode) { (*inode).i_op = &adfs_file_inode_operations; (*inode).i_fop = &adfs_file_operations; (*inode).i_mapping.a_ops = &adfs_aops; (*ADFS_I(inode)).mmu_private = (*inode).i_size; }
    inode_fake_hash(inode); inode
}

unsafe fn adfs_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry, attr: *mut iattr) -> c_int {
    let inode = d_inode(dentry); let sb = (*inode).i_sb; let ia_valid = (*attr).ia_valid;
    let mut error = setattr_prepare(&nop_mnt_idmap, dentry, attr);
    if ((ia_valid & ATTR_UID != 0 && !uid_eq((*attr).ia_uid, (*ADFS_SB(sb)).s_uid)) || (ia_valid & ATTR_GID != 0 && !gid_eq((*attr).ia_gid, (*ADFS_SB(sb)).s_gid))) { error = -EPERM; }
    if error != 0 { return error; }
    if ia_valid & ATTR_SIZE != 0 { truncate_setsize(inode, (*attr).ia_size); }
    if ia_valid & ATTR_MTIME != 0 && adfs_inode_is_stamped(inode) { adfs_unix2adfs_time(inode, &(*attr).ia_mtime); adfs_adfs2unix_time(&mut (*attr).ia_mtime, inode); inode_set_mtime_to_ts(inode, (*attr).ia_mtime); }
    if ia_valid & ATTR_ATIME != 0 { inode_set_atime_to_ts(inode, (*attr).ia_atime); }
    if ia_valid & ATTR_CTIME != 0 { inode_set_ctime_to_ts(inode, (*attr).ia_ctime); }
    if ia_valid & ATTR_MODE != 0 { (*ADFS_I(inode)).attr = adfs_mode2atts(sb, inode, (*attr).ia_mode); (*inode).i_mode = adfs_atts2mode(sb, inode); }
    if ia_valid & (ATTR_SIZE | ATTR_MTIME | ATTR_MODE) != 0 { mark_inode_dirty(inode); } error
}

unsafe fn adfs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int {
    let sb = (*inode).i_sb; let mut obj = object_info { indaddr: (*ADFS_I(inode)).indaddr, name_len: 0, parent_id: (*ADFS_I(inode)).parent_id, loadaddr: (*ADFS_I(inode)).loadaddr, execaddr: (*ADFS_I(inode)).execaddr, attr: (*ADFS_I(inode)).attr, size: (*inode).i_size };
    adfs_dir_update(sb, &mut obj, (*wbc).sync_mode == WB_SYNC_ALL)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
