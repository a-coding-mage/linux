/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

/*
 * The original declarations are excluded when assembling; this Rust
 * translation contains the non-assembler declarations.
 */

/* PAUSE is a good thing to insert into busy-wait loops. */
#[inline(always)]
pub unsafe fn native_pause() {
    core::arch::asm!("pause", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn cpu_relax() {
    native_pause();
}

// `notrace long __vdso_getcpu(unsigned *cpu, unsigned *node, void *unused);`
// The `notrace` attribute is a build-system concern with no direct Rust item
// equivalent here.
unsafe extern "C" {
    pub fn __vdso_getcpu(
        cpu: *mut core::ffi::c_uint,
        node: *mut core::ffi::c_uint,
        unused: *mut core::ffi::c_void,
    ) -> core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
