/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// __LP64__ selects the corresponding architecture-specific syscall definitions:
// asm/unistd_64.h when enabled, otherwise asm/unistd_32.h.

pub const LINUX_GATEWAY_ADDR: usize = 0x100;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
