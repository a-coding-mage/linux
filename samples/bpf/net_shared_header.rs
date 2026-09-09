// SPDX-License-Identifier: GPL-2.0

pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 10;

pub const ETH_ALEN: usize = 6;
pub const ETH_P_802_3_MIN: u16 = 0x0600;
pub const ETH_P_8021Q: u16 = 0x8100;
pub const ETH_P_8021AD: u16 = 0x88A8;
pub const ETH_P_IP: u16 = 0x0800;
pub const ETH_P_IPV6: u16 = 0x86DD;
pub const ETH_P_ARP: u16 = 0x0806;
pub const IPPROTO_ICMPV6: i32 = 58;

pub const TC_ACT_OK: i32 = 0;
pub const TC_ACT_SHOT: i32 = 2;

pub const IFNAMSIZ: usize = 16;

// The C header selects byte swapping or identity based on compiler endianness.
#[cfg(target_endian = "little")]
#[inline]
pub const fn bpf_ntohs(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(target_endian = "little")]
#[inline]
pub const fn bpf_htons(x: u16) -> u16 {
    x.swap_bytes()
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn bpf_ntohs(x: u16) -> u16 {
    x
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn bpf_htons(x: u16) -> u16 {
    x
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
