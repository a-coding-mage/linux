/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Simple pci display device.
 *
 * Framebuffer memory is pci bar 0.
 * Configuration (read-only) is in pci config space.
 * Format field uses drm fourcc codes.
 * ATM only DRM_FORMAT_XRGB8888 is supported.
 */

/* pci ids */
pub const MDPY_PCI_VENDOR_ID: u16 = PCI_VENDOR_ID_REDHAT;
pub const MDPY_PCI_DEVICE_ID: u16 = 0x000f;
pub const MDPY_PCI_SUBVENDOR_ID: u16 = PCI_SUBVENDOR_ID_REDHAT_QUMRANET;
pub const MDPY_PCI_SUBDEVICE_ID: u16 = PCI_SUBDEVICE_ID_QEMU;

/* pci cfg space offsets for fb config (dword) */
pub const MDPY_VENDORCAP_OFFSET: u32 = 0x40;
pub const MDPY_VENDORCAP_SIZE: u32 = 0x10;
pub const MDPY_FORMAT_OFFSET: u32 = MDPY_VENDORCAP_OFFSET + 0x04;
pub const MDPY_WIDTH_OFFSET: u32 = MDPY_VENDORCAP_OFFSET + 0x08;
pub const MDPY_HEIGHT_OFFSET: u32 = MDPY_VENDORCAP_OFFSET + 0x0c;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
