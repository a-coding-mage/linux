/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn do_page_fault(
        regs: *mut pt_regs,
        address: core::ffi::c_ulong,
        error_code: core::ffi::c_ulong,
    ) -> core::ffi::c_int;

    pub fn send_fault_sig(regs: *mut pt_regs) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
