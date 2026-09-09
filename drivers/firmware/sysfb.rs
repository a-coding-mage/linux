// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic System Framebuffers
 * Copyright (c) 2012-2013 David Herrmann <dh.herrmann@gmail.com>
 */

/*
 * Simple-Framebuffer support
 * Create a platform-device for any available boot framebuffer. The
 * simple-framebuffer platform device is already available on DT systems, so
 * this module parses the global "screen_info" object and creates a suitable
 * platform device compatible with the "simple-framebuffer" DT object. If the
 * framebuffer is incompatible, we instead create a legacy
 * "vesa-framebuffer", "efi-framebuffer" or "platform-framebuffer" device and
 * pass the screen_info as platform_data. This allows legacy drivers
 * to pick these devices up without messing with simple-framebuffer drivers.
 * The global "screen_info" is still valid at all times.
 *
 * If CONFIG_SYSFB_SIMPLEFB is not selected, never register "simple-framebuffer"
 * platform devices, but only use legacy framebuffer devices for
 * backwards compatibility.
 *
 * TODO: We set the dev_id field of all platform-devices to 0. This allows
 * other OF/DT parsers to create such devices, too. However, they must
 * start at offset 1 for this to work.
 */

use core::ffi::c_void;

#[repr(C)]
pub struct device { pub parent: *mut device, _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { pub dev: device, _private: [u8; 0] }
#[repr(C)]
pub struct pci_dev { pub dev: device, _private: [u8; 0] }
#[repr(C)]
pub struct screen_info { _private: [u8; 0] }
#[repr(C)]
pub struct sysfb_display_info { pub screen: screen_info, _private: [u8; 0] }
#[repr(C)]
pub struct simplefb_platform_data { _private: [u8; 0] }

extern "C" {
    static mut sysfb_primary_display: sysfb_display_info;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn screen_info_video_type(si: *const screen_info) -> u32;
    fn screen_info_pci_dev(si: *const screen_info) -> *mut pci_dev;
    fn pci_read_config_word(pdev: *mut pci_dev, where_: u32, val: *mut u16) -> i32;
    fn pci_dev_put(pdev: *mut pci_dev);
    fn screen_info_apply_fixups();
    fn sysfb_apply_efi_quirks(si: *mut screen_info);
    fn sysfb_parse_mode(si: *const screen_info, mode: *mut simplefb_platform_data) -> bool;
    fn sysfb_create_simplefb(si: *const screen_info, mode: *const simplefb_platform_data,
                             parent: *mut device) -> *mut platform_device;
    fn platform_device_alloc(name: *const i8, id: i32) -> *mut platform_device;
    fn sysfb_set_efifb_fwnode(si: *const screen_info, pdev: *mut platform_device);
    fn platform_device_add_data(pdev: *mut platform_device, data: *const c_void, size: usize) -> i32;
    fn platform_device_add(pdev: *mut platform_device) -> i32;
    fn platform_device_put(pdev: *mut platform_device);
    fn put_device(dev: *mut device);
}

static mut pd: *mut platform_device = core::ptr::null_mut();
static mut disable_lock: *mut c_void = core::ptr::null_mut(); // DEFINE_MUTEX(disable_lock)
static mut disabled: bool = false;

const PCI_COMMAND: u32 = 0x04;
const PCI_COMMAND_MEMORY: u16 = 0x0002;
const PCIBIOS_SUCCESSFUL: i32 = 0;
const ENODEV: isize = 19;
const ENOMEM: i32 = 12;
const VIDEO_TYPE_EGAC: u32 = 0x10;
const VIDEO_TYPE_VGAC: u32 = 0x11;
const VIDEO_TYPE_VLFB: u32 = 0x12;
const VIDEO_TYPE_EFI: u32 = 0x13;

unsafe fn sysfb_unregister() -> bool {
    if pd.is_null() { return false; }
    platform_device_unregister(pd);
    pd = core::ptr::null_mut();
    true
}

/// sysfb_disable() - disable the Generic System Framebuffers support
pub unsafe extern "C" fn sysfb_disable(dev: *mut device) {
    let si = &mut sysfb_primary_display.screen as *mut screen_info;
    mutex_lock(disable_lock);
    let parent = sysfb_parent_dev(si);
    if dev.is_null() || parent.is_null() || dev == parent {
        sysfb_unregister();
        disabled = true;
    }
    mutex_unlock(disable_lock);
}

/// sysfb_handles_screen_info() - reports if sysfb handles the global screen_info
pub unsafe extern "C" fn sysfb_handles_screen_info() -> bool {
    let si = &sysfb_primary_display.screen as *const screen_info;
    screen_info_video_type(si) != 0
}

#[cfg(feature = "CONFIG_PCI")]
unsafe fn sysfb_pci_dev_is_enabled(pdev: *mut pci_dev) -> bool {
    let mut command = 0u16;
    if pci_read_config_word(pdev, PCI_COMMAND, &mut command) != PCIBIOS_SUCCESSFUL { return false; }
    command & PCI_COMMAND_MEMORY != 0
}

#[cfg(not(feature = "CONFIG_PCI"))]
unsafe fn sysfb_pci_dev_is_enabled(_pdev: *mut pci_dev) -> bool { false }

unsafe fn sysfb_parent_dev(si: *const screen_info) -> *mut device {
    let pdev = screen_info_pci_dev(si);
    if pdev.is_null() { return core::ptr::null_mut(); }
    if !sysfb_pci_dev_is_enabled(pdev) {
        pci_dev_put(pdev);
        return (-ENODEV) as *mut device;
    }
    &mut (*pdev).dev
}

unsafe fn sysfb_init() -> i32 {
    let dpy = &mut sysfb_primary_display as *mut sysfb_display_info;
    let si = &mut (*dpy).screen as *mut screen_info;
    let mut ret = 0;
    screen_info_apply_fixups();
    mutex_lock(disable_lock);
    if disabled { mutex_unlock(disable_lock); return ret; }
    sysfb_apply_efi_quirks(si);
    let parent = sysfb_parent_dev(si);
    if parent as isize == (-ENODEV) { mutex_unlock(disable_lock); return (-ENODEV) as i32; }
    let mut mode = simplefb_platform_data { _private: [] };
    if sysfb_parse_mode(si, &mut mode) {
        pd = sysfb_create_simplefb(si, &mode, parent);
        if !pd.is_null() { put_device(parent); mutex_unlock(disable_lock); return ret; }
    }
    let name = match screen_info_video_type(si) {
        VIDEO_TYPE_EGAC => b"ega-framebuffer\0",
        VIDEO_TYPE_VGAC => b"vga-framebuffer\0",
        VIDEO_TYPE_VLFB => b"vesa-framebuffer\0",
        VIDEO_TYPE_EFI => b"efi-framebuffer\0",
        _ => b"platform-framebuffer\0",
    };
    pd = platform_device_alloc(name.as_ptr() as *const i8, 0);
    if pd.is_null() { ret = -ENOMEM; put_device(parent); mutex_unlock(disable_lock); return ret; }
    (*pd).dev.parent = parent;
    sysfb_set_efifb_fwnode(si, pd);
    ret = platform_device_add_data(pd, dpy as *const c_void, core::mem::size_of::<sysfb_display_info>());
    if ret == 0 { ret = platform_device_add(pd); }
    if ret != 0 { platform_device_put(pd); }
    put_device(parent);
    mutex_unlock(disable_lock);
    ret
}

// device_initcall(sysfb_init); must execute after PCI subsystem for EFI quirks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
