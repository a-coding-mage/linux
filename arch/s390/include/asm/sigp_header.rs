/* SPDX-License-Identifier: GPL-2.0 */

/* SIGP order codes */
pub const SIGP_SENSE: u32 = 1;
pub const SIGP_EXTERNAL_CALL: u32 = 2;
pub const SIGP_EMERGENCY_SIGNAL: u32 = 3;
pub const SIGP_START: u32 = 4;
pub const SIGP_STOP: u32 = 5;
pub const SIGP_RESTART: u32 = 6;
pub const SIGP_STOP_AND_STORE_STATUS: u32 = 9;
pub const SIGP_INITIAL_CPU_RESET: u32 = 11;
pub const SIGP_CPU_RESET: u32 = 12;
pub const SIGP_SET_PREFIX: u32 = 13;
pub const SIGP_STORE_STATUS_AT_ADDRESS: u32 = 14;
pub const SIGP_SET_ARCHITECTURE: u32 = 18;
pub const SIGP_COND_EMERGENCY_SIGNAL: u32 = 19;
pub const SIGP_SENSE_RUNNING: u32 = 21;
pub const SIGP_SET_MULTI_THREADING: u32 = 22;
pub const SIGP_STORE_ADDITIONAL_STATUS: u32 = 23;

/* SIGP condition codes */
pub const SIGP_CC_ORDER_CODE_ACCEPTED: i32 = 0;
pub const SIGP_CC_STATUS_STORED: i32 = 1;
pub const SIGP_CC_BUSY: i32 = 2;
pub const SIGP_CC_NOT_OPERATIONAL: i32 = 3;

/* SIGP cpu status bits */
pub const SIGP_STATUS_INVALID_ORDER: u64 = 0x00000002;
pub const SIGP_STATUS_CHECK_STOP: u64 = 0x00000010;
pub const SIGP_STATUS_STOPPED: u64 = 0x00000040;
pub const SIGP_STATUS_EXT_CALL_PENDING: u64 = 0x00000080;
pub const SIGP_STATUS_INVALID_PARAMETER: u64 = 0x00000100;
pub const SIGP_STATUS_INCORRECT_STATE: u64 = 0x00000200;
pub const SIGP_STATUS_NOT_RUNNING: u64 = 0x00000400;

/* The C implementation uses register_pair, CC_IPM, CC_OUT, CC_CLOBBER, and
 * CC_TRANSFORM from <asm/asm.h>. Those architecture-specific dependencies are
 * intentionally left for the surrounding translated s390 code to provide. */

#[inline]
pub unsafe fn ____pcpu_sigp(
    _addr: u16,
    _order: u8,
    _parm: usize,
    _status: *mut u32,
) -> i32 {
    // Corresponds to: sigp r1,addr,0(order), followed by CC_IPM and status extraction.
    // TODO: lower the s390 inline assembly once the asm.h translation is available.
    unimplemented!()
}

#[inline]
pub unsafe fn __pcpu_sigp(
    addr: u16,
    order: u8,
    parm: usize,
    status: *mut u32,
) -> i32 {
    let mut _status: u32 = 0;
    let cc = ____pcpu_sigp(addr, order, parm, &mut _status as *mut u32);
    if !status.is_null() && cc == SIGP_CC_STATUS_STORED {
        *status = _status;
    }
    cc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
