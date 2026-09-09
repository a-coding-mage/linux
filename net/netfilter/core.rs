// SPDX-License-Identifier: GPL-2.0
/* netfilter.c: look after the filters for various protocols.
 * Heavily influenced by the old firewall.c by David Bonn and Alan Cox.
 *
 * Thanks to Rob `CmdrTaco' Malda for not influencing this code in any
 * way.
 */
// Dependencies are supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_JUMP_LABEL")]
static mut NF_HOOKS_NEEDED: [[static_key; NF_MAX_HOOKS]; NFPROTO_NUMPROTO] = [[static_key::default(); NF_MAX_HOOKS]; NFPROTO_NUMPROTO];

static mut NF_HOOK_MUTEX: mutex = mutex::new();
const MAX_HOOK_COUNT: u32 = 1024;

unsafe fn allocate_hook_entries_size(num: u16) -> *mut nf_hook_entries {
    if num == 0 { return core::ptr::null_mut(); }
    let alloc = core::mem::size_of::<nf_hook_entries>()
        + core::mem::size_of::<nf_hook_entry>() * num as usize
        + core::mem::size_of::<*mut nf_hook_ops>() * num as usize
        + core::mem::size_of::<nf_hook_entries_rcu_head>();
    let e = kvzalloc(alloc, GFP_KERNEL_ACCOUNT) as *mut nf_hook_entries;
    if !e.is_null() { (*e).num_hook_entries = num; }
    e
}

unsafe extern "C" fn __nf_hook_entries_free(h: *mut rcu_head) {
    let head = container_of!(h, nf_hook_entries_rcu_head, head);
    kvfree((*head).allocation as *mut core::ffi::c_void);
}

unsafe fn nf_hook_entries_free(e: *mut nf_hook_entries) {
    if e.is_null() { return; }
    let num = (*e).num_hook_entries as usize;
    let ops = nf_hook_entries_get_hook_ops(e);
    let head = ops.add(num) as *mut nf_hook_entries_rcu_head;
    (*head).allocation = e;
    call_rcu(&mut (*head).head, __nf_hook_entries_free);
}

unsafe extern "C" fn accept_all(_priv: *mut core::ffi::c_void, _skb: *mut sk_buff, _state: *const nf_hook_state) -> u32 { NF_ACCEPT }

static mut DUMMY_OPS: nf_hook_ops = nf_hook_ops { hook: Some(accept_all), priority: INT_MIN, ..nf_hook_ops::zeroed() };

unsafe fn nf_hook_entries_grow(old: *const nf_hook_entries, reg: *const nf_hook_ops) -> *mut nf_hook_entries {
    let mut alloc_entries = 1u32;
    let old_entries = if old.is_null() { 0 } else { (*old).num_hook_entries as u32 };
    let mut orig_ops: *mut *mut nf_hook_ops = core::ptr::null_mut();
    if !old.is_null() {
        orig_ops = nf_hook_entries_get_hook_ops(old as *mut _);
        for i in 0..old_entries as usize {
            if *orig_ops.add(i) != &mut DUMMY_OPS { alloc_entries += 1; }
            if (*reg).priority == (**orig_ops.add(i)).priority && (*reg).hook_ops_type == NF_HOOK_OP_BPF { return ERR_PTR(-EBUSY); }
        }
    }
    if alloc_entries > MAX_HOOK_COUNT { return ERR_PTR(-E2BIG); }
    let new = allocate_hook_entries_size(alloc_entries as u16);
    if new.is_null() { return ERR_PTR(-ENOMEM); }
    let new_ops = nf_hook_entries_get_hook_ops(new);
    let mut i = 0usize; let mut nhooks = 0usize; let mut inserted = false;
    while i < old_entries as usize {
        if *orig_ops.add(i) == &mut DUMMY_OPS { i += 1; continue; }
        if inserted || (*reg).priority > (**orig_ops.add(i)).priority {
            *new_ops.add(nhooks) = *orig_ops.add(i);
            (*new).hooks[nhooks] = (*old).hooks[i]; i += 1;
        } else {
            *new_ops.add(nhooks) = reg as *mut _;
            (*new).hooks[nhooks].hook = (*reg).hook; (*new).hooks[nhooks].priv_ = (*reg).priv_; inserted = true;
        }
        nhooks += 1;
    }
    if !inserted { *new_ops.add(nhooks) = reg as *mut _; (*new).hooks[nhooks].hook = (*reg).hook; (*new).hooks[nhooks].priv_ = (*reg).priv_; }
    new
}

unsafe fn hooks_validate(_hooks: *const nf_hook_entries) {}

pub unsafe fn nf_hook_entries_insert_raw(pp: *mut *mut nf_hook_entries, reg: *const nf_hook_ops) -> i32 {
    let p = *pp; let new_hooks = nf_hook_entries_grow(p, reg); if IS_ERR(new_hooks) { return PTR_ERR(new_hooks); }
    hooks_validate(new_hooks); *pp = new_hooks; BUG_ON(p == new_hooks); nf_hook_entries_free(p); 0
}

unsafe fn __nf_hook_entries_try_shrink(old: *mut nf_hook_entries, pp: *mut *mut nf_hook_entries) -> *mut nf_hook_entries {
    if old.is_null() { return core::ptr::null_mut(); }
    let orig_ops = nf_hook_entries_get_hook_ops(old); let mut skip = 0usize;
    for i in 0..(*old).num_hook_entries as usize { if *orig_ops.add(i) == &mut DUMMY_OPS { skip += 1; } }
    let count = (*old).num_hook_entries as usize; if skip == count { *pp = core::ptr::null_mut(); return old; } if skip == 0 { return core::ptr::null_mut(); }
    let new = allocate_hook_entries_size((count - skip) as u16); if new.is_null() { return core::ptr::null_mut(); }
    let new_ops = nf_hook_entries_get_hook_ops(new); let mut j = 0;
    for i in 0..count { if *orig_ops.add(i) == &mut DUMMY_OPS { continue; } (*new).hooks[j] = (*old).hooks[i]; *new_ops.add(j) = *orig_ops.add(i); j += 1; }
    hooks_validate(new); *pp = new; old
}

pub unsafe fn nf_unregister_net_hook(net: *mut net, reg: *const nf_hook_ops) { __nf_unregister_net_hook(net, (*reg).pf, reg); }
pub unsafe fn nf_register_net_hook(net: *mut net, reg: *const nf_hook_ops) -> i32 { __nf_register_net_hook(net, (*reg).pf, reg) }
pub unsafe fn nf_register_net_hooks(net: *mut net, reg: *const nf_hook_ops, n: u32) -> i32 { for i in 0..n { let e=nf_register_net_hook(net, reg.add(i as usize)); if e != 0 { return e; } } 0 }
pub unsafe fn nf_unregister_net_hooks(net: *mut net, reg: *const nf_hook_ops, n: u32) { for i in 0..n { nf_unregister_net_hook(net, reg.add(i as usize)); } }

// Remaining kernel entry points and configuration-specific declarations are
// preserved as external dependencies for the surrounding translation unit.
extern "C" { fn __nf_register_net_hook(net: *mut net, pf: i32, reg: *const nf_hook_ops) -> i32; fn __nf_unregister_net_hook(net: *mut net, pf: i32, reg: *const nf_hook_ops); }

pub unsafe fn nf_hook_slow(skb: *mut sk_buff, state: *mut nf_hook_state, e: *const nf_hook_entries, mut s: u32) -> i32 {
    while s < (*e).num_hook_entries as u32 {
        let verdict = nf_hook_entry_hookfn(&(*e).hooks[s as usize], skb, state);
        match verdict & NF_VERDICT_MASK {
            NF_ACCEPT => {},
            NF_DROP => { kfree_skb_reason(skb, SKB_DROP_REASON_NETFILTER_DROP); let mut ret = NF_DROP_GETERR(verdict); if ret == 0 { ret = -EPERM; } return ret; },
            NF_QUEUE => { let ret = nf_queue(skb, state, s, verdict); if ret == 1 { s += 1; continue; } return ret; },
            NF_STOLEN => return NF_DROP_GETERR(verdict),
            _ => { WARN_ON_ONCE(true); return 0; }
        }
        s += 1;
    }
    1
}

pub unsafe fn nf_hook_slow_list(head: *mut list_head, state: *mut nf_hook_state, e: *const nf_hook_entries) {
    let mut sublist = list_head::new(); let mut skb = (*head).first as *mut sk_buff;
    while !skb.is_null() { let next = (*skb).next; skb_list_del_init(skb); if nf_hook_slow(skb, state, e, 0) == 1 { list_add_tail(&mut (*skb).list, &mut sublist); } skb = next; }
    list_splice(&mut sublist, head);
}

#[no_mangle] pub static mut nfnl_ct_hook: *const nfnl_ct_hook_t = core::ptr::null();
#[no_mangle] pub static mut nf_ct_hook: *const nf_ct_hook_t = core::ptr::null();
#[no_mangle] pub static mut nf_defrag_v4_hook: *const nf_defrag_hook = core::ptr::null();
#[no_mangle] pub static mut nf_defrag_v6_hook: *const nf_defrag_hook = core::ptr::null();

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[no_mangle] pub static mut nf_ctnetlink_has_listener: u8 = 0;
#[cfg(feature = "CONFIG_NF_CONNTRACK")]
#[no_mangle] pub static mut nf_nat_hook: *const nf_nat_hook = core::ptr::null();

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
pub unsafe fn nf_ct_attach(new: *mut sk_buff, skb: *const sk_buff) {
    if (*skb)._nfct != 0 { rcu_read_lock(); let h = rcu_dereference(nf_ct_hook); if !h.is_null() { ((*h).attach)(new, skb); } rcu_read_unlock(); }
}

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
pub unsafe fn nf_conntrack_destroy(nfct: *mut nf_conntrack) { rcu_read_lock(); let h=rcu_dereference(nf_ct_hook); if !h.is_null() { ((*h).destroy)(nfct); } rcu_read_unlock(); WARN_ON(h.is_null()); }

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
pub unsafe fn nf_ct_set_closing(nfct: *mut nf_conntrack) { if nfct.is_null() { return; } rcu_read_lock(); let h=rcu_dereference(nf_ct_hook); if !h.is_null() { ((*h).set_closing)(nfct); } rcu_read_unlock(); }

#[cfg(feature = "CONFIG_NF_CONNTRACK")]
pub unsafe fn nf_ct_get_tuple_skb(dst: *mut nf_conntrack_tuple, skb: *const sk_buff) -> bool { rcu_read_lock(); let h=rcu_dereference(nf_ct_hook); let ret=if h.is_null(){false}else{((*h).get_tuple_skb)(dst,skb)}; rcu_read_unlock(); ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
