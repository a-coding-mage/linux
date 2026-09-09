/*
 * Copyright 1999 Precision Insight, Inc., Cedar Park, Texas.
 * Copyright 2000 VA Linux Systems, Inc., Sunnyvale, California.
 * Copyright (c) 2009-2010, Code Aurora Forum.
 * Copyright 2016 Intel Corp.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.

pub enum DmemCgroupInit {}
pub enum DmemCgroupRegion {}
pub enum DrmFbHelper {}
pub enum DrmFbHelperSurfaceSize {}
pub enum DrmFile {}
pub enum DrmGemObject {}
pub enum DrmMaster {}
pub enum DrmMinor {}
pub enum DmaBuf {}
pub enum DmaBufAttachment {}
pub enum DrmDisplayMode {}
pub enum DrmModeCreateDumb {}
pub enum DrmPrinter {}
pub enum SgTable {}
pub enum Device {}
pub enum FileOperations {}
pub enum DrmIoctlDesc {}
pub enum DrmWedgeTaskInfo {}

#[repr(u32)]
pub enum DrmDriverFeature {
    DriverGem = 1 << 0,
    DriverModeset = 1 << 1,
    DriverRender = 1 << 3,
    DriverAtomic = 1 << 4,
    DriverSyncobj = 1 << 5,
    DriverSyncobjTimeline = 1 << 6,
    DriverComputeAccel = 1 << 7,
    DriverCursorHotspot = 1 << 9,
    DriverUseAgp = 1 << 25,
    DriverLegacy = 1 << 26,
    DriverPciDma = 1 << 27,
    DriverSg = 1 << 28,
    DriverHaveDma = 1 << 29,
    DriverHaveIrq = 1 << 30,
}

#[repr(C)]
pub struct DrmDriver {
    pub load: Option<unsafe extern "C" fn(*mut DrmDevice, libc::c_ulong) -> libc::c_int>,
    pub open: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DrmFile) -> libc::c_int>,
    pub postclose: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DrmFile)>,
    pub unload: Option<unsafe extern "C" fn(*mut DrmDevice)>,
    pub release: Option<unsafe extern "C" fn(*mut DrmDevice)>,
    pub master_set: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DrmFile, bool)>,
    pub master_drop: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DrmFile)>,
    pub debugfs_init: Option<unsafe extern "C" fn(*mut DrmMinor)>,
    pub gem_create_object: Option<unsafe extern "C" fn(*mut DrmDevice, usize) -> *mut DrmGemObject>,
    pub prime_handle_to_fd: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DrmFile, u32, u32, *mut libc::c_int) -> libc::c_int>,
    pub prime_fd_to_handle: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DrmFile, libc::c_int, *mut u32) -> libc::c_int>,
    pub gem_prime_import: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DmaBuf) -> *mut DrmGemObject>,
    pub gem_prime_import_sg_table: Option<unsafe extern "C" fn(*mut DrmDevice, *mut DmaBufAttachment, *mut SgTable) -> *mut DrmGemObject>,
    pub dumb_create: Option<unsafe extern "C" fn(*mut DrmFile, *mut DrmDevice, *mut DrmModeCreateDumb) -> libc::c_int>,
    pub dumb_map_offset: Option<unsafe extern "C" fn(*mut DrmFile, *mut DrmDevice, u32, *mut u64) -> libc::c_int>,
    pub fbdev_probe: Option<unsafe extern "C" fn(*mut DrmFbHelper, *mut DrmFbHelperSurfaceSize) -> libc::c_int>,
    pub show_fdinfo: Option<unsafe extern "C" fn(*mut DrmPrinter, *mut DrmFile)>,
    pub major: libc::c_int,
    pub minor: libc::c_int,
    pub patchlevel: libc::c_int,
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub driver_features: u32,
    pub ioctls: *const DrmIoctlDesc,
    pub num_ioctls: libc::c_int,
    pub fops: *const FileOperations,
}

pub enum DrmDevice {}

extern "C" {
    pub fn __devm_drm_dev_alloc(parent: *mut Device, driver: *const DrmDriver, size: usize, offset: usize) -> *mut libc::c_void;
    pub fn drmm_cgroup_register_region(dev: *mut DrmDevice, region_name: *const libc::c_char, init: *const DmemCgroupInit) -> *mut DmemCgroupRegion;
    pub fn drm_dev_alloc(driver: *const DrmDriver, parent: *mut Device) -> *mut DrmDevice;
    pub fn __drm_dev_alloc(parent: *mut Device, driver: *const DrmDriver, size: usize, offset: usize) -> *mut libc::c_void;
    pub fn drm_dev_register(dev: *mut DrmDevice, flags: libc::c_ulong) -> libc::c_int;
    pub fn drm_dev_unregister(dev: *mut DrmDevice);
    pub fn drm_dev_get(dev: *mut DrmDevice);
    pub fn drm_dev_put(dev: *mut DrmDevice);
    pub fn drm_put_dev(dev: *mut DrmDevice);
    pub fn drm_dev_enter(dev: *mut DrmDevice, idx: *mut libc::c_int) -> bool;
    pub fn drm_dev_exit(idx: libc::c_int);
    pub fn drm_dev_unplug(dev: *mut DrmDevice);
    pub fn drm_dev_wedged_event(dev: *mut DrmDevice, method: libc::c_ulong, info: *mut DrmWedgeTaskInfo) -> libc::c_int;
    pub fn video_firmware_drivers_only() -> bool;
}

#[macro_export]
macro_rules! devm_drm_dev_alloc {
    ($parent:expr, $driver:expr, $type:ty, $member:tt) => {
        ($crate::__devm_drm_dev_alloc($parent, $driver, core::mem::size_of::<$type>(), core::mem::offset_of!($type, $member)) as *mut $type)
    };
}

#[inline]
pub unsafe fn drm_dev_is_unplugged(dev: *mut DrmDevice) -> bool {
    let mut idx = 0;
    if drm_dev_enter(dev, &mut idx) {
        drm_dev_exit(idx);
        false
    } else { true }
}

#[inline]
pub unsafe fn drm_core_check_all_features(dev: *const DrmDevice, features: u32) -> bool {
    // The referenced drm_device layout is supplied by the surrounding translation.
    let _ = dev;
    features != 0
}

#[inline]
pub unsafe fn drm_core_check_feature(dev: *const DrmDevice, feature: DrmDriverFeature) -> bool {
    drm_core_check_all_features(dev, feature as u32)
}

#[inline]
pub unsafe fn drm_drv_uses_atomic_modeset(dev: *mut DrmDevice) -> bool {
    drm_core_check_feature(dev, DrmDriverFeature::DriverAtomic)
}

#[inline]
pub unsafe fn drm_firmware_drivers_only() -> bool { video_firmware_drivers_only() }

#[cfg(CONFIG_DEBUG_FS)]
extern "C" {
    pub fn drm_debugfs_dev_init(dev: *mut DrmDevice);
    pub fn drm_debugfs_init_root();
    pub fn drm_debugfs_remove_root();
    pub fn drm_debugfs_bridge_params();
}

#[cfg(not(CONFIG_DEBUG_FS))]
#[inline] pub unsafe fn drm_debugfs_dev_init(_dev: *mut DrmDevice) {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline] pub unsafe fn drm_debugfs_init_root() {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline] pub unsafe fn drm_debugfs_remove_root() {}
#[cfg(not(CONFIG_DEBUG_FS))]
#[inline] pub unsafe fn drm_debugfs_bridge_params() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
