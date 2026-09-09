/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * VFIO API definition
 *
 * Copyright (C) 2012 Red Hat, Inc.  All rights reserved.
 *     Author: Alex Williamson <alex.williamson@redhat.com>
 */
/* C header dependencies: linux/iommu.h, linux/mm.h, linux/workqueue.h,
 * linux/poll.h, linux/cdev.h, uapi/linux/vfio.h, linux/iova_bitmap.h,
 * linux/uaccess.h. */

pub struct kvm;
pub struct iommufd_ctx;
pub struct iommufd_device;
pub struct iommufd_access;

#[repr(C)]
pub struct vfio_device_set {
    pub set_id: *mut core::ffi::c_void,
    pub lock: mutex,
    pub device_list: list_head,
    pub device_count: u32,
}

#[repr(C)]
pub struct vfio_device {
    pub dev: *mut device,
    pub ops: *const vfio_device_ops,
    pub mig_ops: *const vfio_migration_ops,
    pub log_ops: *const vfio_log_ops,
    /* CONFIG_VFIO_GROUP */
    pub group: *mut vfio_group,
    pub group_next: list_head,
    pub iommu_entry: list_head,
    pub dev_set: *mut vfio_device_set,
    pub dev_set_list: list_head,
    pub migration_flags: u32,
    pub precopy_info_v2: u8,
    pub kvm: *mut kvm,
    pub index: u32,
    pub device: device,
    /* CONFIG_VFIO_DEVICE_CDEV */
    pub cdev: cdev,
    pub refcount: refcount_t,
    pub open_count: u32,
    pub comp: completion,
    pub iommufd_access: *mut iommufd_access,
    pub put_kvm: Option<unsafe extern "C" fn(*mut kvm)>,
    pub inode: *mut inode,
    /* CONFIG_IOMMUFD */
    pub iommufd_device: *mut iommufd_device,
    pub pasids: ida,
    pub iommufd_attached: u8,
    pub cdev_opened: u8,
    pub noiommu: u8,
    pub debug_root: *mut dentry,
}

#[repr(C)]
pub struct vfio_device_ops {
    pub name: *mut i8,
    pub init: Option<unsafe extern "C" fn(*mut vfio_device) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut vfio_device)>,
    pub bind_iommufd: Option<unsafe extern "C" fn(*mut vfio_device, *mut iommufd_ctx, *mut u32) -> i32>,
    pub unbind_iommufd: Option<unsafe extern "C" fn(*mut vfio_device)>,
    pub attach_ioas: Option<unsafe extern "C" fn(*mut vfio_device, *mut u32) -> i32>,
    pub detach_ioas: Option<unsafe extern "C" fn(*mut vfio_device)>,
    pub pasid_attach_ioas: Option<unsafe extern "C" fn(*mut vfio_device, u32, *mut u32) -> i32>,
    pub pasid_detach_ioas: Option<unsafe extern "C" fn(*mut vfio_device, u32)>,
    pub open_device: Option<unsafe extern "C" fn(*mut vfio_device) -> i32>,
    pub close_device: Option<unsafe extern "C" fn(*mut vfio_device)>,
    pub read: Option<unsafe extern "C" fn(*mut vfio_device, *mut i8, usize, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut vfio_device, *const i8, usize, *mut loff_t) -> ssize_t>,
    pub ioctl: Option<unsafe extern "C" fn(*mut vfio_device, u32, usize) -> isize>,
    pub get_region_info_caps: Option<unsafe extern "C" fn(*mut vfio_device, *mut vfio_region_info, *mut vfio_info_cap) -> i32>,
    pub mmap: Option<unsafe extern "C" fn(*mut vfio_device, *mut vm_area_struct) -> i32>,
    pub request: Option<unsafe extern "C" fn(*mut vfio_device, u32)>,
    pub match_: Option<unsafe extern "C" fn(*mut vfio_device, *mut i8) -> i32>,
    pub match_token_uuid: Option<unsafe extern "C" fn(*mut vfio_device, *const uuid_t) -> i32>,
    pub dma_unmap: Option<unsafe extern "C" fn(*mut vfio_device, u64, u64)>,
    pub device_feature: Option<unsafe extern "C" fn(*mut vfio_device, u32, *mut core::ffi::c_void, usize) -> i32>,
}

/* CONFIG_IOMMUFD declarations; when disabled, these are NULL callbacks or
 * inline functions returning NULL/VFIO_PCI_DEVID_NOT_OWNED as in the C header. */
extern "C" {
    pub fn vfio_iommufd_device_ictx(vdev: *mut vfio_device) -> *mut iommufd_ctx;
    pub fn vfio_iommufd_get_dev_id(vdev: *mut vfio_device, ictx: *mut iommufd_ctx) -> i32;
    pub fn vfio_iommufd_physical_bind(vdev: *mut vfio_device, ictx: *mut iommufd_ctx, out_device_id: *mut u32) -> i32;
    pub fn vfio_iommufd_physical_unbind(vdev: *mut vfio_device);
    pub fn vfio_iommufd_physical_attach_ioas(vdev: *mut vfio_device, pt_id: *mut u32) -> i32;
    pub fn vfio_iommufd_physical_detach_ioas(vdev: *mut vfio_device);
    pub fn vfio_iommufd_physical_pasid_attach_ioas(vdev: *mut vfio_device, pasid: u32, pt_id: *mut u32) -> i32;
    pub fn vfio_iommufd_physical_pasid_detach_ioas(vdev: *mut vfio_device, pasid: u32);
    pub fn vfio_iommufd_emulated_bind(vdev: *mut vfio_device, ictx: *mut iommufd_ctx, out_device_id: *mut u32) -> i32;
    pub fn vfio_iommufd_emulated_unbind(vdev: *mut vfio_device);
    pub fn vfio_iommufd_emulated_attach_ioas(vdev: *mut vfio_device, pt_id: *mut u32) -> i32;
    pub fn vfio_iommufd_emulated_detach_ioas(vdev: *mut vfio_device);
}

pub unsafe fn vfio_device_cdev_opened(device: *const vfio_device) -> bool { (*device).cdev_opened != 0 }

pub const VFIO_DEVICE_FEATURE_GET: u32 = 1 << 0;
pub const VFIO_DEVICE_FEATURE_SET: u32 = 1 << 1;
pub const VFIO_DEVICE_FEATURE_PROBE: u32 = 1 << 2;

#[repr(C)]
pub struct vfio_migration_ops {
    pub migration_set_state: Option<unsafe extern "C" fn(*mut vfio_device, vfio_device_mig_state) -> *mut file>,
    pub migration_get_state: Option<unsafe extern "C" fn(*mut vfio_device, *mut vfio_device_mig_state) -> i32>,
    pub migration_get_data_size: Option<unsafe extern "C" fn(*mut vfio_device, *mut usize) -> i32>,
}

#[repr(C)]
pub struct vfio_log_ops {
    pub log_start: Option<unsafe extern "C" fn(*mut vfio_device, *mut rb_root_cached, u32, *mut u64) -> i32>,
    pub log_stop: Option<unsafe extern "C" fn(*mut vfio_device) -> i32>,
    pub log_read_and_clear: Option<unsafe extern "C" fn(*mut vfio_device, usize, usize, *mut iova_bitmap) -> i32>,
}

pub unsafe fn vfio_check_feature(flags: u32, argsz: usize, supported_ops: u32, minsz: usize) -> i32 {
    if (flags & (VFIO_DEVICE_FEATURE_GET | VFIO_DEVICE_FEATURE_SET)) & !supported_ops != 0 { return -EINVAL; }
    if flags & VFIO_DEVICE_FEATURE_PROBE != 0 { return 0; }
    if flags & (VFIO_DEVICE_FEATURE_GET | VFIO_DEVICE_FEATURE_SET) == 0 { return -EINVAL; }
    if argsz < minsz { return -EINVAL; }
    1
}

pub unsafe fn vfio_check_precopy_ioctl(vdev: *mut vfio_device, cmd: u32, arg: usize, info: *mut vfio_precopy_info) -> i32 {
    if cmd != VFIO_MIG_GET_PRECOPY_INFO { return -ENOTTY; }
    let minsz = core::mem::offset_of!(vfio_precopy_info, dirty_bytes) + core::mem::size_of::<u64>();
    if copy_from_user(info as *mut core::ffi::c_void, arg as *const core::ffi::c_void, minsz) != 0 { return -EFAULT; }
    if (*info).argsz < minsz { return -EINVAL; }
    if (*vdev).precopy_info_v2 != 0 { (*info).flags = 0; }
    0
}

extern "C" {
    pub fn _vfio_alloc_device(size: usize, dev: *mut device, ops: *const vfio_device_ops) -> *mut vfio_device;
    pub fn vfio_register_group_dev(device: *mut vfio_device) -> i32;
    pub fn vfio_register_emulated_iommu_dev(device: *mut vfio_device) -> i32;
    pub fn vfio_unregister_group_dev(device: *mut vfio_device);
    pub fn vfio_device_try_get_registration(device: *mut vfio_device) -> bool;
    pub fn vfio_device_put_registration(device: *mut vfio_device);
    pub fn vfio_assign_device_set(device: *mut vfio_device, set_id: *mut core::ffi::c_void) -> i32;
    pub fn vfio_device_set_open_count(dev_set: *mut vfio_device_set) -> u32;
    pub fn vfio_find_device_in_devset(dev_set: *mut vfio_device_set, dev: *mut device) -> *mut vfio_device;
    pub fn vfio_mig_get_next_state(device: *mut vfio_device, cur_fsm: vfio_device_mig_state, new_fsm: vfio_device_mig_state, next_fsm: *mut vfio_device_mig_state) -> i32;
    pub fn vfio_combine_iova_ranges(root: *mut rb_root_cached, cur_nodes: u32, req_nodes: u32);
    pub fn vfio_file_iommu_group(file: *mut file) -> *mut iommu_group;
    pub fn vfio_file_is_group(file: *mut file) -> bool;
    pub fn vfio_file_has_dev(file: *mut file, device: *mut vfio_device) -> bool;
    pub fn vfio_file_is_valid(file: *mut file) -> bool;
    pub fn vfio_file_enforced_coherent(file: *mut file) -> bool;
    pub fn vfio_file_set_kvm(file: *mut file, kvm: *mut kvm);
    pub fn vfio_pin_pages(device: *mut vfio_device, iova: dma_addr_t, npage: i32, prot: i32, pages: *mut *mut page) -> i32;
    pub fn vfio_unpin_pages(device: *mut vfio_device, iova: dma_addr_t, npage: i32);
    pub fn vfio_dma_rw(device: *mut vfio_device, iova: dma_addr_t, data: *mut core::ffi::c_void, len: usize, write: bool) -> i32;
    pub fn put_device(dev: *mut device);
}

pub unsafe fn vfio_put_device(device: *mut vfio_device) { put_device(&mut (*device).device); }

pub const VFIO_PIN_PAGES_MAX_ENTRIES: usize = PAGE_SIZE / core::mem::size_of::<usize>();

#[repr(C)]
pub struct vfio_info_cap { pub buf: *mut vfio_info_cap_header, pub size: usize }
extern "C" {
    pub fn vfio_info_cap_add(caps: *mut vfio_info_cap, size: usize, id: u16, version: u16) -> *mut vfio_info_cap_header;
    pub fn vfio_info_cap_shift(caps: *mut vfio_info_cap, offset: usize);
    pub fn vfio_info_add_capability(caps: *mut vfio_info_cap, cap: *mut vfio_info_cap_header, size: usize) -> i32;
    pub fn vfio_set_irqs_validate_and_prepare(hdr: *mut vfio_irq_set, num_irqs: i32, max_irq_type: i32, data_size: *mut usize) -> i32;
}

#[repr(C)]
pub struct virqfd {
    pub opaque: *mut core::ffi::c_void,
    pub eventfd: *mut eventfd_ctx,
    pub handler: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> i32>,
    pub thread: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void)>,
    pub data: *mut core::ffi::c_void,
    pub inject: work_struct,
    pub wait: wait_queue_entry_t,
    pub pt: poll_table,
    pub shutdown: work_struct,
    pub flush_inject: work_struct,
    pub pvirqfd: *mut *mut virqfd,
}

extern "C" {
    pub fn vfio_virqfd_enable(opaque: *mut core::ffi::c_void, handler: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> i32>, thread: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void)>, data: *mut core::ffi::c_void, pvirqfd: *mut *mut virqfd, fd: i32) -> i32;
    pub fn vfio_virqfd_disable(pvirqfd: *mut *mut virqfd);
    pub fn vfio_virqfd_flush_thread(pvirqfd: *mut *mut virqfd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
