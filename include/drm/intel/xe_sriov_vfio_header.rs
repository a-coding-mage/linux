/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2025 Intel Corporation
 */

// C dependency: <linux/types.h>

use core::ffi::c_char;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct xe_device {
    _private: [u8; 0],
}

/**
 * xe_sriov_vfio_get_pf() - Get PF &xe_device.
 * @pdev: the VF &pci_dev device
 *
 * Return: pointer to PF &xe_device, NULL otherwise.
 */
extern "C" {
    pub fn xe_sriov_vfio_get_pf(pdev: *mut pci_dev) -> *mut xe_device;

    /**
     * xe_sriov_vfio_migration_supported() - Check if migration is supported.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     *
     * Return: true if migration is supported, false otherwise.
     */
    pub fn xe_sriov_vfio_migration_supported(xe: *mut xe_device) -> bool;

    /**
     * xe_sriov_vfio_flr_prepare() - Notify PF that VF FLR prepare has started.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * This function marks VF FLR as pending before PF receives GuC FLR event.
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_flr_prepare(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_wait_flr_done() - Wait for VF FLR completion.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * This function will wait until VF FLR is processed by PF on all tiles (or
     * until timeout occurs).
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_wait_flr_done(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_suspend_device() - Suspend VF.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * This function will pause VF on all tiles/GTs.
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_suspend_device(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_resume_device() - Resume VF.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * This function will resume VF on all tiles.
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_resume_device(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_stop_copy_enter() - Initiate a VF device migration data save.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_stop_copy_enter(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_stop_copy_exit() - Finish a VF device migration data save.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_stop_copy_exit(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_resume_data_enter() - Initiate a VF device migration data restore.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_resume_data_enter(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_resume_data_exit() - Finish a VF device migration data restore.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_resume_data_exit(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_error() - Move VF device to error state.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * Reset is needed to move it out of error state.
     *
     * Return: 0 on success or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_error(xe: *mut xe_device, vfid: core::ffi::c_uint) -> core::ffi::c_int;

    /**
     * xe_sriov_vfio_data_read() - Read migration data from the VF device.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     * @buf: start address of userspace buffer
     * @len: requested read size from userspace
     *
     * Return: number of bytes that has been successfully read,
     *	   0 if no more migration data is available, -errno on failure.
     */
    pub fn xe_sriov_vfio_data_read(
        xe: *mut xe_device,
        vfid: core::ffi::c_uint,
        buf: *mut c_char,
        len: usize,
    ) -> isize;

    /**
     * xe_sriov_vfio_data_write() - Write migration data to the VF device.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     * @buf: start address of userspace buffer
     * @len: requested write size from userspace
     *
     * Return: number of bytes that has been successfully written, -errno on failure.
     */
    pub fn xe_sriov_vfio_data_write(
        xe: *mut xe_device,
        vfid: core::ffi::c_uint,
        buf: *const c_char,
        len: usize,
    ) -> isize;

    /**
     * xe_sriov_vfio_stop_copy_size() - Get a size estimate of VF device migration data.
     * @xe: the PF &xe_device obtained by calling xe_sriov_vfio_get_pf()
     * @vfid: the VF identifier (can't be 0)
     *
     * Return: migration data size in bytes or a negative error code on failure.
     */
    pub fn xe_sriov_vfio_stop_copy_size(xe: *mut xe_device, vfid: core::ffi::c_uint) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
