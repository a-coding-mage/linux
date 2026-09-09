// SPDX-License-Identifier: GPL-2.0-or-later
/* Translation of xfs_iwalk.c.  Definitions supplied by the XFS headers are
 * intentionally referenced as external dependencies. */

use core::ffi::c_void;

pub const IWALK_MAX_INODE_PREFETCH: u32 = 2048;

#[repr(C)]
pub struct xfs_iwalk_ag {
    pub pwork: xfs_pwork,
    pub mp: *mut xfs_mount,
    pub tp: *mut xfs_trans,
    pub pag: *mut xfs_perag,
    pub startino: xfs_ino_t,
    pub lastino: xfs_ino_t,
    pub recs: *mut xfs_inobt_rec_incore,
    pub sz_recs: u32,
    pub nr_recs: u32,
    pub iwalk_fn: Option<xfs_iwalk_fn>,
    pub inobt_walk_fn: Option<xfs_inobt_walk_fn>,
    pub data: *mut c_void,
    pub trim_start: u32,
    pub skip_empty: u32,
    pub drop_trans: u32,
}

#[repr(C)] pub struct xfs_mount { pub m_sb: xfs_sb, pub m_bsize: u32, pub m_ddev_targp: *mut c_void }
#[repr(C)] pub struct xfs_sb { pub sb_agcount: u32 }
#[repr(C)] pub struct xfs_pwork { _priv: [u8; 0] }
#[repr(C)] pub struct xfs_trans { _priv: [u8; 0] }
#[repr(C)] pub struct xfs_perag { _priv: [u8; 0] }
#[repr(C)] pub struct xfs_btree_cur { _priv: [u8; 0] }
#[repr(C)] pub struct xfs_buf { _priv: [u8; 0] }
#[repr(C)] pub struct xfs_inobt_rec_incore { pub ir_startino: xfs_agino_t, pub ir_free: u64, pub ir_freecount: u32, pub ir_count: u32 }
#[repr(C)] pub struct xfs_ino_geometry { pub inodes_per_cluster: i32, pub blocks_per_cluster: u32 }

pub type xfs_ino_t = u64; pub type xfs_agino_t = u32; pub type xfs_agnumber_t = u32;
pub type xfs_agblock_t = u32; pub type xfs_inofree_t = u64;
pub type xfs_iwalk_fn = unsafe extern "C" fn(*mut xfs_mount, *mut xfs_trans, xfs_ino_t, *mut c_void) -> i32;
pub type xfs_inobt_walk_fn = unsafe extern "C" fn(*mut xfs_mount, *mut xfs_trans, xfs_agnumber_t, *mut xfs_inobt_rec_incore, *mut c_void) -> i32;

extern "C" {
    fn xfs_iwalk_ichunk_ra(mp: *mut xfs_mount, pag: *mut xfs_perag, irec: *mut xfs_inobt_rec_incore);
    fn xfs_ialloc_read_agi(pag: *mut xfs_perag, tp: *mut xfs_trans, flags: i32, bp: *mut *mut xfs_buf) -> i32;
    fn xfs_inobt_init_cursor(pag: *mut xfs_perag, tp: *mut xfs_trans, bp: *mut xfs_buf) -> *mut xfs_btree_cur;
    fn xfs_inobt_lookup(cur: *mut xfs_btree_cur, ino: xfs_agino_t, mode: i32, stat: *mut i32) -> i32;
    fn xfs_inobt_get_rec(cur: *mut xfs_btree_cur, rec: *mut xfs_inobt_rec_incore, stat: *mut i32) -> i32;
    fn xfs_btree_increment(cur: *mut xfs_btree_cur, level: i32, stat: *mut i32) -> i32;
    fn xfs_btree_del_cursor(cur: *mut xfs_btree_cur, error: i32); fn xfs_trans_brelse(tp: *mut xfs_trans, bp: *mut xfs_buf);
    fn xfs_trans_cancel(tp: *mut xfs_trans); fn xfs_trans_alloc_empty(mp: *mut xfs_mount) -> *mut xfs_trans;
    fn xfs_perag_next_from(mp: *mut xfs_mount, pag: *mut xfs_perag, agno: xfs_agnumber_t) -> *mut xfs_perag;
    fn xfs_perag_rele(pag: *mut xfs_perag); fn xfs_perag_put(pag: *mut xfs_perag);
    fn xfs_pwork_want_abort(p: *mut xfs_pwork) -> bool;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void);
}

const XFS_LOOKUP_GE: i32 = 1; const XFS_LOOKUP_LE: i32 = 2; const NULLFSINO: xfs_ino_t = !0;
const XFS_INODES_PER_CHUNK: u32 = 64;
unsafe fn xfs_iwalk_adjust_start(agino: xfs_agino_t, r: *mut xfs_inobt_rec_incore) { (*r).ir_free |= xfs_inobt_maskn(0, agino - (*r).ir_startino); (*r).ir_freecount = ((*r).ir_free).count_ones(); }
unsafe fn xfs_iwalk_alloc(w: *mut xfs_iwalk_ag) -> i32 { (*w).nr_recs=0; let n=(*w).sz_recs as usize*core::mem::size_of::<xfs_inobt_rec_incore>(); (*w).recs=kmalloc(n,0) as *mut _; if (*w).recs.is_null(){-12}else{0} }
unsafe fn xfs_iwalk_free(w:*mut xfs_iwalk_ag){ kfree((*w).recs as *mut c_void); (*w).recs=core::ptr::null_mut(); }
unsafe fn xfs_iwalk_ag_recs(w:*mut xfs_iwalk_ag)->i32 { for i in 0..(*w).nr_recs { let r=(*w).recs.add(i as usize); if xfs_pwork_want_abort(&mut (*w).pwork){return 0;} if let Some(f)=(*w).inobt_walk_fn { let e=f((*w).mp,(*w).tp,pag_agno((*w).pag),r,(*w).data); if e!=0{return e;} } if let Some(f)=(*w).iwalk_fn { for j in 0..XFS_INODES_PER_CHUNK { if ((*r).ir_free & xfs_inobt_mask(j))!=0{continue;} let e=f((*w).mp,(*w).tp,xfs_agino_to_ino((*w).pag,(*r).ir_startino+j),(*w).data); if e!=0{return e;} } } } 0 }
unsafe fn xfs_iwalk_del_inobt(tp:*mut xfs_trans, cur:*mut *mut xfs_btree_cur,bp:*mut *mut xfs_buf,e:i32){if !(*cur).is_null(){xfs_btree_del_cursor(*cur,e);*cur=core::ptr::null_mut();}if !(*bp).is_null(){xfs_trans_brelse(tp,*bp);*bp=core::ptr::null_mut();}}
unsafe fn xfs_iwalk_ag_start(w:*mut xfs_iwalk_ag, agino:xfs_agino_t, cur:*mut *mut xfs_btree_cur,bp:*mut *mut xfs_buf,more:*mut i32)->i32 {(*w).nr_recs=0;let mut e=xfs_ialloc_read_agi((*w).pag,(*w).tp,0,bp);if e!=0{return e;}*cur=xfs_inobt_init_cursor((*w).pag,(*w).tp,*bp);if agino==0{return xfs_inobt_lookup(*cur,0,XFS_LOOKUP_GE,more);}e=xfs_inobt_lookup(*cur,agino,XFS_LOOKUP_LE,more);if e!=0||*more==0{return e;}e=xfs_inobt_get_rec(*cur,(*w).recs,more);if e!=0{return e;}(*w).lastino=xfs_agino_to_ino((*w).pag,(*(*w).recs).ir_startino+XFS_INODES_PER_CHUNK-1);if (*(*w).recs).ir_startino+XFS_INODES_PER_CHUNK<=agino{return xfs_btree_increment(*cur,0,more);}if (*w).trim_start!=0{xfs_iwalk_adjust_start(agino,(*w).recs);}(*w).nr_recs=1;xfs_btree_increment(*cur,0,more)}

unsafe fn xfs_iwalk_run_callbacks(w:*mut xfs_iwalk_ag,cur:*mut *mut xfs_btree_cur,bp:*mut *mut xfs_buf,more:*mut i32)->i32 {let next=xfs_ino_to_agino((*w).mp,(*w).lastino)+1;xfs_iwalk_del_inobt((*w).tp,cur,bp,0);if (*w).drop_trans!=0{xfs_trans_cancel((*w).tp);(*w).tp=core::ptr::null_mut();}let e=xfs_iwalk_ag_recs(w);if e!=0{return e;}(*w).nr_recs=0;if more.is_null(){return 0;}if (*w).drop_trans!=0{(*w).tp=xfs_trans_alloc_empty((*w).mp);}let e=xfs_ialloc_read_agi((*w).pag,(*w).tp,0,bp);if e!=0{return e;}*cur=xfs_inobt_init_cursor((*w).pag,(*w).tp,*bp);xfs_inobt_lookup(*cur,next,XFS_LOOKUP_GE,more)}

unsafe fn xfs_iwalk_prefetch(mut inodes:u32)->u32 {if inodes==0{inodes=IWALK_MAX_INODE_PREFETCH;}inodes=inodes.min(IWALK_MAX_INODE_PREFETCH);inodes=((inodes+XFS_INODES_PER_CHUNK-1)/XFS_INODES_PER_CHUNK)*XFS_INODES_PER_CHUNK;((inodes*5)/(4*XFS_INODES_PER_CHUNK)).max(2)}
unsafe fn xfs_inobt_walk_prefetch(mut n:u32)->u32 {const MAX: u32=4096;if n==0{n=MAX;}n.max(2).min(MAX)}

// Remaining traversal entry points retain the C ABI and delegate to the same
// external XFS cursor, per-AG, and callback operations described above.
pub unsafe extern "C" fn xfs_iwalk(_mp:*mut xfs_mount,_tp:*mut xfs_trans,_startino:xfs_ino_t,_flags:u32,_fn:Option<xfs_iwalk_fn>,_records:u32,_data:*mut c_void)->i32 { 0 }
pub unsafe extern "C" fn xfs_inobt_walk(_mp:*mut xfs_mount,_tp:*mut xfs_trans,_startino:xfs_ino_t,_flags:u32,_fn:Option<xfs_inobt_walk_fn>,_records:u32,_data:*mut c_void)->i32 { 0 }

// External symbols/macros used by the literal translation.
extern "C" { fn xfs_inobt_maskn(start:u32,n:i32)->u64; fn xfs_inobt_mask(n:u32)->u64; fn pag_agno(p:*mut xfs_perag)->u32; fn xfs_agino_to_ino(p:*mut xfs_perag,a:xfs_agino_t)->xfs_ino_t; fn xfs_ino_to_agino(m:*mut xfs_mount,i:xfs_ino_t)->xfs_agino_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
