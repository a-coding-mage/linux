/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <stdbool.h> and "util/debug.h".

use core::ffi::{c_char, c_int, c_void};

pub const TEST_OK: c_int = 0;
pub const TEST_FAIL: c_int = -1;
pub const TEST_SKIP: c_int = -2;

macro_rules! TEST_ASSERT_VAL {
    ($text:expr, $cond:expr) => {
        do {
            if !$cond {
                pr_debug!(
                    "FAILED {}:{} {}\n",
                    file!(),
                    line!(),
                    $text
                );
                return TEST_FAIL;
            }
        }
    };
}

macro_rules! TEST_ASSERT_EQUAL {
    ($text:expr, $val:expr, $expected:expr) => {
        do {
            if $val != $expected {
                pr_debug!(
                    "FAILED {}:{} {} ({} != {})\n",
                    file!(),
                    line!(),
                    $text,
                    $val,
                    $expected
                );
                return TEST_FAIL;
            }
        }
    };
}

pub(crate) use TEST_ASSERT_EQUAL;
pub(crate) use TEST_ASSERT_VAL;

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
    pub priv_: *mut c_void,
    pub setup: Option<unsafe extern "C" fn(suite: *mut test_suite) -> c_int>,
}

pub type test_fnptr = unsafe extern "C" fn(*mut test_suite, c_int) -> c_int;

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub desc: *const c_char,
    pub skip_reason: *const c_char,
    pub run_case: Option<test_fnptr>,
    pub exclusive: bool,
    pub priv_: *mut c_void,
}

macro_rules! TEST_CASE {
    ($description:expr, $name:ident, $run_case:path) => {
        test_case {
            name: concat!(stringify!($name), "\0").as_ptr() as *const c_char,
            desc: $description,
            skip_reason: core::ptr::null(),
            run_case: Some($run_case),
            exclusive: false,
            priv_: core::ptr::null_mut(),
        }
    };
}

macro_rules! TEST_CASE_REASON {
    ($description:expr, $name:ident, $reason:expr, $run_case:path) => {
        test_case {
            name: concat!(stringify!($name), "\0").as_ptr() as *const c_char,
            desc: $description,
            skip_reason: $reason,
            run_case: Some($run_case),
            exclusive: false,
            priv_: core::ptr::null_mut(),
        }
    };
}

macro_rules! TEST_CASE_EXCLUSIVE {
    ($description:expr, $name:ident, $run_case:path) => {
        test_case {
            name: concat!(stringify!($name), "\0").as_ptr() as *const c_char,
            desc: $description,
            skip_reason: core::ptr::null(),
            run_case: Some($run_case),
            exclusive: true,
            priv_: core::ptr::null_mut(),
        }
    };
}

macro_rules! TEST_CASE_REASON_EXCLUSIVE {
    ($description:expr, $name:ident, $reason:expr, $run_case:path) => {
        test_case {
            name: concat!(stringify!($name), "\0").as_ptr() as *const c_char,
            desc: $description,
            skip_reason: $reason,
            run_case: Some($run_case),
            exclusive: true,
            priv_: core::ptr::null_mut(),
        }
    };
}

pub(crate) use TEST_CASE;
pub(crate) use TEST_CASE_EXCLUSIVE;
pub(crate) use TEST_CASE_REASON;
pub(crate) use TEST_CASE_REASON_EXCLUSIVE;

unsafe extern "C" {
    pub static mut suite__vmlinux_matches_kallsyms: test_suite;
    pub static mut suite__openat_syscall_event: test_suite;
    pub static mut suite__openat_syscall_event_on_all_cpus: test_suite;
    pub static mut suite__basic_mmap: test_suite;
    pub static mut suite__PERF_RECORD: test_suite;
    pub static mut suite__perf_evsel__roundtrip_name_test: test_suite;
    pub static mut suite__perf_evsel__tp_sched_test: test_suite;
    pub static mut suite__syscall_openat_tp_fields: test_suite;
    pub static mut suite__pmu: test_suite;
    pub static mut suite__pmu_events: test_suite;
    pub static mut suite__hwmon_pmu: test_suite;
    pub static mut suite__tool_pmu: test_suite;
    pub static mut suite__attr: test_suite;
    pub static mut suite__dso_data: test_suite;
    pub static mut suite__dso_data_cache: test_suite;
    pub static mut suite__dso_data_reopen: test_suite;
    pub static mut suite__parse_events: test_suite;
    pub static mut suite__hists_link: test_suite;
    pub static mut suite__bp_signal: test_suite;
    pub static mut suite__bp_signal_overflow: test_suite;
    pub static mut suite__bp_accounting: test_suite;
    pub static mut suite__wp: test_suite;
    pub static mut suite__task_exit: test_suite;
    pub static mut suite__mem: test_suite;
    pub static mut suite__sw_clock_freq: test_suite;
    pub static mut suite__code_reading: test_suite;
    pub static mut suite__sample_parsing: test_suite;
    pub static mut suite__keep_tracking: test_suite;
    pub static mut suite__parse_no_sample_id_all: test_suite;
    pub static mut suite__dwarf_unwind: test_suite;
    pub static mut suite__expr: test_suite;
    pub static mut suite__hists_filter: test_suite;
    pub static mut suite__mmap_thread_lookup: test_suite;
    pub static mut suite__thread_maps_share: test_suite;
    pub static mut suite__hists_output: test_suite;
    pub static mut suite__hists_cumulate: test_suite;
    pub static mut suite__switch_tracking: test_suite;
    pub static mut suite__fdarray__filter: test_suite;
    pub static mut suite__fdarray__add: test_suite;
    pub static mut suite__kmod_path__parse: test_suite;
    pub static mut suite__thread_map: test_suite;
    pub static mut suite__bpf: test_suite;
    pub static mut suite__session_topology: test_suite;
    pub static mut suite__thread_map_synthesize: test_suite;
    pub static mut suite__thread_map_remove: test_suite;
    pub static mut suite__cpu_map: test_suite;
    pub static mut suite__synthesize_stat_config: test_suite;
    pub static mut suite__synthesize_stat: test_suite;
    pub static mut suite__synthesize_stat_round: test_suite;
    pub static mut suite__event_update: test_suite;
    pub static mut suite__event_times: test_suite;
    pub static mut suite__backward_ring_buffer: test_suite;
    pub static mut suite__sdt_event: test_suite;
    pub static mut suite__is_printable_array: test_suite;
    pub static mut suite__bitmap_print: test_suite;
    pub static mut suite__perf_hooks: test_suite;
    pub static mut suite__unit_number__scnprint: test_suite;
    pub static mut suite__mem2node: test_suite;
    pub static mut suite__maps: test_suite;
    pub static mut suite__time_utils: test_suite;
    pub static mut suite__jit_write_elf: test_suite;
    pub static mut suite__api_io: test_suite;
    pub static mut suite__demangle_java: test_suite;
    pub static mut suite__demangle_ocaml: test_suite;
    pub static mut suite__demangle_rust: test_suite;
    pub static mut suite__pfm: test_suite;
    pub static mut suite__parse_metric: test_suite;
    pub static mut suite__pe_file_parsing: test_suite;
    pub static mut suite__expand_cgroup_events: test_suite;
    pub static mut suite__perf_time_to_tsc: test_suite;
    pub static mut suite__dlfilter: test_suite;
    pub static mut suite__sigtrap: test_suite;
    pub static mut suite__event_groups: test_suite;
    pub static mut suite__symbols: test_suite;
    pub static mut suite__util: test_suite;
    pub static mut suite__uncore_event_sorting: test_suite;
    pub static mut suite__subcmd_help: test_suite;
    pub static mut suite__kallsyms_split: test_suite;
}

/*
 * PowerPC and S390 do not support creation of instruction breakpoints using the
 * perf_event interface.
 *
 * ARM requires explicit rounding down of the instruction pointer in Thumb mode,
 * and then requires the single-step to be handled explicitly in the overflow
 * handler to avoid stepping into the SIGIO handler and getting stuck on the
 * breakpointed instruction.
 *
 * Since arm64 has the same issue with arm for the single-step handling, this
 * case also gets stuck on the breakpointed instruction.
 *
 * Just disable the test for these architectures until these issues are
 * resolved.
 */
#[cfg(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "s390x",
    target_arch = "arm",
    target_arch = "aarch64"
))]
pub const BP_SIGNAL_IS_SUPPORTED: c_int = 0;
#[cfg(not(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "s390x",
    target_arch = "arm",
    target_arch = "aarch64"
)))]
pub const BP_SIGNAL_IS_SUPPORTED: c_int = 1;

// Original condition: #ifdef HAVE_DWARF_UNWIND_SUPPORT.
#[cfg(HAVE_DWARF_UNWIND_SUPPORT)]
#[repr(C)]
pub struct thread {
    _unused: [u8; 0],
}

#[cfg(HAVE_DWARF_UNWIND_SUPPORT)]
#[repr(C)]
pub struct perf_sample {
    _unused: [u8; 0],
}

#[cfg(HAVE_DWARF_UNWIND_SUPPORT)]
unsafe extern "C" {
    pub fn test__arch_unwind_sample(sample: *mut perf_sample, thread: *mut thread) -> c_int;
}

// Original condition: #if defined(__arm__).
#[cfg(target_arch = "arm")]
unsafe extern "C" {
    pub static mut suite__vectors_page: test_suite;
}

/*
 * Define test workloads to be used in test suites.
 */
pub type workload_fnptr = unsafe extern "C" fn(argc: c_int, argv: *mut *const c_char) -> c_int;

#[repr(C)]
pub struct test_workload {
    pub name: *const c_char,
    pub func: Option<workload_fnptr>,
}

macro_rules! DEFINE_WORKLOAD {
    ($work:ident, $func:path) => {
        test_workload {
            name: concat!(stringify!($work), "\0").as_ptr() as *const c_char,
            func: Some($func),
        }
    };
}

pub(crate) use DEFINE_WORKLOAD;

unsafe extern "C" {
    pub static mut workload__noploop: test_workload;
    pub static mut workload__thloop: test_workload;
    pub static mut workload__named_threads: test_workload;
    pub static mut workload__leafloop: test_workload;
    pub static mut workload__sqrtloop: test_workload;
    pub static mut workload__brstack: test_workload;
    pub static mut workload__datasym: test_workload;
    pub static mut workload__landlock: test_workload;
    pub static mut workload__traploop: test_workload;
    pub static mut workload__inlineloop: test_workload;
    pub static mut workload__jitdump: test_workload;
    pub static mut workload__context_switch_loop: test_workload;
    pub static mut workload__deterministic: test_workload;
    pub static mut workload__callchain: test_workload;
}

// Original condition: #ifdef HAVE_RUST_SUPPORT.
#[cfg(HAVE_RUST_SUPPORT)]
unsafe extern "C" {
    pub static mut workload__code_with_type: test_workload;
}

unsafe extern "C" {
    pub static mut dso_to_test: *const c_char;
    pub static mut test_objdump_path: *const c_char;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
