// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    fn start_kernel();
    fn block_signals_trace();
    fn stack_protections(addr: c_ulong);
    fn set_sigstack(stack: *mut c_char, size: usize);
    fn init_new_thread_signals();
    fn start_idle_thread(stack: *mut c_void, switch_buf: *mut c_void) -> c_int;
    fn um_tlb_sync(mm: *mut mm_struct);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
}

type c_int = i32;
type c_ulong = usize;
type c_char = i8;
type c_void = core::ffi::c_void;

// Build-time kernel constants and types are supplied externally.
extern "Rust" {
    static mut cpu_irqstacks: [[c_char; THREAD_SIZE]; NR_CPUS];
    static mut init_task: task_struct;
    static mut current: *mut task_struct;
}

const NR_CPUS: usize = 0;
const THREAD_SIZE: usize = 0;

#[repr(C)]
pub struct spinlock_t {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct mm_id {
    pub stack: c_ulong,
}

#[repr(C)]
pub struct mm_context {
    pub id: mm_id,
}

#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}

#[repr(C)]
pub struct thread_request_thread {
    pub proc: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub arg: *mut c_void,
}

#[repr(C)]
pub struct thread_request {
    pub thread: thread_request_thread,
}

#[repr(C)]
pub struct thread_struct {
    pub request: thread_request,
    pub switch_buf: *mut c_void,
}

#[repr(C)]
pub struct task_struct {
    pub mm: *mut mm_struct,
    pub thread: thread_struct,
}

static mut initial_jmpbuf_spinlock: spinlock_t = spinlock_t { _opaque: [] };

unsafe extern "C" fn start_kernel_proc(_unused: *mut c_void) -> c_int {
    block_signals_trace();
    start_kernel();
    0
}

pub unsafe extern "C" fn start_uml() -> c_int {
    stack_protections((&raw mut cpu_irqstacks[0]) as *mut _ as c_ulong);
    set_sigstack((&raw mut cpu_irqstacks[0][0]) as *mut c_char, THREAD_SIZE);

    init_new_thread_signals();

    init_task.thread.request.thread.proc = Some(start_kernel_proc);
    init_task.thread.request.thread.arg = core::ptr::null_mut();
    start_idle_thread(
        task_stack_page(&mut init_task),
        init_task.thread.switch_buf,
    )
}

extern "C" {
    fn task_stack_page(task: *mut task_struct) -> *mut c_void;
}

pub unsafe extern "C" fn current_stub_stack() -> c_ulong {
    if (*current).mm.is_null() {
        return 0;
    }

    (*(*current).mm).context.id.stack
}

pub unsafe extern "C" fn current_mm_id() -> *mut mm_id {
    if (*current).mm.is_null() {
        return core::ptr::null_mut();
    }

    &mut (*(*current).mm).context.id
}

pub unsafe extern "C" fn current_mm_sync() {
    if (*current).mm.is_null() {
        return;
    }

    um_tlb_sync((*current).mm);
}

pub unsafe extern "C" fn initial_jmpbuf_lock() {
    spin_lock_irq(&raw mut initial_jmpbuf_spinlock);
}

pub unsafe extern "C" fn initial_jmpbuf_unlock() {
    spin_unlock_irq(&raw mut initial_jmpbuf_spinlock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
