/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Transport specific attributes.
 *
 * Copyright (c) 2003 Silicon Graphics, Inc.  All rights reserved.
 */

// Dependencies supplied by the corresponding Linux/SCSI headers are intentionally
// referenced here rather than redefined.

#[repr(C)]
pub struct scsi_transport_template {
    /* the attribute containers */
    pub host_attrs: transport_container,
    pub target_attrs: transport_container,
    pub device_attrs: transport_container,

    /* If set, called from sysfs and legacy procfs rescanning code. */
    pub user_scan: Option<unsafe extern "C" fn(*mut Scsi_Host, u32, u32, u64) -> i32>,

    /* The size of the specific transport attribute structure. */
    pub device_size: i32,
    pub device_private_offset: i32,
    pub target_size: i32,
    pub target_private_offset: i32,
    pub host_size: i32,
    /* no private offset for the host; there's an alternative mechanism */

    /* True if the transport wants to use a host-based work-queue */
    pub create_work_queue: u32,

    /* Allows a transport to override the default error handler. */
    pub eh_strategy_handler: Option<unsafe extern "C" fn(*mut Scsi_Host)>,
}

/* transport_class_to_shost(tc) -> dev_to_shost((tc)->parent) */
#[inline]
pub unsafe fn transport_class_to_shost(tc: *mut transport_container) -> *mut Scsi_Host {
    dev_to_shost((*tc).parent)
}

/* Private area maintenance. The driver requested allocations come directly
 * after the transport class allocations (if any).  The idea is that you must
 * call these only once. */
#[inline]
pub unsafe fn scsi_transport_reserve_target(
    t: *mut scsi_transport_template,
    space: i32,
) {
    BUG_ON((*t).target_private_offset != 0);
    (*t).target_private_offset = align((*t).target_size, core::mem::size_of::<*mut core::ffi::c_void>() as i32);
    (*t).target_size = (*t).target_private_offset + space;
}

#[inline]
pub unsafe fn scsi_transport_reserve_device(
    t: *mut scsi_transport_template,
    space: i32,
) {
    BUG_ON((*t).device_private_offset != 0);
    (*t).device_private_offset = align((*t).device_size, core::mem::size_of::<*mut core::ffi::c_void>() as i32);
    (*t).device_size = (*t).device_private_offset + space;
}

#[inline]
pub unsafe fn scsi_transport_target_data(starget: *mut scsi_target) -> *mut core::ffi::c_void {
    let shost: *mut Scsi_Host = dev_to_shost(&mut (*starget).dev);
    ((*starget).starget_data as *mut u8)
        .add((*(*shost).transportt).target_private_offset as usize)
        as *mut core::ffi::c_void
}

#[inline]
pub unsafe fn scsi_transport_device_data(sdev: *mut scsi_device) -> *mut core::ffi::c_void {
    let shost: *mut Scsi_Host = (*sdev).host;
    ((*sdev).sdev_data as *mut u8)
        .add((*(*shost).transportt).device_private_offset as usize)
        as *mut core::ffi::c_void
}

pub unsafe extern "C" fn scsi_init_limits(shost: *mut Scsi_Host, lim: *mut queue_limits);

// External symbols/macros provided by the included Linux headers.
unsafe extern "C" {
    fn dev_to_shost(dev: *mut device) -> *mut Scsi_Host;
    fn BUG_ON(condition: bool);
}

#[inline]
fn align(value: i32, alignment: i32) -> i32 {
    (value + alignment - 1) & !(alignment - 1)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
