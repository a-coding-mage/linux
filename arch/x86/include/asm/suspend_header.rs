/* SPDX-License-Identifier: GPL-2.0 */

// #ifdef CONFIG_X86_32
// # include <asm/suspend_32.h>
// #else
// # include <asm/suspend_64.h>
// #endif

extern "C" {
    pub static mut restore_jump_address: core::ffi::c_ulong;
    pub static mut jump_address_phys: core::ffi::c_ulong;
    pub static mut restore_cr3: core::ffi::c_ulong;
    pub static mut temp_pgt: core::ffi::c_ulong;
    pub static mut relocated_restore_code: core::ffi::c_ulong;

    pub fn relocate_restore_code() -> core::ffi::c_int;

    // Defined in hibernate_asm_32/64.S
    pub fn restore_image() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
