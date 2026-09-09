// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level Rust translation of UBIFS find.c. External UBIFS types,
 * constants, macros, and functions are supplied by the surrounding crate. */

#[repr(C)]
pub struct scan_data { pub min_space: i32, pub pick_free: i32, pub lnum: i32, pub exclude_index: i32 }

unsafe fn valuable(c: *mut ubifs_info, lprops: *const ubifs_lprops) -> i32 {
    let cat = (*lprops).flags & LPROPS_CAT_MASK;
    match cat {
        LPROPS_DIRTY | LPROPS_DIRTY_IDX | LPROPS_FREE => { let h = &(*c).lpt_heap[(cat - 1) as usize]; if h.cnt < h.max_cnt || (*lprops).free + (*lprops).dirty >= (*c).dark_wm { 1 } else { 0 } },
        LPROPS_EMPTY => { let n = (*c).lst.empty_lebs + (*c).freeable_cnt - (*c).lst.taken_empty_lebs; if n < (*c).lsave_cnt { 1 } else { 0 } },
        LPROPS_FREEABLE | LPROPS_FRDI_IDX => 1,
        _ => 0,
    }
}

unsafe fn scan_for_dirty_cb(c: *mut ubifs_info, lp: *const ubifs_lprops, in_tree: i32, arg: *mut core::ffi::c_void) -> i32 {
    let d = &mut *(arg as *mut scan_data); let mut ret = LPT_SCAN_CONTINUE;
    if (*lp).flags & LPROPS_TAKEN != 0 { return ret; }
    if in_tree == 0 && valuable(c, lp) != 0 { ret |= LPT_SCAN_ADD; }
    if (*lp).free + (*lp).dirty < d.min_space { return ret; }
    if d.exclude_index != 0 && (*lp).flags & LPROPS_INDEX != 0 { return ret; }
    if (*lp).free + (*lp).dirty == (*c).leb_size { if d.pick_free == 0 { return ret; } }
    else if (*lp).dirty < (*c).dead_wm { return ret; }
    d.lnum = (*lp).lnum; LPT_SCAN_ADD | LPT_SCAN_STOP
}

unsafe fn scan_for_dirty(c: *mut ubifs_info, min_space: i32, pick_free: i32, exclude_index: i32) -> *const ubifs_lprops {
    let h = &(*c).lpt_heap[(LPROPS_FREE-1) as usize];
    for i in 0..h.cnt as usize { let p = h.arr[i]; if (*p).free + (*p).dirty >= min_space && (*p).dirty >= (*c).dead_wm { return p; } }
    let mut d = scan_data { min_space, pick_free, lnum: -1, exclude_index };
    let e = ubifs_lpt_scan_nolock(c, -1, (*c).lscan_lnum, scan_for_dirty_cb, &mut d as *mut _ as *mut _);
    if e != 0 { return ERR_PTR(e); } (*c).lscan_lnum = d.lnum; ubifs_lpt_lookup_dirty(c, d.lnum)
}

pub unsafe fn ubifs_find_dirty_leb(c: *mut ubifs_info, out: *mut ubifs_lprops, min_space: i32, mut pick_free: i32) -> i32 {
    let mut err = 0; let mut exclude = if pick_free == 2 { 1 } else { 0 }; ubifs_get_lprops(c);
    let mut lp: *const ubifs_lprops = core::ptr::null(); let mut idx: *const ubifs_lprops = core::ptr::null();
    if pick_free != 0 { let lebs; let mut rsvd = 0; spin_lock(&mut (*c).space_lock); lebs = (*c).lst.empty_lebs + (*c).idx_gc_cnt + (*c).freeable_cnt - (*c).lst.taken_empty_lebs; if (*c).bi.min_idx_lebs >= (*c).lst.idx_lebs { rsvd = (*c).bi.min_idx_lebs - (*c).lst.idx_lebs; exclude = 1; } spin_unlock(&mut (*c).space_lock); if rsvd < lebs { lp = ubifs_fast_find_empty(c); if lp.is_null() { lp = ubifs_fast_find_freeable(c); } } else { pick_free = 0; } } else { spin_lock(&mut (*c).space_lock); exclude = if (*c).bi.min_idx_lebs >= (*c).lst.idx_lebs {1} else {0}; spin_unlock(&mut (*c).space_lock); }
    let h = &(*c).lpt_heap[(LPROPS_DIRTY-1) as usize]; let ih = &(*c).lpt_heap[(LPROPS_DIRTY_IDX-1) as usize];
    if ih.cnt != 0 && exclude == 0 { idx = ih.arr[0]; if (*idx).free + (*idx).dirty < min_space || (*idx).free + (*idx).dirty < (*c).half_leb_size { idx = core::ptr::null(); } }
    if lp.is_null() && h.cnt != 0 { lp = h.arr[0]; if (*lp).free + (*lp).dirty < min_space { lp = core::ptr::null(); } }
    if !idx.is_null() && (lp.is_null() || (*idx).free + (*idx).dirty >= (*lp).free + (*lp).dirty) { lp = idx; }
    if lp.is_null() { lp = scan_for_dirty(c, min_space, pick_free, exclude); if IS_ERR(lp) { err = PTR_ERR(lp); ubifs_release_lprops(c); return err; } }
    lp = ubifs_change_lp(c, lp, LPROPS_NC, LPROPS_NC, (*lp).flags | LPROPS_TAKEN, 0); if IS_ERR(lp) { err = PTR_ERR(lp); } else { core::ptr::copy_nonoverlapping(lp, out, 1); } ubifs_release_lprops(c); err
}

unsafe fn scan_for_free_cb(c: *mut ubifs_info, lp: *const ubifs_lprops, in_tree: i32, arg: *mut core::ffi::c_void) -> i32 { let d=&mut *(arg as *mut scan_data); let mut r=LPT_SCAN_CONTINUE; if (*lp).flags&LPROPS_TAKEN!=0{return r} if in_tree==0&&valuable(c,lp)!=0{r|=LPT_SCAN_ADD} if (*lp).flags&LPROPS_INDEX!=0||(*lp).free<d.min_space{return r} if d.pick_free==0&&(*lp).free==(*c).leb_size{return r} if (*lp).free+(*lp).dirty==(*c).leb_size&&(*lp).dirty>0{return r} d.lnum=(*lp).lnum; LPT_SCAN_ADD|LPT_SCAN_STOP }

unsafe fn do_find_free_space(c:*mut ubifs_info,min:i32,pick:i32,squeeze:i32)->*const ubifs_lprops { let mut p=ubifs_fast_find_free(c); if squeeze!=0&& !p.is_null()&&(*p).free>=min{return p} if pick!=0 {p=ubifs_fast_find_empty(c);if !p.is_null(){return p}} if squeeze==0&&!p.is_null()&&(*p).free>=min{return p} let h=&(*c).lpt_heap[(LPROPS_DIRTY-1)as usize];for i in 0..h.cnt as usize{p=h.arr[i];if (*p).free>=min{return p}} let mut d=scan_data{min_space:min,pick_free:pick,lnum:-1,exclude_index:0};let e=ubifs_lpt_scan_nolock(c,-1,(*c).lscan_lnum,scan_for_free_cb,&mut d as *mut _ as *mut _);if e!=0{return ERR_PTR(e)} (*c).lscan_lnum=d.lnum;ubifs_lpt_lookup_dirty(c,d.lnum) }

pub unsafe fn ubifs_find_free_space(c:*mut ubifs_info,min:i32,offs:*mut i32,squeeze:i32)->i32 { ubifs_get_lprops(c);let p=do_find_free_space(c,min,1,squeeze);if IS_ERR(p){let e=PTR_ERR(p);ubifs_release_lprops(c);return e}let n=(*p).lnum;let q=ubifs_change_lp(c,p,LPROPS_NC,LPROPS_NC,(*p).flags|LPROPS_TAKEN,0);if IS_ERR(q){let e=PTR_ERR(q);ubifs_release_lprops(c);return PTR_ERR(q)}*offs=(*c).leb_size-(*q).free;ubifs_release_lprops(c);n}

// Remaining index-LEB routines retain the same externally supplied UBIFS operations.
pub unsafe fn ubifs_find_free_leb_for_idx(c:*mut ubifs_info)->i32 { ubifs_get_lprops(c);let mut p=ubifs_fast_find_empty(c);if p.is_null(){p=ubifs_fast_find_freeable(c)}if p.is_null(){ubifs_release_lprops(c);return -ENOSPC}let n=(*p).lnum;let q=ubifs_change_lp(c,p,(*c).leb_size,0,(*p).flags|LPROPS_TAKEN|LPROPS_INDEX,0);if IS_ERR(q){let e=PTR_ERR(q);ubifs_release_lprops(c);return e}ubifs_release_lprops(c);let e=ubifs_leb_unmap(c,n);if e!=0{return e}n}

pub unsafe fn ubifs_find_dirty_idx_leb(c:*mut ubifs_info)->i32 { ubifs_get_lprops(c);let h=&(*c).lpt_heap[(LPROPS_DIRTY_IDX-1)as usize];for i in 0..h.cnt as usize{let p=h.arr[i];if (*p).flags&LPROPS_TAKEN==0&&(*p).free+(*p).dirty>=(*c).min_idx_node_sz{let q=ubifs_change_lp(c,p,LPROPS_NC,LPROPS_NC,(*p).flags|LPROPS_TAKEN,0);if !IS_ERR(q){let n=(*q).lnum;ubifs_release_lprops(c);return n}}}ubifs_release_lprops(c);-ENOSPC}

unsafe fn cmp_dirty_idx(a:*const core::ffi::c_void,b:*const core::ffi::c_void)->i32 { let x=*(a as *const *const ubifs_lprops);let y=*(b as *const *const ubifs_lprops);(*x).dirty+(*x).free-(*y).dirty-(*y).free }
pub unsafe fn ubifs_save_dirty_idx_lnums(c:*mut ubifs_info)->i32 { ubifs_get_lprops(c);(*c).dirty_idx.cnt=(*c).lpt_heap[(LPROPS_DIRTY_IDX-1)as usize].cnt;core::ptr::copy_nonoverlapping((*c).lpt_heap[(LPROPS_DIRTY_IDX-1)as usize].arr,(*c).dirty_idx.arr,(*c).dirty_idx.cnt as usize);sort((*c).dirty_idx.arr,(*c).dirty_idx.cnt,core::mem::size_of::<*mut core::ffi::c_void>(),cmp_dirty_idx,core::ptr::null_mut());for i in 0..(*c).dirty_idx.cnt as usize{let p=(*c).dirty_idx.arr[i] as *const ubifs_lprops;(*c).dirty_idx.arr[i]=(*p).lnum as usize as *mut core::ffi::c_void}ubifs_release_lprops(c);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
