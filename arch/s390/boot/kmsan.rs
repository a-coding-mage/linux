// SPDX-License-Identifier: GPL-2.0
// Dependency: <linux/kmsan-checks.h>

pub unsafe fn kmsan_unpoison_memory(address: *const core::ffi::c_void, size: usize) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
