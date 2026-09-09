// SPDX-License-Identifier: GPL-2.0-only
/*
 * KVM nVHE hypervisor stack tracing support.
 *
 * Copyright (C) 2022 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct KvmNvheStacktraceInfo {
    pub stack_base: c_ulong,
    pub overflow_stack_base: c_ulong,
    pub fp: c_ulong,
    pub pc: c_ulong,
}

extern "C" {
    static mut kvm_init_params: KvmNvheInitParams;
    fn this_cpu_ptr<T>(ptr: *mut T) -> *mut T;
    fn is_protected_kvm_enabled() -> bool;
    fn kvm_nvhe_unwind_init(state: *mut UnwindState, fp: c_ulong, pc: c_ulong);
    fn unwind_next_frame_record(state: *mut UnwindState) -> c_int;
}

#[repr(C)]
pub struct KvmNvheInitParams {
    pub stack_hyp_va: c_ulong,
}

#[repr(C)]
pub struct StackInfo {
    pub low: c_ulong,
    pub high: c_ulong,
}

#[repr(C)]
pub struct UnwindState {
    pub stacks: *mut StackInfo,
    pub nr_stacks: usize,
    // Remaining fields are supplied by the architecture unwind implementation.
}

pub type StackTraceConsumeFn = unsafe extern "C" fn(*mut c_void, c_ulong) -> bool;

pub type c_ulong = usize;
pub type c_int = i32;
pub type c_void = core::ffi::c_void;

// DEFINE_PER_CPU(unsigned long [OVERFLOW_STACK_SIZE/sizeof(long)], overflow_stack)
// __aligned(16);
#[repr(align(16))]
pub struct OverflowStack(pub [c_ulong; OVERFLOW_STACK_SIZE / core::mem::size_of::<c_ulong>()]);
pub static mut overflow_stack: OverflowStack = OverflowStack([0; OVERFLOW_STACK_SIZE / core::mem::size_of::<c_ulong>()]);

pub static mut kvm_stacktrace_info: KvmNvheStacktraceInfo = KvmNvheStacktraceInfo {
    stack_base: 0,
    overflow_stack_base: 0,
    fp: 0,
    pc: 0,
};

unsafe fn hyp_prepare_backtrace(fp: c_ulong, pc: c_ulong) {
    let stacktrace_info = this_cpu_ptr(&raw mut kvm_stacktrace_info);
    let params = this_cpu_ptr(&raw mut kvm_init_params);

    (*stacktrace_info).stack_base = (*params).stack_hyp_va - NVHE_STACK_SIZE;
    (*stacktrace_info).overflow_stack_base = this_cpu_ptr(&raw mut overflow_stack) as c_ulong;
    (*stacktrace_info).fp = fp;
    (*stacktrace_info).pc = pc;
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
pub static mut pkvm_stacktrace: [c_ulong; NVHE_STACKTRACE_SIZE / core::mem::size_of::<c_ulong>()] =
    [0; NVHE_STACKTRACE_SIZE / core::mem::size_of::<c_ulong>()];

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe fn stackinfo_get_overflow() -> StackInfo {
    let low = this_cpu_ptr(&raw mut overflow_stack) as c_ulong;
    StackInfo { low, high: low + OVERFLOW_STACK_SIZE }
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe fn stackinfo_get_hyp() -> StackInfo {
    let params = this_cpu_ptr(&raw mut kvm_init_params);
    let high = (*params).stack_hyp_va;
    StackInfo { low: high - NVHE_STACK_SIZE, high }
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe fn unwind_next(state: *mut UnwindState) -> c_int {
    unwind_next_frame_record(state)
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe fn unwind(state: *mut UnwindState, consume_entry: StackTraceConsumeFn, cookie: *mut c_void) {
    loop {
        if !consume_entry(cookie, (*state).pc) {
            break;
        }
        let ret = unwind_next(state);
        if ret < 0 {
            break;
        }
    }
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe extern "C" fn pkvm_save_backtrace_entry(arg: *mut c_void, where_: c_ulong) -> bool {
    let stacktrace = this_cpu_ptr(&raw mut pkvm_stacktrace);
    let idx = arg as *mut c_int;

    // Need 2 free slots: 1 for current entry and 1 for the delimiter.
    if *idx > (NVHE_STACKTRACE_SIZE / core::mem::size_of::<c_ulong>()) as c_int - 2 {
        return false;
    }

    (*stacktrace)[*idx as usize] = where_;
    *idx += 1;
    (*stacktrace)[*idx as usize] = 0;
    true
}

#[cfg(CONFIG_PKVM_STACKTRACE)]
unsafe fn pkvm_save_backtrace(fp: c_ulong, pc: c_ulong) {
    let mut stacks = [stackinfo_get_overflow(), stackinfo_get_hyp()];
    let mut state = UnwindState {
        stacks: stacks.as_mut_ptr(),
        nr_stacks: stacks.len(),
    };
    let mut idx: c_int = 0;

    kvm_nvhe_unwind_init(&mut state, fp, pc);
    unwind(&mut state, pkvm_save_backtrace_entry, &mut idx as *mut c_int as *mut c_void);
}

#[cfg(not(CONFIG_PKVM_STACKTRACE))]
unsafe fn pkvm_save_backtrace(_fp: c_ulong, _pc: c_ulong) {}

pub unsafe fn kvm_nvhe_prepare_backtrace(fp: c_ulong, pc: c_ulong) {
    if is_protected_kvm_enabled() {
        pkvm_save_backtrace(fp, pc);
    } else {
        hyp_prepare_backtrace(fp, pc);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
