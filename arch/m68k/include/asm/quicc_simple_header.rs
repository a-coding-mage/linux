/* SPDX-License-Identifier: GPL-2.0 */
/***********************************
 * $Id: quicc_simple.h,v 1.1 2002/03/02 15:01:10 gerg Exp $
 ***********************************
 *
 ***************************************
 * Simple drivers common header
 ***************************************
 */

/* The C header guard and include directive are not executable Rust. */
/* #include "quicc.h" */

pub const GLB_SCC_0: i32 = 0;
pub const GLB_SCC_1: i32 = 1;
pub const GLB_SCC_2: i32 = 2;
pub const GLB_SCC_3: i32 = 3;

pub type int_routine = unsafe extern "C" fn(interrupt_event: u16);
pub type int_routine_ptr = *mut int_routine;
pub type alloc_routine = unsafe extern "C" fn(length: i32) -> *mut core::ffi::c_void;
pub type free_routine = unsafe extern "C" fn(
    scc_num: i32,
    channel_num: i32,
    buf: *mut core::ffi::c_void,
);
pub type store_rx_buffer_routine = unsafe extern "C" fn(
    scc_num: i32,
    channel_num: i32,
    buff: *mut core::ffi::c_void,
    length: i32,
);
pub type handle_tx_error_routine = unsafe extern "C" fn(
    scc_num: i32,
    channel_num: i32,
    tbd: *mut crate::QUICC_BD,
) -> i32;
pub type handle_rx_error_routine = unsafe extern "C" fn(
    scc_num: i32,
    channel_num: i32,
    rbd: *mut crate::QUICC_BD,
);
pub type handle_lost_error_routine = unsafe extern "C" fn(scc_num: i32, channel_num: i32);

/* user defined functions for global errors */
pub type handle_glob_overrun_routine = unsafe extern "C" fn(scc_number: i32);
pub type handle_glob_underrun_routine = unsafe extern "C" fn(scc_number: i32);
pub type glob_intr_q_overflow_routine = unsafe extern "C" fn(scc_number: i32);

/*
 * General initialization and command routines
 */
unsafe extern "C" {
    pub fn quicc_issue_cmd(cmd: u16, scc_num: i32);
    pub fn quicc_init();
    pub fn quicc_scc_init(scc_number: i32, number_of_rx_buf: i32, number_of_tx_buf: i32);
    pub fn quicc_smc_init(smc_number: i32, number_of_rx_buf: i32, number_of_tx_buf: i32);
    pub fn quicc_scc_start(scc_num: i32);
    pub fn quicc_scc_loopback(scc_num: i32);

    /* Interrupt enable/disable routines for critical pieces of code*/
    pub fn IntrDis() -> u16;
    pub fn IntrEna(old_sr: u16);

    /* For debugging */
    pub fn print_rbd(scc_num: i32);
    pub fn print_tbd(scc_num: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
