/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  IPv6 RPL-SR implementation
 *
 *  Author:
 *  (C) 2020 Alexander Aring <alex.aring@gmail.com>
 */

/* C dependencies: <asm/byteorder.h>, <linux/types.h>, and <linux/in6.h>. */

/*
 * RPL SR Header
 *
 * The C bitfields are represented by their containing 32-bit storage word.
 * Bit positions are interpreted according to the target byte-order cfg.
 */
#[repr(C, packed)]
pub struct ipv6_rpl_sr_hdr {
    pub nexthdr: u8,
    pub hdrlen: u8,
    pub type_: u8,
    pub segments_left: u8,
    #[cfg(any(target_endian = "little", target_endian = "big"))]
    pub bitfield_storage: u32,
    pub segments: ipv6_rpl_sr_hdr_segments,
}

#[repr(C)]
pub union ipv6_rpl_sr_hdr_segments {
    pub addr: [in6_addr; 0],
    pub data: [u8; 0],
}

/* The external Linux IPv6 address type is supplied by the including crate. */
#[allow(non_camel_case_types)]
pub type in6_addr = crate::in6_addr;

/* __LITTLE_ENDIAN_BITFIELD layout: cmpre:4, cmpri:4, reserved:4,
 * pad:4, reserved1:16.
 * __BIG_ENDIAN_BITFIELD layout: cmpri:4, cmpre:4, pad:4, reserved:20.
 */

#[macro_export]
macro_rules! rpl_segaddr {
    ($hdr:expr) => {
        $hdr.segments.addr
    };
}

#[macro_export]
macro_rules! rpl_segdata {
    ($hdr:expr) => {
        $hdr.segments.data
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
