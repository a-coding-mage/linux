/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of linux/iommufd.h.
 * C includes and configuration supplied by other headers are external dependencies.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iommufd_object_type {
    IOMMUFD_OBJ_NONE,
    IOMMUFD_OBJ_ANY = IOMMUFD_OBJ_NONE as isize,
    IOMMUFD_OBJ_DEVICE,
    IOMMUFD_OBJ_HWPT_PAGING,
    IOMMUFD_OBJ_HWPT_NESTED,
    IOMMUFD_OBJ_IOAS,
    IOMMUFD_OBJ_ACCESS,
    IOMMUFD_OBJ_FAULT,
    IOMMUFD_OBJ_VIOMMU,
    IOMMUFD_OBJ_VDEVICE,
    IOMMUFD_OBJ_VEVENTQ,
    IOMMUFD_OBJ_HW_QUEUE,
    // CONFIG_IOMMUFD_TEST: IOMMUFD_OBJ_SELFTEST,
    IOMMUFD_OBJ_MAX,
}

#[repr(C)]
pub struct iommufd_object {
    /* Destroy will sleep and wait for wait_cnt to go to zero. */
    pub wait_cnt: refcount_t,
    pub users: refcount_t,
    pub type_: iommufd_object_type,
    pub id: ::core::ffi::c_uint,
}

extern "C" {
    pub fn iommufd_device_bind(ictx: *mut iommufd_ctx, dev: *mut device, id: *mut u32) -> *mut iommufd_device;
    pub fn iommufd_device_unbind(idev: *mut iommufd_device);
    pub fn iommufd_device_attach(idev: *mut iommufd_device, pasid: ioasid_t, pt_id: *mut u32) -> ::core::ffi::c_int;
    pub fn iommufd_device_replace(idev: *mut iommufd_device, pasid: ioasid_t, pt_id: *mut u32) -> ::core::ffi::c_int;
    pub fn iommufd_device_detach(idev: *mut iommufd_device, pasid: ioasid_t);
    pub fn iommufd_device_to_ictx(idev: *mut iommufd_device) -> *mut iommufd_ctx;
    pub fn iommufd_device_to_id(idev: *mut iommufd_device) -> u32;
}

#[repr(C)]
pub struct iommufd_access_ops {
    pub needs_pin_pages: u8,
    pub unmap: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, c_ulong, c_ulong)>,
}

pub const IOMMUFD_ACCESS_RW_READ: u32 = 0;
pub const IOMMUFD_ACCESS_RW_WRITE: u32 = 1 << 0;
pub const IOMMUFD_ACCESS_RW_KTHREAD: u32 = 1 << 1;
pub const __IOMMUFD_ACCESS_RW_SLOW_PATH: u32 = 1 << 2;

extern "C" {
    pub fn iommufd_access_create(ictx: *mut iommufd_ctx, ops: *const iommufd_access_ops, data: *mut ::core::ffi::c_void, id: *mut u32) -> *mut iommufd_access;
    pub fn iommufd_access_destroy(access: *mut iommufd_access);
    pub fn iommufd_access_attach(access: *mut iommufd_access, ioas_id: u32) -> ::core::ffi::c_int;
    pub fn iommufd_access_replace(access: *mut iommufd_access, ioas_id: u32) -> ::core::ffi::c_int;
    pub fn iommufd_access_detach(access: *mut iommufd_access);
    pub fn iommufd_ctx_get(ictx: *mut iommufd_ctx);
}

#[repr(C)]
pub struct iommufd_viommu {
    pub obj: iommufd_object,
    pub ictx: *mut iommufd_ctx,
    pub iommu_dev: *mut iommu_device,
    pub hwpt: *mut iommufd_hwpt_paging,
    pub ops: *const iommufd_viommu_ops,
    pub vdevs: xarray,
    pub veventqs: list_head,
    pub veventqs_rwsem: rw_semaphore,
    pub type_: iommu_viommu_type,
}

#[repr(C)]
pub struct iommufd_vdevice {
    pub obj: iommufd_object,
    pub viommu: *mut iommufd_viommu,
    pub idev: *mut iommufd_device,
    pub virt_id: u64,
    pub destroy: Option<unsafe extern "C" fn(*mut iommufd_vdevice)>,
}

#[repr(C)]
pub struct iommufd_hw_queue {
    pub obj: iommufd_object,
    pub viommu: *mut iommufd_viommu,
    pub access: *mut iommufd_access,
    pub base_addr: u64,
    pub length: usize,
    pub type_: iommu_hw_queue_type,
    pub destroy: Option<unsafe extern "C" fn(*mut iommufd_hw_queue)>,
}

#[repr(C)]
pub struct iommufd_viommu_ops {
    pub destroy: Option<unsafe extern "C" fn(*mut iommufd_viommu)>,
    pub alloc_domain_nested: Option<unsafe extern "C" fn(*mut iommufd_viommu, u32, *const iommu_user_data) -> *mut iommu_domain>,
    pub cache_invalidate: Option<unsafe extern "C" fn(*mut iommufd_viommu, *mut iommu_user_data_array) -> ::core::ffi::c_int>,
    pub vdevice_size: usize,
    pub vdevice_init: Option<unsafe extern "C" fn(*mut iommufd_vdevice) -> ::core::ffi::c_int>,
    pub get_hw_queue_size: Option<unsafe extern "C" fn(*mut iommufd_viommu, iommu_hw_queue_type) -> usize>,
    pub hw_queue_init_phys: Option<unsafe extern "C" fn(*mut iommufd_hw_queue, u32, phys_addr_t) -> ::core::ffi::c_int>,
}

/* Configuration-dependent declarations are preserved as extern interfaces. */
extern "C" {
    pub fn iommufd_ctx_from_file(file: *mut file) -> *mut iommufd_ctx;
    pub fn iommufd_ctx_from_fd(fd: ::core::ffi::c_int) -> *mut iommufd_ctx;
    pub fn iommufd_ctx_put(ictx: *mut iommufd_ctx);
    pub fn iommufd_ctx_has_group(ictx: *mut iommufd_ctx, group: *mut iommu_group) -> bool;
    pub fn iommufd_access_pin_pages(access: *mut iommufd_access, iova: c_ulong, length: c_ulong, out_pages: *mut *mut page, flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn iommufd_access_unpin_pages(access: *mut iommufd_access, iova: c_ulong, length: c_ulong);
    pub fn iommufd_access_rw(access: *mut iommufd_access, iova: c_ulong, data: *mut ::core::ffi::c_void, len: usize, flags: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn iommufd_vfio_compat_ioas_get_id(ictx: *mut iommufd_ctx, out_ioas_id: *mut u32) -> ::core::ffi::c_int;
    pub fn iommufd_vfio_compat_ioas_create(ictx: *mut iommufd_ctx) -> ::core::ffi::c_int;
    pub fn iommufd_vfio_compat_set_no_iommu(ictx: *mut iommufd_ctx) -> ::core::ffi::c_int;
    pub fn _iommufd_object_depend(obj_dependent: *mut iommufd_object, obj_depended: *mut iommufd_object) -> ::core::ffi::c_int;
    pub fn _iommufd_object_undepend(obj_dependent: *mut iommufd_object, obj_depended: *mut iommufd_object);
    pub fn _iommufd_alloc_mmap(ictx: *mut iommufd_ctx, owner: *mut iommufd_object, mmio_addr: phys_addr_t, length: usize, offset: *mut c_ulong) -> ::core::ffi::c_int;
    pub fn _iommufd_destroy_mmap(ictx: *mut iommufd_ctx, owner: *mut iommufd_object, offset: c_ulong);
    pub fn iommufd_vdevice_to_device(vdev: *mut iommufd_vdevice) -> *mut device;
    pub fn iommufd_viommu_find_dev(viommu: *mut iommufd_viommu, vdev_id: c_ulong) -> *mut device;
    pub fn iommufd_viommu_get_vdev_id(viommu: *mut iommufd_viommu, dev: *mut device, vdev_id: *mut c_ulong) -> ::core::ffi::c_int;
    pub fn iommufd_viommu_report_event(viommu: *mut iommufd_viommu, type_: iommu_veventq_type, event_data: *mut ::core::ffi::c_void, data_len: usize) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn iommufd_viommu_alloc_mmap(viommu: *mut iommufd_viommu, mmio_addr: phys_addr_t, length: usize, offset: *mut c_ulong) -> ::core::ffi::c_int {
    _iommufd_alloc_mmap((*viommu).ictx, &mut (*viommu).obj, mmio_addr, length, offset)
}

#[inline]
pub unsafe fn iommufd_viommu_destroy_mmap(viommu: *mut iommufd_viommu, offset: c_ulong) {
    _iommufd_destroy_mmap((*viommu).ictx, &mut (*viommu).obj, offset)
}

/* C's compile-time layout/type-checking helpers; callers supply equivalent checks. */
#[macro_export]
macro_rules! VIOMMU_STRUCT_SIZE { ($drv_struct:ty, $member:ident) => { ::core::mem::size_of::<$drv_struct>() }; }
#[macro_export]
macro_rules! VDEVICE_STRUCT_SIZE { ($drv_struct:ty, $member:ident) => { ::core::mem::size_of::<$drv_struct>() }; }
#[macro_export]
macro_rules! HW_QUEUE_STRUCT_SIZE { ($drv_struct:ty, $member:ident) => { ::core::mem::size_of::<$drv_struct>() }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
