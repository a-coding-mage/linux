// SPDX-License-Identifier: GPL-2.0-only
/* Staged translation of linux/init/main.c. Boot control flow is incomplete;
 * this file is deliberately not selected by Kbuild. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
#![feature(linkage)]

use kernel::bindings;
use kernel::ffi::{c_char, c_int, c_void};

mod main_globals;
use main_globals::*;
mod main_setup;
mod main_command_line;
mod main_parameters;
mod main_printk;
mod main_debug;
mod main_initcall_types;
mod main_initcall_trace;
mod main_print;
mod main_exec;
mod main_bootoptions;
mod main_early;
pub use main_early::{parse_early_options, parse_early_param};
mod main_console;
pub use main_console::console_on_rootfs;
mod main_unknown;
mod main_core_param;
mod main_blacklist;
#[allow(unused_imports)]
use main_blacklist::{initcall_blacklist, initcall_blacklisted};
mod main_ctors;
#[allow(unused_imports)]
use main_ctors::do_ctors;
mod main_weak;
#[allow(unused_imports)]
use main_unknown::print_unknown_bootoptions;
pub use main_command_line::cmdline_has_extra_options;
#[allow(unused_imports)] // Call sites in the remaining boot sequence are staged next.
use main_command_line::{init_setup, rdinit_setup, setup_command_line};
#[allow(unused_imports)]
use main_parameters::{debug_kernel, loglevel, quiet_kernel, set_reset_devices};
#[allow(unused_imports)]
use main_print::print_kernel_cmdline;
#[allow(unused_imports)]
use main_exec::{execute_init_processes, run_init_process, try_to_run_init_process};
#[allow(unused_imports)]
use main_bootoptions::{obsolete_checksetup, repair_env_string, set_init_arg, unknown_bootoption};

type size_t = usize;
type uint = u32;
type u8_t = u8;
type u32_t = u32;
type ulong = usize;
type ktime_t = i64;
type initcall_t = unsafe extern "C" fn() -> c_int;
type initcall_entry_t = c_void;

extern "C" {
    fn kernel_init(_: *mut c_void) -> !;
    fn strlen(_: *const c_char) -> size_t;
    fn strcmp(_: *const c_char, _: *const c_char) -> c_int;
    fn strncmp(_: *const c_char, _: *const c_char, _: size_t) -> c_int;
    fn memcmp(_: *const c_void, _: *const c_void, _: size_t) -> c_int;
    fn memcpy(_: *mut c_void, _: *const c_void, _: size_t) -> *mut c_void;
    fn memmove(_: *mut c_void, _: *const c_void, _: size_t) -> *mut c_void;
    fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;
    fn strchr(_: *const c_char, _: c_int) -> *mut c_char;
    fn strstarts(_: *const c_char, _: *const c_char) -> bool;
    fn strnchr(_: *const c_char, _: size_t, _: c_int) -> *mut c_char;
    fn strim(_: *mut c_char) -> *mut c_char;
    fn memblock_alloc_or_panic(_: size_t, _: size_t) -> *mut c_char;
    fn memblock_alloc(_: size_t, _: size_t) -> *mut c_char;
    fn memblock_free(_: *mut c_char, _: size_t);
    fn pr_warn(_: *const c_char, ...);
    fn pr_notice(_: *const c_char, ...);
    fn pr_debug(_: *const c_char, ...);
    fn panic(_: *const c_char, ... ) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn do_one_initcall(fn_: initcall_t) -> c_int { fn_() }

#[no_mangle]
pub unsafe extern "C" fn start_kernel() -> ! {
    pr_notice(b"%s\0".as_ptr() as *const c_char, b"Linux\0".as_ptr());
    panic(b"start_kernel requires the Linux kernel runtime\0".as_ptr() as *const c_char)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
