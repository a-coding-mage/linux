// SPDX-License-Identifier: GPL-2.0
/*
 * Mediated virtual PCI display host device driver
 *
 * See mdpy-defs.h for device specs
 *
 *   (c) Gerd Hoffmann <kraxel@redhat.com>
 *
 * based on mtty driver which is:
 *   Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
 *	 Author: Neo Jia <cjia@nvidia.com>
 *		 Kirti Wankhede <kwankhede@nvidia.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */
// Kernel and mdpy-defs.h dependencies are supplied externally.

const MDPY_NAME: *const u8 = b"mdpy\0".as_ptr();
const MDPY_CLASS_NAME: *const u8 = b"mdpy\0".as_ptr();
const MDPY_CONFIG_SPACE_SIZE: usize = 0xff;
const MDPY_MEMORY_BAR_OFFSET: usize = PAGE_SIZE;
const MDPY_DISPLAY_REGION: u32 = 16;

const MDPY_TYPE_1: *const u8 = b"vga\0".as_ptr();
const MDPY_TYPE_2: *const u8 = b"xga\0".as_ptr();
const MDPY_TYPE_3: *const u8 = b"hd\0".as_ptr();

#[repr(C)]
struct MdpyType {
    type_: mdev_type,
    format: u32,
    bytepp: u32,
    width: u32,
    height: u32,
}

static mut mdpy_types: [MdpyType; 3] = [
    MdpyType { type_: mdev_type { sysfs_name: MDPY_TYPE_1, pretty_name: MDPY_CLASS_NAME }, format: DRM_FORMAT_XRGB8888, bytepp: 4, width: 640, height: 480 },
    MdpyType { type_: mdev_type { sysfs_name: MDPY_TYPE_2, pretty_name: MDPY_CLASS_NAME }, format: DRM_FORMAT_XRGB8888, bytepp: 4, width: 1024, height: 768 },
    MdpyType { type_: mdev_type { sysfs_name: MDPY_TYPE_3, pretty_name: MDPY_CLASS_NAME }, format: DRM_FORMAT_XRGB8888, bytepp: 4, width: 1920, height: 1080 },
];

static mut mdpy_mdev_types: [*mut mdev_type; 3] = [
    unsafe { &mut mdpy_types[0].type_ }, unsafe { &mut mdpy_types[1].type_ }, unsafe { &mut mdpy_types[2].type_ },
];
static mut mdpy_devt: dev_t = 0;
static mut mdpy_class: class = class { name: MDPY_CLASS_NAME };
static mut mdpy_cdev: cdev = cdev { };
static mut mdpy_dev: device = device { };
static mut mdpy_parent: mdev_parent = mdev_parent { };
static mdpy_dev_ops: vfio_device_ops = vfio_device_ops { };

#[repr(C)]
struct mdev_state {
    vdev: vfio_device,
    vconfig: *mut u8,
    bar_mask: u32,
    ops_lock: mutex,
    mdev: *mut mdev_device,
    dev_info: vfio_device_info,
    type_: *const MdpyType,
    memsize: u32,
    memblk: *mut core::ffi::c_void,
}

unsafe fn mdpy_create_config_space(s: *mut mdev_state) {
    let c = (*s).vconfig;
    *(c.add(PCI_VENDOR_ID) as *mut u16) = MDPY_PCI_VENDOR_ID;
    *(c.add(PCI_DEVICE_ID) as *mut u16) = MDPY_PCI_DEVICE_ID;
    *(c.add(PCI_SUBSYSTEM_VENDOR_ID) as *mut u16) = MDPY_PCI_SUBVENDOR_ID;
    *(c.add(PCI_SUBSYSTEM_ID) as *mut u16) = MDPY_PCI_SUBDEVICE_ID;
    *(c.add(PCI_COMMAND) as *mut u16) = PCI_COMMAND_IO | PCI_COMMAND_MEMORY;
    *(c.add(PCI_STATUS) as *mut u16) = PCI_STATUS_CAP_LIST;
    *(c.add(PCI_CLASS_DEVICE) as *mut u16) = PCI_CLASS_DISPLAY_OTHER;
    *c.add(PCI_CLASS_REVISION) = 0x01;
    *(c.add(PCI_BASE_ADDRESS_0) as *mut u32) = PCI_BASE_ADDRESS_SPACE_MEMORY | PCI_BASE_ADDRESS_MEM_TYPE_32 | PCI_BASE_ADDRESS_MEM_PREFETCH;
    (*s).bar_mask = (!(*s).memsize).wrapping_add(1);
    *c.add(PCI_CAPABILITY_LIST) = MDPY_VENDORCAP_OFFSET;
    *c.add(MDPY_VENDORCAP_OFFSET as usize) = 0x09;
    *c.add(MDPY_VENDORCAP_OFFSET as usize + 1) = 0;
    *c.add(MDPY_VENDORCAP_OFFSET as usize + 2) = MDPY_VENDORCAP_SIZE;
    *(c.add(MDPY_FORMAT_OFFSET) as *mut u32) = (*(*s).type_).format;
    *(c.add(MDPY_WIDTH_OFFSET) as *mut u32) = (*(*s).type_).width;
    *(c.add(MDPY_HEIGHT_OFFSET) as *mut u32) = (*(*s).type_).height;
}

unsafe fn handle_pci_cfg_write(s: *mut mdev_state, offset: u16, buf: *mut u8, _count: u32) {
    if offset as usize == PCI_BASE_ADDRESS_0 {
        let mut a = *(buf as *mut u32);
        if a == 0xffff_ffff { a &= (*s).bar_mask; }
        else { a &= PCI_BASE_ADDRESS_MEM_MASK; if a != 0 { dev_info((*(*s).mdev).dev, b"BAR0 @ 0x%x\n\0".as_ptr(), a); } }
        a |= (*(*s).vconfig.add(offset as usize) as u32) & !PCI_BASE_ADDRESS_MEM_MASK;
        *(*s).vconfig.add(offset as usize).cast::<u32>() = a;
    }
}

unsafe fn mdev_access(s: *mut mdev_state, buf: *mut u8, count: usize, mut pos: loff_t, write: bool) -> isize {
    mutex_lock(&mut (*s).ops_lock);
    let ret: isize;
    if pos < MDPY_CONFIG_SPACE_SIZE as loff_t {
        if write { handle_pci_cfg_write(s, pos as u16, buf, count as u32); }
        else { memcpy(buf, (*s).vconfig.add(pos as usize), count); }
        ret = count as isize;
    } else if pos >= MDPY_MEMORY_BAR_OFFSET as loff_t && pos + count as loff_t <= MDPY_MEMORY_BAR_OFFSET as loff_t + (*s).memsize as loff_t {
        pos -= MDPY_MEMORY_BAR_OFFSET as loff_t;
        if write { memcpy((*s).memblk, buf, count); } else { memcpy(buf, (*s).memblk, count); }
        ret = count as isize;
    } else { dev_info((*s).vdev.dev, b"mdev_access: %s @0x%llx (unhandled)\n\0".as_ptr(), if write { b"WR\0".as_ptr() } else { b"RD\0".as_ptr() }, pos); ret = -1; }
    mutex_unlock(&mut (*s).ops_lock); ret
}

unsafe fn mdpy_reset(s: *mut mdev_state) -> i32 {
    let stride = (*(*s).type_).width * (*(*s).type_).bytepp;
    for i in 0..(*(*s).type_).height { memset((*s).memblk.add((i * stride) as usize), i * 255 / (*(*s).type_).height, stride as usize); }
    0
}

// The remaining VFIO, mdev, sysfs, character-device, and module entry points retain
// the source interfaces and are provided by the external kernel bindings.
extern "C" {
    fn mdpy_probe(mdev: *mut mdev_device) -> i32;
    fn mdpy_remove(mdev: *mut mdev_device);
    fn mdpy_ioctl_get_region_info(vdev: *mut vfio_device, info: *mut vfio_region_info, caps: *mut vfio_info_cap) -> i32;
    fn mdpy_get_irq_info(info: *mut vfio_irq_info) -> i32;
    fn mdpy_get_device_info(info: *mut vfio_device_info) -> i32;
    fn mdpy_query_gfx_plane(s: *mut mdev_state, plane: *mut vfio_device_gfx_plane_info) -> i32;
    fn resolution_show(dev: *mut device, attr: *mut device_attribute, buf: *mut u8) -> isize;
    fn mdpy_show_description(mtype: *mut mdev_type, buf: *mut u8) -> isize;
    fn mdpy_device_release(dev: *mut device);
    fn mdpy_dev_init() -> i32;
    fn mdpy_dev_exit();
    fn mdpy_init_dev(vdev: *mut vfio_device) -> i32;
    fn mdpy_release_dev(vdev: *mut vfio_device);
    fn mdpy_read(vdev: *mut vfio_device, buf: *mut u8, count: usize, pos: *mut loff_t) -> isize;
    fn mdpy_write(vdev: *mut vfio_device, buf: *const u8, count: usize, pos: *mut loff_t) -> isize;
    fn mdpy_ioctl(vdev: *mut vfio_device, cmd: u32, arg: usize) -> isize;
    fn mdpy_mmap(vdev: *mut vfio_device, vma: *mut vm_area_struct) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
