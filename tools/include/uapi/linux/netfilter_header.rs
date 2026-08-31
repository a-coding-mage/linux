/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * C dependencies removed from executable Rust:
 * <linux/types.h>, <linux/compiler.h>, <linux/in.h>, <linux/in6.h>
 */

use core::mem::ManuallyDrop;

/* Responses from hook functions. */
pub const NF_DROP: u32 = 0;
pub const NF_ACCEPT: u32 = 1;
pub const NF_STOLEN: u32 = 2;
pub const NF_QUEUE: u32 = 3;
pub const NF_REPEAT: u32 = 4;
pub const NF_STOP: u32 = 5; /* Deprecated, for userspace nf_queue compatibility. */
pub const NF_MAX_VERDICT: u32 = NF_STOP;

/* we overload the higher bits for encoding auxiliary data such as the queue
 * number or errno values. Not nice, but better than additional function
 * arguments. */
pub const NF_VERDICT_MASK: u32 = 0x000000ff;

/* extra verdict flags have mask 0x0000ff00 */
pub const NF_VERDICT_FLAG_QUEUE_BYPASS: u32 = 0x00008000;

/* queue number (NF_QUEUE) or errno (NF_DROP) */
pub const NF_VERDICT_QMASK: u32 = 0xffff0000;
pub const NF_VERDICT_QBITS: u32 = 16;

pub const fn NF_QUEUE_NR(x: u32) -> u32 {
    ((x << 16) & NF_VERDICT_QMASK) | NF_QUEUE
}

pub const fn NF_DROP_ERR(x: i32) -> u32 {
    (((-x) as u32) << 16) | NF_DROP
}

/* only for userspace compatibility */
/* Original C condition: #ifndef __KERNEL__ */

/* NF_VERDICT_BITS should be 8 now, but userspace might break if this changes */
pub const NF_VERDICT_BITS: u32 = 16;

pub type nf_inet_hooks = u32;
pub const NF_INET_PRE_ROUTING: nf_inet_hooks = 0;
pub const NF_INET_LOCAL_IN: nf_inet_hooks = 1;
pub const NF_INET_FORWARD: nf_inet_hooks = 2;
pub const NF_INET_LOCAL_OUT: nf_inet_hooks = 3;
pub const NF_INET_POST_ROUTING: nf_inet_hooks = 4;
pub const NF_INET_NUMHOOKS: nf_inet_hooks = 5;
pub const NF_INET_INGRESS: nf_inet_hooks = NF_INET_NUMHOOKS;

pub type nf_dev_hooks = u32;
pub const NF_NETDEV_INGRESS: nf_dev_hooks = 0;
pub const NF_NETDEV_EGRESS: nf_dev_hooks = 1;
pub const NF_NETDEV_NUMHOOKS: nf_dev_hooks = 2;

pub const NFPROTO_UNSPEC: u32 = 0;
pub const NFPROTO_INET: u32 = 1;
pub const NFPROTO_IPV4: u32 = 2;
pub const NFPROTO_ARP: u32 = 3;
pub const NFPROTO_NETDEV: u32 = 5;
pub const NFPROTO_BRIDGE: u32 = 7;
pub const NFPROTO_IPV6: u32 = 10;
/* Original C condition: #ifndef __KERNEL__; no longer supported by kernel */
pub const NFPROTO_DECNET: u32 = 12;
pub const NFPROTO_NUMPROTO: u32 = 13;

#[repr(C)]
pub union nf_inet_addr {
    pub all: [__u32; 4],
    pub ip: __be32,
    pub ip6: [__be32; 4],
    pub in_: ManuallyDrop<in_addr>,
    pub in6: ManuallyDrop<in6_addr>,
}
