/* SPDX-License-Identifier: GPL-2.0-only */

// Declarations corresponding to <linux/kvm_types.h> and <linux/errno.h> are
// supplied by the surrounding translation unit.

use core::ffi::c_void;

pub type gpa_t = u64;

pub struct kvm_io_device;
pub struct kvm_vcpu;

/**
 * kvm_io_device_ops are called under kvm slots_lock.
 * read and write handlers return 0 if the transaction has been handled,
 * or non-zero to have it passed to the next device.
 **/
#[repr(C)]
pub struct kvm_io_device_ops {
    pub read: Option<unsafe extern "C" fn(
        vcpu: *mut kvm_vcpu,
        this: *mut kvm_io_device,
        addr: gpa_t,
        len: i32,
        val: *mut c_void,
    ) -> i32>,
    pub write: Option<unsafe extern "C" fn(
        vcpu: *mut kvm_vcpu,
        this: *mut kvm_io_device,
        addr: gpa_t,
        len: i32,
        val: *const c_void,
    ) -> i32>,
    pub destructor: Option<unsafe extern "C" fn(this: *mut kvm_io_device)>,
}

#[repr(C)]
pub struct kvm_io_device {
    pub ops: *const kvm_io_device_ops,
}

#[inline]
pub unsafe fn kvm_iodevice_init(
    dev: *mut kvm_io_device,
    ops: *const kvm_io_device_ops,
) {
    (*dev).ops = ops;
}

#[inline]
pub unsafe fn kvm_iodevice_read(
    vcpu: *mut kvm_vcpu,
    dev: *mut kvm_io_device,
    addr: gpa_t,
    l: i32,
    v: *mut c_void,
) -> i32 {
    match (*(*dev).ops).read {
        Some(read) => read(vcpu, dev, addr, l, v),
        None => -EOPNOTSUPP,
    }
}

#[inline]
pub unsafe fn kvm_iodevice_write(
    vcpu: *mut kvm_vcpu,
    dev: *mut kvm_io_device,
    addr: gpa_t,
    l: i32,
    v: *const c_void,
) -> i32 {
    match (*(*dev).ops).write {
        Some(write) => write(vcpu, dev, addr, l, v),
        None => -EOPNOTSUPP,
    }
}

// Supplied by <linux/errno.h> in the surrounding translation unit.
unsafe extern "C" {
    static EOPNOTSUPP: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
