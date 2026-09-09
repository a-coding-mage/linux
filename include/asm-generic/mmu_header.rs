/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the mmu.h header for nommu implementations.
 * Architectures with an MMU need something more complex.
 */
#[repr(C)]
pub struct mm_context_t {
    pub end_brk: ::core::ffi::c_ulong,

    /* Corresponds to CONFIG_BINFMT_ELF_FDPIC. */
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub exec_fdpic_loadmap: ::core::ffi::c_ulong,
    #[cfg(feature = "CONFIG_BINFMT_ELF_FDPIC")]
    pub interp_fdpic_loadmap: ::core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
