// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2006-2007 Silicon Graphics, Inc.
 * Copyright (c) 2014 Christoph Hellwig.
 * All Rights Reserved.
 */

// C dependencies are supplied by the surrounding XFS translation.

#[repr(C)]
pub struct xfs_fstrm_item {
    pub mru: xfs_mru_cache_elem,
    pub pag: *mut xfs_perag, // AG in use for this directory
}

pub const XFS_PICK_USERDATA: i32 = 1;
pub const XFS_PICK_LOWSPACE: i32 = 2;

unsafe fn xfs_fstrm_free_func(data: *mut core::ffi::c_void, mru: *mut xfs_mru_cache_elem) {
    let item = (mru as *mut u8).sub(core::mem::offset_of!(xfs_fstrm_item, mru))
        as *mut xfs_fstrm_item;
    let pag = (*item).pag;

    trace_xfs_filestream_free(pag, (*mru).key);
    atomic_dec(&mut (*pag).pagf_fstrms);
    xfs_perag_rele(pag);
    kfree(item as *mut core::ffi::c_void);
}

/*
 * Scan the AGs starting at start_agno looking for an AG that isn't in use and
 * has at least minlen blocks free. If no AG is found to match the allocation
 * requirements, pick the AG with the most free space in it.
 */
unsafe fn xfs_filestream_pick_ag(
    args: *mut xfs_alloc_arg,
    pino: xfs_ino_t,
    start_agno: xfs_agnumber_t,
    mut flags: i32,
    longest: *mut xfs_extlen_t,
) -> i32 {
    let mp = (*args).mp;
    let mut pag: *mut xfs_perag;
    let mut max_pag: *mut xfs_perag = core::ptr::null_mut();
    let minlen = *longest;
    let minfree = (*mp).m_sb.sb_agblocks / 50;
    let mut maxfree: xfs_extlen_t = 0;
    let mut first_pass = true;
    let mut start = start_agno;

    'restart: loop {
        // for_each_perag_wrap(mp, start, agno, pag)
        for agno in xfs_perag_numbers_wrap(mp, start) {
            pag = xfs_perag_get(mp, agno);
            if pag.is_null() { continue; }
            trace_xfs_filestream_scan(pag, pino);
            *longest = 0;
            let mut err = xfs_bmap_longest_free_extent(pag, core::ptr::null_mut(), longest);
            if err != 0 {
                if err == -EAGAIN { continue; }
                xfs_perag_rele(pag);
                if !max_pag.is_null() { xfs_perag_rele(max_pag); }
                return err;
            }
            if (*pag).pagf_freeblks > maxfree {
                maxfree = (*pag).pagf_freeblks;
                if !max_pag.is_null() { xfs_perag_rele(max_pag); }
                atomic_inc(&mut (*pag_group(pag)).xg_active_ref);
                max_pag = pag;
            }
            if atomic_inc_return(&mut (*pag).pagf_fstrms) <= 1 {
                if ((minlen != 0 && *longest >= minlen) ||
                    (minlen == 0 && (*pag).pagf_freeblks >= minfree)) &&
                   (!xfs_perag_prefers_metadata(pag) ||
                    (flags & XFS_PICK_USERDATA) == 0 ||
                    (flags & XFS_PICK_LOWSPACE) != 0) {
                    if !max_pag.is_null() { xfs_perag_rele(max_pag); }
                    break 'restart;
                }
            }
            atomic_dec(&mut (*pag).pagf_fstrms);
        }
        if first_pass { first_pass = false; continue 'restart; }
        if (flags & XFS_PICK_LOWSPACE) == 0 { flags |= XFS_PICK_LOWSPACE; continue 'restart; }
        break;
    }

    if max_pag.is_null() {
        for agno in xfs_perag_numbers_wrap(args.mp, 0) {
            max_pag = xfs_perag_get(args.mp, agno);
            break;
        }
        if max_pag.is_null() { return -ENOSPC; }
    }
    pag = max_pag;
    atomic_inc(&mut (*pag).pagf_fstrms);
    trace_xfs_filestream_pick(pag, pino);
    (*args).pag = pag;
    0
}

unsafe fn xfs_filestream_get_parent(ip: *mut xfs_inode) -> *mut xfs_inode {
    let inode = VFS_I(ip);
    let mut dir: *mut inode = core::ptr::null_mut();
    let dentry = d_find_alias(inode);
    if dentry.is_null() { return core::ptr::null_mut(); }
    let parent = dget_parent(dentry);
    if parent.is_null() { dput(dentry); return core::ptr::null_mut(); }
    dir = igrab(d_inode(parent));
    dput(parent);
    dput(dentry);
    if !dir.is_null() { XFS_I(dir) } else { core::ptr::null_mut() }
}

unsafe fn xfs_filestream_lookup_association(ap: *mut xfs_bmalloca, args: *mut xfs_alloc_arg, pino: xfs_ino_t, longest: *mut xfs_extlen_t) -> i32 {
    let mp = (*args).mp;
    *longest = 0;
    let mru = xfs_mru_cache_lookup((*mp).m_filestream, pino);
    if mru.is_null() { return 0; }
    let pag = (*(mru as *mut u8).sub(core::mem::offset_of!(xfs_fstrm_item, mru)) as *mut xfs_fstrm_item).pag;
    atomic_inc(&mut (*pag_group(pag)).xg_active_ref);
    xfs_mru_cache_done((*mp).m_filestream);
    trace_xfs_filestream_lookup(pag, I_INO((*ap).ip));
    (*ap).blkno = xfs_agbno_to_fsb(pag, 0);
    xfs_bmap_adjacent(ap);
    if ((*(*ap).tp).t_flags & XFS_TRANS_LOWMODE) != 0 { *longest = 1; (*args).pag = pag; return 0; }
    let mut error = xfs_bmap_longest_free_extent(pag, (*args).tp, longest);
    if error == -EAGAIN { error = 0; }
    if error != 0 || *longest < (*args).maxlen { *longest = 0; xfs_perag_rele(pag); return error; }
    (*args).pag = pag;
    0
}

unsafe fn xfs_filestream_create_association(ap: *mut xfs_bmalloca, args: *mut xfs_alloc_arg, pino: xfs_ino_t, longest: *mut xfs_extlen_t) -> i32 {
    let mp = (*args).mp;
    let mut agno = XFS_INO_TO_AGNO(mp, pino);
    let mut flags = 0;
    let mru = xfs_mru_cache_remove((*mp).m_filestream, pino);
    if !mru.is_null() {
        let item = (mru as *mut u8).sub(core::mem::offset_of!(xfs_fstrm_item, mru)) as *mut xfs_fstrm_item;
        agno = (pag_agno((*item).pag) + 1) % (*mp).m_sb.sb_agcount;
        xfs_fstrm_free_func(mp as *mut _, mru);
    } else if xfs_is_inode32(mp) {
        let rotorstep = xfs_rotorstep;
        agno = ((*mp).m_agfrotor / rotorstep) % (*mp).m_sb.sb_agcount;
        (*mp).m_agfrotor = ((*mp).m_agfrotor + 1) % ((*mp).m_sb.sb_agcount * rotorstep);
    }
    (*ap).blkno = XFS_AGB_TO_FSB((*args).mp, agno, 0);
    xfs_bmap_adjacent(ap);
    if ((*ap).datatype & XFS_ALLOC_USERDATA) != 0 { flags |= XFS_PICK_USERDATA; }
    if ((*(*ap).tp).t_flags & XFS_TRANS_LOWMODE) != 0 { flags |= XFS_PICK_LOWSPACE; }
    *longest = (*ap).length;
    let error = xfs_filestream_pick_ag(args, pino, agno, flags, longest);
    if error != 0 { return error; }
    let item = kmalloc_obj::<xfs_fstrm_item>(GFP_KERNEL | __GFP_RETRY_MAYFAIL);
    if item.is_null() { atomic_dec(&mut (*(*args).pag).pagf_fstrms); return 0; }
    atomic_inc(&mut (*pag_group((*args).pag)).xg_active_ref);
    (*item).pag = (*args).pag;
    xfs_mru_cache_insert((*mp).m_filestream, pino, &mut (*item).mru);
    0
}

pub unsafe fn xfs_filestream_select_ag(ap: *mut xfs_bmalloca, args: *mut xfs_alloc_arg, longest: *mut xfs_extlen_t) -> i32 {
    *longest = 0;
    (*args).total = (*ap).total;
    let pip = xfs_filestream_get_parent((*ap).ip);
    let mut ino = 0;
    if !pip.is_null() {
        ino = I_INO(pip);
        let error = xfs_filestream_lookup_association(ap, args, ino, longest);
        xfs_irele(pip);
        if error != 0 { return error; }
        if *longest >= (*args).maxlen || ((*(*ap).tp).t_flags & XFS_TRANS_LOWMODE) != 0 { (*ap).blkno = xfs_agbno_to_fsb((*args).pag, 0); return 0; }
    }
    let error = xfs_filestream_create_association(ap, args, ino, longest);
    if error != 0 { return error; }
    (*ap).blkno = xfs_agbno_to_fsb((*args).pag, 0);
    0
}

pub unsafe fn xfs_filestream_deassociate(ip: *mut xfs_inode) { xfs_mru_cache_delete((*(*ip).i_mount).m_filestream, I_INO(ip)); }

pub unsafe fn xfs_filestream_mount(mp: *mut xfs_mount_t) -> i32 {
    xfs_mru_cache_create(&mut (*mp).m_filestream, mp, xfs_fstrm_centisecs * 10, 10, xfs_fstrm_free_func)
}

pub unsafe fn xfs_filestream_unmount(mp: *mut xfs_mount_t) { xfs_mru_cache_destroy((*mp).m_filestream); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
