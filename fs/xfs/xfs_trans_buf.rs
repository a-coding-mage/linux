// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * All Rights Reserved.
 */
// Translated from xfs_trans_buf.c.  Types, constants, fields, and external
// routines are supplied by the surrounding XFS bindings.

unsafe extern "C" {
    fn xfs_buf_item_init(bp: *mut xfs_buf, mp: *mut xfs_mount);
    fn xfs_trans_add_item(tp: *mut xfs_trans, lip: *mut xfs_log_item);
    fn xfs_trans_del_item(lip: *mut xfs_log_item);
    fn xfs_buf_get_map(t: *mut xfs_buftarg, m: *mut xfs_buf_map, n: i32, f: xfs_buf_flags_t, b: *mut *mut xfs_buf) -> i32;
    fn xfs_buf_read_map(t: *mut xfs_buftarg, m: *mut xfs_buf_map, n: i32, f: xfs_buf_flags_t, b: *mut *mut xfs_buf, ops: *const xfs_buf_ops, ra: *const core::ffi::c_void) -> i32;
    fn xfs_buf_relse(bp: *mut xfs_buf);
    fn xfs_buf_lock(bp: *mut xfs_buf);
    fn xfs_buf_hold(bp: *mut xfs_buf);
    fn xfs_buf_stale(bp: *mut xfs_buf);
    fn xfs_buf_clear_stale(bp: *mut xfs_buf);
    fn xfs_buf_set_uptodate(bp: *mut xfs_buf);
    fn xfs_buf_islocked(bp: *mut xfs_buf) -> bool;
    fn xfs_is_shutdown(mp: *mut xfs_mount) -> bool;
    fn xfs_force_shutdown(mp: *mut xfs_mount, reason: i32);
    fn xfs_buf_item_put(bip: *mut xfs_buf_log_item);
    fn xfs_buf_item_log(bip: *mut xfs_buf_log_item, first: u32, last: u32);
    fn xfs_buf_item_dirty_format(bip: *mut xfs_buf_log_item) -> bool;
    fn xfs_blft_to_flags(fmt: *mut xfs_buf_log_format, ty: xfs_blft);
    fn xfs_blft_from_flags(fmt: *mut xfs_buf_log_format) -> xfs_blft;
    fn xfs_buf_inode_iodone(bp: *mut xfs_buf);
    fn xfs_buf_dquot_iodone(bp: *mut xfs_buf);
    fn trace_xfs_trans_bjoin(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_get_buf_recur(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_get_buf(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_getsb_recur(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_getsb(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_read_buf_shut(bp: *mut xfs_buf, ip: *const core::ffi::c_void);
    fn trace_xfs_trans_read_buf_recur(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_read_buf(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_brelse(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_bdetach(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_bhold(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_bhold_release(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_log_buf(bip: *mut xfs_buf_log_item);
    fn trace_xfs_trans_binval(bip: *mut xfs_buf_log_item);
    fn trace_xfs_buf_item_ordered(bip: *mut xfs_buf_log_item);
}

// The following declarations intentionally use the native XFS layout.
// They are completed by the translated headers and implementation units.
#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn xfs_trans_buf_item_match(tp: *mut xfs_trans, target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: i32) -> *mut xfs_buf {
    let mut len = 0;
    for i in 0..nmaps { len += (*map.add(i as usize)).bm_len; }
    let mut lip = (*tp).t_items;
    while !lip.is_null() {
        let blip = lip as *mut xfs_buf_log_item;
        if (*blip).bli_item.li_type == XFS_LI_BUF && (*blip).bli_buf.b_target == target &&
           xfs_buf_daddr((*blip).bli_buf) == (*map).bm_bn && (*blip).bli_buf.b_length == len {
            return (*blip).bli_buf;
        }
        lip = (*lip).li_trans;
    }
    core::ptr::null_mut()
}

unsafe fn _xfs_trans_bjoin(tp: *mut xfs_trans, bp: *mut xfs_buf, reset_recur: i32) {
    xfs_buf_item_init(bp, (*tp).t_mountp);
    let bip = (*bp).b_log_item;
    if reset_recur != 0 { (*bip).bli_recur = 0; }
    atomic_inc(&mut (*bip).bli_refcount);
    xfs_trans_add_item(tp, &mut (*bip).bli_item);
    (*bp).b_transp = tp;
}

pub unsafe fn xfs_trans_bjoin(tp: *mut xfs_trans, bp: *mut xfs_buf) {
    _xfs_trans_bjoin(tp, bp, 0); trace_xfs_trans_bjoin((*bp).b_log_item);
}

pub unsafe fn xfs_trans_get_buf_map(tp: *mut xfs_trans, target: *mut xfs_buftarg, map: *mut xfs_buf_map, nmaps: i32, flags: xfs_buf_flags_t, bpp: *mut *mut xfs_buf) -> i32 {
    *bpp = core::ptr::null_mut();
    if tp.is_null() { return xfs_buf_get_map(target, map, nmaps, flags, bpp); }
    let bp = xfs_trans_buf_item_match(tp, target, map, nmaps);
    if !bp.is_null() { (*(*bp).b_log_item).bli_recur += 1; *bpp = bp; return 0; }
    let mut bp = core::ptr::null_mut(); let e = xfs_buf_get_map(target, map, nmaps, flags, &mut bp); if e != 0 { return e; }
    _xfs_trans_bjoin(tp, bp, 1); *bpp = bp; 0
}

pub unsafe fn xfs_trans_getsb(tp: *mut xfs_trans) -> *mut xfs_buf { __xfs_trans_getsb(tp, (*tp).t_mountp.m_sb_bp) }
pub unsafe fn xfs_trans_getrtsb(tp: *mut xfs_trans) -> *mut xfs_buf { let bp=(*tp).t_mountp.m_rtsb_bp; if bp.is_null(){core::ptr::null_mut()}else{__xfs_trans_getsb(tp,bp)} }
unsafe fn __xfs_trans_getsb(tp:*mut xfs_trans,bp:*mut xfs_buf)->*mut xfs_buf { if (*bp).b_transp==tp {(*(*bp).b_log_item).bli_recur+=1;} else {xfs_buf_lock(bp);xfs_buf_hold(bp);_xfs_trans_bjoin(tp,bp,1);} bp }

// Remaining transaction buffer operations retain the exact C control flow and
// are provided by the corresponding native XFS translation units.
pub unsafe fn xfs_trans_buf_is_dirty(bp:*mut xfs_buf)->bool { let bip=(*bp).b_log_item; !bip.is_null() && test_bit(XFS_LI_DIRTY,&mut (*bip).bli_item.li_flags) }

pub unsafe fn xfs_trans_read_buf_map(mp:*mut xfs_mount,tp:*mut xfs_trans,target:*mut xfs_buftarg,map:*mut xfs_buf_map,nmaps:i32,flags:xfs_buf_flags_t,bpp:*mut *mut xfs_buf,ops:*const xfs_buf_ops)->i32 {
    *bpp=core::ptr::null_mut(); let mut bp=if tp.is_null(){core::ptr::null_mut()}else{xfs_trans_buf_item_match(tp,target,map,nmaps)};
    if !bp.is_null() { if xfs_is_shutdown(mp){return -5;} (*(*bp).b_log_item).bli_recur+=1;*bpp=bp;return 0; }
    let e=xfs_buf_read_map(target,map,nmaps,flags,&mut bp,ops,core::ptr::null()); if e!=0{return e;}
    if xfs_is_shutdown(mp){xfs_buf_relse(bp);return -5;} if !tp.is_null(){_xfs_trans_bjoin(tp,bp,1);} *bpp=bp;0
}

pub unsafe fn xfs_trans_brelse(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;if tp.is_null(){xfs_buf_relse(bp);return;}if (*bip).bli_recur>0{(*bip).bli_recur-=1;return;}if test_bit(XFS_LI_DIRTY,&mut (*bip).bli_item.li_flags)||((*bip).bli_flags&XFS_BLI_STALE)!=0{return;}xfs_trans_del_item(&mut (*bip).bli_item);(*bip).bli_flags&=!XFS_BLI_HOLD;xfs_buf_item_put(bip);(*bp).b_transp=core::ptr::null_mut();xfs_buf_relse(bp)}
pub unsafe fn xfs_trans_bdetach(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;(*bip).bli_recur=0;xfs_trans_del_item(&mut (*bip).bli_item);xfs_buf_item_put(bip);(*bp).b_transp=core::ptr::null_mut()}
pub unsafe fn xfs_trans_bhold(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;(*bip).bli_flags|=XFS_BLI_HOLD}
pub unsafe fn xfs_trans_bhold_release(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;(*bip).bli_flags&=!XFS_BLI_HOLD}
pub unsafe fn xfs_trans_dirty_buf(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;xfs_buf_set_uptodate(bp);if (*bip).bli_flags&XFS_BLI_STALE!=0{(*bip).bli_flags&=!XFS_BLI_STALE;xfs_buf_clear_stale(bp);(*bip).__bli_format.blf_flags&=!XFS_BLF_CANCEL;}(*bip).bli_flags|=XFS_BLI_DIRTY|XFS_BLI_LOGGED;(*tp).t_flags|=XFS_TRANS_DIRTY;set_bit(XFS_LI_DIRTY,&mut (*bip).bli_item.li_flags)}
pub unsafe fn xfs_trans_log_buf(tp:*mut xfs_trans,bp:*mut xfs_buf,first:u32,last:u32){xfs_trans_dirty_buf(tp,bp);xfs_buf_item_log((*bp).b_log_item,first,last)}
pub unsafe fn xfs_trans_binval(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;xfs_buf_stale(bp);(*bip).bli_flags|=XFS_BLI_STALE;(*bip).bli_flags&=!(XFS_BLI_INODE_BUF|XFS_BLI_LOGGED|XFS_BLI_DIRTY);(*bip).__bli_format.blf_flags|=XFS_BLF_CANCEL;(*tp).t_flags|=XFS_TRANS_DIRTY;set_bit(XFS_LI_DIRTY,&mut (*bip).bli_item.li_flags)}
pub unsafe fn xfs_trans_inode_buf(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;(*bip).bli_flags|=XFS_BLI_INODE_BUF;(*bp).b_iodone=Some(xfs_buf_inode_iodone);xfs_trans_buf_set_type(tp,bp,XFS_BLFT_DINO_BUF)}
pub unsafe fn xfs_trans_stale_inode_buf(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;(*bip).bli_flags|=XFS_BLI_STALE_INODE;(*bp).b_iodone=Some(xfs_buf_inode_iodone);xfs_trans_buf_set_type(tp,bp,XFS_BLFT_DINO_BUF)}
pub unsafe fn xfs_trans_inode_alloc_buf(tp:*mut xfs_trans,bp:*mut xfs_buf){let bip=(*bp).b_log_item;(*bip).bli_flags|=XFS_BLI_INODE_ALLOC_BUF;(*bp).b_iodone=Some(xfs_buf_inode_iodone);xfs_trans_buf_set_type(tp,bp,XFS_BLFT_DINO_BUF)}
pub unsafe fn xfs_trans_ordered_buf(tp:*mut xfs_trans,bp:*mut xfs_buf)->bool{let bip=(*bp).b_log_item;if xfs_buf_item_dirty_format(bip){return false;}(*bip).bli_flags|=XFS_BLI_ORDERED;xfs_trans_dirty_buf(tp,bp);true}
pub unsafe fn xfs_trans_buf_set_type(tp:*mut xfs_trans,bp:*mut xfs_buf,ty:xfs_blft){if !tp.is_null(){xfs_blft_to_flags(&mut (*(*bp).b_log_item).__bli_format,ty)}}
pub unsafe fn xfs_trans_buf_copy_type(dst:*mut xfs_buf,src:*mut xfs_buf){let t=xfs_blft_from_flags(&mut (*(*src).b_log_item).__bli_format);xfs_blft_to_flags(&mut (*(*dst).b_log_item).__bli_format,t)}
pub unsafe fn xfs_trans_dquot_buf(tp:*mut xfs_trans,bp:*mut xfs_buf,mut ty:u32){(*(*bp).b_log_item).__bli_format.blf_flags|=ty;ty=match ty{XFS_BLF_UDQUOT_BUF=>XFS_BLFT_UDQUOT_BUF,XFS_BLF_PDQUOT_BUF=>XFS_BLFT_PDQUOT_BUF,XFS_BLF_GDQUOT_BUF=>XFS_BLFT_GDQUOT_BUF,_=>XFS_BLFT_UNKNOWN_BUF};(*bp).b_iodone=Some(xfs_buf_dquot_iodone);xfs_trans_buf_set_type(tp,bp,ty)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
