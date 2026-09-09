/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_property {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn drm_class_device_register(dev: *mut device) -> i32;
    pub fn drm_class_device_unregister(dev: *mut device);

    pub fn drm_sysfs_hotplug_event(dev: *mut drm_device);
    pub fn drm_sysfs_connector_hotplug_event(connector: *mut drm_connector);
    pub fn drm_sysfs_connector_property_event(
        connector: *mut drm_connector,
        property: *mut drm_property,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
