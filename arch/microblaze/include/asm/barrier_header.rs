/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2015 - 2020 Xilinx, Inc. All rights reserved.
 */

// Equivalent of the C header's inclusion of <asm-generic/barrier.h>.

/// MicroBlaze full memory barrier.
#[inline(always)]
pub unsafe fn mb() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    core::arch::asm!("mbar 1", options(nostack));
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
