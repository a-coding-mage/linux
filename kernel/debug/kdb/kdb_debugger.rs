// SPDX-License-Identifier: GPL-2.0
/*
 * Created by: Jason Wessel <jason.wessel@windriver.com>
 *
 * Copyright (c) 2009 Wind River Systems, Inc.  All Rights Reserved.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2. This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Linux kernel declarations supplied by the surrounding translation unit.

pub type GetCharFunc = unsafe extern "C" fn() -> i32;

#[repr(C)]
pub struct KgdbState {
    pub cpu: i32,
    pub ex_vector: i32,
    pub linux_regs: *mut core::ffi::c_void,
    pub pass_exception: i32,
    pub err_code: i32,
    pub signo: i32,
}

#[repr(C)]
pub struct KdbBreakpoint {
    pub bp_enabled: i32,
    pub bp_addr: usize,
    pub bp_free: i32,
    pub bp_delay: i32,
    pub bp_delayed: i32,
}

extern "C" {
    pub static mut kdb_initial_cpu: i32;
    pub static mut kdb_current_task: *mut core::ffi::c_void;
    pub static mut kdb_current_regs: *mut core::ffi::c_void;
    pub static mut kgdb_active: i32;
    pub static mut kgdb_setting_breakpoint: i32;
    pub static mut kgdb_single_step: i32;
    pub static mut kgdb_info: *mut KgdbInfo;
    pub static mut kdb_breakpoints: *mut KdbBreakpoint;

    pub fn dbg_io_get_char() -> i32;
    pub fn kgdb_arch_pc(ex_vector: i32, regs: *mut core::ffi::c_void) -> usize;
    pub fn instruction_pointer(regs: *mut core::ffi::c_void) -> usize;
    pub fn kgdb_arch_set_pc(regs: *mut core::ffi::c_void, addr: usize);
    pub fn kdb_bp_remove();
    pub fn kdb_bp_install(regs: *mut core::ffi::c_void);
    pub fn kdb_main_loop(reason: i32, db_reason: i32, err_code: i32, db_result: i32,
                         regs: *mut core::ffi::c_void) -> i32;
    pub fn gdbstub_state(ks: *mut KgdbState, buf: *const i8) -> i32;
    pub fn in_nmi() -> i32;
    pub fn atomic_read(value: *mut i32) -> i32;
    pub fn for_each_online_cpu_body(cpu: *mut i32);
}

#[repr(C)]
pub struct KgdbInfo {
    pub task: *mut core::ffi::c_void,
    pub debuggerinfo: *mut core::ffi::c_void,
    pub enter_kgdb: i32,
    pub ret_state: i32,
}

pub static mut kdb_poll_funcs: [Option<GetCharFunc>; 6] = [
    Some(dbg_io_get_char), None, None, None, None, None,
];
pub static mut kdb_poll_idx: i32 = 1;
static mut kdb_ks: *mut KgdbState = core::ptr::null_mut();

pub unsafe extern "C" fn kdb_common_init_state(ks: *mut KgdbState) -> i32 {
    kdb_initial_cpu = atomic_read(&mut kgdb_active);
    kdb_current_task = (*kgdb_info.add((*ks).cpu as usize)).task;
    kdb_current_regs = (*kgdb_info.add((*ks).cpu as usize)).debuggerinfo;
    0
}

pub unsafe extern "C" fn kdb_common_deinit_state() -> i32 {
    kdb_initial_cpu = -1;
    kdb_current_task = core::ptr::null_mut();
    kdb_current_regs = core::ptr::null_mut();
    0
}

pub unsafe extern "C" fn kdb_stub(ks: *mut KgdbState) -> i32 {
    let mut error = 0;
    let mut bp: *mut KdbBreakpoint;
    let mut addr = kgdb_arch_pc((*ks).ex_vector, (*ks).linux_regs);
    let mut reason = 1; // KDB_REASON_OOPS
    let mut db_result = 0; // KDB_DB_NOBPT
    let mut i: i32;

    kdb_ks = ks;
    // KDB_STATE(REENTRY), KDB_STATE_CLEAR, KDB_STATE_SET and KDB_FLAG_* are
    // provided by the KDB state implementation.
    if kdb_state_reentry() {
        reason = 2; // KDB_REASON_SWITCH
        kdb_state_clear_reentry();
        addr = instruction_pointer((*ks).linux_regs);
    }
    (*ks).pass_exception = 0;
    if atomic_read(&mut kgdb_setting_breakpoint) != 0 { reason = 3; }
    if (*ks).err_code == 4 && (*ks).signo == 5 { reason = 4; }
    else if in_nmi() != 0 { reason = 5; }

    for i in 0..KDB_MAXBPT {
        bp = kdb_breakpoints.add(i as usize);
        if (*bp).bp_enabled != 0 && (*bp).bp_addr == addr {
            reason = 6; db_result = 1;
            if addr != instruction_pointer((*ks).linux_regs) { kgdb_arch_set_pc((*ks).linux_regs, addr); }
            break;
        }
    }
    if reason == 6 || reason == 2 {
        for i in 0..KDB_MAXBPT {
            bp = kdb_breakpoints.add(i as usize);
            if (*bp).bp_free != 0 { continue; }
            if (*bp).bp_addr == addr {
                (*bp).bp_delay = 1; (*bp).bp_delayed = 1;
                reason = 6; db_result = 1; kdb_state_set_ssbpt(); break;
            }
        }
    }
    if reason != 6 && (*ks).ex_vector == 0 && (*ks).signo == 5 { reason = 7; db_result = 1; }
    kdb_state_clear_kgdb_trans(); kdb_common_init_state(ks); kdb_bp_remove();
    kdb_state_clear_doing_ss(); kdb_state_set_pager();
    if (*ks).err_code == 8 || reason == 1 { (*ks).pass_exception = 1; kdb_flag_set_catastrophic(); }
    for_each_online_cpu_body(&mut i) { if (*kgdb_info.add(i as usize)).enter_kgdb == 0 { kdb_flag_set_catastrophic(); } }
    if kdb_state_ssbpt() && reason == 7 { kdb_state_clear_ssbpt(); kdb_state_clear_doing_ss(); }
    else { error = kdb_main_loop(9, reason, (*ks).err_code, db_result, (*ks).linux_regs); }
    kdb_common_deinit_state(); kdb_state_clear_pager();
    if error == 10 { if kdb_state_doing_kgdb() { kdb_state_clear_doing_kgdb(); } return 11; }
    kdb_bp_install((*ks).linux_regs);
    if kdb_state_doing_ss() { gdbstub_state(ks, b"s\0".as_ptr() as *const i8); } else { gdbstub_state(ks, b"c\0".as_ptr() as *const i8); }
    kdb_flag_clear_catastrophic();
    (*kgdb_info.add((*ks).cpu as usize)).ret_state = gdbstub_state(ks, b"e\0".as_ptr() as *const i8);
    if (*ks).pass_exception != 0 { (*kgdb_info.add((*ks).cpu as usize)).ret_state = 1; }
    if error == 12 { kdb_state_set_reentry(); kgdb_single_step = 0; return 13; }
    (*kgdb_info.add((*ks).cpu as usize)).ret_state
}

pub unsafe extern "C" fn kdb_gdb_state_pass(buf: *mut i8) { gdbstub_state(kdb_ks, buf); }

// File-local names representing macros and constants supplied by KDB headers.
extern "C" { fn kdb_state_reentry() -> bool; fn kdb_state_clear_reentry(); fn kdb_state_set_ssbpt(); fn kdb_state_clear_ssbpt(); fn kdb_state_ssbpt() -> bool; fn kdb_state_clear_kgdb_trans(); fn kdb_state_clear_doing_ss(); fn kdb_state_set_pager(); fn kdb_state_clear_pager(); fn kdb_state_doing_kgdb() -> bool; fn kdb_state_clear_doing_kgdb(); fn kdb_state_doing_ss() -> bool; fn kdb_state_set_reentry(); fn kdb_flag_set_catastrophic(); fn kdb_flag_clear_catastrophic(); }
const KDB_MAXBPT: i32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
