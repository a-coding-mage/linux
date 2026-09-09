/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 *  IPv6 RPL-SR implementation
 *
 *  Author:
 *  (C) 2020 Alexander Aring <alex.aring@gmail.com>
 */

pub const RPL_IPTUNNEL_UNSPEC: i32 = 0;
pub const RPL_IPTUNNEL_SRH: i32 = 1;
pub const __RPL_IPTUNNEL_MAX: i32 = 2;
pub const RPL_IPTUNNEL_MAX: i32 = __RPL_IPTUNNEL_MAX - 1;

#[macro_export]
macro_rules! RPL_IPTUNNEL_SRH_SIZE {
    ($srh:expr) => {
        (($srh.hdrlen + 1) << 3)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
