/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Created by: Jason Wessel <jason.wessel@windriver.com>
 *
 * Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
 */

/*
 * These are the private implementation headers between the kernel
 * debugger core and the debugger front end code.
 */

/* Required types are supplied by other translated dependencies. */
pub struct pt_regs;
pub struct atomic_t;
pub struct task_struct;

/* kernel debug core data structures */
#[repr(C)]
pub struct kgdb_state {
    pub ex_vector: ::core::ffi::c_int,
    pub signo: ::core::ffi::c_int,
    pub err_code: ::core::ffi::c_int,
    pub cpu: ::core::ffi::c_int,
    pub pass_exception: ::core::ffi::c_int,
    pub thr_query: ::core::ffi::c_ulong,
    pub threadid: ::core::ffi::c_ulong,
    pub kgdb_usethreadid: ::core::ffi::c_long,
    pub linux_regs: *mut pt_regs,
    pub send_ready: *mut atomic_t,
}

/* Exception state values */
pub const DCPU_WANT_MASTER: ::core::ffi::c_int = 0x1; /* Waiting to become a master kgdb cpu */
pub const DCPU_NEXT_MASTER: ::core::ffi::c_int = 0x2; /* Transition from one master cpu to another */
pub const DCPU_IS_SLAVE: ::core::ffi::c_int = 0x4; /* Slave cpu enter exception */
pub const DCPU_WANT_BT: ::core::ffi::c_int = 0x8; /* Slave cpu should backtrace then clear flag */

#[repr(C)]
pub struct debuggerinfo_struct {
    pub debuggerinfo: *mut ::core::ffi::c_void,
    pub task: *mut task_struct,
    pub exception_state: ::core::ffi::c_int,
    pub ret_state: ::core::ffi::c_int,
    pub irq_depth: ::core::ffi::c_int,
    pub enter_kgdb: ::core::ffi::c_int,
    pub rounding_up: bool,
}

extern "C" {
    pub static mut kgdb_info: [debuggerinfo_struct; 0];

    /* kernel debug core break point routines */
    pub fn dbg_remove_all_break() -> ::core::ffi::c_int;
    pub fn dbg_set_sw_break(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn dbg_remove_sw_break(addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn dbg_activate_sw_breakpoints() -> ::core::ffi::c_int;
    pub fn dbg_deactivate_sw_breakpoints() -> ::core::ffi::c_int;

    /* polled character access to i/o module */
    pub fn dbg_io_get_char() -> ::core::ffi::c_int;

    /* Switch from one cpu to another */
    pub static mut dbg_switch_cpu: ::core::ffi::c_int;

    /* gdbstub interface functions */
    pub fn gdb_serial_stub(ks: *mut kgdb_state) -> ::core::ffi::c_int;
    pub fn gdbstub_msg_write(s: *const ::core::ffi::c_char, len: ::core::ffi::c_int);

    /* gdbstub functions used for kdb <-> gdbstub transition */
    pub fn gdbstub_state(
        ks: *mut kgdb_state,
        cmd: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub static mut dbg_kdb_mode: ::core::ffi::c_int;
}

/* stub return value for switching between the gdbstub and kdb */
pub const DBG_PASS_EVENT: ::core::ffi::c_int = -12345;
/* Switch from one cpu to another */
pub const DBG_SWITCH_CPU_EVENT: ::core::ffi::c_int = -123456;

#[cfg(CONFIG_KGDB_KDB)]
extern "C" {
    pub fn kdb_stub(ks: *mut kgdb_state) -> ::core::ffi::c_int;
    pub fn kdb_parse(cmdstr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn kdb_common_init_state(ks: *mut kgdb_state) -> ::core::ffi::c_int;
    pub fn kdb_common_deinit_state() -> ::core::ffi::c_int;
    pub fn kdb_dump_stack_on_cpu(cpu: ::core::ffi::c_int);
}

#[cfg(not(CONFIG_KGDB_KDB))]
#[inline]
pub unsafe fn kdb_stub(_ks: *mut kgdb_state) -> ::core::ffi::c_int {
    DBG_PASS_EVENT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
