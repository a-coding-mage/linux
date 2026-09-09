/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// linux/types.h, linux/compiler.h, linux/in.h, linux/in6.h

/* Responses from hook functions. */
pub const NF_DROP: i32 = 0;
pub const NF_ACCEPT: i32 = 1;
pub const NF_STOLEN: i32 = 2;
pub const NF_QUEUE: i32 = 3;
pub const NF_REPEAT: i32 = 4;
pub const NF_STOP: i32 = 5; /* Deprecated, for userspace nf_queue compatibility. */
pub const NF_MAX_VERDICT: i32 = NF_STOP;

/* we overload the higher bits for encoding auxiliary data such as the queue
 * number or errno values. Not nice, but better than additional function
 * arguments. */
pub const NF_VERDICT_MASK: u32 = 0x000000ff;

/* extra verdict flags have mask 0x0000ff00 */
pub const NF_VERDICT_FLAG_QUEUE_BYPASS: u32 = 0x00008000;

/* queue number (NF_QUEUE) or errno (NF_DROP) */
pub const NF_VERDICT_QMASK: u32 = 0xffff0000;
pub const NF_VERDICT_QBITS: u32 = 16;

pub const fn nf_queue_nr(x: u32) -> u32 {
    (((x << 16) & NF_VERDICT_QMASK) | (NF_QUEUE as u32))
}

pub const fn nf_drop_err(x: i32) -> i32 {
    ((-x) << 16) | NF_DROP
}

/* only for userspace compatibility */
/* NF_VERDICT_BITS should be 8 now, but userspace might break if this changes */
pub const NF_VERDICT_BITS: u32 = 16;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nf_inet_hooks {
    NF_INET_PRE_ROUTING = 0,
    NF_INET_LOCAL_IN,
    NF_INET_FORWARD,
    NF_INET_LOCAL_OUT,
    NF_INET_POST_ROUTING,
    NF_INET_NUMHOOKS,
    NF_INET_INGRESS = NF_INET_NUMHOOKS,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nf_dev_hooks {
    NF_NETDEV_INGRESS = 0,
    NF_NETDEV_EGRESS,
    NF_NETDEV_NUMHOOKS,
}

pub const NFPROTO_UNSPEC: i32 = 0;
pub const NFPROTO_INET: i32 = 1;
pub const NFPROTO_IPV4: i32 = 2;
pub const NFPROTO_ARP: i32 = 3;
pub const NFPROTO_NETDEV: i32 = 5;
pub const NFPROTO_BRIDGE: i32 = 7;
pub const NFPROTO_IPV6: i32 = 10;
/* NFPROTO_DECNET is no longer supported by the kernel; retained for userspace compatibility. */
pub const NFPROTO_DECNET: i32 = 12;
pub const NFPROTO_NUMPROTO: i32 = 13;

#[repr(C)]
pub union nf_inet_addr {
    pub all: [u32; 4],
    pub ip: u32,
    pub ip6: [u32; 4],
    pub r#in: in_addr,
    pub in6: in6_addr,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
