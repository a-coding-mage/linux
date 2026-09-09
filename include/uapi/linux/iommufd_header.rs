/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of the Linux UAPI iommufd header. */

// The ioctl encoding helper and these integer types are supplied by the
// surrounding UAPI bindings.
// use linux_ioctl::_IO;

pub const IOMMUFD_TYPE: u32 = b';' as u32;

pub const IOMMUFD_CMD_BASE: u32 = 0x80;
pub const IOMMUFD_CMD_DESTROY: u32 = IOMMUFD_CMD_BASE;
pub const IOMMUFD_CMD_IOAS_ALLOC: u32 = 0x81;
pub const IOMMUFD_CMD_IOAS_ALLOW_IOVAS: u32 = 0x82;
pub const IOMMUFD_CMD_IOAS_COPY: u32 = 0x83;
pub const IOMMUFD_CMD_IOAS_IOVA_RANGES: u32 = 0x84;
pub const IOMMUFD_CMD_IOAS_MAP: u32 = 0x85;
pub const IOMMUFD_CMD_IOAS_UNMAP: u32 = 0x86;
pub const IOMMUFD_CMD_OPTION: u32 = 0x87;
pub const IOMMUFD_CMD_VFIO_IOAS: u32 = 0x88;
pub const IOMMUFD_CMD_HWPT_ALLOC: u32 = 0x89;
pub const IOMMUFD_CMD_GET_HW_INFO: u32 = 0x8a;
pub const IOMMUFD_CMD_HWPT_SET_DIRTY_TRACKING: u32 = 0x8b;
pub const IOMMUFD_CMD_HWPT_GET_DIRTY_BITMAP: u32 = 0x8c;
pub const IOMMUFD_CMD_HWPT_INVALIDATE: u32 = 0x8d;
pub const IOMMUFD_CMD_FAULT_QUEUE_ALLOC: u32 = 0x8e;
pub const IOMMUFD_CMD_IOAS_MAP_FILE: u32 = 0x8f;
pub const IOMMUFD_CMD_VIOMMU_ALLOC: u32 = 0x90;
pub const IOMMUFD_CMD_VDEVICE_ALLOC: u32 = 0x91;
pub const IOMMUFD_CMD_IOAS_CHANGE_PROCESS: u32 = 0x92;
pub const IOMMUFD_CMD_VEVENTQ_ALLOC: u32 = 0x93;
pub const IOMMUFD_CMD_HW_QUEUE_ALLOC: u32 = 0x94;
pub const IOMMUFD_CMD_IOAS_NOIOMMU_GET_PA: u32 = 0x95;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_destroy { pub size: u32, pub id: u32 }
pub const IOMMU_DESTROY: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_DESTROY);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_alloc { pub size: u32, pub flags: u32, pub out_ioas_id: u32 }
pub const IOMMU_IOAS_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_ALLOC);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_iova_range { pub start: u64, pub last: u64 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_iova_ranges { pub size: u32, pub ioas_id: u32, pub num_iovas: u32, pub __reserved: u32, pub allowed_iovas: u64, pub out_iova_alignment: u64 }
pub const IOMMU_IOAS_IOVA_RANGES: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_IOVA_RANGES);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_allow_iovas { pub size: u32, pub ioas_id: u32, pub num_iovas: u32, pub __reserved: u32, pub allowed_iovas: u64 }
pub const IOMMU_IOAS_ALLOW_IOVAS: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_ALLOW_IOVAS);

pub const IOMMU_IOAS_MAP_FIXED_IOVA: u32 = 1 << 0;
pub const IOMMU_IOAS_MAP_WRITEABLE: u32 = 1 << 1;
pub const IOMMU_IOAS_MAP_READABLE: u32 = 1 << 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_map { pub size: u32, pub flags: u32, pub ioas_id: u32, pub __reserved: u32, pub user_va: u64, pub length: u64, pub iova: u64 }
pub const IOMMU_IOAS_MAP: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_MAP);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_noiommu_get_pa { pub size: u32, pub flags: u32, pub ioas_id: u32, pub __reserved: u32, pub iova: u64, pub length: u64, pub out_phys: u64 }
pub const IOMMU_IOAS_NOIOMMU_GET_PA: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_NOIOMMU_GET_PA);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_map_file { pub size: u32, pub flags: u32, pub ioas_id: u32, pub fd: i32, pub start: u64, pub length: u64, pub iova: u64 }
pub const IOMMU_IOAS_MAP_FILE: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_MAP_FILE);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_copy { pub size: u32, pub flags: u32, pub dst_ioas_id: u32, pub src_ioas_id: u32, pub length: u64, pub dst_iova: u64, pub src_iova: u64 }
pub const IOMMU_IOAS_COPY: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_COPY);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_unmap { pub size: u32, pub ioas_id: u32, pub iova: u64, pub length: u64 }
pub const IOMMU_IOAS_UNMAP: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_UNMAP);

pub const IOMMU_OPTION_RLIMIT_MODE: u32 = 0;
pub const IOMMU_OPTION_HUGE_PAGES: u32 = 1;
pub const IOMMU_OPTION_OP_SET: u32 = 0;
pub const IOMMU_OPTION_OP_GET: u32 = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_option { pub size: u32, pub option_id: u32, pub op: u16, pub __reserved: u16, pub object_id: u32, pub val64: u64 }
pub const IOMMU_OPTION: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_OPTION);

pub const IOMMU_VFIO_IOAS_GET: u32 = 0;
pub const IOMMU_VFIO_IOAS_SET: u32 = 1;
pub const IOMMU_VFIO_IOAS_CLEAR: u32 = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_vfio_ioas { pub size: u32, pub ioas_id: u32, pub op: u16, pub __reserved: u16 }
pub const IOMMU_VFIO_IOAS: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_VFIO_IOAS);

pub const IOMMU_HWPT_ALLOC_NEST_PARENT: u32 = 1 << 0;
pub const IOMMU_HWPT_ALLOC_DIRTY_TRACKING: u32 = 1 << 1;
pub const IOMMU_HWPT_FAULT_ID_VALID: u32 = 1 << 2;
pub const IOMMU_HWPT_ALLOC_PASID: u32 = 1 << 3;
pub const IOMMU_VTD_S1_SRE: u64 = 1 << 0;
pub const IOMMU_VTD_S1_EAFE: u64 = 1 << 1;
pub const IOMMU_VTD_S1_WPE: u64 = 1 << 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_vtd_s1 { pub flags: u64, pub pgtbl_addr: u64, pub addr_width: u32, pub __reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_arm_smmuv3 { pub ste: [u64; 2] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_amd_guest { pub dte: [u64; 4] }
pub const IOMMU_HWPT_DATA_NONE: u32 = 0;
pub const IOMMU_HWPT_DATA_VTD_S1: u32 = 1;
pub const IOMMU_HWPT_DATA_ARM_SMMUV3: u32 = 2;
pub const IOMMU_HWPT_DATA_AMD_GUEST: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_alloc { pub size: u32, pub flags: u32, pub dev_id: u32, pub pt_id: u32, pub out_hwpt_id: u32, pub __reserved: u32, pub data_type: u32, pub data_len: u32, pub data_uptr: u64, pub fault_id: u32, pub __reserved2: u32 }
pub const IOMMU_HWPT_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_HWPT_ALLOC);

pub const IOMMU_HW_INFO_VTD_ERRATA_772415_SPR17: u32 = 1 << 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hw_info_vtd { pub flags: u32, pub __reserved: u32, pub cap_reg: u64, pub ecap_reg: u64 }
pub const IOMMU_HW_INFO_ARM_SMMUV3_ERRATA_REPEAT_TLBI_CFGI: u32 = 1 << 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hw_info_arm_smmuv3 { pub flags: u32, pub __reserved: u32, pub idr: [u32; 6], pub iidr: u32, pub aidr: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hw_info_tegra241_cmdqv { pub flags: u32, pub version: u8, pub log2vcmdqs: u8, pub log2vsids: u8, pub __reserved: u8 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hw_info_amd { pub efr: u64, pub efr2: u64 }
pub const IOMMU_HW_INFO_TYPE_NONE: u32 = 0;
pub const IOMMU_HW_INFO_TYPE_DEFAULT: u32 = 0;
pub const IOMMU_HW_INFO_TYPE_INTEL_VTD: u32 = 1;
pub const IOMMU_HW_INFO_TYPE_ARM_SMMUV3: u32 = 2;
pub const IOMMU_HW_INFO_TYPE_TEGRA241_CMDQV: u32 = 3;
pub const IOMMU_HW_INFO_TYPE_AMD: u32 = 4;
pub const IOMMU_HW_CAP_DIRTY_TRACKING: u64 = 1 << 0;
pub const IOMMU_HW_CAP_PCI_PASID_EXEC: u64 = 1 << 1;
pub const IOMMU_HW_CAP_PCI_PASID_PRIV: u64 = 1 << 2;
pub const IOMMU_HW_CAP_PCI_ATS_NOT_SUPPORTED: u64 = 1 << 3;
pub const IOMMU_HW_INFO_FLAG_INPUT_TYPE: u32 = 1 << 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub union iommu_hw_info_data_type { pub in_data_type: u32, pub out_data_type: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hw_info { pub size: u32, pub flags: u32, pub dev_id: u32, pub data_len: u32, pub data_uptr: u64, pub data_type: iommu_hw_info_data_type, pub out_max_pasid_log2: u8, pub __reserved: [u8; 3], pub out_capabilities: u64 }
pub const IOMMU_GET_HW_INFO: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_GET_HW_INFO);

pub const IOMMU_HWPT_DIRTY_TRACKING_ENABLE: u32 = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_set_dirty_tracking { pub size: u32, pub flags: u32, pub hwpt_id: u32, pub __reserved: u32 }
pub const IOMMU_HWPT_SET_DIRTY_TRACKING: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_HWPT_SET_DIRTY_TRACKING);
pub const IOMMU_HWPT_GET_DIRTY_BITMAP_NO_CLEAR: u32 = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_get_dirty_bitmap { pub size: u32, pub hwpt_id: u32, pub flags: u32, pub __reserved: u32, pub iova: u64, pub length: u64, pub page_size: u64, pub data: u64 }
pub const IOMMU_HWPT_GET_DIRTY_BITMAP: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_HWPT_GET_DIRTY_BITMAP);

pub const IOMMU_HWPT_INVALIDATE_DATA_VTD_S1: u32 = 0;
pub const IOMMU_VIOMMU_INVALIDATE_DATA_ARM_SMMUV3: u32 = 1;
pub const IOMMU_VTD_INV_FLAGS_LEAF: u32 = 1 << 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_vtd_s1_invalidate { pub addr: u64, pub npages: u64, pub flags: u32, pub __reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_viommu_arm_smmuv3_invalidate { pub cmd: [u64; 2] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_invalidate { pub size: u32, pub hwpt_id: u32, pub data_uptr: u64, pub data_type: u32, pub entry_len: u32, pub entry_num: u32, pub __reserved: u32 }
pub const IOMMU_HWPT_INVALIDATE: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_HWPT_INVALIDATE);

pub const IOMMU_PGFAULT_FLAGS_PASID_VALID: u32 = 1 << 0;
pub const IOMMU_PGFAULT_FLAGS_LAST_PAGE: u32 = 1 << 1;
pub const IOMMU_PGFAULT_PERM_READ: u32 = 1 << 0;
pub const IOMMU_PGFAULT_PERM_WRITE: u32 = 1 << 1;
pub const IOMMU_PGFAULT_PERM_EXEC: u32 = 1 << 2;
pub const IOMMU_PGFAULT_PERM_PRIV: u32 = 1 << 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_pgfault { pub flags: u32, pub dev_id: u32, pub pasid: u32, pub grpid: u32, pub perm: u32, pub __reserved: u32, pub addr: u64, pub length: u32, pub cookie: u32 }
pub const IOMMUFD_PAGE_RESP_SUCCESS: u32 = 0;
pub const IOMMUFD_PAGE_RESP_INVALID: u32 = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hwpt_page_response { pub cookie: u32, pub code: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_fault_alloc { pub size: u32, pub flags: u32, pub out_fault_id: u32, pub out_fault_fd: u32 }
pub const IOMMU_FAULT_QUEUE_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_FAULT_QUEUE_ALLOC);

pub const IOMMU_VIOMMU_TYPE_DEFAULT: u32 = 0;
pub const IOMMU_VIOMMU_TYPE_ARM_SMMUV3: u32 = 1;
pub const IOMMU_VIOMMU_TYPE_TEGRA241_CMDQV: u32 = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_viommu_tegra241_cmdqv { pub out_vintf_mmap_offset: u64, pub out_vintf_mmap_length: u64 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_viommu_alloc { pub size: u32, pub flags: u32, pub r#type: u32, pub dev_id: u32, pub hwpt_id: u32, pub out_viommu_id: u32, pub data_len: u32, pub __reserved: u32, pub data_uptr: u64 }
pub const IOMMU_VIOMMU_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_VIOMMU_ALLOC);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_vdevice_alloc { pub size: u32, pub viommu_id: u32, pub dev_id: u32, pub out_vdevice_id: u32, pub virt_id: u64 }
pub const IOMMU_VDEVICE_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_VDEVICE_ALLOC);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_ioas_change_process { pub size: u32, pub __reserved: u32 }
pub const IOMMU_IOAS_CHANGE_PROCESS: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_IOAS_CHANGE_PROCESS);

pub const IOMMU_VEVENTQ_FLAG_LOST_EVENTS: u32 = 1 << 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommufd_vevent_header { pub flags: u32, pub sequence: u32 }
pub const IOMMU_VEVENTQ_TYPE_DEFAULT: u32 = 0;
pub const IOMMU_VEVENTQ_TYPE_ARM_SMMUV3: u32 = 1;
pub const IOMMU_VEVENTQ_TYPE_TEGRA241_CMDQV: u32 = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_vevent_arm_smmuv3 { pub evt: [u64; 4] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_vevent_tegra241_cmdqv { pub lvcmdq_err_map: [u64; 2] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_veventq_alloc { pub size: u32, pub flags: u32, pub viommu_id: u32, pub r#type: u32, pub veventq_depth: u32, pub out_veventq_id: u32, pub out_veventq_fd: u32, pub __reserved: u32 }
pub const IOMMU_VEVENTQ_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_VEVENTQ_ALLOC);

pub const IOMMU_HW_QUEUE_TYPE_DEFAULT: u32 = 0;
pub const IOMMU_HW_QUEUE_TYPE_TEGRA241_CMDQV: u32 = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iommu_hw_queue_alloc { pub size: u32, pub flags: u32, pub viommu_id: u32, pub r#type: u32, pub index: u32, pub out_hw_queue_id: u32, pub nesting_parent_iova: u64, pub length: u64 }
pub const IOMMU_HW_QUEUE_ALLOC: u32 = _IO(IOMMUFD_TYPE, IOMMUFD_CMD_HW_QUEUE_ALLOC);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
