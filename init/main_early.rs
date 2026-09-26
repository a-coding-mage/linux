// SPDX-License-Identifier: GPL-2.0-only
//! Early setup callbacks and the boot command line's parse-once state.

use super::{bindings, main_globals, main_printk::main_printk};
use kernel::ffi::{c_char, c_int, c_void};

/// Visit every matching early setup entry, preserving duplicate callbacks.
///
/// # Safety
///
/// The boot parser owns the writable terminated strings (value may be null).
/// The linker registry and its callback strings remain live in init memory.
/// Each early entry has a nonnull callback, and boot parsing is serialized.
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn do_early_param(
    parameter: *mut c_char,
    value: *mut c_char,
    _unused: *const c_char,
    _argument: *mut c_void,
) -> c_int {
    let mut entry = core::ptr::addr_of!(bindings::__setup_start).cast::<bindings::obs_kernel_param>();
    let end = core::ptr::addr_of!(bindings::__setup_end).cast::<bindings::obs_kernel_param>();
    // SAFETY: the linker and caller supply the live registry and parser strings.
    // No Rust reference to boot state survives a callback.
    unsafe {
        while entry < end {
            if (*entry).early != 0 && bindings::parameq(parameter, (*entry).str_) {
                let callback = (*entry).setup_func.unwrap_unchecked();
                if callback(value) != 0 {
                    main_printk!("do_early_param", b"\x014Malformed early option '%s'\n\0", parameter);
                }
            }
            entry = entry.wrapping_add(1);
        }
    }
    0
}

/// Parse the supplied early options using the actual kernel argument parser.
///
/// # Safety
///
/// `command_line` is writable terminated storage kept alive for all callbacks.
/// The caller serializes early boot state and provides the live setup registry.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn parse_early_options(command_line: *mut c_char) {
    // SAFETY: the original parser mutates caller-owned storage and calls the
    // exact C callback type. There are no ordinary kernel parameters here.
    unsafe {
        bindings::parse_args(c"early options".as_ptr().cast(), command_line,
                             core::ptr::null(), 0, 0, 0, core::ptr::null_mut(),
                             Some(do_early_param));
    }
}

/// Parse a private copy of the boot command line once, for arch and main callers.
///
/// # Safety
///
/// The boot command line is terminated, the setup registry remains live, and
/// calls are serialized. Callbacks obey the original early parsing contract.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn parse_early_param() {
    // SAFETY: these are the sole owner's init-memory buffers and once flag.
    // Set the flag after callbacks finish, in the same order as the C owner.
    unsafe {
        if main_globals::parse_early_param_done != 0 {
            return;
        }
        let copy = core::ptr::addr_of_mut!(main_globals::parse_early_param_tmp_cmdline).cast();
        bindings::sized_strscpy(copy,
            core::ptr::addr_of!(main_globals::boot_command_line).cast(),
            bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize);
        parse_early_options(copy);
        main_globals::parse_early_param_done = 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
