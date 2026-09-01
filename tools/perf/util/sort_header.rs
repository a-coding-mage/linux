/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;
pub type int64_t = i64;
pub type u8 = u8;

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tep_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_hpp_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sort_mode {
    SORT_MODE__NORMAL,
    SORT_MODE__BRANCH,
    SORT_MODE__MEMORY,
    SORT_MODE__TOP,
    SORT_MODE__DIFF,
    SORT_MODE__TRACEPOINT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum sort_type {
    /* common sort keys */
    SORT_PID,
    SORT_COMM,
    SORT_COMM_NODIGIT,
    SORT_DSO,
    SORT_SYM,
    SORT_PARENT,
    SORT_CPU,
    SORT_SOCKET,
    SORT_SRCLINE,
    SORT_SRCFILE,
    SORT_LOCAL_WEIGHT,
    SORT_GLOBAL_WEIGHT,
    SORT_TRANSACTION,
    SORT_TRACE,
    SORT_SYM_SIZE,
    SORT_DSO_SIZE,
    SORT_CGROUP,
    SORT_CGROUP_ID,
    SORT_SYM_IPC_NULL,
    SORT_TIME,
    SORT_CODE_PAGE_SIZE,
    SORT_LOCAL_INS_LAT,
    SORT_GLOBAL_INS_LAT,
    SORT_LOCAL_PIPELINE_STAGE_CYC,
    SORT_GLOBAL_PIPELINE_STAGE_CYC,
    SORT_ADDR,
    SORT_LOCAL_RETIRE_LAT,
    SORT_GLOBAL_RETIRE_LAT,
    SORT_SIMD,
    SORT_ANNOTATE_DATA_TYPE,
    SORT_ANNOTATE_DATA_TYPE_OFFSET,
    SORT_SYM_OFFSET,
    SORT_ANNOTATE_DATA_TYPE_CACHELINE,
    SORT_PARALLELISM,
    SORT_TGID,

    /* branch stack specific sort keys */
    __SORT_BRANCH_STACK,
    SORT_DSO_FROM = sort_type::__SORT_BRANCH_STACK as isize,
    SORT_DSO_TO,
    SORT_SYM_FROM,
    SORT_SYM_TO,
    SORT_MISPREDICT,
    SORT_ABORT,
    SORT_IN_TX,
    SORT_CYCLES,
    SORT_SRCLINE_FROM,
    SORT_SRCLINE_TO,
    SORT_SYM_IPC,
    SORT_ADDR_FROM,
    SORT_ADDR_TO,
    SORT_CALLCHAIN_BRANCH_PREDICTED,
    SORT_CALLCHAIN_BRANCH_ABORT,
    SORT_CALLCHAIN_BRANCH_CYCLES,

    /* memory mode specific sort keys */
    __SORT_MEMORY_MODE,
    SORT_MEM_DADDR_SYMBOL = sort_type::__SORT_MEMORY_MODE as isize,
    SORT_MEM_DADDR_DSO,
    SORT_MEM_LOCKED,
    SORT_MEM_TLB,
    SORT_MEM_LVL,
    SORT_MEM_SNOOP,
    SORT_MEM_DCACHELINE,
    SORT_MEM_IADDR_SYMBOL,
    SORT_MEM_PHYS_DADDR,
    SORT_MEM_DATA_PAGE_SIZE,
    SORT_MEM_BLOCKED,
}

/*
 * configurable sorting bits
 */

#[repr(C)]
pub struct sort_entry {
    pub se_header: *const c_char,

    pub se_cmp: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    pub se_collapse: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    pub se_sort: Option<unsafe extern "C" fn(*mut hist_entry, *mut hist_entry) -> int64_t>,
    pub se_snprintf:
        Option<unsafe extern "C" fn(*mut hist_entry, *mut c_char, size_t, c_uint) -> c_int>,
    pub se_filter: Option<unsafe extern "C" fn(*mut hist_entry, c_int, *const c_void) -> c_int>,
    pub se_init: Option<unsafe extern "C" fn(*mut hist_entry)>,
    pub se_width_idx: u8,
}

unsafe extern "C" {
    pub static mut parent_regex: regex_t;
    pub static sort_order: *const c_char;
    pub static field_order: *const c_char;
    pub static default_parent_pattern: [c_char; 0];
    pub static parent_pattern: *const c_char;
    pub static default_sort_order: *const c_char;
    pub static mut ignore_callees_regex: regex_t;
    pub static mut have_ignore_callees: c_int;
    pub static mut sort__mode: sort_mode;
    pub static mut sort_comm: sort_entry;
    pub static mut sort_dso: sort_entry;
    pub static mut sort_sym: sort_entry;
    pub static mut sort_parent: sort_entry;
    pub static mut sort_dso_from: sort_entry;
    pub static mut sort_dso_to: sort_entry;
    pub static mut sort_sym_from: sort_entry;
    pub static mut sort_sym_to: sort_entry;
    pub static mut sort_srcline: sort_entry;
    pub static mut sort_type: sort_entry;
    pub static default_mem_sort_order: [c_char; 0];
    pub static mut chk_double_cl: bool;

    pub static mut sort_thread: sort_entry;

    pub fn setup_sorting(evlist: *mut evlist, env: *mut perf_env) -> c_int;
    pub fn setup_output_field() -> c_int;
    pub fn reset_output_field();
    pub fn sort__setup_elide(fp: *mut FILE);
    pub fn perf_hpp__set_elide(idx: c_int, elide: bool);

    pub fn sort_help(prefix: *const c_char, mode: sort_mode) -> *mut c_char;

    pub fn report_parse_ignore_callees_opt(
        opt: *const option,
        arg: *const c_char,
        unset: c_int,
    ) -> c_int;

    pub fn is_strict_order(order: *const c_char) -> bool;

    pub fn hpp_dimension__add_output(col: c_uint, implicit: bool) -> c_int;
    pub fn reset_dimensions();
    pub fn sort_dimension__add(
        list: *mut perf_hpp_list,
        tok: *const c_char,
        evlist: *mut evlist,
        env: *mut perf_env,
        level: c_int,
    ) -> c_int;
    pub fn output_field_add(
        list: *mut perf_hpp_list,
        tok: *const c_char,
        level: *mut c_int,
    ) -> c_int;
    pub fn sort__iaddr_cmp(left: *mut hist_entry, right: *mut hist_entry) -> int64_t;
    pub fn sort__daddr_cmp(left: *mut hist_entry, right: *mut hist_entry) -> int64_t;
    pub fn sort__dcacheline_cmp(left: *mut hist_entry, right: *mut hist_entry) -> int64_t;
    pub fn _sort__sym_cmp(sym_l: *mut symbol, sym_r: *mut symbol) -> int64_t;
    pub fn hist_entry__srcline(he: *mut hist_entry) -> *mut c_char;
    pub fn sort__comm_nodigit_len(entry: *mut hist_entry) -> size_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
