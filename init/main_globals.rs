// SPDX-License-Identifier: GPL-2.0-only
//! Global state owned by the staged, currently unselected `init/main.rs`.
//!
//! Types and configuration constants come from the canonical init-main binding
//! input. This module is not linked beside the C owner. Completion, per-CPU,
//! static-key and tracepoint initialization remain part of the unfinished boot
//! implementation; these definitions do not make that implementation selectable.
//! As with the existing division owner, `linkage` must be allowed only for this
//! future native owner, so C-private state retains actual internal linkage.

#![allow(dead_code, non_upper_case_globals, missing_docs)]

use super::bindings;
use core::ptr;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong};

#[path = "../rust/ffi_export.rs"]
mod ffi_export;

#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut early_boot_irqs_disabled: bool = false;

#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut system_state: bindings::system_states = bindings::system_states_SYSTEM_BOOTING;
ffi_export::export_symbol!(system_state, system_state, "", "");

#[no_mangle]
#[link_section = ".init.data"]
pub static mut late_time_init: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
#[link_section = ".init.data"]
pub static mut boot_command_line: [c_char; bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize] =
    [0; bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize];

#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut saved_command_line: *mut c_char = ptr::null_mut();

#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut saved_command_line_len: c_uint = 0;

#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut static_command_line: *mut c_char = ptr::null_mut();
#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut extra_command_line: *mut c_char = ptr::null_mut();
#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut extra_init_args: *mut c_char = ptr::null_mut();

#[cfg(CONFIG_BOOT_CONFIG)]
#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut bootconfig_found: bool = false;
#[cfg(CONFIG_BOOT_CONFIG)]
#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut initargs_offs: usize = 0;
#[cfg(not(CONFIG_BOOT_CONFIG))]
pub(super) const bootconfig_found: bool = false;
#[cfg(not(CONFIG_BOOT_CONFIG))]
pub(super) const initargs_offs: usize = 0;

#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut execute_command: *mut c_char = ptr::null_mut();
#[used]
#[link_section = ".data"]
#[linkage = "internal"]
pub(super) static mut ramdisk_execute_command: *mut c_char = b"/init\0".as_ptr().cast_mut().cast();
#[used]
#[link_section = ".init.data"]
#[linkage = "internal"]
pub(super) static mut ramdisk_execute_command_set: bool = false;

#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut static_key_initialized: bool = false;
ffi_export::export_symbol!(static_key_initialized, static_key_initialized, "GPL", "");

#[no_mangle]
#[link_section = ".bss"]
pub static mut reset_devices: c_uint = 0;
ffi_export::export_symbol!(reset_devices, reset_devices, "", "");

#[used]
#[link_section = ".data"]
#[linkage = "internal"]
pub(super) static mut argv_init: [*const c_char;
    bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize + 2] = {
    let mut argv = [ptr::null(); bindings::RUST_INIT_MAIN_MAX_INIT_ARGS as usize + 2];
    argv[0] = b"init\0".as_ptr().cast();
    argv
};

#[no_mangle]
#[link_section = ".data"]
pub static mut envp_init: [*const c_char; bindings::RUST_INIT_MAIN_MAX_INIT_ENVS as usize + 2] = {
    let mut envp = [ptr::null(); bindings::RUST_INIT_MAIN_MAX_INIT_ENVS as usize + 2];
    envp[0] = b"HOME=/\0".as_ptr().cast();
    envp[1] = b"TERM=linux\0".as_ptr().cast();
    envp
};

#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut panic_later: *const c_char = ptr::null();
#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut panic_param: *const c_char = ptr::null();

#[no_mangle]
#[link_section = ".data"]
pub static mut loops_per_jiffy: c_ulong = 1 << 12;
ffi_export::export_symbol!(loops_per_jiffy, loops_per_jiffy, "", "");

#[used]
#[link_section = ".init.data"]
#[linkage = "internal"]
pub(super) static mut parse_early_param_done: c_int = 0;
#[used]
#[link_section = ".init.data"]
#[linkage = "internal"]
pub(super) static mut parse_early_param_tmp_cmdline: [c_char;
    bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize] =
    [0; bindings::RUST_INIT_MAIN_COMMAND_LINE_SIZE as usize];

#[no_mangle]
#[link_section = ".bss"]
pub static mut initcall_debug: bool = false;

#[cfg(CONFIG_KALLSYMS)]
#[used]
#[cfg_attr(not(CONFIG_MODULES), link_section = ".init.data")]
#[cfg_attr(CONFIG_MODULES, link_section = ".data")]
#[linkage = "internal"]
pub(super) static mut blacklisted_initcalls: bindings::list_head = bindings::list_head {
    next: ptr::addr_of_mut!(blacklisted_initcalls),
    prev: ptr::addr_of_mut!(blacklisted_initcalls),
};

#[used]
#[link_section = ".bss"]
#[linkage = "internal"]
pub(super) static mut initcall_calltime: bindings::ktime_t = 0;

#[used]
#[link_section = ".init.data"]
#[linkage = "internal"]
pub(super) static mut initcall_levels: [*mut bindings::initcall_entry_t; 9] = {
    // Only linker-provided symbol addresses are formed; no memory is
    // accessed while creating this table, just as in the C initializer.
    [
        ptr::addr_of_mut!(bindings::__initcall0_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall1_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall2_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall3_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall4_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall5_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall6_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall7_start).cast(),
        ptr::addr_of_mut!(bindings::__initcall_end).cast(),
    ]
};

#[used]
#[link_section = ".init.data"]
#[linkage = "internal"]
pub(super) static mut initcall_level_names: [*const c_char; 8] = [
    b"pure\0".as_ptr().cast(),
    b"core\0".as_ptr().cast(),
    b"postcore\0".as_ptr().cast(),
    b"arch\0".as_ptr().cast(),
    b"subsys\0".as_ptr().cast(),
    b"fs\0".as_ptr().cast(),
    b"device\0".as_ptr().cast(),
    b"late\0".as_ptr().cast(),
];

#[cfg(any(CONFIG_STRICT_KERNEL_RWX, CONFIG_STRICT_MODULE_RWX))]
#[no_mangle]
#[link_section = ".data..ro_after_init"]
pub static mut rodata_enabled: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
