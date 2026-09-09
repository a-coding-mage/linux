/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) IBM Corporation 2023 */

// C dependencies:
// #include <linux/cdev.h>
// #include <linux/device.h>

#[repr(C)]
pub struct fsi_master {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cdev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsi_slave {
    pub dev: device,
    pub master: *mut fsi_master,
    pub cdev: cdev,
    pub cdev_idx: i32,
    pub id: i32, /* FSI address */
    pub link: i32, /* FSI link# */
    pub cfam_id: u32,
    pub chip_id: i32,
    pub size: u32, /* size of slave address space */
    pub t_send_delay: u8,
    pub t_echo_delay: u8,
}

// Equivalent to: container_of(d, struct fsi_slave, dev)
#[inline]
pub unsafe fn to_fsi_slave(d: *mut device) -> *mut fsi_slave {
    (d as *mut u8).sub(core::mem::offset_of!(fsi_slave, dev)) as *mut fsi_slave
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
