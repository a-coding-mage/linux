// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *  Copyright (C) 2000, 2001, 2002 Andi Kleen, SuSE Labs
 */

// Linux and architecture-specific dependencies are supplied by the surrounding kernel translation.

static EXCEPTION_STACK_NAMES: [&[u8]; 6] = [b"#DF\0", b"NMI\0", b"#DB\0", b"#MC\0", b"#VC\0", b"#VC2\0"];

pub unsafe extern "C" fn stack_type_name(type_: i32) -> *const u8 {
    // BUILD_BUG_ON(N_EXCEPTION_STACKS != 6);
    if type_ == STACK_TYPE_TASK { return b"TASK\0".as_ptr(); }
    if type_ == STACK_TYPE_IRQ { return b"IRQ\0".as_ptr(); }
    if type_ == STACK_TYPE_SOFTIRQ { return b"SOFTIRQ\0".as_ptr(); }
    if type_ == STACK_TYPE_ENTRY {
        /*
         * On 64-bit, we have a generic entry stack that we
         * use for all the kernel entry points, including
         * SYSENTER.
         */
        return b"ENTRY_TRAMPOLINE\0".as_ptr();
    }
    if type_ >= STACK_TYPE_EXCEPTION && type_ <= STACK_TYPE_EXCEPTION_LAST {
        return EXCEPTION_STACK_NAMES[(type_ - STACK_TYPE_EXCEPTION) as usize].as_ptr();
    }
    core::ptr::null()
}

#[repr(C)]
pub struct estack_pages {
    pub offs: u32,
    pub size: u16,
    pub type_: u16,
}

// EPAGERANGE(st) expands to page-range designated initializers in C.
// The architecture supplies the corresponding CEA_ESTACK_* values and page layout.
static ESTACK_PAGES: [estack_pages; CEA_ESTACK_PAGES as usize] = [
    estack_pages { offs: 0, size: 0, type_: 0 };
    CEA_ESTACK_PAGES as usize
];

unsafe extern "C" {
    static mut cea_exception_stacks: usize;
    static mut hardirq_stack_ptr: *mut u64;
}

#[inline(always)]
unsafe fn in_exception_stack(stack: *mut usize, info: *mut stack_info) -> bool {
    let mut begin: usize;
    let mut end: usize;
    let stk = stack as usize;
    // BUILD_BUG_ON(N_EXCEPTION_STACKS != 6);
    begin = core::ptr::read_volatile(&cea_exception_stacks);
    if begin == 0 { return false; }
    end = begin + core::mem::size_of::<cea_exception_stacks>();
    if stk < begin || stk >= end { return false; }
    let k = (stk - begin) >> PAGE_SHIFT;
    let ep = &ESTACK_PAGES[k];
    if ep.size == 0 { return false; }
    begin += ep.offs as usize;
    end = begin + ep.size as usize;
    let regs = (end as *mut pt_regs).offset(-1);
    (*info).type_ = ep.type_ as i32;
    (*info).begin = begin as *mut usize;
    (*info).end = end as *mut usize;
    (*info).next_sp = (*regs).sp as *mut usize;
    true
}

#[inline(always)]
unsafe fn in_irq_stack(stack: *mut usize, info: *mut stack_info) -> bool {
    let mut end = core::ptr::read_volatile(&hardirq_stack_ptr).add(1);
    let begin = end.sub(IRQ_STACK_SIZE / core::mem::size_of::<usize>());
    if stack < begin || stack >= end { return false; }
    (*info).type_ = STACK_TYPE_IRQ;
    (*info).begin = begin;
    (*info).end = end;
    (*info).next_sp = *(end.sub(1) as *mut *mut usize);
    true
}

pub unsafe extern "C" fn get_stack_info_noinstr(
    stack: *mut usize, task: *mut task_struct, info: *mut stack_info,
) -> bool {
    if in_task_stack(stack, task, info) { return true; }
    if task != current { return false; }
    if in_exception_stack(stack, info) { return true; }
    if in_irq_stack(stack, info) { return true; }
    if in_entry_stack(stack, info) { return true; }
    false
}

pub unsafe extern "C" fn get_stack_info(
    stack: *mut usize, mut task: *mut task_struct, info: *mut stack_info,
    visit_mask: *mut usize,
) -> i32 {
    if task.is_null() { task = current; }
    if stack.is_null() { return unknown_stack(info); }
    if !get_stack_info_noinstr(stack, task, info) { return unknown_stack(info); }
    if !visit_mask.is_null() {
        if *visit_mask & (1usize << (*info).type_) != 0 {
            if task == current {
                printk_deferred_once(KERN_WARNING, b"WARNING: stack recursion on stack type %d\n\0".as_ptr(), (*info).type_);
            }
            return unknown_stack(info);
        }
        *visit_mask |= 1usize << (*info).type_;
    }
    0
}

unsafe fn unknown_stack(info: *mut stack_info) -> i32 {
    (*info).type_ = STACK_TYPE_UNKNOWN;
    -EINVAL
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
