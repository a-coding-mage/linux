/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from resctrl.h. C include dependencies intentionally remain as
 * external Rust type/function dependencies for the final integration unit.
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

pub const MB: usize = 1024 * 1024;
pub const RESCTRL_PATH: *const c_char = b"/sys/fs/resctrl\0".as_ptr() as *const c_char;
pub const PHYS_ID_PATH: *const c_char =
    b"/sys/devices/system/cpu/cpu\0".as_ptr() as *const c_char;
pub const INFO_PATH: *const c_char = b"/sys/fs/resctrl/info\0".as_ptr() as *const c_char;

/*
 * CPU vendor IDs
 *
 * Define as bits because they're used for vendor_specific bitmask in
 * the struct resctrl_test.
 */
pub const ARCH_INTEL: c_uint = 1u32 << 0;
pub const ARCH_AMD: c_uint = 1u32 << 1;
pub const ARCH_HYGON: c_uint = 1u32 << 2;

pub const END_OF_TESTS: c_int = 1;

pub const BENCHMARK_ARGS: usize = 64;

pub const MINIMUM_SPAN: usize = 250 * MB;

/*
 * Memory bandwidth (in MiB) below which the bandwidth comparisons
 * between iMC and resctrl are considered unreliable. For example RAS
 * features or memory performance features that generate memory traffic
 * may drive accesses that are counted differently by performance counters
 * and MBM respectively, for instance generating "overhead" traffic which
 * is not counted against any specific RMID.
 */
pub const THROTTLE_THRESHOLD: c_int = 2500;

pub type size_t = usize;
pub type ssize_t = isize;
pub type pid_t = c_int;
pub type __u64 = u64;

/*
 * External C/system types supplied by included headers in the original C file:
 * FILE, cpu_set_t, siginfo_t, and struct perf_event_attr.
 */

/*
 * fill_buf_param:	"fill_buf" benchmark parameters
 * @buf_size:		Size (in bytes) of buffer used in benchmark.
 *			"fill_buf" allocates and initializes buffer of
 *			@buf_size. User can change value via command line.
 * @memflush:		If false the buffer will not be flushed after
 *			allocation and initialization, otherwise the
 *			buffer will be flushed. User can change value via
 *			command line (via integers with 0 interpreted as
 *			false and anything else as true).
 */
#[repr(C)]
pub struct fill_buf_param {
    pub buf_size: size_t,
    pub memflush: bool,
}

/*
 * user_params:		User supplied parameters
 * @cpu:		CPU number to which the benchmark will be bound to
 * @bits:		Number of bits used for cache allocation size
 * @benchmark_cmd:	Benchmark command to run during (some of the) tests
 * @fill_buf:		Pointer to user provided parameters for "fill_buf",
 *			NULL if user did not provide parameters and test
 *			specific defaults should be used.
 */
#[repr(C)]
pub struct user_params {
    pub cpu: c_int,
    pub bits: c_int,
    pub benchmark_cmd: [*const c_char; BENCHMARK_ARGS],
    pub fill_buf: *const fill_buf_param,
}

/*
 * resctrl_test:	resctrl test definition
 * @name:		Test name
 * @group:		Test group - a common name for tests that share some characteristic
 *			(e.g., L3 CAT test belongs to the CAT group). Can be NULL
 * @resource:		Resource to test (e.g., MB, L3, L2, etc.)
 * @vendor_specific:	Bitmask for vendor-specific tests (can be 0 for universal tests)
 * @disabled:		Test is disabled
 * @feature_check:	Callback to check required resctrl features
 * @run_test:		Callback to run the test
 * @cleanup:		Callback to cleanup after the test
 */
#[repr(C)]
pub struct resctrl_test {
    pub name: *const c_char,
    pub group: *const c_char,
    pub resource: *const c_char,
    pub vendor_specific: c_uint,
    pub disabled: bool,
    pub feature_check: Option<unsafe extern "C" fn(test: *const resctrl_test) -> bool>,
    pub run_test: Option<
        unsafe extern "C" fn(test: *const resctrl_test, uparams: *const user_params) -> c_int,
    >,
    pub cleanup: Option<unsafe extern "C" fn()>,
}

/*
 * resctrl_val_param:	resctrl test parameters
 * @ctrlgrp:		Name of the control monitor group (con_mon grp)
 * @mongrp:		Name of the monitor group (mon grp)
 * @filename:		Name of file to which the o/p should be written
 * @init:		Callback function to initialize test environment
 * @setup:		Callback function to setup per test run environment
 * @measure:		Callback that performs the measurement (a single test)
 * @fill_buf:		Parameters for default "fill_buf" benchmark.
 *			Initialized with user provided parameters, possibly
 *			adapted to be relevant to the test. If user does
 *			not provide parameters for "fill_buf" nor a
 *			replacement benchmark then initialized with defaults
 *			appropriate for test. NULL if user provided
 *			benchmark.
 */
#[repr(C)]
pub struct resctrl_val_param {
    pub ctrlgrp: *const c_char,
    pub mongrp: *const c_char,
    pub filename: [c_char; 64],
    pub mask: c_ulong,
    pub num_of_runs: c_int,
    pub init: Option<
        unsafe extern "C" fn(
            test: *const resctrl_test,
            uparams: *const user_params,
            param: *const resctrl_val_param,
            domain_id: c_int,
        ) -> c_int,
    >,
    pub setup: Option<
        unsafe extern "C" fn(
            test: *const resctrl_test,
            uparams: *const user_params,
            param: *mut resctrl_val_param,
        ) -> c_int,
    >,
    pub measure: Option<
        unsafe extern "C" fn(
            uparams: *const user_params,
            param: *mut resctrl_val_param,
            bm_pid: pid_t,
        ) -> c_int,
    >,
    pub fill_buf: *mut fill_buf_param,
}

unsafe extern "C" {
    /*
     * Memory location that consumes values compiler must not optimize away.
     * Volatile ensures writes to this location cannot be optimized away by
     * compiler.
     */
    pub static mut value_sink: *mut c_int;

    pub static mut snc_unreliable: c_int;

    pub static mut llc_occup_path: [c_char; 1024];

    pub fn snc_nodes_per_l3_cache() -> c_int;
    pub fn get_vendor() -> c_uint;
    pub fn check_resctrlfs_support() -> bool;
    pub fn filter_dmesg() -> c_int;
    pub fn get_domain_id(resource: *const c_char, cpu_no: c_int, domain_id: *mut c_int) -> c_int;
    pub fn mount_resctrlfs() -> c_int;
    pub fn umount_resctrlfs() -> c_int;
    pub fn resctrl_resource_exists(resource: *const c_char) -> bool;
    pub fn resctrl_mon_feature_exists(resource: *const c_char, feature: *const c_char) -> bool;
    pub fn resource_info_file_exists(resource: *const c_char, file: *const c_char) -> bool;
    pub fn test_resource_feature_check(test: *const resctrl_test) -> bool;
    pub fn fgrep(inf: *mut FILE, str: *const c_char) -> *mut c_char;
    pub fn taskset_benchmark(
        bm_pid: pid_t,
        cpu_no: c_int,
        old_affinity: *mut cpu_set_t,
    ) -> c_int;
    pub fn taskset_restore(bm_pid: pid_t, old_affinity: *mut cpu_set_t) -> c_int;
    pub fn write_schemata(
        ctrlgrp: *const c_char,
        schemata: *mut c_char,
        cpu_no: c_int,
        resource: *const c_char,
    ) -> c_int;
    pub fn write_bm_pid_to_resctrl(
        bm_pid: pid_t,
        ctrlgrp: *const c_char,
        mongrp: *const c_char,
    ) -> c_int;
    pub fn perf_event_open(
        hw_event: *mut perf_event_attr,
        pid: pid_t,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    pub fn alloc_buffer(buf_size: size_t, memflush: bool) -> *mut c_uchar;
    pub fn mem_flush(buf: *mut c_uchar, buf_size: size_t);
    pub fn fill_cache_read(buf: *mut c_uchar, buf_size: size_t, once: bool);
    pub fn get_fill_buf_size(cpu_no: c_int, cache_type: *const c_char) -> ssize_t;
    pub fn initialize_read_mem_bw_imc() -> c_int;
    pub fn measure_read_mem_bw(
        uparams: *const user_params,
        param: *mut resctrl_val_param,
        bm_pid: pid_t,
    ) -> c_int;
    pub fn initialize_mem_bw_resctrl(param: *const resctrl_val_param, domain_id: c_int);
    pub fn resctrl_val(
        test: *const resctrl_test,
        uparams: *const user_params,
        param: *mut resctrl_val_param,
    ) -> c_int;
    pub fn create_bit_mask(start: c_uint, len: c_uint) -> c_ulong;
    pub fn count_contiguous_bits(val: c_ulong, start: *mut c_uint) -> c_uint;
    pub fn get_full_cbm(cache_type: *const c_char, mask: *mut c_ulong) -> c_int;
    pub fn get_mask_no_shareable(cache_type: *const c_char, mask: *mut c_ulong) -> c_int;
    pub fn get_cache_size(
        cpu_no: c_int,
        cache_type: *const c_char,
        cache_size: *mut c_ulong,
    ) -> c_int;
    pub fn resource_info_unsigned_get(
        resource: *const c_char,
        filename: *const c_char,
        val: *mut c_uint,
    ) -> c_int;
    pub fn ctrlc_handler(signum: c_int, info: *mut siginfo_t, ptr: *mut c_void);
    pub fn signal_handler_register(test: *const resctrl_test) -> c_int;
    pub fn signal_handler_unregister();
    pub fn count_bits(n: c_ulong) -> c_uint;
    pub fn snc_kernel_support() -> c_int;

    pub fn perf_event_attr_initialize(pea: *mut perf_event_attr, config: __u64);
    pub fn perf_open(pea: *mut perf_event_attr, pid: pid_t, cpu_no: c_int) -> c_int;
    pub fn perf_event_reset_enable(pe_fd: c_int) -> c_int;
    pub fn perf_event_measure(pe_fd: c_int, filename: *const c_char, bm_pid: pid_t) -> c_int;
    pub fn measure_llc_resctrl(filename: *const c_char, bm_pid: pid_t) -> c_int;
    pub fn minimize_l2_occupancy(
        test: *const resctrl_test,
        uparams: *const user_params,
        param: *const resctrl_val_param,
    ) -> c_int;
    pub fn show_cache_info(
        no_of_bits: c_int,
        avg_llc_val: __u64,
        cache_span: size_t,
        lines: bool,
    );

    pub static mut mbm_test: resctrl_test;
    pub static mut mba_test: resctrl_test;
    pub static mut cmt_test: resctrl_test;
    pub static mut l3_cat_test: resctrl_test;
    pub static mut l3_noncont_cat_test: resctrl_test;
    pub static mut l2_noncont_cat_test: resctrl_test;
}

/*
 * cache_portion_size - Calculate the size of a cache portion
 * @cache_size:		Total cache size in bytes
 * @portion_mask:	Cache portion mask
 * @full_cache_mask:	Full Cache Bit Mask (CBM) for the cache
 *
 * Return: The size of the cache portion in bytes.
 */
pub unsafe fn cache_portion_size(
    cache_size: c_ulong,
    portion_mask: c_ulong,
    full_cache_mask: c_ulong,
) -> c_ulong {
    let bits: c_uint = unsafe { count_bits(full_cache_mask) };

    /*
     * With no bits the full CBM, assume cache cannot be split into
     * smaller portions. To avoid divide by zero, return cache_size.
     */
    if bits == 0 {
        return cache_size;
    }

    cache_size
        .wrapping_mul(unsafe { count_bits(portion_mask) } as c_ulong)
        .wrapping_div(bits as c_ulong)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
