/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by <asm-generic/sections.h> and <asm/extable.h>.

unsafe extern "C" {
    pub static mut __relocate_kernel_start: [core::ffi::c_char; 0];
    pub static mut __relocate_kernel_end: [core::ffi::c_char; 0];
    pub static mut __brk_base: [core::ffi::c_char; 0];
    pub static mut __brk_limit: [core::ffi::c_char; 0];
    pub static mut __end_rodata_aligned: [core::ffi::c_char; 0];

    // Conditional on CONFIG_X86_64 in the source build configuration.
    #[cfg(feature = "CONFIG_X86_64")]
    pub static mut __end_rodata_hpage_align: [core::ffi::c_char; 0];

    pub static mut __end_of_kernel_reserve: [core::ffi::c_char; 0];

    pub static mut _brk_start: core::ffi::c_ulong;
    pub static mut _brk_end: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
