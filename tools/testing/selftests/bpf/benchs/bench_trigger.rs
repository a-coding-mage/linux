// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* C dependencies: argp.h, unistd.h, stdint.h, bpf_util.h, bench.h,
 * trigger_bench.skel.h, trace_helpers.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

type __u32 = u32;
type error_t = c_int;

const MAX_TRIG_BATCH_ITERS: c_long = 1000;
const ARG_TRIG_BATCH_ITERS: c_int = 7000;
const MAX_BUCKETS: usize = 256;
const ARGP_ERR_UNKNOWN: error_t = 7;
const STDERR_FILENO: c_int = 2;
const __NR_getpgid: c_long = 121;

#[repr(C)]
pub struct argp_option {
    pub name: *const c_char,
    pub key: c_int,
    pub arg: *const c_char,
    pub flags: c_int,
    pub doc: *const c_char,
    pub group: c_int,
}

#[repr(C)]
pub struct argp_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<unsafe extern "C" fn(c_int, *mut c_char, *mut argp_state) -> error_t>,
}

#[repr(C)]
pub struct counter {
    pub value: c_long,
}

#[repr(C)]
pub struct bench_res {
    pub hits: c_long,
}

#[repr(C)]
pub struct bench {
    pub name: *const c_char,
    pub validate: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn()>,
    pub producer_thread: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
    pub measure: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub report_progress: Option<unsafe extern "C" fn(*mut bench_res, *mut bench_res)>,
    pub report_final: Option<unsafe extern "C" fn(*mut bench_res)>,
    pub argp: *const argp,
}

#[repr(C)]
pub struct bench_env {
    pub consumer_cnt: c_int,
    pub stacktrace: bool,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trigger_bench_progs {
    pub trigger_driver: *mut bpf_program,
    pub trigger_kernel_count: *mut bpf_program,
    pub bench_trigger_kprobe: *mut bpf_program,
    pub bench_trigger_kretprobe: *mut bpf_program,
    pub bench_trigger_kprobe_multi: *mut bpf_program,
    pub bench_trigger_kretprobe_multi: *mut bpf_program,
    pub bench_trigger_fentry: *mut bpf_program,
    pub bench_kprobe_multi_empty: *mut bpf_program,
    pub bench_kretprobe_multi_empty: *mut bpf_program,
    pub bench_trigger_fexit: *mut bpf_program,
    pub trigger_driver_kfunc: *mut bpf_program,
    pub bench_trigger_fmodret: *mut bpf_program,
    pub bench_trigger_tp: *mut bpf_program,
    pub bench_trigger_rawtp: *mut bpf_program,
    pub bench_trigger_uprobe_multi: *mut bpf_program,
    pub bench_trigger_uprobe: *mut bpf_program,
    pub bench_trigger_usdt: *mut bpf_program,
}

#[repr(C)]
pub struct trigger_bench_bss {
    pub hits: [counter; MAX_BUCKETS],
}

#[repr(C)]
pub struct trigger_bench_rodata {
    pub batch_iters: __u32,
    pub stacktrace: bool,
}

#[repr(C)]
pub struct trigger_bench_links {
    pub bench_trigger_uprobe_multi: *mut bpf_link,
    pub bench_trigger_uprobe: *mut bpf_link,
    pub bench_trigger_usdt: *mut bpf_link,
}

#[repr(C)]
pub struct trigger_bench {
    pub progs: trigger_bench_progs,
    pub bss: *mut trigger_bench_bss,
    pub rodata: *mut trigger_bench_rodata,
    pub links: trigger_bench_links,
}

#[repr(C)]
pub struct bpf_kprobe_multi_opts {
    pub sz: usize,
    pub syms: *const *const c_char,
    pub cnt: usize,
    pub retprobe: bool,
}

#[repr(C)]
pub struct bpf_uprobe_multi_opts {
    pub sz: usize,
    pub retprobe: bool,
    pub cnt: usize,
    pub offsets: *mut usize,
}

#[repr(C)]
pub struct ksyms {
    pub filtered_syms: *mut *mut c_char,
    pub filtered_cnt: usize,
}

#[repr(C)]
struct trigger_args {
    batch_iters: __u32,
}

#[repr(C)]
struct trigger_ctx {
    skel: *mut trigger_bench,
    usermode_counters: bool,
    driver_prog_fd: c_int,
}

unsafe extern "C" {
    static mut env: bench_env;
    static mut stderr: *mut c_void;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn argp_usage(state: *mut argp_state);
    fn exit(status: c_int) -> !;
    fn syscall(num: c_long, ...) -> c_long;
    fn sys_gettid() -> c_int;
    fn setup_libbpf();
    fn trigger_bench__open() -> *mut trigger_bench;
    fn trigger_bench__load(skel: *mut trigger_bench) -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut c_void) -> c_int;
    fn bpf_get_ksyms(ksyms: *mut *mut ksyms, filter: bool) -> c_int;
    fn free_kallsyms_local(ksyms: *mut ksyms);
    fn bpf_program__attach_kprobe_multi_opts(
        prog: *mut bpf_program,
        pattern: *const c_char,
        opts: *const bpf_kprobe_multi_opts,
    ) -> *mut bpf_link;
    fn get_uprobe_offset(addr: *mut c_void) -> usize;
    fn bpf_program__attach_uprobe_multi(
        prog: *mut bpf_program,
        pid: c_int,
        binary_path: *const c_char,
        func_offset: *const c_char,
        opts: *const bpf_uprobe_multi_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_uprobe(
        prog: *mut bpf_program,
        retprobe: bool,
        pid: c_int,
        binary_path: *const c_char,
        func_offset: usize,
    ) -> *mut bpf_link;
    fn bpf_program__attach_usdt(
        prog: *mut bpf_program,
        pid: c_int,
        binary_path: *const c_char,
        usdt_provider: *const c_char,
        usdt_name: *const c_char,
        opts: *const c_void,
    ) -> *mut bpf_link;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn hits_drops_report_progress(res: *mut bench_res, prev: *mut bench_res);
    fn hits_drops_report_final(res: *mut bench_res);
    fn usdt_1();
    fn usdt_2();
}

static mut args: trigger_args = trigger_args { batch_iters: 100 };

static OPT_TRIG_BATCH_ITERS: &[u8] = b"trig-batch-iters\0";
static OPT_BATCH_ITER_CNT: &[u8] = b"BATCH_ITER_CNT\0";
static OPT_TRIG_DOC: &[u8] = b"Number of in-kernel iterations per one driver test run\0";

static opts: [argp_option; 2] = [
    argp_option {
        name: OPT_TRIG_BATCH_ITERS.as_ptr() as *const c_char,
        key: ARG_TRIG_BATCH_ITERS,
        arg: OPT_BATCH_ITER_CNT.as_ptr() as *const c_char,
        flags: 0,
        doc: OPT_TRIG_DOC.as_ptr() as *const c_char,
        group: 0,
    },
    argp_option {
        name: ptr::null(),
        key: 0,
        arg: ptr::null(),
        flags: 0,
        doc: ptr::null(),
        group: 0,
    },
];

unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    let ret: c_long;

    match key {
        ARG_TRIG_BATCH_ITERS => {
            ret = strtol(arg, ptr::null_mut(), 10);
            if ret < 1 || ret > MAX_TRIG_BATCH_ITERS {
                fprintf(
                    stderr,
                    b"invalid --trig-batch-iters value (should be between %d and %d)\n\0"
                        .as_ptr() as *const c_char,
                    1,
                    MAX_TRIG_BATCH_ITERS as c_int,
                );
                argp_usage(state);
            }
            args.batch_iters = ret as __u32;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }

    0
}

#[unsafe(no_mangle)]
pub static bench_trigger_batch_argp: argp = argp {
    options: opts.as_ptr(),
    parser: Some(parse_arg),
};

/* adjust slot shift in inc_hits() if changing */

/* GCC diagnostic "-Wattributes" ignored in the original C source. */

/* BPF triggering benchmarks */
static mut ctx: trigger_ctx = trigger_ctx {
    skel: ptr::null_mut(),
    usermode_counters: false,
    driver_prog_fd: 0,
};

static mut base_hits: [counter; MAX_BUCKETS] = [counter { value: 0 }; MAX_BUCKETS];

unsafe fn atomic_inc(p: *mut c_long) {
    core::sync::atomic::AtomicI64::from_ptr(p as *mut i64).fetch_add(1, core::sync::atomic::Ordering::SeqCst);
}

unsafe fn atomic_swap(p: *mut c_long, v: c_long) -> c_long {
    core::sync::atomic::AtomicI64::from_ptr(p as *mut i64).swap(v as i64, core::sync::atomic::Ordering::SeqCst) as c_long
}

unsafe fn inc_counter(_counters: *mut counter) {
    static mut TID: c_int = 0;
    let mut slot: c_uint;

    if TID == 0 {
        TID = sys_gettid();
    }

    /* multiplicative hashing, it's fast */
    slot = 2654435769u32.wrapping_mul(TID as c_uint);
    slot >>= 24;

    atomic_inc(&mut base_hits[slot as usize].value as *mut c_long); /* use highest byte as an index */
}

unsafe fn sum_and_reset_counters(counters: *mut counter) -> c_long {
    let mut i: c_int;
    let mut sum: c_long = 0;

    i = 0;
    while i < MAX_BUCKETS as c_int {
        sum += atomic_swap(&mut (*counters.add(i as usize)).value as *mut c_long, 0);
        i += 1;
    }
    sum
}

unsafe extern "C" fn trigger_validate() {
    if env.consumer_cnt != 0 {
        fprintf(stderr, b"benchmark doesn't support consumer!\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn trigger_producer(_input: *mut c_void) -> *mut c_void {
    if ctx.usermode_counters {
        loop {
            syscall(__NR_getpgid);
            inc_counter(base_hits.as_mut_ptr());
        }
    } else {
        loop {
            syscall(__NR_getpgid);
        }
    }
}

unsafe extern "C" fn trigger_producer_batch(_input: *mut c_void) -> *mut c_void {
    let fd: c_int = if ctx.driver_prog_fd != 0 {
        ctx.driver_prog_fd
    } else {
        bpf_program__fd((*ctx.skel).progs.trigger_driver)
    };

    loop {
        bpf_prog_test_run_opts(fd, ptr::null_mut());
    }
}

unsafe extern "C" fn trigger_measure(res: *mut bench_res) {
    if ctx.usermode_counters {
        (*res).hits = sum_and_reset_counters(base_hits.as_mut_ptr());
    } else {
        (*res).hits = sum_and_reset_counters((*(*ctx.skel).bss).hits.as_mut_ptr());
    }
}

unsafe fn setup_ctx() {
    setup_libbpf();

    ctx.skel = trigger_bench__open();
    if ctx.skel.is_null() {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    /* default "driver" BPF program */
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver, true);

    (*(*ctx.skel).rodata).batch_iters = args.batch_iters;
    (*(*ctx.skel).rodata).stacktrace = env.stacktrace;
}

unsafe fn load_ctx() {
    let err: c_int;

    err = trigger_bench__load(ctx.skel);
    if err != 0 {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe fn attach_bpf(prog: *mut bpf_program) {
    let link: *mut bpf_link;

    link = bpf_program__attach(prog);
    if link.is_null() {
        fprintf(stderr, b"failed to attach program!\n\0".as_ptr() as *const c_char);
        exit(1);
    }
}

unsafe extern "C" fn trigger_syscall_count_setup() {
    ctx.usermode_counters = true;
}

/* Batched, staying mostly in-kernel triggering setups */
unsafe extern "C" fn trigger_kernel_count_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver, false);
    bpf_program__set_autoload((*ctx.skel).progs.trigger_kernel_count, true);
    load_ctx();
    /* override driver program */
    ctx.driver_prog_fd = bpf_program__fd((*ctx.skel).progs.trigger_kernel_count);
}

unsafe extern "C" fn trigger_kprobe_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_kprobe, true);
    load_ctx();
    attach_bpf((*ctx.skel).progs.bench_trigger_kprobe);
}

unsafe extern "C" fn trigger_kretprobe_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_kretprobe, true);
    load_ctx();
    attach_bpf((*ctx.skel).progs.bench_trigger_kretprobe);
}

unsafe extern "C" fn trigger_kprobe_multi_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_kprobe_multi, true);
    load_ctx();
    attach_bpf((*ctx.skel).progs.bench_trigger_kprobe_multi);
}

unsafe extern "C" fn trigger_kretprobe_multi_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_kretprobe_multi, true);
    load_ctx();
    attach_bpf((*ctx.skel).progs.bench_trigger_kretprobe_multi);
}

unsafe extern "C" fn trigger_fentry_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_fentry, true);
    load_ctx();
    attach_bpf((*ctx.skel).progs.bench_trigger_fentry);
}

unsafe fn attach_ksyms_all(empty: *mut bpf_program, kretprobe: bool) {
    let mut opts = bpf_kprobe_multi_opts {
        sz: core::mem::size_of::<bpf_kprobe_multi_opts>(),
        syms: ptr::null(),
        cnt: 0,
        retprobe: false,
    };
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut ksyms: *mut ksyms = ptr::null_mut();

    /* Some recursive functions will be skipped in
     * bpf_get_ksyms -> skip_entry, as they can introduce sufficient
     * overhead. However, it's difficut to skip all the recursive
     * functions for a debug kernel.
     *
     * So, don't run the kprobe-multi-all and kretprobe-multi-all on
     * a debug kernel.
     */
    if bpf_get_ksyms(&mut ksyms, true) != 0 {
        fprintf(stderr, b"failed to get ksyms\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    opts.syms = (*ksyms).filtered_syms as *const *const c_char;
    opts.cnt = (*ksyms).filtered_cnt;
    opts.retprobe = kretprobe;
    /* attach empty to all the kernel functions except bpf_get_numa_node_id. */
    link = bpf_program__attach_kprobe_multi_opts(empty, ptr::null(), &opts);
    free_kallsyms_local(ksyms);
    if link.is_null() {
        fprintf(
            stderr,
            b"failed to attach bpf_program__attach_kprobe_multi_opts to all\n\0".as_ptr()
                as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn trigger_kprobe_multi_all_setup() {
    let prog: *mut bpf_program;
    let empty: *mut bpf_program;

    setup_ctx();
    empty = (*ctx.skel).progs.bench_kprobe_multi_empty;
    prog = (*ctx.skel).progs.bench_trigger_kprobe_multi;
    bpf_program__set_autoload(empty, true);
    bpf_program__set_autoload(prog, true);
    load_ctx();

    attach_ksyms_all(empty, false);
    attach_bpf(prog);
}

unsafe extern "C" fn trigger_kretprobe_multi_all_setup() {
    let prog: *mut bpf_program;
    let empty: *mut bpf_program;

    setup_ctx();
    empty = (*ctx.skel).progs.bench_kretprobe_multi_empty;
    prog = (*ctx.skel).progs.bench_trigger_kretprobe_multi;
    bpf_program__set_autoload(empty, true);
    bpf_program__set_autoload(prog, true);
    load_ctx();

    attach_ksyms_all(empty, true);
    attach_bpf(prog);
}

unsafe extern "C" fn trigger_fexit_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_fexit, true);
    load_ctx();
    attach_bpf((*ctx.skel).progs.bench_trigger_fexit);
}

unsafe extern "C" fn trigger_fmodret_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver, false);
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver_kfunc, true);
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_fmodret, true);
    load_ctx();
    /* override driver program */
    ctx.driver_prog_fd = bpf_program__fd((*ctx.skel).progs.trigger_driver_kfunc);
    attach_bpf((*ctx.skel).progs.bench_trigger_fmodret);
}

unsafe extern "C" fn trigger_tp_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver, false);
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver_kfunc, true);
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_tp, true);
    load_ctx();
    /* override driver program */
    ctx.driver_prog_fd = bpf_program__fd((*ctx.skel).progs.trigger_driver_kfunc);
    attach_bpf((*ctx.skel).progs.bench_trigger_tp);
}

unsafe extern "C" fn trigger_rawtp_setup() {
    setup_ctx();
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver, false);
    bpf_program__set_autoload((*ctx.skel).progs.trigger_driver_kfunc, true);
    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_rawtp, true);
    load_ctx();
    /* override driver program */
    ctx.driver_prog_fd = bpf_program__fd((*ctx.skel).progs.trigger_driver_kfunc);
    attach_bpf((*ctx.skel).progs.bench_trigger_rawtp);
}

/* make sure call is not inlined and not avoided by compiler, so __weak and
 * inline asm volatile in the body of the function
 *
 * There is a performance difference between uprobing at nop location vs other
 * instructions. So use two different targets, one of which starts with nop
 * and another doesn't.
 *
 * GCC doesn't generate stack setup preamble for these functions due to them
 * having no input arguments and doing nothing in the body.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_target_nop() {
    core::arch::asm!("nop", options(nostack, preserves_flags));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn opaque_noop_func() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_target_push() -> c_int {
    /* overhead of function call is negligible compared to uprobe
     * triggering, so this shouldn't affect benchmark results much
     */
    opaque_noop_func();
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_target_ret() {
    core::arch::asm!("", options(nostack, preserves_flags));
}

unsafe extern "C" fn uprobe_producer_count(_input: *mut c_void) -> *mut c_void {
    loop {
        uprobe_target_nop();
        inc_counter(base_hits.as_mut_ptr());
    }
}

unsafe extern "C" fn uprobe_producer_nop(_input: *mut c_void) -> *mut c_void {
    loop {
        uprobe_target_nop();
    }
}

unsafe extern "C" fn uprobe_producer_push(_input: *mut c_void) -> *mut c_void {
    loop {
        uprobe_target_push();
    }
}

unsafe extern "C" fn uprobe_producer_ret(_input: *mut c_void) -> *mut c_void {
    loop {
        uprobe_target_ret();
    }
}

#[cfg(target_arch = "x86_64")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_target_nop10() {
    core::arch::asm!(".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00", options(nostack, preserves_flags));
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uprobe_producer_nop10(_input: *mut c_void) -> *mut c_void {
    loop {
        uprobe_target_nop10();
    }
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uprobe_producer_usdt_nop(_input: *mut c_void) -> *mut c_void {
    loop {
        usdt_1();
    }
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uprobe_producer_usdt_nop10(_input: *mut c_void) -> *mut c_void {
    loop {
        usdt_2();
    }
}

unsafe fn usetup(use_retprobe: bool, use_multi: bool, target_addr: *mut c_void) {
    let mut uprobe_offset: usize;
    let link: *mut bpf_link;
    let err: c_int;

    setup_libbpf();

    ctx.skel = trigger_bench__open();
    if ctx.skel.is_null() {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    if use_multi {
        bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_uprobe_multi, true);
    } else {
        bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_uprobe, true);
    }

    err = trigger_bench__load(ctx.skel);
    if err != 0 {
        fprintf(stderr, b"failed to load skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    uprobe_offset = get_uprobe_offset(target_addr);
    if use_multi {
        let opts = bpf_uprobe_multi_opts {
            sz: core::mem::size_of::<bpf_uprobe_multi_opts>(),
            retprobe: use_retprobe,
            cnt: 1,
            offsets: &mut uprobe_offset,
        };
        link = bpf_program__attach_uprobe_multi(
            (*ctx.skel).progs.bench_trigger_uprobe_multi,
            -1, /* all PIDs */
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            ptr::null(),
            &opts,
        );
        (*ctx.skel).links.bench_trigger_uprobe_multi = link;
    } else {
        link = bpf_program__attach_uprobe(
            (*ctx.skel).progs.bench_trigger_uprobe,
            use_retprobe,
            -1, /* all PIDs */
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            uprobe_offset,
        );
        (*ctx.skel).links.bench_trigger_uprobe = link;
    }
    if link.is_null() {
        fprintf(
            stderr,
            b"failed to attach %s!\n\0".as_ptr() as *const c_char,
            if use_multi {
                b"multi-uprobe\0".as_ptr()
            } else {
                b"uprobe\0".as_ptr()
            } as *const c_char,
        );
        exit(1);
    }
}

unsafe extern "C" fn usermode_count_setup() {
    ctx.usermode_counters = true;
}

unsafe extern "C" fn uprobe_nop_setup() {
    usetup(false, false, uprobe_target_nop as *mut c_void);
}

unsafe extern "C" fn uretprobe_nop_setup() {
    usetup(true, false, uprobe_target_nop as *mut c_void);
}

unsafe extern "C" fn uprobe_push_setup() {
    usetup(false, false, uprobe_target_push as *mut c_void);
}

unsafe extern "C" fn uretprobe_push_setup() {
    usetup(true, false, uprobe_target_push as *mut c_void);
}

unsafe extern "C" fn uprobe_ret_setup() {
    usetup(false, false, uprobe_target_ret as *mut c_void);
}

unsafe extern "C" fn uretprobe_ret_setup() {
    usetup(true, false, uprobe_target_ret as *mut c_void);
}

unsafe extern "C" fn uprobe_multi_nop_setup() {
    usetup(false, true, uprobe_target_nop as *mut c_void);
}

unsafe extern "C" fn uretprobe_multi_nop_setup() {
    usetup(true, true, uprobe_target_nop as *mut c_void);
}

unsafe extern "C" fn uprobe_multi_push_setup() {
    usetup(false, true, uprobe_target_push as *mut c_void);
}

unsafe extern "C" fn uretprobe_multi_push_setup() {
    usetup(true, true, uprobe_target_push as *mut c_void);
}

unsafe extern "C" fn uprobe_multi_ret_setup() {
    usetup(false, true, uprobe_target_ret as *mut c_void);
}

unsafe extern "C" fn uretprobe_multi_ret_setup() {
    usetup(true, true, uprobe_target_ret as *mut c_void);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uprobe_nop10_setup() {
    usetup(false, false, uprobe_target_nop10 as *mut c_void);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uretprobe_nop10_setup() {
    usetup(true, false, uprobe_target_nop10 as *mut c_void);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uprobe_multi_nop10_setup() {
    usetup(false, true, uprobe_target_nop10 as *mut c_void);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn uretprobe_multi_nop10_setup() {
    usetup(true, true, uprobe_target_nop10 as *mut c_void);
}

#[cfg(target_arch = "x86_64")]
unsafe fn usdt_setup(name: *const c_char) {
    let link: *mut bpf_link;
    let err: c_int;

    setup_libbpf();

    ctx.skel = trigger_bench__open();
    if ctx.skel.is_null() {
        fprintf(stderr, b"failed to open skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    bpf_program__set_autoload((*ctx.skel).progs.bench_trigger_usdt, true);

    err = trigger_bench__load(ctx.skel);
    if err != 0 {
        fprintf(stderr, b"failed to load skeleton\n\0".as_ptr() as *const c_char);
        exit(1);
    }

    link = bpf_program__attach_usdt(
        (*ctx.skel).progs.bench_trigger_usdt,
        0, /*self*/
        b"/proc/self/exe\0".as_ptr() as *const c_char,
        b"optimized_attach\0".as_ptr() as *const c_char,
        name,
        ptr::null(),
    );
    if libbpf_get_error(link as *const c_void) != 0 {
        fprintf(
            stderr,
            b"failed to attach optimized_attach:%s usdt probe\n\0".as_ptr() as *const c_char,
            name,
        );
        exit(1);
    }
    (*ctx.skel).links.bench_trigger_usdt = link;
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn usdt_nop_setup() {
    usdt_setup(b"usdt_1\0".as_ptr() as *const c_char);
}

#[cfg(target_arch = "x86_64")]
unsafe extern "C" fn usdt_nop10_setup() {
    usdt_setup(b"usdt_2\0".as_ptr() as *const c_char);
}

#[unsafe(no_mangle)]
pub static bench_trig_syscall_count: bench = bench {
    name: b"trig-syscall-count\0".as_ptr() as *const c_char,
    validate: Some(trigger_validate),
    setup: Some(trigger_syscall_count_setup),
    producer_thread: Some(trigger_producer),
    measure: Some(trigger_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
    argp: ptr::null(),
};

macro_rules! BENCH_TRIG_KERNEL {
    ($kind:ident, $setup:ident, $name:expr) => {
        #[unsafe(no_mangle)]
        pub static $kind: bench = bench {
            name: concat!("trig-", $name, "\0").as_ptr() as *const c_char,
            validate: None,
            setup: Some($setup),
            producer_thread: Some(trigger_producer_batch),
            measure: Some(trigger_measure),
            report_progress: Some(hits_drops_report_progress),
            report_final: Some(hits_drops_report_final),
            argp: &bench_trigger_batch_argp,
        };
    };
}

BENCH_TRIG_KERNEL!(bench_trig_kernel_count, trigger_kernel_count_setup, "kernel-count");
BENCH_TRIG_KERNEL!(bench_trig_kprobe, trigger_kprobe_setup, "kprobe");
BENCH_TRIG_KERNEL!(bench_trig_kretprobe, trigger_kretprobe_setup, "kretprobe");
BENCH_TRIG_KERNEL!(bench_trig_kprobe_multi, trigger_kprobe_multi_setup, "kprobe-multi");
BENCH_TRIG_KERNEL!(bench_trig_kretprobe_multi, trigger_kretprobe_multi_setup, "kretprobe-multi");
BENCH_TRIG_KERNEL!(bench_trig_fentry, trigger_fentry_setup, "fentry");
BENCH_TRIG_KERNEL!(bench_trig_kprobe_multi_all, trigger_kprobe_multi_all_setup, "kprobe-multi-all");
BENCH_TRIG_KERNEL!(bench_trig_kretprobe_multi_all, trigger_kretprobe_multi_all_setup, "kretprobe-multi-all");
BENCH_TRIG_KERNEL!(bench_trig_fexit, trigger_fexit_setup, "fexit");
BENCH_TRIG_KERNEL!(bench_trig_fmodret, trigger_fmodret_setup, "fmodret");
BENCH_TRIG_KERNEL!(bench_trig_tp, trigger_tp_setup, "tp");
BENCH_TRIG_KERNEL!(bench_trig_rawtp, trigger_rawtp_setup, "rawtp");

macro_rules! BENCH_TRIG_USERMODE {
    ($kind:ident, $setup:ident, $producer:ident, $name:expr) => {
        #[unsafe(no_mangle)]
        pub static $kind: bench = bench {
            name: concat!("trig-", $name, "\0").as_ptr() as *const c_char,
            validate: Some(trigger_validate),
            setup: Some($setup),
            producer_thread: Some($producer),
            measure: Some(trigger_measure),
            report_progress: Some(hits_drops_report_progress),
            report_final: Some(hits_drops_report_final),
            argp: ptr::null(),
        };
    };
}

BENCH_TRIG_USERMODE!(bench_trig_usermode_count, usermode_count_setup, uprobe_producer_count, "usermode-count");
BENCH_TRIG_USERMODE!(bench_trig_uprobe_nop, uprobe_nop_setup, uprobe_producer_nop, "uprobe-nop");
BENCH_TRIG_USERMODE!(bench_trig_uprobe_push, uprobe_push_setup, uprobe_producer_push, "uprobe-push");
BENCH_TRIG_USERMODE!(bench_trig_uprobe_ret, uprobe_ret_setup, uprobe_producer_ret, "uprobe-ret");
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_nop, uretprobe_nop_setup, uprobe_producer_nop, "uretprobe-nop");
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_push, uretprobe_push_setup, uprobe_producer_push, "uretprobe-push");
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_ret, uretprobe_ret_setup, uprobe_producer_ret, "uretprobe-ret");
BENCH_TRIG_USERMODE!(bench_trig_uprobe_multi_nop, uprobe_multi_nop_setup, uprobe_producer_nop, "uprobe-multi-nop");
BENCH_TRIG_USERMODE!(bench_trig_uprobe_multi_push, uprobe_multi_push_setup, uprobe_producer_push, "uprobe-multi-push");
BENCH_TRIG_USERMODE!(bench_trig_uprobe_multi_ret, uprobe_multi_ret_setup, uprobe_producer_ret, "uprobe-multi-ret");
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_multi_nop, uretprobe_multi_nop_setup, uprobe_producer_nop, "uretprobe-multi-nop");
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_multi_push, uretprobe_multi_push_setup, uprobe_producer_push, "uretprobe-multi-push");
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_multi_ret, uretprobe_multi_ret_setup, uprobe_producer_ret, "uretprobe-multi-ret");

#[cfg(target_arch = "x86_64")]
BENCH_TRIG_USERMODE!(bench_trig_uprobe_nop10, uprobe_nop10_setup, uprobe_producer_nop10, "uprobe-nop10");
#[cfg(target_arch = "x86_64")]
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_nop10, uretprobe_nop10_setup, uprobe_producer_nop10, "uretprobe-nop10");
#[cfg(target_arch = "x86_64")]
BENCH_TRIG_USERMODE!(bench_trig_uprobe_multi_nop10, uprobe_multi_nop10_setup, uprobe_producer_nop10, "uprobe-multi-nop10");
#[cfg(target_arch = "x86_64")]
BENCH_TRIG_USERMODE!(bench_trig_uretprobe_multi_nop10, uretprobe_multi_nop10_setup, uprobe_producer_nop10, "uretprobe-multi-nop10");
#[cfg(target_arch = "x86_64")]
BENCH_TRIG_USERMODE!(bench_trig_usdt_nop, usdt_nop_setup, uprobe_producer_usdt_nop, "usdt-nop");
#[cfg(target_arch = "x86_64")]
BENCH_TRIG_USERMODE!(bench_trig_usdt_nop10, usdt_nop10_setup, uprobe_producer_usdt_nop10, "usdt-nop10");
