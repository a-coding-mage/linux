/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copied from arch/arm64/include/asm/hwcap.h
 *
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2017 SiFive
 */

/*
 * Linux saves the floating-point registers according to the ISA Linux is
 * executing on, as opposed to the ISA the user program is compiled for.  This
 * is necessary for a handful of esoteric use cases: for example, userspace
 * threading libraries must be able to examine the actual machine state in
 * order to fully reconstruct the state of a thread.
 */
pub const COMPAT_HWCAP_ISA_I: i32 = 1 << ('I' as i32 - 'A' as i32);
pub const COMPAT_HWCAP_ISA_M: i32 = 1 << ('M' as i32 - 'A' as i32);
pub const COMPAT_HWCAP_ISA_A: i32 = 1 << ('A' as i32 - 'A' as i32);
pub const COMPAT_HWCAP_ISA_F: i32 = 1 << ('F' as i32 - 'A' as i32);
pub const COMPAT_HWCAP_ISA_D: i32 = 1 << ('D' as i32 - 'A' as i32);
pub const COMPAT_HWCAP_ISA_C: i32 = 1 << ('C' as i32 - 'A' as i32);
pub const COMPAT_HWCAP_ISA_V: i32 = 1 << ('V' as i32 - 'A' as i32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
