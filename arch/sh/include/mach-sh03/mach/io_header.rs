/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm-sh/sh03/io.h
 *
 * Copyright 2004 Interface Co.,Ltd. Saito.K
 *
 * IO functions for an Interface CTP/PCI-SH03
 */

// Dependency intent from <linux/time.h> is preserved; this header declares no
// local items that require it.

pub const IRL0_IRQ: i32 = 2;
pub const IRL0_PRIORITY: i32 = 13;
pub const IRL1_IRQ: i32 = 5;
pub const IRL1_PRIORITY: i32 = 10;
pub const IRL2_IRQ: i32 = 8;
pub const IRL2_PRIORITY: i32 = 7;
pub const IRL3_IRQ: i32 = 11;
pub const IRL3_PRIORITY: i32 = 4;

unsafe extern "C" {
    pub fn heartbeat_sh03();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
