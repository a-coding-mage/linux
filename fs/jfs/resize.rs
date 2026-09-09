// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

// Linux/JFS headers provide the declarations referenced below.

const BITSPERPAGE: i64 = PSIZE << 3;
const L2MEGABYTE: i32 = 20;
const MEGABYTE: i64 = 1 << L2MEGABYTE;
const MEGABYTE32: i64 = MEGABYTE << 5;

#[inline]
unsafe fn blktodmapn(b: i64) -> i64 {
    (b >> 13) + (b >> 23) + (b >> 33) + 3 + 1
}

/*
 * jfs_extendfs - extend file system.
 *
 * new configuration:
 * 1. set new LogSize as specified or default from new LVSize;
 * 2. compute new FSCKSize from new LVSize;
 * 3. set new FSSize as MIN(FSSize, LVSize-(LogSize+FSCKSize)).
 */
pub unsafe fn jfs_extendfs(sb: *mut super_block, new_lv_size: s64, mut new_log_size: i32) -> i32 {
    let mut rc: i32 = 0;
    let sbi = JFS_SBI(sb);
    let ipbmap = (*sbi).ipbmap;
    let mut ipbmap2: *mut inode;
    let ipimap = (*sbi).ipimap;
    let log = (*sbi).log;
    let bmp = (*sbi).bmap;
    let (mut new_log_address, mut new_fsck_address): (s64, s64);
    let mut new_fsck_size: i32;
    let mut new_map_size: s64 = 0;
    let mut map_size: s64;
    let (mut x_address, mut x_size, mut nblocks, mut xoff, mut xaddr, mut t64): (s64,s64,s64,s64,s64,s64);
    let old_lv_size: s64;
    let new_fs_size: s64;
    let volume_size: s64;
    let (mut new_npages, mut n_pages, mut new_page, mut xlen, mut t32): (i32,i32,i32,i32,i32) = (0,0,0,0,0);
    let mut tid: i32;
    let mut log_formatted = 0;
    let mut iplist: [*mut inode; 1] = [core::ptr::null_mut()];
    let (mut j_sb, mut j_sb2): (*mut jfs_superblock, *mut jfs_superblock);
    let mut old_agsize: s64;
    let mut agsizechanged = 0;
    let (mut bh, mut bh2): (*mut buffer_head, *mut buffer_head) = (core::ptr::null_mut(), core::ptr::null_mut());

    if (*sbi).mntflag & JFS_INLINELOG != 0 {
        old_lv_size = addressPXD(&(*sbi).logpxd) + lengthPXD(&(*sbi).logpxd);
    } else {
        old_lv_size = addressPXD(&(*sbi).fsckpxd) + lengthPXD(&(*sbi).fsckpxd);
    }
    if old_lv_size >= new_lv_size {
        printk(KERN_WARNING, b"jfs_extendfs: volume hasn't grown, returning\n\0".as_ptr());
        return rc;
    }
    volume_size = sb_bdev_nr_blocks(sb);
    if volume_size != 0 {
        if new_lv_size > volume_size { rc = -EINVAL; goto out; }
    } else {
        bh = sb_bread(sb, new_lv_size - 1);
        if bh.is_null() { rc = -EINVAL; goto out; }
        bforget(bh);
    }
    if isReadOnly(ipbmap) { rc = -EROFS; goto out; }

    if (*sbi).mntflag & JFS_INLINELOG != 0 {
        if new_log_size == 0 {
            new_log_size = (new_lv_size >> 8) as i32;
            t32 = ((1_i32 << (20 - (*sbi).l2bsize)) - 1);
            new_log_size = (new_log_size + t32) & !t32;
            new_log_size = core::cmp::min(new_log_size, (MEGABYTE32 >> (*sbi).l2bsize) as i32);
        } else {
            new_log_size = (((new_log_size as i64) * MEGABYTE) >> (*sbi).l2bsize) as i32;
        }
    } else { new_log_size = 0; }
    new_log_address = new_lv_size - new_log_size as i64;
    t64 = ((new_lv_size - new_log_size as i64 + BPERDMAP - 1) >> L2BPERDMAP) << L2BPERDMAP;
    t32 = div_round_up(t64, BITSPERPAGE) as i32 + 1 + 50;
    new_fsck_size = t32 << (*sbi).l2nbperpage;
    new_fsck_address = new_log_address - new_fsck_size as i64;
    new_fs_size = new_lv_size - new_log_size as i64 - new_fsck_size as i64;
    if new_fs_size < (*bmp).db_mapsize { rc = -EINVAL; goto out; }

    if (*sbi).mntflag & JFS_INLINELOG != 0 && new_log_address > old_lv_size {
        rc = lmLogFormat(log, new_log_address, new_log_size); if rc != 0 { goto out; }
        log_formatted = 1;
    }
    txQuiesce(sb);
    (*(*sbi).direct_inode).i_size = bdev_nr_bytes((*sb).s_bdev);

    if (*sbi).mntflag & JFS_INLINELOG != 0 {
        lmLogShutdown(log);
        rc = readSuper(sb, &mut bh); if rc != 0 { goto error_out; }
        j_sb = bh.cast::<jfs_superblock>();
        (*j_sb).s_state |= cpu_to_le32(FM_EXTENDFS);
        (*j_sb).s_xsize = cpu_to_le64(new_fs_size);
        PXDaddress(&mut (*j_sb).s_xfsckpxd, new_fsck_address); PXDlength(&mut (*j_sb).s_xfsckpxd, new_fsck_size);
        PXDaddress(&mut (*j_sb).s_xlogpxd, new_log_address); PXDlength(&mut (*j_sb).s_xlogpxd, new_log_size);
        mark_buffer_dirty(bh); sync_dirty_buffer(bh); brelse(bh);
        if log_formatted == 0 { rc = lmLogFormat(log, new_log_address, new_log_size); if rc != 0 { goto error_out; } }
        (*log).base = new_log_address;
        (*log).size = new_log_size >> (L2LOGPSIZE - (*sb).s_blocksize_bits);
        rc = lmLogInit(log); if rc != 0 { goto error_out; }
    }

    new_map_size = new_fs_size;
    t64 = new_map_size - 1 + BPERDMAP;
    new_npages = (blktodmapn(t64) + 1) as i32;

extend_bmap:
    map_size = (*bmp).db_mapsize; x_address = map_size; x_size = new_map_size - map_size; old_agsize = (*bmp).db_agsize;
    t64 = dbMapFileSizeToMapSize(ipbmap);
    if map_size > t64 { rc = -EIO; goto error_out; }
    nblocks = core::cmp::min(t64 - map_size, x_size);
    rc = dbExtendFS(ipbmap, x_address, nblocks); if rc != 0 { goto error_out; }
    if (*bmp).db_agsize != old_agsize { agsizechanged |= 1; }
    x_size -= nblocks;
    n_pages = ((*ipbmap).i_size >> L2PSIZE) as i32;
    if n_pages != new_npages {
        rc = filemap_fdatawait((*ipbmap).i_mapping); if rc != 0 { goto error_out; }
        rc = filemap_write_and_wait((*ipbmap).i_mapping); if rc != 0 { goto error_out; }
        diWriteSpecial(ipbmap, 0);
        new_page = n_pages; xoff = (new_page as i64) << (*sbi).l2nbperpage;
        xlen = (new_npages - n_pages) << (*sbi).l2nbperpage;
        xlen = core::cmp::min(xlen, nblocks as i32) & !((*sbi).nbperpage - 1); xaddr = x_address;
        tid = txBegin(sb, COMMIT_FORCE);
        rc = xtAppend(tid, ipbmap, 0, xoff, nblocks, &mut xlen, &mut xaddr, 0);
        if rc != 0 { txEnd(tid); goto error_out; }
        (*ipbmap).i_size += (xlen as i64) << (*sbi).l2bsize; inode_add_bytes(ipbmap, (xlen as i64) << (*sbi).l2bsize);
        iplist[0] = ipbmap; rc = txCommit(tid, 1, iplist.as_mut_ptr(), COMMIT_FORCE); txEnd(tid);
        if rc != 0 { goto error_out; }
        if x_size != 0 { goto extend_bmap; }
    }
    dbFinalizeBmap(ipbmap);
    if agsizechanged != 0 { rc = diExtendFS(ipimap, ipbmap); if rc != 0 { goto error_out; } rc = diSync(ipimap); if rc != 0 { goto error_out; } }
    rc = dbSync(ipbmap); if rc != 0 { goto error_out; }
    ipbmap2 = diReadSpecial(sb, BMAP_I, 1); if ipbmap2.is_null() { goto error_out; }
    core::ptr::copy_nonoverlapping(&(*JFS_IP(ipbmap)).i_xtroot, &mut (*JFS_IP(ipbmap2)).i_xtroot, 288);
    (*ipbmap2).i_size = (*ipbmap).i_size; (*ipbmap2).i_blocks = (*ipbmap).i_blocks; diWriteSpecial(ipbmap2, 1); diFreeSpecial(ipbmap2);
    rc = readSuper(sb, &mut bh); if rc != 0 { goto error_out; }
    j_sb = bh.cast::<jfs_superblock>(); (*j_sb).s_state &= cpu_to_le32(!FM_EXTENDFS);
    (*j_sb).s_size = cpu_to_le64((*bmp).db_mapsize << le16_to_cpu((*j_sb).s_l2bfactor)); (*j_sb).s_agsize = cpu_to_le32((*bmp).db_agsize);
    if (*sbi).mntflag & JFS_INLINELOG != 0 { PXDaddress(&mut (*j_sb).s_logpxd, new_log_address); PXDlength(&mut (*j_sb).s_logpxd, new_log_size); }
    (*j_sb).s_logserial = cpu_to_le32((*log).serial); PXDaddress(&mut (*j_sb).s_fsckpxd, new_fsck_address); PXDlength(&mut (*j_sb).s_fsckpxd, new_fsck_size); (*j_sb).s_fscklog = 1;
    bh2 = sb_bread(sb, SUPER2_OFF >> (*sb).s_blocksize_bits); if !bh2.is_null() { j_sb2 = bh2.cast(); core::ptr::copy_nonoverlapping(j_sb, j_sb2, core::mem::size_of::<jfs_superblock>()); mark_buffer_dirty(bh2); sync_dirty_buffer(bh2); brelse(bh2); }
    mark_buffer_dirty(bh); sync_dirty_buffer(bh); brelse(bh); goto resume;
error_out:
    jfs_error(sb, b"\n\0".as_ptr());
resume:
    txResume(sb);
out:
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
