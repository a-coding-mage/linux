/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependencies supplied by the surrounding kernel translation:
 * linux/list.h, linux/skbuff.h, and linux/netdevice.h.
 */

/* nf_conntrack_netlink.c: applied on tuple filters */
pub const CTA_FILTER_F_CTA_IP_SRC: i32 = 1 << 0;
pub const CTA_FILTER_F_CTA_IP_DST: i32 = 1 << 1;
pub const CTA_FILTER_F_CTA_TUPLE_ZONE: i32 = 1 << 2;
pub const CTA_FILTER_F_CTA_PROTO_NUM: i32 = 1 << 3;
pub const CTA_FILTER_F_CTA_PROTO_SRC_PORT: i32 = 1 << 4;
pub const CTA_FILTER_F_CTA_PROTO_DST_PORT: i32 = 1 << 5;
pub const CTA_FILTER_F_CTA_PROTO_ICMP_TYPE: i32 = 1 << 6;
pub const CTA_FILTER_F_CTA_PROTO_ICMP_CODE: i32 = 1 << 7;
pub const CTA_FILTER_F_CTA_PROTO_ICMP_ID: i32 = 1 << 8;
pub const CTA_FILTER_F_CTA_PROTO_ICMPV6_TYPE: i32 = 1 << 9;
pub const CTA_FILTER_F_CTA_PROTO_ICMPV6_CODE: i32 = 1 << 10;
pub const CTA_FILTER_F_CTA_PROTO_ICMPV6_ID: i32 = 1 << 11;
pub const CTA_FILTER_F_MAX: i32 = 1 << 12;
pub const CTA_FILTER_F_ALL: i32 = CTA_FILTER_F_MAX - 1;

#[macro_export]
macro_rules! CTA_FILTER_FLAG {
    (CTA_IP_SRC) => { $crate::CTA_FILTER_F_CTA_IP_SRC };
    (CTA_IP_DST) => { $crate::CTA_FILTER_F_CTA_IP_DST };
    (CTA_TUPLE_ZONE) => { $crate::CTA_FILTER_F_CTA_TUPLE_ZONE };
    (CTA_PROTO_NUM) => { $crate::CTA_FILTER_F_CTA_PROTO_NUM };
    (CTA_PROTO_SRC_PORT) => { $crate::CTA_FILTER_F_CTA_PROTO_SRC_PORT };
    (CTA_PROTO_DST_PORT) => { $crate::CTA_FILTER_F_CTA_PROTO_DST_PORT };
    (CTA_PROTO_ICMP_TYPE) => { $crate::CTA_FILTER_F_CTA_PROTO_ICMP_TYPE };
    (CTA_PROTO_ICMP_CODE) => { $crate::CTA_FILTER_F_CTA_PROTO_ICMP_CODE };
    (CTA_PROTO_ICMP_ID) => { $crate::CTA_FILTER_F_CTA_PROTO_ICMP_ID };
    (CTA_PROTO_ICMPV6_TYPE) => { $crate::CTA_FILTER_F_CTA_PROTO_ICMPV6_TYPE };
    (CTA_PROTO_ICMPV6_CODE) => { $crate::CTA_FILTER_F_CTA_PROTO_ICMPV6_CODE };
    (CTA_PROTO_ICMPV6_ID) => { $crate::CTA_FILTER_F_CTA_PROTO_ICMPV6_ID };
}

/* nf_queue.c */
extern "C" {
    pub fn nf_queue_nf_hook_drop(net: *mut net);

    /* nf_log.c */
    pub fn netfilter_log_init() -> i32;

    /* nf_hooks_lwtunnel.c; conditional on CONFIG_LWTUNNEL in the C build. */
    #[cfg(feature = "CONFIG_LWTUNNEL")]
    pub fn netfilter_lwtunnel_init() -> i32;
    #[cfg(feature = "CONFIG_LWTUNNEL")]
    pub fn netfilter_lwtunnel_fini();

    /* core.c */
    pub fn nf_hook_entries_delete_raw(
        pp: *mut *mut nf_hook_entries,
        reg: *const nf_hook_ops,
    );
    pub fn nf_hook_entries_insert_raw(
        pp: *mut *mut nf_hook_entries,
        reg: *const nf_hook_ops,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
