/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes `asm/iommu_64.h` when compiling for sparc64;
// otherwise it includes `asm/iommu_32.h`. Those target-dependent
// declarations are supplied by the corresponding Rust dependencies.
#[cfg(all(target_arch = "sparc64", target_pointer_width = "64"))]
use crate::iommu_64::*;

#[cfg(not(all(target_arch = "sparc64", target_pointer_width = "64")))]
use crate::iommu_32::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
