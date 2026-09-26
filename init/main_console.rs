// SPDX-License-Identifier: GPL-2.0-only
//! Open the original initial console and install stdin, stdout, and stderr.

use super::{bindings, main_printk::main_printk};
use kernel::ffi::c_int;

/// Open and duplicate the boot console using the original init-only services.
///
/// # Safety
///
/// The caller has reached the serialized init task's rootfs setup phase and
/// satisfies the original file-table and filesystem service prerequisites.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn console_on_rootfs() {
    // SAFETY: the caller establishes the original boot lifecycle. filp_open
    // returns either an error pointer or the reference consumed by fput after
    // all three init_dup calls. As in C, duplication return values are ignored.
    unsafe {
        let file = bindings::filp_open(
            c"/dev/console".as_ptr().cast(),
            bindings::O_RDWR as c_int,
            0,
        );
        if kernel::bindings::IS_ERR(file.cast()) {
            main_printk!(
                "console_on_rootfs",
                b"\x013Warning: unable to open an initial console.\n\0"
            );
            return;
        }
        bindings::init_dup(file);
        bindings::init_dup(file);
        bindings::init_dup(file);
        bindings::fput(file);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
