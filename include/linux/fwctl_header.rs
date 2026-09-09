/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
 */

// Dependencies supplied by the Linux device, cdev, cleanup, and fwctl UAPI
// headers are intentionally referenced here but not redefined.

pub struct fwctl_device;
pub struct fwctl_uctx;

/**
 * struct fwctl_ops - Driver provided operations
 *
 * fwctl_unregister() will wait until all excuting ops are completed before it
 * returns. Drivers should be mindful to not let their ops run for too long as
 * it will block device hot unplug and module unloading.
 */
#[repr(C)]
pub struct fwctl_ops {
    /** The drivers assigned device_type number. This is uABI. */
    pub device_type: fwctl_device_type,
    /** The size of the fwctl_uctx struct to allocate. */
    pub uctx_size: usize,
    /** Called when a file descriptor is opened before the uctx is ever used. */
    pub open_uctx: Option<unsafe extern "C" fn(uctx: *mut fwctl_uctx) -> i32>,
    /** Called when the uctx is destroyed, usually when the FD is closed. */
    pub close_uctx: Option<unsafe extern "C" fn(uctx: *mut fwctl_uctx)>,
    /** Implement FWCTL_INFO. */
    pub info: Option<unsafe extern "C" fn(uctx: *mut fwctl_uctx, length: *mut usize) -> *mut core::ffi::c_void>,
    /** Implement FWCTL_RPC. */
    pub fw_rpc: Option<unsafe extern "C" fn(
        uctx: *mut fwctl_uctx,
        scope: fwctl_rpc_scope,
        rpc_in: *mut core::ffi::c_void,
        in_len: usize,
        out_len: *mut usize,
    ) -> *mut core::ffi::c_void>,
}

/**
 * struct fwctl_device - Per-driver registration struct
 * @dev: The sysfs (class/fwctl/fwctlXX) device
 *
 * Each driver instance will have one of these structs with the driver private
 * data following immediately after. This struct is refcounted, it is freed by
 * calling fwctl_put().
 */
#[repr(C)]
pub struct fwctl_device {
    pub dev: device,
    /* private: */
    pub cdev: cdev,

    /* Protect uctx_list */
    pub uctx_list_lock: mutex,
    pub uctx_list: list_head,
    /*
     * Protect ops, held for write when ops becomes NULL during unregister,
     * held for read whenever ops is loaded or an ops function is running.
     */
    pub registration_lock: rw_semaphore,
    pub ops: *const fwctl_ops,
}

unsafe extern "C" {
    pub fn _fwctl_alloc_device(
        parent: *mut device,
        ops: *const fwctl_ops,
        size: usize,
    ) -> *mut fwctl_device;

    pub fn get_device(dev: *mut device) -> *mut device;
    pub fn put_device(dev: *mut device);

    pub fn fwctl_register(fwctl: *mut fwctl_device) -> i32;
    pub fn fwctl_unregister(fwctl: *mut fwctl_device);
}

/**
 * fwctl_alloc_device - Allocate a fwctl
 * @parent: Physical device that provides the FW interface
 * @ops: Driver ops to register
 * @drv_struct: 'struct driver_fwctl' that holds the struct fwctl_device
 * @member: Name of the struct fwctl_device in @drv_struct
 *
 * This allocates and initializes the fwctl_device embedded in the drv_struct.
 * Upon success the pointer must be freed via fwctl_put(). Returns a 'drv_struct
 * *' on success, NULL on error.
 */
#[macro_export]
macro_rules! fwctl_alloc_device {
    ($parent:expr, $ops:expr, $drv_struct:ty, $member:ident) => {{
        const _: () = {
            // C static_assert(__same_type(...)) and offsetof(..., member) == 0.
            let _ = core::mem::offset_of!($drv_struct, $member);
        };
        _fwctl_alloc_device(
            $parent,
            $ops,
            core::mem::size_of::<$drv_struct>(),
        ) as *mut $drv_struct
    }};
}

pub unsafe fn fwctl_get(fwctl: *mut fwctl_device) -> *mut fwctl_device {
    unsafe { get_device(core::ptr::addr_of_mut!((*fwctl).dev)); }
    fwctl
}

pub unsafe fn fwctl_put(fwctl: *mut fwctl_device) {
    unsafe { put_device(core::ptr::addr_of_mut!((*fwctl).dev)); }
}

// DEFINE_FREE(fwctl, struct fwctl_device *, if (_T) fwctl_put(_T));

/**
 * struct fwctl_uctx - Per user FD context
 * @fwctl: fwctl instance that owns the context
 *
 * Every FD opened by userspace will get a unique context allocation. Any driver
 * private data will follow immediately after.
 */
#[repr(C)]
pub struct fwctl_uctx {
    pub fwctl: *mut fwctl_device,
    /* private: */
    /* Head at fwctl_device::uctx_list */
    pub uctx_list_entry: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
