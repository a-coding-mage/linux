/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	IPV6 GSO/GRO offload support
 *	Linux INET6 implementation
 */

// Declarations translated from the C header.
extern "C" {
    pub fn ipv6_exthdrs_offload_init() -> i32;
    pub fn udpv6_offload_init() -> i32;
    pub fn udpv6_offload_exit() -> i32;
    pub fn tcpv6_offload_init() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
