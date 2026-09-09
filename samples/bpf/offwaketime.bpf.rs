/* Copyright (c) 2016 Facebook
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Dependencies supplied by vmlinux.h and the BPF helper headers are external.

#[allow(non_upper_case_globals)]
const PERF_MAX_STACK_DEPTH: u32 = 127;
const MINBLOCK_US: u64 = 1;
const MAX_ENTRIES: u32 = 10000;
const STACKID_FLAGS: u64 = 0 | BPF_F_FAST_STACK_CMP;

#[repr(C)]
pub struct key_t {
    pub waker: [core::ffi::c_char; TASK_COMM_LEN as usize],
    pub target: [core::ffi::c_char; TASK_COMM_LEN as usize],
    pub wret: u32,
    pub tret: u32,
}

// BPF map declarations corresponding to the C SEC(".maps") definitions.
extern "C" { pub static mut counts: CountsMap; }
extern "C" { pub static mut start: StartMap; }

#[repr(C)]
pub struct wokeby_t {
    pub name: [core::ffi::c_char; TASK_COMM_LEN as usize],
    pub ret: u32,
}

extern "C" { pub static mut wokeby: WokebyMap; }
extern "C" { pub static mut stackmap: StackTraceMap; }

// The concrete map types and BPF constants are supplied by the external BPF environment.
extern "C" {
    static mut BPF_F_FAST_STACK_CMP: u64;
    fn bpf_get_current_comm(buf: *mut core::ffi::c_void, size: u32) -> i64;
    fn bpf_get_stackid(ctx: *mut core::ffi::c_void, map: *mut core::ffi::c_void, flags: u64) -> i64;
    fn bpf_map_update_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void, value: *const core::ffi::c_void, flags: u64) -> i64;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_get_current_pid_tgid() -> u32;
}

#[repr(C)]
pub struct CountsMap { _private: [u8; 0] }
#[repr(C)]
pub struct StartMap { _private: [u8; 0] }
#[repr(C)]
pub struct WokebyMap { _private: [u8; 0] }
#[repr(C)]
pub struct StackTraceMap { _private: [u8; 0] }

#[no_mangle]
pub unsafe extern "C" fn waker(ctx: *mut pt_regs) -> i32 {
    let p = PT_REGS_PARM1_CORE(ctx) as *mut task_struct;
    let pid: u32 = BPF_CORE_READ(p, pid);
    let mut woke = core::mem::MaybeUninit::<wokeby_t>::uninit();
    let woke = woke.as_mut_ptr();

    bpf_get_current_comm((*woke).name.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&(*woke).name) as u32);
    (*woke).ret = bpf_get_stackid(ctx as *mut core::ffi::c_void, &mut stackmap as *mut _ as *mut core::ffi::c_void, STACKID_FLAGS) as u32;
    bpf_map_update_elem(&mut wokeby as *mut _ as *mut core::ffi::c_void, &pid as *const _ as *const core::ffi::c_void, woke as *const _ as *const core::ffi::c_void, BPF_ANY);
    0
}

#[inline]
unsafe fn update_counts(ctx: *mut core::ffi::c_void, pid: u32, delta: u64) -> i32 {
    let mut key = core::mem::MaybeUninit::<key_t>::zeroed().assume_init();
    let mut zero: u64 = 0;
    bpf_get_current_comm(key.target.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&key.target) as u32);
    key.tret = bpf_get_stackid(ctx, &mut stackmap as *mut _ as *mut core::ffi::c_void, STACKID_FLAGS) as u32;
    key.wret = 0;

    let woke = bpf_map_lookup_elem(&mut wokeby as *mut _ as *mut core::ffi::c_void, &pid as *const _ as *const core::ffi::c_void) as *mut wokeby_t;
    if !woke.is_null() {
        key.wret = (*woke).ret;
        core::ptr::copy_nonoverlapping((*woke).name.as_ptr(), key.waker.as_mut_ptr(), key.waker.len());
        bpf_map_delete_elem(&mut wokeby as *mut _ as *mut core::ffi::c_void, &pid as *const _ as *const core::ffi::c_void);
    }

    let mut val = bpf_map_lookup_elem(&mut counts as *mut _ as *mut core::ffi::c_void, &key as *const _ as *const core::ffi::c_void) as *mut u64;
    if val.is_null() {
        bpf_map_update_elem(&mut counts as *mut _ as *mut core::ffi::c_void, &key as *const _ as *const core::ffi::c_void, &zero as *const _ as *const core::ffi::c_void, BPF_NOEXIST);
        val = bpf_map_lookup_elem(&mut counts as *mut _ as *mut core::ffi::c_void, &key as *const _ as *const core::ffi::c_void) as *mut u64;
        if val.is_null() { return 0; }
    }
    *val = (*val).wrapping_add(delta);
    0
}

#[no_mangle]
pub unsafe extern "C" fn oncpu(ctx: *mut trace_event_raw_sched_switch) -> i32 {
    // record previous thread sleep time
    let mut pid: u32 = (*ctx).prev_pid;
    let ts = bpf_ktime_get_ns();
    bpf_map_update_elem(&mut start as *mut _ as *mut core::ffi::c_void, &pid as *const _ as *const core::ffi::c_void, &ts as *const _ as *const core::ffi::c_void, BPF_ANY);
    // calculate current thread's delta time
    pid = bpf_get_current_pid_tgid();
    let tsp = bpf_map_lookup_elem(&mut start as *mut _ as *mut core::ffi::c_void, &pid as *const _ as *const core::ffi::c_void) as *mut u64;
    if tsp.is_null() { return 0; }
    let mut delta = bpf_ktime_get_ns().wrapping_sub(*tsp);
    bpf_map_delete_elem(&mut start as *mut _ as *mut core::ffi::c_void, &pid as *const _ as *const core::ffi::c_void);
    delta /= 1000;
    if delta < MINBLOCK_US { return 0; }
    update_counts(ctx as *mut core::ffi::c_void, pid, delta)
}

#[no_mangle]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as _, b'P' as _, b'L' as _, 0];
#[no_mangle]
pub static mut _version: u32 = LINUX_VERSION_CODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
