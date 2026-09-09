// SPDX-License-Identifier: GPL-2.0
/*
 * working set size estimation: monitor access pattern of given process and
 * print estimated working set size (total size of regions that showing some
 * access).
 */

// Dependencies supplied by the kernel DAMON, init, kernel, and module APIs.

static mut target_pid: i32 = 0;

extern "C" {
    static mut enabled: bool;
    static mut ctx: *mut damon_ctx;
    static mut target_pidp: *mut pid;

    fn param_get_bool(
        kp: *const kernel_param,
        val: *mut core::ffi::c_char,
    ) -> i32;
    fn kstrtobool(val: *const core::ffi::c_char, result: *mut bool) -> i32;
    fn damon_new_ctx() -> *mut damon_ctx;
    fn damon_select_ops(ctx: *mut damon_ctx, ops: i32) -> i32;
    fn damon_destroy_ctx(ctx: *mut damon_ctx);
    fn damon_new_target() -> *mut damon_target;
    fn damon_add_target(ctx: *mut damon_ctx, target: *mut damon_target);
    fn find_get_pid(pid: i32) -> *mut pid;
    fn damon_start(ctx: *mut *mut damon_ctx, nr_ctxs: i32, exclusive: bool) -> i32;
    fn damon_call(ctx: *mut damon_ctx, control: *mut damon_call_control) -> i32;
    fn damon_stop(ctx: *mut *mut damon_ctx, nr_ctxs: i32);
    fn damon_initialized() -> bool;
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct damon_ctx {
    _private: [u8; 0],
}
#[repr(C)]
pub struct damon_target {
    pub pid: *mut pid,
    _private: [u8; 0],
}
#[repr(C)]
pub struct damon_region {
    pub nr_accesses: u32,
    pub ar: damon_addr_range,
}
#[repr(C)]
pub struct damon_addr_range {
    pub start: usize,
    pub end: usize,
}
#[repr(C)]
pub struct pid {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kernel_param {
    _private: [u8; 0],
}
#[repr(C)]
pub struct damon_call_control {
    pub data: *mut core::ffi::c_void,
    pub fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub repeat: bool,
}

const DAMON_OPS_VADDR: i32 = 0;

unsafe extern "C" fn damon_sample_wsse_repeat_call_fn(data: *mut core::ffi::c_void) -> i32 {
    let c = data as *mut damon_ctx;
    // damon_for_each_target(t, c)
    let mut t: *mut damon_target = core::ptr::null_mut();
    while !t.is_null() {
        let mut wss: usize = 0;
        // damon_for_each_region(r, t)
        let mut r: *mut damon_region = core::ptr::null_mut();
        while !r.is_null() {
            if (*r).nr_accesses > 0 {
                wss = wss.wrapping_add((*r).ar.end.wrapping_sub((*r).ar.start));
            }
            r = core::ptr::null_mut();
        }
        pr_info(b"wss: %lu\0".as_ptr() as *const _, wss);
        t = core::ptr::null_mut();
    }
    let _ = c;
    0
}

static mut repeat_call_control: damon_call_control = damon_call_control {
    data: core::ptr::null_mut(),
    fn_: Some(damon_sample_wsse_repeat_call_fn),
    repeat: true,
};

unsafe fn damon_sample_wsse_start() -> i32 {
    let mut target: *mut damon_target;
    let mut err: i32;

    pr_info(b"start\0".as_ptr() as *const _);
    ctx = damon_new_ctx();
    if ctx.is_null() { return -12; }
    if damon_select_ops(ctx, DAMON_OPS_VADDR) != 0 {
        damon_destroy_ctx(ctx); return -22;
    }
    target = damon_new_target();
    if target.is_null() { damon_destroy_ctx(ctx); return -12; }
    damon_add_target(ctx, target);
    target_pidp = find_get_pid(target_pid);
    if target_pidp.is_null() {
        damon_destroy_ctx(ctx); return -22;
    }
    (*target).pid = target_pidp;
    err = damon_start(&mut ctx, 1, true);
    if err != 0 { damon_destroy_ctx(ctx); return err; }
    repeat_call_control.data = ctx as *mut _;
    err = damon_call(ctx, &mut repeat_call_control);
    if err != 0 { damon_destroy_ctx(ctx); }
    err
}

unsafe fn damon_sample_wsse_stop() {
    pr_info(b"stop\0".as_ptr() as *const _);
    if !ctx.is_null() {
        damon_stop(&mut ctx, 1);
        damon_destroy_ctx(ctx);
    }
}

unsafe fn damon_sample_wsse_enable_store(
    val: *const core::ffi::c_char, _kp: *const kernel_param,
) -> i32 {
    let is_enabled = enabled;
    let mut err = kstrtobool(val, &mut enabled);
    if err != 0 { return err; }
    if enabled == is_enabled { return 0; }
    if !damon_initialized() { return 0; }
    if enabled {
        err = damon_sample_wsse_start();
        if err != 0 { enabled = false; }
        return err;
    }
    damon_sample_wsse_stop();
    0
}

unsafe extern "C" fn damon_sample_wsse_init() -> i32 {
    let mut err = 0;
    if !damon_initialized() {
        err = -12;
        if enabled { enabled = false; }
    }
    if enabled {
        err = damon_sample_wsse_start();
        if err != 0 { enabled = false; }
    }
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
