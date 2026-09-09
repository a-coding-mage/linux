/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 */

/*
 * C header guard: _XTENSA_MMU_H
 *
 * When CONFIG_MMU is not enabled, the original header includes
 * <asm-generic/mmu.h>; that external dependency is intentionally not
 * reimplemented here.
 */

#[cfg(feature = "CONFIG_MMU")]
#[repr(C)]
pub struct mm_context_t {
    pub asid: [usize; NR_CPUS],
    pub cpu: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
