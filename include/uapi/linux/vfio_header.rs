/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of uapi/linux/vfio.h. Linux ABI types and _IO are external dependencies. */

pub const VFIO_API_VERSION: u32 = 0;
pub const VFIO_TYPE: u32 = b';' as u32;
pub const VFIO_BASE: u32 = 100;

pub const VFIO_TYPE1_IOMMU: u32 = 1;
pub const VFIO_SPAPR_TCE_IOMMU: u32 = 2;
pub const VFIO_TYPE1v2_IOMMU: u32 = 3;
pub const VFIO_DMA_CC_IOMMU: u32 = 4;
pub const VFIO_EEH: u32 = 5;
pub const __VFIO_RESERVED_TYPE1_NESTING_IOMMU: u32 = 6;
pub const VFIO_SPAPR_TCE_v2_IOMMU: u32 = 7;
pub const VFIO_NOIOMMU_IOMMU: u32 = 8;
pub const VFIO_UNMAP_ALL: u32 = 9;
pub const VFIO_UPDATE_VADDR: u32 = 10;

#[repr(C)]
pub struct vfio_info_cap_header { pub id: __u16, pub version: __u16, pub next: __u32 }

pub const VFIO_GET_API_VERSION: _ = _IO(VFIO_TYPE, VFIO_BASE + 0);
pub const VFIO_CHECK_EXTENSION: _ = _IO(VFIO_TYPE, VFIO_BASE + 1);
pub const VFIO_SET_IOMMU: _ = _IO(VFIO_TYPE, VFIO_BASE + 2);

#[repr(C)]
pub struct vfio_group_status { pub argsz: __u32, pub flags: __u32 }
pub const VFIO_GROUP_FLAGS_VIABLE: __u32 = 1 << 0;
pub const VFIO_GROUP_FLAGS_CONTAINER_SET: __u32 = 1 << 1;
pub const VFIO_GROUP_GET_STATUS: _ = _IO(VFIO_TYPE, VFIO_BASE + 3);
pub const VFIO_GROUP_SET_CONTAINER: _ = _IO(VFIO_TYPE, VFIO_BASE + 4);
pub const VFIO_GROUP_UNSET_CONTAINER: _ = _IO(VFIO_TYPE, VFIO_BASE + 5);
pub const VFIO_GROUP_GET_DEVICE_FD: _ = _IO(VFIO_TYPE, VFIO_BASE + 6);

#[repr(C)]
pub struct vfio_device_info { pub argsz: __u32, pub flags: __u32, pub num_regions: __u32, pub num_irqs: __u32, pub cap_offset: __u32, pub pad: __u32 }
pub const VFIO_DEVICE_FLAGS_RESET: __u32 = 1<<0; pub const VFIO_DEVICE_FLAGS_PCI: __u32 = 1<<1;
pub const VFIO_DEVICE_FLAGS_PLATFORM: __u32 = 1<<2; pub const VFIO_DEVICE_FLAGS_AMBA: __u32 = 1<<3;
pub const VFIO_DEVICE_FLAGS_CCW: __u32 = 1<<4; pub const VFIO_DEVICE_FLAGS_AP: __u32 = 1<<5;
pub const VFIO_DEVICE_FLAGS_FSL_MC: __u32 = 1<<6; pub const VFIO_DEVICE_FLAGS_CAPS: __u32 = 1<<7;
pub const VFIO_DEVICE_FLAGS_CDX: __u32 = 1<<8;
pub const VFIO_DEVICE_GET_INFO: _ = _IO(VFIO_TYPE, VFIO_BASE + 7);
pub const VFIO_DEVICE_API_PCI_STRING: &str = "vfio-pci";
pub const VFIO_DEVICE_API_PLATFORM_STRING: &str = "vfio-platform";
pub const VFIO_DEVICE_API_AMBA_STRING: &str = "vfio-amba";
pub const VFIO_DEVICE_API_CCW_STRING: &str = "vfio-ccw";
pub const VFIO_DEVICE_API_AP_STRING: &str = "vfio-ap";
pub const VFIO_DEVICE_INFO_CAP_ZPCI_BASE: u32=1; pub const VFIO_DEVICE_INFO_CAP_ZPCI_GROUP: u32=2;
pub const VFIO_DEVICE_INFO_CAP_ZPCI_UTIL: u32=3; pub const VFIO_DEVICE_INFO_CAP_ZPCI_PFIP: u32=4;
pub const VFIO_DEVICE_INFO_CAP_PCI_ATOMIC_COMP: u32=5;
#[repr(C)] pub struct vfio_device_info_cap_pci_atomic_comp { pub header: vfio_info_cap_header, pub flags: __u32, pub reserved: __u32 }
pub const VFIO_PCI_ATOMIC_COMP32: __u32=1<<0; pub const VFIO_PCI_ATOMIC_COMP64: __u32=1<<1; pub const VFIO_PCI_ATOMIC_COMP128: __u32=1<<2;

#[repr(C)] pub struct vfio_region_info { pub argsz: __u32, pub flags: __u32, pub index: __u32, pub cap_offset: __u32, pub size: __aligned_u64, pub offset: __aligned_u64 }
pub const VFIO_REGION_INFO_FLAG_READ: __u32=1<<0; pub const VFIO_REGION_INFO_FLAG_WRITE: __u32=1<<1; pub const VFIO_REGION_INFO_FLAG_MMAP: __u32=1<<2; pub const VFIO_REGION_INFO_FLAG_CAPS: __u32=1<<3;
pub const VFIO_DEVICE_GET_REGION_INFO: _ = _IO(VFIO_TYPE, VFIO_BASE + 8);
pub const VFIO_REGION_INFO_CAP_SPARSE_MMAP: u32=1;
#[repr(C)] pub struct vfio_region_sparse_mmap_area { pub offset: __aligned_u64, pub size: __aligned_u64 }
#[repr(C)] pub struct vfio_region_info_cap_sparse_mmap { pub header: vfio_info_cap_header, pub nr_areas: __u32, pub reserved: __u32, pub areas: [vfio_region_sparse_mmap_area; 0] }
pub const VFIO_REGION_INFO_CAP_TYPE: u32=2;
#[repr(C)] pub struct vfio_region_info_cap_type { pub header: vfio_info_cap_header, pub type_: __u32, pub subtype: __u32 }
pub const VFIO_REGION_TYPE_PCI_VENDOR_TYPE: __u32=1<<31; pub const VFIO_REGION_TYPE_PCI_VENDOR_MASK: __u32=0xffff; pub const VFIO_REGION_TYPE_GFX: u32=1; pub const VFIO_REGION_TYPE_CCW: u32=2; pub const VFIO_REGION_TYPE_MIGRATION_DEPRECATED: u32=3;
pub const VFIO_REGION_SUBTYPE_INTEL_IGD_OPREGION: u32=1; pub const VFIO_REGION_SUBTYPE_INTEL_IGD_HOST_CFG: u32=2; pub const VFIO_REGION_SUBTYPE_INTEL_IGD_LPC_CFG: u32=3;
pub const VFIO_REGION_SUBTYPE_NVIDIA_NVLINK2_RAM: u32=1; pub const VFIO_REGION_SUBTYPE_IBM_NVLINK2_ATSD: u32=1; pub const VFIO_REGION_SUBTYPE_GFX_EDID: u32=1;
#[repr(C)] pub struct vfio_region_gfx_edid { pub edid_offset: __u32, pub edid_max_size: __u32, pub edid_size: __u32, pub max_xres: __u32, pub max_yres: __u32, pub link_state: __u32 }
pub const VFIO_DEVICE_GFX_LINK_STATE_UP: u32=1; pub const VFIO_DEVICE_GFX_LINK_STATE_DOWN: u32=2;
pub const VFIO_REGION_SUBTYPE_CCW_ASYNC_CMD: u32=1; pub const VFIO_REGION_SUBTYPE_CCW_SCHIB: u32=2; pub const VFIO_REGION_SUBTYPE_CCW_CRW: u32=3; pub const VFIO_REGION_SUBTYPE_MIGRATION_DEPRECATED: u32=1;
#[repr(C)] pub struct vfio_device_migration_info { pub device_state: __u32, pub reserved: __u32, pub pending_bytes: __aligned_u64, pub data_offset: __aligned_u64, pub data_size: __aligned_u64 }
pub const VFIO_DEVICE_STATE_V1_STOP: __u32=0; pub const VFIO_DEVICE_STATE_V1_RUNNING: __u32=1<<0; pub const VFIO_DEVICE_STATE_V1_SAVING: __u32=1<<1; pub const VFIO_DEVICE_STATE_V1_RESUMING: __u32=1<<2; pub const VFIO_DEVICE_STATE_MASK: __u32=7;
#[inline] pub const fn VFIO_DEVICE_STATE_VALID(state: __u32)->bool { (state & VFIO_DEVICE_STATE_V1_RESUMING)==0 || (state & VFIO_DEVICE_STATE_MASK)==VFIO_DEVICE_STATE_V1_RESUMING }
#[inline] pub const fn VFIO_DEVICE_STATE_IS_ERROR(state: __u32)->bool { (state & VFIO_DEVICE_STATE_MASK)==(VFIO_DEVICE_STATE_V1_SAVING|VFIO_DEVICE_STATE_V1_RESUMING) }
#[inline] pub const fn VFIO_DEVICE_STATE_SET_ERROR(state: __u32)->__u32 { (state & !VFIO_DEVICE_STATE_MASK)|VFIO_DEVICE_STATE_V1_SAVING|VFIO_DEVICE_STATE_V1_RESUMING }

#[repr(C)] pub struct vfio_region_info_cap_nvlink2_ssatgt { pub header: vfio_info_cap_header, pub tgt: __aligned_u64 }
pub const VFIO_REGION_INFO_CAP_MSIX_MAPPABLE: u32=3; pub const VFIO_REGION_INFO_CAP_NVLINK2_SSATGT: u32=4; pub const VFIO_REGION_INFO_CAP_NVLINK2_LNKSPD: u32=5;
#[repr(C)] pub struct vfio_region_info_cap_nvlink2_lnkspd { pub header: vfio_info_cap_header, pub link_speed: __u32, pub __pad: __u32 }
#[repr(C)] pub struct vfio_irq_info { pub argsz: __u32, pub flags: __u32, pub index: __u32, pub count: __u32 }
pub const VFIO_IRQ_INFO_EVENTFD: __u32=1<<0; pub const VFIO_IRQ_INFO_MASKABLE: __u32=1<<1; pub const VFIO_IRQ_INFO_AUTOMASKED: __u32=1<<2; pub const VFIO_IRQ_INFO_NORESIZE: __u32=1<<3; pub const VFIO_DEVICE_GET_IRQ_INFO: _=_IO(VFIO_TYPE,VFIO_BASE+9);
#[repr(C)] pub struct vfio_irq_set { pub argsz: __u32, pub flags: __u32, pub index: __u32, pub start: __u32, pub count: __u32, pub data: [__u8;0] }
pub const VFIO_IRQ_SET_DATA_NONE: __u32=1<<0; pub const VFIO_IRQ_SET_DATA_BOOL: __u32=1<<1; pub const VFIO_IRQ_SET_DATA_EVENTFD: __u32=1<<2; pub const VFIO_IRQ_SET_ACTION_MASK: __u32=1<<3; pub const VFIO_IRQ_SET_ACTION_UNMASK: __u32=1<<4; pub const VFIO_IRQ_SET_ACTION_TRIGGER: __u32=1<<5; pub const VFIO_IRQ_SET_DATA_TYPE_MASK: __u32=7; pub const VFIO_IRQ_SET_ACTION_TYPE_MASK: __u32=7<<3; pub const VFIO_DEVICE_SET_IRQS: _=_IO(VFIO_TYPE,VFIO_BASE+10); pub const VFIO_DEVICE_RESET: _=_IO(VFIO_TYPE,VFIO_BASE+11);

pub const VFIO_PCI_BAR0_REGION_INDEX:u32=0; pub const VFIO_PCI_BAR1_REGION_INDEX:u32=1; pub const VFIO_PCI_BAR2_REGION_INDEX:u32=2; pub const VFIO_PCI_BAR3_REGION_INDEX:u32=3; pub const VFIO_PCI_BAR4_REGION_INDEX:u32=4; pub const VFIO_PCI_BAR5_REGION_INDEX:u32=5; pub const VFIO_PCI_ROM_REGION_INDEX:u32=6; pub const VFIO_PCI_CONFIG_REGION_INDEX:u32=7; pub const VFIO_PCI_VGA_REGION_INDEX:u32=8; pub const VFIO_PCI_NUM_REGIONS:u32=9;
pub const VFIO_PCI_INTX_IRQ_INDEX:u32=0; pub const VFIO_PCI_MSI_IRQ_INDEX:u32=1; pub const VFIO_PCI_MSIX_IRQ_INDEX:u32=2; pub const VFIO_PCI_ERR_IRQ_INDEX:u32=3; pub const VFIO_PCI_REQ_IRQ_INDEX:u32=4; pub const VFIO_PCI_NUM_IRQS:u32=5;
pub const VFIO_CCW_CONFIG_REGION_INDEX:u32=0; pub const VFIO_CCW_NUM_REGIONS:u32=1; pub const VFIO_CCW_IO_IRQ_INDEX:u32=0; pub const VFIO_CCW_CRW_IRQ_INDEX:u32=1; pub const VFIO_CCW_REQ_IRQ_INDEX:u32=2; pub const VFIO_CCW_NUM_IRQS:u32=3; pub const VFIO_AP_REQ_IRQ_INDEX:u32=0; pub const VFIO_AP_CFG_CHG_IRQ_INDEX:u32=1; pub const VFIO_AP_NUM_IRQS:u32=2;

#[repr(C)] pub union vfio_pci_dependent_device__bindgen_ty_1 { pub group_id: __u32, pub devid: __u32 }
#[repr(C)] pub struct vfio_pci_dependent_device { pub _bindgen_union: vfio_pci_dependent_device__bindgen_ty_1, pub segment: __u16, pub bus: __u8, pub devfn: __u8 }
pub const VFIO_PCI_DEVID_OWNED: __u32=0; pub const VFIO_PCI_DEVID_NOT_OWNED: __u32=0xffff_ffff;
#[repr(C)] pub struct vfio_pci_hot_reset_info { pub argsz: __u32, pub flags: __u32, pub count: __u32, pub devices: [vfio_pci_dependent_device;0] }
pub const VFIO_PCI_HOT_RESET_FLAG_DEV_ID: __u32=1; pub const VFIO_PCI_HOT_RESET_FLAG_DEV_ID_OWNED: __u32=2; pub const VFIO_DEVICE_GET_PCI_HOT_RESET_INFO:_=_IO(VFIO_TYPE,VFIO_BASE+12);
#[repr(C)] pub struct vfio_pci_hot_reset { pub argsz:__u32,pub flags:__u32,pub count:__u32,pub group_fds:[__s32;0] } pub const VFIO_DEVICE_PCI_HOT_RESET:_=_IO(VFIO_TYPE,VFIO_BASE+13);

#[repr(C)] pub union vfio_device_gfx_plane_info__bindgen_ty_1 { pub region_index:__u32, pub dmabuf_id:__u32 }
#[repr(C)] pub struct vfio_device_gfx_plane_info { pub argsz:__u32,pub flags:__u32,pub drm_plane_type:__u32,pub drm_format:__u32,pub drm_format_mod:__aligned_u64,pub width:__u32,pub height:__u32,pub stride:__u32,pub size:__u32,pub x_pos:__u32,pub y_pos:__u32,pub x_hot:__u32,pub y_hot:__u32,pub _bindgen_union:vfio_device_gfx_plane_info__bindgen_ty_1,pub reserved:__u32 }
pub const VFIO_GFX_PLANE_TYPE_PROBE:__u32=1;pub const VFIO_GFX_PLANE_TYPE_DMABUF:__u32=2;pub const VFIO_GFX_PLANE_TYPE_REGION:__u32=4;pub const VFIO_DEVICE_QUERY_GFX_PLANE:_=_IO(VFIO_TYPE,VFIO_BASE+14);pub const VFIO_DEVICE_GET_GFX_DMABUF:_=_IO(VFIO_TYPE,VFIO_BASE+15);
#[repr(C)] pub struct vfio_device_ioeventfd { pub argsz:__u32,pub flags:__u32,pub offset:__aligned_u64,pub data:__aligned_u64,pub fd:__s32,pub reserved:__u32 }
pub const VFIO_DEVICE_IOEVENTFD_8:__u32=1;pub const VFIO_DEVICE_IOEVENTFD_16:__u32=2;pub const VFIO_DEVICE_IOEVENTFD_32:__u32=4;pub const VFIO_DEVICE_IOEVENTFD_64:__u32=8;pub const VFIO_DEVICE_IOEVENTFD_SIZE_MASK:__u32=0xf;pub const VFIO_DEVICE_IOEVENTFD:_=_IO(VFIO_TYPE,VFIO_BASE+16);
#[repr(C)] pub struct vfio_device_feature { pub argsz:__u32,pub flags:__u32,pub data:[__u8;0] } pub const VFIO_DEVICE_FEATURE_MASK:__u32=0xffff;pub const VFIO_DEVICE_FEATURE_GET:__u32=1<<16;pub const VFIO_DEVICE_FEATURE_SET:__u32=1<<17;pub const VFIO_DEVICE_FEATURE_PROBE:__u32=1<<18;pub const VFIO_DEVICE_FEATURE:_=_IO(VFIO_TYPE,VFIO_BASE+17);

#[repr(C)] pub struct vfio_device_bind_iommufd { pub argsz:__u32,pub flags:__u32,pub iommufd:__s32,pub out_devid:__u32,pub token_uuid_ptr:__aligned_u64 } pub const VFIO_DEVICE_BIND_FLAG_TOKEN:__u32=1;pub const VFIO_DEVICE_BIND_IOMMUFD:_=_IO(VFIO_TYPE,VFIO_BASE+18);
#[repr(C)] pub struct vfio_device_attach_iommufd_pt { pub argsz:__u32,pub flags:__u32,pub pt_id:__u32,pub pasid:__u32 } pub const VFIO_DEVICE_ATTACH_PASID:__u32=1;pub const VFIO_DEVICE_ATTACH_IOMMUFD_PT:_=_IO(VFIO_TYPE,VFIO_BASE+19);
#[repr(C)] pub struct vfio_device_detach_iommufd_pt { pub argsz:__u32,pub flags:__u32,pub pasid:__u32 } pub const VFIO_DEVICE_DETACH_PASID:__u32=1;pub const VFIO_DEVICE_DETACH_IOMMUFD_PT:_=_IO(VFIO_TYPE,VFIO_BASE+20);
pub const VFIO_DEVICE_FEATURE_PCI_VF_TOKEN:u32=0;
#[repr(u32)] pub enum vfio_device_mig_state { VFIO_DEVICE_STATE_ERROR=0, VFIO_DEVICE_STATE_STOP=1, VFIO_DEVICE_STATE_RUNNING=2, VFIO_DEVICE_STATE_STOP_COPY=3, VFIO_DEVICE_STATE_RESUMING=4, VFIO_DEVICE_STATE_RUNNING_P2P=5, VFIO_DEVICE_STATE_PRE_COPY=6, VFIO_DEVICE_STATE_PRE_COPY_P2P=7, VFIO_DEVICE_STATE_NR=8 }
#[repr(C)] pub struct vfio_device_feature_migration { pub flags:__aligned_u64 } pub const VFIO_MIGRATION_STOP_COPY:__u64=1;pub const VFIO_MIGRATION_P2P:__u64=2;pub const VFIO_MIGRATION_PRE_COPY:__u64=4;pub const VFIO_DEVICE_FEATURE_MIGRATION:u32=1;
#[repr(C)] pub struct vfio_device_feature_mig_state { pub device_state:__u32,pub data_fd:__s32 } pub const VFIO_DEVICE_FEATURE_MIG_DEVICE_STATE:u32=2;
#[repr(C)] pub struct vfio_precopy_info { pub argsz:__u32,pub flags:__u32,pub initial_bytes:__aligned_u64,pub dirty_bytes:__aligned_u64 } pub const VFIO_PRECOPY_INFO_REINIT:__u32=1;
pub const VFIO_MIG_GET_PRECOPY_INFO:_=_IO(VFIO_TYPE,VFIO_BASE+21);
#[repr(C)] pub struct vfio_device_low_power_entry_with_wakeup { pub wakeup_eventfd:__s32,pub reserved:__u32 }
pub const VFIO_DEVICE_FEATURE_LOW_POWER_ENTRY:u32=3;pub const VFIO_DEVICE_FEATURE_LOW_POWER_ENTRY_WITH_WAKEUP:u32=4;pub const VFIO_DEVICE_FEATURE_LOW_POWER_EXIT:u32=5;
#[repr(C)] pub struct vfio_device_feature_dma_logging_control { pub page_size:__aligned_u64,pub num_ranges:__u32,pub __reserved:__u32,pub ranges:__aligned_u64 }
#[repr(C)] pub struct vfio_device_feature_dma_logging_range { pub iova:__aligned_u64,pub length:__aligned_u64 }
pub const VFIO_DEVICE_FEATURE_DMA_LOGGING_START:u32=6;pub const VFIO_DEVICE_FEATURE_DMA_LOGGING_STOP:u32=7;
#[repr(C)] pub struct vfio_device_feature_dma_logging_report { pub iova:__aligned_u64,pub length:__aligned_u64,pub page_size:__aligned_u64,pub bitmap:__aligned_u64 } pub const VFIO_DEVICE_FEATURE_DMA_LOGGING_REPORT:u32=8;
#[repr(C)] pub struct vfio_device_feature_mig_data_size { pub stop_copy_length:__aligned_u64 } pub const VFIO_DEVICE_FEATURE_MIG_DATA_SIZE:u32=9;
#[repr(C)] pub struct vfio_device_feature_bus_master { pub op:__u32 } pub const VFIO_DEVICE_FEATURE_CLEAR_MASTER:__u32=0;pub const VFIO_DEVICE_FEATURE_SET_MASTER:__u32=1;pub const VFIO_DEVICE_FEATURE_BUS_MASTER:u32=10;pub const VFIO_DEVICE_FEATURE_DMA_BUF:u32=11;
#[repr(C)] pub struct vfio_region_dma_range { pub offset:__u64,pub length:__u64 }
#[repr(C)] pub struct vfio_device_feature_dma_buf { pub region_index:__u32,pub open_flags:__u32,pub flags:__u32,pub nr_ranges:__u32,pub dma_ranges:[vfio_region_dma_range;0] }
pub const VFIO_DEVICE_FEATURE_MIG_PRECOPY_INFOv2:u32=12; #[repr(C)] pub struct vfio_device_feature_zpci_err { pub data:__aligned_u64 } pub const VFIO_DEVICE_FEATURE_ZPCI_ERROR:u32=13;

#[repr(C)] pub struct vfio_iommu_type1_info { pub argsz:__u32,pub flags:__u32,pub iova_pgsizes:__aligned_u64,pub cap_offset:__u32,pub pad:__u32 } pub const VFIO_IOMMU_INFO_PGSIZES:__u32=1;pub const VFIO_IOMMU_INFO_CAPS:__u32=2;pub const VFIO_IOMMU_TYPE1_INFO_CAP_IOVA_RANGE:u32=1;
#[repr(C)] pub struct vfio_iova_range { pub start:__u64,pub end:__u64 } #[repr(C)] pub struct vfio_iommu_type1_info_cap_iova_range { pub header:vfio_info_cap_header,pub nr_iovas:__u32,pub reserved:__u32,pub iova_ranges:[vfio_iova_range;0] }
pub const VFIO_IOMMU_TYPE1_INFO_CAP_MIGRATION:u32=2;#[repr(C)] pub struct vfio_iommu_type1_info_cap_migration { pub header:vfio_info_cap_header,pub flags:__u32,pub pgsize_bitmap:__u64,pub max_dirty_bitmap_size:__u64 }
pub const VFIO_IOMMU_TYPE1_INFO_DMA_AVAIL:u32=3;#[repr(C)] pub struct vfio_iommu_type1_info_dma_avail { pub header:vfio_info_cap_header,pub avail:__u32 }
pub const VFIO_IOMMU_GET_INFO:_=_IO(VFIO_TYPE,VFIO_BASE+12);
#[repr(C)] pub struct vfio_iommu_type1_dma_map { pub argsz:__u32,pub flags:__u32,pub vaddr:__u64,pub iova:__u64,pub size:__u64 } pub const VFIO_DMA_MAP_FLAG_READ:__u32=1;pub const VFIO_DMA_MAP_FLAG_WRITE:__u32=2;pub const VFIO_DMA_MAP_FLAG_VADDR:__u32=4;pub const VFIO_IOMMU_MAP_DMA:_=_IO(VFIO_TYPE,VFIO_BASE+13);
#[repr(C)] pub struct vfio_bitmap { pub pgsize:__u64,pub size:__u64,pub data:*mut __u64 }
#[repr(C)] pub struct vfio_iommu_type1_dma_unmap { pub argsz:__u32,pub flags:__u32,pub iova:__u64,pub size:__u64,pub data:[__u8;0] } pub const VFIO_DMA_UNMAP_FLAG_GET_DIRTY_BITMAP:__u32=1;pub const VFIO_DMA_UNMAP_FLAG_ALL:__u32=2;pub const VFIO_DMA_UNMAP_FLAG_VADDR:__u32=4;pub const VFIO_IOMMU_UNMAP_DMA:_=_IO(VFIO_TYPE,VFIO_BASE+14);pub const VFIO_IOMMU_ENABLE:_=_IO(VFIO_TYPE,VFIO_BASE+15);pub const VFIO_IOMMU_DISABLE:_=_IO(VFIO_TYPE,VFIO_BASE+16);
#[repr(C)] pub struct vfio_iommu_type1_dirty_bitmap { pub argsz:__u32,pub flags:__u32,pub data:[__u8;0] } pub const VFIO_IOMMU_DIRTY_PAGES_FLAG_START:__u32=1;pub const VFIO_IOMMU_DIRTY_PAGES_FLAG_STOP:__u32=2;pub const VFIO_IOMMU_DIRTY_PAGES_FLAG_GET_BITMAP:__u32=4; #[repr(C)] pub struct vfio_iommu_type1_dirty_bitmap_get { pub iova:__u64,pub size:__u64,pub bitmap:vfio_bitmap } pub const VFIO_IOMMU_DIRTY_PAGES:_=_IO(VFIO_TYPE,VFIO_BASE+17);

#[repr(C)] pub struct vfio_iommu_spapr_tce_ddw_info { pub pgsizes:__u64,pub max_dynamic_windows_supported:__u32,pub levels:__u32 }
#[repr(C)] pub struct vfio_iommu_spapr_tce_info { pub argsz:__u32,pub flags:__u32,pub dma32_window_start:__u32,pub dma32_window_size:__u32,pub ddw:vfio_iommu_spapr_tce_ddw_info } pub const VFIO_IOMMU_SPAPR_INFO_DDW:__u32=1;pub const VFIO_IOMMU_SPAPR_TCE_GET_INFO:_=_IO(VFIO_TYPE,VFIO_BASE+12);
#[repr(C)] pub struct vfio_eeh_pe_err { pub type_:__u32,pub func:__u32,pub addr:__u64,pub mask:__u64 } #[repr(C)] pub union vfio_eeh_pe_op__bindgen_ty_1 { pub err:vfio_eeh_pe_err } #[repr(C)] pub struct vfio_eeh_pe_op { pub argsz:__u32,pub flags:__u32,pub op:__u32,pub _bindgen_union:vfio_eeh_pe_op__bindgen_ty_1 }
pub const VFIO_EEH_PE_DISABLE:u32=0;pub const VFIO_EEH_PE_ENABLE:u32=1;pub const VFIO_EEH_PE_UNFREEZE_IO:u32=2;pub const VFIO_EEH_PE_UNFREEZE_DMA:u32=3;pub const VFIO_EEH_PE_GET_STATE:u32=4;pub const VFIO_EEH_PE_STATE_NORMAL:u32=0;pub const VFIO_EEH_PE_STATE_RESET:u32=1;pub const VFIO_EEH_PE_STATE_STOPPED:u32=2;pub const VFIO_EEH_PE_STATE_STOPPED_DMA:u32=4;pub const VFIO_EEH_PE_STATE_UNAVAIL:u32=5;pub const VFIO_EEH_PE_RESET_DEACTIVATE:u32=5;pub const VFIO_EEH_PE_RESET_HOT:u32=6;pub const VFIO_EEH_PE_RESET_FUNDAMENTAL:u32=7;pub const VFIO_EEH_PE_CONFIGURE:u32=8;pub const VFIO_EEH_PE_INJECT_ERR:u32=9;pub const VFIO_EEH_PE_OP:_=_IO(VFIO_TYPE,VFIO_BASE+21);
#[repr(C)] pub struct vfio_iommu_spapr_register_memory { pub argsz:__u32,pub flags:__u32,pub vaddr:__u64,pub size:__u64 } pub const VFIO_IOMMU_SPAPR_REGISTER_MEMORY:_=_IO(VFIO_TYPE,VFIO_BASE+17);pub const VFIO_IOMMU_SPAPR_UNREGISTER_MEMORY:_=_IO(VFIO_TYPE,VFIO_BASE+18);
#[repr(C)] pub struct vfio_iommu_spapr_tce_create { pub argsz:__u32,pub flags:__u32,pub page_shift:__u32,pub __resv1:__u32,pub window_size:__u64,pub levels:__u32,pub __resv2:__u32,pub start_addr:__u64 } pub const VFIO_IOMMU_SPAPR_TCE_CREATE:_=_IO(VFIO_TYPE,VFIO_BASE+19);
#[repr(C)] pub struct vfio_iommu_spapr_tce_remove { pub argsz:__u32,pub flags:__u32,pub start_addr:__u64 } pub const VFIO_IOMMU_SPAPR_TCE_REMOVE:_=_IO(VFIO_TYPE,VFIO_BASE+20);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
