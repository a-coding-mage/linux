/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * INET		An implementation of the TCP/IP protocol suite for the LINUX
 *		operating system.  INET is implemented using the  BSD Socket
 *		interface as the means of communication with the user level.
 *
 *		Definitions for the ICMP protocol.
 *
 * Version:	@(#)icmp.h	1.0.3	04/28/93
 *
 * Author:	Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
 */

// Dependencies supplied by other translation units:
// linux/skbuff.h, uapi/linux/icmp.h, and uapi/linux/errqueue.h.

#[inline]
pub unsafe fn icmp_hdr(skb: *const sk_buff) -> *mut icmphdr {
    skb_transport_header(skb) as *mut icmphdr
}

#[inline]
pub fn icmp_is_err(type_: i32) -> bool {
    match type_ {
        ICMP_DEST_UNREACH |
        ICMP_SOURCE_QUENCH |
        ICMP_REDIRECT |
        ICMP_TIME_EXCEEDED |
        ICMP_PARAMETERPROB => true,
        _ => false,
    }
}

extern "C" {
    pub fn ip_icmp_error_rfc4884(
        skb: *const sk_buff,
        out: *mut sock_ee_data_rfc4884,
        thlen: i32,
        off: i32,
    );
}

/* RFC 4884 */
pub const ICMP_EXT_ORIG_DGRAM_MIN_LEN: u32 = 128;
pub const ICMP_EXT_VERSION_2: u32 = 2;

/* ICMP Extension Object Classes */
pub const ICMP_EXT_OBJ_CLASS_IIO: u32 = 2; /* RFC 5837 */

/* Interface Information Object - RFC 5837 */
pub const ICMP_EXT_CTYPE_IIO_ROLE_IIF: u32 = 0;

#[inline]
pub const fn ICMP_EXT_CTYPE_IIO_ROLE(role: u32) -> u32 {
    role << 6
}

pub const ICMP_EXT_CTYPE_IIO_MTU: u32 = 1u32 << 0;
pub const ICMP_EXT_CTYPE_IIO_NAME: u32 = 1u32 << 1;
pub const ICMP_EXT_CTYPE_IIO_IPADDR: u32 = 1u32 << 2;
pub const ICMP_EXT_CTYPE_IIO_IFINDEX: u32 = 1u32 << 3;

#[repr(C)]
pub struct icmp_ext_iio_name_subobj {
    pub len: u8,
    pub name: [core::ffi::c_char; IFNAMSIZ],
}

/* RFC 5837 - Incoming IP Interface Role */
pub const ICMP_ERR_EXT_IIO_IIF: u32 = 0;
/* Add new constants above. Used by "icmp_errors_extension_mask"
 * sysctl.
 */
pub const ICMP_ERR_EXT_COUNT: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
