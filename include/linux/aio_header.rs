/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency provided by linux/aio_abi.h. */
use core::ffi::c_int;

#[repr(C)]
pub struct kioctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

pub type kiocb_cancel_fn = unsafe extern "C" fn(req: *mut kiocb) -> c_int;

/* prototypes */
#[cfg(CONFIG_AIO)]
extern "C" {
    pub fn exit_aio(mm: *mut mm_struct);
    pub fn kiocb_set_cancel_fn(
        req: *mut kiocb,
        cancel: Option<kiocb_cancel_fn>,
    );
}

#[cfg(not(CONFIG_AIO))]
#[inline]
pub unsafe fn exit_aio(_mm: *mut mm_struct) {}

#[cfg(not(CONFIG_AIO))]
#[inline]
pub unsafe fn kiocb_set_cancel_fn(
    _req: *mut kiocb,
    _cancel: Option<kiocb_cancel_fn>,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
