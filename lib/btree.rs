// SPDX-License-Identifier: GPL-2.0-only
/*
 * lib/btree.c - Simple In-memory B+Tree
 *
 * This is a source-level Rust translation of the corresponding C implementation.
 * Linux kernel dependencies remain external declarations/usages.
 */

use core::mem::size_of;

#[repr(C)]
pub struct btree_geo {
    pub keylen: i32,
    pub no_pairs: i32,
    pub no_longs: i32,
}

extern "C" {
    static mut btree_cachep: *mut kmem_cache;
    fn kmem_cache_alloc(cache: *mut kmem_cache, gfp: gfp_t) -> *mut core::ffi::c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, element: *mut core::ffi::c_void);
    fn mempool_alloc(pool: *mut mempool_t, gfp: gfp_t) -> *mut core::ffi::c_void;
    fn mempool_free(element: *mut core::ffi::c_void, pool: *mut mempool_t);
    fn mempool_create(
        min_nr: u32,
        alloc_fn: unsafe extern "C" fn(gfp_t, *mut core::ffi::c_void) -> *mut core::ffi::c_void,
        free_fn: unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
        pool_data: *mut core::ffi::c_void,
    ) -> *mut mempool_t;
    fn mempool_destroy(pool: *mut mempool_t);
    fn kmem_cache_create(name: *const i8, size: usize, align: usize, flags: u32, ctor: *mut core::ffi::c_void) -> *mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn BUG_ON(condition: bool);
}

pub type gfp_t = u32;
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct mempool_t { _private: [u8; 0] }

#[repr(C)]
pub struct btree_head {
    pub node: *mut usize,
    pub height: i32,
    pub mempool: *mut mempool_t,
}

const L1_CACHE_BYTES: usize = 128;
const NODESIZE: usize = if L1_CACHE_BYTES > 128 { L1_CACHE_BYTES } else { 128 };
const BITS_PER_LONG: usize = usize::BITS as usize;
const LONG_PER_U64: usize = 64 / BITS_PER_LONG;
const MAX_KEYLEN: usize = 2 * LONG_PER_U64;

pub static mut btree_geo32: btree_geo = btree_geo { keylen: 1, no_pairs: (NODESIZE / size_of::<usize>() / 2) as i32, no_longs: (NODESIZE / size_of::<usize>() / 2) as i32 };
pub static mut btree_geo64: btree_geo = btree_geo { keylen: LONG_PER_U64 as i32, no_pairs: (NODESIZE / size_of::<usize>() / (1 + LONG_PER_U64)) as i32, no_longs: (LONG_PER_U64 * (NODESIZE / size_of::<usize>() / (1 + LONG_PER_U64))) as i32 };
pub static mut btree_geo128: btree_geo = btree_geo { keylen: (2 * LONG_PER_U64) as i32, no_pairs: (NODESIZE / size_of::<usize>() / (1 + 2 * LONG_PER_U64)) as i32, no_longs: (2 * LONG_PER_U64 * (NODESIZE / size_of::<usize>() / (1 + 2 * LONG_PER_U64))) as i32 };

pub unsafe extern "C" fn btree_alloc(gfp_mask: gfp_t, _pool_data: *mut core::ffi::c_void) -> *mut core::ffi::c_void { kmem_cache_alloc(btree_cachep, gfp_mask) }
pub unsafe extern "C" fn btree_free(element: *mut core::ffi::c_void, _pool_data: *mut core::ffi::c_void) { kmem_cache_free(btree_cachep, element); }

unsafe fn btree_node_alloc(head: *mut btree_head, gfp: gfp_t) -> *mut usize {
    let node = mempool_alloc((*head).mempool, gfp) as *mut usize;
    if !node.is_null() { memset(node as *mut _, 0, NODESIZE); }
    node
}
unsafe fn longcmp(l1: *const usize, l2: *const usize, n: usize) -> i32 { for i in 0..n { if *l1.add(i) < *l2.add(i) { return -1; } if *l1.add(i) > *l2.add(i) { return 1; } } 0 }
unsafe fn longcpy(dest: *mut usize, src: *const usize, n: usize) -> *mut usize { for i in 0..n { *dest.add(i) = *src.add(i); } dest }
unsafe fn longset(s: *mut usize, c: usize, n: usize) -> *mut usize { for i in 0..n { *s.add(i) = c; } s }
unsafe fn dec_key(geo: *btree_geo, key: *mut usize) { for i in (0..(*geo).keylen as usize).rev() { let val = *key.add(i); *key.add(i) = val.wrapping_sub(1); if val != 0 { break; } } }
unsafe fn bkey(geo: *btree_geo, node: *mut usize, n: i32) -> *mut usize { node.add(n as usize * (*geo).keylen as usize) }
unsafe fn bval(geo: *btree_geo, node: *mut usize, n: i32) -> *mut core::ffi::c_void { *node.add((*geo).no_longs as usize + n as usize) as *mut core::ffi::c_void }
unsafe fn setkey(geo: *btree_geo, node: *mut usize, n: i32, key: *const usize) { longcpy(bkey(geo,node,n), key, (*geo).keylen as usize); }
unsafe fn setval(geo: *btree_geo, node: *mut usize, n: i32, val: *mut core::ffi::c_void) { *node.add((*geo).no_longs as usize+n as usize) = val as usize; }
unsafe fn clearpair(geo: *btree_geo, node: *mut usize, n: i32) { longset(bkey(geo,node,n), 0, (*geo).keylen as usize); *node.add((*geo).no_longs as usize+n as usize)=0; }
unsafe fn __btree_init(head: *mut btree_head) { (*head).node = core::ptr::null_mut(); (*head).height=0; }

pub unsafe extern "C" fn btree_init_mempool(head: *mut btree_head, mempool: *mut mempool_t) { __btree_init(head); (*head).mempool=mempool; }
pub unsafe extern "C" fn btree_init(head: *mut btree_head) -> i32 { __btree_init(head); (*head).mempool=mempool_create(0, btree_alloc, btree_free, core::ptr::null_mut()); if (*head).mempool.is_null() { -12 } else { 0 } }
pub unsafe extern "C" fn btree_destroy(head: *mut btree_head) { mempool_free((*head).node as *mut _, (*head).mempool); mempool_destroy((*head).mempool); (*head).mempool=core::ptr::null_mut(); }

unsafe fn keycmp(geo:*mut btree_geo,node:*mut usize,pos:i32,key:*mut usize)->i32 { longcmp(bkey(geo,node,pos),key,(*geo).keylen as usize) }
unsafe fn keyzero(geo:*mut btree_geo,key:*mut usize)->i32 { for i in 0..(*geo).keylen { if *key.add(i as usize)!=0{return 0;} } 1 }
unsafe fn getpos(geo:*mut btree_geo,node:*mut usize,key:*mut usize)->i32 { for i in 0..(*geo).no_pairs { if keycmp(geo,node,i,key)<=0{return i;} } (*geo).no_pairs }
unsafe fn getfill(geo:*mut btree_geo,node:*mut usize,start:i32)->i32 { for i in start..(*geo).no_pairs { if bval(geo,node,i).is_null(){return i;} } (*geo).no_pairs }

pub unsafe extern "C" fn btree_lookup(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize)->*mut core::ffi::c_void { if (*head).height==0{return core::ptr::null_mut();} let mut node=(*head).node; for _ in 1..(*head).height { let mut i=0; while i<(*geo).no_pairs && keycmp(geo,node,i,key)>0{i+=1;} if i==(*geo).no_pairs{return core::ptr::null_mut();} node=bval(geo,node,i) as *mut usize; if node.is_null(){return core::ptr::null_mut();} } for i in 0..(*geo).no_pairs { if keycmp(geo,node,i,key)==0{return bval(geo,node,i);} } core::ptr::null_mut() }
pub unsafe extern "C" fn btree_update(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize,val:*mut core::ffi::c_void)->i32 { let node=btree_lookup_node(head,geo,key); if node.is_null(){return -2;} for i in 0..(*geo).no_pairs {if keycmp(geo,node,i,key)==0{setval(geo,node,i,val);return 0;}} -2 }
unsafe fn btree_lookup_node(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize)->*mut usize { if (*head).height==0{return core::ptr::null_mut();} let mut node=(*head).node; for _ in 1..(*head).height { let mut i=0; while i<(*geo).no_pairs && keycmp(geo,node,i,key)>0{i+=1;} if i==(*geo).no_pairs{return core::ptr::null_mut();} node=bval(geo,node,i) as *mut usize; if node.is_null(){return node;} } node }

unsafe fn find_level(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize,level:i32)->*mut usize { let mut node=(*head).node; for _ in level..(*head).height { let mut i=0; while i<(*geo).no_pairs && keycmp(geo,node,i,key)>0{i+=1;} if i==(*geo).no_pairs || bval(geo,node,i).is_null(){i-=1;setkey(geo,node,i,key);} BUG_ON(i<0); node=bval(geo,node,i) as *mut usize;} BUG_ON(node.is_null()); node }

unsafe fn btree_grow(head:*mut btree_head,geo:*mut btree_geo,gfp:gfp_t)->i32 { let node=btree_node_alloc(head,gfp); if node.is_null(){return -12;} if !(*head).node.is_null(){let fill=getfill(geo,(*head).node,0);setkey(geo,node,0,bkey(geo,(*head).node,fill-1));setval(geo,node,0,(*head).node as *mut _);} (*head).node=node;(*head).height+=1;0 }
unsafe fn btree_shrink(head:*mut btree_head,geo:*mut btree_geo){if (*head).height<=1{return;}let node=(*head).node;let fill=getfill(geo,node,0);BUG_ON(fill>1);(*head).node=bval(geo,node,0) as *mut usize;(*head).height-=1;mempool_free(node as *mut _,(*head).mempool);}

unsafe fn btree_insert_level(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize,val:*mut core::ffi::c_void,level:i32,gfp:gfp_t)->i32 { BUG_ON(val.is_null()); if (*head).height<level {let err=btree_grow(head,geo,gfp);if err!=0{return err;}} loop {let node=find_level(head,geo,key,level);let pos=getpos(geo,node,key);let fill=getfill(geo,node,pos);BUG_ON(pos<fill&&keycmp(geo,node,pos,key)==0);if fill==(*geo).no_pairs {let new=btree_node_alloc(head,gfp);if new.is_null(){return -12;}let err=btree_insert_level(head,geo,bkey(geo,node,fill/2-1),new as *mut _,level+1,gfp);if err!=0{mempool_free(new as *mut _,(*head).mempool);return err;}for i in 0..fill/2 {setkey(geo,new,i,bkey(geo,node,i));setval(geo,new,i,bval(geo,node,i));setkey(geo,node,i,bkey(geo,node,i+fill/2));setval(geo,node,i,bval(geo,node,i+fill/2));clearpair(geo,node,i+fill/2);}if fill&1!=0 {setkey(geo,node,fill/2,bkey(geo,node,fill-1));setval(geo,node,fill/2,bval(geo,node,fill-1));clearpair(geo,node,fill-1);}continue;}for i in (pos..fill).rev(){setkey(geo,node,i+1,bkey(geo,node,i));setval(geo,node,i+1,bval(geo,node,i));}setkey(geo,node,pos,key);setval(geo,node,pos,val);return 0;}}
pub unsafe extern "C" fn btree_insert(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize,val:*mut core::ffi::c_void,gfp:gfp_t)->i32 {btree_insert_level(head,geo,key,val,1,gfp)}

unsafe fn btree_remove_level(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize,level:i32)->*mut core::ffi::c_void {if level>(*head).height{(*head).height=0;(*head).node=core::ptr::null_mut();return core::ptr::null_mut();}let node=find_level(head,geo,key,level);let pos=getpos(geo,node,key);let fill=getfill(geo,node,pos);if level==1&&keycmp(geo,node,pos,key)!=0{return core::ptr::null_mut();}let ret=bval(geo,node,pos);for i in pos..fill-1{setkey(geo,node,i,bkey(geo,node,i+1));setval(geo,node,i,bval(geo,node,i+1));}clearpair(geo,node,fill-1);if fill-1<(*geo).no_pairs/2&&level==(*head).height&&fill-1==1{btree_shrink(head,geo);}ret}
pub unsafe extern "C" fn btree_remove(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize)->*mut core::ffi::c_void {if (*head).height==0{core::ptr::null_mut()}else{btree_remove_level(head,geo,key,1)}}

pub unsafe extern "C" fn btree_last(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize)->*mut core::ffi::c_void {if (*head).height==0{return core::ptr::null_mut();}let mut node=(*head).node;for _ in 1..(*head).height{node=bval(geo,node,0) as *mut usize;}longcpy(key,bkey(geo,node,0),(*geo).keylen as usize);bval(geo,node,0)}

pub unsafe extern "C" fn btree_get_prev(head:*mut btree_head,geo:*mut btree_geo,key:*mut usize)->*mut core::ffi::c_void { if keyzero(geo,key)!=0||(*head).height==0{return core::ptr::null_mut();} let mut k=[0usize;MAX_KEYLEN];longcpy(k.as_mut_ptr(),key,(*geo).keylen as usize);loop{dec_key(geo,k.as_mut_ptr());let mut node=(*head).node;let mut retry=core::ptr::null_mut();let mut miss=false;for _ in 1..(*head).height{let mut i=0;while i<(*geo).no_pairs&&keycmp(geo,node,i,k.as_mut_ptr())>0{i+=1;}if i==(*geo).no_pairs{miss=true;break;}let old=node;node=bval(geo,node,i) as *mut usize;if node.is_null(){miss=true;break;}retry=bkey(geo,old,i);}if !miss&&!node.is_null(){for i in 0..(*geo).no_pairs{if keycmp(geo,node,i,k.as_mut_ptr())<=0{let v=bval(geo,node,i);if !v.is_null(){longcpy(key,bkey(geo,node,i),(*geo).keylen as usize);return v;}miss=true;break;}}}if retry.is_null(){return core::ptr::null_mut();}longcpy(k.as_mut_ptr(),retry,(*geo).keylen as usize);}}

pub unsafe extern "C" fn btree_merge(target:*mut btree_head,victim:*mut btree_head,geo:*mut btree_geo,gfp:gfp_t)->i32 {BUG_ON(target==victim);if (*target).node.is_null(){(*target).node=(*victim).node;(*target).height=(*victim).height;__btree_init(victim);return 0;}let mut key=[0usize;MAX_KEYLEN];let mut dup=[0usize;MAX_KEYLEN];loop{let val=btree_last(victim,geo,key.as_mut_ptr());if val.is_null(){break;}let err=btree_insert(target,geo,key.as_mut_ptr(),val,gfp);if err!=0{return err;}longcpy(dup.as_mut_ptr(),key.as_ptr(),(*geo).keylen as usize);btree_remove(victim,geo,dup.as_mut_ptr());}0}

pub type btree_visit_fn=unsafe extern "C" fn(*mut core::ffi::c_void,usize,*mut usize,usize,*mut core::ffi::c_void);
unsafe fn walk(head:*mut btree_head,geo:*mut btree_geo,node:*mut usize,opaque:usize,func:btree_visit_fn,func2:*mut core::ffi::c_void,height:i32,count:&mut usize,reap:bool){for i in 0..(*geo).no_pairs{let child=bval(geo,node,i) as *mut usize;if child.is_null(){break;}if height>1{walk(head,geo,child,opaque,func,func2,height-1,count,reap);}else{func(child as *mut _,opaque,bkey(geo,node,i),*count,func2);*count+=1;}}if reap{mempool_free(node as *mut _,(*head).mempool);}}
unsafe extern "C" fn empty(_:*mut core::ffi::c_void,_:usize,_:*mut usize,_:usize,_:*mut core::ffi::c_void){}
pub unsafe extern "C" fn btree_visitor(head:*mut btree_head,geo:*mut btree_geo,opaque:usize,func:btree_visit_fn,func2:*mut core::ffi::c_void)->usize{let mut n=0;if !(*head).node.is_null(){walk(head,geo,(*head).node,opaque,if func2.is_null(){empty}else{func},func2,(*head).height,&mut n,false);}n}
pub unsafe extern "C" fn btree_grim_visitor(head:*mut btree_head,geo:*mut btree_geo,opaque:usize,func:btree_visit_fn,func2:*mut core::ffi::c_void)->usize{let mut n=0;if !(*head).node.is_null(){walk(head,geo,(*head).node,opaque,if func2.is_null(){empty}else{func},func2,(*head).height,&mut n,true);}__btree_init(head);n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
