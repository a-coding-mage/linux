// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 2000, 2001, 2002 Andi Kleen, SuSE Labs
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn this_cpu_read_hardirq_stack_ptr() -> usize;
    fn this_cpu_read_softirq_stack_ptr() -> usize;
    fn raw_smp_processor_id() -> i32;
    fn get_cpu_entry_area(cpu: i32) -> *mut cpu_entry_area;
    static mut current: *mut task_struct;
    fn printk_deferred_once(format: *const core::ffi::c_char, ...);
    fn this_cpu_read_cpu_tss_rw_sp() -> *mut usize;
}

const EINVAL: i32 = 22;

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct cpu_entry_area {
    pub doublefault_stack: doublefault_stack,
}

#[repr(C)]
pub struct doublefault_stack {
    pub stack: [u8; THREAD_SIZE],
}

#[repr(C)]
pub struct stack_info {
    pub type_: stack_type,
    pub begin: *mut usize,
    pub end: *mut usize,
    pub next_sp: *mut usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN = 0,
    STACK_TYPE_IRQ,
    STACK_TYPE_SOFTIRQ,
    STACK_TYPE_ENTRY,
    STACK_TYPE_EXCEPTION,
}

extern "C" {
    fn in_task_stack(stack: *mut usize, task: *mut task_struct, info: *mut stack_info) -> bool;
    fn in_entry_stack(stack: *mut usize, info: *mut stack_info) -> bool;
}

const THREAD_SIZE: usize = 0; // Supplied by the kernel build configuration.

pub unsafe extern "C" fn stack_type_name(type_: stack_type) -> *const core::ffi::c_char {
    if matches!(type_, stack_type::STACK_TYPE_IRQ) {
        return b"IRQ\0".as_ptr() as *const core::ffi::c_char;
    }

    if matches!(type_, stack_type::STACK_TYPE_SOFTIRQ) {
        return b"SOFTIRQ\0".as_ptr() as *const core::ffi::c_char;
    }

    if matches!(type_, stack_type::STACK_TYPE_ENTRY) {
        return b"ENTRY_TRAMPOLINE\0".as_ptr() as *const core::ffi::c_char;
    }

    if matches!(type_, stack_type::STACK_TYPE_EXCEPTION) {
        return b"#DF\0".as_ptr() as *const core::ffi::c_char;
    }

    core::ptr::null()
}

unsafe fn in_hardirq_stack(stack: *mut usize, info: *mut stack_info) -> bool {
    let begin = this_cpu_read_hardirq_stack_ptr() as *mut usize;
    let end = begin.add(THREAD_SIZE / core::mem::size_of::<usize>());

    /*
     * This is a software stack, so 'end' can be a valid stack pointer.
     * It just means the stack is empty.
     */
    if stack < begin || stack > end {
        return false;
    }

    (*info).type_ = stack_type::STACK_TYPE_IRQ;
    (*info).begin = begin;
    (*info).end = end;

    /*
     * See irq_32.c -- the next stack pointer is stored at the beginning of
     * the stack.
     */
    (*info).next_sp = *begin as *mut usize;

    true
}

unsafe fn in_softirq_stack(stack: *mut usize, info: *mut stack_info) -> bool {
    let begin = this_cpu_read_softirq_stack_ptr() as *mut usize;
    let end = begin.add(THREAD_SIZE / core::mem::size_of::<usize>());

    /*
     * This is a software stack, so 'end' can be a valid stack pointer.
     * It just means the stack is empty.
     */
    if stack < begin || stack > end {
        return false;
    }

    (*info).type_ = stack_type::STACK_TYPE_SOFTIRQ;
    (*info).begin = begin;
    (*info).end = end;

    /*
     * The next stack pointer is stored at the beginning of the stack.
     * See irq_32.c.
     */
    (*info).next_sp = *begin as *mut usize;

    true
}

unsafe fn in_doublefault_stack(stack: *mut usize, info: *mut stack_info) -> bool {
    let cea = get_cpu_entry_area(raw_smp_processor_id());
    let ss = &mut (*cea).doublefault_stack;

    let begin = ss.stack.as_mut_ptr();
    let end = begin.add(core::mem::size_of_val(&ss.stack));

    if (stack as *mut u8) < begin || (stack as *mut u8) >= end {
        return false;
    }

    (*info).type_ = stack_type::STACK_TYPE_EXCEPTION;
    (*info).begin = begin as *mut usize;
    (*info).end = end as *mut usize;
    (*info).next_sp = this_cpu_read_cpu_tss_rw_sp();

    true
}

pub unsafe extern "C" fn get_stack_info(
    stack: *mut usize,
    mut task: *mut task_struct,
    info: *mut stack_info,
    visit_mask: *mut usize,
) -> i32 {
    if stack.is_null() {
        (*info).type_ = stack_type::STACK_TYPE_UNKNOWN;
        return -EINVAL;
    }

    if task.is_null() {
        task = current;
    }

    if in_task_stack(stack, task, info) {
        return recursion_check(info, visit_mask);
    }

    if task != current {
        (*info).type_ = stack_type::STACK_TYPE_UNKNOWN;
        return -EINVAL;
    }

    if in_entry_stack(stack, info) {
        return recursion_check(info, visit_mask);
    }

    if in_hardirq_stack(stack, info) {
        return recursion_check(info, visit_mask);
    }

    if in_softirq_stack(stack, info) {
        return recursion_check(info, visit_mask);
    }

    if in_doublefault_stack(stack, info) {
        return recursion_check(info, visit_mask);
    }

    (*info).type_ = stack_type::STACK_TYPE_UNKNOWN;
    -EINVAL
}

unsafe fn recursion_check(info: *mut stack_info, visit_mask: *mut usize) -> i32 {
    /*
     * Make sure we don't iterate through any given stack more than once.
     * If it comes up a second time then there's something wrong going on:
     * just break out and report an unknown stack type.
     */
    if !visit_mask.is_null() {
        let bit = 1usize.wrapping_shl((*info).type_ as u32);
        if (*visit_mask & bit) != 0 {
            static WARNING: &[u8] = b"WARNING: stack recursion on stack type %d\n\0";
            printk_deferred_once(WARNING.as_ptr() as *const core::ffi::c_char, (*info).type_ as i32);
            (*info).type_ = stack_type::STACK_TYPE_UNKNOWN;
            return -EINVAL;
        }
        *visit_mask |= bit;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
