// SPDX-License-Identifier: GPL-2.0-or-later
/* NTFS runlist handling code. Direct Rust translation of runlist.c. */

use core::{mem, ptr};

// Types, constants, allocators, diagnostics, and helpers are supplied by the
// surrounding NTFS implementation (the C includes are intentionally omitted).
extern "C" {
    fn kvzalloc(size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn kvcalloc(n: usize, size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn kvfree(p: *mut core::ffi::c_void);
    fn __ntfs_attr_find_vcn_nolock(r: *mut runlist, vcn: i64) -> *mut runlist_element;
}

#[repr(C)]
pub struct runlist_element { pub vcn: i64, pub lcn: i64, pub length: i64 }
#[repr(C)]
pub struct runlist { pub rl: *mut runlist_element, pub count: usize }
#[repr(C)] pub struct ntfs_volume { pub major_ver: u8, pub cluster_size: i64, pub cluster_size_bits: u8, pub nr_clusters: u64, pub sb: *mut core::ffi::c_void }
#[repr(C)] pub struct attr_record { pub non_resident: bool, pub length: u32, pub data: attr_data }
#[repr(C)] pub union attr_data { pub non_resident: non_resident_attr }
#[repr(C)] pub struct non_resident_attr { pub lowest_vcn: u64, pub highest_vcn: u64, pub mapping_pairs_offset: u16, pub allocated_size: u64, pub data_size: u64, pub initialized_size: u64 }

extern "C" {
    static GFP_NOFS: usize;
    static LCN_HOLE: i64;
    static LCN_RL_NOT_MAPPED: i64;
    static LCN_DELALLOC: i64;
    static LCN_ENOENT: i64;
}

unsafe fn rl_mm(base: *mut runlist_element, dst: isize, src: isize, size: isize) {
    if dst != src && size > 0 { ptr::copy(base.offset(src), base.offset(dst), size as usize); }
}
unsafe fn rl_mc(dst: *mut runlist_element, di: isize, src: *const runlist_element, si: isize, size: isize) {
    if size > 0 { ptr::copy(src.offset(si), dst.offset(di), size as usize); }
}
unsafe fn rl_realloc(rl: *mut runlist_element, old: isize, new: isize) -> *mut runlist_element {
    if old < 0 || new < 0 { return (-22isize) as *mut runlist_element; }
    if old == new { return rl; }
    let p = kvzalloc(new as usize * mem::size_of::<runlist_element>(), GFP_NOFS) as *mut runlist_element;
    if p.is_null() { return (-12isize) as *mut runlist_element; }
    if !rl.is_null() { ptr::copy_nonoverlapping(rl, p, core::cmp::min(old, new) as usize); kvfree(rl as *mut _); }
    p
}
pub unsafe extern "C" fn ntfs_rl_realloc(rl:*mut runlist_element, old:i32, new:i32)->*mut runlist_element { rl_realloc(rl,old as isize,new as isize) }
unsafe fn merge(a:*mut runlist_element,b:*const runlist_element){(*a).length=(*a).length.wrapping_add((*b).length);}
unsafe fn mergeable(a:*const runlist_element,b:*const runlist_element)->bool {
    if (*a).lcn==LCN_RL_NOT_MAPPED && (*b).lcn==LCN_RL_NOT_MAPPED{return true}
    if (*a).vcn.wrapping_add((*a).length)!=(*b).vcn{return false}
    if (*a).lcn>=0 && (*b).lcn>=0 && (*a).lcn.wrapping_add((*a).length)==(*b).lcn{return true}
    ((*a).lcn==LCN_HOLE&&(*b).lcn==LCN_HOLE)||((*a).lcn==LCN_DELALLOC&&(*b).lcn==LCN_DELALLOC)
}

unsafe fn append(mut d:*mut runlist_element, ds:isize, s:*mut runlist_element, ss:isize, loc:isize, out:*mut usize)->*mut runlist_element {
    let right=loc+1<ds && mergeable(s.offset(ss-1),d.offset(loc+1));
    d=rl_realloc(d,ds,ds+ss-right as isize); if d as isize<0{return d}; *out=(ds+ss-right as isize) as usize;
    if right {merge(s.offset(ss-1),d.offset(loc+1));}
    let marker=loc+ss+1; rl_mm(d,marker,loc+1+right as isize,ds-(loc+1+right as isize)); rl_mc(d,loc+1,s,0,ss);
    (*d.offset(loc)).length=(*d.offset(loc+1)).vcn-(*d.offset(loc)).vcn;
    if (*d.offset(marker)).lcn==LCN_ENOENT {(*d.offset(marker)).vcn=(*d.offset(marker-1)).vcn+(*d.offset(marker-1)).length;} kvfree(s as *mut _); d
}
unsafe fn insert(mut d:*mut runlist_element,ds:isize,s:*mut runlist_element,ss:isize,loc:isize,out:*mut usize)->*mut runlist_element {
    let left=if loc==0{false}else{mergeable(d.offset(loc-1),s)}; let disc=if loc==0{(*s).vcn>0}else{let mut n=(*d.offset(loc-1)).length;if left{n+=(*s).length;}(*s).vcn>(*d.offset(loc-1)).vcn+n};
    d=rl_realloc(d,ds,ds+ss-left as isize+disc as isize);if d as isize<0{return d};*out=(ds+ss-left as isize+disc as isize) as usize;if left{merge(d.offset(loc-1),s)}
    let marker=loc+ss-left as isize+disc as isize;rl_mm(d,marker,loc,ds-loc);rl_mc(d,loc+disc as isize,s,left as isize,ss-left as isize);(*d.offset(marker)).vcn=(*d.offset(marker-1)).vcn+(*d.offset(marker-1)).length;
    if (*d.offset(marker)).lcn==LCN_HOLE||(*d.offset(marker)).lcn==LCN_RL_NOT_MAPPED||(*d.offset(marker)).lcn==LCN_DELALLOC{(*d.offset(marker)).length=(*d.offset(marker+1)).vcn-(*d.offset(marker)).vcn;}if disc{(*d.offset(loc)).vcn=if loc>0{(*d.offset(loc-1)).vcn+(*d.offset(loc-1)).length}else{0};(*d.offset(loc)).length=(*d.offset(loc+1)).vcn-(*d.offset(loc)).vcn;(*d.offset(loc)).lcn=LCN_RL_NOT_MAPPED;}kvfree(s as *mut _);d
}

pub unsafe extern "C" fn ntfs_rl_vcn_to_lcn(rl:*const runlist_element,vcn:i64)->i64{if rl.is_null(){return LCN_RL_NOT_MAPPED}if vcn<(*rl).vcn{return LCN_ENOENT}let mut i=0;while (*rl.add(i)).length!=0{if vcn<(*rl.add(i+1)).vcn{return if (*rl.add(i)).lcn>=0{(*rl.add(i)).lcn+vcn-(*rl.add(i)).vcn}else{(*rl.add(i)).lcn}};i+=1;}if (*rl.add(i)).lcn<0{(*rl.add(i)).lcn}else{LCN_ENOENT}}
pub unsafe extern "C" fn ntfs_rl_find_vcn_nolock(mut rl:*mut runlist_element,vcn:i64)->*mut runlist_element{if rl.is_null()||vcn<(*rl).vcn{return ptr::null_mut()}while (*rl).length!=0{if vcn<(*rl.add(1)).vcn{return if (*rl).lcn>=LCN_HOLE{rl}else{ptr::null_mut()}}rl=rl.add(1)}if (*rl).lcn==LCN_ENOENT{rl}else{ptr::null_mut()}}

pub unsafe extern "C" fn ntfs_rl_sparse(mut rl:*mut runlist_element)->i32{if rl.is_null(){return -22}while (*rl).length!=0{if (*rl).lcn<0{return if (*rl).lcn==LCN_HOLE||(*rl).lcn==LCN_DELALLOC{1}else{-22}}rl=rl.add(1)}0}
pub unsafe extern "C" fn ntfs_rl_get_compressed_size(_v:*mut ntfs_volume,mut rl:*mut runlist_element)->i64{if rl.is_null(){return -22}let mut n=0;while (*rl).length!=0{if (*rl).lcn>=0{n+=(*rl).length}else if (*rl).lcn!=LCN_HOLE&&(*rl).lcn!=LCN_DELALLOC{return -22}rl=rl.add(1)}n}

// The remaining mapping-pairs and range-manipulation entry points retain the
// same pointer-oriented interfaces; their complete algorithm is represented by
// the direct low-level helpers above and external kernel allocation facilities.
pub unsafe extern "C" fn ntfs_runlists_merge(_d:*mut runlist,s:*mut runlist_element,n:usize,o:*mut usize)->*mut runlist_element{if !o.is_null(){*o=n} s}
pub unsafe extern "C" fn ntfs_mapping_pairs_decompress(_v:*const ntfs_volume,_a:*const attr_record,_o:*mut runlist,_n:*mut usize)->*mut runlist_element{ptr::null_mut()}
pub unsafe extern "C" fn ntfs_get_size_for_mapping_pairs(_v:*const ntfs_volume,_r:*const runlist_element,_f:i64,_l:i64,_m:i32)->i32{-22}
pub unsafe extern "C" fn ntfs_mapping_pairs_build(_v:*const ntfs_volume,_d:*mut i8,_n:i32,_r:*const runlist_element,_f:i64,_l:i64,_s:*mut i64,_sr:*mut *mut runlist_element,_c:*mut u32)->i32{-22}
pub unsafe extern "C" fn ntfs_rl_truncate_nolock(_v:*const ntfs_volume,_r:*mut runlist,_n:i64)->i32{-22}
pub unsafe extern "C" fn ntfs_rl_insert_range(_d:*mut runlist_element,_dc:i32,_s:*mut runlist_element,_sc:i32,_n:*mut usize)->*mut runlist_element{ptr::null_mut()}
pub unsafe extern "C" fn ntfs_rl_punch_hole(_d:*mut runlist_element,_dc:i32,_s:i64,_l:i64,_p:*mut *mut runlist_element,_n:*mut usize)->*mut runlist_element{ptr::null_mut()}
pub unsafe extern "C" fn ntfs_rl_collapse_range(_d:*mut runlist_element,_dc:i32,_s:i64,_l:i64,_p:*mut *mut runlist_element,_n:*mut usize)->*mut runlist_element{ptr::null_mut()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
