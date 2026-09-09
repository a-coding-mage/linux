/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET\t\tAn implementation of the TCP/IP protocol suite for the LINUX
 *\t\toperating system.  INET is implemented using the  BSD Socket
 *\t\tinterface as the means of communication with the user level.
 *
 *\t\tChecksumming functions for IPv6
 *
 * Authors:\tJorge Cwik, <jorge@laser.satlink.net>
 *\t\tArnt Gulbrandsen, <agulbra@nvg.unit.no>
 *\t\tBorrows very liberally from tcp.c and ip.c, see those
 *\t\tfiles for more names.
 */

/*
 *\tFixes:
 *
 * Ralf Baechle\t\t\t:\tgeneric ipv6 checksum
 * <ralf@waldorf-gmbh.de>
 */

// C includes: <asm/types.h>, <asm/byteorder.h>, <net/ip.h>,
// <asm/checksum.h>, <linux/in6.h>, <linux/tcp.h>, and <linux/ipv6.h>.
// Their declarations are supplied by the surrounding translation unit.

// C conditional declaration: preserved from !_HAVE_ARCH_IPV6_CSUM.
#[cfg(not(have_arch_ipv6_csum))]
unsafe extern "C" {
    pub fn csum_ipv6_magic(
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: u32,
        proto: u8,
        csum: __wsum,
    ) -> __sum16;
}

#[inline]
pub unsafe fn ip6_compute_pseudo(skb: *mut sk_buff, proto: i32) -> __wsum {
    (!csum_unfold(csum_ipv6_magic(
        &(*ipv6_hdr(skb)).saddr,
        &(*ipv6_hdr(skb)).daddr,
        (*skb).len,
        proto as u8,
        0,
    ))) as __wsum
}

#[inline]
pub unsafe fn tcp_v6_check(
    len: i32,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    base: __wsum,
) -> __sum16 {
    csum_ipv6_magic(saddr, daddr, len as u32, IPPROTO_TCP as u8, base)
}

#[inline]
pub unsafe fn __tcp_v6_send_check(
    skb: *mut sk_buff,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
) {
    let th: *mut tcphdr = tcp_hdr(skb);

    (*th).check = (!tcp_v6_check((*skb).len as i32, saddr, daddr, 0)) as __sum16;
    (*skb).csum_start = (skb_transport_header(skb) as usize - (*skb).head as usize) as _;
    (*skb).csum_offset = core::mem::offset_of!(tcphdr, check) as _;
}

#[inline]
pub unsafe fn tcp_v6_gso_csum_prep(skb: *mut sk_buff) {
    let ipv6h: *mut ipv6hdr = ipv6_hdr(skb);
    let th: *mut tcphdr = tcp_hdr(skb);

    (*ipv6h).payload_len = 0;
    (*th).check = (!tcp_v6_check(
        0,
        &(*ipv6h).saddr,
        &(*ipv6h).daddr,
        0,
    )) as __sum16;
}

#[inline]
pub unsafe fn udp_v6_check(
    len: i32,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    base: __wsum,
) -> __sum16 {
    csum_ipv6_magic(saddr, daddr, len as u32, IPPROTO_UDP as u8, base)
}

unsafe extern "C" {
    pub fn udp6_set_csum(
        nocheck: bool,
        skb: *mut sk_buff,
        saddr: *const in6_addr,
        daddr: *const in6_addr,
        len: i32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
