// SPDX-License-Identifier: GPL-2.0
/*
 * Framebuffer driver for mdpy (mediated virtual pci display device).
 *
 * See mdpy-defs.h for device specs
 *
 *   (c) Gerd Hoffmann <kraxel@redhat.com>
 *
 * Using some code snippets from simplefb and cirrusfb.
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms and conditions of the GNU General Public License,
 * version 2, as published by the Free Software Foundation.
 */

// C dependencies supplied by the kernel and mdpy-defs.h are external.

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EIO: i32 = 5;
const FB_TYPE_PACKED_PIXELS: u32 = 0;
const FB_VISUAL_TRUECOLOR: u32 = 2;
const FB_ACCEL_NONE: u16 = 0;
const FB_ACTIVATE_NOW: u32 = 0;
const FB_VMODE_NONINTERLACED: u32 = 0;
const DRM_FORMAT_XRGB8888: u32 = 0x34325258;
const PSEUDO_PALETTE_SIZE: usize = 16;

type U32 = u32;
type UInt = u32;

#[repr(C)]
#[derive(Copy, Clone)]
struct FbBitfield { offset: u32, length: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
struct FbFixScreeninfo {
    id: [u8; 16],
    smem_start: usize,
    smem_len: u32,
    type_: u32,
    visual: u32,
    type_aux: u16,
    xpanstep: u16,
    ypanstep: u16,
    ywrapstep: u16,
    line_length: u32,
    mmio_start: usize,
    mmio_len: u32,
    accel: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FbVarScreeninfo {
    xres: u32, yres: u32, xres_virtual: u32, yres_virtual: u32,
    xoffset: u32, yoffset: u32, bits_per_pixel: u32, grayscale: u32,
    red: FbBitfield, green: FbBitfield, blue: FbBitfield, transp: FbBitfield,
    nonstd: u32, activate: u32, height: u32, width: u32, accel_flags: u32,
    pixclock: u32, left_margin: u32, right_margin: u32, upper_margin: u32,
    lower_margin: u32, hsync_len: u32, vsync_len: u32, sync: u32,
    vmode: u32, rotate: u32, colorspace: u32, reserved: [u32; 4],
}

#[repr(C)]
struct FbInfo {
    fix: FbFixScreeninfo,
    var: FbVarScreeninfo,
    screen_size: usize,
    screen_base: *mut core::ffi::c_void,
    fbops: *const FbOps,
    pseudo_palette: *mut u32,
    par: *mut core::ffi::c_void,
    node: i32,
}

#[repr(C)] struct PciDev;
#[repr(C)] struct PciDeviceId { vendor: u32, device: u32, subvendor: u32, subdevice: u32 }
#[repr(C)] struct Device;

type Setcolreg = unsafe extern "C" fn(UInt, UInt, UInt, UInt, UInt, *mut FbInfo) -> i32;
type Destroy = unsafe extern "C" fn(*mut FbInfo);
#[repr(C)] struct FbOps { owner: *const core::ffi::c_void, fb_destroy: Option<Destroy>, fb_setcolreg: Option<Setcolreg> }

#[repr(C)]
struct MdpyFbPar { palette: [U32; PSEUDO_PALETTE_SIZE] }

#[repr(C)]
struct PciDriver { name: *const u8, id_table: *mut PciDeviceId, probe: Option<unsafe extern "C" fn(*mut PciDev, *const PciDeviceId) -> i32>, remove: Option<unsafe extern "C" fn(*mut PciDev)> }

extern "C" {
    fn pci_enable_device(pdev: *mut PciDev) -> i32;
    fn pci_request_regions(pdev: *mut PciDev, name: *const u8) -> i32;
    fn pci_read_config_dword(pdev: *mut PciDev, offset: u32, value: *mut u32);
    fn pci_set_drvdata(pdev: *mut PciDev, data: *mut FbInfo);
    fn pci_get_drvdata(pdev: *mut PciDev) -> *mut FbInfo;
    fn pci_resource_start(pdev: *mut PciDev, bar: u32) -> usize;
    fn pci_resource_len(pdev: *mut PciDev, bar: u32) -> u32;
    fn pci_release_regions(pdev: *mut PciDev);
    fn pci_disable_device(pdev: *mut PciDev);
    fn framebuffer_alloc(size: usize, dev: *mut Device) -> *mut FbInfo;
    fn framebuffer_release(info: *mut FbInfo);
    fn register_framebuffer(info: *mut FbInfo) -> i32;
    fn unregister_framebuffer(info: *mut FbInfo);
    fn ioremap(start: usize, size: usize) -> *mut core::ffi::c_void;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn pci_register_driver(driver: *mut PciDriver) -> i32;
}

static mut MDPY_FB_FIX: FbFixScreeninfo = FbFixScreeninfo { id: *b"mdpy-fb\0\0\0\0\0\0\0\0", smem_start: 0, smem_len: 0, type_: FB_TYPE_PACKED_PIXELS, visual: FB_VISUAL_TRUECOLOR, type_aux: 0, xpanstep: 0, ypanstep: 0, ywrapstep: 0, line_length: 0, mmio_start: 0, mmio_len: 0, accel: FB_ACCEL_NONE };
static mut MDPY_FB_VAR: FbVarScreeninfo = FbVarScreeninfo { xres: 0, yres: 0, xres_virtual: 0, yres_virtual: 0, xoffset: 0, yoffset: 0, bits_per_pixel: 32, grayscale: 0, red: FbBitfield { offset: 16, length: 8 }, green: FbBitfield { offset: 8, length: 8 }, blue: FbBitfield { offset: 0, length: 8 }, transp: FbBitfield { offset: 24, length: 8 }, nonstd: 0, activate: FB_ACTIVATE_NOW, height: u32::MAX, width: u32::MAX, accel_flags: 0, pixclock: 0, left_margin: 0, right_margin: 0, upper_margin: 0, lower_margin: 0, hsync_len: 0, vsync_len: 0, sync: 0, vmode: FB_VMODE_NONINTERLACED, rotate: 0, colorspace: 0, reserved: [0; 4] };

unsafe extern "C" fn mdpy_fb_setcolreg(regno: UInt, red: UInt, green: UInt, blue: UInt, _transp: UInt, info: *mut FbInfo) -> i32 {
    let pal = (*info).pseudo_palette;
    let cr = red >> (16 - (*info).var.red.length);
    let cg = green >> (16 - (*info).var.green.length);
    let cb = blue >> (16 - (*info).var.blue.length);
    if regno >= PSEUDO_PALETTE_SIZE as u32 { return -EINVAL; }
    let mut value = (cr << (*info).var.red.offset) | (cg << (*info).var.green.offset) | (cb << (*info).var.blue.offset);
    if (*info).var.transp.length > 0 {
        let mut mask = (1u32 << (*info).var.transp.length) - 1;
        mask <<= (*info).var.transp.offset;
        value |= mask;
    }
    *pal.add(regno as usize) = value;
    0
}

unsafe extern "C" fn mdpy_fb_destroy(info: *mut FbInfo) { if !(*info).screen_base.is_null() { iounmap((*info).screen_base); } }

static mut MDPY_FB_OPS: FbOps = FbOps { owner: core::ptr::null(), fb_destroy: Some(mdpy_fb_destroy), fb_setcolreg: Some(mdpy_fb_setcolreg) };

unsafe extern "C" fn mdpy_fb_probe(pdev: *mut PciDev, _ent: *const PciDeviceId) -> i32 {
    let mut format = 0u32; let mut width = 0u32; let mut height = 0u32;
    let ret = pci_enable_device(pdev); if ret < 0 { return ret; }
    let ret = pci_request_regions(pdev, b"mdpy-fb\0".as_ptr()); if ret < 0 { pci_disable_device(pdev); return ret; }
    pci_read_config_dword(pdev, MDPY_FORMAT_OFFSET, &mut format); pci_read_config_dword(pdev, MDPY_WIDTH_OFFSET, &mut width); pci_read_config_dword(pdev, MDPY_HEIGHT_OFFSET, &mut height);
    if format != DRM_FORMAT_XRGB8888 || width < 100 || width > 10000 || height < 100 || height > 10000 { pci_release_regions(pdev); pci_disable_device(pdev); return -EINVAL; }
    let info = framebuffer_alloc(core::mem::size_of::<MdpyFbPar>(), core::ptr::null_mut()); if info.is_null() { pci_release_regions(pdev); pci_disable_device(pdev); return -ENOMEM; }
    pci_set_drvdata(pdev, info); let par = (*info).par as *mut MdpyFbPar;
    (*info).fix = MDPY_FB_FIX; (*info).fix.smem_start = pci_resource_start(pdev, 0); (*info).fix.smem_len = pci_resource_len(pdev, 0); (*info).fix.line_length = width * 4;
    (*info).var = MDPY_FB_VAR; (*info).var.xres = width; (*info).var.yres = height; (*info).var.xres_virtual = width; (*info).var.yres_virtual = height;
    (*info).screen_size = (*info).fix.smem_len as usize; (*info).screen_base = ioremap((*info).fix.smem_start, (*info).screen_size);
    if (*info).screen_base.is_null() { framebuffer_release(info); pci_release_regions(pdev); pci_disable_device(pdev); return -EIO; }
    (*info).fbops = &MDPY_FB_OPS; (*info).pseudo_palette = (*par).palette.as_mut_ptr();
    let ret = register_framebuffer(info); if ret < 0 { iounmap((*info).screen_base); framebuffer_release(info); pci_release_regions(pdev); pci_disable_device(pdev); return ret; } 0
}

unsafe extern "C" fn mdpy_fb_remove(pdev: *mut PciDev) { let info = pci_get_drvdata(pdev); unregister_framebuffer(info); iounmap((*info).screen_base); framebuffer_release(info); pci_release_regions(pdev); pci_disable_device(pdev); }

static mut MDPY_FB_PCI_TABLE: [PciDeviceId; 2] = [PciDeviceId { vendor: MDPY_PCI_VENDOR_ID, device: MDPY_PCI_DEVICE_ID, subvendor: MDPY_PCI_SUBVENDOR_ID, subdevice: MDPY_PCI_SUBDEVICE_ID }, PciDeviceId { vendor: 0, device: 0, subvendor: 0, subdevice: 0 }];
static mut MDPY_FB_PCI_DRIVER: PciDriver = PciDriver { name: b"mdpy-fb\0".as_ptr(), id_table: unsafe { MDPY_FB_PCI_TABLE.as_mut_ptr() }, probe: Some(mdpy_fb_probe), remove: Some(mdpy_fb_remove) };

unsafe extern "C" fn mdpy_fb_init() -> i32 { let ret = pci_register_driver(&mut MDPY_FB_PCI_DRIVER); if ret != 0 { return ret; } 0 }

// module_init(mdpy_fb_init); MODULE_DEVICE_TABLE(pci, mdpy_fb_pci_table);
// MODULE_DESCRIPTION("Framebuffer driver for mdpy (mediated virtual pci display device)");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
