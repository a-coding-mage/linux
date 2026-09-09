/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Platform definitions for Titan
 */

// Dependency provided by the Linux SH interrupt-controller headers.
// #include <linux/sh_intc.h>

// The C header defines __IO_PREFIX as titan before including the generic
// I/O declarations.  The corresponding declarations are supplied elsewhere.
// #define __IO_PREFIX titan
// #include <asm/io_generic.h>

/* IRQ assignments */
pub const TITAN_IRQ_WAN: i32 = evt2irq(0x240); /* eth0 (WAN) */
pub const TITAN_IRQ_LAN: i32 = evt2irq(0x2a0); /* eth1 (LAN) */
pub const TITAN_IRQ_MPCIA: i32 = evt2irq(0x300); /* mPCI A */
pub const TITAN_IRQ_MPCIB: i32 = evt2irq(0x360); /* mPCI B */
pub const TITAN_IRQ_USB: i32 = evt2irq(0x360); /* USB */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
