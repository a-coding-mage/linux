// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ufs/ialloc.c
 *
 * Copyright (c) 1998
 * Daniel Pirkl <daniel.pirkl@email.cz>
 * Charles University, Faculty of Mathematics and Physics
 *
 *  from linux/fs/ext2/ialloc.c
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  BSD ufs-inspired inode and directory allocation by
 *  Stephen Tweedie (sct@dcs.ed.ac.uk), 1993
 *  Big-endian to little-endian byte-swapping/bitmaps by
 *        David S. Miller (davem@caip.rutgers.edu), 1995
 *
 * UFS2 write support added by
 * Evgeniy Dushistov <dushistov@mail.ru>, 2007
 */

/* Kernel and UFS declarations are supplied by the surrounding translation unit. */

/*
 * NOTE! When we get the inode, we're the only people that have access to it,
 * and as such there are no race conditions we have to worry about. The inode
 * is not on the hash-lists, and it cannot be reached through the filesystem
 * because the directory entry has been deleted earlier.
 *
 * HOWEVER: we must make sure that we get no aliases, which means that we have
 * to call clear_inode() _before_ we mark the inode not in use in the inode
 * bitmaps.
 */
pub unsafe fn ufs_free_inode(inode: *mut inode) {
    let sb: *mut super_block;
    let uspi: *mut ufs_sb_private_info;
    let ucpi: *mut ufs_cg_private_info;
    let ucg: *mut ufs_cylinder_group;
    let is_directory: i32;
    let ino: u32;
    let cg: u32;
    let bit: u32;

    UFSD!("ENTER, ino %llu\n", (*inode).i_ino);
    sb = (*inode).i_sb;
    uspi = (*UFS_SB(sb)).s_uspi;
    ino = (*inode).i_ino as u32;
    mutex_lock(&mut (*UFS_SB(sb)).s_lock);
    if !((ino > 1) && (ino < (*uspi).s_ncg * (*uspi).s_ipg)) {
        ufs_warning(sb, "ufs_free_inode", "reserved inode or nonexistent inode %u\n", ino);
        mutex_unlock(&mut (*UFS_SB(sb)).s_lock);
        return;
    }
    cg = ufs_inotocg(ino);
    bit = ufs_inotocgoff(ino);
    ucpi = ufs_load_cylinder(sb, cg);
    if ucpi.is_null() { mutex_unlock(&mut (*UFS_SB(sb)).s_lock); return; }
    ucg = ubh_get_ucg(UCPI_UBH(ucpi));
    if !ufs_cg_chkmagic(sb, ucg) { ufs_panic(sb, "ufs_free_fragments", "internal error, bad cg magic number"); }
    (*ucg).cg_time = ufs_get_seconds(sb);
    is_directory = if S_ISDIR((*inode).i_mode) { 1 } else { 0 };
    if ubh_isclr(UCPI_UBH(ucpi), (*ucpi).c_iusedoff, bit) {
        ufs_error(sb, "ufs_free_inode", "bit already cleared for inode %u", ino);
    } else {
        ubh_clrbit(UCPI_UBH(ucpi), (*ucpi).c_iusedoff, bit);
        if ino < (*ucpi).c_irotor { (*ucpi).c_irotor = ino; }
        fs32_add(sb, &mut (*ucg).cg_cs.cs_nifree, 1);
        (*uspi).cs_total.cs_nifree -= 1;
        fs32_add(sb, &mut UFS_SB(sb).fs_cs(cg).cs_nifree, 1);
        if is_directory != 0 {
            fs32_sub(sb, &mut (*ucg).cg_cs.cs_ndir, 1);
            (*uspi).cs_total.cs_ndir -= 1;
            fs32_sub(sb, &mut UFS_SB(sb).fs_cs(cg).cs_ndir, 1);
        }
    }
    ubh_mark_buffer_dirty(USPI_UBH(uspi));
    ubh_mark_buffer_dirty(UCPI_UBH(ucpi));
    if (*sb).s_flags & SB_SYNCHRONOUS != 0 { ubh_sync_block(UCPI_UBH(ucpi)); }
    ufs_mark_sb_dirty(sb);
    mutex_unlock(&mut (*UFS_SB(sb)).s_lock);
    UFSD!("EXIT\n");
}

/* Nullify a new chunk of inodes. */
unsafe fn ufs2_init_inodes_chunk(sb: *mut super_block, ucpi: *mut ufs_cg_private_info, ucg: *mut ufs_cylinder_group) {
    let uspi = (*UFS_SB(sb)).s_uspi;
    let mut beg = (*uspi).s_sbbase + ufs_inotofsba((*ucpi).c_cgx * (*uspi).s_ipg + fs32_to_cpu(sb, (*ucg).cg_u.cg_u2.cg_initediblk));
    let end = beg + (*uspi).s_fpb;
    UFSD!("ENTER cgno %d\n", (*ucpi).c_cgx);
    while beg < end {
        let bh = sb_getblk(sb, beg);
        lock_buffer(bh);
        memset((*bh).b_data, 0, (*sb).s_blocksize);
        set_buffer_uptodate(bh);
        mark_buffer_dirty(bh);
        unlock_buffer(bh);
        if (*sb).s_flags & SB_SYNCHRONOUS != 0 { sync_dirty_buffer(bh); }
        brelse(bh);
        beg += 1;
    }
    fs32_add(sb, &mut (*ucg).cg_u.cg_u2.cg_initediblk, (*uspi).s_inopb);
    ubh_mark_buffer_dirty(UCPI_UBH(ucpi));
    if (*sb).s_flags & SB_SYNCHRONOUS != 0 { ubh_sync_block(UCPI_UBH(ucpi)); }
    UFSD!("EXIT\n");
}

pub unsafe fn ufs_new_inode(dir: *mut inode, mode: umode_t) -> *mut inode {
    let mut err: i32 = -ENOSPC;
    if dir.is_null() || (*dir).i_nlink == 0 { return ERR_PTR(-EPERM); }
    let sb = (*dir).i_sb;
    let inode = new_inode(sb);
    if inode.is_null() { return ERR_PTR(-ENOMEM); }
    let ufsi = UFS_I(inode);
    let sbi = UFS_SB(sb);
    let uspi = (*sbi).s_uspi;
    mutex_lock(&mut (*sbi).s_lock);
    let mut i = ufs_inotocg((*dir).i_ino);
    let mut cg;
    if sbi.fs_cs(i).cs_nifree != 0 { cg = i; } else {
        let mut j = 1;
        let mut found = false;
        while j < (*uspi).s_ncg { i += j; if i >= (*uspi).s_ncg { i -= (*uspi).s_ncg; } if sbi.fs_cs(i).cs_nifree != 0 { found = true; break; } j <<= 1; }
        if found { cg = i; } else {
            i = ufs_inotocg((*dir).i_ino) + 1; let mut j = 2; found = false;
            while j < (*uspi).s_ncg { i += 1; if i >= (*uspi).s_ncg { i = 0; } if sbi.fs_cs(i).cs_nifree != 0 { found = true; break; } j += 1; }
            if !found { mutex_unlock(&mut (*sbi).s_lock); make_bad_inode(inode); iput(inode); return ERR_PTR(err); }
            cg = i;
        }
    }
    let ucpi = ufs_load_cylinder(sb, cg);
    if ucpi.is_null() { err = -EIO; mutex_unlock(&mut (*sbi).s_lock); make_bad_inode(inode); iput(inode); return ERR_PTR(err); }
    let ucg = ubh_get_ucg(UCPI_UBH(ucpi));
    if !ufs_cg_chkmagic(sb, ucg) { ufs_panic(sb, "ufs_new_inode", "internal error, bad cg magic number"); }
    let start = (*ucpi).c_irotor;
    let mut bit = ubh_find_next_zero_bit(UCPI_UBH(ucpi), (*ucpi).c_iusedoff, (*uspi).s_ipg, start);
    if bit >= (*uspi).s_ipg { bit = ubh_find_first_zero_bit(UCPI_UBH(ucpi), (*ucpi).c_iusedoff, start); if bit >= start { ufs_error(sb, "ufs_new_inode", "cylinder group %u corrupted - error in inode bitmap\n", cg); err = -EIO; mutex_unlock(&mut (*sbi).s_lock); make_bad_inode(inode); iput(inode); return ERR_PTR(err); } }
    if ubh_isclr(UCPI_UBH(ucpi), (*ucpi).c_iusedoff, bit) { ubh_setbit(UCPI_UBH(ucpi), (*ucpi).c_iusedoff, bit); } else { ufs_panic(sb, "ufs_new_inode", "internal error"); err = -EIO; mutex_unlock(&mut (*sbi).s_lock); make_bad_inode(inode); iput(inode); return ERR_PTR(err); }
    if (*uspi).fs_magic == UFS2_MAGIC { let initediblk = fs32_to_cpu(sb, (*ucg).cg_u.cg_u2.cg_initediblk); if bit + (*uspi).s_inopb > initediblk && initediblk < fs32_to_cpu(sb, (*ucg).cg_u.cg_u2.cg_niblk) { ufs2_init_inodes_chunk(sb, ucpi, ucg); } }
    fs32_sub(sb, &mut (*ucg).cg_cs.cs_nifree, 1); (*uspi).cs_total.cs_nifree -= 1; fs32_sub(sb, &mut sbi.fs_cs(cg).cs_nifree, 1);
    if S_ISDIR(mode) { fs32_add(sb, &mut (*ucg).cg_cs.cs_ndir, 1); (*uspi).cs_total.cs_ndir += 1; fs32_add(sb, &mut sbi.fs_cs(cg).cs_ndir, 1); }
    ubh_mark_buffer_dirty(USPI_UBH(uspi)); ubh_mark_buffer_dirty(UCPI_UBH(ucpi)); if (*sb).s_flags & SB_SYNCHRONOUS != 0 { ubh_sync_block(UCPI_UBH(ucpi)); } ufs_mark_sb_dirty(sb);
    (*inode).i_ino = cg * (*uspi).s_ipg + bit; inode_init_owner(&nop_mnt_idmap, inode, dir, mode); (*inode).i_blocks = 0; (*inode).i_generation = 0; simple_inode_init_ts(inode);
    (*ufsi).i_flags = (*UFS_I(dir)).i_flags; (*ufsi).i_lastfrag = 0; (*ufsi).i_shadow = 0; (*ufsi).i_osync = 0; (*ufsi).i_oeftflag = 0; (*ufsi).i_dir_start_lookup = 0; memset(&mut (*ufsi).i_u1, 0, core::mem::size_of_val(&(*ufsi).i_u1));
    if insert_inode_locked(inode) < 0 { err = -EIO; mutex_unlock(&mut (*sbi).s_lock); make_bad_inode(inode); iput(inode); return ERR_PTR(err); }
    mark_inode_dirty(inode);
    if (*uspi).fs_magic == UFS2_MAGIC { let bh = sb_bread(sb, (*uspi).s_sbbase + ufs_inotofsba((*inode).i_ino)); if bh.is_null() { ufs_warning(sb, "ufs_read_inode", "unable to read inode %llu\n", (*inode).i_ino); err = -EIO; mutex_unlock(&mut (*sbi).s_lock); clear_nlink(inode); discard_new_inode(inode); return ERR_PTR(err); } lock_buffer(bh); let ufs2_inode = ((*bh).b_data as *mut ufs2_inode).add(ufs_inotofsbo((*inode).i_ino) as usize); let mut ts = timespec64 { tv_sec: 0, tv_nsec: 0 }; ktime_get_real_ts64(&mut ts); (*ufs2_inode).ui_birthtime = cpu_to_fs64(sb, ts.tv_sec); (*ufs2_inode).ui_birthnsec = cpu_to_fs32(sb, ts.tv_nsec); mark_buffer_dirty(bh); unlock_buffer(bh); if (*sb).s_flags & SB_SYNCHRONOUS != 0 { sync_dirty_buffer(bh); } brelse(bh); }
    mutex_unlock(&mut (*sbi).s_lock); UFSD!("allocating inode %llu\n", (*inode).i_ino); UFSD!("EXIT\n"); inode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
