/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency: <asm/barrier.h>

/// Corresponds to the C macro `cpu_relax()`.
#[inline]
pub unsafe fn cpu_relax() {
    bcr_serialize();
}

unsafe extern "C" {
    fn bcr_serialize();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
