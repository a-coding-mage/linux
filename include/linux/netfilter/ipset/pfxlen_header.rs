/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// asm/byteorder.h, linux/netfilter.h, and net/tcp.h.

/* Prefixlen maps, by Jan Engelhardt */
extern "C" {
    pub static ip_set_netmask_map: *const crate::nf_inet_addr;
    pub static ip_set_hostmask_map: *const crate::nf_inet_addr;
}

#[inline]
pub unsafe fn ip_set_netmask(pfxlen: u8) -> u32 {
    (*ip_set_netmask_map.add(pfxlen as usize)).ip
}

#[inline]
pub unsafe fn ip_set_netmask6(pfxlen: u8) -> *const u32 {
    (*ip_set_netmask_map.add(pfxlen as usize)).ip6.as_ptr()
}

#[inline]
pub unsafe fn ip_set_hostmask(pfxlen: u8) -> u32 {
    (*ip_set_hostmask_map.add(pfxlen as usize)).ip
}

#[inline]
pub unsafe fn ip_set_hostmask6(pfxlen: u8) -> *const u32 {
    (*ip_set_hostmask_map.add(pfxlen as usize)).ip6.as_ptr()
}

extern "C" {
    pub fn ip_set_range_to_cidr(from: u32, to: u32, cidr: *mut u8) -> u32;
}

#[macro_export]
macro_rules! ip_set_mask_from_to {
    ($from:expr, $to:expr, $cidr:expr) => {{
        $from &= unsafe { $crate::ip_set_hostmask($cidr) };
        $to = $from | !unsafe { $crate::ip_set_hostmask($cidr) };
    }};
}

#[inline]
pub unsafe fn ip6_netmask(ip: *mut crate::nf_inet_addr, prefix: u8) {
    (*ip).ip6[0] &= *ip_set_netmask6(prefix).add(0);
    (*ip).ip6[1] &= *ip_set_netmask6(prefix).add(1);
    (*ip).ip6[2] &= *ip_set_netmask6(prefix).add(2);
    (*ip).ip6[3] &= *ip_set_netmask6(prefix).add(3);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
