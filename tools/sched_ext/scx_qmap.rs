/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2022 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2022 David Vernet <dvernet@meta.com>
 */

/*
 * Translated from sched_ext/scx_qmap.c. C include dependencies are expected to
 * provide the libbpf, sched_ext, skeleton, and qmap arena definitions.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type bool_ = bool;
type size_t = usize;
type time_t = c_long;
type va_list = *mut c_void;
type u32 = u32;
type u64 = u64;
type s32 = i32;

const LIBBPF_DEBUG: libbpf_print_level = 0;
const SCX_OPS_SWITCH_PARTIAL: u64 = 1 << 0;
const SCX_OPS_ALWAYS_ENQ_IMMED: u64 = 1 << 1;
const SCX_QMAP_MAX_CPUS: c_int = 4096;
const MAX_SUB_SCHEDS: usize = 32;
const CID_SHARED: s32 = -1;
const CID_SELF: s32 = -2;
const QMAP_CID_OVR_SHUFFLE: u32 = 1;
const QMAP_CID_OVR_BAD_DUP: u32 = 2;
const QMAP_CID_OVR_BAD_RANGE: u32 = 3;
const QMAP_CID_OVR_BAD_MONO: u32 = 4;
const QMAP_INJ_WRONG_CID: u32 = 1;
const QMAP_INJ_INIT_FAIL: u32 = 2;
const QMAP_INJ_CGRP_INIT_FAIL: u32 = 3;

type libbpf_print_level = c_int;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    pub st_ino: u64,
}

#[repr(C)]
pub struct qmap_part {
    pub cid_owner: [s32; SCX_QMAP_MAX_CPUS as usize],
    pub nr_excl: u32,
    pub nr_shared: u32,
    pub nr_rr: u32,
    pub rr_slots: [u64; MAX_SUB_SCHEDS + 1],
    pub rr_pos: u32,
}

#[repr(C)]
pub struct sub_sched_ctx {
    pub cgroup_id: u64,
    pub weight: u32,
    pub nr_dsps: u64,
}

#[repr(C)]
pub struct qmap_arena {
    pub nr_cids: u32,
    pub part: qmap_part,
    pub alloc_ns: [u64; MAX_SUB_SCHEDS],
    pub self_alloc_ns: u64,
    pub alloc_window_ns: u64,
    pub nr_reenq_cap: u64,
    pub nr_reenq_immed: u64,
    pub nr_inject_attempts: u64,
    pub nr_rescue_dsp: u64,
    pub nr_sub_scheds: u64,
    pub sub_sched_ctxs: [sub_sched_ctx; MAX_SUB_SCHEDS],
    pub cid_override_cpu_to_cid: [s32; SCX_QMAP_MAX_CPUS as usize],
    pub cid_override_shard_start: [s32; SCX_QMAP_MAX_CPUS as usize],
    pub test_error_cnt: u32,
    pub inject_mode: u32,
    pub nr_enqueued: c_long,
    pub nr_dispatched: c_long,
    pub nr_reenqueued: u64,
    pub nr_reenqueued_cid0: u64,
    pub nr_dequeued: u64,
    pub nr_core_sched_execed: u64,
    pub nr_ddsp_from_enq: u64,
    pub nr_expedited_local: u64,
    pub nr_expedited_remote: u64,
    pub nr_expedited_from_timer: u64,
    pub nr_expedited_lost: u64,
    pub cpuperf_min: u32,
    pub cpuperf_avg: u32,
    pub cpuperf_max: u32,
    pub cpuperf_target_min: u32,
    pub cpuperf_target_avg: u32,
    pub cpuperf_target_max: u32,
}

#[repr(C)]
pub struct qmap_rodata {
    pub slice_ns: u64,
    pub max_tasks: u32,
    pub stall_user_nth: u32,
    pub stall_kernel_nth: u32,
    pub dsp_inf_loop_after: u32,
    pub dsp_batch: u32,
    pub print_dsqs_and_events: bool_,
    pub print_msgs: bool_,
    pub highpri_boosting: bool_,
    pub sub_cgroup_id: u64,
    pub disallow_tgid: c_long,
    pub suppress_dump: bool_,
    pub immed_stress_nth: u32,
    pub cid_override_mode: u32,
    pub cid_override_nr_shards: u32,
    pub round_robin_ns: u64,
}

#[repr(C)]
pub struct qmap_ops {
    pub sub_cgroup_id: u64,
    pub exit_dump_len: u32,
    pub flags: u64,
    pub rescue_bandwidth_ppt: u32,
    pub rescue_quantum_us: u32,
}

#[repr(C)]
pub struct scx_qmap_struct_ops {
    pub qmap_ops: *mut qmap_ops,
}

#[repr(C)]
pub struct scx_qmap_progs {
    pub flush_alloc: *mut bpf_program,
}

#[repr(C)]
pub struct scx_qmap_arena_wrapper {
    pub qa: qmap_arena,
}

#[repr(C)]
pub struct scx_qmap {
    pub rodata: *mut qmap_rodata,
    pub struct_ops: scx_qmap_struct_ops,
    pub progs: scx_qmap_progs,
    pub arena: *mut scx_qmap_arena_wrapper,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;
    static mut stdout: *mut c_void;
    static mut optarg: *mut c_char;
    static mut optind: c_int;

    fn vfprintf(stream: *mut c_void, format: *const c_char, args: va_list) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn perror(s: *const c_char);
    fn basename(path: *mut c_char) -> *mut c_char;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn atoi(nptr: *const c_char) -> c_int;
    fn getpid() -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> usize;
    fn sleep(seconds: c_uint) -> c_uint;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn localtime(timep: *const time_t) -> *mut c_void;
    fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const c_void) -> size_t;

    fn libbpf_set_print(print_fn: extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int);
    fn libbpf_num_possible_cpus() -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn __COMPAT_ENUM_OR_ZERO(enum_name: *const c_char, value_name: *const c_char) -> u64;
    fn __COMPAT_has_ksym(name: *const c_char) -> bool_;
    fn SCX_OPS_CID_OPEN(ops_name: *const c_char, skel_name: *const c_char) -> *mut scx_qmap;
    fn SCX_OPS_LOAD(skel: *mut scx_qmap, ops_name: *const c_char, skel_name: *const c_char, uei_name: *const c_char);
    fn SCX_OPS_ATTACH(skel: *mut scx_qmap, ops_name: *const c_char, skel_name: *const c_char) -> *mut bpf_link;
    fn UEI_EXITED(skel: *mut scx_qmap, uei_name: *const c_char) -> bool_;
    fn UEI_REPORT(skel: *mut scx_qmap, uei_name: *const c_char) -> u64;
    fn UEI_ECODE_RESTART(ecode: u64) -> bool_;
    fn scx_qmap__destroy(skel: *mut scx_qmap);
}

static HELP_FMT: &[u8] = b"A simple five-level FIFO queue sched_ext scheduler.\n\
\n\
It also demonstrates hierarchical sub-scheduling: a scheduler can hand some\n\
of its cpus to a child cgroup that runs its own scheduler. Run one qmap as\n\
the parent, then run another qmap on a child cgroup with -c to attach it\n\
beneath the parent.\n\
\n\
The policy below is deliberately simplistic and the resulting behavior can\n\
look odd. qmap is a demo: it exists to exercise every sub-scheduling\n\
primitive the kernel offers with as little code as possible, not to schedule\n\
well.\n\
\n\
A parent divides the full cpus it holds among itself and its children in\n\
proportion to cpu.weight. The cpus left over by rounding are time-shared,\n\
handed to each participant in turn every -R ms. A cpu a scheduler only\n\
holds a time-share of is never handed further down, and a parent left with\n\
no full cpu of its own shuts its children down.\n\
\n\
See the top-of-file comment in .bpf.c for the design.\n\
\n\
Usage: %s [-s SLICE_US] [-e COUNT] [-t COUNT] [-T COUNT] [-l COUNT] [-b COUNT]\n\
       [-N COUNT] [-P] [-M] [-H] [-c CG_PATH] [-d PID] [-D LEN] [-S] [-p] [-I]\n\
       [-F COUNT] [-i SEC] [-R MS] [-J MODE] [-v]\n\
\n\
  -s SLICE_US   Override slice duration\n\
  -e COUNT      Trigger scx_bpf_error() after COUNT enqueues\n\
  -t COUNT      Stall every COUNT'th user thread\n\
  -T COUNT      Stall every COUNT'th kernel thread\n\
  -N COUNT      Size of the task_ctx arena slab (default 16384)\n\
  -l COUNT      Trigger dispatch infinite looping after COUNT dispatches\n\
  -b COUNT      Dispatch upto COUNT tasks together\n\
  -P            Print out DSQ content and event counters to trace_pipe every second\n\
  -M            Print out debug messages to trace_pipe\n\
  -H            Boost nice -20 tasks in SHARED_DSQ, use with -b\n\
  -c CG_PATH    Cgroup path to attach as sub-scheduler, must run parent scheduler first\n\
  -d PID        Disallow a process from switching into SCHED_EXT (-1 for self)\n\
  -D LEN        Set scx_exit_info.dump buffer length\n\
  -S            Suppress qmap-specific debug dump\n\
  -p            Switch only tasks on SCHED_EXT policy instead of all\n\
  -I            Turn on SCX_OPS_ALWAYS_ENQ_IMMED\n\
  -F COUNT      IMMED stress: force every COUNT'th enqueue to a busy local DSQ (use with -I)\n\
  -C MODE       cid-override test (shuffle|bad-dup|bad-range|bad-mono)\n\
  -i SEC        Stats interval, seconds (default 5)\n\
  -R MS         Round-robin period for time-shared cpus, ms (default 200)\n\
  -J MODE       Fault injection (wrong-cid: dispatch to a cid not held,\n\
                init-fail/cgrp-init-fail: fail init_task/cpuctl_init for\n\
                \"qmfail*\" comms/cgroups)\n\
  -B PPT        Rescue bandwidth in parts per thousand, 0 disables (root only, default 20)\n\
  -q US         Rescue batch quantum in microseconds (root only, default 5000)\n\
  -v            Print libbpf debug messages\n\
  -h            Display this help and exit\n\0";

static mut VERBOSE: bool_ = false;
static mut EXIT_REQ: c_int = 0;

extern "C" fn libbpf_print_fn(level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int {
    unsafe {
        if level == LIBBPF_DEBUG && !VERBOSE {
            return 0;
        }
        vfprintf(stderr, format, args)
    }
}

extern "C" fn sigint_handler(_dummy: c_int) {
    unsafe {
        EXIT_REQ = 1;
    }
}

unsafe fn invoke_flush_alloc(skel: *mut scx_qmap) {
    let mut opts: bpf_test_run_opts = core::mem::zeroed();

    bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.flush_alloc), &mut opts);
}

/* previous counter snapshots for the per-interval hier stats */
#[repr(C)]
struct hier_prev {
    alloc_ns: [u64; MAX_SUB_SCHEDS],
    self_alloc_ns: u64,
    alloc_window_ns: u64,
    nr_dsps: [u64; MAX_SUB_SCHEDS],
    nr_reenq_cap: u64,
    nr_reenq_immed: u64,
    nr_inject_attempts: u64,
    nr_rescue_dsp: u64,
}

/* current wall-clock time as "HH:MM:SS" for the startup and interval headers */
unsafe fn tstamp(buf: *mut c_char, sz: size_t) -> *const c_char {
    let now: time_t = time(core::ptr::null_mut());

    strftime(buf, sz, c"%H:%M:%S".as_ptr(), localtime(&now));
    buf
}

/* format the cids whose cid_owner[] matches @owner as "0-3,8", "-" if none */
unsafe fn format_cid_ranges(qa: *mut qmap_arena, owner: s32, buf: *mut c_char, sz: size_t) {
    let nr: u32 = (*qa).nr_cids;
    let mut cid: u32;
    let mut off: size_t = 0;
    let mut start: s32 = -1;

    *buf = b'\0' as c_char;
    cid = 0;
    while cid <= nr {
        let match_: bool_ = cid < nr && (*qa).part.cid_owner[cid as usize] == owner;
        let n: c_int;

        if match_ {
            if start < 0 {
                start = cid as s32;
            }
            cid += 1;
            continue;
        }
        if start < 0 {
            cid += 1;
            continue;
        }

        if start == cid as s32 - 1 {
            n = snprintf(
                buf.add(off),
                sz - off,
                c"%s%d".as_ptr(),
                if off != 0 { c",".as_ptr() } else { c"".as_ptr() },
                start,
            );
        } else {
            n = snprintf(
                buf.add(off),
                sz - off,
                c"%s%d-%d".as_ptr(),
                if off != 0 { c",".as_ptr() } else { c"".as_ptr() },
                start,
                cid - 1,
            );
        }
        if n < 0 || n as size_t >= sz - off {
            strcpy(buf.add(sz - 4), c"...".as_ptr());
            return;
        }
        off += n as size_t;
        start = -1;
        cid += 1;
    }
    if off == 0 {
        strcpy(buf, c"-".as_ptr());
    }
}

/* partition summary + one row per sched: weight, cpus, dispatch rate, cids */
unsafe fn print_hier(qa: *mut qmap_arena, prev: *mut hier_prev, own_cgid: u64) {
    let mut ranges: [c_char; 128] = [0; 128];
    let mut who: [c_char; 16] = [0; 16];
    let mut rr: *const c_char = c"-".as_ptr();
    let secs: f64;
    let mut i: u32;

    /*
     * account_alloc() bumps alloc_window_ns together with the per-owner
     * counters, so dividing by the same window yields exact cid counts.
     */
    secs = ((*qa).alloc_window_ns - (*prev).alloc_window_ns) as f64 / 1e9f64;
    (*prev).alloc_window_ns = (*qa).alloc_window_ns;

    /* resolve the live shared-pool holder */
    if (*qa).part.nr_shared != 0 && (*qa).part.nr_rr != 0 {
        let cgid: u64 = (*qa).part.rr_slots[(*qa).part.rr_pos as usize];

        rr = c"self".as_ptr();
        if cgid != 0 {
            rr = c"?".as_ptr();
            i = 0;
            while (i as usize) < MAX_SUB_SCHEDS {
                if (*qa).sub_sched_ctxs[i as usize].cgroup_id == cgid {
                    snprintf(who.as_mut_ptr(), who.len(), c"sub%u".as_ptr(), i);
                    rr = who.as_ptr();
                    break;
                }
                i += 1;
            }
        }
    }

    format_cid_ranges(qa, CID_SHARED, ranges.as_mut_ptr(), ranges.len());
    printf(
        c"hier   : nsub=%llu excl=%u shared=%s rr=%s reenq cap/immed +%llu/+%llu inj=+%llu rescue=+%llu\n".as_ptr(),
        (*qa).nr_sub_scheds as c_ulong,
        (*qa).part.nr_excl,
        ranges.as_ptr(),
        rr,
        ((*qa).nr_reenq_cap - (*prev).nr_reenq_cap) as c_ulong,
        ((*qa).nr_reenq_immed - (*prev).nr_reenq_immed) as c_ulong,
        ((*qa).nr_inject_attempts - (*prev).nr_inject_attempts) as c_ulong,
        ((*qa).nr_rescue_dsp - (*prev).nr_rescue_dsp) as c_ulong,
    );
    (*prev).nr_reenq_cap = (*qa).nr_reenq_cap;
    (*prev).nr_reenq_immed = (*qa).nr_reenq_immed;
    (*prev).nr_inject_attempts = (*qa).nr_inject_attempts;
    (*prev).nr_rescue_dsp = (*qa).nr_rescue_dsp;

    printf(
        c"hier   : %-4s %10s %4s %6s %8s  %s\n".as_ptr(),
        c"".as_ptr(),
        c"cgroup".as_ptr(),
        c"w".as_ptr(),
        c"alloc".as_ptr(),
        c"disp/s".as_ptr(),
        c"cids".as_ptr(),
    );

    format_cid_ranges(qa, CID_SELF, ranges.as_mut_ptr(), ranges.len());
    printf(
        c"hier   : %-4s %10llu %4u %6.2f %8s  %s\n".as_ptr(),
        c"self".as_ptr(),
        own_cgid as c_ulong,
        100u32,
        if secs > 0.0 {
            ((*qa).self_alloc_ns - (*prev).self_alloc_ns) as f64 / (secs * 1e9f64)
        } else {
            0.0
        },
        c"-".as_ptr(),
        ranges.as_ptr(),
    );
    (*prev).self_alloc_ns = (*qa).self_alloc_ns;

    i = 0;
    while (i as usize) < MAX_SUB_SCHEDS {
        let sc: *mut sub_sched_ctx = &mut (*qa).sub_sched_ctxs[i as usize];

        if (*sc).cgroup_id == 0 {
            i += 1;
            continue;
        }

        snprintf(who.as_mut_ptr(), who.len(), c"sub%u".as_ptr(), i);
        format_cid_ranges(qa, i as s32, ranges.as_mut_ptr(), ranges.len());
        printf(
            c"hier   : %-4s %10llu %4u %6.2f %8.1f  %s\n".as_ptr(),
            who.as_ptr(),
            (*sc).cgroup_id as c_ulong,
            (*sc).weight,
            if secs > 0.0 {
                ((*qa).alloc_ns[i as usize] - (*prev).alloc_ns[i as usize]) as f64 / (secs * 1e9f64)
            } else {
                0.0
            },
            if secs > 0.0 {
                ((*sc).nr_dsps - (*prev).nr_dsps[i as usize]) as f64 / secs
            } else {
                0.0
            },
            ranges.as_ptr(),
        );
        (*prev).alloc_ns[i as usize] = (*qa).alloc_ns[i as usize];
        (*prev).nr_dsps[i as usize] = (*sc).nr_dsps;
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut skel: *mut scx_qmap;
    let mut link: *mut bpf_link;
    let mut qa: *mut qmap_arena;
    let mut test_error_cnt: u32 = 0;
    let mut ecode: u64;
    let mut opt: c_int;
    let mut stats_intv: c_int = 5;
    let mut i: c_int;
    let mut round_robin_ms: c_int = 200;
    let mut hprev: hier_prev = core::mem::zeroed();
    let mut sub_cg_path: *const c_char = core::ptr::null();
    let mut tbuf: [c_char; 32] = [0; 32];
    let mut inject_mode: u32 = 0;
    let mut own_cgid: u64 = 0;
    let mut cid_override_shard_sz: s32 = 4;

    libbpf_set_print(libbpf_print_fn);
    signal(2, sigint_handler);
    signal(15, sigint_handler);

    if libbpf_num_possible_cpus() > SCX_QMAP_MAX_CPUS {
        fprintf(
            stderr,
            c"scx_qmap: %d possible CPUs exceeds compile-time cap %d; rebuild with larger SCX_QMAP_MAX_CPUS\n".as_ptr(),
            libbpf_num_possible_cpus(),
            SCX_QMAP_MAX_CPUS,
        );
        return 1;
    }

    'restart: loop {
        optind = 1;
        skel = SCX_OPS_CID_OPEN(c"qmap_ops".as_ptr(), c"scx_qmap".as_ptr());

        (*(*skel).rodata).slice_ns = __COMPAT_ENUM_OR_ZERO(c"scx_public_consts".as_ptr(), c"SCX_SLICE_DFL".as_ptr());
        (*(*skel).rodata).max_tasks = 16384;

        loop {
            opt = getopt(argc, argv, c"s:e:t:T:l:b:N:PMHc:d:D:SpIF:C:i:R:J:B:q:vh".as_ptr());
            if opt == -1 {
                break;
            }
            match opt as u8 as char {
                's' => {
                    (*(*skel).rodata).slice_ns = strtoull(optarg, core::ptr::null_mut(), 0) * 1000;
                }
                'e' => {
                    test_error_cnt = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                't' => {
                    (*(*skel).rodata).stall_user_nth = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'T' => {
                    (*(*skel).rodata).stall_kernel_nth = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'l' => {
                    (*(*skel).rodata).dsp_inf_loop_after = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'b' => {
                    (*(*skel).rodata).dsp_batch = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'N' => {
                    (*(*skel).rodata).max_tasks = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'P' => {
                    (*(*skel).rodata).print_dsqs_and_events = true;
                }
                'M' => {
                    (*(*skel).rodata).print_msgs = true;
                }
                'H' => {
                    (*(*skel).rodata).highpri_boosting = true;
                }
                'c' => {
                    let mut st: stat = core::mem::zeroed();
                    if stat(optarg, &mut st) < 0 {
                        perror(c"stat".as_ptr());
                        return 1;
                    }
                    (*(*skel).struct_ops.qmap_ops).sub_cgroup_id = st.st_ino;
                    (*(*skel).rodata).sub_cgroup_id = st.st_ino;
                    own_cgid = st.st_ino;
                    sub_cg_path = optarg;
                }
                'd' => {
                    (*(*skel).rodata).disallow_tgid = strtol(optarg, core::ptr::null_mut(), 0);
                    if (*(*skel).rodata).disallow_tgid < 0 {
                        (*(*skel).rodata).disallow_tgid = getpid() as c_long;
                    }
                }
                'D' => {
                    (*(*skel).struct_ops.qmap_ops).exit_dump_len = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'S' => {
                    (*(*skel).rodata).suppress_dump = true;
                }
                'p' => {
                    (*(*skel).struct_ops.qmap_ops).flags |= SCX_OPS_SWITCH_PARTIAL;
                }
                'I' => {
                    (*(*skel).struct_ops.qmap_ops).flags |= SCX_OPS_ALWAYS_ENQ_IMMED;
                }
                'F' => {
                    (*(*skel).rodata).immed_stress_nth = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'C' => {
                    let nr_cpus: u32 = libbpf_num_possible_cpus() as u32;
                    let mode: u32;

                    if strcmp(optarg, c"shuffle".as_ptr()) == 0 {
                        mode = QMAP_CID_OVR_SHUFFLE;
                    } else if strcmp(optarg, c"bad-dup".as_ptr()) == 0 {
                        mode = QMAP_CID_OVR_BAD_DUP;
                    } else if strcmp(optarg, c"bad-range".as_ptr()) == 0 {
                        mode = QMAP_CID_OVR_BAD_RANGE;
                    } else if strcmp(optarg, c"bad-mono".as_ptr()) == 0 {
                        mode = QMAP_CID_OVR_BAD_MONO;
                    } else {
                        fprintf(stderr, c"unknown cid-override mode '%s'\n".as_ptr(), optarg);
                        return 1;
                    }
                    (*(*skel).rodata).cid_override_mode = mode;
                    cid_override_shard_sz = 4;

                    /*
                     * bad-mono needs >= 3 shards to build a 0-based but
                     * non-monotonic shard_start. Shrink the shard size so
                     * the test runs on any machine with >= 3 cpus.
                     */
                    if mode == QMAP_CID_OVR_BAD_MONO {
                        if nr_cpus < 3 {
                            fprintf(stderr, c"bad-mono needs >= 3 cpus (have %u)\n".as_ptr(), nr_cpus);
                            return 1;
                        }
                        cid_override_shard_sz = (nr_cpus / 3) as s32;
                    }

                    /* shards of shard_sz each */
                    (*(*skel).rodata).cid_override_nr_shards =
                        ((nr_cpus as s32 + cid_override_shard_sz - 1) / cid_override_shard_sz) as u32;
                }
                'i' => {
                    stats_intv = atoi(optarg);
                    if stats_intv < 1 {
                        stats_intv = 1;
                    }
                }
                'R' => {
                    round_robin_ms = atoi(optarg);
                    if round_robin_ms < 10 {
                        round_robin_ms = 10;
                    }
                }
                'J' => {
                    if strcmp(optarg, c"wrong-cid".as_ptr()) == 0 {
                        inject_mode = QMAP_INJ_WRONG_CID;
                    } else if strcmp(optarg, c"init-fail".as_ptr()) == 0 {
                        inject_mode = QMAP_INJ_INIT_FAIL;
                    } else if strcmp(optarg, c"cgrp-init-fail".as_ptr()) == 0 {
                        inject_mode = QMAP_INJ_CGRP_INIT_FAIL;
                    } else {
                        inject_mode = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                    }
                }
                'B' => {
                    let mut ppt: u32 = strtoul(optarg, core::ptr::null_mut(), 0) as u32;

                    if ppt == 0 {
                        ppt = __COMPAT_ENUM_OR_ZERO(c"scx_consts".as_ptr(), c"SCX_RESCUE_DISABLE".as_ptr()) as u32;
                    }
                    (*(*skel).struct_ops.qmap_ops).rescue_bandwidth_ppt = ppt;
                }
                'q' => {
                    (*(*skel).struct_ops.qmap_ops).rescue_quantum_us = strtoul(optarg, core::ptr::null_mut(), 0) as u32;
                }
                'v' => {
                    VERBOSE = true;
                }
                _ => {
                    fprintf(stderr, HELP_FMT.as_ptr() as *const c_char, basename(*argv));
                    return if opt != 'h' as c_int { 1 } else { 0 };
                }
            }
        }

        (*(*skel).rodata).round_robin_ns = round_robin_ms as u64 * 1000000;

        SCX_OPS_LOAD(skel, c"qmap_ops".as_ptr(), c"scx_qmap".as_ptr(), c"uei".as_ptr());

        qa = &mut (*(*skel).arena).qa;

        /*
         * The cid-override arrays live in the arena, which is mmapped at load.
         * Populate them before qmap_init_cids() consumes them at attach.
         */
        if (*(*skel).rodata).cid_override_mode != 0 {
            let mode: u32 = (*(*skel).rodata).cid_override_mode;
            let nr_cpus: u32 = libbpf_num_possible_cpus() as u32;
            let mut j: u32;

            /* shuffle: reversed cpu_to_cid; others: identity */
            j = 0;
            while j < nr_cpus {
                if mode == QMAP_CID_OVR_SHUFFLE {
                    (*qa).cid_override_cpu_to_cid[j as usize] = (nr_cpus - 1 - j) as s32;
                } else {
                    (*qa).cid_override_cpu_to_cid[j as usize] = j as s32;
                }
                j += 1;
            }
            if mode == QMAP_CID_OVR_BAD_DUP && nr_cpus >= 2 {
                (*qa).cid_override_cpu_to_cid[1] = 0;
            }
            if mode == QMAP_CID_OVR_BAD_RANGE {
                (*qa).cid_override_cpu_to_cid[0] = nr_cpus as s32;
            }

            j = 0;
            while j < (*(*skel).rodata).cid_override_nr_shards {
                (*qa).cid_override_shard_start[j as usize] = (j as s32) * cid_override_shard_sz;
                j += 1;
            }

            if mode == QMAP_CID_OVR_BAD_MONO {
                /* swap [1] and [2] to break monotonicity */
                let tmp: s32 = (*qa).cid_override_shard_start[1];
                (*qa).cid_override_shard_start[1] = (*qa).cid_override_shard_start[2];
                (*qa).cid_override_shard_start[2] = tmp;
            }
        }

        link = SCX_OPS_ATTACH(skel, c"qmap_ops".as_ptr(), c"scx_qmap".as_ptr());

        (*qa).test_error_cnt = test_error_cnt;
        (*qa).inject_mode = inject_mode;

        if !sub_cg_path.is_null() {
            printf(
                c"%s scx_qmap started: sub-scheduler on %s, stats every %ds\n".as_ptr(),
                tstamp(tbuf.as_mut_ptr(), tbuf.len()),
                sub_cg_path,
                stats_intv,
            );
        } else {
            printf(
                c"%s scx_qmap started: root scheduler, stats every %ds\n".as_ptr(),
                tstamp(tbuf.as_mut_ptr(), tbuf.len()),
                stats_intv,
            );
        }
        fflush(stdout);

        while EXIT_REQ == 0 && !UEI_EXITED(skel, c"uei".as_ptr()) {
            let nr_enqueued: c_long = (*qa).nr_enqueued;
            let nr_dispatched: c_long = (*qa).nr_dispatched;

            printf(c"---- %s ----\n".as_ptr(), tstamp(tbuf.as_mut_ptr(), tbuf.len()));
            printf(
                c"stats  : enq=%lu dsp=%lu delta=%ld reenq/cid0=%llu/%llu deq=%llu core=%llu enq_ddsp=%llu\n".as_ptr(),
                nr_enqueued as c_ulong,
                nr_dispatched as c_ulong,
                nr_enqueued - nr_dispatched,
                (*qa).nr_reenqueued as c_ulong,
                (*qa).nr_reenqueued_cid0 as c_ulong,
                (*qa).nr_dequeued as c_ulong,
                (*qa).nr_core_sched_execed as c_ulong,
                (*qa).nr_ddsp_from_enq as c_ulong,
            );
            printf(
                c"         exp_local=%llu exp_remote=%llu exp_timer=%llu exp_lost=%llu\n".as_ptr(),
                (*qa).nr_expedited_local as c_ulong,
                (*qa).nr_expedited_remote as c_ulong,
                (*qa).nr_expedited_from_timer as c_ulong,
                (*qa).nr_expedited_lost as c_ulong,
            );
            if __COMPAT_has_ksym(c"scx_bpf_cidperf_cur".as_ptr()) {
                printf(
                    c"cpuperf: cur min/avg/max=%u/%u/%u target min/avg/max=%u/%u/%u\n".as_ptr(),
                    (*qa).cpuperf_min,
                    (*qa).cpuperf_avg,
                    (*qa).cpuperf_max,
                    (*qa).cpuperf_target_min,
                    (*qa).cpuperf_target_avg,
                    (*qa).cpuperf_target_max,
                );
            }

            invoke_flush_alloc(skel);
            print_hier(qa, &mut hprev, own_cgid);
            fflush(stdout);

            i = 0;
            while i < stats_intv && EXIT_REQ == 0 && !UEI_EXITED(skel, c"uei".as_ptr()) {
                sleep(1);
                i += 1;
            }
        }

        bpf_link__destroy(link);
        ecode = UEI_REPORT(skel, c"uei".as_ptr());
        scx_qmap__destroy(skel);

        if EXIT_REQ == 0 && UEI_ECODE_RESTART(ecode) {
            continue 'restart;
        }
        return 0;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
