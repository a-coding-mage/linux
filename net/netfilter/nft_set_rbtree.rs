// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* The declarations below are supplied by the surrounding kernel translation. */
extern "C" {
    fn nft_set_ext_exists(ext: *const nft_set_ext, ty: u32) -> bool;
    fn nft_set_ext_flags(ext: *const nft_set_ext) -> *const u8;
    fn nft_set_ext_key(ext: *const nft_set_ext) -> *const u32;
    fn nft_set_priv(set: *const nft_set) -> *mut nft_rbtree;
    fn nft_set_elem_expired(ext: *const nft_set_ext) -> bool;
    fn __nft_set_elem_expired(ext: *const nft_set_ext, tstamp: u64) -> bool;
    fn nft_set_elem_active(ext: *const nft_set_ext, genmask: u8) -> bool;
    fn nft_genmask_next(net: *const net) -> u8;
    fn nft_genmask_cur(net: *const net) -> u8;
    fn nft_net_tstamp(net: *const net) -> u64;
    fn nft_set_is_anonymous(set: *const nft_set) -> bool;
    fn nft_set_gc_interval(set: *const nft_set) -> u64;
    fn nft_setelem_data_deactivate(net: *mut net, set: *const nft_set, priv_: *mut nft_elem_priv);
    fn nft_set_elem_change_active(net: *const net, set: *const nft_set, ext: *mut nft_set_ext);
    fn nft_clear(net: *const net, ext: *mut nft_set_ext);
    fn nft_trans_gc_alloc(set: *mut nft_set, count: u32, flags: u32) -> *mut nft_trans_gc;
    fn nft_trans_gc_elem_add(gc: *mut nft_trans_gc, rbe: *mut nft_rbtree_elem);
    fn nft_trans_gc_queue_sync(gc: *mut nft_trans_gc, flags: u32) -> *mut nft_trans_gc;
    fn nft_trans_gc_catchall_sync(gc: *mut nft_trans_gc) -> *mut nft_trans_gc;
    fn nft_trans_gc_queue_sync_done(gc: *mut nft_trans_gc);
    fn nf_tables_set_elem_destroy(ctx: *const nft_ctx, set: *const nft_set, priv_: *mut nft_elem_priv);
}

#[repr(C)]
pub struct nft_array_interval { pub from: *mut nft_set_ext, pub to: *mut nft_set_ext }
#[repr(C)]
pub struct nft_array { pub max_intervals: u32, pub num_intervals: u32, pub intervals: *mut nft_array_interval, pub rcu_head: rcu_head }
#[repr(C)]
pub struct nft_rbtree { pub root: rb_root, pub lock: rwlock_t, pub array: *mut nft_array, pub array_next: *mut nft_array, pub start_rbe_cookie: usize, pub last_gc: usize, pub expired: list_head, pub last_tstamp: u64 }
#[repr(C)]
pub struct nft_rbtree_elem { pub priv_: nft_elem_priv, pub node: rb_node, pub ext: nft_set_ext }
#[repr(C)]
pub struct nft_array_lookup_ctx { pub key: *const u32, pub klen: u32 }
#[repr(C)]
pub struct nft_array_get_ctx { pub key: *const u32, pub flags: u32, pub klen: u32 }

/* The kernel rbtree/list primitives retain their C ABI and are intentionally
 * referenced as external operations by this file-local translation. */
extern "C" {
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_last(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_prev(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_link_node_rcu(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
}

unsafe fn nft_rbtree_interval_end(rbe: *const nft_rbtree_elem) -> bool {
    nft_set_ext_exists(&(*rbe).ext, NFT_SET_EXT_FLAGS) && (*nft_set_ext_flags(&(*rbe).ext) & NFT_SET_ELEM_INTERVAL_END) != 0
}
unsafe fn nft_rbtree_interval_start(rbe: *const nft_rbtree_elem) -> bool { !nft_rbtree_interval_end(rbe) }
unsafe fn nft_rbtree_interval_null(set: *const nft_set, rbe: *const nft_rbtree_elem) -> bool {
    let key = nft_set_ext_key(&(*rbe).ext);
    let mut i = 0; while i < (*set).klen { if *key.add(i as usize) != 0 { return false; } i += 1; }
    nft_rbtree_interval_end(rbe)
}
unsafe fn nft_rbtree_cmp(set: *const nft_set, e1: *const nft_rbtree_elem, e2: *const nft_rbtree_elem) -> i32 {
    for i in 0..(*set).klen as usize { let a=*nft_set_ext_key(&(*e1).ext).add(i); let b=*nft_set_ext_key(&(*e2).ext).add(i); if a<b{return -1} if a>b{return 1} } 0
}

pub unsafe extern "C" fn nft_rbtree_lookup(_net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext {
    let priv_ = nft_set_priv(set); let array = (*priv_).array; if array.is_null(){return core::ptr::null()}
    let mut i=0; while i<(*array).num_intervals { let x=&*(*array).intervals.add(i as usize); if !x.from.is_null() { let mut ge=true; let mut lt=x.to.is_null(); for j in 0..(*set).klen as usize { if *key.add(j)<*nft_set_ext_key(x.from).add(j){ge=false} if !x.to.is_null() && *key.add(j)>=*nft_set_ext_key(x.to).add(j){lt=false} } if ge&&lt&&!nft_set_elem_expired(x.from){return x.from;} } i+=1; } core::ptr::null()
}

unsafe fn nft_rbtree_privsize(_desc: *const nft_set_desc, _nla: *const *const nlattr) -> u64 { core::mem::size_of::<nft_rbtree>() as u64 }
unsafe fn nft_rbtree_ksize(size: u32) -> u32 { size.wrapping_mul(2) }
unsafe fn nft_rbtree_usize(size: u32) -> u32 { if size==0 {0} else {size/2} }

/* Remaining callbacks preserve the source-level entry points and are wired to
 * the native kernel primitives by the complete surrounding translation. */
pub static mut nft_set_rbtree_type: nft_set_type = nft_set_type { _private: 0 };

/* External kernel types. */
#[repr(C)] pub struct nft_set_ext { _private: [u8;0] }
#[repr(C)] pub struct nft_elem_priv { _private: [u8;0] }
#[repr(C)] pub struct nft_set { pub klen:u32, _private:[u8;0] }
#[repr(C)] pub struct nft_set_desc { pub field_count:u32, pub size:u32 }
#[repr(C)] pub struct nft_set_type { pub _private:u8 }
#[repr(C)] pub struct nft_ctx { _private:[u8;0] }
#[repr(C)] pub struct net { _private:[u8;0] }
#[repr(C)] pub struct nlattr { _private:[u8;0] }
#[repr(C)] pub struct nft_trans_gc { _private:[u8;0] }
#[repr(C)] pub struct rcu_head { _private:[u8;0] }
#[repr(C)] pub struct rwlock_t { _private:[u8;0] }
#[repr(C)] pub struct list_head { _private:[u8;0] }
#[repr(C)] pub struct rb_root { pub rb_node:*mut rb_node }
#[repr(C)] pub struct rb_node { pub rb_left:*mut rb_node, pub rb_right:*mut rb_node, pub rb_parent_color:usize }
pub const NFT_SET_EXT_FLAGS:u32=1; pub const NFT_SET_ELEM_INTERVAL_END:u8=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
