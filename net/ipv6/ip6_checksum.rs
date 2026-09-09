// SPDX-License-Identifier: GPL-2.0
// Translated from C. Dependencies supplied by the surrounding kernel sources.

// #include <net/ip.h>
// #include <net/ip6_checksum.h>
// #include <net/udp.h>
// #include <asm/checksum.h>

// The C implementation is compiled only when the architecture does not
// provide its own IPv6 checksum helper.
#[cfg(not(_HAVE_ARCH_IPV6_CSUM))]
#[no_mangle]
pub unsafe extern "C" fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    csum: __wsum,
) -> __sum16 {
    let mut carry: i32;
    let ulen: __u32;
    let uproto: __u32;
    let mut sum: __u32 = csum as __u32;

    sum = sum.wrapping_add((*saddr).s6_addr32[0] as __u32);
    carry = (sum < (*saddr).s6_addr32[0] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*saddr).s6_addr32[1] as __u32);
    carry = (sum < (*saddr).s6_addr32[1] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*saddr).s6_addr32[2] as __u32);
    carry = (sum < (*saddr).s6_addr32[2] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*saddr).s6_addr32[3] as __u32);
    carry = (sum < (*saddr).s6_addr32[3] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*daddr).s6_addr32[0] as __u32);
    carry = (sum < (*daddr).s6_addr32[0] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*daddr).s6_addr32[1] as __u32);
    carry = (sum < (*daddr).s6_addr32[1] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*daddr).s6_addr32[2] as __u32);
    carry = (sum < (*daddr).s6_addr32[2] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    sum = sum.wrapping_add((*daddr).s6_addr32[3] as __u32);
    carry = (sum < (*daddr).s6_addr32[3] as __u32) as i32;
    sum = sum.wrapping_add(carry as __u32);

    ulen = htonl(len) as __u32;
    sum = sum.wrapping_add(ulen);
    carry = (sum < ulen) as i32;
    sum = sum.wrapping_add(carry as __u32);

    uproto = htonl(proto as __u32) as __u32;
    sum = sum.wrapping_add(uproto);
    carry = (sum < uproto) as i32;
    sum = sum.wrapping_add(carry as __u32);

    csum_fold(sum as __wsum)
}

// Function to set UDP checksum for an IPv6 UDP packet. This is intended
// for the simple case like when setting the checksum for a UDP tunnel.
#[no_mangle]
pub unsafe extern "C" fn udp6_set_csum(
    nocheck: bool,
    skb: *mut sk_buff,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: i32,
) {
    let uh: *mut udphdr = udp_hdr(skb);

    if nocheck {
        (*uh).check = 0;
    } else if skb_is_gso(skb) {
        (*uh).check = !udp_v6_check(len, saddr, daddr, 0);
    } else if (*skb).ip_summed == CHECKSUM_PARTIAL {
        (*uh).check = 0;
        (*uh).check = udp_v6_check(len, saddr, daddr, lco_csum(skb));
        if (*uh).check == 0 {
            (*uh).check = CSUM_MANGLED_0;
        }
    } else {
        (*skb).ip_summed = CHECKSUM_PARTIAL;
        (*skb).csum_start = skb_transport_header(skb).offset_from((*skb).head) as _;
        (*skb).csum_offset = core::mem::offset_of!(udphdr, check) as _;
        (*uh).check = !udp_v6_check(len, saddr, daddr, 0);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
