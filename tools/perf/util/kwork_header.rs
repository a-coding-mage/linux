// Translated from perf/util/kwork.h.
// Original C dependencies: "perf.h", "util/tool.h", "util/time-utils.h",
// <stdlib.h>, <linux/bitmap.h>, <linux/list.h>, <linux/rbtree.h>,
// and <linux/types.h>.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn free(ptr: *mut c_void);
}

extern "C" {
    pub type perf_sample;
    pub type perf_session;
    pub type machine;
    pub type evsel_str_handler;
    pub type list_head;
    pub type rb_node;
    pub type rb_root_cached;
    pub type perf_tool;
    pub type perf_time_interval;
}

pub const fn bits_to_longs(nr: usize) -> usize {
    (nr + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kwork_class_type {
    KWORK_CLASS_IRQ,
    KWORK_CLASS_SOFTIRQ,
    KWORK_CLASS_WORKQUEUE,
    KWORK_CLASS_SCHED,
    KWORK_CLASS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kwork_report_type {
    KWORK_REPORT_RUNTIME,
    KWORK_REPORT_LATENCY,
    KWORK_REPORT_TIMEHIST,
    KWORK_REPORT_TOP,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kwork_trace_type {
    KWORK_TRACE_RAISE,
    KWORK_TRACE_ENTRY,
    KWORK_TRACE_EXIT,
    KWORK_TRACE_MAX,
}

/*
 * data structure:
 *
 *                 +==================+ +============+ +======================+
 *                 |      class       | |    work    | |         atom         |
 *                 +==================+ +============+ +======================+
 * +------------+  |  +-----+         | |  +------+  | |  +-------+   +-----+ |
 * | perf_kwork | +-> | irq | --------|+-> | eth0 | --+-> | raise | - | ... | --+   +-----------+
 * +-----+------+ ||  +-----+         |||  +------+  |||  +-------+   +-----+ | |   |           |
 *       |        ||                  |||            |||                      | +-> | atom_page |
 *       |        ||                  |||            |||  +-------+   +-----+ |     |           |
 *       |  class_list                |||            |+-> | entry | - | ... | ----> |           |
 *       |        ||                  |||            |||  +-------+   +-----+ |     |           |
 *       |        ||                  |||            |||                      | +-> |           |
 *       |        ||                  |||            |||  +-------+   +-----+ | |   |           |
 *       |        ||                  |||            |+-> | exit  | - | ... | --+   +-----+-----+
 *       |        ||                  |||            | |  +-------+   +-----+ |           |
 *       |        ||                  |||            | |                      |           |
 *       |        ||                  |||  +-----+   | |                      |           |
 *       |        ||                  |+-> | ... |   | |                      |           |
 *       |        ||                  | |  +-----+   | |                      |           |
 *       |        ||                  | |            | |                      |           |
 *       |        ||  +---------+     | |  +-----+   | |  +-------+   +-----+ |           |
 *       |        +-> | softirq | -------> | RCU | ---+-> | raise | - | ... | --+   +-----+-----+
 *       |        ||  +---------+     | |  +-----+   |||  +-------+   +-----+ | |   |           |
 *       |        ||                  | |            |||                      | +-> | atom_page |
 *       |        ||                  | |            |||  +-------+   +-----+ |     |           |
 *       |        ||                  | |            |+-> | entry | - | ... | ----> |           |
 *       |        ||                  | |            |||  +-------+   +-----+ |     |           |
 *       |        ||                  | |            |||                      | +-> |           |
 *       |        ||                  | |            |||  +-------+   +-----+ | |   |           |
 *       |        ||                  | |            |+-> | exit  | - | ... | --+   +-----+-----+
 *       |        ||                  | |            | |  +-------+   +-----+ |           |
 *       |        ||                  | |            | |                      |           |
 *       |        ||  +-----------+   | |  +-----+   | |                      |           |
 *       |        +-> | workqueue | -----> | ... |   | |                      |           |
 *       |         |  +-----------+   | |  +-----+   | |                      |           |
 *       |         +==================+ +============+ +======================+           |
 *       |                                                                                |
 *       +---->  atom_page_list  ---------------------------------------------------------+
 *
 */

#[repr(C)]
pub struct kwork_atom {
    pub list: list_head,
    pub time: u64,
    pub prev: *mut kwork_atom,

    pub page_addr: *mut c_void,
    pub bit_inpage: c_ulong,
}

pub const NR_ATOM_PER_PAGE: usize = 128;

#[repr(C)]
pub struct kwork_atom_page {
    pub list: list_head,
    pub atoms: [kwork_atom; NR_ATOM_PER_PAGE],
    pub bitmap: [c_ulong; bits_to_longs(NR_ATOM_PER_PAGE)],
}

#[repr(C)]
pub struct kwork_work {
    /*
     * class field
     */
    pub node: rb_node,
    pub class: *mut kwork_class,

    /*
     * work field
     */
    pub id: u64,
    pub cpu: c_int,
    pub name: *mut c_char,

    /*
     * atom field
     */
    pub nr_atoms: u64,
    pub atom_list: [list_head; kwork_trace_type::KWORK_TRACE_MAX as usize],

    /*
     * runtime report
     */
    pub max_runtime: u64,
    pub max_runtime_start: u64,
    pub max_runtime_end: u64,
    pub total_runtime: u64,

    /*
     * latency report
     */
    pub max_latency: u64,
    pub max_latency_start: u64,
    pub max_latency_end: u64,
    pub total_latency: u64,

    /*
     * top report
     */
    pub cpu_usage: u32,
    pub tgid: u32,
    pub is_kthread: bool,
}

#[repr(C)]
pub struct kwork_class {
    pub list: list_head,
    pub name: *const c_char,
    pub type_: kwork_class_type,

    pub nr_tracepoints: c_uint,
    pub tp_handlers: *const evsel_str_handler,

    pub work_root: rb_root_cached,

    pub class_init: Option<
        unsafe extern "C" fn(class: *mut kwork_class, session: *mut perf_session) -> c_int,
    >,

    pub work_init: Option<
        unsafe extern "C" fn(
            kwork: *mut perf_kwork,
            class: *mut kwork_class,
            work: *mut kwork_work,
            src_type: kwork_trace_type,
            sample: *mut perf_sample,
            machine: *mut machine,
        ),
    >,

    pub work_name:
        Option<unsafe extern "C" fn(work: *mut kwork_work, buf: *mut c_char, len: c_int)>,
}

pub unsafe fn work_exit(work: *mut kwork_work) {
    if !work.is_null() {
        unsafe {
            free((*work).name as *mut c_void);
            (*work).name = core::ptr::null_mut();
        }
    }
}

#[repr(C)]
pub struct trace_kwork_handler {
    pub raise_event: Option<
        unsafe extern "C" fn(
            kwork: *mut perf_kwork,
            class: *mut kwork_class,
            sample: *mut perf_sample,
            machine: *mut machine,
        ) -> c_int,
    >,

    pub entry_event: Option<
        unsafe extern "C" fn(
            kwork: *mut perf_kwork,
            class: *mut kwork_class,
            sample: *mut perf_sample,
            machine: *mut machine,
        ) -> c_int,
    >,

    pub exit_event: Option<
        unsafe extern "C" fn(
            kwork: *mut perf_kwork,
            class: *mut kwork_class,
            sample: *mut perf_sample,
            machine: *mut machine,
        ) -> c_int,
    >,

    pub sched_switch_event: Option<
        unsafe extern "C" fn(
            kwork: *mut perf_kwork,
            class: *mut kwork_class,
            sample: *mut perf_sample,
            machine: *mut machine,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct __top_cpus_runtime {
    pub load: u64,
    pub idle: u64,
    pub irq: u64,
    pub softirq: u64,
    pub total: u64,
}

#[repr(C)]
pub struct kwork_top_stat {
    pub all_cpus_bitmap: [c_ulong; bits_to_longs(crate::MAX_NR_CPUS as usize)],
    pub cpus_runtime: *mut __top_cpus_runtime,
    pub nr_skipped_cpu: c_uint,
}

#[repr(C)]
pub struct perf_kwork {
    /*
     * metadata
     */
    pub tool: perf_tool,
    pub class_list: list_head,
    pub atom_page_list: list_head,
    pub sort_list: list_head,
    pub cmp_id: list_head,
    pub sorted_work_root: rb_root_cached,
    pub tp_handler: *const trace_kwork_handler,

    /*
     * profile filters
     */
    pub profile_name: *const c_char,

    pub cpu_list: *const c_char,
    pub cpu_bitmap: [c_ulong; bits_to_longs(crate::MAX_NR_CPUS as usize)],

    pub time_str: *const c_char,
    pub ptime: perf_time_interval,

    /*
     * options for command
     */
    pub force: bool,
    pub event_list_str: *const c_char,
    pub report: kwork_report_type,

    /*
     * options for subcommand
     */
    pub summary: bool,
    pub sort_order: *const c_char,
    pub show_callchain: bool,
    pub max_stack: c_uint,
    pub use_bpf: bool,

    /*
     * statistics
     */
    pub timestart: u64,
    pub timeend: u64,

    pub nr_events: c_ulong,
    pub nr_lost_chunks: c_ulong,
    pub nr_lost_events: c_ulong,

    pub all_runtime: u64,
    pub all_count: u64,
    pub nr_skipped_events: [u64; kwork_trace_type::KWORK_TRACE_MAX as usize + 1],

    /*
     * perf kwork top data
     */
    pub top_stat: kwork_top_stat,

    /* Add work callback. */
    pub add_work: Option<
        unsafe extern "C" fn(
            kwork: *mut perf_kwork,
            class: *mut kwork_class,
            key: *mut kwork_work,
        ) -> *mut kwork_work,
    >,
}

// C conditional: #ifdef HAVE_BPF_SKEL
#[cfg(feature = "have_bpf_skel")]
extern "C" {
    pub fn perf_kwork__trace_prepare_bpf(kwork: *mut perf_kwork) -> c_int;
    pub fn perf_kwork__report_read_bpf(kwork: *mut perf_kwork) -> c_int;
    pub fn perf_kwork__report_cleanup_bpf();

    pub fn perf_kwork__trace_start();
    pub fn perf_kwork__trace_finish();

    pub fn perf_kwork__top_prepare_bpf(kwork: *mut perf_kwork) -> c_int;
    pub fn perf_kwork__top_read_bpf(kwork: *mut perf_kwork) -> c_int;
    pub fn perf_kwork__top_cleanup_bpf();

    pub fn perf_kwork__top_start();
    pub fn perf_kwork__top_finish();
}

// C conditional: #else  /* !HAVE_BPF_SKEL */
#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__trace_prepare_bpf(_kwork: *mut perf_kwork) -> c_int {
    -1
}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__report_read_bpf(_kwork: *mut perf_kwork) -> c_int {
    -1
}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__report_cleanup_bpf() {}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__trace_start() {}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__trace_finish() {}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__top_prepare_bpf(_kwork: *mut perf_kwork) -> c_int {
    -1
}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__top_read_bpf(_kwork: *mut perf_kwork) -> c_int {
    -1
}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__top_cleanup_bpf() {}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__top_start() {}

#[cfg(not(feature = "have_bpf_skel"))]
pub unsafe fn perf_kwork__top_finish() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
