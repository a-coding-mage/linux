// SPDX-License-Identifier: GPL-2.0-only
//! Report arguments retained for init after kernel command-line parsing.

use super::{bindings, main_globals::*, main_printk::main_printk};
use kernel::ffi::c_char;

/// Print the original unknown-option notice and release its early allocation.
///
/// # Safety
///
/// Early boot serializes access to the init argument/environment arrays and
/// deferred-panic state. When no panic is pending, both arrays are terminated
/// and every nonnull entry points to a live terminated string.
#[link_section = ".init.text"]
pub(super) unsafe fn print_unknown_bootoptions() {
    // SAFETY: the arrays and their borrowed string storage follow the original
    // boot parser's lifetimes. Raw pointers avoid references across callbacks.
    unsafe {
        let arguments = core::ptr::addr_of!(argv_init).cast::<*const c_char>().wrapping_add(1);
        let environment = core::ptr::addr_of!(envp_init).cast::<*const c_char>().wrapping_add(2);
        if !panic_later.is_null() || (arguments.read().is_null() && environment.read().is_null()) {
            return;
        }
        let mut length = 1usize;
        for start in [arguments, environment] {
            let mut entry = start;
            while !entry.read().is_null() {
                length = length.wrapping_add(1).wrapping_add(bindings::strlen(entry.read()));
                entry = entry.wrapping_add(1);
            }
        }
        // Original memblock_alloc() is this inline call with accessible memory
        // limits and no preferred NUMA node; it may return NULL.
        let buffer = bindings::memblock_alloc_try_nid(
            length as bindings::phys_addr_t,
            bindings::RUST_INIT_MAIN_SMP_CACHE_BYTES as bindings::phys_addr_t,
            bindings::RUST_INIT_MAIN_MEMBLOCK_LOW_LIMIT,
            bindings::RUST_INIT_MAIN_MEMBLOCK_ALLOC_ACCESSIBLE,
            bindings::NUMA_NO_NODE,
        ).cast::<c_char>();
        if buffer.is_null() {
            main_printk!("print_unknown_bootoptions", b"\x013%s: Failed to allocate %zu bytes\n\0",
                c"print_unknown_bootoptions".as_ptr().cast::<c_char>(), length);
            return;
        }
        let mut end = buffer;
        for start in [arguments, environment] {
            let mut entry = start;
            while !entry.read().is_null() {
                let written = bindings::sprintf(end, c" %s".as_ptr().cast(), entry.read());
                end = end.wrapping_offset(written as isize);
                entry = entry.wrapping_add(1);
            }
        }
        main_printk!("print_unknown_bootoptions",
            b"\x015Unknown kernel command line parameters \"%s\", will be passed to user space.\n\0",
            buffer.wrapping_add(1));
        bindings::memblock_free(buffer.cast(), length);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
