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
    pub static mut uml_exitcode: ::core::ffi::c_int;
    pub static mut kmalloc_ok: ::core::ffi::c_int;

    pub fn alloc_stack(order: ::core::ffi::c_int, atomic: ::core::ffi::c_int)
        -> ::core::ffi::c_ulong;
    pub fn free_stack(stack: ::core::ffi::c_ulong, order: ::core::ffi::c_int);

    pub fn do_signal(regs: *mut pt_regs);
    pub fn interrupt_end();
    pub fn relay_signal(
        sig: ::core::ffi::c_int,
        si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::core::ffi::c_void,
    );

    pub fn segv(
        fi: faultinfo,
        ip: ::core::ffi::c_ulong,
        is_user: ::core::ffi::c_int,
        regs: *mut uml_pt_regs,
        mc: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_ulong;
    pub fn handle_page_fault(
        address: ::core::ffi::c_ulong,
        ip: ::core::ffi::c_ulong,
        is_write: ::core::ffi::c_int,
        is_user: ::core::ffi::c_int,
        code_out: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn do_IRQ(irq: ::core::ffi::c_int, regs: *mut uml_pt_regs)
        -> ::core::ffi::c_uint;
    pub fn initial_thread_cb(proc: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>, arg: *mut ::core::ffi::c_void);

    pub fn timer_handler(
        sig: ::core::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
    );

    pub fn uml_pm_wake();
    pub fn start_uml() -> ::core::ffi::c_int;
    pub fn uml_cleanup();
    pub fn do_uml_exitcalls();

    /*
     * Are we disallowed to sleep? Used to choose between GFP_KERNEL and
     * GFP_ATOMIC.
     */
    pub fn __uml_cant_sleep() -> ::core::ffi::c_int;
    pub fn get_current_pid() -> ::core::ffi::c_int;
    pub fn copy_from_user_proc(
        to: *mut ::core::ffi::c_void,
        from: *mut ::core::ffi::c_void,
        size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn uml_strdup(string: *const ::core::ffi::c_char)
        -> *mut ::core::ffi::c_char;
    pub fn uml_need_resched() -> ::core::ffi::c_int;

    pub fn to_irq_stack(mask_out: *mut ::core::ffi::c_ulong)
        -> ::core::ffi::c_ulong;
    pub fn from_irq_stack(nested: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    pub fn singlestepping() -> ::core::ffi::c_int;

    pub fn segv_handler(
        sig: ::core::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::core::ffi::c_void,
    );
    pub fn winch(
        sig: ::core::ffi::c_int,
        unused_si: *mut siginfo,
        regs: *mut uml_pt_regs,
        mc: *mut ::core::ffi::c_void,
    );
    pub fn fatal_sigsegv() -> !;

    pub fn um_idle_sleep();
    pub fn kasan_map_memory(start: *mut ::core::ffi::c_void, len: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
