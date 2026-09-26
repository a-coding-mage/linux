// SPDX-License-Identifier: GPL-2.0-only
//! Original boot-option routing and mutation of the init argument arrays.
//!
//! All routines run during serialized early boot. Strings remain owned by the
//! command-line buffers; the argument arrays borrow their storage and allocate
//! nothing. The setup registry is the actual linker-provided C-layout array.

use super::{bindings, main_globals::*, main_printk::main_printk};
use kernel::ffi::{c_char, c_int, c_uint, c_void};

/// Restore the separator removed by `next_arg`, including quoted values.
///
/// # Safety
///
/// The parser supplies its writable terminated parameter/value buffer, with
/// `value` null or following the parameter's terminator. An invalid nonnull
/// parser geometry triggers the original BUG boundary.
#[link_section = ".init.text"]
pub(super) unsafe fn repair_env_string(parameter: *mut c_char, value: *mut c_char) {
    // SAFETY: the parser owns this writable buffer until boot parsing finishes.
    unsafe {
        if !value.is_null() {
            let length = bindings::strlen(parameter);
            if value == parameter.wrapping_add(length.wrapping_add(1)) {
                value.wrapping_sub(1).write(b'=');
            } else if value == parameter.wrapping_add(length.wrapping_add(2)) {
                value.wrapping_sub(2).write(b'=');
                bindings::memmove(value.wrapping_sub(1).cast(), value.cast(),
                                  bindings::strlen(value).wrapping_add(1));
            } else {
                kernel::bindings::BUG();
            }
        }
    }
}

/// Match the original setup registry, including early and obsolete options.
///
/// # Safety
///
/// `line` is a writable terminated boot-option string. The nonempty setup
/// registry and every referenced string/callback remain live in init memory.
/// The caller serializes all callback effects with the rest of boot parsing.
#[link_section = ".init.text"]
pub(super) unsafe fn obsolete_checksetup(line: *mut c_char) -> bool {
    let mut entry = core::ptr::addr_of!(bindings::__setup_start).cast::<bindings::obs_kernel_param>();
    let end = core::ptr::addr_of!(bindings::__setup_end).cast::<bindings::obs_kernel_param>();
    let mut had_early = false;
    // SAFETY: the linker and caller provide the original live registry/string
    // contract. Only raw pointers cross callbacks that can mutate boot state.
    unsafe {
        loop {
            let name = (*entry).str_;
            let length = bindings::strlen(name) as c_int;
            if bindings::parameqn(line, name, length as usize) {
                if (*entry).early != 0 {
                    let next = line.wrapping_offset(length as isize).read();
                    if next == 0 || next == b'=' {
                        had_early = true;
                    }
                } else if let Some(callback) = (*entry).setup_func {
                    if callback(line.wrapping_offset(length as isize)) != 0 {
                        return true;
                    }
                } else {
                    main_printk!("obsolete_checksetup", b"\x014Parameter %s is obsolete, ignored\n\0", name);
                    return true;
                }
            }
            entry = entry.wrapping_add(1);
            if entry >= end {
                break;
            }
        }
    }
    had_early
}

/// Append an argument after the original `--` separator.
///
/// # Safety
///
/// The original parser owns the writable parameter/value buffer and serializes
/// access to the terminated init argument array and deferred-panic state.
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn set_init_arg(
    parameter: *mut c_char,
    value: *mut c_char,
    _unused: *const c_char,
    _argument: *mut c_void,
) -> c_int {
    // SAFETY: the caller upholds the parser and global-array invariants.
    unsafe {
        if !panic_later.is_null() {
            return 0;
        }
        repair_env_string(parameter, value);
        let array = core::ptr::addr_of_mut!(argv_init).cast::<*const c_char>();
        let mut index: c_uint = 0;
        while !array.wrapping_add(index as usize).read().is_null() {
            if index == bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as c_uint {
                panic_later = c"init".as_ptr().cast();
                panic_param = parameter;
                return 0;
            }
            index = index.wrapping_add(1);
        }
        array.wrapping_add(index as usize).write(parameter);
    }
    0
}

/// Route an unknown boot option to its original setup, environment or argv path.
///
/// # Safety
///
/// The original parser supplies a writable parameter/value buffer with boot
/// lifetime. The caller serializes setup callbacks, the terminated init arrays
/// and deferred-panic state. Their extra sentinel slot remains available for
/// the final overflowing option, as in the original C declarations.
#[link_section = ".init.text"]
pub(super) unsafe extern "C" fn unknown_bootoption(
    parameter: *mut c_char,
    value: *mut c_char,
    _unused: *const c_char,
    _argument: *mut c_void,
) -> c_int {
    // SAFETY: raw pointers preserve the original parser's owned storage and do
    // not hold Rust references across a callback that can update the arrays.
    unsafe {
        let length = bindings::strlen(parameter);
        #[cfg(CONFIG_SYSCTL)]
        if bindings::sysctl_is_alias(parameter) {
            return 0;
        }
        repair_env_string(parameter, value);
        // Original strstarts() is an inline strlen/strncmp operation.
        for prefix in [c"BOOT_IMAGE=", c"kexec"] {
            let prefix = prefix.as_ptr().cast();
            if bindings::strncmp(parameter, prefix, bindings::strlen(prefix)) == 0 {
                return 0;
            }
        }
        if obsolete_checksetup(parameter)
            || !bindings::strnchr(parameter, length, b'.' as c_int).is_null()
            || !panic_later.is_null()
        {
            return 0;
        }
        let mut index: c_uint = 0;
        if !value.is_null() {
            let array = core::ptr::addr_of_mut!(envp_init).cast::<*const c_char>();
            while !array.wrapping_add(index as usize).read().is_null() {
                if index == bindings::RUST_INIT_MAIN_MAX_INIT_ENVS as c_uint {
                    panic_later = c"env".as_ptr().cast();
                    panic_param = parameter;
                }
                if bindings::strncmp(parameter, array.wrapping_add(index as usize).read(),
                                     length.wrapping_add(1)) == 0 {
                    break;
                }
                index = index.wrapping_add(1);
            }
            array.wrapping_add(index as usize).write(parameter);
        } else {
            let array = core::ptr::addr_of_mut!(argv_init).cast::<*const c_char>();
            while !array.wrapping_add(index as usize).read().is_null() {
                if index == bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as c_uint {
                    panic_later = c"init".as_ptr().cast();
                    panic_param = parameter;
                }
                index = index.wrapping_add(1);
            }
            array.wrapping_add(index as usize).write(parameter);
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
