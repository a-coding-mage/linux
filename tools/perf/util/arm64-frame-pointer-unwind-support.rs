// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/arm64-frame-pointer-unwind-support.c.
// C includes removed; declarations below refer to external perf dependencies.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};

type u64 = u64;
type size_t = usize;

// From "../../arch/arm64/include/uapi/asm/perf_regs.h" through
// `#define perf_event_arm_regs perf_event_arm64_regs`.
extern "C" {
    static PERF_REG_ARM64_LR: c_int;
    static PERF_REG_ARM64_PC: c_int;
    static PERF_REG_ARM64_SP: c_int;
}

extern "C" {
    static mut callchain_param: callchain_param;

    fn perf_sample__user_regs(sample: *mut perf_sample) -> *mut regs_dump;
    fn unwind__get_entries(
        cb: unwind_entry_cb_t,
        arg: *mut c_void,
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
    ) -> c_int;
}

type unwind_entry_cb_t = Option<unsafe extern "C" fn(entry: *mut unwind_entry, arg: *mut c_void) -> c_int>;

const CALLCHAIN_FP: c_int = 1;
const ORDER_CALLER: c_int = 0;

#[repr(C)]
pub struct record_opts {
    pub sample_user_regs: u64,
}

#[repr(C)]
pub struct regs_dump {
    pub regs: *mut u64,
    pub mask: u64,
    pub cache_mask: u64,
    pub cache_regs: *mut u64,
}

#[repr(C)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub callchain: *mut ip_callchain,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct unwind_entry {
    pub ip: u64,
}

#[repr(C)]
pub struct callchain_param {
    pub record_mode: c_int,
    pub order: c_int,
}

#[repr(C)]
struct entries {
    stack: [u64; 2],
    length: size_t,
}

#[inline]
fn SMPL_REG_MASK(b: c_int) -> u64 {
    1u64 << (b as u32)
}

#[no_mangle]
pub unsafe extern "C" fn add_leaf_frame_caller_opts_aarch64(opts: *mut record_opts) {
    (*opts).sample_user_regs |= SMPL_REG_MASK(PERF_REG_ARM64_LR);
}

unsafe fn get_leaf_frame_caller_enabled(sample: *mut perf_sample) -> bool {
    let regs: *mut regs_dump;

    if callchain_param.record_mode != CALLCHAIN_FP {
        return false;
    }

    regs = perf_sample__user_regs(sample);
    !(*regs).regs.is_null() && ((*regs).mask & SMPL_REG_MASK(PERF_REG_ARM64_LR)) != 0
}

unsafe extern "C" fn add_entry(entry: *mut unwind_entry, arg: *mut c_void) -> c_int {
    let entries = arg as *mut entries;

    (*entries).stack[(*entries).length] = (*entry).ip;
    (*entries).length += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_leaf_frame_caller_aarch64(
    sample: *mut perf_sample,
    thread: *mut thread,
    usr_idx: c_int,
) -> u64 {
    let ret: c_int;
    let mut entries = entries {
        stack: [0; 2],
        length: 0,
    };
    let old_regs: regs_dump;
    let regs: *mut regs_dump;

    if !get_leaf_frame_caller_enabled(sample) {
        return 0;
    }

    /*
     * If PC and SP are not recorded, get the value of PC from the stack
     * and set its mask. SP is not used when doing the unwinding but it
     * still needs to be set to prevent failures.
     */
    regs = perf_sample__user_regs(sample);
    old_regs = core::ptr::read(regs);
    if ((*regs).mask & SMPL_REG_MASK(PERF_REG_ARM64_PC)) == 0 {
        (*regs).cache_mask |= SMPL_REG_MASK(PERF_REG_ARM64_PC);
        (*regs).cache_regs.add(PERF_REG_ARM64_PC as usize).write(
            (*(*sample).callchain)
                .ips
                .as_ptr()
                .add((usr_idx + 1) as usize)
                .read(),
        );
    }

    if ((*regs).mask & SMPL_REG_MASK(PERF_REG_ARM64_SP)) == 0 {
        (*regs).cache_mask |= SMPL_REG_MASK(PERF_REG_ARM64_SP);
        (*regs)
            .cache_regs
            .add(PERF_REG_ARM64_SP as usize)
            .write(0);
    }

    ret = unwind__get_entries(
        Some(add_entry),
        &mut entries as *mut entries as *mut c_void,
        thread,
        sample,
        2,
        true,
    );
    core::ptr::write(regs, old_regs);

    if ret != 0 || entries.length != 2 {
        return ret as u64;
    }

    if callchain_param.order == ORDER_CALLER {
        entries.stack[0]
    } else {
        entries.stack[1]
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
