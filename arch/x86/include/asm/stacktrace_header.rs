/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 2000, 2001, 2002 Andi Kleen, SuSE Labs
 */

/* Dependencies supplied by the surrounding kernel translation. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN,
    STACK_TYPE_TASK,
    STACK_TYPE_IRQ,
    STACK_TYPE_SOFTIRQ,
    STACK_TYPE_ENTRY,
    STACK_TYPE_EXCEPTION,
    STACK_TYPE_EXCEPTION_LAST = STACK_TYPE_EXCEPTION as isize + N_EXCEPTION_STACKS as isize - 1,
}

#[repr(C)]
pub struct stack_info {
    pub type_: stack_type,
    pub begin: *mut core::ffi::c_ulong,
    pub end: *mut core::ffi::c_ulong,
    pub next_sp: *mut core::ffi::c_ulong,
}

extern "C" {
    pub fn in_task_stack(
        stack: *mut core::ffi::c_ulong,
        task: *mut task_struct,
        info: *mut stack_info,
    ) -> bool;
    pub fn in_entry_stack(stack: *mut core::ffi::c_ulong, info: *mut stack_info) -> bool;
    pub fn get_stack_info(
        stack: *mut core::ffi::c_ulong,
        task: *mut task_struct,
        info: *mut stack_info,
        visit_mask: *mut core::ffi::c_ulong,
    ) -> core::ffi::c_int;
    pub fn get_stack_info_noinstr(
        stack: *mut core::ffi::c_ulong,
        task: *mut task_struct,
        info: *mut stack_info,
    ) -> bool;
    pub fn stack_type_name(type_: stack_type) -> *const core::ffi::c_char;
    pub fn show_opcodes(regs: *mut pt_regs, loglvl: *const core::ffi::c_char);
    pub fn show_ip(regs: *mut pt_regs, loglvl: *const core::ffi::c_char);
}

#[inline(always)]
pub unsafe fn get_stack_guard_info(
    stack: *mut core::ffi::c_ulong,
    info: *mut stack_info,
) -> bool {
    /* make sure it's not in the stack proper */
    if get_stack_info_noinstr(stack, current, info) {
        return false;
    }
    /* but if it is in the page below it, we hit a guard */
    get_stack_info_noinstr((stack as *mut u8).add(PAGE_SIZE) as *mut core::ffi::c_ulong, current, info)
}

#[inline]
pub unsafe fn on_stack(info: *mut stack_info, addr: *mut core::ffi::c_void, len: usize) -> bool {
    let begin = (*info).begin as *mut core::ffi::c_void;
    let end = (*info).end as *mut core::ffi::c_void;

    ((*info).type_ != stack_type::STACK_TYPE_UNKNOWN
        && (addr as usize) >= begin as usize
        && (addr as usize) < end as usize
        && (addr as usize).wrapping_add(len) > begin as usize
        && (addr as usize).wrapping_add(len) <= end as usize)
}

#[cfg(CONFIG_X86_32)]
pub const STACKSLOTS_PER_LINE: usize = 8;
#[cfg(not(CONFIG_X86_32))]
pub const STACKSLOTS_PER_LINE: usize = 4;

#[cfg(CONFIG_FRAME_POINTER)]
#[inline]
pub unsafe fn get_frame_pointer(task: *mut task_struct, regs: *mut pt_regs) -> *mut core::ffi::c_ulong {
    if !regs.is_null() {
        return (*regs).bp as *mut core::ffi::c_ulong;
    }
    if task == current {
        return __builtin_frame_address(0) as *mut core::ffi::c_ulong;
    }
    &mut (*( (*task).thread.sp as *mut inactive_task_frame)).bp
}

#[cfg(not(CONFIG_FRAME_POINTER))]
#[inline]
pub unsafe fn get_frame_pointer(_task: *mut task_struct, _regs: *mut pt_regs) -> *mut core::ffi::c_ulong {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn get_stack_pointer(task: *mut task_struct, regs: *mut pt_regs) -> *mut core::ffi::c_ulong {
    if !regs.is_null() {
        return (*regs).sp as *mut core::ffi::c_ulong;
    }
    if task == current {
        return __builtin_frame_address(0) as *mut core::ffi::c_ulong;
    }
    (*task).thread.sp as *mut core::ffi::c_ulong
}

/* The form of the top of the frame on the stack */
#[repr(C)]
pub struct stack_frame {
    pub next_frame: *mut stack_frame,
    pub return_address: core::ffi::c_ulong,
}

#[repr(C)]
pub struct stack_frame_ia32 {
    pub next_frame: u32,
    pub return_address: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
