/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the original sysdep headers remain external.

#[repr(C)]
pub struct siginfo {
    _private: [u8; 0],
}

// `pt_regs`, `uml_pt_regs`, and `faultinfo` are supplied by the corresponding
// external dependencies.

unsafe extern "C" {
    pub static mut uml_exitcode: ::kernel::ffi::c_int;
    pub static mut kmalloc_ok: ::kernel::ffi::c_int;

    pub fn alloc_stack(order: ::kernel::ffi::c_int, atomic: ::kernel::ffi::c_int)
        -> ::kernel::ffi::c_ulong;
    pub fn free_stack(stack: ::kernel::ffi::c_ulong, order: ::kernel::ffi::c_int);

    pub fn do_signal(regs: *mut pt_regs);
    pub fn interrupt_end();
    pub fn relay_signal(
        sig: ::kernel::ffi::c_int,
        si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::kernel::ffi::c_void,
    );

    pub fn segv(
        fi: faultinfo,
        ip: ::kernel::ffi::c_ulong,
        is_user: ::kernel::ffi::c_int,
        regs: *mut uml_pt_regs,
        mc: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_ulong;
    pub fn handle_page_fault(
        address: ::kernel::ffi::c_ulong,
        ip: ::kernel::ffi::c_ulong,
        is_write: ::kernel::ffi::c_int,
        is_user: ::kernel::ffi::c_int,
        code_out: *mut ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;

    pub fn do_IRQ(irq: ::kernel::ffi::c_int, regs: *mut uml_pt_regs)
        -> ::kernel::ffi::c_uint;
    pub fn initial_thread_cb(proc: Option<unsafe extern "C" fn(*mut ::kernel::ffi::c_void)>, arg: *mut ::kernel::ffi::c_void);

    pub fn timer_handler(
        sig: ::kernel::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
    );

    pub fn uml_pm_wake();
    pub fn start_uml() -> ::kernel::ffi::c_int;
    pub fn uml_cleanup();
    pub fn do_uml_exitcalls();

    /*
     * Are we disallowed to sleep? Used to choose between GFP_KERNEL and
     * GFP_ATOMIC.
     */
    pub fn __uml_cant_sleep() -> ::kernel::ffi::c_int;
    pub fn get_current_pid() -> ::kernel::ffi::c_int;
    pub fn copy_from_user_proc(
        to: *mut ::kernel::ffi::c_void,
        from: *mut ::kernel::ffi::c_void,
        size: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
    pub fn uml_strdup(string: *const ::kernel::ffi::c_char)
        -> *mut ::kernel::ffi::c_char;
    pub fn uml_need_resched() -> ::kernel::ffi::c_int;

    pub fn to_irq_stack(mask_out: *mut ::kernel::ffi::c_ulong)
        -> ::kernel::ffi::c_ulong;
    pub fn from_irq_stack(nested: ::kernel::ffi::c_int) -> ::kernel::ffi::c_ulong;
    pub fn singlestepping() -> ::kernel::ffi::c_int;

    pub fn segv_handler(
        sig: ::kernel::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::kernel::ffi::c_void,
    );
    pub fn winch(
        sig: ::kernel::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::kernel::ffi::c_void,
    );
    pub fn fatal_sigsegv() -> !;

    pub fn um_idle_sleep();
    pub fn kasan_map_memory(start: *mut ::kernel::ffi::c_void, len: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
