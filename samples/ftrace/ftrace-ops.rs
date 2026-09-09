// SPDX-License-Identifier: GPL-2.0-only

// C dependencies supplied by the kernel headers are intentionally left as
// external Rust declarations.

use core::ffi::c_void;

extern "C" {
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn ftrace_set_filter_ip(ops: *mut ftrace_ops, ip: usize, remove: i32, reset: i32) -> i32;
    fn register_ftrace_function(ops: *mut ftrace_ops) -> i32;
    fn unregister_ftrace_function(ops: *mut ftrace_ops) -> i32;
    fn ftrace_free_filter(ops: *mut ftrace_ops);
    fn ktime_get() -> ktime_t;
    fn ktime_sub(a: ktime_t, b: ktime_t) -> ktime_t;
    fn ktime_to_ns(kt: ktime_t) -> u64;
    fn div_u64(n: u64, d: u32) -> u64;
}

type ktime_t = i64;
type ftrace_func_t = unsafe extern "C" fn(
    ip: usize,
    parent_ip: usize,
    op: *mut ftrace_ops,
    fregs: *mut ftrace_regs,
);

#[repr(C)]
pub struct ftrace_ops {
    pub func: Option<ftrace_func_t>,
    pub flags: usize,
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ftrace_regs {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct sample_ops {
    pub ops: ftrace_ops,
    pub count: u32,
}

const GFP_KERNEL: u32 = 0;
const FTRACE_OPS_FL_SAVE_REGS: usize = 1 << 0;
const FTRACE_OPS_FL_RECURSION: usize = 1 << 1;
const FTRACE_OPS_FL_RCU: usize = 1 << 2;

static mut nr_function_calls: u32 = 100000;
static mut nr_ops_relevant: u32 = 1;
static mut nr_ops_irrelevant: u32 = 0;
static mut save_regs: bool = false;
static mut assist_recursion: bool = false;
static mut assist_rcu: bool = false;
static mut check_count: bool = false;
static mut persist: bool = false;

static mut ops_relevant: *mut sample_ops = core::ptr::null_mut();
static mut ops_irrelevant: *mut sample_ops = core::ptr::null_mut();

#[inline(never)]
unsafe extern "C" fn tracee_relevant() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline(never)]
unsafe extern "C" fn tracee_irrelevant() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

unsafe extern "C" fn ops_func_nop(
    _ip: usize,
    _parent_ip: usize,
    _op: *mut ftrace_ops,
    _fregs: *mut ftrace_regs,
) {
    // do nothing
}

unsafe extern "C" fn ops_func_count(
    _ip: usize,
    _parent_ip: usize,
    op: *mut ftrace_ops,
    _fregs: *mut ftrace_regs,
) {
    let self_ptr = (op as *mut u8).sub(core::mem::offset_of!(sample_ops, ops)) as *mut sample_ops;
    (*self_ptr).count = (*self_ptr).count.wrapping_add(1);
}

unsafe fn ops_alloc_init(
    tracee: *mut c_void,
    func: ftrace_func_t,
    flags: usize,
    nr: i32,
) -> *mut sample_ops {
    let ops = kcalloc(nr as usize, core::mem::size_of::<sample_ops>(), GFP_KERNEL)
        as *mut sample_ops;
    if ops.is_null() {
        return core::ptr::null_mut();
    }

    for i in 0..(nr as usize) {
        (*ops.add(i)).ops.func = Some(func);
        (*ops.add(i)).ops.flags = flags;
        let _ = ftrace_set_filter_ip(&mut (*ops.add(i)).ops, tracee as usize, 0, 0);
        let _ = register_ftrace_function(&mut (*ops.add(i)).ops);
    }
    ops
}

unsafe fn ops_destroy(ops: *mut sample_ops, nr: i32) {
    if ops.is_null() {
        return;
    }
    for i in 0..(nr as usize) {
        let _ = unregister_ftrace_function(&mut (*ops.add(i)).ops);
        ftrace_free_filter(&mut (*ops.add(i)).ops);
    }
    kfree(ops as *mut c_void);
}

unsafe fn ops_check(ops: *mut sample_ops, nr: i32, expected_count: u32) {
    if ops.is_null() || !check_count {
        return;
    }
    for i in 0..(nr as usize) {
        if (*ops.add(i)).count == expected_count {
            continue;
        }
        // C: pr_warn("Counter called %u times (expected %u)\n", ...)
    }
}

static mut tracer_relevant: ftrace_func_t = ops_func_nop;
static mut tracer_irrelevant: ftrace_func_t = ops_func_nop;

unsafe extern "C" fn ftrace_ops_sample_init() -> i32 {
    let mut flags: usize = 0;
    let start: ktime_t;
    let end: ktime_t;
    let period: u64;

    if save_regs {
        flags |= FTRACE_OPS_FL_SAVE_REGS;
    }
    if assist_recursion { flags |= FTRACE_OPS_FL_RECURSION; }
    if assist_rcu { flags |= FTRACE_OPS_FL_RCU; }
    if check_count {
        tracer_relevant = ops_func_count;
        tracer_irrelevant = ops_func_count;
    }

    ops_relevant = ops_alloc_init(tracee_relevant as *mut c_void, tracer_relevant, flags, nr_ops_relevant as i32);
    ops_irrelevant = ops_alloc_init(tracee_irrelevant as *mut c_void, tracer_irrelevant, flags, nr_ops_irrelevant as i32);

    start = ktime_get();
    for _ in 0..nr_function_calls { tracee_relevant(); }
    end = ktime_get();
    ops_check(ops_relevant, nr_ops_relevant as i32, nr_function_calls);
    ops_check(ops_irrelevant, nr_ops_irrelevant as i32, 0);
    period = ktime_to_ns(ktime_sub(end, start));

    if persist { return 0; }
    ops_destroy(ops_relevant, nr_ops_relevant as i32);
    ops_destroy(ops_irrelevant, nr_ops_irrelevant as i32);
    -22
}

unsafe extern "C" fn ftrace_ops_sample_exit() {
    ops_destroy(ops_relevant, nr_ops_relevant as i32);
    ops_destroy(ops_irrelevant, nr_ops_irrelevant as i32);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
