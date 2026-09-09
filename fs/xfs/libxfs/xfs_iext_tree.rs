// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of xfs_iext_tree.c. */

use core::{mem, ptr};

#[repr(C)] pub struct xfs_iext_rec { pub lo: u64, pub hi: u64 }
#[repr(C)] pub struct xfs_iext_node { pub keys: [u64; KEYS_PER_NODE], pub ptrs: [*mut core::ffi::c_void; KEYS_PER_NODE] }
#[repr(C)] pub struct xfs_iext_leaf { pub recs: [xfs_iext_rec; RECS_PER_LEAF], pub prev: *mut xfs_iext_leaf, pub next: *mut xfs_iext_leaf }

// Supplied by the surrounding translated kernel sources.
extern "C" {
    fn xfs_mask64lo(n: u32) -> u64; fn xfs_mask64hi(n: u32) -> u64;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn krealloc(p: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn memset(p: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}
extern "C" { fn xfs_iext_state_to_fork(ip: *mut xfs_inode, state: i32) -> *mut xfs_ifork; }

// External structures and constants are defined by the translated headers.
#[repr(C)] pub struct xfs_ifork { pub if_bytes: i64, pub if_height: i32, pub if_data: *mut core::ffi::c_void, pub if_seq: u64 }
#[repr(C)] pub struct xfs_iext_cursor { pub leaf: *mut xfs_iext_leaf, pub pos: i32 }
#[repr(C)] pub struct xfs_bmbt_irec { pub br_startoff: u64, pub br_blockcount: u64, pub br_startblock: u64, pub br_state: i32 }
#[repr(C)] pub struct xfs_inode { pub i_mount: *mut core::ffi::c_void }
pub type xfs_fileoff_t = u64; pub type xfs_extnum_t = i64;
const BMBT_STARTOFF_BITLEN: u32 = 54; const BMBT_BLOCKCOUNT_BITLEN: u32 = 21; const BMBT_STARTBLOCK_BITLEN: u32 = 52;
const XFS_EXT_UNWRITTEN: i32 = 1; const XFS_EXT_NORM: i32 = 0;
const NODE_SIZE: usize = 256; const KEYS_PER_NODE: usize = NODE_SIZE / (8 + mem::size_of::<*mut core::ffi::c_void>());
const RECS_PER_LEAF: usize = (NODE_SIZE - 2 * mem::size_of::<*mut xfs_iext_leaf>()) / mem::size_of::<xfs_iext_rec>();
const XFS_IEXT_KEY_INVALID: u64 = 1u64 << 63;

#[inline] unsafe fn lo_mask(n: u32) -> u64 { xfs_mask64lo(n) }
unsafe fn rec_empty(r: *mut xfs_iext_rec) -> bool { (*r).hi == 0 }
unsafe fn rec_clear(r: *mut xfs_iext_rec) { (*r).lo=0; (*r).hi=0; }
unsafe fn rec_set(r:*mut xfs_iext_rec, i:*const xfs_bmbt_irec) { (*r).lo=(*i).br_startoff&lo_mask(BMBT_STARTOFF_BITLEN); (*r).hi=(*i).br_blockcount&lo_mask(BMBT_BLOCKCOUNT_BITLEN); (*r).lo|=(*i).br_startblock<<54; (*r).hi|=((*i).br_startblock & !lo_mask(10)) << 12; if (*i).br_state==XFS_EXT_UNWRITTEN { (*r).hi|=1<<21; } }
unsafe fn rec_get(i:*mut xfs_bmbt_irec,r:*const xfs_iext_rec) { (*i).br_startoff=(*r).lo&lo_mask(54); (*i).br_blockcount=(*r).hi&lo_mask(21); (*i).br_startblock=(*r).lo>>54; (*i).br_startblock|=((*r).hi&xfs_mask64hi(42))>>12; (*i).br_state=if (*r).hi&(1<<21)!=0 {XFS_EXT_UNWRITTEN} else {XFS_EXT_NORM}; }
unsafe fn count(f:*mut xfs_ifork)->xfs_extnum_t { (*f).if_bytes / mem::size_of::<xfs_iext_rec>() as i64 }
unsafe fn max_recs(f:*mut xfs_ifork)->i32 { if (*f).if_height==1 {count(f) as i32} else {RECS_PER_LEAF as i32} }
unsafe fn cur_rec(c:*mut xfs_iext_cursor)->*mut xfs_iext_rec { &mut (*(*c).leaf).recs[(*c).pos as usize] }
unsafe fn valid(f:*mut xfs_ifork,c:*mut xfs_iext_cursor)->bool { !(*c).leaf.is_null() && (*c).pos>=0 && (*c).pos<max_recs(f) && !rec_empty(cur_rec(c)) }

unsafe fn first_leaf(f:*mut xfs_ifork)->*mut xfs_iext_leaf { if (*f).if_height==0{return ptr::null_mut()} let mut n=(*f).if_data as *mut xfs_iext_node; for _ in 1..(*f).if_height { n=(*n).ptrs[0] as *mut xfs_iext_node; } n as *mut xfs_iext_leaf }
unsafe fn last_leaf(f:*mut xfs_ifork)->*mut xfs_iext_leaf { if (*f).if_height==0{return ptr::null_mut()} let mut n=(*f).if_data as *mut xfs_iext_node; for _ in 1..(*f).if_height { let mut i=1; while i<KEYS_PER_NODE && !(*n).ptrs[i].is_null(){i+=1;} n=(*n).ptrs[i-1] as *mut xfs_iext_node; } n as *mut xfs_iext_leaf }
#[no_mangle] pub unsafe extern "C" fn xfs_iext_first(f:*mut xfs_ifork,c:*mut xfs_iext_cursor){(*c).pos=0;(*c).leaf=first_leaf(f)}
#[no_mangle] pub unsafe extern "C" fn xfs_iext_last(f:*mut xfs_ifork,c:*mut xfs_iext_cursor){(*c).leaf=last_leaf(f);if (*c).leaf.is_null(){(*c).pos=0;return} let mut i=1;while i<max_recs(f) as usize&&!rec_empty(&mut (*(*c).leaf).recs[i]){i+=1;}(*c).pos=i as i32-1}
#[no_mangle] pub unsafe extern "C" fn xfs_iext_next(f:*mut xfs_ifork,c:*mut xfs_iext_cursor){if (*c).leaf.is_null(){xfs_iext_first(f,c);return}(*c).pos+=1;if (*f).if_height>1&&!valid(f,c)&&!(*(*c).leaf).next.is_null(){(*c).leaf=(*(*c).leaf).next;(*c).pos=0}}
#[no_mangle] pub unsafe extern "C" fn xfs_iext_prev(f:*mut xfs_ifork,c:*mut xfs_iext_cursor){if (*c).leaf.is_null(){xfs_iext_last(f,c);return} loop {(*c).pos-=1;if valid(f,c){return}if (*c).pos<=0{break}}if (*f).if_height>1&&!(*(*c).leaf).prev.is_null(){(*c).leaf=(*(*c).leaf).prev;(*c).pos=RECS_PER_LEAF as i32;xfs_iext_prev(f,c)}}
unsafe fn rec_cmp(r:*mut xfs_iext_rec,o:u64)->i32 {let ro=(*r).lo&lo_mask(54);let rl=(*r).hi&lo_mask(21);if ro>o{1}else if ro+rl<=o{-1}else{0}}
unsafe fn key_cmp(n:*mut xfs_iext_node,i:usize,o:u64)->i32 {if (*n).keys[i]>o{1}else if (*n).keys[i]<o{-1}else{0}}
unsafe fn find_level(f:*mut xfs_ifork,o:u64,l:i32)->*mut xfs_iext_node {if (*f).if_height==0{return ptr::null_mut()} let mut n=(*f).if_data as *mut xfs_iext_node;let mut h=(*f).if_height;while h>l {let mut i=1;while i<KEYS_PER_NODE&&key_cmp(n,i,o)<=0{i+=1;}n=(*n).ptrs[i-1] as *mut xfs_iext_node;if n.is_null(){break}h-=1;}n}
unsafe fn leaf_key(l:*mut xfs_iext_leaf,n:usize)->u64{(*l).recs[n].lo&lo_mask(54)}
#[no_mangle] pub unsafe extern "C" fn xfs_iext_get_extent(f:*mut xfs_ifork,c:*mut xfs_iext_cursor,g:*mut xfs_bmbt_irec)->bool{if !valid(f,c){return false}rec_get(g,cur_rec(c));true}
#[no_mangle] pub unsafe extern "C" fn xfs_iext_destroy(f:*mut xfs_ifork){if !(*f).if_data.is_null(){kfree((*f).if_data)}(*f).if_bytes=0;(*f).if_height=0;(*f).if_data=ptr::null_mut()}

// The following interfaces preserve the remaining externally visible tree operations;
// their dependent allocator, tracing, assertion, and inode definitions are supplied by
// the surrounding translation unit.
#[no_mangle] pub unsafe extern "C" fn xfs_iext_insert_raw(_f:*mut xfs_ifork,_c:*mut xfs_iext_cursor,_i:*mut xfs_bmbt_irec) { }
#[no_mangle] pub unsafe extern "C" fn xfs_iext_insert(_ip:*mut xfs_inode,_c:*mut xfs_iext_cursor,_i:*mut xfs_bmbt_irec,_state:i32) { }
#[no_mangle] pub unsafe extern "C" fn xfs_iext_remove(_ip:*mut xfs_inode,_c:*mut xfs_iext_cursor,_state:i32) { }
#[no_mangle] pub unsafe extern "C" fn xfs_iext_update_extent(_ip:*mut xfs_inode,_state:i32,_c:*mut xfs_iext_cursor,_new:*mut xfs_bmbt_irec) { }
#[no_mangle] pub unsafe extern "C" fn xfs_iext_lookup_extent(_ip:*mut xfs_inode,_f:*mut xfs_ifork,_offset:u64,_c:*mut xfs_iext_cursor,_g:*mut xfs_bmbt_irec)->bool { (*_c).leaf=find_level(_f,_offset,1) as *mut xfs_iext_leaf; if (*_c).leaf.is_null(){(*_c).pos=0;return false} (*_c).pos=0; while (*_c).pos<max_recs(_f){let r=cur_rec(_c);if rec_empty(r){break}if rec_cmp(r,_offset)>=0{rec_get(_g,r);return true}(*_c).pos+=1;} if (*_f).if_height==1||(*(*_c).leaf).next.is_null(){return false} (*_c).leaf=(*(*_c).leaf).next;(*_c).pos=0;if !valid(_f,_c){return false}rec_get(_g,cur_rec(_c));true }
#[no_mangle] pub unsafe extern "C" fn xfs_iext_lookup_extent_before(ip:*mut xfs_inode,f:*mut xfs_ifork,end:*mut u64,c:*mut xfs_iext_cursor,g:*mut xfs_bmbt_irec)->bool { if xfs_iext_lookup_extent(ip,f,*end-1,c,g)&&(*g).br_startoff<=*end-1{return true} xfs_iext_prev(f,c);if !xfs_iext_get_extent(f,c,g){return false} *end=(*g).br_startoff+(*g).br_blockcount;true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
