/* SPDX-License-Identifier: GPL-2.0-only */

// C dependency: <linux/linkage.h> (asmlinkage).

#[repr(C)]
pub struct frame {
    _private: [u8; 0],
}

extern "C" {
    pub fn buserr_c(fp: *mut frame);
    pub fn fpemu_signal(signal: core::ffi::c_int, code: core::ffi::c_int, addr: *mut core::ffi::c_void);
    pub fn fpsp040_die();
    pub fn set_esp0(ssp: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
