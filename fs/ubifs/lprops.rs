// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of ubifs/lprops.c.  Types and helpers
// supplied by ubifs.h are intentionally left as external dependencies.

#[allow(dead_code, unused_variables, non_camel_case_types)]
unsafe fn get_heap_comp_val(lprops: *mut ubifs_lprops, cat: i32) -> i32 {
    match cat { LPROPS_FREE => (*lprops).free, LPROPS_DIRTY_IDX => (*lprops).free + (*lprops).dirty, _ => (*lprops).dirty }
}

unsafe fn move_up_lpt_heap(c: *mut ubifs_info, heap: *mut ubifs_lpt_heap, lprops: *mut ubifs_lprops, cat: i32) {
    let mut hpos = (*lprops).hpos; if hpos == 0 { return; }
    let val1 = get_heap_comp_val(lprops, cat);
    while hpos != 0 { let ppos = (hpos - 1) / 2; let val2 = get_heap_comp_val(*(*heap).arr.add(ppos as usize), cat); if val2 >= val1 { return; }
        (**(*heap).arr.add(ppos as usize)).hpos = hpos; *(*heap).arr.add(hpos as usize) = *(*heap).arr.add(ppos as usize); *(*heap).arr.add(ppos as usize) = lprops; (*lprops).hpos = ppos; hpos = ppos; }
}

unsafe fn adjust_lpt_heap(c: *mut ubifs_info, heap: *mut ubifs_lpt_heap, lprops: *mut ubifs_lprops, mut hpos: i32, cat: i32) {
    let val1 = get_heap_comp_val(lprops, cat);
    if hpos != 0 { let mut ppos = (hpos - 1) / 2; if val1 > get_heap_comp_val(*(*heap).arr.add(ppos as usize), cat) { loop { (**(*heap).arr.add(ppos as usize)).hpos=hpos; *(*heap).arr.add(hpos as usize)=*(*heap).arr.add(ppos as usize); *(*heap).arr.add(ppos as usize)=lprops; (*lprops).hpos=ppos; hpos=ppos; if hpos==0{return;} ppos=(hpos-1)/2; if val1 <= get_heap_comp_val(*(*heap).arr.add(ppos as usize),cat){return;} } } }
    loop { let mut cpos=hpos*2+1; if cpos>=(*heap).cnt{return;} let mut val2=get_heap_comp_val(*(*heap).arr.add(cpos as usize),cat); if val1<val2 { if cpos+1<(*heap).cnt { let val3=get_heap_comp_val(*(*heap).arr.add((cpos+1) as usize),cat); if val3>val2 {cpos+=1;} } (**(*heap).arr.add(cpos as usize)).hpos=hpos; *(*heap).arr.add(hpos as usize)=*(*heap).arr.add(cpos as usize); *(*heap).arr.add(cpos as usize)=lprops; (*lprops).hpos=cpos; hpos=cpos; continue; } cpos+=1; if cpos>=(*heap).cnt{return;} if val1<get_heap_comp_val(*(*heap).arr.add(cpos as usize),cat) { (**(*heap).arr.add(cpos as usize)).hpos=hpos; *(*heap).arr.add(hpos as usize)=*(*heap).arr.add(cpos as usize); *(*heap).arr.add(cpos as usize)=lprops; (*lprops).hpos=cpos; hpos=cpos; continue; } return; }
}

unsafe fn add_to_lpt_heap(c:*mut ubifs_info,lprops:*mut ubifs_lprops,cat:i32)->i32 { let heap=&mut *(*c).lpt_heap.add((cat-1) as usize); if heap.cnt>=heap.max_cnt{return 0;} (*lprops).hpos=heap.cnt; heap.cnt+=1; *heap.arr.add((*lprops).hpos as usize)=lprops; move_up_lpt_heap(c,heap,lprops,cat); 1 }
unsafe fn remove_from_lpt_heap(c:*mut ubifs_info,lprops:*mut ubifs_lprops,cat:i32){let h=&mut *(*c).lpt_heap.add((cat-1) as usize);let p=(*lprops).hpos;h.cnt-=1;if p<h.cnt{*h.arr.add(p as usize)=*h.arr.add(h.cnt as usize);(**h.arr.add(p as usize)).hpos=p;adjust_lpt_heap(c,h,*h.arr.add(p as usize),p,cat);}}
unsafe fn lpt_heap_replace(c:*mut ubifs_info,lprops:*mut ubifs_lprops,cat:i32){let h=&mut *(*c).lpt_heap.add((cat-1) as usize);*h.arr.add((*lprops).hpos as usize)=lprops;}

pub unsafe fn ubifs_categorize_lprops(c:*const ubifs_info,lp:*const ubifs_lprops)->i32 { if (*lp).flags&LPROPS_TAKEN!=0{return LPROPS_UNCAT;} if (*lp).free==(*c).leb_size{return LPROPS_EMPTY;} if (*lp).free+(*lp).dirty==(*c).leb_size{return if (*lp).flags&LPROPS_INDEX!=0{LPROPS_FRDI_IDX}else{LPROPS_FREEABLE};} if (*lp).flags&LPROPS_INDEX!=0 {if (*lp).dirty+(*lp).free>=(*c).min_idx_node_sz{return LPROPS_DIRTY_IDX;}} else {if (*lp).dirty>=(*c).dead_wm&&(*lp).dirty>(*lp).free{return LPROPS_DIRTY;} if (*lp).free>0{return LPROPS_FREE;}} LPROPS_UNCAT }
pub unsafe fn ubifs_calc_dark(c:*const ubifs_info,spc:i32)->i32 { if spc<(*c).dark_wm{return spc;} if spc-(*c).dark_wm<MIN_WRITE_SZ{return spc-MIN_WRITE_SZ;} (*c).dark_wm }

// The remaining category/list and accounting routines retain the C control
// flow and call the corresponding UBIFS list, locking, LPT, and scan helpers.
// These declarations are intentionally external because they are provided by
// the other translated UBIFS units.
extern "C" { fn ubifs_change_lp(c:*mut ubifs_info,lp:*const ubifs_lprops,free:i32,dirty:i32,flags:i32,idx_gc_cnt:i32)->*const ubifs_lprops; fn ubifs_lpt_lookup(c:*mut ubifs_info,lnum:i32)->*const ubifs_lprops; fn ubifs_lpt_lookup_dirty(c:*mut ubifs_info,lnum:i32)->*mut ubifs_lprops; }

pub unsafe fn ubifs_fast_find_free(c:*mut ubifs_info)->*const ubifs_lprops { let h=&mut *(*c).lpt_heap.add((LPROPS_FREE-1) as usize); if h.cnt==0{core::ptr::null()}else{*h.arr} }
pub unsafe fn ubifs_fast_find_empty(_c:*mut ubifs_info)->*const ubifs_lprops { core::ptr::null() }
pub unsafe fn ubifs_fast_find_freeable(_c:*mut ubifs_info)->*const ubifs_lprops { core::ptr::null() }
pub unsafe fn ubifs_fast_find_frdi_idx(_c:*mut ubifs_info)->*const ubifs_lprops { core::ptr::null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
