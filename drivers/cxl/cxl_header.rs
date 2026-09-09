/* SPDX-License-Identifier: GPL-2.0-only */
/* Source-level Rust translation of cxl.h. */

// Linux dependencies are supplied by the surrounding translation unit.
use core::ffi::c_void;

pub const CXL_COMPONENT_REG_BLOCK_SIZE: usize = 0x10000;
pub const CXL_CM_OFFSET: u32 = 0x1000;
pub const CXL_CM_CAP_HDR_OFFSET: u32 = 0;
pub const CXL_CM_CAP_HDR_ID_MASK: u32 = 0xffff;
pub const CM_CAP_HDR_CAP_ID: u32 = 1;
pub const CXL_CM_CAP_HDR_VERSION_MASK: u32 = 0xf0000;
pub const CM_CAP_HDR_CAP_VERSION: u32 = 1;
pub const CXL_CM_CAP_HDR_CACHE_MEM_VERSION_MASK: u32 = 0xf00000;
pub const CM_CAP_HDR_CACHE_MEM_VERSION: u32 = 1;
pub const CXL_CM_CAP_HDR_ARRAY_SIZE_MASK: u32 = 0xff000000;
pub const CXL_CM_CAP_PTR_MASK: u32 = 0xfff00000;
pub const CXL_CM_CAP_CAP_ID_RAS: u32 = 2;
pub const CXL_CM_CAP_CAP_ID_HDM: u32 = 5;
pub const CXL_CM_CAP_HDM_VERSION: u32 = 1;

pub const CXL_HDM_DECODER_CAP_OFFSET: u32 = 0;
pub const CXL_HDM_DECODER_COUNT_MASK: u32 = 0xf;
pub const CXL_HDM_DECODER_TARGET_COUNT_MASK: u32 = 0xf0;
pub const CXL_HDM_DECODER_INTERLEAVE_11_8: u32 = 1 << 8;
pub const CXL_HDM_DECODER_INTERLEAVE_14_12: u32 = 1 << 9;
pub const CXL_HDM_DECODER_INTERLEAVE_3_6_12_WAY: u32 = 1 << 11;
pub const CXL_HDM_DECODER_INTERLEAVE_16_WAY: u32 = 1 << 12;
pub const CXL_HDM_DECODER_CTRL_OFFSET: u32 = 4;
pub const CXL_HDM_DECODER_ENABLE: u32 = 1 << 1;
pub const CXL_DECODER_MIN_GRANULARITY: i32 = 256;
pub const CXL_DECODER_MAX_ENCODED_IG: u16 = 6;

pub const CXL_DECODER_MAX_INTERLEAVE: usize = 16;
pub const CXL_QOS_CLASS_INVALID: i32 = -1;
pub const CXL_RESOURCE_NONE: u64 = u64::MAX;
pub const CXL_TARGET_STRLEN: usize = 20;
pub const CXL_DEV_ID_LEN: usize = 21;
pub const CXL_HEADERLOG_SIZE: usize = 64;
pub const CXL_HEADERLOG_SIZE_U32: usize = 16;
pub const CXL_HEADERLOG_TRACE_SIZE: usize = 512;
pub const CXL_HEADERLOG_TRACE_SIZE_U32: usize = 128;

pub const CXL_RAS_UNCORRECTABLE_STATUS_OFFSET: u32 = 0;
pub const CXL_RAS_UNCORRECTABLE_MASK_OFFSET: u32 = 4;
pub const CXL_RAS_UNCORRECTABLE_SEVERITY_OFFSET: u32 = 8;
pub const CXL_RAS_CORRECTABLE_STATUS_OFFSET: u32 = 0xc;
pub const CXL_RAS_CORRECTABLE_MASK_OFFSET: u32 = 0x10;
pub const CXL_RAS_CAP_CONTROL_OFFSET: u32 = 0x14;
pub const CXL_RAS_HEADER_LOG_OFFSET: u32 = 0x18;
pub const CXL_RAS_CAPABILITY_LENGTH: u32 = 0x58;

pub const CXLDEV_CAP_ARRAY_OFFSET: u32 = 0;
pub const CXLDEV_CAP_ARRAY_CAP_ID: u32 = 0;
pub const CXLDEV_CAP_ARRAY_ID_MASK: u64 = 0xffff;
pub const CXLDEV_CAP_ARRAY_COUNT_MASK: u64 = 0xffff_0000_0000;
pub const CXLDEV_CAP_HDR_CAP_ID_MASK: u32 = 0xffff;
pub const CXLDEV_CAP_CAP_ID_DEVICE_STATUS: u32 = 1;
pub const CXLDEV_CAP_CAP_ID_PRIMARY_MAILBOX: u32 = 2;
pub const CXLDEV_CAP_CAP_ID_SECONDARY_MAILBOX: u32 = 3;
pub const CXLDEV_CAP_CAP_ID_MEMDEV: u32 = 0x4000;

pub const CXL_DECODER_F_RAM: usize = 1 << 0;
pub const CXL_DECODER_F_PMEM: usize = 1 << 1;
pub const CXL_DECODER_F_TYPE2: usize = 1 << 2;
pub const CXL_DECODER_F_TYPE3: usize = 1 << 3;
pub const CXL_DECODER_F_LOCK: usize = 1 << 4;
pub const CXL_DECODER_F_ENABLE: usize = 1 << 5;
pub const CXL_DECODER_F_NORMALIZED_ADDRESSING: usize = 1 << 6;
pub const CXL_DECODER_F_RESET_MASK: usize = (1 << 5) | (1 << 4);

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct nvdimm_bus { _private: [u8; 0] }
#[repr(C)] pub struct nvdimm_bus_descriptor { _private: [u8; 0] }
#[repr(C)] pub struct nd_region { _private: [u8; 0] }
#[repr(C)] pub struct cxl_register_map { _private: [u8; 0] }
#[repr(C)] pub struct cxl_component_regs { _private: [u8; 0] }
#[repr(C)] pub struct cxl_device_regs { _private: [u8; 0] }
#[repr(C)] pub struct cxl_regs { _private: [u8; 0] }
#[repr(C)] pub struct access_coordinate { _private: [u8; 0] }
#[repr(C)] pub struct uuid_t { pub bytes: [u8; 16] }
#[repr(C)] pub struct atomic_t { pub value: i32 }
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }
#[repr(C)] pub struct pci_bus { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct cxl_memdev { _private: [u8; 0] }
#[repr(C)] pub struct cxl_dev_state { _private: [u8; 0] }
#[repr(C)] pub struct nvdimm_security_ops { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }

#[repr(C)] pub struct cxl_dport { pub dport_dev: *mut device, pub reg_map: cxl_register_map, pub port_id: i32, pub rcrb: cxl_rcrb_info, pub rch: bool, pub port: *mut cxl_port, pub regs: cxl_regs, pub coord: *mut access_coordinate, pub link_latency: i64, pub gpf_dvsec: i32 }
#[repr(C)] pub struct cxl_rcrb_info { pub base: u64, pub aer_cap: u16 }
#[repr(C)] pub struct cxl_decoder { pub dev: device, pub id: i32, pub hpa_range: range, pub interleave_ways: i32, pub interleave_granularity: i32, pub target_type: cxl_decoder_type, pub region: *mut cxl_region, pub flags: usize, pub target_map: [u32; CXL_DECODER_MAX_INTERLEAVE], pub commit: Option<unsafe extern "C" fn(*mut cxl_decoder) -> i32>, pub reset: Option<unsafe extern "C" fn(*mut cxl_decoder)> }
#[repr(C)] pub struct cxl_endpoint_decoder { pub cxld: cxl_decoder, pub dpa_res: *mut resource, pub skip: u64, pub state: cxl_decoder_state, pub part: i32, pub pos: i32 }
#[repr(C)] pub struct cxl_switch_decoder { pub cxld: cxl_decoder, pub nr_targets: i32, pub target: [*mut cxl_dport; 0] }
#[repr(C)] pub struct cxl_rd_ops { pub hpa_to_spa: Option<unsafe extern "C" fn(*mut cxl_root_decoder,u64)->u64>, pub spa_to_hpa: Option<unsafe extern "C" fn(*mut cxl_root_decoder,u64)->u64> }
#[repr(C)] pub struct cxl_root_decoder { pub res:*mut resource, pub cache_size:u64, pub region_id:atomic_t, pub platform_data:*mut c_void, pub regions_lock:mutex, pub regions:xarray, pub dead:bool, pub qos_class:i32, pub ops:cxl_rd_ops, pub cxlsd:cxl_switch_decoder }
#[repr(C)] pub struct cxl_region_params { pub state:cxl_config_state, pub uuid:uuid_t, pub interleave_ways:i32, pub interleave_granularity:i32, pub res:*mut resource, pub targets:[*mut cxl_endpoint_decoder;16], pub nr_targets:i32, pub cache_size:u64 }
#[repr(C)] pub struct cxl_region { pub dev:device, pub id:i32, pub cxlrd:*mut cxl_root_decoder, pub hpa_range:range, pub mode:i32, pub r#type:cxl_decoder_type, pub cxl_nvb:*mut cxl_nvdimm_bridge, pub cxlr_pmem:*mut cxl_pmem_region, pub flags:usize, pub params:cxl_region_params, pub coord:*mut access_coordinate, pub node_notifier:notifier_block, pub adist_notifier:notifier_block, pub mce_notifier:notifier_block }
#[repr(C)] pub struct cxl_nvdimm_bridge { pub id:i32, pub dev:device, pub port:*mut cxl_port, pub nvdimm_bus:*mut nvdimm_bus, pub nd_desc:nvdimm_bus_descriptor }
#[repr(C)] pub struct cxl_nvdimm { pub dev:device, pub cxlmd:*mut cxl_memdev, pub dev_id:[u8;21], pub dirty_shutdowns:u64, pub flags:usize }
#[repr(C)] pub struct cxl_pmem_region_mapping { pub cxlmd:*mut cxl_memdev, pub cxl_nvd:*mut cxl_nvdimm, pub start:u64, pub size:u64, pub position:i32 }
#[repr(C)] pub struct cxl_pmem_region { pub dev:device, pub cxlr:*mut cxl_region, pub nd_region:*mut nd_region, pub hpa_range:range, pub nr_mappings:i32, pub mapping:[cxl_pmem_region_mapping;0] }
#[repr(C)] pub struct cxl_dax_region { pub dev:device, pub cxlr:*mut cxl_region, pub hpa_range:range }
#[repr(C)] pub struct cxl_port { pub dev:device, pub uport_dev:*mut device, pub host_bridge:*mut device, pub id:i32, pub dports:xarray, pub endpoints:xarray, pub regions:xarray, pub parent_dport:*mut cxl_dport, pub decoder_ida:ida, pub reg_map:cxl_register_map, pub regs:cxl_component_regs, pub nr_dports:i32, pub hdm_end:i32, pub commit_end:i32, pub dead:bool, pub depth:u32, pub cdat:*mut c_void, pub cdat_available:bool, pub pci_latency:i64, pub component_reg_phys:u64 }
#[repr(C)] pub struct cxl_root { pub port:cxl_port, pub ops:cxl_root_ops }
#[repr(C)] pub struct cxl_root_ops { pub qos_class:Option<unsafe extern "C" fn(*mut cxl_root,*mut access_coordinate,i32,*mut i32)->i32>, pub translation_setup_root:Option<unsafe extern "C" fn(*mut cxl_root,*mut c_void)->i32> }
#[repr(C)] pub struct cxl_ep { pub ep:*mut device, pub dport:*mut cxl_dport, pub next:*mut cxl_port }
#[repr(C)] pub struct cxl_region_ref { pub port:*mut cxl_port, pub decoder:*mut cxl_decoder, pub region:*mut cxl_region, pub endpoints:xarray, pub nr_targets_set:i32, pub nr_eps:i32, pub nr_targets:i32 }
#[repr(C)] pub struct cxl_cxims_data { pub nr_maps:i32, pub xormaps:[u64;0] }
#[repr(C)] pub struct cxl_endpoint_dvsec_info { pub mem_enabled:bool, pub ranges:i32, pub port:*mut cxl_port, pub dvsec_range:[range;2] }
#[repr(C)] pub struct cxl_driver { pub name:*const i8, pub probe:Option<unsafe extern "C" fn(*mut device)->i32>, pub remove:Option<unsafe extern "C" fn(*mut device)>, pub add_dport:Option<unsafe extern "C" fn(*mut cxl_port,*mut device)->*mut cxl_dport>, pub drv:device_driver, pub id:i32 }

#[repr(C)] pub enum cxl_decoder_type { CXL_DECODER_DEVMEM=2, CXL_DECODER_HOSTONLYMEM=3 }
#[repr(C)] pub enum cxl_decoder_state { CXL_DECODER_STATE_MANUAL, CXL_DECODER_STATE_AUTO, CXL_DECODER_STATE_AUTO_STAGED }
#[repr(C)] pub enum cxl_config_state { CXL_CONFIG_IDLE, CXL_CONFIG_INTERLEAVE_ACTIVE, CXL_CONFIG_ACTIVE, CXL_CONFIG_RESET_PENDING, CXL_CONFIG_COMMIT }

pub const CXL_DEVICE_NVDIMM_BRIDGE:u32=1; pub const CXL_DEVICE_NVDIMM:u32=2; pub const CXL_DEVICE_PORT:u32=3; pub const CXL_DEVICE_ROOT:u32=4; pub const CXL_DEVICE_MEMORY_EXPANDER:u32=5; pub const CXL_DEVICE_REGION:u32=6; pub const CXL_DEVICE_PMEM_REGION:u32=7; pub const CXL_DEVICE_DAX_REGION:u32=8; pub const CXL_DEVICE_PMU:u32=9;
pub const CXL_INSTANCES_COUNT:i32=-1; pub const CXL_REGION_F_AUTO:u32=0; pub const CXL_REGION_F_NEEDS_RESET:u32=1; pub const CXL_REGION_F_LOCK:u32=2; pub const CXL_REGION_F_NORMALIZED_ADDRESSING:u32=3;
pub const CXLDEV_DEV_EVENT_STATUS_OFFSET:u32=0; pub const CXLDEV_EVENT_STATUS_INFO:u32=1; pub const CXLDEV_EVENT_STATUS_WARN:u32=2; pub const CXLDEV_EVENT_STATUS_FAIL:u32=4; pub const CXLDEV_EVENT_STATUS_FATAL:u32=8; pub const CXLDEV_EVENT_STATUS_ALL:u32=15;
pub const CXLDEV_EVENT_INT_MODE_MASK:u32=3; pub const CXLDEV_EVENT_INT_MSGNUM_MASK:u32=0xf0; pub const CXLDEV_MBOX_CAPS_OFFSET:u32=0; pub const CXLDEV_MBOX_CTRL_OFFSET:u32=4; pub const CXLDEV_MBOX_CMD_OFFSET:u32=8; pub const CXLDEV_MBOX_STATUS_OFFSET:u32=0x10; pub const CXLDEV_MBOX_BG_CMD_STATUS_OFFSET:u32=0x18; pub const CXLDEV_MBOX_PAYLOAD_OFFSET:u32=0x20;
pub const CXL_HDM_DECODER0_CTRL_IG_MASK:u32=15; pub const CXL_HDM_DECODER0_CTRL_IW_MASK:u32=240; pub const CXL_HDM_DECODER0_CTRL_LOCK:u32=1<<8; pub const CXL_HDM_DECODER0_CTRL_COMMIT:u32=1<<9; pub const CXL_HDM_DECODER0_CTRL_COMMITTED:u32=1<<10; pub const CXL_HDM_DECODER0_CTRL_COMMIT_ERROR:u32=1<<11; pub const CXL_HDM_DECODER0_CTRL_HOSTONLY:u32=1<<12;
pub const CXL_DECODER0_OFFSET_STRIDE:u32=0x20;
pub const CXL_HDM_DECODER0_BASE_LOW_OFFSET:unsafe fn(u32)->u32=|i|0x20*i+0x10;
pub const CXL_HDM_DECODER0_BASE_HIGH_OFFSET:unsafe fn(u32)->u32=|i|0x20*i+0x14;
pub const CXL_HDM_DECODER0_SIZE_LOW_OFFSET:unsafe fn(u32)->u32=|i|0x20*i+0x18;
pub const CXL_HDM_DECODER0_SIZE_HIGH_OFFSET:unsafe fn(u32)->u32=|i|0x20*i+0x1c;
pub const CXL_HDM_DECODER0_CTRL_OFFSET:unsafe fn(u32)->u32=|i|0x20*i+0x20;
pub const CXL_HDM_DECODER0_TL_LOW:unsafe fn(u32)->u32=|i|0x20*i+0x24;
pub const CXL_HDM_DECODER0_TL_HIGH:unsafe fn(u32)->u32=|i|0x20*i+0x28;

pub unsafe fn cxl_hdm_decoder_count(cap_hdr:u32)->i32 { let v=cap_hdr & 0xf; match v {0=>1,1..=8=>(v*2) as i32,9..=12=>((v-4)*4) as i32,_=>-6} }
pub unsafe fn eig_to_granularity(eig:u16, granularity:*mut i32)->i32 { if eig>6{return -22;} *granularity=256i32.wrapping_shl(eig as u32); 0 }
pub unsafe fn eiw_to_ways(eiw:u8, ways:*mut i32)->i32 { match eiw {0..=4=>*ways=1i32<<(eiw as i32),8..=10=>*ways=3i32<<((eiw-8) as i32),_=>(*ways=0;return -22)} 0 }
pub unsafe fn granularity_to_eig(g:i32,eig:*mut u16)->i32 { *eig=0; if g>16384||g<256||(g&(g-1))!=0{return -22;} *eig=(31-g.leading_zeros()-8) as u16; 0 }
pub unsafe fn ways_to_eiw(mut ways:u32,eiw:*mut u8)->i32 { *eiw=0;if ways>16{return -22;} if ways.is_power_of_two(){*eiw=ways.trailing_zeros() as u8;return 0;} if ways%3!=0{return -22;}ways/=3;if !ways.is_power_of_two(){return -22;}*eiw=ways.trailing_zeros() as u8+8;0 }

extern "C" {
    pub static cxl_security_ops:*const nvdimm_security_ops;
    pub fn cxl_validate_translation_params(eiw:u8,eig:u16,pos:i32)->i32;
    pub fn cxl_calculate_hpa_offset(dpa_offset:u64,pos:i32,eiw:u8,eig:u16)->u64;
    pub fn cxl_calculate_dpa_offset(hpa_offset:u64,eiw:u8,eig:u16)->u64;
    pub fn cxl_calculate_position(hpa_offset:u64,eiw:u8,eig:u16)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
