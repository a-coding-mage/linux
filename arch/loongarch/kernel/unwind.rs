// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/ftrace.h, and asm/unwind.h

#[repr(C)]
pub struct stack_info {
    pub end: usize,
    pub next_sp: usize,
}

#[repr(C)]
pub struct unwind_state {
    pub stack_info: stack_info,
    pub sp: usize,
    pub pc: usize,
    pub task: *mut core::ffi::c_void,
}

extern "C" {
    fn unwind_done(state: *mut unwind_state) -> bool;
    fn unwind_graph_addr(state: *mut unwind_state, addr: usize, sp: usize) -> usize;
    fn __kernel_text_address(addr: usize) -> bool;
    fn get_stack_info(
        sp: usize,
        task: *mut core::ffi::c_void,
        info: *mut stack_info,
    ) -> bool;
}

pub unsafe fn default_next_frame(state: *mut unwind_state) -> bool {
    let info: *mut stack_info = &mut (*state).stack_info;
    let mut addr: usize;

    if unwind_done(state) {
        return false;
    }

    loop {
        while {
            (*state).sp = (*state).sp.wrapping_add(core::mem::size_of::<usize>());
            (*state).sp < (*info).end
        } {
            addr = *( (*state).sp as *const usize );
            (*state).pc = unwind_graph_addr(state, addr, (*state).sp.wrapping_add(8));
            if __kernel_text_address((*state).pc) {
                return true;
            }
        }

        (*state).sp = (*info).next_sp;

        if get_stack_info((*state).sp, (*state).task, info) {
            break;
        }
    }

    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
