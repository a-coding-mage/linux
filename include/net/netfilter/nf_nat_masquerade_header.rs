/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by <linux/skbuff.h> and <net/netfilter/nf_nat.h>.
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_nat_range2 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn nf_nat_masquerade_ipv4(
        skb: *mut sk_buff,
        hooknum: u32,
        range: *const nf_nat_range2,
        out: *const net_device,
    ) -> u32;

    pub fn nf_nat_masquerade_inet_register_notifiers() -> i32;
    pub fn nf_nat_masquerade_inet_unregister_notifiers();

    pub fn nf_nat_masquerade_ipv6(
        skb: *mut sk_buff,
        range: *const nf_nat_range2,
        out: *const net_device,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
