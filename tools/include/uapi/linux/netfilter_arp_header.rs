/* SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note */

/* ARP-specific defines for netfilter.
 * (C)2002 Rusty Russell IBM -- This code is GPL.
 */

/* C header dependency: <linux/netfilter.h> */

/* There is no PF_ARP. */
pub const NF_ARP: i32 = 0;

/* ARP Hooks */
pub const NF_ARP_IN: i32 = 0;
pub const NF_ARP_OUT: i32 = 1;
pub const NF_ARP_FORWARD: i32 = 2;

/* Original C condition: #ifndef __KERNEL__ */
pub const NF_ARP_NUMHOOKS: i32 = 3;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
