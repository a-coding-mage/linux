// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct feature_status {
    pub name: *const c_char,
    pub macro_: *const c_char,
    pub tip: *const c_char,
    pub is_builtin: c_int,
}

// Forward declaration from C: struct cmdnames;
pub enum cmdnames {}

unsafe extern "C" {
    pub static mut supported_features: [feature_status; 0];

    pub fn feature_status__printf(feature: *const feature_status);

    pub fn list_common_cmds_help();
    pub fn help_unknown_cmd(cmd: *const c_char, main_cmds: *mut cmdnames) -> *const c_char;

    pub fn cmd_annotate(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_bench(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_buildid_cache(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_buildid_list(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_check(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_config(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_c2c(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_diff(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_evlist(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_help(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_sched(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_kallsyms(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_list(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_record(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_report(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_stat(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_timechart(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_top(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_script(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_version(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_probe(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_kmem(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_lock(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_kvm(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_test(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_trace(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_inject(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_mem(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_data(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_ftrace(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_daemon(argc: c_int, argv: *const *const c_char) -> c_int;
    pub fn cmd_kwork(argc: c_int, argv: *const *const c_char) -> c_int;
}
