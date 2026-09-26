// SPDX-License-Identifier: GPL-2.0-only
//! Original read-only memory option and post-init protection ordering.

use super::main_printk::main_printk;
#[cfg(any(CONFIG_STRICT_KERNEL_RWX, CONFIG_STRICT_MODULE_RWX))]
use super::{bindings, main_globals};
#[cfg(any(CONFIG_STRICT_KERNEL_RWX, CONFIG_STRICT_MODULE_RWX))]
use kernel::ffi::{c_char, c_int};

/// Apply the actual ARM64 setup.h policy, including its `noalias` option.
#[cfg(all(CONFIG_ARM64, any(CONFIG_STRICT_KERNEL_RWX, CONFIG_STRICT_MODULE_RWX)))]
unsafe fn arch_parse_debug_rodata(value: *mut c_char) -> bool {
    // SAFETY: the callback has exclusive boot access to both original globals
    // and supplies a terminated argument when it is nonnull.
    unsafe {
        if value.is_null() {
            return false;
        }
        if bindings::strcmp(value, c"on".as_ptr().cast()) == 0 {
            main_globals::rodata_enabled = true;
            bindings::rodata_full = true;
            return true;
        }
        if bindings::strcmp(value, c"off".as_ptr().cast()) == 0 {
            main_globals::rodata_enabled = false;
            bindings::rodata_full = false;
            return true;
        }
        if bindings::strcmp(value, c"noalias".as_ptr().cast()) == 0 {
            main_globals::rodata_enabled = true;
            bindings::rodata_full = false;
            return true;
        }
        false
    }
}

/// Preserve the original early callback, including diagnostics for null values.
///
/// # Safety
///
/// Boot callback execution is serialized and a nonnull value is a live C string.
#[cfg(any(CONFIG_STRICT_KERNEL_RWX, CONFIG_STRICT_MODULE_RWX))]
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn set_debug_rodata(value: *mut c_char) -> c_int {
    // SAFETY: the boot parameter caller owns the globals and argument storage.
    unsafe {
        #[cfg(CONFIG_ARM64)]
        if arch_parse_debug_rodata(value) {
            return 0;
        }
        if !value.is_null() && bindings::strcmp(value, c"on".as_ptr().cast()) == 0 {
            main_globals::rodata_enabled = true;
        } else if !value.is_null() && bindings::strcmp(value, c"off".as_ptr().cast()) == 0 {
            main_globals::rodata_enabled = false;
        } else {
            main_printk!("set_debug_rodata", b"\x014Invalid option string for rodata: '%s'\n\0", value);
        }
    }
    0
}

#[cfg(any(CONFIG_STRICT_KERNEL_RWX, CONFIG_STRICT_MODULE_RWX))]
super::main_setup::setup_param!("rodata", rodata_record, Some(set_debug_rodata), 1);

/// Finish the original module/jump-label/protection checks in their C order.
///
/// # Safety
///
/// The caller must reach the original post-init phase after freeing init memory
/// and must serialize access to boot protection state. This function does not
/// establish that lifecycle or replace the architecture protection subsystem.
#[cfg_attr(not(CONFIG_PRINTK), allow(unused_unsafe))]
pub(super) unsafe fn mark_readonly() {
    // SAFETY: the caller preserves the original lifecycle and subsystem state.
    unsafe {
        #[cfg(CONFIG_STRICT_KERNEL_RWX)]
        if main_globals::rodata_enabled {
            #[cfg(CONFIG_MODULES)]
            bindings::flush_module_init_free_work();
            #[cfg(CONFIG_JUMP_LABEL)]
            bindings::jump_label_init_ro();
            bindings::mark_rodata_ro();
            #[cfg(CONFIG_DEBUG_WX)]
            { bindings::ptdump_check_wx(); }
            #[cfg(CONFIG_DEBUG_RODATA_TEST)]
            bindings::rodata_test();
        } else {
            main_printk!("mark_readonly", b"\x016Kernel memory protection disabled.\n\0");
        }
        #[cfg(all(not(CONFIG_STRICT_KERNEL_RWX), CONFIG_ARCH_HAS_STRICT_KERNEL_RWX))]
        main_printk!("mark_readonly", b"\x014Kernel memory protection not selected by kernel config.\n\0");
        #[cfg(not(any(CONFIG_STRICT_KERNEL_RWX, CONFIG_ARCH_HAS_STRICT_KERNEL_RWX)))]
        main_printk!("mark_readonly", b"\x014This architecture does not have kernel memory protection.\n\0");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
