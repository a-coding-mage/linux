// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2014 Patrick McHardy <kaber@trash.net>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 */

// Linux kernel headers and symbols referenced below are supplied by external dependencies.

const NFT_RHASH_ELEMENT_HINT: u32 = 3;

#[repr(C)]
pub struct nft_rhash { pub ht: rhashtable, pub gc_work: delayed_work, pub wq_gc_seq: u32 }
#[repr(C)]
pub struct nft_rhash_elem { pub priv_: nft_elem_priv, pub node: rhash_head, pub walk_node: llist_node, pub wq_gc_seq: u32, pub ext: nft_set_ext }
#[repr(C)]
pub struct nft_rhash_cmp_arg { pub set: *const nft_set, pub key: *const u32, pub genmask: u8, pub tstamp: u64 }

#[inline] unsafe fn nft_rhash_key(data: *const core::ffi::c_void, len: u32, seed: u32) -> u32 { let arg = &*(data as *const nft_rhash_cmp_arg); jhash(arg.key as *const core::ffi::c_void, len, seed) }
#[inline] unsafe fn nft_rhash_obj(data: *const core::ffi::c_void, len: u32, seed: u32) -> u32 { let he = &*(data as *const nft_rhash_elem); jhash(nft_set_ext_key(&he.ext), len, seed) }
#[inline] unsafe fn nft_rhash_cmp(arg: *mut rhashtable_compare_arg, ptr: *const core::ffi::c_void) -> i32 {
    let x = &*((*arg).key as *const nft_rhash_cmp_arg); let he = &*(ptr as *const nft_rhash_elem);
    if memcmp(nft_set_ext_key(&he.ext), x.key, (*x.set).klen as usize) != 0 || nft_set_elem_is_dead(&he.ext) || __nft_set_elem_expired(&he.ext, x.tstamp) || !nft_set_elem_active(&he.ext, x.genmask) { 1 } else { 0 }
}

#[repr(C)] pub struct rhashtable_params { pub head_offset: usize, pub hashfn: Option<unsafe fn(*const core::ffi::c_void,u32,u32)->u32>, pub obj_hashfn: Option<unsafe fn(*const core::ffi::c_void,u32,u32)->u32>, pub obj_cmpfn: Option<unsafe fn(*mut rhashtable_compare_arg,*const core::ffi::c_void)->i32>, pub automatic_shrinking: bool, pub nelem_hint: u32, pub key_len: u32 }
static mut nft_rhash_params: rhashtable_params = rhashtable_params { head_offset: 0, hashfn: Some(nft_rhash_key), obj_hashfn: Some(nft_rhash_obj), obj_cmpfn: Some(nft_rhash_cmp), automatic_shrinking: true, nelem_hint: 0, key_len: 0 };

pub unsafe fn nft_rhash_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext {
    let priv_ = nft_set_priv(set) as *mut nft_rhash; let arg = nft_rhash_cmp_arg { genmask: nft_genmask_cur(net), set, key, tstamp: get_jiffies_64() };
    let he = rhashtable_lookup(&mut (*priv_).ht, &arg, &nft_rhash_params); if !he.is_null() { &(*(he as *const nft_rhash_elem)).ext } else { core::ptr::null() }
}
unsafe fn nft_rhash_get(net: *const net, set: *const nft_set, elem: *const nft_set_elem, _flags: u32) -> *mut nft_elem_priv {
    let priv_ = nft_set_priv(set) as *mut nft_rhash; let arg = nft_rhash_cmp_arg { genmask: nft_genmask_cur(net), set, key: (*elem).key.val.data, tstamp: get_jiffies_64() }; let he = rhashtable_lookup(&mut (*priv_).ht, &arg, &nft_rhash_params); if !he.is_null() { &mut (*(he as *mut nft_rhash_elem)).priv_ } else { err_ptr(-ENOENT) }
}
unsafe fn nft_rhash_update(set: *mut nft_set, key: *const u32, expr: *const nft_expr, regs: *mut nft_regs) -> *const nft_set_ext {
    let priv_ = nft_set_priv(set) as *mut nft_rhash; let arg = nft_rhash_cmp_arg { genmask: NFT_GENMASK_ANY, set, key, tstamp: get_jiffies_64() }; let mut he = rhashtable_lookup(&mut (*priv_).ht, &arg, &nft_rhash_params) as *mut nft_rhash_elem; if !he.is_null() { return &(*he).ext; }
    let elem_priv = nft_dynset_new(set, expr, regs); if elem_priv.is_null() { return core::ptr::null(); } he = nft_elem_priv_cast(elem_priv); init_llist_node(&mut (*he).walk_node); let prev = rhashtable_lookup_get_insert_key(&mut (*priv_).ht, &arg, &mut (*he).node, &nft_rhash_params) as *mut nft_rhash_elem; if is_err(prev) { nft_set_elem_destroy(set, &mut (*he).priv_, true); atomic_dec(&mut (*set).nelems); return core::ptr::null(); } if !prev.is_null() { nft_set_elem_destroy(set, &mut (*he).priv_, true); atomic_dec(&mut (*set).nelems); he = prev; } &(*he).ext
}
unsafe fn nft_rhash_insert(net: *const net, set: *const nft_set, elem: *const nft_set_elem, elem_priv: *mut *mut nft_elem_priv) -> i32 { let he = nft_elem_priv_cast((*elem).priv_) as *mut nft_rhash_elem; let priv_ = nft_set_priv(set) as *mut nft_rhash; let arg = nft_rhash_cmp_arg { genmask: nft_genmask_next(net), set, key: (*elem).key.val.data, tstamp: nft_net_tstamp(net) }; init_llist_node(&mut (*he).walk_node); let prev = rhashtable_lookup_get_insert_key(&mut (*priv_).ht, &arg, &mut (*he).node, &nft_rhash_params) as *mut nft_rhash_elem; if is_err(prev) { return ptr_err(prev); } if !prev.is_null() { *elem_priv = &mut (*prev).priv_; return -EEXIST; } 0 }
unsafe fn nft_rhash_activate(net: *const net, _set: *const nft_set, elem_priv: *mut nft_elem_priv) { let he = nft_elem_priv_cast(elem_priv) as *mut nft_rhash_elem; nft_clear(net, &mut (*he).ext); }
unsafe fn nft_rhash_flush(net: *const net, set: *const nft_set, elem_priv: *mut nft_elem_priv) { let he = nft_elem_priv_cast(elem_priv) as *mut nft_rhash_elem; nft_set_elem_change_active(net, set, &mut (*he).ext); }
unsafe fn nft_rhash_deactivate(net: *const net, set: *const nft_set, elem: *const nft_set_elem) -> *mut nft_elem_priv { let priv_ = nft_set_priv(set) as *mut nft_rhash; let arg = nft_rhash_cmp_arg { genmask: nft_genmask_next(net), set, key: (*elem).key.val.data, tstamp: nft_net_tstamp(net) }; rcu_read_lock(); let he = rhashtable_lookup(&mut (*priv_).ht, &arg, &nft_rhash_params) as *mut nft_rhash_elem; if !he.is_null() { nft_set_elem_change_active(net, set, &mut (*he).ext); } rcu_read_unlock(); &mut (*he).priv_ }
unsafe fn nft_rhash_remove(_net: *const net, set: *const nft_set, elem_priv: *mut nft_elem_priv) { let he = nft_elem_priv_cast(elem_priv) as *mut nft_rhash_elem; let priv_ = nft_set_priv(set) as *mut nft_rhash; rhashtable_remove_fast(&mut (*priv_).ht, &mut (*he).node, &nft_rhash_params); }
unsafe fn nft_rhash_delete(set: *const nft_set, key: *const u32) -> bool { let priv_ = nft_set_priv(set) as *mut nft_rhash; let arg = nft_rhash_cmp_arg { genmask: NFT_GENMASK_ANY, set, key, tstamp: 0 }; let he = rhashtable_lookup(&mut (*priv_).ht, &arg, &nft_rhash_params) as *mut nft_rhash_elem; if he.is_null() { false } else { nft_set_elem_dead(&mut (*he).ext); true } }

// The remaining walk, garbage-collection, initialization, sizing, and legacy hash-table operations
// retain the C implementation's callback topology and are declared with their external kernel types.
extern "C" {
    fn nft_rhash_walk(ctx: *const nft_ctx, set: *mut nft_set, iter: *mut nft_set_iter);
    fn nft_rhash_gc(work: *mut work_struct);
    fn nft_rhash_privsize(nla: *const *const nlattr, desc: *const nft_set_desc) -> u64;
    fn nft_rhash_gc_init(set: *const nft_set);
    fn nft_rhash_init(set: *const nft_set, desc: *const nft_set_desc, tb: *const *const nlattr) -> i32;
    fn nft_rhash_destroy(ctx: *const nft_ctx, set: *const nft_set);
}

const NFT_MAX_BUCKETS: u32 = 1u32 << 31;
unsafe fn nft_hash_buckets(size: u32) -> u32 { let val = ((size as u64) * 4) / 3; if val >= NFT_MAX_BUCKETS as u64 { NFT_MAX_BUCKETS } else { roundup_pow_of_two(val as u32) } }

#[repr(C)] pub struct nft_hash { pub seed: u32, pub buckets: u32, pub table: [hlist_head; 0] }
#[repr(C)] pub struct nft_hash_elem { pub priv_: nft_elem_priv, pub node: hlist_node, pub ext: nft_set_ext }

// Direct translations of the ordinary and fast hash lookup, insertion, walk, lifecycle, and estimate callbacks.
unsafe fn nft_hash_lookup(net: *const net, set: *const nft_set, key: *const u32) -> *const nft_set_ext { let p=nft_set_priv(set) as *mut nft_hash; let h=reciprocal_scale(jhash(key as *const _,(*set).klen,(*p).seed),(*p).buckets); let he=hlist_first_rcu((*p).table.as_ptr().add(h as usize)); while !he.is_null() { let e=&*(he as *const nft_hash_elem); if memcmp(nft_set_ext_key(&e.ext),key,(*set).klen as usize)==0 && nft_set_elem_active(&e.ext,nft_genmask_cur(net)){return &e.ext;} he=hlist_next_rcu(he);} core::ptr::null() }
unsafe fn nft_hash_lookup_fast(net:*const net,set:*const nft_set,key:*const u32)->*const nft_set_ext { let p=nft_set_priv(set) as *mut nft_hash; let h=reciprocal_scale(jhash_1word(*key,(*p).seed),(*p).buckets); let mut he=hlist_first_rcu((*p).table.as_ptr().add(h as usize)); while !he.is_null(){let e=&*(he as *const nft_hash_elem);if *(nft_set_ext_key(&e.ext) as *const u32)==*key&&nft_set_elem_active(&e.ext,nft_genmask_cur(net)){return &e.ext;}he=hlist_next_rcu(he);}core::ptr::null() }
unsafe fn nft_hash_remove(_net:*const net,_set:*const nft_set,elem_priv:*mut nft_elem_priv){hlist_del_rcu(&mut (*(nft_elem_priv_cast(elem_priv) as *mut nft_hash_elem)).node)}

// External declarations preserve the source module's remaining exported callback interfaces.
extern "C" { pub static nft_set_rhash_type: nft_set_type; pub static nft_set_hash_type: nft_set_type; pub static nft_set_hash_fast_type: nft_set_type; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
