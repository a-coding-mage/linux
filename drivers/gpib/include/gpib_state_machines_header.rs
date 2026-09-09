/* SPDX-License-Identifier: GPL-2.0 */

/***************************************************************************
 *    copyright            : (C) 2006 by Frank Mori Hess
 **************************************************************************/

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum talker_function_state {
    talker_idle,
    talker_addressed,
    talker_active,
    serial_poll_active,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum listener_function_state {
    listener_idle,
    listener_addressed,
    listener_active,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
