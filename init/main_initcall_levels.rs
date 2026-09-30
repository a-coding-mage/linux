// SPDX-License-Identifier: GPL-2.0-only
//! Original initcall level parsing and boot phase ordering.

use super::{bindings, main_globals, main_initcall, main_initcall_trace};
use core::ptr;
use kernel::ffi::{c_char, c_int, c_void};

#[path = "../include/linux/init_header.rs"]
#[allow(dead_code, unreachable_pub)]
mod init_header;

#[link_section = ".init.text"]
unsafe extern "C" fn ignore_unknown_bootoption(
    _parameter: *mut c_char,
    _value: *mut c_char,
    _unused: *const c_char,
    _argument: *mut c_void,
) -> c_int {
    0
}

/// Parse one level's parameters, then invoke its linker entries in order.
///
/// # Safety
/// `level` indexes the original eight levels. `command_line` is writable and
/// NUL terminated. The parameter and initcall linker ranges remain live; each
/// decoded initcall is nonnull and its boot prerequisites have been satisfied.
#[link_section = ".init.text"]
pub(super) unsafe fn do_initcall_level(level: c_int, command_line: *mut c_char) {
    // SAFETY: the boot caller owns the command line and linker ranges. Forming
    // the byte distance avoids Rust allocation-provenance assumptions for two
    // separately declared linker symbols, while retaining C's element count.
    unsafe {
        let name = ptr::addr_of!(main_globals::initcall_level_names)
            .cast::<*const c_char>()
            .add(level as usize);
        let parameters = ptr::addr_of!(bindings::__start___param).cast::<bindings::kernel_param>();
        let parameter_end =
            ptr::addr_of!(bindings::__stop___param).cast::<bindings::kernel_param>();
        let count = (parameter_end as usize).wrapping_sub(parameters as usize)
            / core::mem::size_of::<bindings::kernel_param>();
        bindings::parse_args(
            name.read(),
            command_line,
            parameters,
            count as _,
            level as _,
            level as _,
            ptr::null_mut(),
            Some(ignore_unknown_bootoption),
        );

        main_initcall_trace::do_trace_initcall_level(name.read());
        let levels =
            ptr::addr_of!(main_globals::initcall_levels).cast::<*mut bindings::initcall_entry_t>();
        let mut entry = levels.add(level as usize).read();
        // C rereads the end bound after each callback; callbacks may update
        // boot state, so preserve those live reads rather than caching it.
        while entry < levels.add(level as usize + 1).read() {
            let function = init_header::initcall_from_entry(entry.cast()).unwrap_unchecked();
            main_initcall::do_one_initcall(function);
            entry = entry.wrapping_add(1);
        }
    }
}

/// Restore the mutable command line before processing each initcall level.
///
/// # Safety
/// The caller owns the initcall phase and all global command-line/linker state.
/// The saved command line fits the original saved length plus its terminator.
#[link_section = ".init.text"]
pub(super) unsafe fn do_initcalls() {
    // SAFETY: the original boot phase supplies valid saved state. Addition
    // occurs in unsigned int before conversion to size_t, as in the C owner.
    unsafe {
        let length = main_globals::saved_command_line_len.wrapping_add(1) as usize;
        let command_line = bindings::rust_init_main_kzalloc_command_line(length);
        if command_line.is_null() {
            bindings::panic(
                b"%s: Failed to allocate %zu bytes\n\0".as_ptr().cast(),
                b"do_initcalls\0".as_ptr().cast::<c_char>(),
                length,
            );
        }
        for level in 0..8 {
            bindings::strcpy(command_line, main_globals::saved_command_line);
            do_initcall_level(level, command_line);
        }
        bindings::kfree(command_line.cast());
    }
}

/// Initialize the original subsystem sequence before normal initcall levels.
///
/// # Safety
/// The CPU, scheduler, allocator and process-management boot prerequisites hold.
#[link_section = ".init.text"]
pub(super) unsafe fn do_basic_setup() {
    // SAFETY: the caller establishes the boot phase prerequisites above.
    unsafe {
        #[cfg(CONFIG_CPUSETS)]
        bindings::cpuset_init_smp();
        bindings::ksysfs_init();
        bindings::driver_init();
        #[cfg(CONFIG_PROC_FS)]
        bindings::init_irq_proc();
        super::main_ctors::do_ctors();
        do_initcalls();
    }
}

/// Invoke the early linker range before secondary CPUs are initialized.
///
/// # Safety
/// The linker range and its nonnull callbacks remain live, and every early
/// callback's prerequisites hold. The caller serializes the initcall phase.
#[link_section = ".init.text"]
pub(super) unsafe fn do_pre_smp_initcalls() {
    // SAFETY: the caller owns the original early initcall phase and ranges.
    unsafe {
        main_initcall_trace::do_trace_initcall_level(b"early\0".as_ptr().cast());
        let mut entry =
            ptr::addr_of_mut!(bindings::__initcall_start).cast::<bindings::initcall_entry_t>();
        let end =
            ptr::addr_of_mut!(bindings::__initcall0_start).cast::<bindings::initcall_entry_t>();
        while entry < end {
            let function = init_header::initcall_from_entry(entry.cast()).unwrap_unchecked();
            main_initcall::do_one_initcall(function);
            entry = entry.wrapping_add(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
