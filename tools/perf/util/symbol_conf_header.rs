/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_ulong, c_ushort};

// C header dependencies intentionally left external:
// <stdbool.h>, <linux/bitmap.h>, and "perf.h".

#[repr(C)]
pub struct strlist {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct intlist {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum unwind_style {
    UNWIND_STYLE_UNKNOWN = 0,
    UNWIND_STYLE_LIBDW,
    UNWIND_STYLE_LIBUNWIND,
}

pub const MAX_UNWIND_STYLE: usize = unwind_style::UNWIND_STYLE_LIBUNWIND as usize + 1;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum a2l_style {
    A2L_STYLE_UNKNOWN = 0,
    A2L_STYLE_LIBDW,
    A2L_STYLE_LLVM,
    A2L_STYLE_LIBBFD,
    A2L_STYLE_CMD,
}

pub const MAX_A2L_STYLE: usize = a2l_style::A2L_STYLE_CMD as usize + 1;

#[repr(C)]
pub struct symbol_conf {
    pub nanosecs: bool,
    pub priv_size: c_ushort,
    pub try_vmlinux_path: bool,
    pub init_annotation: bool,
    pub force: bool,
    pub ignore_vmlinux: bool,
    pub ignore_vmlinux_buildid: bool,
    pub show_kernel_path: bool,
    pub use_modules: bool,
    pub allow_aliases: bool,
    pub show_nr_samples: bool,
    pub show_total_period: bool,
    pub use_callchain: bool,
    pub cumulate_callchain: bool,
    pub show_branchflag_count: bool,
    pub exclude_other: bool,
    pub show_cpu_utilization: bool,
    pub initialized: bool,
    pub kptr_restrict: bool,
    pub event_group: bool,
    pub demangle: bool,
    pub demangle_kernel: bool,
    pub filter_relative: bool,
    pub show_hist_headers: bool,
    pub has_filter: bool,
    pub show_ref_callgraph: bool,
    pub hide_unresolved: bool,
    pub raw_trace: bool,
    pub report_hierarchy: bool,
    pub report_block: bool,
    pub report_individual_block: bool,
    pub inline_name: bool,
    pub addr2line_disable_warn: bool,
    pub no_buildid_mmap2: bool,
    pub guest_code: bool,
    pub lazy_load_kernel_maps: bool,
    pub keep_exited_threads: bool,
    pub annotate_data_member: bool,
    pub annotate_data_sample: bool,
    pub skip_empty: bool,
    pub enable_latency: bool,
    pub prefer_latency: bool,
    pub vmlinux_name: *const c_char,
    pub kallsyms_name: *const c_char,
    pub source_prefix: *const c_char,
    pub field_sep: *const c_char,
    pub graph_function: *const c_char,
    pub default_guest_vmlinux_name: *const c_char,
    pub default_guest_kallsyms: *const c_char,
    pub default_guest_modules: *const c_char,
    pub guestmount: *const c_char,
    pub dso_list_str: *const c_char,
    pub comm_list_str: *const c_char,
    pub pid_list_str: *const c_char,
    pub tid_list_str: *const c_char,
    pub sym_list_str: *const c_char,
    pub parallelism_list_str: *const c_char,
    pub col_width_list_str: *const c_char,
    pub bt_stop_list_str: *const c_char,
    pub addr2line_path: *const c_char,
    pub addr2line_style: [a2l_style; MAX_A2L_STYLE],
    pub addr2line_timeout_ms: c_int,
    pub unwind_style: [unwind_style; MAX_UNWIND_STYLE],
    pub time_quantum: c_ulong,
    pub dso_list: *mut strlist,
    pub comm_list: *mut strlist,
    pub sym_list: *mut strlist,
    pub dso_from_list: *mut strlist,
    pub dso_to_list: *mut strlist,
    pub sym_from_list: *mut strlist,
    pub sym_to_list: *mut strlist,
    pub bt_stop_list: *mut strlist,
    pub pid_list: *mut intlist,
    pub tid_list: *mut intlist,
    pub addr_list: *mut intlist,
    pub symfs: *const c_char,
    pub symfs_layout_flat: bool,
    pub res_sample: c_int,
    pub pad_output_len_dso: c_int,
    pub group_sort_idx: c_int,
    pub addr_range: c_int,
    // DECLARE_BITMAP(parallelism_filter, MAX_NR_CPUS + 1);
    pub parallelism_filter: [c_ulong; BITS_TO_LONGS(MAX_NR_CPUS + 1)],
}

unsafe extern "C" {
    pub static mut symbol_conf: symbol_conf;
}
