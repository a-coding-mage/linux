/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header guard and include directives have no direct Rust equivalent.
 * Names supplied by the included headers remain external dependencies.
 */

use core::ffi::c_ulong;

/* C: register unsigned long __local_per_cpu_offset asm("g5");
 * The register binding is target/toolchain-specific and is preserved by this
 * externally visible mutable symbol declaration.
 */
#[cfg(not(feature = "BUILD_VDSO"))]
extern "C" {
    pub static mut __local_per_cpu_offset: c_ulong;
}

/* CONFIG_SMP */
#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn __per_cpu_offset(__cpu: usize) -> c_ulong {
    /* Corresponds to: trap_block[__cpu].__per_cpu_base */
    crate::trap_block[__cpu].__per_cpu_base
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn per_cpu_offset(x: usize) -> c_ulong {
    __per_cpu_offset(x)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn __my_cpu_offset() -> c_ulong {
    __local_per_cpu_offset
}

/* !SMP: no declarations are present in the source header. */

/* The asm-generic/percpu.h declarations are provided by another dependency. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
