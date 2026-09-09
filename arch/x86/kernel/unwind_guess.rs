// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux kernel headers:
// linux/sched.h, linux/ftrace.h, asm/ptrace.h, asm/bitops.h,
// asm/stacktrace.h, and asm/unwind.h.

use core::mem::{align_of, size_of};
use core::ptr;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stack_info {
    pub end: *mut usize,
    pub next_sp: *mut usize,
}

#[repr(C)]
pub struct unwind_state {
    pub task: *mut task_struct,
    pub sp: *mut usize,
    pub stack_info: stack_info,
    pub stack_mask: usize,
}

extern "C" {
    fn unwind_done(state: *const unwind_state) -> bool;
    fn unwind_recover_ret_addr(
        state: *mut unwind_state,
        addr: usize,
        sp: *mut usize,
    ) -> usize;
    fn __kernel_text_address(addr: usize) -> bool;
    fn get_stack_info(
        sp: *mut usize,
        task: *mut task_struct,
        info: *mut stack_info,
        mask: *mut usize,
    ) -> bool;
    fn on_stack(info: *const stack_info, addr: *mut usize, size: usize) -> bool;
}

#[inline]
unsafe fn read_once_nocheck<T: Copy>(src: *const T) -> T {
    ptr::read_volatile(src)
}

#[inline]
unsafe fn ptr_align<T>(p: *mut T, align: usize) -> *mut T {
    let value = p as usize;
    ((value + align - 1) & !(align - 1)) as *mut T
}

pub unsafe fn unwind_get_return_address(state: *mut unwind_state) -> usize {
    let addr: usize;

    if unwind_done(state) {
        return 0;
    }

    addr = read_once_nocheck((*state).sp as *const usize);

    unwind_recover_ret_addr(state, addr, (*state).sp)
}

// EXPORT_SYMBOL_GPL(unwind_get_return_address);

pub unsafe fn unwind_get_return_address_ptr(_state: *mut unwind_state) -> *mut usize {
    ptr::null_mut()
}

pub unsafe fn unwind_next_frame(state: *mut unwind_state) -> bool {
    let info: *mut stack_info = &mut (*state).stack_info;

    if unwind_done(state) {
        return false;
    }

    loop {
        while {
            (*state).sp = (*state).sp.add(1);
            (*state).sp < (*info).end
        } {
            let addr: usize = read_once_nocheck((*state).sp as *const usize);

            if __kernel_text_address(addr) {
                return true;
            }
        }

        (*state).sp = ptr_align((*info).next_sp, size_of::<usize>());

        if get_stack_info(
            (*state).sp,
            (*state).task,
            info,
            &mut (*state).stack_mask,
        ) {
            continue;
        }
        break;
    }

    false
}

// EXPORT_SYMBOL_GPL(unwind_next_frame);

pub unsafe fn __unwind_start(
    state: *mut unwind_state,
    task: *mut task_struct,
    regs: *mut pt_regs,
    first_frame: *mut usize,
) {
    let _ = regs;
    ptr::write_bytes(state as *mut u8, 0, size_of::<unwind_state>());

    (*state).task = task;
    (*state).sp = ptr_align(first_frame, align_of::<usize>());

    get_stack_info(
        first_frame,
        (*state).task,
        &mut (*state).stack_info,
        &mut (*state).stack_mask,
    );

    /*
     * The caller can provide the address of the first frame directly
     * (first_frame) or indirectly (regs->sp) to indicate which stack frame
     * to start unwinding at.  Skip ahead until we reach it.
     */
    if !unwind_done(state)
        && (!on_stack(&(*state).stack_info, first_frame, size_of::<usize>())
            || !__kernel_text_address(read_once_nocheck(first_frame as *const usize)))
    {
        unwind_next_frame(state);
    }
}

// EXPORT_SYMBOL_GPL(__unwind_start);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
