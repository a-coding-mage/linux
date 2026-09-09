// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024 Intel Corporation */
// Dependencies supplied by the corresponding Linux QAT headers and driver sources.

extern "C" {
    fn adf_devmgr_pci_to_accel_dev(pdev: *mut pci_dev) -> *mut adf_accel_dev;
    fn kmalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn ERR_PTR(error: core::ffi::c_long) -> *mut qat_mig_dev;
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qat_migdev_ops {
    pub init: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut qat_mig_dev)>,
    pub reset: Option<unsafe extern "C" fn(*mut qat_mig_dev)>,
    pub open: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(*mut qat_mig_dev)>,
    pub suspend: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub save_state: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub load_state: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub save_setup: Option<unsafe extern "C" fn(*mut qat_mig_dev) -> core::ffi::c_int>,
    pub load_setup: Option<unsafe extern "C" fn(*mut qat_mig_dev, core::ffi::c_int) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct qat_mig_dev {
    pub vf_id: core::ffi::c_int,
    pub parent_accel_dev: *mut adf_accel_dev,
}

// GET_VFMIG_OPS(accel_dev) is provided by the QAT driver headers.
extern "C" {
    fn GET_VFMIG_OPS(accel_dev: *mut adf_accel_dev) -> *mut qat_migdev_ops;
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_create(
    pdev: *mut pci_dev,
    vf_id: core::ffi::c_int,
) -> *mut qat_mig_dev {
    let accel_dev = adf_devmgr_pci_to_accel_dev(pdev);
    if accel_dev.is_null() {
        return ERR_PTR(-19);
    }

    let ops = GET_VFMIG_OPS(accel_dev);
    if ops.is_null()
        || (*ops).init.is_none()
        || (*ops).cleanup.is_none()
        || (*ops).reset.is_none()
        || (*ops).open.is_none()
        || (*ops).close.is_none()
        || (*ops).suspend.is_none()
        || (*ops).resume.is_none()
        || (*ops).save_state.is_none()
        || (*ops).load_state.is_none()
        || (*ops).save_setup.is_none()
        || (*ops).load_setup.is_none()
    {
        return ERR_PTR(-22);
    }

    let mdev = kmalloc_obj::<qat_mig_dev>();
    if mdev.is_null() {
        return ERR_PTR(-12);
    }

    (*mdev).vf_id = vf_id;
    (*mdev).parent_accel_dev = accel_dev;

    mdev
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_init(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).init.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_cleanup(mdev: *mut qat_mig_dev) {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).cleanup.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_reset(mdev: *mut qat_mig_dev) {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).reset.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_open(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).open.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_close(mdev: *mut qat_mig_dev) {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).close.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_suspend(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).suspend.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_resume(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).resume.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_save_state(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).save_state.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_save_setup(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).save_setup.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_load_state(mdev: *mut qat_mig_dev) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).load_state.unwrap())(mdev)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_load_setup(
    mdev: *mut qat_mig_dev,
    size: core::ffi::c_int,
) -> core::ffi::c_int {
    let accel_dev = (*mdev).parent_accel_dev;
    ((*GET_VFMIG_OPS(accel_dev)).load_setup.unwrap())(mdev, size)
}

#[no_mangle]
pub unsafe extern "C" fn qat_vfmig_destroy(mdev: *mut qat_mig_dev) {
    kfree(mdev.cast());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
