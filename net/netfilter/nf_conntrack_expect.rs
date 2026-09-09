// SPDX-License-Identifier: GPL-2.0-only
/* Expectation handling for nf_conntrack. */
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2003,2004 USAGI/WIDE Project <http://www.linux-ipv6.org>
 * (c) 2005-2012 Patrick McHardy <kaber@trash.net>
 */

// Kernel declarations supplied by the surrounding translation unit.

pub static mut nf_ct_expect_hsize: u32 = 0;
pub static mut nf_ct_expect_hash: *mut hlist_head = core::ptr::null_mut();
pub static mut nf_ct_expect_max: u32 = 0;
static mut nf_ct_expect_cachep: *mut kmem_cache = core::ptr::null_mut();
static mut nf_ct_expect_hashrnd: siphash_aligned_key_t = unsafe { core::mem::zeroed() };

pub unsafe fn nf_ct_expectation_gc(master_help: *mut nf_conn_help) {
    let mut exp: *mut nf_conntrack_expect;
    let mut next: *mut hlist_node;
    if hlist_empty(&(*master_help).expectations) { return; }
    spin_lock_bh(&raw mut nf_conntrack_expect_lock);
    hlist_for_each_entry_safe!(exp, next, &mut (*master_help).expectations, lnode, {
        if !nf_ct_exp_is_expired(exp) { continue; }
        nf_ct_unlink_expect(exp);
    });
    spin_unlock_bh(&raw mut nf_conntrack_expect_lock);
}

pub unsafe fn nf_ct_unlink_expect_report(exp: *mut nf_conntrack_expect, portid: u32, report: i32) {
    let master_help = nfct_help((*exp).master);
    let net = nf_ct_exp_net(exp);
    let cnet = nf_ct_pernet(net);
    lockdep_nfct_expect_lock_held();
    hlist_del_rcu(&mut (*exp).hnode);
    (*cnet).expect_count -= 1;
    hlist_del_rcu(&mut (*exp).lnode);
    if !master_help.is_null() { (*master_help).expecting[(*exp).class as usize] -= 1; }
    nf_ct_expect_event_report(IPEXP_DESTROY, exp, portid, report);
    nf_ct_expect_put(exp);
    NF_CT_STAT_INC!(net, expect_delete);
}

unsafe fn nf_ct_expect_dst_hash(n: *const net, tuple: *const nf_conntrack_tuple) -> u32 {
    let mut combined: aligned_combined = core::mem::zeroed();
    let mut hash: u32;
    get_random_once(&raw mut nf_ct_expect_hashrnd as *mut _, core::mem::size_of::<siphash_aligned_key_t>());
    combined.dst_addr = (*tuple).dst.u3;
    combined.net_mix = net_hash_mix(n);
    combined.dport = (*tuple).dst.u_.all as u16;
    combined.l3num = (*tuple).src.l3num;
    combined.protonum = (*tuple).dst.protonum;
    hash = siphash(&combined as *const _ as *const u8, core::mem::size_of_val(&combined), &raw const nf_ct_expect_hashrnd);
    reciprocal_scale(hash, nf_ct_expect_hsize)
}

#[repr(C)] struct aligned_combined { dst_addr: union_nf_inet_addr, net_mix: u32, dport: u16, l3num: u8, protonum: u8 }

unsafe fn nf_ct_exp_equal(tuple: *const nf_conntrack_tuple, i: *const nf_conntrack_expect, zone: *const nf_conntrack_zone, net: *const net) -> bool {
    nf_ct_tuple_mask_cmp(tuple, &(*i).tuple, &(*i).mask) && net_eq(net, read_pnet(&(*i).net)) && nf_ct_exp_zone_equal_any(i, zone)
}

pub unsafe fn __nf_ct_expect_find(net: *mut net, zone: *const nf_conntrack_zone, tuple: *const nf_conntrack_tuple) -> *mut nf_conntrack_expect {
    let cnet = nf_ct_pernet(net); let mut i: *mut nf_conntrack_expect;
    if (*cnet).expect_count == 0 { return core::ptr::null_mut(); }
    let h = nf_ct_expect_dst_hash(net, tuple);
    hlist_for_each_entry_rcu!(i, (*nf_ct_expect_hash.add(h as usize)), hnode, {
        if nf_ct_exp_is_expired(i) { continue; }
        if nf_ct_exp_equal(tuple, i, zone, net) { return i; }
    });
    core::ptr::null_mut()
}

pub unsafe fn nf_ct_expect_find_get(net: *mut net, zone: *const nf_conntrack_zone, tuple: *const nf_conntrack_tuple) -> *mut nf_conntrack_expect {
    rcu_read_lock(); let mut i = __nf_ct_expect_find(net, zone, tuple);
    if !i.is_null() && !refcount_inc_not_zero(&mut (*i).use_) { i = core::ptr::null_mut(); }
    rcu_read_unlock(); i
}

pub unsafe fn nf_ct_find_expectation(net: *mut net, zone: *const nf_conntrack_zone, tuple: *const nf_conntrack_tuple, unlink: bool) -> *mut nf_conntrack_expect {
    let cnet = nf_ct_pernet(net); let mut i: *mut nf_conntrack_expect; let mut exp = core::ptr::null_mut(); let mut next: *mut hlist_node;
    lockdep_nfct_expect_lock_held(); if (*cnet).expect_count == 0 { return exp; }
    let h = nf_ct_expect_dst_hash(net, tuple);
    hlist_for_each_entry_safe!(i, next, (*nf_ct_expect_hash.add(h as usize)), hnode, {
        if nf_ct_exp_is_expired(i) { nf_ct_unlink_expect(i); continue; }
        if ((*i).flags & NF_CT_EXPECT_INACTIVE) == 0 && nf_ct_exp_equal(tuple, i, zone, net) { exp = i; break; }
    });
    if exp.is_null() || !refcount_inc_not_zero(&mut (*exp).use_) { return core::ptr::null_mut(); }
    if !nf_ct_is_confirmed((*exp).master) { nf_ct_expect_put(exp); return core::ptr::null_mut(); }
    if unlikely(nf_ct_is_dying((*exp).master) || !refcount_inc_not_zero(&mut (*(*exp).master).ct_general.use_)) { nf_ct_expect_put(exp); return core::ptr::null_mut(); }
    if ((*exp).flags & NF_CT_EXPECT_PERMANENT) != 0 || !unlink { return exp; }
    nf_ct_unlink_expect(exp); exp
}

pub unsafe fn nf_ct_remove_expectations(ct: *mut nf_conn) {
    let help = nfct_help(ct); if help.is_null() { return; }
    let mut exp: *mut nf_conntrack_expect; let mut next: *mut hlist_node;
    spin_lock_bh(&raw mut nf_conntrack_expect_lock);
    hlist_for_each_entry_safe!(exp, next, &mut (*help).expectations, lnode, { nf_ct_unlink_expect(exp); });
    spin_unlock_bh(&raw mut nf_conntrack_expect_lock);
}

unsafe fn expect_clash(a: *const nf_conntrack_expect, b: *const nf_conntrack_expect) -> i32 {
    let mut m: nf_conntrack_tuple_mask = core::mem::zeroed(); m.src.u_.all = (*a).mask.src.u_.all & (*b).mask.src.u_.all;
    for count in 0..NF_CT_TUPLE_L3SIZE { m.src.u3.all[count] = (*a).mask.src.u3.all[count] & (*b).mask.src.u3.all[count]; }
    (nf_ct_tuple_mask_cmp(&(*a).tuple, &(*b).tuple, &m) && net_eq(nf_ct_net((*a).master), nf_ct_net((*b).master)) && nf_ct_zone_equal_any((*a).master, nf_ct_zone((*b).master))) as i32
}
unsafe fn expect_matches(a: *const nf_conntrack_expect, b: *const nf_conntrack_expect) -> i32 { (nf_ct_tuple_equal(&(*a).tuple, &(*b).tuple) && nf_ct_tuple_mask_equal(&(*a).mask, &(*b).mask) && net_eq(nf_ct_net((*a).master), nf_ct_net((*b).master)) && nf_ct_zone_equal_any((*a).master, nf_ct_zone((*b).master))) as i32 }
unsafe fn master_matches(a: *const nf_conntrack_expect, b: *const nf_conntrack_expect, flags: u32) -> bool { flags & NF_CT_EXP_F_SKIP_MASTER != 0 || (*a).master == (*b).master }

pub unsafe fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect) { spin_lock_bh(&raw mut nf_conntrack_expect_lock); WRITE_ONCE!((*exp).flags, (*exp).flags | NF_CT_EXPECT_DEAD); spin_unlock_bh(&raw mut nf_conntrack_expect_lock); }

pub unsafe fn nf_ct_expect_alloc(me: *mut nf_conn) -> *mut nf_conntrack_expect {
    let new = kmem_cache_zalloc(nf_ct_expect_cachep, GFP_ATOMIC); if new.is_null() { return core::ptr::null_mut(); }
    (*new).timeout = nfct_time_stamp; (*new).master = me; refcount_set(&mut (*new).use_, 1); new
}

pub unsafe fn nf_ct_expect_put(exp: *mut nf_conntrack_expect) { if refcount_dec_and_test(&mut (*exp).use_) { call_rcu(&mut (*exp).rcu, nf_ct_expect_free_rcu); } }
unsafe fn nf_ct_expect_free_rcu(head: *mut rcu_head) { let exp = container_of!(head, nf_conntrack_expect, rcu); kmem_cache_free(nf_ct_expect_cachep, exp); }

pub unsafe fn nf_ct_expect_init(exp: *mut nf_conntrack_expect, class: u32, family: u8, saddr: *const union_nf_inet_addr, daddr: *const union_nf_inet_addr, proto: u8, src: *const __be16, dst: *const __be16) {
    let ct = (*exp).master; let len = if family == AF_INET { 4 } else { 16 };
    (*exp).flags = 0; (*exp).class = class; (*exp).expectfn = None;
    let ecache = nf_ct_ecache_find(ct); if !ecache.is_null() { (*exp).event_mask = (*ecache).expmask; }
    let help = nfct_help(ct); let helper = if help.is_null() { core::ptr::null_mut() } else { rcu_dereference((*help).helper) };
    rcu_assign_pointer!((*exp).helper, helper); rcu_assign_pointer!((*exp).assign_helper, core::ptr::null_mut()); write_pnet(&mut (*exp).net, read_pnet(&(*ct).ct_net));
    (*exp).tuple.src.l3num = family; (*exp).tuple.dst.protonum = proto; (*exp).master_tuple = (*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple;
    if !saddr.is_null() { memcpy(&mut (*exp).tuple.src.u3 as *mut _ as *mut u8, saddr as *const u8, len); memset(&mut (*exp).mask.src.u3 as *mut _ as *mut u8, 0xff, len); } else { memset(&mut (*exp).tuple.src.u3 as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*exp).tuple.src.u3)); memset(&mut (*exp).mask.src.u3 as *mut _ as *mut u8, 0, core::mem::size_of_val(&(*exp).mask.src.u3)); }
    if !src.is_null() { (*exp).tuple.src.u_.all = *src; (*exp).mask.src.u_.all = htons(0xffff); } else { (*exp).tuple.src.u_.all = 0; (*exp).mask.src.u_.all = 0; }
    memcpy(&mut (*exp).tuple.dst.u3 as *mut _ as *mut u8, daddr as *const u8, len); (*exp).tuple.dst.u_.all = *dst;
}

pub unsafe fn nf_ct_expect_iterate_destroy(iter: Option<unsafe fn(*mut nf_conntrack_expect,*mut core::ffi::c_void)->bool>, data: *mut core::ffi::c_void) { spin_lock_bh(&raw mut nf_conntrack_expect_lock); for i in 0..nf_ct_expect_hsize { let mut e: *mut nf_conntrack_expect; let mut n: *mut hlist_node; hlist_for_each_entry_safe!(e,n,(*nf_ct_expect_hash.add(i as usize)),hnode,{ if iter.unwrap()(e,data) { nf_ct_unlink_expect(e); }}); } spin_unlock_bh(&raw mut nf_conntrack_expect_lock); }

pub unsafe fn nf_conntrack_expect_pernet_init(net: *mut net) -> i32 { exp_proc_init(net) }
pub unsafe fn nf_conntrack_expect_pernet_fini(net: *mut net) { exp_proc_remove(net); }
unsafe fn exp_proc_init(_net: *mut net) -> i32 { 0 }
unsafe fn exp_proc_remove(_net: *mut net) {}

pub unsafe fn nf_ct_expect_related_report(expect: *mut nf_conntrack_expect, portid: u32, report: i32, _flags: u32) -> i32 {
    let help = nfct_help((*expect).master); if help.is_null() { return -ESHUTDOWN; }
    spin_lock_bh(&raw mut nf_conntrack_expect_lock); (*help).expecting[(*expect).class as usize] += 1; nf_ct_expect_insert(expect, help); nf_ct_expect_event_report(IPEXP_NEW, expect, portid, report); spin_unlock_bh(&raw mut nf_conntrack_expect_lock); 0
}
unsafe fn nf_ct_expect_insert(exp: *mut nf_conntrack_expect, help: *mut nf_conn_help) { refcount_inc(&mut (*exp).use_); hlist_add_head_rcu(&mut (*exp).lnode, &mut (*help).expectations); let h=nf_ct_expect_dst_hash(nf_ct_exp_net(exp),&(*exp).tuple); hlist_add_head_rcu(&mut (*exp).hnode, &mut *nf_ct_expect_hash.add(h as usize)); (*nf_ct_pernet(nf_ct_exp_net(exp))).expect_count += 1; }
pub unsafe fn nf_ct_expect_related_pair(_expect: *mut *mut nf_conntrack_expect, _flags: u32) -> i32 { -EINVAL }

pub unsafe fn nf_conntrack_expect_init() -> i32 {
    if nf_ct_expect_hsize == 0 { nf_ct_expect_hsize = nf_conntrack_htable_size / 256; if nf_ct_expect_hsize == 0 { nf_ct_expect_hsize = 1; } }
    nf_ct_expect_max = nf_ct_expect_hsize * 4; nf_ct_expect_cachep = KMEM_CACHE!(nf_conntrack_expect, 0); if nf_ct_expect_cachep.is_null() { return -ENOMEM; }
    nf_ct_expect_hash = nf_ct_alloc_hashtable(&mut nf_ct_expect_hsize, 0); if nf_ct_expect_hash.is_null() { kmem_cache_destroy(nf_ct_expect_cachep); return -ENOMEM; } 0
}
pub unsafe fn nf_conntrack_expect_fini() { rcu_barrier(); kmem_cache_destroy(nf_ct_expect_cachep); kvfree(nf_ct_expect_hash as *mut core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
