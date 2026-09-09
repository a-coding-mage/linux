/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/module.h, linux/debugfs.h, and drbd_int.h.

// C build-time condition: CONFIG_DEBUG_FS.
#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn drbd_debugfs_init();
    pub fn drbd_debugfs_cleanup();

    pub fn drbd_debugfs_resource_add(resource: *mut drbd_resource);
    pub fn drbd_debugfs_resource_cleanup(resource: *mut drbd_resource);

    pub fn drbd_debugfs_connection_add(connection: *mut drbd_connection);
    pub fn drbd_debugfs_connection_cleanup(connection: *mut drbd_connection);

    pub fn drbd_debugfs_device_add(device: *mut drbd_device);
    pub fn drbd_debugfs_device_cleanup(device: *mut drbd_device);

    pub fn drbd_debugfs_peer_device_add(peer_device: *mut drbd_peer_device);
    pub fn drbd_debugfs_peer_device_cleanup(peer_device: *mut drbd_peer_device);
}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_init() {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_cleanup() {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_resource_add(_resource: *mut drbd_resource) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_resource_cleanup(_resource: *mut drbd_resource) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_connection_add(_connection: *mut drbd_connection) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_connection_cleanup(_connection: *mut drbd_connection) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_device_add(_device: *mut drbd_device) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_device_cleanup(_device: *mut drbd_device) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_peer_device_add(_peer_device: *mut drbd_peer_device) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn drbd_debugfs_peer_device_cleanup(_peer_device: *mut drbd_peer_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
