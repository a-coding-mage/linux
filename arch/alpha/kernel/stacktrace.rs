// SPDX-License-Identifier: GPL-2.0

// Declarations supplied by the Linux kernel headers and architecture code.
use core::ffi::c_void;

pub type c_ulong = usize;
pub type stack_trace_consume_fn = unsafe extern "C" fn(*mut c_void, c_ulong) -> bool;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_info {
    pub pcb: thread_struct,
}

#[repr(C)]
pub struct thread_struct {
    pub ksp: c_ulong,
}

unsafe extern "C" {
    pub static mut current: *mut task_struct;
    fn kstack_end(addr: *const c_ulong) -> bool;
    fn __kernel_text_address(addr: c_ulong) -> bool;
    fn task_thread_info(task: *mut task_struct) -> *mut thread_info;
}

#[inline(always)]
unsafe fn alpha_get_current_ksp() -> c_ulong {
    let sp: c_ulong;
    core::arch::asm!("mov $30, {0}", out(reg) sp);
    sp
}

unsafe fn alpha_scan_kernel_stack(
    ksp: c_ulong,
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
) {
    let mut p = ksp as *mut c_ulong;

    if (ksp & (core::mem::size_of::<c_ulong>() - 1)) != 0 {
        return;
    }

    while !kstack_end(p) {
        let addr = core::ptr::read_volatile(p);
        p = p.add(1);

        if !__kernel_text_address(addr) {
            continue;
        }

        if !consume_entry(cookie, addr) {
            break;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_stack_walk(
    consume_entry: stack_trace_consume_fn,
    cookie: *mut c_void,
    mut task: *mut task_struct,
    regs: *mut pt_regs,
) {
    let ksp: c_ulong;

    if task.is_null() {
        task = current;
    }

    if !regs.is_null() && task == current {
        /*
         * pt_regs is stored on the kernel stack; regs+1 matches
         * what arch/alpha/kernel/traps.c uses as the trace start.
         */
        ksp = regs.add(1) as c_ulong;
    } else if task == current {
        ksp = alpha_get_current_ksp();
    } else {
        ksp = (*task_thread_info(task)).pcb.ksp;
    }

    alpha_scan_kernel_stack(ksp, consume_entry, cookie);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
