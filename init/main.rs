// SPDX-License-Identifier: GPL-2.0-only
//! Staged translation of linux/init/main.c with the original boot sequence.
//! Native integration and architecture validation precede Kbuild selection.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
// Kbuild enables linkage/no_sanitize only for this native boot owner.

// Built-in boot declarations must be generated with init_main.h; the ordinary
// kernel bindings use MODULE and deliberately omit built-in setup records.
use init_main_bindings as bindings;

mod main_globals;
pub use main_globals::*;
mod main_setup;
mod main_command_line;
mod main_bootconfig;
mod main_completion;
mod main_rodata;
mod main_parameters;
mod main_printk;
mod main_debug;
mod main_initcall_types;
mod main_initcall_trace;
mod main_warn;
mod main_initcall_context;
mod main_initcall;
pub use main_initcall::do_one_initcall;
mod main_initcall_levels;
mod main_freeable;
mod main_kernel_init;
mod main_rest;
mod main_start_arch;
mod main_random_kstack;
mod main_start;
pub use main_start::start_kernel;
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
mod main_ctors;
mod main_weak;
pub use main_command_line::cmdline_has_extra_options;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
