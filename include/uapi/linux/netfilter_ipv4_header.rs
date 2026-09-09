/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* IPv4-specific defines for netfilter.
 * (C)1998 Rusty Russell -- This code is GPL.
 */

/* Dependency intent: symbols from <linux/netfilter.h> and
 * <linux/typelimits.h> are supplied by other translated dependencies.
 */

/* only for userspace compatibility */
/* The following constants are present when __KERNEL__ is not defined. */

/* IP Hooks */
/* After promisc drops, checksum checks. */
pub const NF_IP_PRE_ROUTING: i32 = 0;
/* If the packet is destined for this box. */
pub const NF_IP_LOCAL_IN: i32 = 1;
/* If the packet is destined for another interface. */
pub const NF_IP_FORWARD: i32 = 2;
/* Packets coming from a local process. */
pub const NF_IP_LOCAL_OUT: i32 = 3;
/* Packets about to hit the wire. */
pub const NF_IP_POST_ROUTING: i32 = 4;
pub const NF_IP_NUMHOOKS: i32 = 5;

#[repr(i32)]
pub enum nf_ip_hook_priorities {
    NF_IP_PRI_FIRST = __KERNEL_INT_MIN,
    NF_IP_PRI_RAW_BEFORE_DEFRAG = -450,
    NF_IP_PRI_CONNTRACK_DEFRAG = -400,
    NF_IP_PRI_RAW = -300,
    NF_IP_PRI_SELINUX_FIRST = -225,
    NF_IP_PRI_CONNTRACK = -200,
    NF_IP_PRI_MANGLE = -150,
    NF_IP_PRI_NAT_DST = -100,
    NF_IP_PRI_FILTER = 0,
    NF_IP_PRI_SECURITY = 50,
    NF_IP_PRI_NAT_SRC = 100,
    NF_IP_PRI_SELINUX_LAST = 225,
    NF_IP_PRI_CONNTRACK_HELPER = 300,
    NF_IP_PRI_CONNTRACK_CONFIRM = __KERNEL_INT_MAX,
    NF_IP_PRI_LAST = __KERNEL_INT_MAX,
}

/* Arguments for setsockopt SOL_IP: */
/* 2.0 firewalling went from 64 through 71 (and +256, +512, etc). */
/* 2.2 firewalling (+ masq) went from 64 through 76 */
/* 2.4 firewalling went 64 through 67. */
pub const SO_ORIGINAL_DST: i32 = 80;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
