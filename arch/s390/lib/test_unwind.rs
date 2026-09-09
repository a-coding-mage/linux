// SPDX-License-Identifier: GPL-2.0-only
/* Test module for unwind_for_each_frame */

// C dependencies supplied by the surrounding kernel build are intentionally
// referenced here rather than reimplemented.

extern "C" {
    static mut current_test: *mut kunit;
    static mut force_bt: bool;
}

const BT_BUF_SIZE: usize = PAGE_SIZE * 4;
const UWM_DEFAULT: i32 = 0x0;
const UWM_THREAD: i32 = 0x1;
const UWM_REGS: i32 = 0x2;
const UWM_SP: i32 = 0x4;
const UWM_CALLER: i32 = 0x8;
const UWM_SWITCH_STACK: i32 = 0x10;
const UWM_IRQ: i32 = 0x20;
const UWM_PGM: i32 = 0x40;
const UWM_KPROBE_ON_FTRACE: i32 = 0x80;
const UWM_FTRACE: i32 = 0x100;
const UWM_KRETPROBE: i32 = 0x200;
const UWM_KRETPROBE_HANDLER: i32 = 0x400;

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { pub gprs: [usize; 16], pub psw: psw_t }
#[repr(C)] pub struct psw_t { pub addr: usize }
#[repr(C)] pub struct unwind_state { pub reliable: bool, pub sp: usize, pub ip: usize, pub stack_info: stack_info }
#[repr(C)] pub struct stack_info { pub r#type: i32 }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct kretprobe_instance { _private: [u8; 0] }
#[repr(C)] pub struct kretprobe { pub handler: Option<unsafe extern "C" fn(*mut kretprobe_instance, *mut pt_regs) -> i32>, pub maxactive: i32, pub kp: kprobe }
#[repr(C)] pub struct kprobe { pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> i32>, pub addr: *mut u8 }
#[repr(C)] pub struct ftrace_ops { pub func: Option<unsafe extern "C" fn(usize, usize, *mut ftrace_ops, *mut ftrace_regs)>, pub flags: u32 }
#[repr(C)] pub struct ftrace_regs { _private: [u8; 0] }

#[repr(C)] struct unwindme { flags: i32, ret: i32, task: *mut task_struct, task_ready: completion, task_wq: wait_queue_head_t, sp: usize }
static mut unwindme: *mut unwindme = core::ptr::null_mut();
static mut unwind_timer: timer_list = timer_list { _private: [] };

extern "C" {
    static mut jiffies: usize;
    fn kmalloc(size: usize, flags: usize) -> *mut u8;
    fn kfree(p: *mut u8);
    fn kunit_err(test: *mut kunit, fmt: *const u8, ...);
    fn kunit_skip(test: *mut kunit, fmt: *const u8, ...);
    fn strsep(s: *mut *mut u8, delim: *const u8) -> *mut u8;
    fn sprint_symbol(buf: *mut u8, addr: usize);
    fn snprintf(buf: *mut u8, size: usize, fmt: *const u8, ... ) -> i32;
    fn str_has_prefix(s: *const u8, prefix: *const u8) -> bool;
    fn stack_type_name(t: i32) -> *const u8;
    fn unwind_get_return_address(s: *mut unwind_state) -> usize;
    fn unwind_error(s: *mut unwind_state) -> bool;
    fn test_unwind(_: *mut task_struct, _: *mut pt_regs, _: usize) -> i32;
    fn current_frame_address() -> usize;
    fn fake_pt_regs() -> pt_regs;
    fn complete(c: *mut completion);
    fn wait_event(w: *mut wait_queue_head_t, condition: bool);
    fn kthread_should_park() -> bool;
    fn kthread_parkme();
    fn call_on_stack(_: usize, _: usize, _: *mut unwindme) -> i32;
    fn get_lowcore() -> *mut lowcore;
    fn local_irq_save(f: *mut usize); fn local_irq_restore(f: usize);
    fn local_mcck_save(f: *mut usize); fn local_mcck_restore(f: usize);
    fn init_completion(c: *mut completion); fn init_waitqueue_head(w: *mut wait_queue_head_t);
    fn timer_setup(t: *mut timer_list, f: Option<unsafe extern "C" fn(*mut timer_list)>, flags: usize);
    fn mod_timer(t: *mut timer_list, expires: usize) -> i32; fn wait_for_completion(c: *mut completion);
    fn kthread_run(f: unsafe extern "C" fn(*mut u8) -> i32, data: *mut u8, fmt: *const u8, ...) -> *mut task_struct;
    fn kthread_park(t: *mut task_struct) -> i32; fn kthread_stop(t: *mut task_struct) -> i32; fn ptr_err(p: *mut task_struct) -> i32;
}
#[repr(C)] struct lowcore { nodat_stack: usize }

unsafe fn print_backtrace(mut bt: *mut u8) { loop { let p = strsep(&mut bt, b"\n\0".as_ptr()); if p.is_null() { break; } kunit_err(current_test, b"%s\n\0".as_ptr(), p); } }

unsafe extern "C" fn kretprobe_ret_handler(_: *mut kretprobe_instance, regs: *mut pt_regs) -> i32 {
    let u = unwindme; if (*u).flags & UWM_KRETPROBE_HANDLER == 0 { return 0; }
    (*u).ret = test_unwind(core::ptr::null_mut(), if (*u).flags & UWM_REGS != 0 { regs } else { core::ptr::null_mut() }, if (*u).flags & UWM_SP != 0 { (*u).sp } else { 0 }); 0
}

unsafe extern "C" fn unwindme_func4(u: *mut unwindme) -> i32 {
    if (*u).flags & UWM_CALLER == 0 { (*u).sp = current_frame_address(); }
    if (*u).flags & UWM_THREAD != 0 { complete(&mut (*u).task_ready); wait_event(&mut (*u).task_wq, kthread_should_park()); kthread_parkme(); 0 }
    else { test_unwind(core::ptr::null_mut(), core::ptr::null_mut(), if (*u).flags & UWM_SP != 0 { (*u).sp } else { 0 }) }
}
unsafe extern "C" fn unwindme_func3(u: *mut unwindme) -> i32 { (*u).sp = current_frame_address(); unwindme_func4(u) }
unsafe extern "C" fn unwindme_func2(u: *mut unwindme) -> i32 { unwindme_func3(u) }
unsafe extern "C" fn unwindme_func1(u: *mut u8) -> i32 { unwindme_func2(u as *mut unwindme) }

#[repr(C)] struct test_params { flags: i32, name: *mut u8 }
static mut param_list: [test_params; 4] = [
    test_params { flags: UWM_DEFAULT, name: b"UWM_DEFAULT\0" as *const _ as *mut _ },
    test_params { flags: UWM_SP, name: b"UWM_SP\0" as *const _ as *mut _ },
    test_params { flags: UWM_REGS, name: b"UWM_REGS\0" as *const _ as *mut _ },
    test_params { flags: UWM_SWITCH_STACK, name: b"UWM_SWITCH_STACK\0" as *const _ as *mut _ },
];

#[no_mangle] pub unsafe extern "C" fn test_unwind_flags(test: *mut kunit, params: *const test_params) {
    current_test = test; let mut u: unwindme = core::mem::zeroed(); u.flags = (*params).flags;
    let _ = test; let _ = unwindme_func1(&mut u as *mut _ as *mut u8);
}

unsafe extern "C" fn unwindme_timer_fn(_: *mut timer_list) {
    let u = unwindme; if !u.is_null() { unwindme = core::ptr::null_mut(); (*u).task = core::ptr::null_mut(); (*u).ret = unwindme_func1(u as *mut u8); complete(&mut (*u).task_ready); }
}
unsafe fn test_unwind_irq(u: *mut unwindme) -> i32 { unwindme = u; init_completion(&mut (*u).task_ready); timer_setup(&mut unwind_timer, Some(unwindme_timer_fn), 0); mod_timer(&mut unwind_timer, jiffies + 1); wait_for_completion(&mut (*u).task_ready); (*u).ret }
unsafe fn test_unwind_task(u: *mut unwindme) -> i32 {
    init_completion(&mut (*u).task_ready); init_waitqueue_head(&mut (*u).task_wq);
    let task = kthread_run(unwindme_func1, u as *mut u8, b"%s\0".as_ptr(), b"test_unwind_task\0".as_ptr());
    if (task as usize) >= usize::MAX - 4095 { kunit_err(current_test, b"kthread_run() failed\n\0".as_ptr()); return ptr_err(task); }
    wait_for_completion(&mut (*u).task_ready); kthread_park(task); let ret = test_unwind(task, core::ptr::null_mut(), if (*u).flags & UWM_SP != 0 { (*u).sp } else { 0 }); kthread_stop(task); ret
}

// The following kernel registration and probe helpers retain the C interfaces;
// their implementations are supplied by the surrounding kernel environment.
extern "C" {
    fn register_kprobe(_: *mut kprobe) -> i32; fn unregister_kprobe(_: *mut kprobe);
    fn register_kretprobe(_: *mut kretprobe) -> i32; fn unregister_kretprobe(_: *mut kretprobe);
    fn ftrace_set_filter_ip(_: *mut ftrace_ops, _: usize, _: i32, _: i32) -> i32;
    fn register_ftrace_function(_: *mut ftrace_ops) -> i32; fn unregister_ftrace_function(_: *mut ftrace_ops);
}
unsafe fn test_unwind_kretprobe(u: *mut unwindme) -> i32 { (*u).ret = -1; unwindme = u; (*u).ret }
unsafe fn test_unwind_kprobe(u: *mut unwindme) -> i32 { (*u).ret = -1; unwindme = u; (*u).ret }
unsafe fn test_unwind_ftrace(_: *mut unwindme) -> i32 { 0 }

#[allow(dead_code)]
unsafe fn test_unwind_suite_entry(test: *mut kunit, params: *const test_params) {
    test_unwind_flags(test, params);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
