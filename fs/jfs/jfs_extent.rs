// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

// Linux/JFS dependencies supplied by the surrounding translation unit.

/* forward references are resolved by the function definitions below. */

macro_rules! DPD { ($a:expr) => { printk!("(a): %d\n", $a) }; }
macro_rules! DPC { ($a:expr) => { printk!("(a): %c\n", $a) }; }
macro_rules! DPL1 { ($a:expr) => {{
    if ($a >> 32) != 0 { printk!("(a): %x%08x  ", $a); }
    else { printk!("(a): %x  ", $a << 32); }
}}; }
macro_rules! DPL { ($a:expr) => {{
    if ($a >> 32) != 0 { printk!("(a): %x%08x\n", $a); }
    else { printk!("(a): %x\n", $a << 32); }
}}; }
macro_rules! DPD1 { ($a:expr) => { printk!("(a): %d  ", $a) }; }
macro_rules! DPX { ($a:expr) => { printk!("(a): %08x\n", $a) }; }
macro_rules! DPX1 { ($a:expr) => { printk!("(a): %08x  ", $a) }; }
macro_rules! DPS { ($a:expr) => { printk!("%s\n", $a) }; }
macro_rules! DPE { ($a:expr) => { printk!("\nENTERING: %s\n", $a) }; }
macro_rules! DPE1 { ($a:expr) => { printk!("\nENTERING: %s", $a) }; }
macro_rules! DPS1 { ($a:expr) => { printk!("  %s  ", $a) }; }

pub unsafe fn extAlloc(ip: *mut inode, mut xlen: i64, pno: i64, xp: *mut xad_t, abnr: bool) -> i32 {
    let sbi = JFS_SBI((*ip).i_sb);
    let mut nxlen: i64;
    let mut nxaddr: i64 = 0;
    let mut xoff: i64;
    let mut hint: i64;
    let mut xaddr: i64 = 0;
    let mut rc: i32;
    let xflag: i32;

    if isReadOnly(ip) {
        jfs_error((*ip).i_sb, "read-only filesystem\n");
        return -EIO;
    }

    txBeginAnon((*ip).i_sb);
    mutex_lock(&mut JFS_IP(ip).commit_mutex);

    if xlen > MAXXLEN { xlen = MAXXLEN; }
    xoff = pno << sbi.l2nbperpage;

    hint = addressXAD(xp);
    if hint != 0 {
        nxlen = lengthXAD(xp);
        if offsetXAD(xp) + nxlen == xoff &&
            abnr == (((*xp).flag & XAD_NOTRECORDED) != 0) {
            xaddr = hint + nxlen;
        }
        hint += nxlen - 1;
    }

    nxlen = xlen;
    rc = extBalloc(ip, if hint != 0 { hint } else { INOHINT(ip) }, &mut nxlen, &mut nxaddr);
    if rc != 0 {
        mutex_unlock(&mut JFS_IP(ip).commit_mutex);
        return rc;
    }

    rc = dquot_alloc_block(ip, nxlen);
    if rc != 0 {
        dbFree(ip, nxaddr, nxlen);
        mutex_unlock(&mut JFS_IP(ip).commit_mutex);
        return rc;
    }

    xflag = if abnr { XAD_NOTRECORDED } else { 0 };
    if xaddr != 0 && xaddr == nxaddr {
        rc = xtExtend(0, ip, xoff, nxlen as i32, 0);
    } else {
        rc = xtInsert(0, ip, xflag, xoff, nxlen as i32, &mut nxaddr, 0);
    }

    if rc != 0 {
        dbFree(ip, nxaddr, nxlen);
        dquot_free_block(ip, nxlen);
        mutex_unlock(&mut JFS_IP(ip).commit_mutex);
        return rc;
    }

    XADaddress(xp, nxaddr);
    XADlength(xp, nxlen);
    XADoffset(xp, xoff);
    (*xp).flag = xflag;
    mark_inode_dirty(ip);
    mutex_unlock(&mut JFS_IP(ip).commit_mutex);

    if test_and_clear_cflag(COMMIT_Synclist, ip) {
        jfs_commit_inode(ip, 0);
    }
    0
}

pub unsafe fn extHint(ip: *mut inode, offset: i64, xp: *mut xad_t) -> i32 {
    let sb = (*ip).i_sb;
    let nbperpage = JFS_SBI(sb).nbperpage;
    let prev = ((offset & !POFFSET) >> JFS_SBI(sb).l2bsize) - nbperpage as i64;
    let mut rc: i32 = 0;
    let mut xaddr: i64 = 0;
    let mut xlen: i32 = 0;
    let mut xflag: i32 = 0;

    XADaddress(xp, 0);
    if prev < 0 { return rc; }
    rc = xtLookup(ip, prev, nbperpage, &mut xflag, &mut xaddr, &mut xlen, 0);
    if rc == 0 && xlen != 0 {
        if xlen != nbperpage {
            jfs_error((*ip).i_sb, "corrupt xtree\n");
            rc = -EIO;
        }
        XADaddress(xp, xaddr);
        XADlength(xp, xlen as i64);
        XADoffset(xp, prev);
        (*xp).flag = xflag & XAD_NOTRECORDED;
    } else { rc = 0; }
    rc
}

pub unsafe fn extRecord(ip: *mut inode, xp: *mut xad_t) -> i32 {
    if isReadOnly(ip) {
        jfs_error((*ip).i_sb, "read-only filesystem\n");
        return -EIO;
    }
    txBeginAnon((*ip).i_sb);
    mutex_lock(&mut JFS_IP(ip).commit_mutex);
    let rc = xtUpdate(0, ip, xp);
    mutex_unlock(&mut JFS_IP(ip).commit_mutex);
    rc
}

unsafe fn extBalloc(ip: *mut inode, hint: i64, nblocks: *mut i64, blkno: *mut i64) -> i32 {
    let ji = JFS_IP(ip);
    let sbi = JFS_SBI((*ip).i_sb);
    let nbperpage = sbi.nbperpage;
    let bmp = sbi.bmap;
    let mut nb: i64;
    let mut nblks: i64;
    let mut daddr: i64 = 0;
    let max: i64;
    let mut rc: i32;

    if bmp.db_maxfreebud == -1 { return -ENOSPC; }
    max = 1i64 << bmp.db_maxfreebud;
    if *nblocks >= max && *nblocks > nbperpage as i64 {
        nb = if max > nbperpage as i64 { max } else { nbperpage as i64 };
        nblks = nb;
    } else { nb = *nblocks; nblks = nb; }

    loop {
        rc = dbAlloc(ip, hint, nb, &mut daddr);
        if rc == 0 { break; }
        if rc != -ENOSPC { return rc; }
        nb = std::cmp::min(nblks, extRoundDown(nb));
        if nb < nbperpage as i64 { return rc; }
    }

    *nblocks = nb;
    *blkno = daddr;
    if S_ISREG((*ip).i_mode) && ji.fileset == FILESYSTEM_I {
        let ag = BLKTOAG(daddr, sbi);
        spin_lock_irq(&mut ji.ag_lock);
        if ji.active_ag == -1 {
            atomic_inc(&mut bmp.db_active[ag]);
            ji.active_ag = ag;
        } else if ji.active_ag != ag {
            atomic_dec(&mut bmp.db_active[ji.active_ag]);
            atomic_inc(&mut bmp.db_active[ag]);
            ji.active_ag = ag;
        }
        spin_unlock_irq(&mut ji.ag_lock);
    }
    0
}

unsafe fn extRoundDown(nb: i64) -> i64 {
    let mut i: i32 = 0;
    let mut m: u64 = 1u64 << 63;
    while i < 64 {
        if (m & nb as u64) != 0 { break; }
        i += 1;
        m >>= 1;
    }
    i = 63 - i;
    let mut k = 1u64 << i;
    k = if ((k - 1) & nb as u64) != 0 { k } else { k >> 1 };
    k as i64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
