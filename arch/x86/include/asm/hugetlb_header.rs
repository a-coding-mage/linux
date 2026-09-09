/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <asm/page.h>
// #include <asm-generic/hugetlb.h>

/// Equivalent of the C macro:
/// `#define hugepages_supported() boot_cpu_has(X86_FEATURE_PSE)`
#[inline]
pub(crate) unsafe fn hugepages_supported() -> bool {
    boot_cpu_has(X86_FEATURE_PSE)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
