// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/affs/amigaffs.c
 *
 *  (c) 1996  Hans-Joachim Widmaier - Rewritten
 *
 *  (C) 1993  Ray Burr - Amiga FFS filesystem.
 *
 *  Please send bug reports to: hjw@zvw.de
 */

// Dependencies supplied by the surrounding kernel/filesystem translation.

/* Functions for accessing Amiga-FFS structures. */

pub unsafe fn affs_insert_hash(dir: *mut inode, bh: *mut buffer_head) -> i32 {
    let sb = (*dir).i_sb;
    let mut dir_bh: *mut buffer_head;
    let ino: u32 = (*bh).b_blocknr;
    let hash_ino: u32;
    let offset: i32;

    offset = affs_hash_name(sb, unsafe { AFFS_TAIL(sb, bh).as_ref().unwrap().name.as_ptr().add(1) }, unsafe { AFFS_TAIL(sb, bh).as_ref().unwrap().name[0] });
    pr_debug!("%s(dir=%llu, ino=%d)\n", __func__, (*dir).i_ino, ino);

    dir_bh = affs_bread(sb, (*dir).i_ino);
    if dir_bh.is_null() { return -EIO; }
    hash_ino = be32_to_cpu((*AFFS_HEAD(dir_bh)).table[offset as usize]);
    let mut hash_ino = hash_ino;
    while hash_ino != 0 {
        affs_brelse(dir_bh);
        dir_bh = affs_bread(sb, hash_ino);
        if dir_bh.is_null() { return -EIO; }
        hash_ino = be32_to_cpu((*AFFS_TAIL(sb, dir_bh)).hash_chain);
    }
    (*AFFS_TAIL(sb, bh)).parent = cpu_to_be32((*dir).i_ino);
    (*AFFS_TAIL(sb, bh)).hash_chain = 0;
    affs_fix_checksum(sb, bh);
    if (*dir).i_ino == (*dir_bh).b_blocknr {
        (*AFFS_HEAD(dir_bh)).table[offset as usize] = cpu_to_be32(ino);
    } else {
        (*AFFS_TAIL(sb, dir_bh)).hash_chain = cpu_to_be32(ino);
    }
    affs_adjust_checksum(dir_bh, ino);
    mark_buffer_dirty(dir_bh);
    affs_brelse(dir_bh);
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    inode_inc_iversion(dir);
    mark_inode_dirty(dir);
    0
}

pub unsafe fn affs_remove_hash(dir: *mut inode, rem_bh: *mut buffer_head) -> i32 {
    let sb = (*dir).i_sb;
    let rem_ino = (*rem_bh).b_blocknr;
    let offset = affs_hash_name(sb, (*AFFS_TAIL(sb, rem_bh)).name.as_ptr().add(1), (*AFFS_TAIL(sb, rem_bh)).name[0]);
    pr_debug!("%s(dir=%llu, ino=%d, hashval=%d)\n", __func__, (*dir).i_ino, rem_ino, offset);
    let mut bh = affs_bread(sb, (*dir).i_ino);
    if bh.is_null() { return -EIO; }
    let mut retval = -ENOENT;
    let mut hash_ino = be32_to_cpu((*AFFS_HEAD(bh)).table[offset as usize]);
    while hash_ino != 0 {
        if hash_ino == rem_ino {
            let ino = (*AFFS_TAIL(sb, rem_bh)).hash_chain;
            if (*dir).i_ino == (*bh).b_blocknr { (*AFFS_HEAD(bh)).table[offset as usize] = ino; }
            else { (*AFFS_TAIL(sb, bh)).hash_chain = ino; }
            affs_adjust_checksum(bh, be32_to_cpu(ino).wrapping_sub(hash_ino));
            mark_buffer_dirty(bh);
            (*AFFS_TAIL(sb, rem_bh)).parent = 0;
            retval = 0;
            break;
        }
        affs_brelse(bh);
        bh = affs_bread(sb, hash_ino);
        if bh.is_null() { return -EIO; }
        hash_ino = be32_to_cpu((*AFFS_TAIL(sb, bh)).hash_chain);
    }
    affs_brelse(bh);
    inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir));
    inode_inc_iversion(dir);
    mark_inode_dirty(dir);
    retval
}

unsafe fn affs_fix_dcache(inode: *mut inode, entry_ino: u32) {
    spin_lock(&mut (*inode).i_lock);
    for_each_alias!(dentry, inode) {
        if entry_ino == (*dentry).d_fsdata as usize as u32 {
            (*dentry).d_fsdata = (*inode).i_ino as usize as *mut core::ffi::c_void;
            break;
        }
    }
    spin_unlock(&mut (*inode).i_lock);
}

unsafe fn affs_remove_link(dentry: *mut dentry) -> i32 {
    let inode = d_inode(dentry);
    let sb = (*inode).i_sb;
    let mut bh = affs_bread(sb, (*inode).i_ino);
    let mut link_bh: *mut buffer_head = core::ptr::null_mut();
    let mut retval = -EIO;
    if bh.is_null() { return retval; }
    let mut link_ino = (*dentry).d_fsdata as usize as u32;
    if (*inode).i_ino == link_ino {
        link_ino = be32_to_cpu((*AFFS_TAIL(sb, bh)).link_chain);
        link_bh = affs_bread(sb, link_ino);
        if link_bh.is_null() { goto_done!(link_bh, bh, retval); }
        let dir = affs_iget(sb, be32_to_cpu((*AFFS_TAIL(sb, link_bh)).parent));
        if IS_ERR(dir) { retval = PTR_ERR(dir); goto_done!(link_bh, bh, retval); }
        affs_lock_dir(dir);
        affs_fix_dcache(inode, link_ino);
        retval = affs_remove_hash(dir, link_bh);
        if retval != 0 { affs_unlock_dir(dir); goto_done!(link_bh, bh, retval); }
        mark_buffer_dirty(link_bh);
        core::ptr::copy_nonoverlapping((*AFFS_TAIL(sb, link_bh)).name.as_ptr(), (*AFFS_TAIL(sb, bh)).name.as_mut_ptr(), 32);
        retval = affs_insert_hash(dir, bh);
        if retval != 0 { affs_unlock_dir(dir); goto_done!(link_bh, bh, retval); }
        mark_buffer_dirty(bh);
        affs_unlock_dir(dir); iput(dir);
    } else {
        link_bh = affs_bread(sb, link_ino);
        if link_bh.is_null() { goto_done!(link_bh, bh, retval); }
    }
    loop {
        let ino = be32_to_cpu((*AFFS_TAIL(sb, bh)).link_chain);
        if ino == 0 { retval = -ENOENT; break; }
        if ino == link_ino {
            let ino2 = (*AFFS_TAIL(sb, link_bh)).link_chain;
            (*AFFS_TAIL(sb, bh)).link_chain = ino2;
            affs_adjust_checksum(bh, be32_to_cpu(ino2).wrapping_sub(link_ino));
            mark_buffer_dirty(bh); retval = 0;
            match be32_to_cpu((*AFFS_TAIL(sb, bh)).stype) {
                ST_LINKDIR | ST_LINKFILE => {}
                _ => if (*AFFS_TAIL(sb, bh)).link_chain == 0 { set_nlink(inode, 1); }
            }
            affs_free_block(sb, link_ino); break;
        }
        affs_brelse(bh); bh = affs_bread(sb, ino);
        if bh.is_null() { break; }
    }
goto_done!(link_bh, bh, retval)
}

unsafe fn affs_empty_dir(inode: *mut inode) -> i32 {
    let sb = (*inode).i_sb;
    let bh = affs_bread(sb, (*inode).i_ino);
    if bh.is_null() { return -EIO; }
    let mut retval = -ENOTEMPTY;
    let mut size = (*AFFS_SB(sb)).s_hashsize - 1;
    while size >= 0 { if (*AFFS_HEAD(bh)).table[size as usize] != 0 { affs_brelse(bh); return retval; } size -= 1; }
    retval = 0; affs_brelse(bh); retval
}

pub unsafe fn affs_checksum_block(sb: *mut super_block, bh: *mut buffer_head) -> u32 {
    let mut ptr = (*bh).b_data as *mut u32;
    let mut sum = 0u32;
    let mut n = (*sb).s_blocksize / core::mem::size_of::<u32>();
    while n > 0 { sum = sum.wrapping_add(be32_to_cpu(*ptr)); ptr = ptr.add(1); n -= 1; }
    sum
}

pub unsafe fn affs_fix_checksum(sb: *mut super_block, bh: *mut buffer_head) {
    let mut ptr = (*bh).b_data as *mut u32;
    let mut cnt = (*sb).s_blocksize / core::mem::size_of::<u32>();
    let check = ptr.add(5); *check = 0;
    let mut checksum = 0u32;
    while cnt > 0 { checksum = checksum.wrapping_add(be32_to_cpu(*ptr)); ptr = ptr.add(1); cnt -= 1; }
    *check = cpu_to_be32(0u32.wrapping_sub(checksum));
}

pub unsafe fn affs_secs_to_datestamp(mut secs: time64_t, ds: *mut affs_date) {
    secs -= sys_tz.tz_minuteswest as i64 * 60 + AFFS_EPOCH_DELTA;
    if secs < 0 { secs = 0; }
    let days = secs / 86400; let mut rem = secs % 86400;
    let minute = rem / 60; rem -= minute * 60;
    (*ds).days = cpu_to_be32(days as u32); (*ds).mins = cpu_to_be32(minute as u32); (*ds).ticks = cpu_to_be32((rem * 50) as u32);
}

pub fn affs_prot_to_mode(prot: u32) -> umode_t {
    let mut mode = 0;
    if prot & FIBF_NOWRITE == 0 { mode |= 0o200; } if prot & FIBF_NOREAD == 0 { mode |= 0o400; } if prot & FIBF_NOEXECUTE == 0 { mode |= 0o100; }
    if prot & FIBF_GRP_WRITE != 0 { mode |= 0o020; } if prot & FIBF_GRP_READ != 0 { mode |= 0o040; } if prot & FIBF_GRP_EXECUTE != 0 { mode |= 0o010; }
    if prot & FIBF_OTR_WRITE != 0 { mode |= 0o002; } if prot & FIBF_OTR_READ != 0 { mode |= 0o004; } if prot & FIBF_OTR_EXECUTE != 0 { mode |= 0o001; } mode
}

pub unsafe fn affs_mode_to_prot(inode: *mut inode) {
    let mut prot = AFFS_I(inode).i_protect; let mode = (*inode).i_mode;
    prot &= !(FIBF_NOEXECUTE | FIBF_NOREAD | FIBF_NOWRITE | FIBF_NODELETE | FIBF_GRP_EXECUTE | FIBF_GRP_READ | FIBF_GRP_WRITE | FIBF_GRP_DELETE | FIBF_OTR_EXECUTE | FIBF_OTR_READ | FIBF_OTR_WRITE | FIBF_OTR_DELETE);
    if mode & 0o100 == 0 { prot |= FIBF_NOEXECUTE; } if mode & 0o400 == 0 { prot |= FIBF_NOREAD; } if mode & 0o200 == 0 { prot |= FIBF_NOWRITE; }
    if mode & 0o010 != 0 { prot |= FIBF_GRP_EXECUTE; } if mode & 0o040 != 0 { prot |= FIBF_GRP_READ; } if mode & 0o020 != 0 { prot |= FIBF_GRP_WRITE; } if mode & 0o070 != 0 { prot |= FIBF_GRP_DELETE; }
    if mode & 1 != 0 { prot |= FIBF_OTR_EXECUTE; } if mode & 4 != 0 { prot |= FIBF_OTR_READ; } if mode & 2 != 0 { prot |= FIBF_OTR_WRITE; } if mode & 7 != 0 { prot |= FIBF_OTR_DELETE; }
    AFFS_I(inode).i_protect = prot;
}

pub unsafe fn affs_error(sb: *mut super_block, function: *const i8, fmt: *const i8, mut args: ...) { let _ = (sb, function, fmt, &mut args); /* kernel variadic logging dependency */ }
pub unsafe fn affs_warning(sb: *mut super_block, function: *const i8, fmt: *const i8, mut args: ...) { let _ = (sb, function, fmt, &mut args); /* kernel variadic logging dependency */ }

pub unsafe fn affs_nofilenametruncate(dentry: *const dentry) -> bool { affs_test_opt(AFFS_SB((*dentry).d_sb).s_flags, SF_NO_TRUNCATE) }

pub unsafe fn affs_check_name(name: *const u8, mut len: i32, notruncate: bool) -> i32 {
    if len > AFFSNAMEMAX { if notruncate { return -ENAMETOOLONG; } len = AFFSNAMEMAX; }
    for i in 0..len { let c = *name.add(i as usize); if c < b' ' || c == b':' || (c > 0x7e && c < 0xa0) { return -EINVAL; } } 0
}

pub unsafe fn affs_copy_name(mut bstr: *mut u8, dentry: *mut dentry) -> i32 {
    let len = core::cmp::min((*dentry).d_name.len, AFFSNAMEMAX); *bstr = len as u8; bstr = bstr.add(1);
    core::ptr::copy_nonoverlapping((*dentry).d_name.name, bstr, len as usize); len as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
