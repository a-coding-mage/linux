/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the IPv6 netfilter reject header.
// External types are supplied by the corresponding kernel dependencies.

extern "C" {
    pub fn nf_send_unreach6(
        net: *mut net,
        skb_in: *mut sk_buff,
        code: u8,
        hooknum: u32,
    );

    pub fn nf_send_reset6(
        net: *mut net,
        sk: *mut sock,
        oldskb: *mut sk_buff,
        hook: i32,
    );

    pub fn nf_reject_skb_v6_tcp_reset(
        net: *mut net,
        oldskb: *mut sk_buff,
        dev: *const net_device,
        hook: i32,
    ) -> *mut sk_buff;

    pub fn nf_reject_skb_v6_unreach(
        net: *mut net,
        oldskb: *mut sk_buff,
        dev: *const net_device,
        hook: i32,
        code: u8,
    ) -> *mut sk_buff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
