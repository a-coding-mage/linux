/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/firewire-constants.h>, <linux/types.h>,
// <linux/sched.h>, <sound/rawmidi.h>.

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

pub const FW_GENERATION_MASK: c_uint = 0x00ff;
pub const FW_FIXED_GENERATION: c_uint = 0x0100;
pub const FW_QUIET: c_uint = 0x0200;

unsafe extern "C" {
    pub fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: u64,
        buffer: *mut c_void,
        length: usize,
        flags: c_uint,
    ) -> c_int;
}

/* returns true if retrying the transaction would not make sense */
#[inline]
pub unsafe fn rcode_is_permanent_error(rcode: c_int) -> bool {
    rcode == RCODE_TYPE_ERROR || rcode == RCODE_ADDRESS_ERROR
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
