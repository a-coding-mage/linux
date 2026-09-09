// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct translation of jfs_mount.c. */

unsafe fn jfs_mount(sb: *mut super_block) -> c_int {
    let mut rc: c_int = 0;
    let sbi = JFS_SBI(sb);
    let mut ipaimap: *mut inode = core::ptr::null_mut();
    let mut ipaimap2: *mut inode = core::ptr::null_mut();
    let mut ipimap: *mut inode = core::ptr::null_mut();
    let mut ipbmap: *mut inode = core::ptr::null_mut();

    rc = chkSuper(sb);
    if rc != 0 { return rc; }

    ipaimap = diReadSpecial(sb, AGGREGATE_I, 0);
    if ipaimap.is_null() { jfs_err!("jfs_mount: Failed to read AGGREGATE_I"); rc = -EIO; return rc; }
    (*sbi).ipaimap = ipaimap;
    jfs_info!("jfs_mount: ipaimap:0x%p", ipaimap);

    rc = diMount(ipaimap);
    if rc != 0 { jfs_err!("jfs_mount: diMount(ipaimap) failed w/rc = %d", rc); diFreeSpecial(ipaimap); return rc; }

    ipbmap = diReadSpecial(sb, BMAP_I, 0);
    if ipbmap.is_null() { rc = -EIO; diUnmount(ipaimap, 1); diFreeSpecial(ipaimap); return rc; }
    jfs_info!("jfs_mount: ipbmap:0x%p", ipbmap);
    (*sbi).ipbmap = ipbmap;
    rc = dbMount(ipbmap);
    if rc != 0 { jfs_err!("jfs_mount: dbMount failed w/rc = %d", rc); diFreeSpecial(ipbmap); diUnmount(ipaimap, 1); diFreeSpecial(ipaimap); return rc; }

    if ((*sbi).mntflag & JFS_BAD_SAIT) == 0 {
        ipaimap2 = diReadSpecial(sb, AGGREGATE_I, 1);
        if ipaimap2.is_null() { jfs_err!("jfs_mount: Failed to read AGGREGATE_I"); rc = -EIO; dbUnmount(ipbmap, 1); diFreeSpecial(ipbmap); diUnmount(ipaimap, 1); diFreeSpecial(ipaimap); return rc; }
        (*sbi).ipaimap2 = ipaimap2;
        jfs_info!("jfs_mount: ipaimap2:0x%p", ipaimap2);
        rc = diMount(ipaimap2);
        if rc != 0 { jfs_err!("jfs_mount: diMount(ipaimap2) failed, rc = %d", rc); diFreeSpecial(ipaimap2); dbUnmount(ipbmap, 1); diFreeSpecial(ipbmap); diUnmount(ipaimap, 1); diFreeSpecial(ipaimap); return rc; }
    } else { (*sbi).ipaimap2 = core::ptr::null_mut(); }

    ipimap = diReadSpecial(sb, FILESYSTEM_I, 0);
    if ipimap.is_null() { jfs_err!("jfs_mount: Failed to read FILESYSTEM_I"); if !ipaimap2.is_null() { diUnmount(ipaimap2, 1); diFreeSpecial(ipaimap2); } dbUnmount(ipbmap, 1); diFreeSpecial(ipbmap); diUnmount(ipaimap, 1); diFreeSpecial(ipaimap); return -EIO; }
    jfs_info!("jfs_mount: ipimap:0x%p", ipimap);
    rc = diMount(ipimap);
    if rc != 0 { jfs_err!("jfs_mount: diMount failed w/rc = %d", rc); diFreeSpecial(ipimap); if !ipaimap2.is_null() { diUnmount(ipaimap2, 1); diFreeSpecial(ipaimap2); } dbUnmount(ipbmap, 1); diFreeSpecial(ipbmap); diUnmount(ipaimap, 1); diFreeSpecial(ipaimap); return rc; }
    (*sbi).ipimap = ipimap;
    rc
}

unsafe fn jfs_mount_rw(sb: *mut super_block, remount: c_int) -> c_int {
    let sbi = JFS_SBI(sb);
    let mut rc: c_int;
    if remount != 0 {
        if chkSuper(sb) != 0 || (*sbi).state != FM_CLEAN { return -EINVAL; }
        truncate_inode_pages((*(*sbi).ipimap).i_mapping, 0);
        truncate_inode_pages((*(*sbi).ipbmap).i_mapping, 0);
        IWRITE_LOCK((*sbi).ipimap, RDWRLOCK_IMAP);
        diUnmount((*sbi).ipimap, 1);
        rc = diMount((*sbi).ipimap);
        if rc != 0 { IWRITE_UNLOCK((*sbi).ipimap); jfs_err!("jfs_mount_rw: diMount failed!"); return rc; }
        IWRITE_UNLOCK((*sbi).ipimap);
        dbUnmount((*sbi).ipbmap, 1);
        rc = dbMount((*sbi).ipbmap);
        if rc != 0 { jfs_err!("jfs_mount_rw: dbMount failed!"); return rc; }
    }
    rc = lmLogOpen(sb); if rc != 0 { return rc; }
    rc = updateSuper(sb, FM_MOUNT);
    if rc != 0 { jfs_err!("jfs_mount: updateSuper failed w/rc = %d", rc); lmLogClose(sb); return rc; }
    logMOUNT(sb); rc
}

unsafe fn chkSuper(sb: *mut super_block) -> c_int {
    let sbi = JFS_SBI(sb); let mut bh: *mut buffer_head; let mut rc: c_int;
    rc = readSuper(sb, &mut bh); if rc != 0 { return rc; }
    let js = (*bh).b_data as *mut jfs_superblock;
    if strncmp((*js).s_magic.as_ptr(), JFS_MAGIC, 4) != 0 || le32_to_cpu((*js).s_version) > JFS_VERSION { rc = -EINVAL; brelse(bh); return rc; }
    let bsize = le32_to_cpu((*js).s_bsize); if bsize != PSIZE { jfs_err!("Only 4K block size supported!"); brelse(bh); return -EINVAL; }
    jfs_info!("superblock: flag:0x%08x state:0x%08x size:0x%Lx", le32_to_cpu((*js).s_flag), le32_to_cpu((*js).s_state), le64_to_cpu((*js).s_size));
    if ((*js).s_flag & cpu_to_le32(JFS_BAD_SAIT)) != cpu_to_le32(JFS_BAD_SAIT) {
        let aim = lengthPXD(&(*js).s_aim2) * bsize; let ait = lengthPXD(&(*js).s_ait2) * bsize;
        let aa = addressPXD(&(*js).s_aim2) * bsize; let ta = addressPXD(&(*js).s_ait2) * bsize; let fa = addressPXD(&(*js).s_fsckpxd) * bsize;
        if aim != 2 * PSIZE || ait != 4 * PSIZE || ta - aa != aim || fa - ta <= ait { (*js).s_flag |= cpu_to_le32(JFS_BAD_SAIT); }
    }
    if ((*js).s_flag & cpu_to_le32(JFS_GROUPCOMMIT)) == 0 { (*js).s_flag |= cpu_to_le32(JFS_GROUPCOMMIT); }
    if (*js).s_state != cpu_to_le32(FM_CLEAN) && !sb_rdonly(sb) { jfs_err!("jfs_mount: Mount Failure: File System Dirty."); brelse(bh); return -EINVAL; }
    (*sbi).state = le32_to_cpu((*js).s_state); (*sbi).mntflag = le32_to_cpu((*js).s_flag); (*sbi).bsize = bsize; (*sbi).l2bsize = le16_to_cpu((*js).s_l2bsize);
    if (*sbi).l2bsize != ilog2(bsize as u32) || (*js).pad != 0 || le32_to_cpu((*js).s_state) > FM_STATE_MAX { jfs_err!("jfs_mount: Mount Failure: superblock is corrupt!"); brelse(bh); return -EINVAL; }
    (*sbi).nbperpage = PSIZE >> (*sbi).l2bsize; (*sbi).l2nbperpage = L2PSIZE - (*sbi).l2bsize; (*sbi).l2niperblk = (*sbi).l2bsize - L2DISIZE; uuid_copy(&mut (*sbi).uuid, &(*js).s_uuid);
    if (*sbi).mntflag & JFS_INLINELOG != 0 { (*sbi).logpxd = (*js).s_logpxd; } else { (*sbi).logdev = new_decode_dev(le32_to_cpu((*js).s_logdev)); uuid_copy(&mut (*sbi).loguuid, &(*js).s_loguuid); }
    (*sbi).fsckpxd = (*js).s_fsckpxd; (*sbi).ait2 = (*js).s_ait2; brelse(bh); 0
}

unsafe fn updateSuper(sb: *mut super_block, mut state: uint) -> c_int {
    let sbi = JFS_SBI(sb); let mut bh: *mut buffer_head; let mut rc: c_int;
    if (*sbi).flag & JFS_NOINTEGRITY != 0 {
        if state == FM_DIRTY { (*sbi).p_state = state; return 0; }
        if state == FM_MOUNT { (*sbi).p_state = (*sbi).state; state = FM_DIRTY; }
        else if state == FM_CLEAN { state = (*sbi).p_state; } else { jfs_err!("updateSuper: bad state"); }
    } else if (*sbi).state == FM_DIRTY { return 0; }
    rc = readSuper(sb, &mut bh); if rc != 0 { return rc; }
    let j_sb = (*bh).b_data as *mut jfs_superblock; (*j_sb).s_state = cpu_to_le32(state); (*sbi).state = state;
    if state == FM_MOUNT { (*j_sb).s_logdev = cpu_to_le32(new_encode_dev(file_bdev((*sbi).log.bdev_file).bd_dev)); (*j_sb).s_logserial = cpu_to_le32((*sbi).log.serial); }
    else if state == FM_CLEAN && (*j_sb).s_flag & cpu_to_le32(JFS_DASD_ENABLED) != 0 { (*j_sb).s_flag |= cpu_to_le32(JFS_DASD_PRIME); }
    mark_buffer_dirty(bh); sync_dirty_buffer(bh); brelse(bh); 0
}

unsafe fn readSuper(sb: *mut super_block, bpp: *mut *mut buffer_head) -> c_int {
    *bpp = sb_bread(sb, SUPER1_OFF >> (*sb).s_blocksize_bits); if !(*bpp).is_null() { return 0; }
    *bpp = sb_bread(sb, SUPER2_OFF >> (*sb).s_blocksize_bits); if !(*bpp).is_null() { return 0; } -EIO
}

unsafe fn logMOUNT(sb: *mut super_block) -> c_int {
    let log = (*JFS_SBI(sb)).log; let mut lrd: lrd = core::mem::zeroed();
    lrd.logtid = 0; lrd.backchain = 0; lrd.r#type = cpu_to_le16(LOG_MOUNT); lrd.length = 0; lrd.aggregate = cpu_to_le32(new_encode_dev((*sb).s_bdev.bd_dev)); lmLog(log, core::ptr::null_mut(), &mut lrd, core::ptr::null_mut()); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
