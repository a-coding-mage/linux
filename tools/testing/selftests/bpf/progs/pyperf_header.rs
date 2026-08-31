// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
//
// Translated from testing/selftests/bpf/progs/pyperf.h.
// C include dependencies intentionally remain external: linux/sched.h,
// linux/ptrace.h, linux/bpf.h, bpf_helpers.h, bpf_misc.h, and
// bpf_compiler.h provide helper declarations, BPF constants, SEC-like
// placement, STACK_MAX_LEN, and compiler attributes in the original source.

use core::ffi::c_void;
use core::mem::{offset_of, size_of};
use core::ptr;

pub const FUNCTION_NAME_LEN: usize = 64;
pub const FILE_NAME_LEN: usize = 128;
pub const TASK_COMM_LEN: usize = 16;

pub type pid_t = i32;

#[repr(C)]
pub struct OffsetConfig {
    pub PyThreadState_frame: i32,
    pub PyThreadState_thread: i32,
    pub PyFrameObject_back: i32,
    pub PyFrameObject_code: i32,
    pub PyFrameObject_lineno: i32,
    pub PyCodeObject_filename: i32,
    pub PyCodeObject_name: i32,
    pub String_data: i32,
    pub String_size: i32,
}

#[repr(C)]
pub struct PidData {
    pub current_state_addr: usize,
    pub tls_key_addr: usize,
    pub offsets: OffsetConfig,
    pub use_tls: bool,
}

#[repr(C)]
pub struct Stats {
    pub success: u32,
}

#[repr(C)]
pub struct Symbol {
    pub name: [i8; FUNCTION_NAME_LEN],
    pub file: [i8; FILE_NAME_LEN],
}

#[repr(C)]
pub struct Event {
    pub pid: u32,
    pub tid: u32,
    pub comm: [i8; TASK_COMM_LEN],
    pub kernel_stack_id: i32,
    pub user_stack_id: i32,
    pub thread_current: bool,
    pub pthread_match: bool,
    pub stack_complete: bool,
    pub stack_len: i16,
    pub stack: [i32; STACK_MAX_LEN],

    pub has_meta: i32,
    pub metadata: i32,
    pub dummy_safeguard: i8,
}

#[repr(C)]
pub struct FrameData {
    pub f_back: *mut c_void,      // PyFrameObject.f_back, previous frame
    pub f_code: *mut c_void,      // PyFrameObject.f_code, pointer to PyCodeObject
    pub co_filename: *mut c_void, // PyCodeObject.co_filename
    pub co_name: *mut c_void,     // PyCodeObject.co_name
}

#[repr(C)]
pub struct bpf_raw_tracepoint_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut pidmap: c_void;
    pub static mut eventmap: c_void;
    pub static mut symbolmap: c_void;
    pub static mut statsmap: c_void;
    pub static mut perfmap: c_void;
    pub static mut stackmap: c_void;

    pub static STACK_MAX_LEN: usize;
    pub static BPF_F_USER_STACK: u64;

    pub fn bpf_probe_read_user(dst: *mut c_void, size: usize, unsafe_ptr: *const c_void) -> i64;
    pub fn bpf_probe_read_user_str(dst: *mut c_void, size: usize, unsafe_ptr: *const c_void) -> i64;
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    pub fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> i64;
    pub fn bpf_get_current_comm(buf: *mut c_void, size: usize) -> i64;
    pub fn bpf_get_stackid(ctx: *mut bpf_raw_tracepoint_args, map: *mut c_void, flags: u64) -> i64;
    pub fn bpf_get_current_task() -> *mut c_void;
    pub fn bpf_get_smp_processor_id() -> i32;
    pub fn bpf_perf_event_output(
        ctx: *mut bpf_raw_tracepoint_args,
        map: *mut c_void,
        flags: u64,
        data: *mut c_void,
        size: u64,
    ) -> i64;

    #[cfg(USE_BPF_LOOP)]
    pub fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(u32, *mut process_frame_ctx) -> i32,
        callback_ctx: *mut process_frame_ctx,
        flags: u64,
    ) -> i64;
}

#[inline(always)]
pub unsafe fn get_thread_state(tls_base: *mut c_void, pidData: *mut PidData) -> *mut c_void {
    let mut thread_state: *mut c_void = ptr::null_mut();
    let mut key: i32 = 0;

    bpf_probe_read_user(
        &mut key as *mut _ as *mut c_void,
        size_of::<i32>(),
        (*pidData).tls_key_addr as *const c_void,
    );
    bpf_probe_read_user(
        &mut thread_state as *mut _ as *mut c_void,
        size_of::<*mut c_void>(),
        (tls_base as usize + 0x310usize + (key as usize).wrapping_mul(0x10usize) + 0x08usize)
            as *const c_void,
    );
    thread_state
}

#[no_mangle]
pub unsafe extern "C" fn __get_frame_data(
    frame_ptr_: isize,
    pidData: *mut PidData,
    frame: *mut FrameData,
    symbol: *mut Symbol,
) -> bool {
    let frame_ptr = frame_ptr_ as *mut c_void;

    // read data from PyFrameObject
    bpf_probe_read_user(
        &mut (*frame).f_back as *mut _ as *mut c_void,
        size_of::<*mut c_void>(),
        (frame_ptr as usize + (*pidData).offsets.PyFrameObject_back as usize) as *const c_void,
    );
    bpf_probe_read_user(
        &mut (*frame).f_code as *mut _ as *mut c_void,
        size_of::<*mut c_void>(),
        (frame_ptr as usize + (*pidData).offsets.PyFrameObject_code as usize) as *const c_void,
    );

    // read data from PyCodeObject
    if (*frame).f_code.is_null() {
        return false;
    }
    bpf_probe_read_user(
        &mut (*frame).co_filename as *mut _ as *mut c_void,
        size_of::<*mut c_void>(),
        ((*frame).f_code as usize + (*pidData).offsets.PyCodeObject_filename as usize)
            as *const c_void,
    );
    bpf_probe_read_user(
        &mut (*frame).co_name as *mut _ as *mut c_void,
        size_of::<*mut c_void>(),
        ((*frame).f_code as usize + (*pidData).offsets.PyCodeObject_name as usize)
            as *const c_void,
    );
    // read actual names into symbol
    if !(*frame).co_filename.is_null() {
        bpf_probe_read_user_str(
            (*symbol).file.as_mut_ptr() as *mut c_void,
            size_of::<[i8; FILE_NAME_LEN]>(),
            ((*frame).co_filename as usize + (*pidData).offsets.String_data as usize)
                as *const c_void,
        );
    }
    if !(*frame).co_name.is_null() {
        bpf_probe_read_user_str(
            (*symbol).name.as_mut_ptr() as *mut c_void,
            size_of::<[i8; FUNCTION_NAME_LEN]>(),
            ((*frame).co_name as usize + (*pidData).offsets.String_data as usize) as *const c_void,
        );
    }
    true
}

#[inline(always)]
pub unsafe fn get_frame_data(
    frame_ptr: *mut c_void,
    pidData: *mut PidData,
    frame: *mut FrameData,
    symbol: *mut Symbol,
) -> bool {
    __get_frame_data(frame_ptr as isize, pidData, frame, symbol)
}

// Original BPF map definitions:
// pidmap: BPF_MAP_TYPE_HASH, max_entries 1, key int, value PidData
// eventmap: BPF_MAP_TYPE_HASH, max_entries 1, key int, value Event
// symbolmap: BPF_MAP_TYPE_HASH, max_entries 1, key Symbol, value int
// statsmap: BPF_MAP_TYPE_ARRAY, max_entries 1, key int, value Stats
// perfmap: BPF_MAP_TYPE_PERF_EVENT_ARRAY, max_entries 32, key_size sizeof(int), value_size sizeof(int)
// stackmap: BPF_MAP_TYPE_STACK_TRACE, max_entries 1000, key_size sizeof(int),
//           value_size sizeof(long long) * 127

#[cfg(USE_BPF_LOOP)]
#[repr(C)]
pub struct process_frame_ctx {
    pub cur_cpu: i32,
    pub symbol_counter: *mut i32,
    pub frame_ptr: *mut c_void,
    pub frame: *mut FrameData,
    pub pidData: *mut PidData,
    pub sym: *mut Symbol,
    pub event: *mut Event,
    pub done: bool,
}

#[cfg(USE_BPF_LOOP)]
pub unsafe extern "C" fn process_frame_callback(i: u32, ctx: *mut process_frame_ctx) -> i32 {
    let zero: i32 = 0;
    let mut frame_ptr = (*ctx).frame_ptr;
    let pidData = (*ctx).pidData;
    let frame = (*ctx).frame;
    let symbol_counter = (*ctx).symbol_counter;
    let cur_cpu = (*ctx).cur_cpu;
    let event = (*ctx).event;
    let sym = (*ctx).sym;

    if !frame_ptr.is_null() && get_frame_data(frame_ptr, pidData, frame, sym) {
        let new_symbol_id = (*symbol_counter).wrapping_mul(64).wrapping_add(cur_cpu);
        let mut symbol_id = bpf_map_lookup_elem(&mut symbolmap, sym as *const c_void) as *mut i32;

        if symbol_id.is_null() {
            bpf_map_update_elem(
                &mut symbolmap,
                sym as *const c_void,
                &zero as *const _ as *const c_void,
                0,
            );
            symbol_id = bpf_map_lookup_elem(&mut symbolmap, sym as *const c_void) as *mut i32;
            if symbol_id.is_null() {
                (*ctx).done = true;
                return 1;
            }
        }
        if *symbol_id == new_symbol_id {
            *symbol_counter = (*symbol_counter).wrapping_add(1);
        }

        // barrier_var(i) in the original C constrains verifier/compiler reasoning.
        if i as usize >= STACK_MAX_LEN {
            return 1;
        }

        (*event).stack[i as usize] = *symbol_id;
        (*event).stack_len = i.wrapping_add(1) as i16;
        frame_ptr = (*frame).f_back;
        (*ctx).frame_ptr = frame_ptr;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __on_event(ctx: *mut bpf_raw_tracepoint_args) -> i32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as pid_t;
    let pidData = bpf_map_lookup_elem(
        &mut pidmap,
        &pid as *const _ as *const c_void,
    ) as *mut PidData;
    if pidData.is_null() {
        return 0;
    }

    let zero: i32 = 0;
    let event = bpf_map_lookup_elem(
        &mut eventmap,
        &zero as *const _ as *const c_void,
    ) as *mut Event;
    if event.is_null() {
        return 0;
    }

    (*event).pid = pid as u32;

    (*event).tid = pid_tgid as pid_t as u32;
    bpf_get_current_comm(
        (*event).comm.as_mut_ptr() as *mut c_void,
        size_of::<[i8; TASK_COMM_LEN]>(),
    );

    (*event).user_stack_id = bpf_get_stackid(ctx, &mut stackmap, BPF_F_USER_STACK) as i32;
    (*event).kernel_stack_id = bpf_get_stackid(ctx, &mut stackmap, 0) as i32;

    let mut thread_state_current: *mut c_void = ptr::null_mut();
    bpf_probe_read_user(
        &mut thread_state_current as *mut _ as *mut c_void,
        size_of::<*mut c_void>(),
        (*pidData).current_state_addr as *const c_void,
    );

    let task = bpf_get_current_task() as *mut task_struct;
    let tls_base = task as *mut c_void;

    let mut thread_state = if (*pidData).use_tls {
        get_thread_state(tls_base, pidData)
    } else {
        thread_state_current
    };
    (*event).thread_current = thread_state == thread_state_current;

    if (*pidData).use_tls {
        let mut pthread_created: u64 = 0;
        let mut pthread_self: u64 = 0;
        bpf_probe_read_user(
            &mut pthread_self as *mut _ as *mut c_void,
            size_of::<u64>(),
            (tls_base as usize + 0x10usize) as *const c_void,
        );

        bpf_probe_read_user(
            &mut pthread_created as *mut _ as *mut c_void,
            size_of::<u64>(),
            (thread_state as usize + (*pidData).offsets.PyThreadState_thread as usize)
                as *const c_void,
        );
        (*event).pthread_match = pthread_created == pthread_self;
    } else {
        (*event).pthread_match = true;
    }

    if (*event).pthread_match || !(*pidData).use_tls {
        let mut frame_ptr: *mut c_void = ptr::null_mut();
        let mut frame = FrameData {
            f_back: ptr::null_mut(),
            f_code: ptr::null_mut(),
            co_filename: ptr::null_mut(),
            co_name: ptr::null_mut(),
        };
        let mut sym = Symbol {
            name: [0; FUNCTION_NAME_LEN],
            file: [0; FILE_NAME_LEN],
        };
        let cur_cpu = bpf_get_smp_processor_id();

        bpf_probe_read_user(
            &mut frame_ptr as *mut _ as *mut c_void,
            size_of::<*mut c_void>(),
            (thread_state as usize + (*pidData).offsets.PyThreadState_frame as usize)
                as *const c_void,
        );

        let symbol_counter = bpf_map_lookup_elem(
            &mut symbolmap,
            &sym as *const _ as *const c_void,
        ) as *mut i32;
        if symbol_counter.is_null() {
            return 0;
        }

        #[cfg(USE_BPF_LOOP)]
        {
            let mut loop_ctx = process_frame_ctx {
                cur_cpu,
                symbol_counter,
                frame_ptr,
                frame: &mut frame,
                pidData,
                sym: &mut sym,
                event,
                done: false,
            };

            bpf_loop(
                STACK_MAX_LEN as u32,
                process_frame_callback,
                &mut loop_ctx,
                0,
            );
            if loop_ctx.done {
                return 0;
            }
            frame_ptr = loop_ctx.frame_ptr;
        }

        #[cfg(not(USE_BPF_LOOP))]
        {
            // Original C supports USE_ITER, NO_UNROLL, and UNROLL_COUNT pragma choices here.
            // This Rust translation preserves the plain loop body and leaves unroll policy to
            // the surrounding BPF build configuration.
            let mut i: usize = 0;
            while i < STACK_MAX_LEN {
                if !frame_ptr.is_null() && get_frame_data(frame_ptr, pidData, &mut frame, &mut sym) {
                    let new_symbol_id = (*symbol_counter).wrapping_mul(64).wrapping_add(cur_cpu);
                    let mut symbol_id = bpf_map_lookup_elem(
                        &mut symbolmap,
                        &sym as *const _ as *const c_void,
                    ) as *mut i32;
                    if symbol_id.is_null() {
                        bpf_map_update_elem(
                            &mut symbolmap,
                            &sym as *const _ as *const c_void,
                            &zero as *const _ as *const c_void,
                            0,
                        );
                        symbol_id = bpf_map_lookup_elem(
                            &mut symbolmap,
                            &sym as *const _ as *const c_void,
                        ) as *mut i32;
                        if symbol_id.is_null() {
                            return 0;
                        }
                    }
                    if *symbol_id == new_symbol_id {
                        *symbol_counter = (*symbol_counter).wrapping_add(1);
                    }
                    (*event).stack[i] = *symbol_id;
                    (*event).stack_len = i.wrapping_add(1) as i16;
                    frame_ptr = frame.f_back;
                }
                i = i.wrapping_add(1);
            }
        }
        (*event).stack_complete = frame_ptr.is_null();
    } else {
        (*event).stack_complete = true;
    }

    let stats = bpf_map_lookup_elem(
        &mut statsmap,
        &zero as *const _ as *const c_void,
    ) as *mut Stats;
    if !stats.is_null() {
        (*stats).success = (*stats).success.wrapping_add(1);
    }

    (*event).has_meta = 0;
    bpf_perf_event_output(
        ctx,
        &mut perfmap,
        0,
        event as *mut c_void,
        offset_of!(Event, metadata) as u64,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn on_event(ctx: *mut bpf_raw_tracepoint_args) -> i32 {
    let mut ret: i32 = 0;
    ret |= __on_event(ctx);
    ret |= __on_event(ctx);
    ret |= __on_event(ctx);
    ret |= __on_event(ctx);
    ret |= __on_event(ctx);
    ret
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";
