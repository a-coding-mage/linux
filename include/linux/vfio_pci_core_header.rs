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
    pub fn vfio_pci_core_fill_phys_vec(*mut phys_vec, *mut vfio_region_dma_range, usize, phys_addr_t, phys_addr_t) -> i32;
    pub fn vfio_pci_core_get_dmabuf_phys(*mut vfio_pci_core_device, *mut *mut p2pdma_provider, u32, *mut phys_vec, *mut vfio_region_dma_range, usize) -> i32;
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
    pub fn vfio_pci_core_register_dev_region(*mut vfio_pci_core_device, u32, u32, *const vfio_pci_regops, usize, u32, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_close_device(*mut vfio_device);
    pub fn vfio_pci_core_init_dev(*mut vfio_device) -> i32;
    pub fn vfio_pci_core_release_dev(*mut vfio_device);
    pub fn vfio_pci_core_register_device(*mut vfio_pci_core_device) -> i32;
    pub fn vfio_pci_core_unregister_device(*mut vfio_pci_core_device);
    pub static vfio_pci_core_err_handlers: pci_error_handlers;
    pub fn vfio_pci_core_sriov_configure(*mut vfio_pci_core_device, i32) -> i32;
    pub fn vfio_pci_core_ioctl(*mut vfio_device, u32, usize) -> isize;
    pub fn vfio_pci_core_ioctl_feature(*mut vfio_device, u32, *mut core::ffi::c_void, usize) -> i32;
    pub fn vfio_pci_ioctl_get_region_info(*mut vfio_device, *mut vfio_region_info, *mut vfio_info_cap) -> i32;
    pub fn vfio_pci_core_read(*mut vfio_device, *mut u8, usize, *mut loff_t) -> ssize_t;
    pub fn vfio_pci_core_write(*mut vfio_device, *const u8, usize, *mut loff_t) -> ssize_t;
    pub fn vfio_pci_vmf_insert_pfn(*mut vfio_pci_core_device, *mut vm_fault, usize, u32) -> vm_fault_t;
    pub fn vfio_pci_core_mmap(*mut vfio_device, *mut vm_area_struct) -> i32;
    pub fn vfio_pci_core_request(*mut vfio_device, u32);
    pub fn vfio_pci_core_match(*mut vfio_device, *mut u8) -> i32;
    pub fn vfio_pci_core_match_token_uuid(*mut vfio_device, *const uuid_t) -> i32;
    pub fn vfio_pci_core_enable(*mut vfio_pci_core_device) -> i32;
    pub fn vfio_pci_core_disable(*mut vfio_pci_core_device);
    pub fn vfio_pci_core_finish_enable(*mut vfio_pci_core_device);
    pub fn vfio_pci_core_aer_err_detected(*mut pci_dev, pci_channel_state_t) -> pci_ers_result_t;
    pub fn vfio_pci_core_do_io_rw(*mut vfio_pci_core_device, bool, *mut core::ffi::c_void, *mut u8, loff_t, usize, usize, usize, bool, vfio_pci_io_width) -> ssize_t;
    pub fn __vfio_pci_memory_enabled(*mut vfio_pci_core_device) -> bool;
    pub fn vfio_pci_core_range_intersect_range(loff_t, usize, loff_t, usize, *mut loff_t, *mut usize, *mut usize) -> bool;
    pub fn vfio_pci_core_iowrite8(*mut vfio_pci_core_device, bool, u8, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_iowrite16(*mut vfio_pci_core_device, bool, u16, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_iowrite32(*mut vfio_pci_core_device, bool, u32, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_iowrite64(*mut vfio_pci_core_device, bool, u64, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread8(*mut vfio_pci_core_device, bool, *mut u8, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread16(*mut vfio_pci_core_device, bool, *mut u16, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread32(*mut vfio_pci_core_device, bool, *mut u32, *mut core::ffi::c_void) -> i32;
    pub fn vfio_pci_core_ioread64(*mut vfio_pci_core_device, bool, *mut u64, *mut core::ffi::c_void) -> i32;
}

#[inline]
pub unsafe fn is_aligned_for_order(vma: *const vm_area_struct, addr: usize, pfn: usize, order: u32) -> bool {
    !(order != 0 && (addr < (*vma).vm_start || addr.wrapping_add(PAGE_SIZE << order) > (*vma).vm_end || pfn & ((1usize << order) - 1) != 0))
}

extern "C" {
    pub fn vfio_pci_core_get_iomap(vdev: *mut vfio_pci_core_device, bar: u32) -> *mut core::ffi::c_void;
    pub fn vfio_pci_dma_buf_iommufd_map(*mut dma_buf_attachment, *mut phys_vec) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
