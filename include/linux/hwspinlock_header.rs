/* SPDX-License-Identifier: GPL-2.0 */
/* Hardware spinlock public header; translated from linux/hwspinlock.h. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Includes: linux/err.h and linux/sched.h are external dependencies. */

/* hwspinlock mode argument */
pub const HWLOCK_IRQSTATE: c_int = 0x01; /* Disable interrupts, save state */
pub const HWLOCK_IRQ: c_int = 0x02; /* Disable interrupts, don't save state */
pub const HWLOCK_RAW: c_int = 0x03;
pub const HWLOCK_IN_ATOMIC: c_int = 0x04; /* Called while in atomic context */

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct hwspinlock { _private: [u8; 0] }
#[repr(C)]
pub struct hwspinlock_device { _private: [u8; 0] }
#[repr(C)]
pub struct hwspinlock_ops { _private: [u8; 0] }

/* ERR_PTR(-ENODEV) is supplied by linux/err.h. */
extern "C" {
    pub fn ERR_PTR(error: isize) -> *mut c_void;
}

#[cfg(feature = "CONFIG_HWSPINLOCK")]
extern "C" {
    pub fn hwspin_lock_register(bank: *mut hwspinlock_device, dev: *mut device,
        ops: *const hwspinlock_ops, base_id: c_int, num_locks: c_int) -> c_int;
    pub fn hwspin_lock_unregister(bank: *mut hwspinlock_device) -> c_int;
    pub fn hwspin_lock_request_specific(id: c_uint) -> *mut hwspinlock;
    pub fn hwspin_lock_free(hwlock: *mut hwspinlock) -> c_int;
    pub fn of_hwspin_lock_get_id(np: *mut device_node, index: c_int) -> c_int;
    pub fn __hwspin_lock_timeout(hwlock: *mut hwspinlock, to: c_uint, mode: c_int,
        flags: *mut c_ulong) -> c_int;
    pub fn __hwspin_trylock(hwlock: *mut hwspinlock, mode: c_int,
        flags: *mut c_ulong) -> c_int;
    pub fn __hwspin_unlock(hwlock: *mut hwspinlock, mode: c_int, flags: *mut c_ulong);
    pub fn of_hwspin_lock_get_id_byname(np: *mut device_node, name: *const c_char) -> c_int;
    pub fn hwspin_lock_bust(hwlock: *mut hwspinlock, id: c_uint) -> c_int;
    pub fn devm_hwspin_lock_free(dev: *mut device, hwlock: *mut hwspinlock) -> c_int;
    pub fn devm_hwspin_lock_request_specific(dev: *mut device, id: c_uint) -> *mut hwspinlock;
    pub fn devm_hwspin_lock_unregister(dev: *mut device, bank: *mut hwspinlock_device) -> c_int;
    pub fn devm_hwspin_lock_register(dev: *mut device, bank: *mut hwspinlock_device,
        ops: *const hwspinlock_ops, base_id: c_int, num_locks: c_int) -> c_int;
}

#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn hwspin_lock_request_specific(_id: c_uint) -> *mut hwspinlock {
    ERR_PTR(-19) as *mut hwspinlock /* -ENODEV */
}
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn hwspin_lock_free(_hwlock: *mut hwspinlock) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn __hwspin_lock_timeout(_hwlock: *mut hwspinlock, _to: c_uint, _mode: c_int,
    _flags: *mut c_ulong) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn __hwspin_trylock(_hwlock: *mut hwspinlock, _mode: c_int,
    _flags: *mut c_ulong) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn __hwspin_unlock(_hwlock: *mut hwspinlock, _mode: c_int, _flags: *mut c_ulong) {}
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn hwspin_lock_bust(_hwlock: *mut hwspinlock, _id: c_uint) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn of_hwspin_lock_get_id(_np: *mut device_node, _index: c_int) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn of_hwspin_lock_get_id_byname(_np: *mut device_node, _name: *const c_char) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn devm_hwspin_lock_free(_dev: *mut device, _hwlock: *mut hwspinlock) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_HWSPINLOCK"))]
pub unsafe fn devm_hwspin_lock_request_specific(_dev: *mut device, _id: c_uint) -> *mut hwspinlock {
    ERR_PTR(-19) as *mut hwspinlock /* -ENODEV */
}

pub unsafe fn hwspin_trylock_irqsave(hwlock: *mut hwspinlock, flags: *mut c_ulong) -> c_int {
    __hwspin_trylock(hwlock, HWLOCK_IRQSTATE, flags)
}
pub unsafe fn hwspin_trylock_irq(hwlock: *mut hwspinlock) -> c_int {
    __hwspin_trylock(hwlock, HWLOCK_IRQ, core::ptr::null_mut())
}
pub unsafe fn hwspin_trylock_raw(hwlock: *mut hwspinlock) -> c_int {
    __hwspin_trylock(hwlock, HWLOCK_RAW, core::ptr::null_mut())
}
pub unsafe fn hwspin_trylock_in_atomic(hwlock: *mut hwspinlock) -> c_int {
    __hwspin_trylock(hwlock, HWLOCK_IN_ATOMIC, core::ptr::null_mut())
}
pub unsafe fn hwspin_trylock(hwlock: *mut hwspinlock) -> c_int {
    __hwspin_trylock(hwlock, 0, core::ptr::null_mut())
}

pub unsafe fn hwspin_lock_timeout_irqsave(hwlock: *mut hwspinlock, to: c_uint, flags: *mut c_ulong) -> c_int {
    __hwspin_lock_timeout(hwlock, to, HWLOCK_IRQSTATE, flags)
}
pub unsafe fn hwspin_lock_timeout_irq(hwlock: *mut hwspinlock, to: c_uint) -> c_int {
    __hwspin_lock_timeout(hwlock, to, HWLOCK_IRQ, core::ptr::null_mut())
}
pub unsafe fn hwspin_lock_timeout_raw(hwlock: *mut hwspinlock, to: c_uint) -> c_int {
    __hwspin_lock_timeout(hwlock, to, HWLOCK_RAW, core::ptr::null_mut())
}
pub unsafe fn hwspin_lock_timeout_in_atomic(hwlock: *mut hwspinlock, to: c_uint) -> c_int {
    __hwspin_lock_timeout(hwlock, to, HWLOCK_IN_ATOMIC, core::ptr::null_mut())
}
pub unsafe fn hwspin_lock_timeout(hwlock: *mut hwspinlock, to: c_uint) -> c_int {
    __hwspin_lock_timeout(hwlock, to, 0, core::ptr::null_mut())
}

pub unsafe fn hwspin_unlock_irqrestore(hwlock: *mut hwspinlock, flags: *mut c_ulong) {
    __hwspin_unlock(hwlock, HWLOCK_IRQSTATE, flags)
}
pub unsafe fn hwspin_unlock_irq(hwlock: *mut hwspinlock) {
    __hwspin_unlock(hwlock, HWLOCK_IRQ, core::ptr::null_mut())
}
pub unsafe fn hwspin_unlock_raw(hwlock: *mut hwspinlock) {
    __hwspin_unlock(hwlock, HWLOCK_RAW, core::ptr::null_mut())
}
pub unsafe fn hwspin_unlock_in_atomic(hwlock: *mut hwspinlock) {
    __hwspin_unlock(hwlock, HWLOCK_IN_ATOMIC, core::ptr::null_mut())
}
pub unsafe fn hwspin_unlock(hwlock: *mut hwspinlock) {
    __hwspin_unlock(hwlock, 0, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
