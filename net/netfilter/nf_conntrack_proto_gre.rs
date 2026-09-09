// SPDX-License-Identifier: GPL-2.0-only
/*
 * Connection tracking protocol helper module for GRE.
 *
 * GRE is a generic encapsulation protocol, which is generally not very
 * suited for NAT, as it has no protocol-specific part as port numbers.
 *
 * It has an optional key field, which may help us distinguishing two
 * connections between the same two hosts.
 *
 * GRE is defined in RFC 1701 and RFC 1702, as well as RFC 2784
 *
 * PPTP is built on top of a modified version of GRE, and has a mandatory
 * field called "CallID", which serves us for the same purpose as the key
 * field in plain GRE.
 *
 * Documentation about PPTP can be found in RFC 2637
 *
 * (C) 2000-2005 by Harald Welte <laforge@gnumonks.org>
 *
 * Development of this code funded by Astaro AG (http://www.astaro.com/)
 *
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

static const GRE_CT_MAX: usize = 2;
static const gre_timeouts: [u32; GRE_CT_MAX] = [30 * HZ, 180 * HZ];

// used when expectation is added
static mut keymap_lock: DEFINE_SPINLOCK = DEFINE_SPINLOCK_INIT;

#[inline]
unsafe fn gre_pernet(net: *mut net) -> *mut nf_gre_net {
    &mut (*net).ct.nf_ct_proto.gre
}

#[inline]
unsafe fn gre_key_cmpfn(km: *const nf_ct_gre_keymap, t: *const nf_conntrack_tuple) -> bool {
    (*km).tuple.src.l3num == (*t).src.l3num
        && libc::memcmp(
            &(*km).tuple.src.u3 as *const _ as *const libc::c_void,
            &(*t).src.u3 as *const _ as *const libc::c_void,
            core::mem::size_of_val(&(*t).src.u3),
        ) == 0
        && libc::memcmp(
            &(*km).tuple.dst.u3 as *const _ as *const libc::c_void,
            &(*t).dst.u3 as *const _ as *const libc::c_void,
            core::mem::size_of_val(&(*t).dst.u3),
        ) == 0
        && (*km).tuple.dst.protonum == (*t).dst.protonum
        && (*km).tuple.dst.u.all == (*t).dst.u.all
}

/* look up the source key for a given tuple */
unsafe fn gre_keymap_lookup(net: *mut net, t: *mut nf_conntrack_tuple) -> __be16 {
    let net_gre = gre_pernet(net);
    let mut key: __be16 = 0;
    list_for_each_entry_rcu!(km, &(*net_gre).keymap_list, list, {
        if gre_key_cmpfn(km, t) {
            key = (*km).tuple.src.u.gre.key;
            break;
        }
    });
    pr_debug!("lookup src key 0x%x for ", key);
    nf_ct_dump_tuple(t);
    key
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum nf_ct_gre_km_act { NF_CT_GRE_KM_NEW, NF_CT_GRE_KM_BAD, NF_CT_GRE_KM_DUP }

unsafe fn nf_ct_gre_km_acceptable(
    ct_pptp_info: *const nf_ct_pptp_master,
    orig: *const nf_conntrack_tuple,
    repl: *const nf_conntrack_tuple,
) -> nf_ct_gre_km_act {
    lockdep_assert_held!(&keymap_lock);
    let km_orig = (*ct_pptp_info).keymap[IP_CT_DIR_ORIGINAL];
    let km_repl = (*ct_pptp_info).keymap[IP_CT_DIR_REPLY];
    if !km_orig.is_null() && !km_repl.is_null() {
        if !gre_key_cmpfn(km_orig, orig) { return nf_ct_gre_km_act::NF_CT_GRE_KM_BAD; }
        if !gre_key_cmpfn(km_repl, repl) { return nf_ct_gre_km_act::NF_CT_GRE_KM_BAD; }
        return nf_ct_gre_km_act::NF_CT_GRE_KM_DUP;
    }
    DEBUG_NET_WARN_ON_ONCE!(km_orig);
    DEBUG_NET_WARN_ON_ONCE!(km_repl);
    nf_ct_gre_km_act::NF_CT_GRE_KM_NEW
}

/* add keymap entries, associate with specified master ct */
pub unsafe fn nf_ct_gre_keymap_add(ct: *mut nf_conn, orig: *const nf_conntrack_tuple, repl: *const nf_conntrack_tuple) -> bool {
    let net = nf_ct_net(ct);
    let net_gre = gre_pernet(net);
    let info = nfct_help_data(ct) as *mut nf_ct_pptp_master;
    if info.is_null() { return false; }
    let km_orig = kmalloc_obj!(nf_ct_gre_keymap, GFP_ATOMIC);
    if km_orig.is_null() { return false; }
    let km_repl = kmalloc_obj!(nf_ct_gre_keymap, GFP_ATOMIC);
    if km_repl.is_null() { kfree(km_orig); return false; }
    core::ptr::copy_nonoverlapping(orig, &mut (*km_orig).tuple, 1);
    core::ptr::copy_nonoverlapping(repl, &mut (*km_repl).tuple, 1);
    spin_lock_bh!(&keymap_lock);
    if nf_ct_is_dying(ct) { goto_unlock_free!(km_orig, km_repl); }
    match nf_ct_gre_km_acceptable(info, orig, repl) {
        nf_ct_gre_km_act::NF_CT_GRE_KM_NEW => {},
        nf_ct_gre_km_act::NF_CT_GRE_KM_DUP => { spin_unlock_bh!(&keymap_lock); kfree(km_orig); kfree(km_repl); return true; },
        nf_ct_gre_km_act::NF_CT_GRE_KM_BAD => { pr_debug!("trying to override keymap for ct %p\n", ct); goto_unlock_free!(km_orig, km_repl); }
    }
    if !(*info).keymap[IP_CT_DIR_ORIGINAL].is_null() || !(*info).keymap[IP_CT_DIR_REPLY].is_null() { goto_unlock_free!(km_orig, km_repl); }
    pr_debug!("adding new entries %p,%p: ", km_orig, km_repl);
    nf_ct_dump_tuple(&(*km_orig).tuple); nf_ct_dump_tuple(&(*km_repl).tuple);
    list_add_tail_rcu!(&mut (*km_orig).list, &mut (*net_gre).keymap_list);
    list_add_tail_rcu!(&mut (*km_repl).list, &mut (*net_gre).keymap_list);
    (*info).keymap[IP_CT_DIR_ORIGINAL] = km_orig; (*info).keymap[IP_CT_DIR_REPLY] = km_repl;
    spin_unlock_bh!(&keymap_lock); true
}

/* destroy the keymap entries associated with specified master ct */
pub unsafe fn nf_ct_gre_keymap_destroy(ct: *mut nf_conn) {
    let info = nfct_help_data(ct) as *mut nf_ct_pptp_master;
    if info.is_null() { return; }
    pr_debug!("entering for ct %p\n", ct); spin_lock_bh!(&keymap_lock);
    for dir in IP_CT_DIR_ORIGINAL..IP_CT_DIR_MAX { let km = (*info).keymap[dir]; if !km.is_null() { pr_debug!("removing %p from list\n", km); list_del_rcu!(&mut (*km).list); kfree_rcu!(km, rcu); (*info).keymap[dir] = core::ptr::null_mut(); } }
    spin_unlock_bh!(&keymap_lock);
}

/* PUBLIC CONNTRACK PROTO HELPER FUNCTIONS */

/* gre hdr info to tuple */
pub unsafe fn gre_pkt_to_tuple(skb: *const sk_buff, dataoff: u32, net: *mut net, tuple: *mut nf_conntrack_tuple) -> bool {
    let mut grehdr = core::mem::MaybeUninit::<gre_base_hdr>::uninit();
    let grehdr = skb_header_pointer(skb, dataoff, core::mem::size_of::<gre_base_hdr>(), grehdr.as_mut_ptr());
    if grehdr.is_null() || ((*grehdr).flags & GRE_VERSION) != GRE_VERSION_1 { (*tuple).src.u.all = 0; (*tuple).dst.u.all = 0; return true; }
    let mut pgrehdr = core::mem::MaybeUninit::<pptp_gre_header>::uninit();
    let pgrehdr = skb_header_pointer(skb, dataoff, 8, pgrehdr.as_mut_ptr());
    if pgrehdr.is_null() { return true; }
    if (*grehdr).protocol != GRE_PROTO_PPP { pr_debug!("Unsupported GRE proto(0x%x)\n", ntohs((*grehdr).protocol)); return false; }
    (*tuple).dst.u.gre.key = (*pgrehdr).call_id;
    (*tuple).src.u.gre.key = gre_keymap_lookup(net, tuple); true
}

#[cfg(CONFIG_NF_CONNTRACK_PROCFS)]
unsafe fn gre_print_conntrack(s: *mut seq_file, ct: *mut nf_conn) { seq_printf!(s, "timeout=%u, stream_timeout=%u ", (*ct).proto.gre.timeout / HZ, (*ct).proto.gre.stream_timeout / HZ); }

unsafe fn gre_get_timeouts(net: *mut net) -> *mut u32 { (*gre_pernet(net)).timeouts.as_mut_ptr() }

/* Returns verdict for packet, and may modify conntrack */
pub unsafe fn nf_conntrack_gre_packet(ct: *mut nf_conn, skb: *mut sk_buff, _dataoff: u32, ctinfo: ip_conntrack_info, _state: *const nf_hook_state) -> i32 {
    if !nf_ct_is_confirmed(ct) { let mut timeouts = nf_ct_timeout_lookup(ct); if timeouts.is_null() { timeouts = gre_get_timeouts(nf_ct_net(ct)); } (*ct).proto.gre.stream_timeout = *timeouts.add(GRE_CT_REPLIED); (*ct).proto.gre.timeout = *timeouts.add(GRE_CT_UNREPLIED); }
    let status = READ_ONCE!((*ct).status);
    if status & IPS_SEEN_REPLY != 0 { nf_ct_refresh_acct(ct, ctinfo, skb, (*ct).proto.gre.stream_timeout); if status & IPS_NAT_CLASH != 0 { return NF_ACCEPT; } if !test_and_set_bit(IPS_ASSURED_BIT, &mut (*ct).status) { nf_conntrack_event_cache(IPCT_ASSURED, ct); } } else { nf_ct_refresh_acct(ct, ctinfo, skb, (*ct).proto.gre.timeout); }
    NF_ACCEPT
}

/* Conditional kernel configuration sections are preserved by cfg attributes. */

#[cfg(IS_ENABLED_CONFIG_NF_CONNTRACK_PPTP)]
unsafe fn destroy_sibling_or_exp(net: *mut net, ct: *mut nf_conn, t: *const nf_conntrack_tuple) -> i32 {
    pr_debug!("trying to timeout ct or exp for tuple "); nf_ct_dump_tuple(t);
    let zone = nf_ct_zone(ct); let h = nf_conntrack_find_get(net, zone, t);
    if !h.is_null() { let sibling = nf_ct_tuplehash_to_ctrack(h); pr_debug!("setting timeout of conntrack %p to 0\n", sibling); (*sibling).proto.gre.timeout = 0; (*sibling).proto.gre.stream_timeout = 0; nf_ct_kill(sibling); nf_ct_put(sibling); return 1; }
    let exp = nf_ct_expect_find_get(net, zone, t); if !exp.is_null() { pr_debug!("unexpect_related of expect %p\n", exp); nf_ct_unexpect_related(exp); nf_ct_expect_put(exp); return 1; } 0
}

#[cfg(IS_ENABLED_CONFIG_NF_CONNTRACK_PPTP)]
pub unsafe fn gre_pptp_destroy_siblings(ct: *mut nf_conn) {
    let net = nf_ct_net(ct); let info = nfct_help_data(ct) as *const nf_ct_pptp_master; if info.is_null() { return; }
    nf_ct_gre_keymap_destroy(ct); let mut t: nf_conntrack_tuple = core::mem::zeroed();
    core::ptr::copy_nonoverlapping(&(*ct).tuplehash[IP_CT_DIR_ORIGINAL].tuple, &mut t, 1); t.dst.protonum = IPPROTO_GRE; t.src.u.gre.key = (*info).pns_call_id; t.dst.u.gre.key = (*info).pac_call_id; if destroy_sibling_or_exp(net, ct, &t) == 0 { pr_debug!("failed to timeout original pns->pac ct/exp\n"); }
    core::ptr::copy_nonoverlapping(&(*ct).tuplehash[IP_CT_DIR_REPLY].tuple, &mut t, 1); t.dst.protonum = IPPROTO_GRE; t.src.u.gre.key = (*info).pac_call_id; t.dst.u.gre.key = (*info).pns_call_id; if destroy_sibling_or_exp(net, ct, &t) == 0 { pr_debug!("failed to timeout reply pac->pns ct/exp\n"); }
}

pub unsafe fn nf_conntrack_gre_init_net(net: *mut net) {
    let net_gre = gre_pernet(net); INIT_LIST_HEAD!(&mut (*net_gre).keymap_list); for i in 0..GRE_CT_MAX { (*net_gre).timeouts[i] = gre_timeouts[i]; }
}

/* protocol helper struct */
pub static nf_conntrack_l4proto_gre: nf_conntrack_l4proto = nf_conntrack_l4proto {
    l4proto: IPPROTO_GRE, allow_clash: true,
    // Optional procfs, netlink, and timeout callbacks are supplied by build configuration.
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
