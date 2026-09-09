// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025 Ant Group
 * Author: Tiwei Bie <tiwei.btw@antgroup.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Constants and declarations are supplied by the corresponding system and UML headers.
extern "C" {
    static uml_reserved: c_ulong;
    static uml_physmem: c_ulong;
    static physmem_size: c_ulong;
    fn uml_kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, size: usize) -> isize;
    fn eventfd(initval: c_uint, flags: c_int) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: usize, offset: i64) -> isize;
    fn pwrite(fd: c_int, buf: *const c_void, count: usize, offset: i64) -> isize;
}

#[repr(C)]
pub struct vfio_region {
    pub size: u64,
    pub offset: u64,
}

#[repr(C)]
pub struct uml_vfio_user_device {
    pub device: c_int,
    pub num_regions: c_uint,
    pub region: *mut vfio_region,
    pub irq_count: c_uint,
    pub irqfd: *mut c_int,
}

#[repr(C)]
struct vfio_iommu_type1_dma_map {
    argsz: u32,
    flags: u32,
    vaddr: u64,
    iova: u64,
    size: u64,
}

#[repr(C)]
struct vfio_device_info { argsz: u32, flags: u32, num_regions: u32, num_irqs: u32 }
#[repr(C)]
struct vfio_region_info { argsz: u32, flags: u32, index: u32, cap_offset: u32, size: u64, offset: u64 }
#[repr(C)]
struct vfio_irq_info { argsz: u32, flags: u32, index: u32, count: u32 }

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const UM_GFP_KERNEL: c_uint = 0;
const VFIO_API_VERSION: c_int = 0;
const VFIO_TYPE1_IOMMU: c_int = 1;
const VFIO_PCI_MSIX_IRQ_INDEX: u32 = 2;
const VFIO_PCI_CONFIG_REGION_INDEX: u32 = 7;
const VFIO_DMA_MAP_FLAG_READ: u32 = 1;
const VFIO_DMA_MAP_FLAG_WRITE: u32 = 2;
const VFIO_IRQ_SET_DATA_EVENTFD: u32 = 1 << 2;
const VFIO_IRQ_SET_ACTION_TRIGGER: u32 = 1 << 3;
const O_RDWR: c_int = 2;
const EFD_NONBLOCK: c_int = 0x800;
const EFD_CLOEXEC: c_int = 0x80000;

#[inline]
unsafe fn neg_errno() -> c_int { -(*__errno_location()) }
extern "C" { fn __errno_location() -> *mut c_int; }

pub unsafe fn uml_vfio_user_open_container() -> c_int {
    let fd = open(b"/dev/vfio/vfio\0".as_ptr() as *const c_char, O_RDWR);
    if fd < 0 { return neg_errno(); }
    let mut r = ioctl(fd, 0, 0);
    if r != VFIO_API_VERSION { r = if r < 0 { neg_errno() } else { -EINVAL }; close(fd); return r; }
    r = ioctl(fd, 0, VFIO_TYPE1_IOMMU);
    if r <= 0 { r = if r < 0 { neg_errno() } else { -EINVAL }; close(fd); return r; }
    fd
}

pub unsafe fn uml_vfio_user_setup_iommu(container: c_int) -> c_int {
    let reserved = uml_reserved - uml_physmem;
    let mut dma_map = vfio_iommu_type1_dma_map { argsz: core::mem::size_of::<vfio_iommu_type1_dma_map>() as u32, flags: VFIO_DMA_MAP_FLAG_READ | VFIO_DMA_MAP_FLAG_WRITE, vaddr: uml_reserved as u64, iova: reserved as u64, size: (physmem_size - reserved) as u64 };
    if ioctl(container, 0, VFIO_TYPE1_IOMMU) < 0 { return neg_errno(); }
    if ioctl(container, 0, &mut dma_map) < 0 { return neg_errno(); }
    0
}

pub unsafe fn uml_vfio_user_get_group_id(device: *const c_char) -> c_int {
    let path = uml_kmalloc(4096, UM_GFP_KERNEL) as *mut c_char;
    if path.is_null() { return -ENOMEM; }
    // Equivalent to sprintf(path, "/sys/bus/pci/devices/%s/iommu_group", device).
    let prefix = b"/sys/bus/pci/devices/\0";
    core::ptr::copy_nonoverlapping(prefix.as_ptr() as *const c_char, path, prefix.len());
    let mut p = path.add(prefix.len() - 1);
    let mut d = device;
    while *d != 0 { *p = *d; p = p.add(1); d = d.add(1); }
    let suffix = b"/iommu_group\0";
    core::ptr::copy_nonoverlapping(suffix.as_ptr() as *const c_char, p, suffix.len());
    let buf = uml_kmalloc(4097, UM_GFP_KERNEL) as *mut c_char;
    if buf.is_null() { kfree(path as *mut c_void); return -ENOMEM; }
    let mut r = readlink(path, buf, 4096);
    if r < 0 { r = neg_errno() as isize; } else { *buf.add(r as usize) = 0; }
    if r < 0 { kfree(buf as *mut c_void); kfree(path as *mut c_void); return r as c_int; }
    let mut q = buf.add(r as usize);
    while q > buf && *q.sub(1) != b'/' as c_char { q = q.sub(1); }
    let name = q;
    let mut value: c_int = 0;
    let mut end = name;
    while *end != 0 { if *end < b'0' as c_char || *end > b'9' as c_char { kfree(buf as *mut c_void); kfree(path as *mut c_void); return -EINVAL; } value = value.wrapping_mul(10).wrapping_add((*end - b'0' as c_char) as c_int); end = end.add(1); }
    if end == name { value = -EINVAL; }
    kfree(buf as *mut c_void); kfree(path as *mut c_void); value
}

pub unsafe fn uml_vfio_user_open_group(group_id: c_int) -> c_int {
    let path = uml_kmalloc(4096, UM_GFP_KERNEL) as *mut c_char;
    if path.is_null() { return -ENOMEM; }
    // Equivalent to sprintf(path, "/dev/vfio/%d", group_id).
    let s = alloc::format!("/dev/vfio/{}\0", group_id);
    core::ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, path, s.len());
    let fd = open(path, O_RDWR); kfree(path as *mut c_void); if fd < 0 { neg_errno() } else { fd }
}

pub unsafe fn uml_vfio_user_set_container(container: c_int, group: c_int) -> c_int { if ioctl(group, 0, &container) < 0 { neg_errno() } else { 0 } }
pub unsafe fn uml_vfio_user_unset_container(container: c_int, group: c_int) -> c_int { if ioctl(group, 0, &container) < 0 { neg_errno() } else { 0 } }

#[repr(C)] struct vfio_irq_set { argsz: u32, flags: u32, index: u32, start: u32, count: u32, data: [u8; 0] }
unsafe fn vfio_set_irqs(device: c_int, start: c_int, count: c_uint, irqfd: *mut c_int) -> c_int {
    let argsz = core::mem::size_of::<vfio_irq_set>() + core::mem::size_of::<c_int>() * count as usize;
    let p = uml_kmalloc(argsz, UM_GFP_KERNEL) as *mut vfio_irq_set; if p.is_null() { return -ENOMEM; }
    (*p).argsz = argsz as u32; (*p).flags = VFIO_IRQ_SET_DATA_EVENTFD | VFIO_IRQ_SET_ACTION_TRIGGER; (*p).index = VFIO_PCI_MSIX_IRQ_INDEX; (*p).start = start as u32; (*p).count = count;
    core::ptr::copy_nonoverlapping(irqfd as *const u8, (*p).data.as_mut_ptr(), 4 * count as usize);
    let r = if ioctl(device, 0, p) < 0 { neg_errno() } else { 0 }; kfree(p as *mut c_void); r
}

pub unsafe fn uml_vfio_user_setup_device(dev: *mut uml_vfio_user_device, group: c_int, device: *const c_char) -> c_int {
    let mut info = vfio_device_info { argsz: core::mem::size_of::<vfio_device_info>() as u32, flags: 0, num_regions: 0, num_irqs: 0 };
    (*dev).device = ioctl(group, 0, device); if (*dev).device < 0 { return neg_errno(); }
    if ioctl((*dev).device, 0, &mut info) < 0 { let r = neg_errno(); close((*dev).device); return r; }
    (*dev).num_regions = info.num_regions.min(VFIO_PCI_CONFIG_REGION_INDEX + 1);
    (*dev).region = uml_kmalloc(core::mem::size_of::<vfio_region>() * (*dev).num_regions as usize, UM_GFP_KERNEL) as *mut vfio_region;
    if (*dev).region.is_null() { close((*dev).device); return -ENOMEM; }
    for i in 0..(*dev).num_regions { let mut ri = vfio_region_info { argsz: core::mem::size_of::<vfio_region_info>() as u32, flags: 0, index: i, cap_offset: 0, size: 0, offset: 0 }; if ioctl((*dev).device, 0, &mut ri) < 0 { let r = neg_errno(); kfree((*dev).region as *mut c_void); close((*dev).device); return r; } (*dev).region.add(i as usize).write(vfio_region { size: ri.size, offset: ri.offset }); }
    let mut irq = vfio_irq_info { argsz: core::mem::size_of::<vfio_irq_info>() as u32, flags: 0, index: VFIO_PCI_MSIX_IRQ_INDEX, count: 0 }; if ioctl((*dev).device, 0, &mut irq) < 0 { let r = neg_errno(); kfree((*dev).region as *mut c_void); close((*dev).device); return r; }
    (*dev).irq_count = irq.count; (*dev).irqfd = uml_kmalloc(4 * irq.count as usize, UM_GFP_KERNEL) as *mut c_int; if (*dev).irqfd.is_null() { kfree((*dev).region as *mut c_void); close((*dev).device); return -ENOMEM; }
    for i in 0..irq.count { *(*dev).irqfd.add(i as usize) = -1; }
    let r = vfio_set_irqs((*dev).device, 0, (*dev).irq_count, (*dev).irqfd); if r != 0 { kfree((*dev).irqfd as *mut c_void); kfree((*dev).region as *mut c_void); close((*dev).device); return r; } 0
}

pub unsafe fn uml_vfio_user_teardown_device(dev: *mut uml_vfio_user_device) { kfree((*dev).irqfd as *mut c_void); kfree((*dev).region as *mut c_void); close((*dev).device); }
pub unsafe fn uml_vfio_user_activate_irq(dev: *mut uml_vfio_user_device, index: c_int) -> c_int { let fd = eventfd(0, EFD_NONBLOCK | EFD_CLOEXEC); if fd < 0 { return neg_errno(); } *(*dev).irqfd.add(index as usize) = fd; fd }
pub unsafe fn uml_vfio_user_deactivate_irq(dev: *mut uml_vfio_user_device, index: c_int) { close(*(*dev).irqfd.add(index as usize)); *(*dev).irqfd.add(index as usize) = -1; }
pub unsafe fn uml_vfio_user_update_irqs(dev: *mut uml_vfio_user_device) -> c_int { vfio_set_irqs((*dev).device, 0, (*dev).irq_count, (*dev).irqfd) }

unsafe fn vfio_region_read(dev: *mut uml_vfio_user_device, index: c_uint, offset: u64, buf: *mut c_void, size: u64) -> c_int { if index >= (*dev).num_regions || offset + size > (*(*dev).region.add(index as usize)).size { return -EINVAL; } if pread((*dev).device, buf, size as usize, ((*(*dev).region.add(index as usize)).offset + offset) as i64) < 0 { return neg_errno(); } 0 }
unsafe fn vfio_region_write(dev: *mut uml_vfio_user_device, index: c_uint, offset: u64, buf: *const c_void, size: u64) -> c_int { if index >= (*dev).num_regions || offset + size > (*(*dev).region.add(index as usize)).size { return -EINVAL; } if pwrite((*dev).device, buf, size as usize, ((*(*dev).region.add(index as usize)).offset + offset) as i64) < 0 { return neg_errno(); } 0 }
pub unsafe fn uml_vfio_user_cfgspace_read(dev: *mut uml_vfio_user_device, offset: c_uint, buf: *mut c_void, size: c_int) -> c_int { vfio_region_read(dev, VFIO_PCI_CONFIG_REGION_INDEX, offset as u64, buf, size as u64) }
pub unsafe fn uml_vfio_user_cfgspace_write(dev: *mut uml_vfio_user_device, offset: c_uint, buf: *const c_void, size: c_int) -> c_int { vfio_region_write(dev, VFIO_PCI_CONFIG_REGION_INDEX, offset as u64, buf, size as u64) }
pub unsafe fn uml_vfio_user_bar_read(dev: *mut uml_vfio_user_device, bar: c_int, offset: c_uint, buf: *mut c_void, size: c_int) -> c_int { vfio_region_read(dev, bar as c_uint, offset as u64, buf, size as u64) }
pub unsafe fn uml_vfio_user_bar_write(dev: *mut uml_vfio_user_device, bar: c_int, offset: c_uint, buf: *const c_void, size: c_int) -> c_int { vfio_region_write(dev, bar as c_uint, offset as u64, buf, size as u64) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
