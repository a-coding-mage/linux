/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */

/* ARP-specific defines for netfilter.
 * (C)2002 Rusty Russell IBM -- This code is GPL.
 */

// Dependency: <linux/netfilter.h>

/* There is no PF_ARP. */
pub const NF_ARP: i32 = 0;

/* ARP Hooks */
pub const NF_ARP_IN: i32 = 0;
pub const NF_ARP_OUT: i32 = 1;
pub const NF_ARP_FORWARD: i32 = 2;

// C build condition: NF_ARP_NUMHOOKS is defined only when __KERNEL__ is not defined.
#[cfg(not(feature = "__KERNEL__"))]
pub const NF_ARP_NUMHOOKS: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
