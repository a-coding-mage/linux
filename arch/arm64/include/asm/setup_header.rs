// SPDX-License-Identifier: GPL-2.0

// Translated from the C header __ARM64_ASM_SETUP_H.

// Dependency: <linux/string.h>
// Dependency: <uapi/asm/setup.h>

/*
 * These two variables are used in the head.S file.
 */
extern "C" {
    // __initdata
    static mut __fdt_pointer: phys_addr_t;
    // __cacheline_aligned
    static mut boot_args: [u64; 4];
}

/// Direct translation of the C `static inline` helper.
pub unsafe fn arch_parse_debug_rodata(arg: *mut core::ffi::c_char) -> bool {
    extern "C" {
        static mut rodata_enabled: bool;
        static mut rodata_full: bool;
        fn strcmp(lhs: *const core::ffi::c_char, rhs: *const core::ffi::c_char) -> core::ffi::c_int;
    }

    if arg.is_null() {
        return false;
    }

    if strcmp(arg as *const core::ffi::c_char, b"on\0".as_ptr() as *const core::ffi::c_char) == 0 {
        rodata_enabled = true;
        rodata_full = true;
        return true;
    }

    if strcmp(arg as *const core::ffi::c_char, b"off\0".as_ptr() as *const core::ffi::c_char) == 0 {
        rodata_enabled = false;
        rodata_full = false;
        return true;
    }

    if strcmp(arg as *const core::ffi::c_char, b"noalias\0".as_ptr() as *const core::ffi::c_char) == 0 {
        rodata_enabled = true;
        rodata_full = false;
        return true;
    }

    false
}

// C compatibility macro: #define arch_parse_debug_rodata arch_parse_debug_rodata

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
