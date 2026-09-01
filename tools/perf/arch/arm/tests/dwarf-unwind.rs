// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/arch/arm/tests/dwarf-unwind.c.
// C includes translated as external dependencies from:
// string.h, perf_regs.h, thread.h, map.h, maps.h, event.h, debug.h,
// tests/tests.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

type u64 = u64;

const STACK_SIZE: usize = 8192;

// Constants supplied by perf_regs.h in the original C translation unit.
extern "C" {
    static PERF_REG_ARM_SP: c_int;
    static PERF_REGS_MAX: c_int;
    static PERF_SAMPLE_REGS_ABI: u64;
    static PERF_REGS_MASK: u64;
}

#[repr(C)]
pub struct stack_dump {
    pub data: *mut c_char,
    pub size: u64,
}

#[repr(C)]
pub struct regs_dump {
    pub abi: u64,
    pub mask: u64,
    pub regs: *mut u64,
}

#[repr(C)]
pub struct perf_sample {
    pub user_stack: stack_dump,
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn pr_debug(fmt: *const c_char, ...);

    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__find(maps: *mut maps, addr: u64) -> *mut map;
    fn map__end(map: *mut map) -> u64;
    fn perf_sample__user_regs(sample: *mut perf_sample) -> *mut regs_dump;
    fn perf_regs_load(regs: *mut u64);
}

unsafe fn sample_ustack(sample: *mut perf_sample, thread: *mut thread, regs: *mut u64) -> c_int {
    let stack: *mut stack_dump = &mut (*sample).user_stack;
    let map: *mut map;
    let sp: c_ulong;
    let mut stack_size: u64;
    let buf: *mut u64;

    buf = malloc(STACK_SIZE) as *mut u64;
    if buf.is_null() {
        pr_debug(b"failed to allocate sample uregs data\n\0".as_ptr() as *const c_char);
        return -1;
    }

    sp = *regs.add(PERF_REG_ARM_SP as usize) as c_ulong;

    map = maps__find(thread__maps(thread), sp as u64);
    if map.is_null() {
        pr_debug(b"failed to get stack map\n\0".as_ptr() as *const c_char);
        free(buf as *mut c_void);
        return -1;
    }

    stack_size = map__end(map).wrapping_sub(sp as u64);
    stack_size = if stack_size > STACK_SIZE as u64 {
        STACK_SIZE as u64
    } else {
        stack_size
    };

    memcpy(buf as *mut c_void, sp as *const c_void, stack_size as usize);
    (*stack).data = buf as *mut c_char;
    (*stack).size = stack_size;
    0
}

#[no_mangle]
pub unsafe extern "C" fn test__arch_unwind_sample(
    sample: *mut perf_sample,
    thread: *mut thread,
) -> c_int {
    let regs: *mut regs_dump = perf_sample__user_regs(sample);
    let buf: *mut u64;

    buf = calloc(1, core::mem::size_of::<u64>() * PERF_REGS_MAX as usize) as *mut u64;
    if buf.is_null() {
        pr_debug(b"failed to allocate sample uregs data\n\0".as_ptr() as *const c_char);
        return -1;
    }

    perf_regs_load(buf);
    (*regs).abi = PERF_SAMPLE_REGS_ABI;
    (*regs).regs = buf;
    (*regs).mask = PERF_REGS_MASK;

    sample_ustack(sample, thread, buf)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
