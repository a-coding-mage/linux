// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 Pablo Neira Ayuso <pablo@netfilter.org>
 */

// Linux kernel and nf_tables dependencies supplied by other translation units.

#[repr(C)]
pub struct nft_bitmap_elem {
    pub priv_: nft_elem_priv,
    pub head: list_head,
    pub ext: nft_set_ext,
}

/* This bitmap uses two bits to represent one element. These two bits determine
 * the element state in the current and the future generation.
 *
 * An element can be in three states. The generation cursor is represented using
 * the ^ character, note that this cursor shifts on every successful transaction.
 * If no transaction is going on, we observe all elements are in the following
 * state:
 *
 * 11 = this element is active in the current generation. In case of no updates,
 * ^    it stays active in the next generation.
 * 00 = this element is inactive in the current generation. In case of no
 * ^    updates, it stays inactive in the next generation.
 *
 * On transaction handling, we observe these two temporary states:
 *
 * 01 = this element is inactive in the current generation and it becomes active
 * ^    in the next one. This happens when the element is inserted but commit
 *      path has not yet been executed yet, so activation is still pending. On
 *      transaction abortion, the element is removed.
 * 10 = this element is active in the current generation and it becomes inactive
 * ^    in the next one. This happens when the element is deactivated but commit
 *      path has not yet been executed yet, so removal is still pending. On
 *      transaction abortion, the next generation bit is reset to go back to
 *      restore its previous state.
 */
#[repr(C)]
pub struct nft_bitmap {
    pub list: list_head,
    pub bitmap_size: u16,
    pub bitmap: [u8; 0],
}

#[inline]
unsafe fn nft_bitmap_location(set: *const nft_set, key: *const c_void,
                               idx: *mut u32, off: *mut u32) {
    let mut k: u32;
    if (*set).klen == 2 {
        k = (*(key as *const u16)) as u32;
    } else {
        k = *(key as *const u8) as u32;
    }
    k <<= 1;
    *idx = k / BITS_PER_BYTE;
    *off = k % BITS_PER_BYTE;
}

#[inline]
unsafe fn nft_bitmap_active(bitmap: *const u8, idx: u32, off: u32, genmask: u8) -> bool {
    ((*bitmap.add(idx as usize) & (0x3u8 << off)) & (genmask << off)) != 0
}

pub unsafe fn nft_bitmap_lookup(net: *const net, set: *const nft_set,
                                key: *const u32) -> *const nft_set_ext {
    let priv_ = nft_set_priv(set) as *const nft_bitmap;
    static FOUND: nft_set_ext = nft_set_ext { _private: [0; 0] };
    let genmask = nft_genmask_cur(net);
    let mut idx = 0u32;
    let mut off = 0u32;
    nft_bitmap_location(set, key as *const c_void, &mut idx, &mut off);
    if nft_bitmap_active((*priv_).bitmap.as_ptr(), idx, off, genmask) { &FOUND } else { core::ptr::null() }
}

unsafe fn nft_bitmap_elem_find(net: *const net, set: *const nft_set,
                               this: *mut nft_bitmap_elem, genmask: u8) -> *mut nft_bitmap_elem {
    let priv_ = nft_set_priv(set) as *const nft_bitmap;
    let mut be: *mut nft_bitmap_elem;
    list_for_each_entry_rcu!(be, &(*priv_).list, head,
        lockdep_is_held(&(*nft_pernet(net)).commit_mutex));
    if memcmp(nft_set_ext_key(&(*be).ext), nft_set_ext_key(&(*this).ext), (*set).klen) == 0
        && nft_set_elem_active(&(*be).ext, genmask) { return be; }
    core::ptr::null_mut()
}

unsafe fn nft_bitmap_get(net: *const net, set: *const nft_set,
                         elem: *const nft_set_elem, _flags: c_uint) -> *mut nft_elem_priv {
    let priv_ = nft_set_priv(set) as *const nft_bitmap;
    let genmask = nft_genmask_cur(net);
    let mut be: *mut nft_bitmap_elem;
    list_for_each_entry_rcu!(be, &(*priv_).list, head);
    if memcmp(nft_set_ext_key(&(*be).ext), (*elem).key.val.data, (*set).klen) == 0
        && nft_set_elem_active(&(*be).ext, genmask) { return &mut (*be).priv_; }
    ERR_PTR(-ENOENT)
}

unsafe fn nft_bitmap_insert(net: *const net, set: *const nft_set,
                            elem: *const nft_set_elem,
                            elem_priv: *mut *mut nft_elem_priv) -> c_int {
    let new = nft_elem_priv_cast((*elem).priv_) as *mut nft_bitmap_elem;
    let priv_ = nft_set_priv(set) as *mut nft_bitmap;
    let genmask = nft_genmask_next(net);
    let mut idx = 0u32; let mut off = 0u32;
    let be = nft_bitmap_elem_find(net, set, new, genmask);
    if !be.is_null() { *elem_priv = &mut (*be).priv_; return -EEXIST; }
    nft_bitmap_location(set, nft_set_ext_key(&(*new).ext), &mut idx, &mut off);
    (*priv_).bitmap.as_mut_ptr().add(idx as usize).write((*priv_).bitmap.as_ptr().add(idx as usize).read() | (genmask << off));
    list_add_tail_rcu!(&mut (*new).head, &mut (*priv_).list);
    0
}

unsafe fn nft_bitmap_remove(net: *const net, set: *const nft_set, elem_priv: *mut nft_elem_priv) {
    let be = nft_elem_priv_cast(elem_priv) as *mut nft_bitmap_elem;
    let priv_ = nft_set_priv(set) as *mut nft_bitmap;
    let genmask = nft_genmask_next(net); let mut idx = 0u32; let mut off = 0u32;
    nft_bitmap_location(set, nft_set_ext_key(&(*be).ext), &mut idx, &mut off);
    let p = (*priv_).bitmap.as_mut_ptr().add(idx as usize); p.write(p.read() & !(genmask << off));
    list_del_rcu!(&mut (*be).head);
}

unsafe fn nft_bitmap_activate(net: *const net, set: *const nft_set, elem_priv: *mut nft_elem_priv) {
    let be = nft_elem_priv_cast(elem_priv) as *mut nft_bitmap_elem; let priv_ = nft_set_priv(set) as *mut nft_bitmap;
    let genmask = nft_genmask_next(net); let mut idx = 0u32; let mut off = 0u32;
    nft_bitmap_location(set, nft_set_ext_key(&(*be).ext), &mut idx, &mut off);
    let p = (*priv_).bitmap.as_mut_ptr().add(idx as usize); p.write(p.read() | (genmask << off)); nft_clear(net, &mut (*be).ext);
}

unsafe fn nft_bitmap_flush(net: *const net, set: *const nft_set, elem_priv: *mut nft_elem_priv) {
    let be = nft_elem_priv_cast(elem_priv) as *mut nft_bitmap_elem; let priv_ = nft_set_priv(set) as *mut nft_bitmap;
    let genmask = nft_genmask_next(net); let mut idx = 0u32; let mut off = 0u32;
    nft_bitmap_location(set, nft_set_ext_key(&(*be).ext), &mut idx, &mut off);
    let p = (*priv_).bitmap.as_mut_ptr().add(idx as usize); p.write(p.read() & !(genmask << off)); nft_set_elem_change_active(net, set, &mut (*be).ext);
}

unsafe fn nft_bitmap_deactivate(net: *const net, set: *const nft_set, elem: *const nft_set_elem) -> *mut nft_elem_priv {
    let this = nft_elem_priv_cast((*elem).priv_) as *mut nft_bitmap_elem; let priv_ = nft_set_priv(set) as *mut nft_bitmap;
    let genmask = nft_genmask_next(net); let mut idx = 0u32; let mut off = 0u32;
    nft_bitmap_location(set, (*elem).key.val.data, &mut idx, &mut off);
    let be = nft_bitmap_elem_find(net, set, this, genmask); if be.is_null() { return core::ptr::null_mut(); }
    let p = (*priv_).bitmap.as_mut_ptr().add(idx as usize); p.write(p.read() & !(genmask << off)); nft_set_elem_change_active(net, set, &mut (*be).ext); &mut (*be).priv_
}

unsafe fn nft_bitmap_walk(ctx: *const nft_ctx, set: *mut nft_set, iter: *mut nft_set_iter) {
    let priv_ = nft_set_priv(set) as *const nft_bitmap; let mut be: *mut nft_bitmap_elem;
    list_for_each_entry_rcu!(be, &(*priv_).list, head, lockdep_is_held(&(*nft_pernet((*ctx).net)).commit_mutex));
    if (*iter).count >= (*iter).skip { (*iter).err = ((*iter).fn_)(ctx, set, iter, &mut (*be).priv_); if (*iter).err < 0 { return; } }
    (*iter).count += 1;
}

#[inline] fn nft_bitmap_size(klen: u32) -> u32 { ((2u32 << ((klen * BITS_PER_BYTE) - 1)) / BITS_PER_BYTE) << 1 }
#[inline] fn nft_bitmap_total_size(klen: u32) -> u64 { core::mem::size_of::<nft_bitmap>() as u64 + nft_bitmap_size(klen) as u64 }

unsafe fn nft_bitmap_privsize(nla: *const *const nlattr, _desc: *const nft_set_desc) -> u64 { nft_bitmap_total_size(ntohl(nla_get_be32(*nla.add(NFTA_SET_KEY_LEN)))) }

unsafe fn nft_bitmap_init(set: *const nft_set, _desc: *const nft_set_desc, _nla: *const *const nlattr) -> c_int {
    let priv_ = nft_set_priv(set) as *mut nft_bitmap; INIT_LIST_HEAD!(&mut (*priv_).list); (*priv_).bitmap_size = nft_bitmap_size((*set).klen) as u16; 0
}

unsafe fn nft_bitmap_destroy(ctx: *const nft_ctx, set: *const nft_set) { let priv_ = nft_set_priv(set) as *mut nft_bitmap; let mut be: *mut nft_bitmap_elem; let mut n: *mut nft_bitmap_elem; list_for_each_entry_safe!(be, n, &mut (*priv_).list, head) { nf_tables_set_elem_destroy(ctx, set, &mut (*be).priv_); } }

unsafe fn nft_bitmap_estimate(desc: *const nft_set_desc, _features: u32, est: *mut nft_set_estimate) -> bool {
    if (*desc).klen > 2 || !(*desc).expr.is_null() { return false; }
    (*est).size = nft_bitmap_total_size((*desc).klen); (*est).lookup = NFT_SET_CLASS_O_1; (*est).space = NFT_SET_CLASS_O_1; true
}

pub static mut nft_set_bitmap_type: nft_set_type = nft_set_type {
    ops: nft_set_ops {
        privsize: Some(nft_bitmap_privsize), elemsize: core::mem::offset_of!(nft_bitmap_elem, ext), estimate: Some(nft_bitmap_estimate), init: Some(nft_bitmap_init), destroy: Some(nft_bitmap_destroy), insert: Some(nft_bitmap_insert), remove: Some(nft_bitmap_remove), deactivate: Some(nft_bitmap_deactivate), flush: Some(nft_bitmap_flush), activate: Some(nft_bitmap_activate), lookup: Some(nft_bitmap_lookup), walk: Some(nft_bitmap_walk), get: Some(nft_bitmap_get),
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
