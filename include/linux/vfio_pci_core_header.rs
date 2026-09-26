/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of vfio_pci_core.h. */

// Kernel includes are dependencies supplied by other translated files.

pub const VFIO_PCI_OFFSET_SHIFT: u32 = 40;
#[inline]
pub const fn vfio_pci_offset_to_index(off: u64) -> u64 { off >> VFIO_PCI_OFFSET_SHIFT }
#[inline]
pub const fn vfio_pci_index_to_offset(index: u64) -> u64 { index << VFIO_PCI_OFFSET_SHIFT }
pub const VFIO_PCI_OFFSET_MASK: u64 = (1u64 << VFIO_PCI_OFFSET_SHIFT) - 1;

pub struct vfio_pci_core_device;
pub struct vfio_pci_region;
pub struct p2pdma_provider;
pub struct dma_buf_attachment;

#[repr(C)]
pub struct vfio_pci_eventfd {
    pub ctx: *mut eventfd_ctx,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct vfio_pci_regops {
    pub rw: Option<unsafe extern "C" fn(*mut vfio_pci_core_device, *mut u8, usize, *mut loff_t, bool) -> ssize_t>,
    pub release: Option<unsafe extern "C" fn(*mut vfio_pci_core_device, *mut vfio_pci_region)>,
    pub mmap: Option<unsafe extern "C" fn(*mut vfio_pci_core_device, *mut vfio_pci_region, *mut vm_area_struct) -> i32>,
    pub add_capability: Option<unsafe extern "C" fn(*mut vfio_pci_core_device, *mut vfio_pci_region, *mut vfio_info_cap) -> i32>,
}

#[repr(C)]
pub struct vfio_pci_region {
    pub type_: u32,
    pub subtype: u32,
    pub ops: *const vfio_pci_regops,
    pub data: *mut core::ffi::c_void,
    pub size: usize,
    pub flags: u32,
}

#[repr(C)]
pub struct vfio_pci_device_ops {
    pub get_dmabuf_phys: Option<unsafe extern "C" fn(*mut vfio_pci_core_device, *mut *mut p2pdma_provider, u32, *mut phys_vec, *mut vfio_region_dma_range, usize) -> i32>,
}

// CONFIG_VFIO_PCI_DMABUF selects the external implementations; these declarations
// preserve the disabled-build inline return values from the C header.
extern "C" {
    pub fn vfio_pci_core_fill_phys_vec(_: *mut phys_vec, _: *mut vfio_region_dma_range, _: usize, _: phys_addr_t, _: phys_addr_t) -> i32;
    pub fn vfio_pci_core_get_dmabuf_phys(_: *mut vfio_pci_core_device, _: *mut *mut p2pdma_provider, _: u32, _: *mut phys_vec, _: *mut vfio_region_dma_range, _: usize) -> i32;
}

#[repr(C)]
pub struct vfio_pci_core_device {
    pub vdev: vfio_device,
    pub pdev: *mut pci_dev,
    pub pci_ops: *const vfio_pci_device_ops,
    pub barmap: [*mut core::ffi::c_void; PCI_STD_NUM_BARS],
    pub bar_mmap_supported: [bool; PCI_STD_NUM_BARS],
    pub virq_disabled: bool,
    pub bardirty: bool,
    pub pci_config_map: *mut u8,
    pub vconfig: *mut u8,
    pub msi_perm: *mut perm_bits,
    pub irqlock: spinlock_t,
    pub igate: mutex,
    pub ctx: xarray,
    pub irq_type: i32,
    pub num_regions: i32,
    pub region: *mut vfio_pci_region,
    pub msi_qmax: u8,
    pub msix_bar: u8,
    pub msix_size: u16,
    pub msix_offset: u32,
    pub rbar: [u32; 7],
    pub has_dyn_msix: bool,
    pub pci_2_3: bool,
    pub reset_works: bool,
    pub extended_caps: bool,
    pub has_vga: bool,
    pub nointx: bool,
    pub needs_pm_restore: bool,
    pub disable_idle_d3: bool,
    pub nointxmask: bool,
    pub disable_vga: bool,
    pub needs_reset: bool,
    pub pm_intx_masked: bool,
    pub pm_runtime_engaged: bool,
    pub sriov_active: bool,
    pub pci_saved_state: *mut pci_saved_state,
    pub pm_save: *mut pci_saved_state,
    pub ioeventfds_nr: i32,
    pub err_trigger: *mut vfio_pci_eventfd,
    pub req_trigger: *mut vfio_pci_eventfd,
    pub pm_wake_eventfd_ctx: *mut eventfd_ctx,
    pub dummy_resources_list: list_head,
    pub ioeventfds_lock: mutex,
    pub ioeventfds_list: list_head,
    pub vf_token: *mut vfio_pci_vf_token,
    pub sriov_pfs_item: list_head,
    pub sriov_pf_core_dev: *mut vfio_pci_core_device,
    pub nb: notifier_block,
    pub memory_lock: rw_semaphore,
    pub dmabufs: list_head,
}

#[repr(C)]
pub enum vfio_pci_io_width { VFIO_PCI_IO_WIDTH_1 = 1, VFIO_PCI_IO_WIDTH_2 = 2, VFIO_PCI_IO_WIDTH_4 = 4, VFIO_PCI_IO_WIDTH_8 = 8 }

extern "C" {
    pub fn vfio_pci_core_register_dev_region(_: *mut vfio_pci_core_device, _: u32, _: u32, _: *const vfio_pci_regops, _: usize, _: u32, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_close_device(_: *mut vfio_device);
    pub fn vfio_pci_core_init_dev(_: *mut vfio_device) -> i32;
    pub fn vfio_pci_core_release_dev(_: *mut vfio_device);
    pub fn vfio_pci_core_register_device(_: *mut vfio_pci_core_device) -> i32;
    pub fn vfio_pci_core_unregister_device(_: *mut vfio_pci_core_device);
    pub static vfio_pci_core_err_handlers: pci_error_handlers;
    pub fn vfio_pci_core_sriov_configure(_: *mut vfio_pci_core_device, _: i32) -> i32;
    pub fn vfio_pci_core_ioctl(_: *mut vfio_device, _: u32, _: usize) -> isize;
    pub fn vfio_pci_core_ioctl_feature(_: *mut vfio_device, _: u32, _: *mut core::ffi::c_void, _: usize) -> i32;
    pub fn vfio_pci_ioctl_get_region_info(_: *mut vfio_device, _: *mut vfio_region_info, _: *mut vfio_info_cap) -> i32;
    pub fn vfio_pci_core_read(_: *mut vfio_device, _: *mut u8, _: usize, _: *mut loff_t) -> ssize_t;
    pub fn vfio_pci_core_write(_: *mut vfio_device, _: *const u8, _: usize, _: *mut loff_t) -> ssize_t;
    pub fn vfio_pci_vmf_insert_pfn(_: *mut vfio_pci_core_device, _: *mut vm_fault, _: usize, _: u32) -> vm_fault_t;
    pub fn vfio_pci_core_mmap(_: *mut vfio_device, _: *mut vm_area_struct) -> i32;
    pub fn vfio_pci_core_request(_: *mut vfio_device, _: u32);
    pub fn vfio_pci_core_match(_: *mut vfio_device, _: *mut u8) -> i32;
    pub fn vfio_pci_core_match_token_uuid(_: *mut vfio_device, _: *const uuid_t) -> i32;
    pub fn vfio_pci_core_enable(_: *mut vfio_pci_core_device) -> i32;
    pub fn vfio_pci_core_disable(_: *mut vfio_pci_core_device);
    pub fn vfio_pci_core_finish_enable(_: *mut vfio_pci_core_device);
    pub fn vfio_pci_core_aer_err_detected(_: *mut pci_dev, _: pci_channel_state_t) -> pci_ers_result_t;
    pub fn vfio_pci_core_do_io_rw(_: *mut vfio_pci_core_device, _: bool, _: *mut core::ffi::c_void, _: *mut u8, _: loff_t, _: usize, _: usize, _: usize, _: bool, _: vfio_pci_io_width) -> ssize_t;
    pub fn __vfio_pci_memory_enabled(_: *mut vfio_pci_core_device) -> bool;
    pub fn vfio_pci_core_range_intersect_range(_: loff_t, _: usize, _: loff_t, _: usize, _: *mut loff_t, _: *mut usize, _: *mut usize) -> bool;
    pub fn vfio_pci_core_iowrite8(_: *mut vfio_pci_core_device, _: bool, _: u8, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_iowrite16(_: *mut vfio_pci_core_device, _: bool, _: u16, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_iowrite32(_: *mut vfio_pci_core_device, _: bool, _: u32, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_iowrite64(_: *mut vfio_pci_core_device, _: bool, _: u64, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread8(_: *mut vfio_pci_core_device, _: bool, _: *mut u8, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread16(_: *mut vfio_pci_core_device, _: bool, _: *mut u16, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread32(_: *mut vfio_pci_core_device, _: bool, _: *mut u32, _: *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread64(_: *mut vfio_pci_core_device, _: bool, _: *mut u64, _: *mut core::ffi::c_void) -> i32;
}

#[inline]
pub unsafe fn is_aligned_for_order(vma: *const vm_area_struct, addr: usize, pfn: usize, order: u32) -> bool {
    !(order != 0 && (addr < (*vma).vm_start || addr.wrapping_add(PAGE_SIZE << order) > (*vma).vm_end || pfn & ((1usize << order) - 1) != 0))
}

extern "C" {
    pub fn vfio_pci_core_get_iomap(vdev: *mut vfio_pci_core_device, bar: u32) -> *mut core::ffi::c_void;
    pub fn vfio_pci_dma_buf_iommufd_map(_: *mut dma_buf_attachment, _: *mut phys_vec) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
