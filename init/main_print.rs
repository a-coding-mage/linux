// SPDX-License-Identifier: GPL-2.0-only
//! Original command-line wrapping for the staged, unselected init/main owner.

use super::{bindings, main_printk::main_printk};
use kernel::ffi::{c_char, c_int};

const PREFIX: &[u8] = b"Kernel command line: \0";
const CONTINUATION: &[u8] = b" \\\0";
const PREFIX_LEN: usize = PREFIX.len() - 1;
const CONTINUATION_LEN: usize = CONTINUATION.len() - 1;
const CONFIG_WRAP: usize = bindings::RUST_INIT_MAIN_CMDLINE_LOG_WRAP_IDEAL_LEN as usize;
const MIN_WRAP: usize = PREFIX_LEN + CONTINUATION_LEN;
const WRAP: usize = if CONFIG_WRAP > MIN_WRAP { CONFIG_WRAP } else { MIN_WRAP };
const IDEAL_LEN: usize = WRAP - PREFIX_LEN;
const SPLIT_LEN: usize = IDEAL_LEN - CONTINUATION_LEN;
const WRAPPING: bool = CONFIG_WRAP != 0
    && IDEAL_LEN < bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize - 1;

/// Print the original space-separated wrapping, including its empty-line rule.
///
/// Quotes do not affect the split points. Leading runs of spaces are skipped
/// only when a split consumes them; final trailing spaces are retained. A zero
/// configuration or one at the original command-line-size cutoff disables all
/// splitting and emits even an empty command line.
///
/// # Safety
///
/// `cmdline` must be a readable NUL-terminated C string for the entire call.
#[link_section = ".init.text"]
pub(super) unsafe fn print_kernel_cmdline(mut cmdline: *const c_char) {
    // SAFETY: the caller supplies a terminated readable C string. Every cursor
    // stays within that string, and every precision is promoted to the same C
    // int as in the original implementation before reaching printk's varargs.
    unsafe {
        if !WRAPPING {
            main_printk!(@index !WRAPPING, "print_kernel_cmdline", b"\x015%s%s\n\0", PREFIX.as_ptr(), cmdline);
            return;
        }
        let mut len = bindings::strlen(cmdline);
        while len > IDEAL_LEN {
            let mut previous = core::ptr::null();
            let mut cutoff = cmdline;
            loop {
                cutoff = bindings::strchr(cutoff.add(1), b' ' as c_int);
                if cutoff.is_null() || cutoff.offset_from(cmdline) as usize > SPLIT_LEN {
                    break;
                }
                previous = cutoff;
            }
            if !previous.is_null() {
                cutoff = previous;
            } else if cutoff.is_null() {
                break;
            }
            let mut first_space = cutoff;
            while first_space > cmdline && first_space.sub(1).read() == b' ' as c_char {
                first_space = first_space.sub(1);
            }
            let to_print = first_space.offset_from(cmdline) as c_int;
            while cutoff.read() == b' ' as c_char {
                cutoff = cutoff.add(1);
            }
            let used = cutoff.offset_from(cmdline) as usize;
            if len == used {
                break;
            }
            if to_print != 0 {
                main_printk!(@index WRAPPING, "print_kernel_cmdline", b"\x015%s%.*s%s\n\0",
                             PREFIX.as_ptr(), to_print, cmdline, CONTINUATION.as_ptr());
            }
            len -= used;
            cmdline = cmdline.add(used);
        }
        if len != 0 {
            main_printk!(@index WRAPPING, "print_kernel_cmdline", b"\x015%s%s\n\0", PREFIX.as_ptr(), cmdline);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
