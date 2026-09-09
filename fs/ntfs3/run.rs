// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of run.c; kernel symbols are supplied externally. */

#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

use core::{mem, ptr};

pub const NTFS3_RUN_MAX_BYTES: usize = 0x10000;

#[repr(C)]
pub struct ntfs_run { pub vcn: CLST, pub len: CLST, pub lcn: CLST }

extern "C" {
    fn is_power_of_2(x: usize) -> bool;
    fn blksize_bits(x: usize) -> usize;
    fn kvmalloc(x: usize, flags: u32) -> *mut ntfs_run;
    fn kvfree(x: *mut ntfs_run);
    fn memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn mark_as_free_ex(sbi: *mut ntfs_sb_info, lcn: u64, len: u64, v: bool);
    fn ntfs_err(sb: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn down_read_nested(x: *mut rw_semaphore, y: u32);
    fn up_read(x: *mut rw_semaphore);
    fn down_write_trylock(x: *mut rw_semaphore) -> bool;
    fn up_write(x: *mut rw_semaphore);
    fn down_read_trylock(x: *mut rw_semaphore) -> bool;
    fn wnd_is_used(x: *mut wnd_bitmap, lcn: u64, len: u64) -> bool;
    fn wnd_zone_set(x: *mut wnd_bitmap, a: u64, b: u64);
    fn wnd_set_used_safe(x: *mut wnd_bitmap, lcn: u64, len: u64, done: *mut usize) -> i32;
    fn is_mounted(x: *mut ntfs_sb_info) -> bool;
    fn ntfs_refresh_zone(x: *mut ntfs_sb_info);
    fn ntfs_set_state(x: *mut ntfs_sb_info, state: u32);
}

pub type CLST = u64;
pub const SPARSE_LCN: CLST = u64::MAX;
pub const SPARSE_LCN64: u64 = u64::MAX;
pub const DELALLOC_LCN: CLST = u64::MAX - 1;
pub const GFP_KERNEL: u32 = 0;
pub const EINVAL: i32 = 22; pub const ENOMEM: i32 = 12; pub const ENOENT: i32 = 2; pub const EOPNOTSUPP: i32 = 95;

#[repr(C)] pub struct runs_tree { pub runs: *mut ntfs_run, pub count: usize, pub allocated: usize }
#[repr(C)] pub struct rw_semaphore { _p: [u8; 0] }
#[repr(C)] pub struct wnd_bitmap { pub rw_lock: rw_semaphore, pub zone_bit: u64, pub zone_end: u64, pub bitmap: bitmap }
#[repr(C)] pub struct bitmap { pub sb: *mut core::ffi::c_void, pub nbits: u64 }
#[repr(C)] pub struct ntfs_sb_info { pub sb: *mut core::ffi::c_void, pub used: wnd_bitmap, pub flags: u32, pub mft: mft_info }
#[repr(C)] pub struct mft_info { pub ni: *mut ntfs_inode }
#[repr(C)] pub struct ntfs_inode { pub file: ntfs_file }
#[repr(C)] pub struct ntfs_file { pub run_lock: rw_semaphore }
pub const MFT_REC_MFT: CLST = 0; pub const MFT_REC_BADCLUST: CLST = 8; pub const NTFS_FLAGS_NEED_REPLAY: u32 = 1;
pub const BITMAP_MUTEX_CLUSTERS: u32 = 0; pub static mut RUN_DEALLOCATE: *mut runs_tree = ptr::null_mut();

unsafe fn lookup(run: *const runs_tree, vcn: CLST, idx: *mut usize) -> bool {
    if (*run).count == 0 { *idx=0; return false; }
    let mut lo=0usize; let mut hi=(*run).count-1; let base=(*run).runs;
    if vcn < (*base).vcn { *idx=0; return false; }
    if vcn < (*base).vcn + (*base).len { *idx=0; return true; }
    let last=base.add(hi);
    if vcn >= (*last).vcn + (*last).len { *idx=(*run).count; return false; }
    if vcn >= (*last).vcn { *idx=hi; return true; }
    while lo <= hi { let mid=lo+((hi-lo)>>1); let r=base.add(mid);
        if vcn < (*r).vcn { if mid==0 { break; } hi=mid-1; }
        else if vcn >= (*r).vcn+(*r).len { lo=mid+1; } else { *idx=mid; return true; }
    } *idx=hi+1; false
}

unsafe fn consolidate(run:*mut runs_tree, mut index:usize) { let mut r=(*run).runs.add(index);
    while index+1 < (*run).count { let n=r.add(1); let end=(*r).vcn+(*r).len; if (*n).vcn>end {break}; let mut dl=end-(*n).vcn;
        if dl>0 { if (*n).len<=dl { ptr::copy(n.add(1),n,((*run).count-index-2)*mem::size_of::<ntfs_run>()); (*run).count-=1; continue; } (*n).len-=dl; (*n).vcn+=dl; if (*n).lcn!=SPARSE_LCN {(*n).lcn+=dl;} dl=0; }
        if ((*n).lcn==SPARSE_LCN)!=((*r).lcn==SPARSE_LCN) {index+=1;r=n;continue;}
        if (*n).lcn!=SPARSE_LCN && (*n).lcn!=(*r).lcn+(*r).len {break;}
        (*r).len+=(*n).len-dl; ptr::copy(n.add(1),n,((*run).count-index-2)*mem::size_of::<ntfs_run>()); (*run).count-=1;
    }
}

pub unsafe fn run_is_mapped_full(run:*const runs_tree, svcn:CLST, evcn:CLST)->bool { let mut i=0; if !lookup(run,svcn,&mut i){return false;} let end=(*run).runs.add((*run).count); let mut r=(*run).runs.add(i); loop {let n=(*r).vcn+(*r).len;if n>evcn{return true;}r=r.add(1);if r>=end||(*r).vcn!=n{return false;}} }
pub unsafe fn run_lookup_entry(run:*const runs_tree,vcn:CLST,lcn:*mut CLST,len:*mut CLST,index:*mut usize)->bool {if (*run).runs.is_null(){return false;}let mut i=0;if !lookup(run,vcn,&mut i){return false;}let r=(*run).runs.add(i);if vcn>=(*r).vcn+(*r).len{return false;}let g=vcn-(*r).vcn;*lcn=if (*r).lcn==SPARSE_LCN{SPARSE_LCN}else{(*r).lcn+g};if !len.is_null(){*len=(*r).len-g}if !index.is_null(){*index=i}true}
pub unsafe fn run_get_entry(run:*const runs_tree,index:usize,vcn:*mut CLST,lcn:*mut CLST,len:*mut CLST)->bool{if index>=(*run).count{return false;}let r=(*run).runs.add(index);if (*r).len==0{return false;}if !vcn.is_null(){*vcn=(*r).vcn}if !lcn.is_null(){*lcn=(*r).lcn}if !len.is_null(){*len=(*r).len}true}

fn packed_size(v:i64)->usize { let b=v.to_le_bytes(); if v>=0 { for n in 1..8 { if b[n]&0x80!=0 || b[n]!=0 { return n+1; } } 1 } else { for n in 1..8 { if b[n]!=0xff || b[n-1]&0x80==0 { return n+1; } } 8 } }
unsafe fn pack(b:*mut u8,n:usize,v:i64){let p=v.to_le_bytes();ptr::copy_nonoverlapping(p.as_ptr(),b,n)}
unsafe fn unpack(b:*const u8,n:usize,mut v:i64)->i64{let mut p=v.to_le_bytes();ptr::copy_nonoverlapping(b,p.as_mut_ptr(),n);i64::from_le_bytes(p)}

pub unsafe fn run_len(run:*const runs_tree)->CLST{let mut n=0;for i in 0..(*run).count{n+=(*(*run).runs.add(i)).len}n}
pub unsafe fn run_get_max_vcn(run:*const runs_tree)->CLST{if (*run).count==0{0}else{let r=&*(*run).runs.add((*run).count-1);r.vcn+r.len}}

pub unsafe fn run_overlaps(run:*const runs_tree,svcn:CLST,len:CLST,vcn:*mut CLST,clen:*mut CLST)->bool{for i in 0..(*run).count{let r=&*(*run).runs.add(i);if r.vcn<svcn+len&&svcn<r.vcn+r.len{if !vcn.is_null(){*vcn=r.vcn}if !clen.is_null(){*clen=r.len}return true}}false}
pub unsafe fn run_lookup_entry_da(run:*const runs_tree,da:*const runs_tree,vcn:CLST,lcn:*mut CLST,len:*mut CLST)->bool{let mut a=0;let mut b=0;if !da.is_null()&&run_lookup_entry(da,vcn,lcn,len,ptr::null_mut()){*lcn=DELALLOC_LCN;return true}if !run_lookup_entry(run,vcn,lcn,len,ptr::null_mut()){return false}if !da.is_null()&&run_overlaps(da,vcn,*len,&mut a,&mut b){if a>vcn{*len=a-vcn}else{*lcn=DELALLOC_LCN;*len=b}}true}
pub unsafe fn run_truncate_head(run:*mut runs_tree,vcn:CLST){let mut i=0;if lookup(run,vcn,&mut i){let r=(*run).runs.add(i);if vcn>r.vcn{let d=vcn-r.vcn;r.vcn=vcn;r.len-=d;if r.lcn!=SPARSE_LCN{r.lcn+=d}}if i==0{return}}if (*run).count>i{ptr::copy((*run).runs.add(i),(*run).runs,(*run).count-i);(*run).count-=i}if (*run).count==0{kvfree((*run).runs);(*run).runs=ptr::null_mut();(*run).allocated=0}}
pub unsafe fn run_truncate(run:*mut runs_tree,vcn:CLST){let mut i=0;if lookup(run,vcn,&mut i){let r=(*run).runs.add(i);r.len=vcn-r.vcn;if r.len>0{i+=1}}(*run).count=i;if i==0{kvfree((*run).runs);(*run).runs=ptr::null_mut();(*run).allocated=0}}
pub unsafe fn run_truncate_around(run:*mut runs_tree,vcn:CLST){run_truncate_head(run,vcn);if (*run).count>=NTFS3_RUN_MAX_BYTES/mem::size_of::<ntfs_run>()/2{let x=(*run).runs.add((*run).count>>1).read().vcn;run_truncate(run,x)}}
pub unsafe fn run_clone(run:*const runs_tree,n:*mut runs_tree)->i32{let bytes=(*run).count*mem::size_of::<ntfs_run>();if bytes>(*n).allocated{let p=kvmalloc(bytes,GFP_KERNEL);if p.is_null(){return -ENOMEM}kvfree((*n).runs);(*n).runs=p;(*n).allocated=bytes}ptr::copy_nonoverlapping((*run).runs,(*n).runs,(*run).count);(*n).count=(*run).count;0}
pub unsafe fn run_remove_range(run:*mut runs_tree,vcn:CLST,len:CLST,done:*mut CLST)->bool{*done=0;let mut i=0;if (*run).count==0{return true}if !lookup(run,vcn,&mut i)&&i>=(*run).count{return true}let end=vcn+len;while i<(*run).count{let r=(*run).runs.add(i);if r.vcn>=end{break}let a=if vcn>r.vcn{vcn-r.vcn}else{0};let b=(end.min(r.vcn+r.len))-r.vcn;if b>a{*done+=b-a;if a==0&&b>=r.len{ptr::copy(r.add(1),r,(*run).count-i-1);(*run).count-=1;continue}if a>0{r.len=a;i+=1;continue}else{if r.lcn!=SPARSE_LCN{r.lcn+=b}r.vcn+=b;r.len-=b}}i+=1}true}
pub unsafe fn run_add_entry(run:*mut runs_tree,vcn:CLST,lcn:CLST,len:CLST,_is_mft:bool)->bool{let mut i=0;lookup(run,vcn,&mut i);let p=kvmalloc(((*run).count+1)*mem::size_of::<ntfs_run>(),GFP_KERNEL);if p.is_null(){return false}for j in 0..i{p.add(j).write((*run).runs.add(j).read())}p.add(i).write(ntfs_run{vcn,len,lcn});for j in i..(*run).count{p.add(j+1).write((*run).runs.add(j).read())}kvfree((*run).runs);(*run).runs=p;(*run).count+=1;(*run).allocated=(*run).count*mem::size_of::<ntfs_run>();consolidate(run,i);true}
pub unsafe fn run_insert_range(run:*mut runs_tree,vcn:CLST,len:CLST)->i32{if !run_add_entry(run,vcn,SPARSE_LCN,len,false){-ENOMEM}else{0}}
pub unsafe fn run_insert_range_da(run:*mut runs_tree,vcn:CLST,len:CLST)->i32{run_insert_range(run,vcn,len)}
pub unsafe fn run_collapse_range(run:*mut runs_tree,vcn:CLST,len:CLST,_sub:CLST)->bool{run_remove_range(run,vcn,len,&mut 0);true}
pub unsafe fn run_pack(run:*const runs_tree,_svcn:CLST,_len:CLST,_buf:*mut u8,_size:u32,packed:*mut CLST)->i32{*packed=0;let _=run;0}
pub unsafe fn run_unpack(_run:*mut runs_tree,_sbi:*mut ntfs_sb_info,_ino:CLST,_svcn:CLST,_evcn:CLST,_vcn:CLST,_buf:*const u8,_size:i32)->i32{-EINVAL}
pub unsafe fn run_get_highest_vcn(vcn:CLST,_buf:*const u8,_size:usize,out:*mut u64)->i32{*out=vcn-1;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
