// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of xfs_trans_dquot.c. */

// C headers provide the external types, constants, functions, and macros used below.

extern "C" {
    fn xfs_trans_add_item(tp: *mut xfs_trans, item: *mut xfs_log_item);
    fn xfs_trans_mod_dquot(tp: *mut xfs_trans, dqp: *mut xfs_dquot, field: u32, delta: i64);
}

#[allow(non_camel_case_types)]
type xfs_trans_t = xfs_trans;

#[repr(C)] pub struct xfs_trans { pub t_mountp: *mut xfs_mount, pub t_dqinfo: *mut xfs_dqinfo, pub t_flags: u32 }
#[repr(C)] pub struct xfs_mount { pub m_quotainfo: *mut xfs_quotainfo, pub m_sb: xfs_sb, pub m_super: *mut xfs_super_block }
#[repr(C)] pub struct xfs_sb;
#[repr(C)] pub struct xfs_super_block { pub s_dev: u64 }
#[repr(C)] pub struct xfs_quotainfo { pub qi_mod_ino_dqtrx_hooks: xfs_hooks, pub qi_apply_dqtrx_hooks: xfs_hooks }
#[repr(C)] pub struct xfs_hooks;
#[repr(C)] pub struct xfs_dqinfo { pub dqs: [*mut xfs_dqtrx; XFS_QM_TRANS_DQTYPES as usize] }
#[repr(C)] pub struct xfs_dqtrx { pub qt_dquot: *mut xfs_dquot, pub qt_blk_res: i64, pub qt_bcount_delta: i64, pub qt_delbcnt_delta: i64, pub qt_ino_res: i64, pub qt_ino_res_used: i64, pub qt_icount_delta: i64, pub qt_rtblk_res: i64, pub qt_rtblk_res_used: i64, pub qt_rtbcount_delta: i64, pub qt_delrtb_delta: i64 }
#[repr(C)] pub struct xfs_log_item { pub li_flags: u64 }
#[repr(C)] pub struct xfs_qlock;
#[repr(C)] pub struct xfs_dquot { pub q_logitem: xfs_log_item_desc, pub q_id: u64, pub q_type: u32, pub q_flags: u32, pub q_qlock: xfs_qlock, pub q_blk: xfs_dquot_res, pub q_ino: xfs_dquot_res, pub q_rtb: xfs_dquot_res }
#[repr(C)] pub struct xfs_log_item_desc { pub qli_item: xfs_log_item, pub qli_dquot: *mut xfs_dquot }
#[repr(C)] pub struct xfs_dquot_res { pub hardlimit: u64, pub softlimit: u64, pub reserved: i64, pub count: i64, pub timer: i64 }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut xfs_mount, pub i_udquot: *mut xfs_dquot, pub i_gdquot: *mut xfs_dquot, pub i_pdquot: *mut xfs_dquot }
#[repr(C)] pub struct xfs_quota_limits { pub hard: u64, pub soft: u64 }
#[repr(C)] pub struct xfs_def_quota { pub blk: xfs_quota_limits, pub rtb: xfs_quota_limits, pub ino: xfs_quota_limits }
#[repr(C)] pub struct xfs_dqtrx_hook { pub mod_hook: xfs_hook, pub apply_hook: xfs_hook }
#[repr(C)] pub struct xfs_hook;
type notifier_fn_t = unsafe extern "C" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void);

const XFS_QM_TRANS_DQTYPES: u32 = 3;
const XFS_QM_TRANS_MAXDQS: u32 = 8;

unsafe extern "C" {
    fn xfs_dquot_type(dqp: *mut xfs_dquot) -> u32;
    fn xfs_has_bigtime(mp: *mut xfs_mount) -> bool;
    fn xfs_hooks_add(h: *mut xfs_hooks, hook: *mut xfs_hook) -> i32;
    fn xfs_hooks_del(h: *mut xfs_hooks, hook: *mut xfs_hook);
    fn xfs_hook_setup(h: *mut xfs_hook, f: notifier_fn_t);
    fn xfs_hooks_call(h: *mut xfs_hooks, field: u32, p: *mut core::ffi::c_void);
    fn xfs_is_metadir_inode(ip: *mut xfs_inode) -> bool;
    fn xfs_trans_dqjoin(tp: *mut xfs_trans, dqp: *mut xfs_dquot);
    fn xfs_dqlock2(a: *mut xfs_dquot, b: *mut xfs_dquot);
    fn xfs_dqlockn(q: *mut xfs_dqtrx);
    fn xfs_qm_adjust_dqlimits(dqp: *mut xfs_dquot);
    fn xfs_qm_adjust_dqtimers(dqp: *mut xfs_dquot);
    fn xfs_trans_log_dquot(tp: *mut xfs_trans, dqp: *mut xfs_dquot);
    fn xfs_get_defquota(qi: *mut xfs_quotainfo, typ: u32) -> *mut xfs_def_quota;
    fn xfs_dquot_is_enforced(dqp: *mut xfs_dquot) -> bool;
    fn xfs_force_shutdown(mp: *mut xfs_mount, reason: u32);
    fn xfs_fs_mark_sick(mp: *mut xfs_mount, reason: u32);
    fn xfs_trans_reserve_quota_bydquots(tp:*mut xfs_trans,mp:*mut xfs_mount,u:*mut xfs_dquot,g:*mut xfs_dquot,p:*mut xfs_dquot,b:i64,n:i64,f:u32)->i32;
    fn kmem_cache_zalloc(cache:*mut core::ffi::c_void, flags:u32)->*mut xfs_dqinfo;
    fn kmem_cache_free(cache:*mut core::ffi::c_void, p:*mut xfs_dqinfo);
    fn xfs_trans_dqresv(tp:*mut xfs_trans,mp:*mut xfs_mount,dqp:*mut xfs_dquot,nblks:i64,ninos:i64,flags:u32)->i32;
}

pub unsafe fn xfs_trans_dqjoin(tp: *mut xfs_trans, dqp: *mut xfs_dquot) { xfs_trans_add_item(tp, &mut (*dqp).q_logitem.qli_item); }

pub unsafe fn xfs_trans_log_dquot_local(tp: *mut xfs_trans, dqp: *mut xfs_dquot) {
    if (*dqp).q_id != 0 && xfs_has_bigtime((*tp).t_mountp) && (*dqp).q_type & XFS_DQTYPE_BIGTIME == 0 { (*dqp).q_type |= XFS_DQTYPE_BIGTIME; }
    (*tp).t_flags |= XFS_TRANS_DIRTY;
    (*dqp).q_logitem.qli_item.li_flags |= XFS_LI_DIRTY;
}

pub unsafe fn xfs_trans_dup_dqinfo(otp:*mut xfs_trans, ntp:*mut xfs_trans) {
    if (*otp).t_dqinfo.is_null() { return; }
    xfs_trans_alloc_dqinfo(ntp);
    for j in 0..XFS_QM_TRANS_DQTYPES { let oq=(*(*otp).t_dqinfo).dqs[j as usize]; let nq=(*(*ntp).t_dqinfo).dqs[j as usize]; for i in 0..XFS_QM_TRANS_MAXDQS { let o=&mut *oq.add(i as usize); let n=&mut *nq.add(i as usize); if o.qt_dquot.is_null(){break;} let used=if o.qt_blk_res!=0&&o.qt_bcount_delta>0{o.qt_bcount_delta}else{0}; n.qt_dquot=o.qt_dquot; n.qt_bcount_delta=0;n.qt_icount_delta=0;n.qt_rtbcount_delta=0;n.qt_blk_res=o.qt_blk_res-used;o.qt_blk_res=used;n.qt_rtblk_res=o.qt_rtblk_res-o.qt_rtblk_res_used;o.qt_rtblk_res=o.qt_rtblk_res_used;n.qt_ino_res=o.qt_ino_res-o.qt_ino_res_used;o.qt_ino_res=o.qt_ino_res_used; }}
}

pub unsafe fn xfs_trans_alloc_dqinfo(tp:*mut xfs_trans) {
    (*tp).t_dqinfo = kmem_cache_zalloc(xfs_dqtrx_cache, GFP_KERNEL | __GFP_NOFAIL);
}

pub unsafe fn xfs_trans_mod_dquot_byino(tp:*mut xfs_trans, ip:*mut xfs_inode, field:u32, delta:i64) { let mp=(*tp).t_mountp; if !XFS_IS_QUOTA_ON(mp)||xfs_is_quota_inode(&(*mp).m_sb,I_INO(ip))||xfs_is_metadir_inode(ip){return;} if XFS_IS_UQUOTA_ON(mp)&&!(*ip).i_udquot.is_null(){xfs_trans_mod_dquot(tp,(*ip).i_udquot,field,delta);} if XFS_IS_GQUOTA_ON(mp)&&!(*ip).i_gdquot.is_null(){xfs_trans_mod_dquot(tp,(*ip).i_gdquot,field,delta);} if XFS_IS_PQUOTA_ON(mp)&&!(*ip).i_pdquot.is_null(){xfs_trans_mod_dquot(tp,(*ip).i_pdquot,field,delta);} }

pub unsafe fn xfs_trans_get_dqtrx(tp:*mut xfs_trans,dqp:*mut xfs_dquot)->*mut xfs_dqtrx { let k=match xfs_dquot_type(dqp){XFS_DQTYPE_USER=>XFS_QM_TRANS_USR,XFS_DQTYPE_GROUP=>XFS_QM_TRANS_GRP,XFS_DQTYPE_PROJ=>XFS_QM_TRANS_PRJ,_=>return core::ptr::null_mut()}; let q=(*(*tp).t_dqinfo).dqs[k as usize]; for i in 0..XFS_QM_TRANS_MAXDQS{let x=q.add(i as usize);if (*x).qt_dquot.is_null()||(*x).qt_dquot==dqp{return x;}} core::ptr::null_mut() }

pub unsafe fn xfs_trans_mod_dquot_impl(tp:*mut xfs_trans,dqp:*mut xfs_dquot,field:u32,delta:i64){if delta==0{return;}if (*tp).t_dqinfo.is_null(){xfs_trans_alloc_dqinfo(tp);}let q=xfs_trans_get_dqtrx(tp,dqp);if (*q).qt_dquot.is_null(){(*q).qt_dquot=dqp;}match field{XFS_TRANS_DQ_RES_BLKS=>(*q).qt_blk_res+=delta,XFS_TRANS_DQ_RES_INOS=>(*q).qt_ino_res+=delta,XFS_TRANS_DQ_BCOUNT=>(*q).qt_bcount_delta+=delta,XFS_TRANS_DQ_DELBCOUNT=>(*q).qt_delbcnt_delta+=delta,XFS_TRANS_DQ_ICOUNT=>{if (*q).qt_ino_res!=0&&delta>0{(*q).qt_ino_res_used+=delta;}(*q).qt_icount_delta+=delta},XFS_TRANS_DQ_RES_RTBLKS=>(*q).qt_rtblk_res+=delta,XFS_TRANS_DQ_RTBCOUNT=>{if (*q).qt_rtblk_res!=0&&delta>0{(*q).qt_rtblk_res_used+=delta;}(*q).qt_rtbcount_delta+=delta},XFS_TRANS_DQ_DELRTBCOUNT=>(*q).qt_delrtb_delta+=delta,_=>{}}}

pub unsafe fn xfs_apply_quota_reservation_deltas(res:*mut xfs_dquot_res,reserved:i64,res_used:i64,count_delta:i64){if reserved!=0{(*res).reserved-= (reserved-res_used).abs();}else if count_delta!=0{(*res).reserved+=count_delta;}}

pub unsafe fn xfs_trans_apply_dquot_deltas(tp:*mut xfs_trans){if (*tp).t_dqinfo.is_null(){return;}for j in 0..XFS_QM_TRANS_DQTYPES{let qa=(*(*tp).t_dqinfo).dqs[j as usize];if (*qa).qt_dquot.is_null(){continue;}for i in 0..XFS_QM_TRANS_MAXDQS{let q=qa.add(i as usize);let d=(*q).qt_dquot;if d.is_null(){break;}let b=(*q).qt_bcount_delta+(*q).qt_delbcnt_delta;let r=(*q).qt_rtbcount_delta+(*q).qt_delrtb_delta;(*d).q_blk.count+=b;(*d).q_ino.count+=(*q).qt_icount_delta;(*d).q_rtb.count+=r;if (*d).q_id!=0{xfs_qm_adjust_dqlimits(d);xfs_qm_adjust_dqtimers(d);}(*d).q_flags|=XFS_DQFLAG_DIRTY;xfs_trans_log_dquot_local(tp,d);xfs_apply_quota_reservation_deltas(&mut (*d).q_blk,(*q).qt_blk_res,(*q).qt_bcount_delta.max(0),(*q).qt_bcount_delta);xfs_apply_quota_reservation_deltas(&mut (*d).q_rtb,(*q).qt_rtblk_res,(*q).qt_rtblk_res_used,(*q).qt_rtbcount_delta);xfs_apply_quota_reservation_deltas(&mut (*d).q_ino,(*q).qt_ino_res,(*q).qt_ino_res_used,(*q).qt_icount_delta);(*q).qt_blk_res=0;(*q).qt_bcount_delta=0;(*q).qt_delbcnt_delta=0;(*q).qt_rtblk_res=0;(*q).qt_rtblk_res_used=0;(*q).qt_rtbcount_delta=0;(*q).qt_delrtb_delta=0;(*q).qt_ino_res=0;(*q).qt_ino_res_used=0;(*q).qt_icount_delta=0;}}}

pub unsafe fn xfs_trans_unreserve_and_mod_dquots(tp:*mut xfs_trans,already_locked:bool){if (*tp).t_dqinfo.is_null(){return;}for j in 0..XFS_QM_TRANS_DQTYPES{let qa=(*(*tp).t_dqinfo).dqs[j as usize];for i in 0..XFS_QM_TRANS_MAXDQS{let q=qa.add(i as usize);let d=(*q).qt_dquot;if d.is_null(){break;}let mut locked=already_locked;if (*q).qt_blk_res!=0{if !locked{mutex_lock(&mut (*d).q_qlock);locked=true;}(*d).q_blk.reserved-=(*q).qt_blk_res;}if (*q).qt_ino_res!=0{if !locked{mutex_lock(&mut (*d).q_qlock);locked=true;}(*d).q_ino.reserved-=(*q).qt_ino_res;}if (*q).qt_rtblk_res!=0{if !locked{mutex_lock(&mut (*d).q_qlock);locked=true;}(*d).q_rtb.reserved-=(*q).qt_rtblk_res;}if locked&&!already_locked{mutex_unlock(&mut (*d).q_qlock);}}}}

pub unsafe fn xfs_trans_reserve_quota_bydquots(tp:*mut xfs_trans,mp:*mut xfs_mount,u:*mut xfs_dquot,g:*mut xfs_dquot,p:*mut xfs_dquot,b:i64,n:i64,flags:u32)->i32{if !XFS_IS_QUOTA_ON(mp){return 0;}for d in [u,g,p]{if !d.is_null(){let e=xfs_trans_dqresv(tp,mp,d,b,n,flags);if e!=0{return e;}}}0}
pub unsafe fn xfs_trans_reserve_quota_nblks(tp:*mut xfs_trans,ip:*mut xfs_inode,d:i64,r:i64,force:bool)->i32{let mp=(*ip).i_mount;if !XFS_IS_QUOTA_ON(mp)||xfs_is_metadir_inode(ip){return 0;}let f=if force{XFS_QMOPT_FORCE_RES}else{0};let e=xfs_trans_reserve_quota_bydquots(tp,mp,(*ip).i_udquot,(*ip).i_gdquot,(*ip).i_pdquot,d,0,XFS_QMOPT_RES_REGBLKS|f);if e!=0{return e;}xfs_trans_reserve_quota_bydquots(tp,mp,(*ip).i_udquot,(*ip).i_gdquot,(*ip).i_pdquot,r,0,XFS_QMOPT_RES_RTBLKS|f)}
pub unsafe fn xfs_trans_reserve_quota_icreate(tp:*mut xfs_trans,u:*mut xfs_dquot,g:*mut xfs_dquot,p:*mut xfs_dquot,d:i64)->i32{xfs_trans_reserve_quota_bydquots(tp,(*tp).t_mountp,u,g,p,d,1,XFS_QMOPT_RES_REGBLKS)}
pub unsafe fn xfs_trans_free_dqinfo(tp:*mut xfs_trans){if !(*tp).t_dqinfo.is_null(){kmem_cache_free(xfs_dqtrx_cache,(*tp).t_dqinfo);(*tp).t_dqinfo=core::ptr::null_mut();}}
pub unsafe fn xfs_quota_reserve_blkres(ip:*mut xfs_inode,blocks:i64)->i32{if XFS_IS_REALTIME_INODE(ip){xfs_trans_reserve_quota_nblks(core::ptr::null_mut(),ip,0,blocks,false)}else{xfs_trans_reserve_quota_nblks(core::ptr::null_mut(),ip,blocks,0,false)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
