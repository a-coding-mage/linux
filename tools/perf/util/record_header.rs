/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies preserved for context:
// <time.h>, <stdbool.h>, <linux/types.h>, <linux/stddef.h>,
// <linux/perf_event.h>, "util/target.h"

use core::ffi::c_char;

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    pub target: target,
    pub inherit_stat: bool,
    pub no_buffering: bool,
    pub no_inherit: bool,
    pub no_inherit_set: bool,
    pub no_samples: bool,
    pub raw_samples: bool,
    pub sample_address: bool,
    pub sample_phys_addr: bool,
    pub sample_data_page_size: bool,
    pub sample_code_page_size: bool,
    pub sample_weight: bool,
    pub sample_time: bool,
    pub sample_time_set: bool,
    pub sample_cpu: bool,
    pub sample_identifier: bool,
    pub sample_data_src: bool,
    pub period: bool,
    pub period_set: bool,
    pub running_time: bool,
    pub full_auxtrace: bool,
    pub auxtrace_snapshot_mode: bool,
    pub auxtrace_snapshot_on_exit: bool,
    pub auxtrace_sample_mode: bool,
    pub record_namespaces: bool,
    pub record_cgroup: bool,
    pub record_switch_events: bool,
    pub record_switch_events_set: bool,
    pub record_data_mmap: bool,
    pub record_data_mmap_set: bool,
    pub all_kernel: bool,
    pub all_user: bool,
    pub kernel_callchains: bool,
    pub user_callchains: bool,
    pub tail_synthesize: bool,
    pub overwrite: bool,
    pub ignore_missing_thread: bool,
    pub strict_freq: bool,
    pub sample_id: bool,
    pub no_bpf_event: bool,
    pub kcore: bool,
    pub text_poke: bool,
    pub build_id: bool,
    pub freq: u32,
    pub mmap_pages: u32,
    pub auxtrace_mmap_pages: u32,
    pub user_freq: u32,
    pub branch_stack: u64,
    pub sample_intr_regs: u64,
    pub sample_user_regs: u64,
    pub default_interval: u64,
    pub user_interval: u64,
    pub auxtrace_snapshot_size: usize,
    pub auxtrace_snapshot_opts: *const c_char,
    pub auxtrace_sample_opts: *const c_char,
    pub sample_transaction: bool,
    pub use_clockid: bool,
    pub clockid: clockid_t,
    pub clockid_res_ns: u64,
    pub nr_cblocks: i32,
    pub affinity: i32,
    pub mmap_flush: i32,
    pub comp_level: u32,
    pub nr_threads_synthesize: u32,
    pub ctl_fd: i32,
    pub ctl_fd_ack: i32,
    pub ctl_fd_close: bool,
    pub synth: i32,
    pub threads_spec: i32,
    pub threads_user_spec: *const c_char,
    pub off_cpu_thresh_ns: u64,
}

unsafe extern "C" {
    pub static record_usage: *const *const c_char;
    pub static mut record_options: *mut option;

    pub fn record__parse_freq(opt: *const option, str_: *const c_char, unset: i32) -> i32;
}

#[inline]
pub unsafe fn record_opts__no_switch_events(opts: *const record_opts) -> bool {
    unsafe { (*opts).record_switch_events_set && !(*opts).record_switch_events }
}
