/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	Linux INET6 implementation
 *
 *	Authors:
 *	Pedro Roque		<roque@di.fc.ul.pt>
 */

// Dependency corresponding to: #include <uapi/linux/ipv6_route.h>

#[macro_export]
macro_rules! IPV6_EXTRACT_PREF {
    ($flag:expr) => {
        (($flag & RTF_PREF_MASK) >> 27)
    };
}

#[macro_export]
macro_rules! IPV6_DECODE_PREF {
    ($pref:expr) => {
        ($pref ^ 2) /* 1:low,2:med,3:high */
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
