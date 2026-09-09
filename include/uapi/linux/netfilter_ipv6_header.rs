/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* IPv6-specific defines for netfilter.
 * (C)1998 Rusty Russell -- This code is GPL.
 * (C)1999 David Jeffery
 *   this header was blatantly ripped from netfilter_ipv4.h
 *   it's amazing what adding a bunch of 6s can do =8^)
 */

// Dependencies supplied by the corresponding Linux headers are intentionally
// not reproduced here.

/* only for userspace compatibility */

/* IP6 Hooks */
/* After promisc drops, checksum checks. */
pub const NF_IP6_PRE_ROUTING: i32 = 0;
/* If the packet is destined for this box. */
pub const NF_IP6_LOCAL_IN: i32 = 1;
/* If the packet is destined for another interface. */
pub const NF_IP6_FORWARD: i32 = 2;
/* Packets coming from a local process. */
pub const NF_IP6_LOCAL_OUT: i32 = 3;
/* Packets about to hit the wire. */
pub const NF_IP6_POST_ROUTING: i32 = 4;
pub const NF_IP6_NUMHOOKS: i32 = 5;

#[repr(i32)]
pub enum NfIp6HookPriorities {
    NF_IP6_PRI_FIRST = i32::MIN,
    NF_IP6_PRI_RAW_BEFORE_DEFRAG = -450,
    NF_IP6_PRI_CONNTRACK_DEFRAG = -400,
    NF_IP6_PRI_RAW = -300,
    NF_IP6_PRI_SELINUX_FIRST = -225,
    NF_IP6_PRI_CONNTRACK = -200,
    NF_IP6_PRI_MANGLE = -150,
    NF_IP6_PRI_NAT_DST = -100,
    NF_IP6_PRI_FILTER = 0,
    NF_IP6_PRI_SECURITY = 50,
    NF_IP6_PRI_NAT_SRC = 100,
    NF_IP6_PRI_SELINUX_LAST = 225,
    NF_IP6_PRI_CONNTRACK_HELPER = 300,
    NF_IP6_PRI_LAST = i32::MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
