/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2026 Isovalent */

// Required external dependency types corresponding to <linux/in.h> and
// <linux/in6.h> are supplied by the surrounding translation unit.

/**
 * struct bpf_ksock_create_opts - BPF kernel socket creation parameters
 * @family: Address family: AF_INET or AF_INET6.
 * @type: Socket type: only SOCK_DGRAM supported for now.
 * @protocol: Protocol number (e.g. IPPROTO_UDP), or 0 for the default protocol
 *            of the given type.
 * @reserved: Must be zero. Reserved for future use.
 */
#[repr(C)]
pub struct bpf_ksock_create_opts {
    pub family: u8,
    pub r#type: u8,
    pub protocol: u8,
    pub reserved: u8,
}

/**
 * union bpf_ksock_addr - IPv4 or IPv6 socket address
 * @sin: IPv4 socket address.
 * @sin6: IPv6 socket address.
 */
#[repr(C)]
pub union bpf_ksock_addr {
    pub sin: sockaddr_in,
    pub sin6: sockaddr_in6,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
