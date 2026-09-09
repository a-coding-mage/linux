/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declaration corresponding to `struct comedi_subdevice`.
#[repr(C)]
pub struct comedi_subdevice {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn addi_watchdog_reset(iobase: core::ffi::c_ulong);
    pub fn addi_watchdog_init(
        s: *mut comedi_subdevice,
        iobase: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
