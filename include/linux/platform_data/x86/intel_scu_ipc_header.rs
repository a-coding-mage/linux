/* SPDX-License-Identifier: GPL-2.0 */
// Translated from intel_scu_ipc.h. Linux dependencies are supplied externally.

use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intel_scu_ipc_dev {
    _private: [u8; 0],
}

/**
 * struct intel_scu_ipc_data - Data used to configure SCU IPC
 * @mem: Base address of SCU IPC MMIO registers
 * @irq: The IRQ number used for SCU (optional)
 */
#[repr(C)]
pub struct intel_scu_ipc_data {
    pub mem: resource,
    pub irq: core::ffi::c_int,
}

extern "C" {
    pub static mut THIS_MODULE: *mut module;

    pub fn __intel_scu_ipc_register(
        parent: *mut device,
        scu_data: *const intel_scu_ipc_data,
        owner: *mut module,
    ) -> *mut intel_scu_ipc_dev;

    pub fn intel_scu_ipc_unregister(scu: *mut intel_scu_ipc_dev);

    pub fn __devm_intel_scu_ipc_register(
        parent: *mut device,
        scu_data: *const intel_scu_ipc_data,
        owner: *mut module,
    ) -> *mut intel_scu_ipc_dev;

    pub fn intel_scu_ipc_dev_get() -> *mut intel_scu_ipc_dev;
    pub fn intel_scu_ipc_dev_put(scu: *mut intel_scu_ipc_dev);
    pub fn devm_intel_scu_ipc_dev_get(dev: *mut device) -> *mut intel_scu_ipc_dev;

    pub fn intel_scu_ipc_dev_ioread8(
        scu: *mut intel_scu_ipc_dev,
        addr: u16,
        data: *mut u8,
    ) -> core::ffi::c_int;
    pub fn intel_scu_ipc_dev_iowrite8(
        scu: *mut intel_scu_ipc_dev,
        addr: u16,
        data: u8,
    ) -> core::ffi::c_int;
    pub fn intel_scu_ipc_dev_readv(
        scu: *mut intel_scu_ipc_dev,
        addr: *mut u16,
        data: *mut u8,
        len: usize,
    ) -> core::ffi::c_int;
    pub fn intel_scu_ipc_dev_writev(
        scu: *mut intel_scu_ipc_dev,
        addr: *mut u16,
        data: *mut u8,
        len: usize,
    ) -> core::ffi::c_int;

    pub fn intel_scu_ipc_dev_update(
        scu: *mut intel_scu_ipc_dev,
        addr: u16,
        data: u8,
        mask: u8,
    ) -> core::ffi::c_int;

    pub fn intel_scu_ipc_dev_simple_command(
        scu: *mut intel_scu_ipc_dev,
        cmd: core::ffi::c_int,
        sub: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn intel_scu_ipc_dev_command_with_size(
        scu: *mut intel_scu_ipc_dev,
        cmd: core::ffi::c_int,
        sub: core::ffi::c_int,
        input: *const c_void,
        inlen: usize,
        size: usize,
        out: *mut c_void,
        outlen: usize,
    ) -> core::ffi::c_int;
}

// #define intel_scu_ipc_register(parent, scu_data) \
//     __intel_scu_ipc_register(parent, scu_data, THIS_MODULE)
#[inline]
pub unsafe fn intel_scu_ipc_register(
    parent: *mut device,
    scu_data: *const intel_scu_ipc_data,
) -> *mut intel_scu_ipc_dev {
    __intel_scu_ipc_register(parent, scu_data, THIS_MODULE)
}

// #define devm_intel_scu_ipc_register(parent, scu_data) \
//     __devm_intel_scu_ipc_register(parent, scu_data, THIS_MODULE)
#[inline]
pub unsafe fn devm_intel_scu_ipc_register(
    parent: *mut device,
    scu_data: *const intel_scu_ipc_data,
) -> *mut intel_scu_ipc_dev {
    __devm_intel_scu_ipc_register(parent, scu_data, THIS_MODULE)
}

#[inline]
pub unsafe fn intel_scu_ipc_dev_command(
    scu: *mut intel_scu_ipc_dev,
    cmd: core::ffi::c_int,
    sub: core::ffi::c_int,
    input: *const c_void,
    inlen: usize,
    out: *mut c_void,
    outlen: usize,
) -> core::ffi::c_int {
    intel_scu_ipc_dev_command_with_size(scu, cmd, sub, input, inlen, inlen, out, outlen)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
