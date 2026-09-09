/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by the corresponding kernel headers are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_nat_manip_type {
    NF_NAT_MANIP_SRC,
    NF_NAT_MANIP_DST,
}

/* SRC manip occurs POST_ROUTING or LOCAL_IN */
#[inline]
pub const fn HOOK2MANIP(hooknum: u32) -> bool {
    hooknum != NF_INET_POST_ROUTING && hooknum != NF_INET_LOCAL_IN
}

/* per conntrack: nat application helper private data */
#[repr(C)]
pub union nf_conntrack_nat_help {
    // CONFIG_NF_NAT_PPTP-dependent member from the original header.
    #[cfg(feature = "CONFIG_NF_NAT_PPTP")]
    pub nat_pptp_info: nf_nat_pptp,
}

/* The structure embedded in the conntrack structure. */
#[repr(C)]
pub struct nf_conn_nat {
    pub help: nf_conntrack_nat_help,
    // CONFIG_NF_NAT_MASQUERADE-dependent member from the original header.
    #[cfg(feature = "CONFIG_NF_NAT_MASQUERADE")]
    pub masq_index: i32,
}

/* Set up the info structure to map into this range. */
extern "C" {
    pub fn nf_nat_setup_info(
        ct: *mut nf_conn,
        range: *const nf_nat_range2,
        maniptype: nf_nat_manip_type,
    ) -> u32;

    pub fn nf_nat_alloc_null_binding(ct: *mut nf_conn, hooknum: u32) -> u32;

    pub fn nf_ct_nat_ext_add(ct: *mut nf_conn) -> *mut nf_conn_nat;
}

#[inline]
pub unsafe fn nfct_nat(ct: *const nf_conn) -> *mut nf_conn_nat {
    #[cfg(feature = "CONFIG_NF_NAT")]
    {
        nf_ct_ext_find(ct, NF_CT_EXT_NAT)
    }
    #[cfg(not(feature = "CONFIG_NF_NAT"))]
    {
        core::ptr::null_mut()
    }
}

#[inline]
pub unsafe fn nf_nat_oif_changed(
    hooknum: u32,
    ctinfo: ip_conntrack_info,
    nat: *mut nf_conn_nat,
    out: *const net_device,
) -> bool {
    #[cfg(feature = "CONFIG_NF_NAT_MASQUERADE")]
    {
        !nat.is_null()
            && (*nat).masq_index != 0
            && hooknum == NF_INET_POST_ROUTING
            && CTINFO2DIR(ctinfo) == IP_CT_DIR_ORIGINAL
            && (*nat).masq_index != (*out).ifindex
    }
    #[cfg(not(feature = "CONFIG_NF_NAT_MASQUERADE"))]
    {
        false
    }
}

extern "C" {
    pub fn nf_nat_register_fn(
        net: *mut net,
        pf: u8,
        ops: *const nf_hook_ops,
        nat_ops: *const nf_hook_ops,
        ops_count: u32,
    ) -> i32;
    pub fn nf_nat_unregister_fn(
        net: *mut net,
        pf: u8,
        ops: *const nf_hook_ops,
        ops_count: u32,
    );

    pub fn nf_nat_packet(
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        hooknum: u32,
        skb: *mut sk_buff,
    ) -> u32;
    pub fn nf_nat_manip_pkt(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        mtype: nf_nat_manip_type,
        dir: ip_conntrack_dir,
    ) -> u32;
    pub fn nf_nat_csum_recalc(
        skb: *mut sk_buff,
        nfproto: u8,
        proto: u8,
        data: *mut core::ffi::c_void,
        check: *mut __sum16,
        datalen: i32,
        oldlen: i32,
    );

    pub fn nf_nat_icmp_reply_translation(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        hooknum: u32,
    ) -> i32;
    pub fn nf_nat_icmpv6_reply_translation(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        hooknum: u32,
        hdrlen: u32,
    ) -> i32;

    pub fn nf_nat_ipv4_register_fn(net: *mut net, ops: *const nf_hook_ops) -> i32;
    pub fn nf_nat_ipv4_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
    pub fn nf_nat_ipv6_register_fn(net: *mut net, ops: *const nf_hook_ops) -> i32;
    pub fn nf_nat_ipv6_unregister_fn(net: *mut net, ops: *const nf_hook_ops);
    pub fn nf_nat_inet_register_fn(net: *mut net, ops: *const nf_hook_ops) -> i32;
    pub fn nf_nat_inet_unregister_fn(net: *mut net, ops: *const nf_hook_ops);

    pub fn nf_nat_inet_fn(
        priv_: *mut core::ffi::c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> u32;

    pub fn nf_ct_nat(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        ctinfo: ip_conntrack_info,
        action: *mut i32,
        range: *const nf_nat_range2,
        commit: bool,
    ) -> i32;
}

#[inline]
pub unsafe fn nf_nat_initialized(ct: *const nf_conn, manip: nf_nat_manip_type) -> i32 {
    if manip == NF_NAT_MANIP_SRC {
        (*ct).status & IPS_SRC_NAT_DONE
    } else {
        (*ct).status & IPS_DST_NAT_DONE
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
