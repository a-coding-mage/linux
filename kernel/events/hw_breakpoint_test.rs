// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for hw_breakpoint constraints accounting logic.
 *
 * Copyright (C) 2022, Google LLC.
 */

// Dependencies supplied by the kernel/KUnit environment are intentionally external.

const MAX_TEST_BREAKPOINTS: usize = 512;

static mut BREAK_VARS: [core::ffi::c_char; MAX_TEST_BREAKPOINTS] = [0; MAX_TEST_BREAKPOINTS];
static mut TEST_BPS: [*mut perf_event; MAX_TEST_BREAKPOINTS] = [core::ptr::null_mut(); MAX_TEST_BREAKPOINTS];
static mut OTHER_TASK: *mut task_struct = core::ptr::null_mut();

#[allow(non_camel_case_types)]
#[repr(C)]
struct kunit;
#[allow(non_camel_case_types)]
#[repr(C)]
struct perf_event;
#[allow(non_camel_case_types)]
#[repr(C)]
struct task_struct;
#[allow(non_camel_case_types)]
#[repr(C)]
struct perf_event_attr {
    _data: [u8; 0],
}
#[allow(non_camel_case_types)]
#[repr(C)]
struct kunit_case {
    _data: [u8; 0],
}
#[allow(non_camel_case_types)]
#[repr(C)]
struct kunit_suite {
    name: *const core::ffi::c_char,
    test_cases: *mut kunit_case,
    init: Option<unsafe extern "C" fn(*mut kunit) -> i32>,
    exit: Option<unsafe extern "C" fn(*mut kunit)>,
}

extern "C" {
    fn hw_breakpoint_init(attr: *mut perf_event_attr);
    fn perf_event_create_kernel_counter(attr: *mut perf_event_attr, cpu: i32, tsk: *mut task_struct, overflow_handler: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut perf_event;
    fn unregister_hw_breakpoint(bp: *mut perf_event);
    fn hw_breakpoint_slots(kind: i32) -> i32;
    fn kthread_create(threadfn: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void, name: *const core::ffi::c_char) -> *mut task_struct;
    fn kthread_stop(tsk: *mut task_struct);
    fn num_online_cpus() -> i32;
    fn hw_breakpoint_is_used() -> bool;
    static mut current: *mut task_struct;
}

const TYPE_DATA: i32 = 0;
const HW_BREAKPOINT_LEN_1: u32 = 1;
const HW_BREAKPOINT_RW: u32 = 3;

unsafe fn register_test_bp(cpu: i32, tsk: *mut task_struct, idx: i32) -> *mut perf_event {
    let mut attr: perf_event_attr = core::mem::zeroed();
    if idx < 0 || idx >= MAX_TEST_BREAKPOINTS as i32 {
        return core::ptr::null_mut();
    }
    hw_breakpoint_init(&mut attr);
    let _ = (&mut BREAK_VARS[idx as usize]) as *mut _ as usize;
    perf_event_create_kernel_counter(&mut attr, cpu, tsk, core::ptr::null_mut(), core::ptr::null_mut())
}

unsafe fn unregister_test_bp(bp: *mut *mut perf_event) {
    if (*bp).is_null() {
        return;
    }
    unregister_hw_breakpoint(*bp);
    *bp = core::ptr::null_mut();
}

unsafe fn get_test_bp_slots() -> i32 {
    static mut SLOTS: i32 = 0;
    if SLOTS == 0 {
        SLOTS = hw_breakpoint_slots(TYPE_DATA);
    }
    SLOTS
}

unsafe fn fill_one_bp_slot(_test: *mut kunit, id: &mut i32, cpu: i32, tsk: *mut task_struct) {
    let bp = register_test_bp(cpu, tsk, *id);
    TEST_BPS[*id as usize] = bp;
    *id += 1;
}

unsafe fn fill_bp_slots(test: *mut kunit, id: &mut i32, cpu: i32, tsk: *mut task_struct, skip: i32) -> bool {
    for _ in 0..(get_test_bp_slots() - skip) {
        fill_one_bp_slot(test, id, cpu, tsk);
    }
    *id + get_test_bp_slots() <= MAX_TEST_BREAKPOINTS as i32
}

unsafe extern "C" fn dummy_kthread(_arg: *mut core::ffi::c_void) -> i32 { 0 }

unsafe fn get_other_task(_test: *mut kunit) -> *mut task_struct {
    if !OTHER_TASK.is_null() { return OTHER_TASK; }
    OTHER_TASK = kthread_create(dummy_kthread, core::ptr::null_mut(), b"hw_breakpoint_dummy_task\0".as_ptr() as *const _);
    OTHER_TASK
}

unsafe fn get_test_cpu(num: i32) -> i32 {
    let mut cpu = 0;
    let mut n = num;
    while cpu < num_online_cpus() {
        if n <= 0 { break; }
        n -= 1;
        cpu += 1;
    }
    cpu
}

// ===== Test cases =====

unsafe fn test_one_cpu(test: *mut kunit) {
    let mut idx = 0;
    fill_bp_slots(test, &mut idx, get_test_cpu(0), core::ptr::null_mut(), 0);
    let _ = register_test_bp(-1, current, idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
}

unsafe fn test_many_cpus(test: *mut kunit) {
    let mut idx = 0;
    for cpu in 0..num_online_cpus() {
        if !fill_bp_slots(test, &mut idx, cpu, core::ptr::null_mut(), 0) { break; }
        let _ = register_test_bp(cpu, core::ptr::null_mut(), idx);
    }
}

unsafe fn test_one_task_on_all_cpus(test: *mut kunit) {
    let mut idx = 0;
    fill_bp_slots(test, &mut idx, -1, current, 0);
    let _ = register_test_bp(-1, current, idx);
    let _ = register_test_bp(get_test_cpu(0), current, idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    unregister_test_bp(&mut TEST_BPS[0]);
    fill_one_bp_slot(test, &mut idx, get_test_cpu(0), core::ptr::null_mut());
}

unsafe fn test_two_tasks_on_all_cpus(test: *mut kunit) {
    let mut idx = 0;
    fill_bp_slots(test, &mut idx, -1, current, 0);
    fill_bp_slots(test, &mut idx, -1, get_other_task(test), 0);
    let _ = register_test_bp(-1, current, idx);
    let _ = register_test_bp(-1, get_other_task(test), idx);
    let _ = register_test_bp(get_test_cpu(0), current, idx);
    let _ = register_test_bp(get_test_cpu(0), get_other_task(test), idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    unregister_test_bp(&mut TEST_BPS[0]);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
}

unsafe fn test_one_task_on_one_cpu(test: *mut kunit) {
    let mut idx = 0;
    fill_bp_slots(test, &mut idx, get_test_cpu(0), current, 0);
    let _ = register_test_bp(-1, current, idx);
    let _ = register_test_bp(get_test_cpu(0), current, idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    unregister_test_bp(&mut TEST_BPS[0]);
    fill_one_bp_slot(test, &mut idx, get_test_cpu(0), core::ptr::null_mut());
}

unsafe fn test_one_task_mixed(test: *mut kunit) {
    let mut idx = 0;
    fill_one_bp_slot(test, &mut idx, get_test_cpu(0), current);
    fill_bp_slots(test, &mut idx, -1, current, 1);
    let _ = register_test_bp(-1, current, idx);
    let _ = register_test_bp(get_test_cpu(0), current, idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    unregister_test_bp(&mut TEST_BPS[0]); unregister_test_bp(&mut TEST_BPS[1]);
    fill_one_bp_slot(test, &mut idx, get_test_cpu(0), core::ptr::null_mut());
    fill_one_bp_slot(test, &mut idx, get_test_cpu(0), core::ptr::null_mut());
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
}

unsafe fn test_two_tasks_on_one_cpu(test: *mut kunit) {
    let mut idx = 0;
    fill_bp_slots(test, &mut idx, get_test_cpu(0), current, 0);
    fill_bp_slots(test, &mut idx, get_test_cpu(0), get_other_task(test), 0);
    let _ = register_test_bp(-1, current, idx); let _ = register_test_bp(-1, get_other_task(test), idx);
    let _ = register_test_bp(get_test_cpu(0), current, idx); let _ = register_test_bp(get_test_cpu(0), get_other_task(test), idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    fill_bp_slots(test, &mut idx, get_test_cpu(1), core::ptr::null_mut(), 0);
}

unsafe fn test_two_tasks_on_one_all_cpus(test: *mut kunit) {
    let mut idx = 0;
    fill_bp_slots(test, &mut idx, get_test_cpu(0), current, 0);
    fill_bp_slots(test, &mut idx, -1, get_other_task(test), 0);
    let _ = register_test_bp(-1, current, idx); let _ = register_test_bp(-1, get_other_task(test), idx);
    let _ = register_test_bp(get_test_cpu(0), current, idx); let _ = register_test_bp(get_test_cpu(0), get_other_task(test), idx);
    let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    let _ = register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx);
}

unsafe fn test_task_on_all_and_one_cpu(test: *mut kunit) {
    let mut idx = 0; fill_bp_slots(test, &mut idx, -1, current, 2);
    let tsk_on_cpu_idx = idx; fill_one_bp_slot(test, &mut idx, get_test_cpu(0), current); fill_one_bp_slot(test, &mut idx, -1, current);
    let _ = register_test_bp(-1, current, idx); let _ = register_test_bp(get_test_cpu(0), current, idx); let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx);
    let cpu_idx = idx; fill_one_bp_slot(test, &mut idx, get_test_cpu(1), core::ptr::null_mut()); let _ = register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx);
    unregister_test_bp(&mut TEST_BPS[tsk_on_cpu_idx as usize]); let _ = register_test_bp(-1, current, idx);
    unregister_test_bp(&mut TEST_BPS[cpu_idx as usize]); fill_one_bp_slot(test, &mut idx, -1, current);
    let _ = register_test_bp(-1, current, idx); let _ = register_test_bp(get_test_cpu(0), current, idx); let _ = register_test_bp(get_test_cpu(0), core::ptr::null_mut(), idx); let _ = register_test_bp(get_test_cpu(1), core::ptr::null_mut(), idx);
}

unsafe fn test_init(_test: *mut kunit) -> i32 { 0 }
unsafe fn test_exit(_test: *mut kunit) {
    for i in 0..MAX_TEST_BREAKPOINTS { if !TEST_BPS[i].is_null() { unregister_test_bp(&mut TEST_BPS[i]); } }
    if !OTHER_TASK.is_null() { kthread_stop(OTHER_TASK); OTHER_TASK = core::ptr::null_mut(); }
}

// The C KUnit case array, suite registration, and MODULE_AUTHOR metadata are
// supplied by the kernel's Rust-facing integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
