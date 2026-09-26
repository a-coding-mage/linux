// SPDX-License-Identifier: GPL-2.0-only
//! Command-line ownership for the staged, unselected early-boot implementation.
//!
//! State belongs solely to `main_globals`. Parsing/registration and the rest of
//! the boot sequence remain incomplete; these routines alone cannot boot Linux.

use super::{bindings, main_globals::*};
use kernel::ffi::{c_char, c_int};

/// Report whether bootconfig supplied kernel options or init arguments.
///
/// # Safety
///
/// The caller must serialize access with early bootconfig state changes.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn cmdline_has_extra_options() -> bool {
    // SAFETY: early boot serializes these state reads with bootconfig setup.
    unsafe { !extra_command_line.is_null() || !extra_init_args.is_null() }
}

/// Store an explicit init executable and discard preceding init arguments.
///
/// # Safety
///
/// The parser supplies a persistent writable command-line string during the
/// single-threaded command-line setup phase.
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn init_setup(value: *mut c_char) -> c_int {
    // SAFETY: the caller serializes the parser and the arrays use the same
    // canonical INIT_ENV_ARG_LIMIT as the original C declaration.
    unsafe {
        execute_command = value;
        for index in 1..bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize {
            argv_init[index] = core::ptr::null();
        }
    }
    1
}

/// Store an explicit early-userspace executable and discard preceding arguments.
///
/// # Safety
///
/// The caller supplies the same string lifetime and serialized boot phase as
/// `init_setup`.
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn rdinit_setup(value: *mut c_char) -> c_int {
    // SAFETY: the caller provides the original parser's state/string lifetime.
    unsafe {
        ramdisk_execute_command = value;
        ramdisk_execute_command_set = true;
        for index in 1..bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize {
            argv_init[index] = core::ptr::null();
        }
    }
    1
}

super::main_setup::setup_param!("init=", init_setup_record, Some(init_setup), 0);
super::main_setup::setup_param!("rdinit=", rdinit_setup_record, Some(rdinit_setup), 0);

/// Preserve the untouched command line and the separately mutable parser copy.
///
/// # Safety
///
/// Called once after architecture and bootconfig setup, before concurrent users
/// exist. Every input pointer names its original terminated command-line buffer;
/// a nonzero `initargs_offs` is the offset supplied by bootconfig's real parser.
#[link_section = ".init.text"]
pub(super) unsafe fn setup_command_line(command_line: *mut c_char) {
    // SAFETY: all pointer operations follow the original boot-owned strings and
    // checked memblock allocations; the caller guarantees their boot lifetime.
    unsafe {
        let mut extra_length = 0usize;
        let mut init_length = 0usize;
        if !extra_command_line.is_null() {
            extra_length = bindings::strlen(extra_command_line);
        }
        if !extra_init_args.is_null() {
            extra_init_args = bindings::strim(extra_init_args);
            init_length = bindings::strlen(extra_init_args).wrapping_add(4);
        }
        let boot = core::ptr::addr_of!(boot_command_line).cast::<c_char>();
        let length = extra_length.wrapping_add(bindings::strlen(boot))
            .wrapping_add(init_length).wrapping_add(1);
        let alignment = bindings::RUST_INIT_MAIN_SMP_CACHE_BYTES as bindings::phys_addr_t;
        let function = c"setup_command_line".as_ptr().cast::<c_char>();
        saved_command_line = bindings::__memblock_alloc_or_panic(
            length as bindings::phys_addr_t, alignment, function,
        ).cast();
        let length = extra_length.wrapping_add(bindings::strlen(command_line)).wrapping_add(1);
        static_command_line = bindings::__memblock_alloc_or_panic(
            length as bindings::phys_addr_t, alignment, function,
        ).cast();

        if extra_length != 0 {
            bindings::strcpy(saved_command_line, extra_command_line);
            bindings::strcpy(static_command_line, extra_command_line);
        }
        bindings::strcpy(saved_command_line.wrapping_add(extra_length), boot);
        bindings::strcpy(static_command_line.wrapping_add(extra_length), command_line);

        if init_length != 0 {
            if initargs_offs != 0 {
                let mut length = extra_length.wrapping_add(initargs_offs);
                bindings::strcpy(saved_command_line.wrapping_add(length), extra_init_args);
                length = length.wrapping_add(init_length.wrapping_sub(4));
                bindings::strcpy(saved_command_line.wrapping_add(length),
                                 boot.wrapping_add(initargs_offs.wrapping_sub(1)));
            } else {
                let mut length = bindings::strlen(saved_command_line);
                bindings::strcpy(saved_command_line.wrapping_add(length), c" -- ".as_ptr().cast());
                length = length.wrapping_add(4);
                bindings::strcpy(saved_command_line.wrapping_add(length), extra_init_args);
            }
        }
        saved_command_line_len = bindings::strlen(saved_command_line) as kernel::ffi::c_uint;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
