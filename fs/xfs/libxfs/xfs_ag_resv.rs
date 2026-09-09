// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/* Per-AG Block Reservations */

pub unsafe fn xfs_ag_resv_critical(
    pag: *mut xfs_perag,
    r#type: xfs_ag_resv_type,
) -> bool {
    let mp = pag_mount(pag);
    let (avail, orig) = match r#type {
        XFS_AG_RESV_METADATA => ((*pag).pagf_freeblks - (*pag).pag_rmapbt_resv.ar_reserved,
            (*pag).pag_meta_resv.ar_asked),
        XFS_AG_RESV_RMAPBT => ((*pag).pagf_freeblks + (*pag).pagf_flcount
            - (*pag).pag_meta_resv.ar_reserved,
            (*pag).pag_rmapbt_resv.ar_asked),
        _ => {
            ASSERT(0);
            return false;
        }
    };

    trace_xfs_ag_resv_critical(pag, r#type, avail);
    avail < orig / 10 || avail < (*mp).m_agbtree_maxlevels
        || XFS_TEST_ERROR(mp, XFS_ERRTAG_AG_RESV_CRITICAL)
}

pub unsafe fn xfs_ag_resv_needed(
    pag: *mut xfs_perag,
    r#type: xfs_ag_resv_type,
) -> xfs_extlen_t {
    let mut len = (*pag).pag_meta_resv.ar_reserved + (*pag).pag_rmapbt_resv.ar_reserved;
    match r#type {
        XFS_AG_RESV_METADATA | XFS_AG_RESV_RMAPBT => {
            len -= xfs_perag_resv(pag, r#type).as_ref().unwrap().ar_reserved;
        }
        XFS_AG_RESV_METAFILE | XFS_AG_RESV_NONE => {}
        _ => ASSERT(0),
    }
    trace_xfs_ag_resv_needed(pag, r#type, len);
    len
}

unsafe fn __xfs_ag_resv_free(pag: *mut xfs_perag, r#type: xfs_ag_resv_type) {
    trace_xfs_ag_resv_free(pag, r#type, 0);
    let resv = xfs_perag_resv(pag);
    if pag_agno(pag) == 0 {
        (*pag_mount(pag)).m_ag_max_usable += (*resv).ar_asked;
    }
    let oldresv = if r#type == XFS_AG_RESV_RMAPBT {
        (*resv).ar_orig_reserved
    } else {
        (*resv).ar_reserved
    };
    xfs_add_fdblocks(pag_mount(pag), oldresv);
    (*resv).ar_reserved = 0;
    (*resv).ar_asked = 0;
    (*resv).ar_orig_reserved = 0;
}

pub unsafe fn xfs_ag_resv_free(pag: *mut xfs_perag) {
    __xfs_ag_resv_free(pag, XFS_AG_RESV_RMAPBT);
    __xfs_ag_resv_free(pag, XFS_AG_RESV_METADATA);
}

unsafe fn __xfs_ag_resv_init(
    pag: *mut xfs_perag,
    r#type: xfs_ag_resv_type,
    mut ask: xfs_extlen_t,
    used: xfs_extlen_t,
) -> i32 {
    let mp = pag_mount(pag);
    if used > ask { ask = used; }
    let hidden_space = match r#type {
        XFS_AG_RESV_RMAPBT => ask,
        XFS_AG_RESV_METADATA => ask - used,
        _ => { ASSERT(0); return -EINVAL; }
    };
    let error = if XFS_TEST_ERROR(mp, XFS_ERRTAG_AG_RESV_FAIL) {
        -ENOSPC
    } else {
        xfs_dec_fdblocks(mp, hidden_space, true)
    };
    if error != 0 {
        trace_xfs_ag_resv_init_error(pag, error, _RET_IP_);
        xfs_warn(mp, "Per-AG reservation for AG %u failed.  Filesystem may run out of space.", pag_agno(pag));
        return error;
    }
    if pag_agno(pag) == 0 { (*mp).m_ag_max_usable -= ask; }
    let resv = xfs_perag_resv(pag, r#type);
    (*resv).ar_asked = ask;
    (*resv).ar_orig_reserved = hidden_space;
    (*resv).ar_reserved = ask - used;
    trace_xfs_ag_resv_init(pag, r#type, ask);
    0
}

pub unsafe fn xfs_ag_resv_init(pag: *mut xfs_perag, tp: *mut xfs_trans) -> i32 {
    let mp = pag_mount(pag);
    let (mut ask, mut used): (xfs_extlen_t, xfs_extlen_t);
    let mut error = 0;
    let mut has_resv = false;
    if (*pag).pag_meta_resv.ar_asked == 0 {
        ask = 0; used = 0;
        error = xfs_refcountbt_calc_reserves(mp, tp, pag, &mut ask, &mut used);
        if error != 0 { goto_out!(); }
        error = xfs_finobt_calc_reserves(pag, tp, &mut ask, &mut used);
        if error != 0 { goto_out!(); }
        error = __xfs_ag_resv_init(pag, XFS_AG_RESV_METADATA, ask, used);
        if error != 0 {
            ask = 0; used = 0; (*mp).m_finobt_nores = true;
            error = xfs_refcountbt_calc_reserves(mp, tp, pag, &mut ask, &mut used);
            if error != 0 { goto_out!(); }
            error = __xfs_ag_resv_init(pag, XFS_AG_RESV_METADATA, ask, used);
            if error != 0 { goto_out!(); }
        }
        if ask != 0 { has_resv = true; }
    }
    if (*pag).pag_rmapbt_resv.ar_asked == 0 {
        ask = 0; used = 0;
        error = xfs_rmapbt_calc_reserves(mp, tp, pag, &mut ask, &mut used);
        if error != 0 { goto_out!(); }
        error = __xfs_ag_resv_init(pag, XFS_AG_RESV_RMAPBT, ask, used);
        if error != 0 { goto_out!(); }
        if ask != 0 { has_resv = true; }
    }
    if has_resv {
        let error2 = xfs_alloc_read_agf(pag, tp, 0, core::ptr::null_mut());
        if error2 != 0 { return error2; }
        if error == 0 && (*xfs_perag_resv(pag, XFS_AG_RESV_METADATA)).ar_reserved
            + (*xfs_perag_resv(pag, XFS_AG_RESV_RMAPBT)).ar_reserved
            > (*pag).pagf_freeblks + (*pag).pagf_flcount { error = -ENOSPC; }
    }
    error
}

pub unsafe fn xfs_ag_resv_alloc_extent(pag: *mut xfs_perag, r#type: xfs_ag_resv_type, args: *mut xfs_alloc_arg) {
    trace_xfs_ag_resv_alloc_extent(pag, r#type, (*args).len);
    match r#type {
        XFS_AG_RESV_AGFL | XFS_AG_RESV_METAFILE => return,
        XFS_AG_RESV_METADATA | XFS_AG_RESV_RMAPBT => {},
        _ => { ASSERT(0); if r#type != XFS_AG_RESV_NONE { return; }
            let field = if (*args).wasdel { XFS_TRANS_SB_RES_FDBLOCKS } else { XFS_TRANS_SB_FDBLOCKS };
            xfs_trans_mod_sb((*args).tp, field, -((*args).len as i64)); return; }
    }
    let resv = xfs_perag_resv(pag, r#type);
    let len = core::cmp::min((*args).len, (*resv).ar_reserved);
    (*resv).ar_reserved -= len;
    if r#type == XFS_AG_RESV_RMAPBT { return; }
    xfs_trans_mod_sb((*args).tp, XFS_TRANS_SB_RES_FDBLOCKS, -(len as i64));
    if (*args).len > len { xfs_trans_mod_sb((*args).tp, XFS_TRANS_SB_FDBLOCKS, -(((*args).len - len) as i64)); }
}

pub unsafe fn xfs_ag_resv_free_extent(pag: *mut xfs_perag, r#type: xfs_ag_resv_type, tp: *mut xfs_trans, len: xfs_extlen_t) {
    trace_xfs_ag_resv_free_extent(pag, r#type, len);
    match r#type {
        XFS_AG_RESV_AGFL | XFS_AG_RESV_METAFILE => return,
        XFS_AG_RESV_METADATA | XFS_AG_RESV_RMAPBT => {},
        XFS_AG_RESV_NONE => { xfs_trans_mod_sb(tp, XFS_TRANS_SB_FDBLOCKS, len as i64); return; },
        XFS_AG_RESV_IGNORE => return,
        _ => { ASSERT(0); return; }
    }
    let resv = xfs_perag_resv(pag, r#type);
    let leftover = core::cmp::min(len, (*resv).ar_asked - (*resv).ar_reserved);
    (*resv).ar_reserved += leftover;
    if r#type == XFS_AG_RESV_RMAPBT { return; }
    xfs_trans_mod_sb(tp, XFS_TRANS_SB_RES_FDBLOCKS, len as i64);
    if len > leftover { xfs_trans_mod_sb(tp, XFS_TRANS_SB_FDBLOCKS, (len - leftover) as i64); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
