/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Bridge-specific defines for netfilter.
 *
 * C header dependencies are supplied by other translated headers.
 */

/* Bridge Hooks */
/* After promisc drops, checksum checks. */
pub const NF_BR_PRE_ROUTING: i32 = 0;
/* If the packet is destined for this box. */
pub const NF_BR_LOCAL_IN: i32 = 1;
/* If the packet is destined for another interface. */
pub const NF_BR_FORWARD: i32 = 2;
/* Packets coming from a local process. */
pub const NF_BR_LOCAL_OUT: i32 = 3;
/* Packets about to hit the wire. */
pub const NF_BR_POST_ROUTING: i32 = 4;
/* Not really a hook, but used for the ebtables broute table */
pub const NF_BR_BROUTING: i32 = 5;
pub const NF_BR_NUMHOOKS: i32 = 6;

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum nf_br_hook_priorities {
    NF_BR_PRI_FIRST = __KERNEL_INT_MIN,
    NF_BR_PRI_NAT_DST_BRIDGED = -300,
    NF_BR_PRI_FILTER_BRIDGED = -200,
    NF_BR_PRI_BRNF = 0,
    NF_BR_PRI_NAT_DST_OTHER = 100,
    NF_BR_PRI_FILTER_OTHER = 200,
    NF_BR_PRI_NAT_SRC = 300,
    NF_BR_PRI_LAST = __KERNEL_INT_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
