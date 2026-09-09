// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match EUI64 address parameters. */

/* (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu>
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/module.h, linux/skbuff.h, linux/ipv6.h, linux/if_arp.h,
// linux/if_ether.h, linux/netfilter/x_tables.h, and
// linux/netfilter_ipv6/ip6_tables.h.

use core::ffi::{c_int, c_uchar, c_uint, c_void};

const ETH_HLEN: usize = 14;
const ETH_P_IPV6: u16 = 0x86dd;
const ARPHRD_ETHER: u16 = 1;
const NFPROTO_IPV6: u8 = 10;
const NF_INET_PRE_ROUTING: c_uint = 0;
const NF_INET_LOCAL_IN: c_uint = 1;
const NF_INET_FORWARD: c_uint = 2;

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_device {
    pub type_: u16,
}

#[repr(C)]
pub struct xt_action_param {
    pub hotdrop: bool,
}

#[repr(C)]
pub struct ethhdr {
    pub h_dest: [c_uchar; 6],
    pub h_source: [c_uchar; 6],
    pub h_proto: u16,
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [c_uchar; 16],
}

#[repr(C)]
pub struct ipv6hdr {
    pub version: u8,
}

#[repr(C)]
pub struct xt_match {
    pub name: *const u8,
    pub family: u8,
    pub match_: Option<unsafe extern "C" fn(*const sk_buff, *mut xt_action_param) -> bool>,
    pub matchsize: usize,
    pub hooks: c_uint,
    pub me: *mut c_void,
}

extern "C" {
    fn skb_mac_header_was_set(skb: *const sk_buff) -> bool;
    fn skb_mac_header_len(skb: *const sk_buff) -> usize;
    fn eth_hdr(skb: *const sk_buff) -> *const ethhdr;
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn htons(value: u16) -> u16;
    fn xt_register_match(m: *mut xt_match) -> c_int;
    fn xt_unregister_match(m: *mut xt_match);
}

// MODULE_DESCRIPTION("Xtables: IPv6 EUI64 address match");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Andras Kis-Szabo <kisza@sch.bme.hu>");

unsafe extern "C" fn eui64_mt6(
    skb: *const sk_buff,
    par: *mut xt_action_param,
) -> bool {
    let mut eui64 = [0u8; 8];

    if (*skb).dev.is_null() || (*(*skb).dev).type_ != ARPHRD_ETHER {
        return false;
    }

    if !skb_mac_header_was_set(skb) || skb_mac_header_len(skb) < ETH_HLEN {
        (*par).hotdrop = true;
        return false;
    }

    if (*eth_hdr(skb)).h_proto == htons(ETH_P_IPV6) {
        if (*ipv6_hdr(skb)).version == 0x6 {
            let ethernet = eth_hdr(skb);
            eui64[0..3].copy_from_slice(&(*ethernet).h_source[0..3]);
            eui64[5..8].copy_from_slice(&(*ethernet).h_source[3..6]);
            eui64[3] = 0xff;
            eui64[4] = 0xfe;
            eui64[0] ^= 0x02;

            // The IPv6 source address and its layout are supplied by the
            // surrounding kernel translation.
            let saddr = (ipv6_hdr(skb) as *const u8).add(8);
            if core::slice::from_raw_parts(saddr, 8) == &eui64[..] {
                return true;
            }
        }
    }

    false
}

static mut eui64_mt6_reg: xt_match = xt_match {
    name: b"eui64\0".as_ptr(),
    family: NFPROTO_IPV6,
    match_: Some(eui64_mt6),
    matchsize: core::mem::size_of::<c_int>(),
    hooks: (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_FORWARD),
    me: core::ptr::null_mut(),
};

unsafe extern "C" fn eui64_mt6_init() -> c_int {
    xt_register_match(&mut eui64_mt6_reg)
}

unsafe extern "C" fn eui64_mt6_exit() {
    xt_unregister_match(&mut eui64_mt6_reg);
}

// module_init(eui64_mt6_init);
// module_exit(eui64_mt6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
