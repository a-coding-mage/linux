/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * apple-gmux.h - microcontroller built into dual GPU MacBook Pro & Mac Pro
 * Copyright (C) 2015 Lukas Wunner <lukas@wunner.de>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const GMUX_ACPI_HID: &str = "APP000B";

/* gmux port offsets. */
pub const GMUX_PORT_VERSION_MAJOR: usize = 0x04;
pub const GMUX_PORT_VERSION_MINOR: usize = 0x05;
pub const GMUX_PORT_VERSION_RELEASE: usize = 0x06;
pub const GMUX_PORT_SWITCH_DISPLAY: usize = 0x10;
pub const GMUX_PORT_SWITCH_GET_DISPLAY: usize = 0x11;
pub const GMUX_PORT_INTERRUPT_ENABLE: usize = 0x14;
pub const GMUX_PORT_INTERRUPT_STATUS: usize = 0x16;
pub const GMUX_PORT_SWITCH_DDC: usize = 0x28;
pub const GMUX_PORT_SWITCH_EXTERNAL: usize = 0x40;
pub const GMUX_PORT_SWITCH_GET_EXTERNAL: usize = 0x41;
pub const GMUX_PORT_DISCRETE_POWER: usize = 0x50;
pub const GMUX_PORT_MAX_BRIGHTNESS: usize = 0x70;
pub const GMUX_PORT_BRIGHTNESS: usize = 0x74;
pub const GMUX_PORT_VALUE: usize = 0xc2;
pub const GMUX_PORT_READ: usize = 0xd0;
pub const GMUX_PORT_WRITE: usize = 0xd4;

pub const GMUX_MMIO_PORT_SELECT: usize = 0x0e;
pub const GMUX_MMIO_COMMAND_SEND: usize = 0x0f;
pub const GMUX_MMIO_READ: usize = 0x00;
pub const GMUX_MMIO_WRITE: usize = 0x40;
pub const GMUX_MIN_IO_LEN: usize = GMUX_PORT_BRIGHTNESS + 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum apple_gmux_type {
    APPLE_GMUX_TYPE_PIO,
    APPLE_GMUX_TYPE_INDEXED,
    APPLE_GMUX_TYPE_MMIO,
}

#[cfg(feature = "CONFIG_APPLE_GMUX")]
pub unsafe fn apple_gmux_is_indexed(iostart: libc::c_ulong) -> bool {
    let val: u16;
    outb(0xaa, iostart + 0xcc);
    outb(0x55, iostart + 0xcd);
    outb(0x00, iostart + 0xce);
    val = (inb(iostart + 0xcc) as u16) | ((inb(iostart + 0xcd) as u16) << 8);
    val == 0x55aa
}

#[cfg(feature = "CONFIG_APPLE_GMUX")]
pub unsafe fn apple_gmux_is_mmio(iostart: libc::c_ulong) -> bool {
    let iomem_base = ioremap(iostart, 16);
    if iomem_base.is_null() {
        return false;
    }
    let val = ioread8(iomem_base.add(GMUX_MMIO_COMMAND_SEND));
    iounmap(iomem_base);
    val != 0xff
}

#[cfg(feature = "CONFIG_APPLE_GMUX")]
pub unsafe fn apple_gmux_detect(
    mut pnp_dev: *mut pnp_dev,
    type_ret: *mut apple_gmux_type,
) -> bool {
    let mut dev: *mut device = core::ptr::null_mut();
    let mut res: *mut resource;
    let mut ty = apple_gmux_type::APPLE_GMUX_TYPE_PIO;

    if pnp_dev.is_null() {
        let adev = acpi_dev_get_first_match_dev(GMUX_ACPI_HID.as_ptr() as *const _, core::ptr::null(), -1);
        if adev.is_null() { return false; }
        dev = get_device(acpi_get_first_physical_node(adev));
        acpi_dev_put(adev);
        if dev.is_null() { return false; }
        pnp_dev = to_pnp_dev(dev);
    }

    res = pnp_get_resource(pnp_dev, IORESOURCE_IO, 0);
    if !res.is_null() && resource_size(res) >= GMUX_MIN_IO_LEN {
        let ver_major = inb((*res).start + GMUX_PORT_VERSION_MAJOR as _);
        let ver_minor = inb((*res).start + GMUX_PORT_VERSION_MINOR as _);
        let ver_release = inb((*res).start + GMUX_PORT_VERSION_RELEASE as _);
        if ver_major == 0xff && ver_minor == 0xff && ver_release == 0xff {
            if apple_gmux_is_indexed((*res).start) { ty = apple_gmux_type::APPLE_GMUX_TYPE_INDEXED; }
            else { put_device(dev); return false; }
        }
    } else {
        res = pnp_get_resource(pnp_dev, IORESOURCE_MEM, 0);
        if !res.is_null() && apple_gmux_is_mmio((*res).start) { ty = apple_gmux_type::APPLE_GMUX_TYPE_MMIO; }
        else { put_device(dev); return false; }
    }
    if !type_ret.is_null() { *type_ret = ty; }
    put_device(dev);
    true
}

#[cfg(feature = "CONFIG_APPLE_GMUX")]
pub unsafe fn apple_gmux_present() -> bool { acpi_dev_found(GMUX_ACPI_HID.as_ptr() as *const _) }

#[cfg(not(feature = "CONFIG_APPLE_GMUX"))]
pub unsafe fn apple_gmux_present() -> bool { false }

#[cfg(not(feature = "CONFIG_APPLE_GMUX"))]
pub unsafe fn apple_gmux_detect(_pnp_dev: *mut pnp_dev, _indexed_ret: *mut bool) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
