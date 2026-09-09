/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct UmlVfioUserDeviceRegion {
    pub size: u64,
    pub offset: u64,
}

#[repr(C)]
pub struct UmlVfioUserDevice {
    pub device: i32,
    pub region: *mut UmlVfioUserDeviceRegion,
    pub num_regions: i32,
    pub irqfd: *mut i32,
    pub irq_count: i32,
}

unsafe extern "C" {
    pub fn uml_vfio_user_open_container() -> i32;
    pub fn uml_vfio_user_setup_iommu(container: i32) -> i32;

    pub fn uml_vfio_user_get_group_id(device: *const c_char) -> i32;
    pub fn uml_vfio_user_open_group(group_id: i32) -> i32;
    pub fn uml_vfio_user_set_container(container: i32, group: i32) -> i32;
    pub fn uml_vfio_user_unset_container(container: i32, group: i32) -> i32;

    pub fn uml_vfio_user_setup_device(
        dev: *mut UmlVfioUserDevice,
        group: i32,
        device: *const c_char,
    ) -> i32;
    pub fn uml_vfio_user_teardown_device(dev: *mut UmlVfioUserDevice);

    pub fn uml_vfio_user_activate_irq(dev: *mut UmlVfioUserDevice, index: i32) -> i32;
    pub fn uml_vfio_user_deactivate_irq(dev: *mut UmlVfioUserDevice, index: i32);
    pub fn uml_vfio_user_update_irqs(dev: *mut UmlVfioUserDevice) -> i32;

    pub fn uml_vfio_user_cfgspace_read(
        dev: *mut UmlVfioUserDevice,
        offset: u32,
        buf: *mut c_void,
        size: i32,
    ) -> i32;
    pub fn uml_vfio_user_cfgspace_write(
        dev: *mut UmlVfioUserDevice,
        offset: u32,
        buf: *const c_void,
        size: i32,
    ) -> i32;

    pub fn uml_vfio_user_bar_read(
        dev: *mut UmlVfioUserDevice,
        bar: i32,
        offset: u32,
        buf: *mut c_void,
        size: i32,
    ) -> i32;
    pub fn uml_vfio_user_bar_write(
        dev: *mut UmlVfioUserDevice,
        bar: i32,
        offset: u32,
        buf: *const c_void,
        size: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
