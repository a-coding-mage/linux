/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependency supplied by <vdso/datapage.h> in the original header:
 * `struct vdso_clock`, including its `mask` field.
 */

pub const VDSO_PRECISION_MASK: u64 = !(0xFF00u64 << 48);

/*
 * Update the vDSO data page to keep in sync with kernel timekeeping.
 */
#[inline(always)]
unsafe fn __arch_update_vdso_clock(vc: *mut vdso_clock) {
    (*vc).mask = VDSO_PRECISION_MASK;
}

/* The asm-generic header needs to be included after the definitions above. */
/* Dependency supplied by <asm-generic/vdso/vsyscall.h>. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
