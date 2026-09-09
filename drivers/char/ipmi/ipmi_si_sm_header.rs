/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * ipmi_si_sm.h
 *
 * State machine interface for low-level IPMI system management
 * interface state machines.  This code is the interface between
 * the ipmi_smi code (that handles the policy of a KCS, SMIC, or
 * BT interface) and the actual low-level state machine.
 *
 * Author: MontaVista Software, Inc.
 *         Corey Minyard <minyard@mvista.com>
 *         source@mvista.com
 *
 * Copyright 2002 MontaVista Software Inc.
 */

use core::ffi::{c_char, c_int, c_long};

/* Supplied by ipmi_si.h. */
#[repr(C)]
pub struct si_sm_io {
    _private: [u8; 0],
}

/*
 * This is defined by the state machines themselves, it is an opaque
 * data type for them to use.
 */
#[repr(C)]
pub struct si_sm_data {
    _private: [u8; 0],
}

/* Results of SMI events. */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum si_sm_result {
    SI_SM_CALL_WITHOUT_DELAY,
    SI_SM_CALL_WITH_DELAY,
    SI_SM_CALL_WITH_TICK_DELAY,
    SI_SM_TRANSACTION_COMPLETE,
    SI_SM_IDLE,
    SI_SM_HOSED,
    SI_SM_ATTN,
}

/* Handlers for the SMI state machine. */
#[repr(C)]
pub struct si_sm_handlers {
    /* Put the version number of the state machine here. */
    pub version: *mut c_char,

    /* Initialize the data and return the amount of I/O space to reserve. */
    pub init_data:
        Option<unsafe extern "C" fn(smi: *mut si_sm_data, io: *mut si_sm_io) -> u32>,

    /* Start a new transaction in the state machine. */
    pub start_transaction: Option<
        unsafe extern "C" fn(smi: *mut si_sm_data, data: *mut u8, size: u32) -> c_int,
    >,

    /* Return the results after the transaction. */
    pub get_result: Option<
        unsafe extern "C" fn(smi: *mut si_sm_data, data: *mut u8, length: u32) -> c_int,
    >,

    /* Call periodically or upon receiving an interrupt. */
    pub event: Option<unsafe extern "C" fn(smi: *mut si_sm_data, time: c_long) -> si_sm_result>,

    /* Attempt to detect an SMI. */
    pub detect: Option<unsafe extern "C" fn(smi: *mut si_sm_data) -> c_int>,

    /* The interface is shutting down, so clean it up. */
    pub cleanup: Option<unsafe extern "C" fn(smi: *mut si_sm_data)>,

    /* Return the size of the SMI structure in bytes. */
    pub size: Option<unsafe extern "C" fn() -> c_int>,
}

/* Current state machines that we can use. */
unsafe extern "C" {
    pub static kcs_smi_handlers: si_sm_handlers;
    pub static smic_smi_handlers: si_sm_handlers;
    pub static bt_smi_handlers: si_sm_handlers;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
