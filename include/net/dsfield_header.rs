/* SPDX-License-Identifier: GPL-2.0 */
/* include/net/dsfield.h - Manipulation of the Differentiated Services field */

/* Written 1998-2000 by Werner Almesberger, EPFL ICA */

/* Dependencies corresponding to linux/types.h, linux/ip.h, linux/ipv6.h,
 * and asm/byteorder.h are supplied by other translation units. */

#[inline]
pub unsafe fn ipv4_get_dsfield(iph: *const iphdr) -> __u8 {
    (*iph).tos
}

#[inline]
pub unsafe fn ipv6_get_dsfield(ipv6h: *const ipv6hdr) -> __u8 {
    ntohs(*(ipv6h as *const __be16)) >> 4
}

#[inline]
pub unsafe fn ipv4_change_dsfield(iph: *mut iphdr, mask: __u8, value: __u8) {
    let mut check: __u32 = ntohs((*iph).check as __be16);
    let dsfield: __u8;

    dsfield = ((*iph).tos & mask) | value;
    check = check.wrapping_add((*iph).tos as __u32);
    if (check.wrapping_add(1) >> 16) != 0 {
        check = check.wrapping_add(1) & 0xffff;
    }
    check = check.wrapping_sub(dsfield as __u32);
    check = check.wrapping_add(check >> 16); /* adjust carry */
    (*iph).check = htons(check as __be16) as __sum16;
    (*iph).tos = dsfield;
}

#[inline]
pub unsafe fn ipv6_change_dsfield(ipv6h: *mut ipv6hdr, mask: __u8, value: __u8) {
    let p = ipv6h as *mut __be16;

    *p = ((*p as u16) & htons((((mask as u16) << 4) | 0xf00f)))
        | htons((value as u16) << 4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
