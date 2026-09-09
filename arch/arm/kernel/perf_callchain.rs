// SPDX-License-Identifier: GPL-2.0
/*
 * ARM callchain support
 *
 * Copyright (C) 2009 picoChip Designs, Ltd., Jamie Iles
 * Copyright (C) 2010 ARM Ltd., Will Deacon <will.deacon@arm.com>
 *
 * This code is based on the ARM OProfile backtrace code.
 */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

/*
 * The registers we're interested in are at the end of the variable
 * length saved register structure. The fp points at the end of this
 * structure so the address of this struct is:
 * (struct frame_tail *)(xxx->fp)-1
 *
 * This code has been adapted from the ARM OProfile support.
 */
#[repr(C, packed)]
struct frame_tail {
    fp: *mut frame_tail,
    sp: ::core::ffi::c_ulong,
    lr: ::core::ffi::c_ulong,
}

// External kernel types and functions referenced by this implementation.
#[repr(C)]
pub struct perf_callchain_entry_ctx {
    pub nr: ::core::ffi::c_ulong,
    pub max_stack: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct pt_regs {
    pub ARM_pc: ::core::ffi::c_ulong,
    pub ARM_fp: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct stackframe {
    _private: [u8; 0],
}

extern "C" {
    static mut current: *mut task_struct;
    fn access_ok(addr: *const c_void, size: usize) -> bool;
    fn pagefault_disable();
    fn __copy_from_user_inatomic(to: *mut c_void, from: *const c_void, n: usize) -> ::core::ffi::c_ulong;
    fn pagefault_enable();
    fn perf_callchain_store(entry: *mut perf_callchain_entry_ctx, ip: ::core::ffi::c_ulong) -> i32;
    fn arm_get_current_stackframe(regs: *mut pt_regs, frame: *mut stackframe);
    fn walk_stackframe(
        frame: *mut stackframe,
        fn_: unsafe extern "C" fn(*mut c_void, ::core::ffi::c_ulong) -> bool,
        data: *mut c_void,
    );
}

#[repr(C)]
struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct task_struct {
    mm: *mut mm_struct,
}

/*
 * Get the return address for a single stackframe and return a pointer to the
 * next frame tail.
 */
unsafe fn user_backtrace(
    tail: *mut frame_tail,
    entry: *mut perf_callchain_entry_ctx,
) -> *mut frame_tail {
    let mut buftail = core::mem::MaybeUninit::<frame_tail>::uninit();
    let err: ::core::ffi::c_ulong;

    if !access_ok(tail as *const c_void, core::mem::size_of::<frame_tail>()) {
        return core::ptr::null_mut();
    }

    pagefault_disable();
    err = __copy_from_user_inatomic(
        buftail.as_mut_ptr() as *mut c_void,
        tail as *const c_void,
        core::mem::size_of::<frame_tail>(),
    );
    pagefault_enable();

    if err != 0 {
        return core::ptr::null_mut();
    }

    let buftail = buftail.assume_init();
    perf_callchain_store(entry, buftail.lr);

    /*
     * Frame pointers should strictly progress back up the stack
     * (towards higher addresses).
     */
    if tail.add(1) >= buftail.fp {
        return core::ptr::null_mut();
    }

    buftail.fp.sub(1)
}

pub unsafe extern "C" fn perf_callchain_user(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    let mut tail: *mut frame_tail;

    perf_callchain_store(entry, (*regs).ARM_pc);

    if (*current).mm.is_null() {
        return;
    }

    tail = ((*regs).ARM_fp as *mut frame_tail).sub(1);

    while (*entry).nr < (*entry).max_stack
        && !tail.is_null()
        && ((tail as usize) & 0x3) == 0
    {
        tail = user_backtrace(tail, entry);
    }
}

/*
 * Gets called by walk_stackframe() for every stackframe. This will be called
 * whist unwinding the stackframe and is like a subroutine return so we use
 * the PC.
 */
unsafe extern "C" fn callchain_trace(data: *mut c_void, pc: ::core::ffi::c_ulong) -> bool {
    let entry = data as *mut perf_callchain_entry_ctx;
    perf_callchain_store(entry, pc) == 0
}

pub unsafe extern "C" fn perf_callchain_kernel(
    entry: *mut perf_callchain_entry_ctx,
    regs: *mut pt_regs,
) {
    let mut fr = core::mem::MaybeUninit::<stackframe>::uninit();

    arm_get_current_stackframe(regs, fr.as_mut_ptr());
    walk_stackframe(
        fr.as_mut_ptr(),
        callchain_trace,
        entry as *mut c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
