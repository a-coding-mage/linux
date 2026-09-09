// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	Handle firewalling core
 *	Linux ethernet bridge
 *
 *	Authors:
 *	Lennert Buytenhek		<buytenh@gnu.org>
 *	Bart De Schuymer		<bdschuym@pandora.be>
 *
 *	Lennert dedicates this file to Kerstin Wurdinger.
 */

// C dependencies supplied by the surrounding kernel translation unit.

unsafe fn fake_update_pmtu(
    _dst: *mut dst_entry,
    _sk: *mut sock,
    _skb: *mut sk_buff,
    _mtu: u32,
    _confirm_neigh: bool,
) {
}

unsafe fn fake_redirect(_dst: *mut dst_entry, _sk: *mut sock, _skb: *mut sk_buff) {
}

unsafe fn fake_cow_metrics(_dst: *mut dst_entry, _old: libc::c_ulong) -> *mut u32 {
    core::ptr::null_mut()
}

unsafe fn fake_neigh_lookup(
    _dst: *const dst_entry,
    _skb: *mut sk_buff,
    _daddr: *const core::ffi::c_void,
) -> *mut neighbour {
    core::ptr::null_mut()
}

unsafe fn fake_mtu(dst: *const dst_entry) -> libc::c_uint {
    (*dst).dev.mtu
}

static mut fake_dst_ops: dst_ops = dst_ops {
    family: AF_INET,
    update_pmtu: Some(fake_update_pmtu),
    redirect: Some(fake_redirect),
    cow_metrics: Some(fake_cow_metrics),
    neigh_lookup: Some(fake_neigh_lookup),
    mtu: Some(fake_mtu),
};

/*
 * Initialize bogus route table used to keep netfilter happy.
 * Currently, we fill in the PMTU entry because netfilter
 * refragmentation needs it, and the rt_flags entry because
 * ipt_REJECT needs it.  Future netfilter modules might
 * require us to fill additional fields.
 */
unsafe fn br_netfilter_rtable_init(br: *mut net_bridge) {
    let rt: *mut rtable = &mut (*br).fake_rtable;

    rcuref_init(&mut (*rt).dst.__rcuref, 1);
    (*rt).dst.dev = (*br).dev;
    dst_init_metrics(&mut (*rt).dst, (*br).metrics, false);
    dst_metric_set(&mut (*rt).dst, RTAX_MTU, (*br).dev.mtu);
    (*rt).dst.flags = DST_NOXFRM | DST_FAKE_RTABLE;
    (*rt).dst.ops = &raw mut fake_dst_ops;
}

unsafe fn br_nf_core_init() -> libc::c_int {
    dst_entries_init(&raw mut fake_dst_ops)
}

unsafe fn br_nf_core_fini() {
    dst_entries_destroy(&raw mut fake_dst_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
