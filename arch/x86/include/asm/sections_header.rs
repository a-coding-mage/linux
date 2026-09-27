/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by <asm-generic/sections.h> and <asm/extable.h>.

unsafe extern "C" {
    pub static mut __relocate_kernel_start: [kernel::ffi::c_char; 0];
    pub static mut __relocate_kernel_end: [kernel::ffi::c_char; 0];
    pub static mut __brk_base: [kernel::ffi::c_char; 0];
    pub static mut __brk_limit: [kernel::ffi::c_char; 0];
    pub static mut __end_rodata_aligned: [kernel::ffi::c_char; 0];

    // Conditional on CONFIG_X86_64 in the source build configuration.
    #[cfg(CONFIG_X86_64)]
    pub static mut __end_rodata_hpage_align: [kernel::ffi::c_char; 0];

    pub static mut __end_of_kernel_reserve: [kernel::ffi::c_char; 0];

    pub static mut _brk_start: kernel::ffi::c_ulong;
    pub static mut _brk_end: kernel::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
