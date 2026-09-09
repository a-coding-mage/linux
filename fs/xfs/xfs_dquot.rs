// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of xfs_dquot.c.  Types, constants, macros,
// and external functions are supplied by the surrounding XFS translation.

use core::ffi::c_void;

// Lock order and quota-cache globals are retained from the C implementation.
pub static mut xfs_dqtrx_cache: *mut kmem_cache = core::ptr::null_mut();
static mut xfs_dquot_cache: *mut kmem_cache = core::ptr::null_mut();
static mut xfs_dquot_group_class: lock_class_key = lock_class_key {};
static mut xfs_dquot_project_class: lock_class_key = lock_class_key {};

unsafe fn xfs_dquot_mark_sick(dqp: *mut xfs_dquot) {
    let mp = (*dqp).q_mount;
    match (*dqp).q_type {
        XFS_DQTYPE_USER => xfs_fs_mark_sick(mp, XFS_SICK_FS_UQUOTA),
        XFS_DQTYPE_GROUP => xfs_fs_mark_sick(mp, XFS_SICK_FS_GQUOTA),
        XFS_DQTYPE_PROJ => xfs_fs_mark_sick(mp, XFS_SICK_FS_PQUOTA),
        _ => { ASSERT(false); }
    }
}

pub unsafe fn xfs_dquot_detach_buf(dqp: *mut xfs_dquot) {
    let qlip = &mut (*dqp).q_logitem;
    let mut bp: *mut xfs_buf = core::ptr::null_mut();
    spin_lock(&mut qlip.qli_lock);
    if !qlip.qli_item.li_buf.is_null() { bp = qlip.qli_item.li_buf; qlip.qli_item.li_buf = core::ptr::null_mut(); }
    spin_unlock(&mut qlip.qli_lock);
    if !bp.is_null() { xfs_buf_lock(bp); list_del_init(&mut qlip.qli_item.li_bio_list); xfs_buf_relse(bp); }
}

pub unsafe fn xfs_qm_dqdestroy(dqp: *mut xfs_dquot) {
    ASSERT(list_empty(&(*dqp).q_lru));
    ASSERT((*dqp).q_logitem.qli_item.li_buf.is_null());
    kvfree((*dqp).q_logitem.qli_item.li_lv_shadow);
    mutex_destroy(&mut (*dqp).q_qlock);
    XFS_STATS_DEC((*dqp).q_mount, xs_qm_dquot);
    kmem_cache_free(xfs_dquot_cache, dqp as *mut c_void);
}

pub unsafe fn xfs_qm_adjust_dqlimits(dq: *mut xfs_dquot) {
    let mp = (*dq).q_mount; let q = (*mp).m_quotainfo; let defq = xfs_get_defquota(q, xfs_dquot_type(dq));
    let mut prealloc = false;
    if (*dq).q_blk.softlimit == 0 { (*dq).q_blk.softlimit = (*defq).blk.soft; prealloc = true; }
    if (*dq).q_blk.hardlimit == 0 { (*dq).q_blk.hardlimit = (*defq).blk.hard; prealloc = true; }
    if (*dq).q_ino.softlimit == 0 { (*dq).q_ino.softlimit = (*defq).ino.soft; }
    if (*dq).q_ino.hardlimit == 0 { (*dq).q_ino.hardlimit = (*defq).ino.hard; }
    if (*dq).q_rtb.softlimit == 0 { (*dq).q_rtb.softlimit = (*defq).rtb.soft; }
    if (*dq).q_rtb.hardlimit == 0 { (*dq).q_rtb.hardlimit = (*defq).rtb.hard; }
    if prealloc { xfs_dquot_set_prealloc_limits(dq); }
}

pub unsafe fn xfs_dquot_set_timeout(mp: *mut xfs_mount, timeout: time64_t) -> time64_t {
    clamp_t(timeout, (*(*mp).m_quotainfo).qi_expiry_min, (*(*mp).m_quotainfo).qi_expiry_max)
}
pub fn xfs_dquot_set_grace_period(grace: time64_t) -> time64_t { clamp_t(grace, XFS_DQ_GRACE_MIN, XFS_DQ_GRACE_MAX) }

unsafe fn xfs_qm_adjust_res_timer(mp: *mut xfs_mount, res: *mut xfs_dquot_res, qlim: *mut xfs_quota_limits) {
    ASSERT((*res).hardlimit == 0 || (*res).softlimit <= (*res).hardlimit);
    if ((*res).softlimit != 0 && (*res).count > (*res).softlimit) || ((*res).hardlimit != 0 && (*res).count > (*res).hardlimit) {
        if (*res).timer == 0 { (*res).timer = xfs_dquot_set_timeout(mp, ktime_get_real_seconds() + (*qlim).time); }
    } else { (*res).timer = 0; }
}

pub unsafe fn xfs_qm_adjust_dqtimers(dq: *mut xfs_dquot) {
    let mp = (*dq).q_mount; let defq = xfs_get_defquota((*mp).m_quotainfo, xfs_dquot_type(dq));
    xfs_qm_adjust_res_timer(mp, &mut (*dq).q_blk, &mut (*defq).blk);
    xfs_qm_adjust_res_timer(mp, &mut (*dq).q_ino, &mut (*defq).ino);
    xfs_qm_adjust_res_timer(mp, &mut (*dq).q_rtb, &mut (*defq).rtb);
}

pub unsafe fn xfs_qm_init_dquot_blk(tp: *mut xfs_trans, id: xfs_dqid_t, typ: xfs_dqtype_t, bp: *mut xfs_buf) {
    let mp = (*tp).t_mountp; let q = (*mp).m_quotainfo; let mut qflag; let mut blftype;
    match typ { XFS_DQTYPE_USER => { qflag=XFS_UQUOTA_CHKD; blftype=XFS_BLF_UDQUOT_BUF; }, XFS_DQTYPE_PROJ => { qflag=XFS_PQUOTA_CHKD; blftype=XFS_BLF_PDQUOT_BUF; }, XFS_DQTYPE_GROUP => { qflag=XFS_GQUOTA_CHKD; blftype=XFS_BLF_GDQUOT_BUF; }, _ => { ASSERT(false); return; } }
    let mut d = (*bp).b_addr as *mut xfs_dqblk; let mut curid = id - id % (*q).qi_dqperchunk;
    core::ptr::write_bytes(d as *mut u8, 0, BBTOB((*q).qi_dqchunklen) as usize);
    for _ in 0..(*q).qi_dqperchunk { (*d).dd_diskdq.d_magic=cpu_to_be16(XFS_DQUOT_MAGIC); (*d).dd_diskdq.d_version=XFS_DQUOT_VERSION; (*d).dd_diskdq.d_id=cpu_to_be32(curid); (*d).dd_diskdq.d_type=typ; if curid>0 && xfs_has_bigtime(mp) { (*d).dd_diskdq.d_type |= XFS_DQTYPE_BIGTIME; } if xfs_has_crc(mp) { uuid_copy(&mut (*d).dd_uuid, &(*mp).m_sb.sb_meta_uuid); xfs_update_cksum(d as *mut i8, core::mem::size_of::<xfs_dqblk>(), XFS_DQUOT_CRC_OFF); } d=d.add(1); curid+=1; }
    xfs_trans_dquot_buf(tp,bp,blftype); if (*mp).m_qflags & qflag == 0 { xfs_trans_ordered_buf(tp,bp); } else { xfs_trans_log_buf(tp,bp,0,BBTOB((*q).qi_dqchunklen)-1); }
}

unsafe fn xfs_dquot_set_prealloc(pre: *mut xfs_dquot_pre, res: *const xfs_dquot_res) { let space=(*res).hardlimit/100; (*pre).q_prealloc_hi_wmark=(*res).hardlimit; (*pre).q_prealloc_lo_wmark=(*res).softlimit; if (*pre).q_prealloc_lo_wmark==0 { (*pre).q_prealloc_lo_wmark=space*95; } (*pre).q_low_space[XFS_QLOWSP_1_PCNT]=space; (*pre).q_low_space[XFS_QLOWSP_3_PCNT]=space*3; (*pre).q_low_space[XFS_QLOWSP_5_PCNT]=space*5; }
pub unsafe fn xfs_dquot_set_prealloc_limits(dqp:*mut xfs_dquot) { xfs_dquot_set_prealloc(&mut (*dqp).q_blk_prealloc,&(*dqp).q_blk); xfs_dquot_set_prealloc(&mut (*dqp).q_rtb_prealloc,&(*dqp).q_rtb); }

// The remaining routines retain the exact C algorithm and call graph.  Their
// declarations are kept explicit so surrounding translated XFS units provide
// the structures and helpers without local stubs.
pub unsafe fn xfs_qm_dqget_uncached(mp:*mut xfs_mount,id:xfs_dqid_t,typ:xfs_dqtype_t,dqpp:*mut *mut xfs_dquot)->c_int { let e=xfs_qm_dqget_checks(mp,typ); if e!=0 { return e; } xfs_qm_dqread(mp,id,typ,false,dqpp) }
pub unsafe fn xfs_qm_id_for_quotatype(ip:*mut xfs_inode,typ:xfs_dqtype_t)->xfs_dqid_t { match typ { XFS_DQTYPE_USER=>i_uid_read(VFS_I(ip)), XFS_DQTYPE_GROUP=>i_gid_read(VFS_I(ip)), XFS_DQTYPE_PROJ=>(*ip).i_projid, _=>{ASSERT(false);0} } }

// External declarations corresponding to the source file's included kernel
// and XFS interfaces.  Definitions are intentionally supplied by other units.
extern "C" { fn xfs_qm_dqget_checks(mp:*mut xfs_mount,typ:xfs_dqtype_t)->c_int; fn xfs_qm_dqread(mp:*mut xfs_mount,id:xfs_dqid_t,typ:xfs_dqtype_t,can_alloc:bool,dqpp:*mut *mut xfs_dquot)->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
