// SPDX-License-Identifier: GPL-2.0
/*
 * Handling of different ABIs (personalities).
 *
 * We group personalities into execution domains which have their
 * own handlers for kernel entry points, signal mapping, etc...
 *
 * 2001-05-06	Complete rewrite,  Christoph Hellwig (hch@infradead.org)
 */

// Dependencies: linux/init.h, linux/personality.h, linux/proc_fs.h,
// linux/sched.h, linux/seq_file.h, linux/syscalls.h.
//
// The SYSCALL_DEFINE1(personality) entry wrapper and the module_init()
// record stay in kernel/exec_domain_helpers.c; they call the Rust bodies.

use kernel::bindings::get_current;
use kernel::ffi::c_uint;

/// The /proc/execdomains entry (CONFIG_PROC_FS).
#[cfg(CONFIG_PROC_FS)]
pub mod procfs {
    use core::ffi::c_void;
    use core::ptr::null_mut;
    use kernel::bindings::{seq_file, seq_write};
    use kernel::ffi::{c_char, c_int, c_uint};

    // seq_file embeds zero-sized lockdep keys; only pointers cross here.
    #[allow(improper_ctypes)]
    extern "C" {
        fn proc_create_single_data(
            name: *const c_char,
            mode: c_uint,
            parent: *mut c_void,
            show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int,
            data: *mut c_void,
        ) -> *mut c_void;
    }

    /// The /proc/execdomains show callback.
    ///
    /// # Safety
    /// `m` is the seq_file being shown.
    unsafe extern "C" fn execdomains_proc_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
        // seq_puts() of a constant string is seq_write() of its length.
        let s = c"0-0\tLinux           \t[kernel]\n";
        // SAFETY: m is live for the show callback.
        unsafe { seq_write(m, s.as_ptr().cast::<c_void>(), s.count_bytes()) };
        0
    }

    /// The module_init() of the original unit.
    ///
    /// # Safety
    /// Called once during boot.
    #[no_mangle]
    #[link_section = ".init.text"]
    pub unsafe extern "C" fn proc_execdomains_init() -> c_int {
        // SAFETY: proc_create_single(name, mode, NULL, show).
        unsafe {
            proc_create_single_data(c"execdomains".as_ptr().cast(), 0, null_mut(), execdomains_proc_show, null_mut());
        }
        0
    }
}

/// The body of SYSCALL_DEFINE1(personality, unsigned int, personality).
#[no_mangle]
pub extern "C" fn lupos_sys_personality(personality: c_uint) -> c_uint {
    // SAFETY: current is the running task; set_personality() assigns
    // current->personality.
    unsafe {
        let current = get_current();
        let old = (*current).personality;

        if personality != 0xffffffff {
            (*current).personality = personality;
        }

        old
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
