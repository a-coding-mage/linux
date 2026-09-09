/*
 *  linux/fs/hfs/mdb.c
 *
 * Copyright (C) 1995-1997  Paul H. Hargrove
 * (C) 2003 Ardis Technologies <roman@ardistech.com>
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * This file contains functions for reading/writing the MDB.
 */

// External kernel, HFS, and B-tree declarations are supplied by other files.

static unsafe fn hfs_get_last_session(
    sb: *mut super_block,
    start: *mut sector_t,
    size: *mut sector_t,
) -> c_int {
    let cdi = disk_to_cdi((*(*sb).s_bdev).bd_disk);

    *start = 0;
    *size = bdev_nr_sectors((*sb).s_bdev);

    if HFS_SB(sb).session >= 0 {
        let mut te: cdrom_tocentry = core::mem::zeroed();
        if cdi.is_null() {
            return -EINVAL;
        }
        te.cdte_track = HFS_SB(sb).session;
        te.cdte_format = CDROM_LBA;
        if cdrom_read_tocentry(cdi, &mut te) != 0
            || (te.cdte_ctrl & CDROM_DATA_TRACK) != 4
        {
            pr_err!("invalid session number or type of track\n");
            return -EINVAL;
        }
        *start = (te.cdte_addr.lba as sector_t) << 2;
    } else if !cdi.is_null() {
        let mut ms_info: cdrom_multisession = core::mem::zeroed();
        ms_info.addr_format = CDROM_LBA;
        if cdrom_multisession(cdi, &mut ms_info) == 0 && ms_info.xa_flag != 0 {
            *start = (ms_info.addr.lba as sector_t) << 2;
        }
    }
    0
}

pub unsafe fn is_hfs_cnid_counts_valid(sb: *mut super_block) -> bool {
    let sbi = HFS_SB(sb);
    let mut corrupted = false;
    if atomic64_read(&sbi.next_id) > U32_MAX as i64 {
        pr_warn!("next CNID exceeds limit\n");
        corrupted = true;
    }
    if atomic64_read(&sbi.file_count) > U32_MAX as i64 {
        pr_warn!("file count exceeds limit\n");
        corrupted = true;
    }
    if atomic64_read(&sbi.folder_count) > U32_MAX as i64 {
        pr_warn!("folder count exceeds limit\n");
        corrupted = true;
    }
    !corrupted
}

static unsafe fn hfs_sect_offset(sb: *mut super_block, sec: sector_t) -> c_uint {
    let start = (sec as loff_t) << HFS_SECTOR_SIZE_BITS;
    (start & ((*sb).s_blocksize - 1) as loff_t) as c_uint
}

static unsafe fn hfs_mdb_publish(sbi: *mut hfs_sb_info) {
    lock_buffer((*sbi).mdb_bh);
    memcpy(
        (*(*sbi).mdb_bh).b_data.add((*sbi).mdb_offset as usize),
        (*sbi).mdb as *const c_void,
        HFS_SECTOR_SIZE,
    );
    mark_buffer_dirty((*sbi).mdb_bh);
    unlock_buffer((*sbi).mdb_bh);
}

static unsafe fn hfs_alt_mdb_publish(sbi: *mut hfs_sb_info) {
    lock_buffer((*sbi).alt_mdb_bh);
    memcpy(
        (*(*sbi).alt_mdb_bh).b_data.add((*sbi).alt_mdb_offset as usize),
        (*sbi).alt_mdb as *const c_void,
        HFS_SECTOR_SIZE,
    );
    mark_buffer_dirty((*sbi).alt_mdb_bh);
    unlock_buffer((*sbi).alt_mdb_bh);
}

pub unsafe fn hfs_mdb_get(sb: *mut super_block) -> c_int {
    let mut bh: *mut buffer_head;
    let mut mdb: *mut hfs_mdb = core::ptr::null_mut();
    let mut alt_mdb: *mut hfs_mdb = core::ptr::null_mut();
    let mut block: c_uint;
    let mut ptr: *mut c_char;
    let (mut off2, mut len, mut size, mut sect): (c_int, c_int, c_int, c_int);
    let (mut part_start, mut part_size): (sector_t, sector_t) = (0, 0);
    let mut off: loff_t;
    let mut attrib: __be16;

    size = sb_min_blocksize(sb, HFS_SECTOR_SIZE);
    if size == 0 { return -EINVAL; }
    if hfs_get_last_session(sb, &mut part_start, &mut part_size) != 0 { return -EINVAL; }
    loop {
        bh = sb_bread512(sb, part_start + HFS_MDB_BLK, &mut mdb);
        if bh.is_null() { return -EIO; }
        if (*mdb).drSigWord == cpu_to_be16(HFS_SUPER_MAGIC) { break; }
        brelse(bh);
        if hfs_part_find(sb, &mut part_start, &mut part_size) != 0 { return -EIO; }
    }
    HFS_SB(sb).alloc_blksz = be32_to_cpu((*mdb).drAlBlkSiz);
    size = HFS_SB(sb).alloc_blksz as c_int;
    if size == 0 || (size & (HFS_SECTOR_SIZE - 1)) != 0 {
        pr_err!("bad allocation block size %d\n", size); brelse(bh); return -EIO;
    }
    size = core::cmp::min(HFS_SB(sb).alloc_blksz, PAGE_SIZE as u32) as c_int;
    while (size & (size - 1)) != 0 { size -= HFS_SECTOR_SIZE; }
    sect = be16_to_cpu((*mdb).drAlBlSt) as c_int + part_start as c_int;
    while (sect & ((size - 1) >> HFS_SECTOR_SIZE_BITS)) != 0 { size >>= 1; }
    while (HFS_SB(sb).alloc_blksz & (size as u32 - 1)) != 0 { size >>= 1; }
    brelse(bh);
    if sb_set_blocksize(sb, size) == 0 { pr_err!("unable to set blocksize to %u\n", size); return -EIO; }
    bh = sb_bread512(sb, part_start + HFS_MDB_BLK, &mut mdb);
    if bh.is_null() || (*mdb).drSigWord != cpu_to_be16(HFS_SUPER_MAGIC) {
        if !bh.is_null() { brelse(bh); } return -EIO;
    }
    mdb = kmemdup(mdb as *const c_void, HFS_SECTOR_SIZE, GFP_KERNEL) as *mut hfs_mdb;
    if mdb.is_null() { brelse(bh); return -ENOMEM; }
    HFS_SB(sb).mdb_bh = bh; HFS_SB(sb).mdb_offset = hfs_sect_offset(sb, part_start + HFS_MDB_BLK); HFS_SB(sb).mdb = mdb;
    HFS_SB(sb).part_start = part_start;
    HFS_SB(sb).fs_ablocks = be16_to_cpu((*mdb).drNmAlBlks);
    HFS_SB(sb).fs_div = HFS_SB(sb).alloc_blksz >> (*sb).s_blocksize_bits;
    HFS_SB(sb).clumpablks = be32_to_cpu((*mdb).drClpSiz) / HFS_SB(sb).alloc_blksz;
    if HFS_SB(sb).clumpablks == 0 { HFS_SB(sb).clumpablks = 1; }
    HFS_SB(sb).fs_start = (be16_to_cpu((*mdb).drAlBlSt) as u32 + part_start as u32) >> ((*sb).s_blocksize_bits - HFS_SECTOR_SIZE_BITS);
    HFS_SB(sb).free_ablocks = be16_to_cpu((*mdb).drFreeBks);
    atomic64_set(&HFS_SB(sb).next_id, be32_to_cpu((*mdb).drNxtCNID) as i64);
    HFS_SB(sb).root_files = be16_to_cpu((*mdb).drNmFls); HFS_SB(sb).root_dirs = be16_to_cpu((*mdb).drNmRtDirs);
    atomic64_set(&HFS_SB(sb).file_count, be32_to_cpu((*mdb).drFilCnt) as i64); atomic64_set(&HFS_SB(sb).folder_count, be32_to_cpu((*mdb).drDirCnt) as i64);
    if !is_hfs_cnid_counts_valid(sb) { pr_warn!("filesystem possibly corrupted, running fsck.hfs is recommended. Mounting read-only.\n"); (*sb).s_flags |= SB_RDONLY; }
    sect = (part_start + part_size - 2) as c_int;
    bh = sb_bread512(sb, sect as sector_t, &mut alt_mdb);
    if !bh.is_null() {
        if (*alt_mdb).drSigWord == cpu_to_be16(HFS_SUPER_MAGIC) {
            alt_mdb = kmemdup(alt_mdb as *const c_void, HFS_SECTOR_SIZE, GFP_KERNEL) as *mut hfs_mdb;
            if !alt_mdb.is_null() { HFS_SB(sb).alt_mdb_bh = bh; HFS_SB(sb).alt_mdb_offset = hfs_sect_offset(sb, sect as sector_t); HFS_SB(sb).alt_mdb = alt_mdb; } else { brelse(bh); }
        } else { brelse(bh); }
    }
    if HFS_SB(sb).alt_mdb.is_null() { pr_warn!("unable to locate alternate MDB\n"); pr_warn!("continuing without an alternate MDB\n"); }
    HFS_SB(sb).bitmap = kzalloc(8192, GFP_KERNEL);
    if HFS_SB(sb).bitmap.is_null() { return -EIO; }
    block = be16_to_cpu((*mdb).drVBMSt) as u32 + part_start as u32; off = (block as loff_t) << HFS_SECTOR_SIZE_BITS;
    size = ((HFS_SB(sb).fs_ablocks + 8) / 8) as c_int; ptr = HFS_SB(sb).bitmap as *mut c_char;
    while size != 0 { bh = sb_bread(sb, off >> (*sb).s_blocksize_bits); if bh.is_null() { pr_err!("unable to read volume bitmap\n"); return -EIO; } off2 = (off & ((*sb).s_blocksize - 1) as loff_t) as c_int; len = core::cmp::min((*sb).s_blocksize as c_int - off2, size); memcpy(ptr as *mut c_void, (*bh).b_data.add(off2 as usize) as *const c_void, len as usize); brelse(bh); ptr = ptr.add(len as usize); off += len as loff_t; size -= len; }
    HFS_SB(sb).ext_tree = hfs_btree_open(sb, HFS_EXT_CNID, hfs_ext_keycmp); if HFS_SB(sb).ext_tree.is_null() { pr_err!("unable to open extent tree\n"); return -EIO; }
    HFS_SB(sb).cat_tree = hfs_btree_open(sb, HFS_CAT_CNID, hfs_cat_keycmp); if HFS_SB(sb).cat_tree.is_null() { pr_err!("unable to open catalog tree\n"); return -EIO; }
    attrib = (*mdb).drAtrb;
    if (attrib & cpu_to_be16(HFS_SB_ATTRIB_UNMNT)) == 0 { pr_warn!("filesystem was not cleanly unmounted, running fsck.hfs is recommended.\tMounting read-only.\n"); (*sb).s_flags |= SB_RDONLY; }
    if (attrib & cpu_to_be16(HFS_SB_ATTRIB_SLOCK)) != 0 { pr_warn!("filesystem is marked locked, mounting read-only.\n"); (*sb).s_flags |= SB_RDONLY; }
    if !sb_rdonly(sb) { attrib &= cpu_to_be16(!HFS_SB_ATTRIB_UNMNT); attrib |= cpu_to_be16(HFS_SB_ATTRIB_INCNSTNT); (*mdb).drAtrb = attrib; be32_add_cpu(&mut (*mdb).drWrCnt, 1); (*mdb).drLsMod = hfs_mtime(); hfs_mdb_publish(HFS_SB(sb)); sync_dirty_buffer(HFS_SB(sb).mdb_bh); }
    0
}

pub unsafe fn hfs_mdb_commit(sb: *mut super_block) -> c_int {
    let mdb = HFS_SB(sb).mdb; let mut ret = 0;
    if sb_rdonly(sb) { return 0; }
    if !buffer_uptodate(HFS_SB(sb).mdb_bh) { pr_err!("primary MDB is corrupt, mounting read-only\n"); (*sb).s_flags |= SB_RDONLY; return -EIO; }
    if test_and_clear_bit(HFS_FLG_MDB_DIRTY, &mut HFS_SB(sb).flags) {
        (*mdb).drLsMod = hfs_mtime(); (*mdb).drFreeBks = cpu_to_be16(HFS_SB(sb).free_ablocks); (*mdb).drNxtCNID = cpu_to_be32(atomic64_read(&HFS_SB(sb).next_id) as u32); (*mdb).drNmFls = cpu_to_be16(HFS_SB(sb).root_files); (*mdb).drNmRtDirs = cpu_to_be16(HFS_SB(sb).root_dirs); (*mdb).drFilCnt = cpu_to_be32(atomic64_read(&HFS_SB(sb).file_count) as u32); (*mdb).drDirCnt = cpu_to_be32(atomic64_read(&HFS_SB(sb).folder_count) as u32);
        hfs_inode_write_fork((*HFS_SB(sb).ext_tree).inode, (*mdb).drXTExtRec, &mut (*mdb).drXTFlSize, core::ptr::null_mut()); hfs_inode_write_fork((*HFS_SB(sb).cat_tree).inode, (*mdb).drCTExtRec, &mut (*mdb).drCTFlSize, core::ptr::null_mut()); hfs_mdb_publish(HFS_SB(sb)); sync_dirty_buffer(HFS_SB(sb).mdb_bh);
    }
    if test_and_clear_bit(HFS_FLG_ALT_MDB_DIRTY, &mut HFS_SB(sb).flags) && !HFS_SB(sb).alt_mdb.is_null() {
        if !buffer_uptodate(HFS_SB(sb).alt_mdb_bh) { pr_err!("alternate MDB is corrupt, mounting read-only\n"); (*sb).s_flags |= SB_RDONLY; ret = -EIO; return ret; }
        memcpy(HFS_SB(sb).alt_mdb as *mut c_void, mdb as *const c_void, HFS_SECTOR_SIZE); (*HFS_SB(sb).alt_mdb).drAtrb |= cpu_to_be16(HFS_SB_ATTRIB_UNMNT); (*HFS_SB(sb).alt_mdb).drAtrb &= cpu_to_be16(!HFS_SB_ATTRIB_INCNSTNT); hfs_alt_mdb_publish(HFS_SB(sb)); sync_dirty_buffer(HFS_SB(sb).alt_mdb_bh);
    }
    if test_and_clear_bit(HFS_FLG_BITMAP_DIRTY, &mut HFS_SB(sb).flags) {
        let mut bh: *mut buffer_head; let mut block: sector_t; let mut ptr: *mut c_char; let (mut off, mut size, mut len): (c_int, c_int, c_int);
        block = be16_to_cpu((*HFS_SB(sb).mdb).drVBMSt) as sector_t + HFS_SB(sb).part_start; off = ((block << HFS_SECTOR_SIZE_BITS) & ((*sb).s_blocksize - 1) as sector_t) as c_int; block >>= (*sb).s_blocksize_bits - HFS_SECTOR_SIZE_BITS; size = ((HFS_SB(sb).fs_ablocks + 7) / 8) as c_int; ptr = HFS_SB(sb).bitmap as *mut c_char;
        while size != 0 { bh = sb_bread(sb, block); if bh.is_null() { pr_err!("unable to read volume bitmap\n"); break; } len = core::cmp::min((*sb).s_blocksize as c_int - off, size); lock_buffer(bh); memcpy((*bh).b_data.add(off as usize) as *mut c_void, ptr as *const c_void, len as usize); unlock_buffer(bh); mark_buffer_dirty(bh); brelse(bh); block += 1; off = 0; ptr = ptr.add(len as usize); size -= len; }
    }
    ret
}

pub unsafe fn hfs_mdb_close(sb: *mut super_block) {
    if sb_rdonly(sb) || !buffer_uptodate(HFS_SB(sb).mdb_bh) { return; }
    (*HFS_SB(sb).mdb).drAtrb |= cpu_to_be16(HFS_SB_ATTRIB_UNMNT); (*HFS_SB(sb).mdb).drAtrb &= cpu_to_be16(!HFS_SB_ATTRIB_INCNSTNT); hfs_mdb_publish(HFS_SB(sb));
}

pub unsafe fn hfs_mdb_put(sb: *mut super_block) {
    hfs_btree_close(HFS_SB(sb).ext_tree); hfs_btree_close(HFS_SB(sb).cat_tree); brelse(HFS_SB(sb).mdb_bh); brelse(HFS_SB(sb).alt_mdb_bh); kfree(HFS_SB(sb).mdb as *mut c_void); kfree(HFS_SB(sb).alt_mdb as *mut c_void); unload_nls(HFS_SB(sb).nls_io); unload_nls(HFS_SB(sb).nls_disk); kfree(HFS_SB(sb).bitmap as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
