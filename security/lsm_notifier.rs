// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LSM notifier functions
 *
 */

// C dependencies:
// #include <linux/notifier.h>
// #include <linux/security.h>

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

pub type lsm_event = u32;

unsafe extern "C" {
    fn blocking_notifier_call_chain(
        nh: *mut blocking_notifier_head,
        val: lsm_event,
        v: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn blocking_notifier_chain_register(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
    fn blocking_notifier_chain_unregister(
        nh: *mut blocking_notifier_head,
        nb: *mut notifier_block,
    ) -> core::ffi::c_int;
}

// static BLOCKING_NOTIFIER_HEAD(blocking_lsm_notifier_chain);
static mut blocking_lsm_notifier_chain: blocking_notifier_head = blocking_notifier_head { _private: [] };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn call_blocking_lsm_notifier(
    event: lsm_event,
    data: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    unsafe {
        blocking_notifier_call_chain(
            &raw mut blocking_lsm_notifier_chain,
            event,
            data,
        )
    }
}
// EXPORT_SYMBOL(call_blocking_lsm_notifier);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn register_blocking_lsm_notifier(
    nb: *mut notifier_block,
) -> core::ffi::c_int {
    unsafe {
        blocking_notifier_chain_register(
            &raw mut blocking_lsm_notifier_chain,
            nb,
        )
    }
}
// EXPORT_SYMBOL(register_blocking_lsm_notifier);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn unregister_blocking_lsm_notifier(
    nb: *mut notifier_block,
) -> core::ffi::c_int {
    unsafe {
        blocking_notifier_chain_unregister(
            &raw mut blocking_lsm_notifier_chain,
            nb,
        )
    }
}
// EXPORT_SYMBOL(unregister_blocking_lsm_notifier);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
