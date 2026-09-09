/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/* QLogic qedr NIC Driver
 * Copyright (c) 2015-2017  QLogic Corporation
 * Copyright (c) 2019-2020 Marvell International Ltd.
 */

// C dependencies supplied by the surrounding kernel/Rust translation.

use core::ffi::c_void;

#[repr(C)]
pub struct qedr_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qed_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qede_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum qede_rdma_event {
    QEDE_UP,
    QEDE_DOWN,
    QEDE_CHANGE_ADDR,
    QEDE_CLOSE,
    QEDE_CHANGE_MTU,
}

#[repr(C)]
pub struct qede_rdma_event_work {
    pub list: list_head,
    pub work: work_struct,
    pub ptr: *mut c_void,
    pub event: qede_rdma_event,
}

#[repr(C)]
pub struct qedr_driver {
    pub name: [u8; 32],
    pub add: Option<unsafe extern "C" fn(
        *mut qed_dev,
        *mut pci_dev,
        *mut net_device,
    ) -> *mut qedr_dev>,
    pub remove: Option<unsafe extern "C" fn(*mut qedr_dev)>,
    pub notify: Option<unsafe extern "C" fn(*mut qedr_dev, qede_rdma_event)>,
}

/* APIs for RDMA driver to register callback handlers,
 * which will be invoked when device is added, removed, ifup, ifdown
 */
extern "C" {
    pub fn qede_rdma_register_driver(drv: *mut qedr_driver) -> i32;
    pub fn qede_rdma_unregister_driver(drv: *mut qedr_driver);

    pub fn qede_rdma_supported(dev: *mut qede_dev) -> bool;
}

// Equivalent to the build-time CONFIG_QED_RDMA condition.
#[cfg(feature = "CONFIG_QED_RDMA")]
extern "C" {
    pub fn qede_rdma_dev_add(dev: *mut qede_dev, recovery: bool) -> i32;
    pub fn qede_rdma_dev_event_open(dev: *mut qede_dev);
    pub fn qede_rdma_dev_event_close(dev: *mut qede_dev);
    pub fn qede_rdma_dev_remove(dev: *mut qede_dev, recovery: bool);
    pub fn qede_rdma_event_changeaddr(edr: *mut qede_dev);
    pub fn qede_rdma_event_change_mtu(edev: *mut qede_dev);
}

#[cfg(not(feature = "CONFIG_QED_RDMA"))]
pub unsafe fn qede_rdma_dev_add(_dev: *mut qede_dev, _recovery: bool) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_QED_RDMA"))]
pub unsafe fn qede_rdma_dev_event_open(_dev: *mut qede_dev) {}

#[cfg(not(feature = "CONFIG_QED_RDMA"))]
pub unsafe fn qede_rdma_dev_event_close(_dev: *mut qede_dev) {}

#[cfg(not(feature = "CONFIG_QED_RDMA"))]
pub unsafe fn qede_rdma_dev_remove(_dev: *mut qede_dev, _recovery: bool) {}

#[cfg(not(feature = "CONFIG_QED_RDMA"))]
pub unsafe fn qede_rdma_event_changeaddr(_edr: *mut qede_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
