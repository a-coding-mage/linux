/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of cxlmem.h. External kernel/project types and functions are supplied elsewhere. */

// C build-time configuration conditions are preserved as comments where applicable.

pub const CXLMDEV_STATUS_OFFSET: u32 = 0x0;
pub const CXLMDEV_DEV_FATAL: u32 = 1 << 0;
pub const CXLMDEV_FW_HALT: u32 = 1 << 1;
pub const CXLMDEV_STATUS_MEDIA_STATUS_MASK: u32 = 0b11 << 2;
pub const CXLMDEV_MS_NOT_READY: u32 = 0;
pub const CXLMDEV_MS_READY: u32 = 1;
pub const CXLMDEV_MS_ERROR: u32 = 2;
pub const CXLMDEV_MS_DISABLED: u32 = 3;
pub const CXLMDEV_MBOX_IF_READY: u32 = 1 << 4;
pub const CXLMDEV_RESET_NEEDED_MASK: u32 = 0b111 << 5;
pub const CXLMDEV_RESET_NEEDED_NOT: u32 = 0;
pub const CXLMDEV_RESET_NEEDED_COLD: u32 = 1;
pub const CXLMDEV_RESET_NEEDED_WARM: u32 = 2;
pub const CXLMDEV_RESET_NEEDED_HOT: u32 = 3;
pub const CXLMDEV_RESET_NEEDED_CXL: u32 = 4;
#[inline] pub const fn CXLMDEV_READY(status: u32) -> bool { ((status & CXLMDEV_STATUS_MEDIA_STATUS_MASK) >> 2) == CXLMDEV_MS_READY }
#[inline] pub const fn CXLMDEV_RESET_NEEDED(status: u32) -> bool { ((status & CXLMDEV_RESET_NEEDED_MASK) >> 5) != CXLMDEV_RESET_NEEDED_NOT }

#[repr(C)] pub struct cxl_memdev { pub dev: device, pub cdev: cdev, pub cxlds: *mut cxl_dev_state, pub detach_work: work_struct, pub cxl_nvb: *mut cxl_nvdimm_bridge, pub cxl_nvd: *mut cxl_nvdimm, pub endpoint: *mut cxl_port, pub attach: *const cxl_memdev_attach, pub id: i32, pub depth: i32, pub scrub_cycle: u8, pub scrub_region_id: i32, pub err_rec_array: *mut cxl_mem_err_rec }
#[inline] pub unsafe fn to_cxl_memdev(dev: *mut device) -> *mut cxl_memdev { container_of(dev, 0) }
#[inline] pub unsafe fn cxled_to_port(cxled: *mut cxl_endpoint_decoder) -> *mut cxl_port { to_cxl_port((*cxled).cxld.dev.parent) }
#[inline] pub unsafe fn cxlrd_to_port(cxlrd: *mut cxl_root_decoder) -> *mut cxl_port { to_cxl_port((*cxlrd).cxlsd.cxld.dev.parent) }
#[inline] pub unsafe fn cxled_to_memdev(cxled: *mut cxl_endpoint_decoder) -> *mut cxl_memdev { to_cxl_memdev((*cxled_to_port(cxled)).uport_dev) }
extern "C" { pub fn is_cxl_memdev(dev: *const device) -> bool; }
#[inline] pub unsafe fn is_cxl_endpoint(port: *mut cxl_port) -> bool { is_cxl_memdev((*port).uport_dev) }
#[repr(C)] pub struct cxl_memdev_attach { pub probe: Option<unsafe extern "C" fn(*mut cxl_memdev) -> i32> }
#[repr(C)] pub struct cxl_attach_region { pub attach: cxl_memdev_attach, pub hpa_range: range }

// CONFIG_CXL_REGION
extern "C" { pub fn cxl_memdev_attach_region(cxlmd: *mut cxl_memdev) -> i32; }
extern "C" { pub fn devm_cxl_add_classdev(cxlds: *mut cxl_dev_state) -> *mut cxl_memdev; pub fn __devm_cxl_add_memdev(cxlds: *mut cxl_dev_state, attach: *const cxl_memdev_attach) -> *mut cxl_memdev; pub fn devm_cxl_sanitize_setup_notifier(host: *mut device, cxlmd: *mut cxl_memdev) -> i32; pub fn devm_cxl_setup_fw_upload(host: *mut device, mds: *mut cxl_memdev_state) -> i32; pub fn devm_cxl_dpa_reserve(cxled: *mut cxl_endpoint_decoder, base: resource_size_t, len: resource_size_t, skipped: resource_size_t) -> i32; }
#[repr(C)] pub struct cxl_dpa_info { pub size: u64, pub part: [cxl_dpa_part_info; CXL_NR_PARTITIONS_MAX as usize], pub nr_partitions: i32 }
#[repr(C)] pub struct cxl_dpa_part_info { pub range: range, pub mode: cxl_partition_mode }
extern "C" { pub fn cxl_dpa_setup(cxlds: *mut cxl_dev_state, info: *const cxl_dpa_info) -> i32; }
#[inline] pub unsafe fn cxl_ep_load(port: *mut cxl_port, cxlmd: *mut cxl_memdev) -> *mut cxl_ep { if port.is_null() { core::ptr::null_mut() } else { xa_load(&mut (*port).endpoints, &(*cxlmd).dev as *const _ as usize) } }

pub const CXL_CAPACITY_MULTIPLIER: usize = 256 * 1024 * 1024;
#[repr(C, packed)] pub struct cxl_event_interrupt_policy { pub info_settings:u8, pub warn_settings:u8, pub failure_settings:u8, pub fatal_settings:u8 }
#[repr(C)] pub struct cxl_event_state { pub buf:*mut cxl_get_event_payload, pub log_lock:mutex }
pub const CXL_POISON_ENABLED_MAX: usize=6; pub const CXL_SEC_ENABLED_MAX: usize=8;
#[repr(C)] pub struct cxl_poison_state { pub max_errors:u32, pub enabled_cmds:[usize;1], pub list_out:*mut cxl_mbox_poison_out, pub mutex:mutex }
#[repr(C, packed)] pub struct cxl_mbox_get_fw_info { pub num_slots:u8,pub slot_info:u8,pub activation_cap:u8,pub reserved:[u8;13],pub slot_1_revision:[i8;16],pub slot_2_revision:[i8;16],pub slot_3_revision:[i8;16],pub slot_4_revision:[i8;16] }
pub const CXL_FW_INFO_SLOT_INFO_CUR_MASK:u8=0x7; pub const CXL_FW_INFO_SLOT_INFO_NEXT_MASK:u8=0x38; pub const CXL_FW_INFO_SLOT_INFO_NEXT_SHIFT:u8=3; pub const CXL_FW_INFO_ACTIVATION_CAP_HAS_LIVE_ACTIVATE:u8=1;
#[repr(C, packed)] pub struct cxl_mbox_transfer_fw { pub action:u8,pub slot:u8,pub reserved:[u8;2],pub offset:u32,pub reserved2:[u8;0x78],pub data:[u8;0] }
pub const CXL_FW_TRANSFER_ALIGNMENT:usize=128; pub const CXL_FW_ACTIVATE_ONLINE:u8=0; pub const CXL_FW_ACTIVATE_OFFLINE:u8=1; pub const CXL_FW_STATE_BITS:usize=32; pub const CXL_FW_CANCEL:usize=0;
#[repr(C, packed)] pub struct cxl_mbox_activate_fw { pub action:u8,pub slot:u8 }
#[repr(C)] pub struct cxl_fw_state { pub state:[usize;1],pub oneshot:bool,pub num_slots:i32,pub cur_slot:i32,pub next_slot:i32 }
#[repr(C)] pub struct cxl_security_state { pub state:usize,pub enabled_cmds:[usize;1],pub poll_tmo_secs:i32,pub sanitize_active:bool,pub poll_dwork:delayed_work,pub sanitize_node:*mut kernfs_node }

#[repr(C)] pub struct cxl_memdev_state { pub cxlds:cxl_dev_state,pub lsa_size:usize,pub firmware_version:[i8;0x10],pub total_bytes:u64,pub volatile_only_bytes:u64,pub persistent_only_bytes:u64,pub partition_align_bytes:u64,pub active_volatile_bytes:u64,pub active_persistent_bytes:u64,pub event:cxl_event_state,pub poison:cxl_poison_state,pub security:cxl_security_state,pub fw:cxl_fw_state }
#[inline] pub unsafe fn to_cxl_memdev_state(cxlds:*mut cxl_dev_state)->*mut cxl_memdev_state { if (*cxlds).type_ != CXL_DEVTYPE_CLASSMEM { core::ptr::null_mut() } else { container_of(cxlds,0) } }
#[repr(C)] pub struct cxl_hdm { pub regs:cxl_component_regs,pub decoder_count:i32,pub target_count:u32,pub interleave_mask:u32,pub iw_cap_mask:usize,pub port:*mut cxl_port }

// The remaining mailbox payloads and command interfaces retain C ABI/layout through declarations.
extern "C" { pub fn cxl_internal_send_cmd(m:*mut cxl_mailbox,c:*mut cxl_mbox_cmd)->i32; pub fn cxl_dev_state_identify(m:*mut cxl_memdev_state)->i32; pub fn cxl_await_media_ready(c:*mut cxl_dev_state)->i32; pub fn cxl_enumerate_cmds(m:*mut cxl_memdev_state)->i32; pub fn cxl_mem_sanitize(m:*mut cxl_memdev,c:u16)->i32; pub fn cxl_debugfs_create_dir(d:*const i8)->*mut dentry; }

// External declarations/types referenced by the original header.
pub type resource_size_t = usize; pub enum cxl_partition_mode {} pub enum cxl_opcode {} pub enum cxl_event_log_type {} pub enum cxl_event_type {}
#[repr(C)] pub struct device{pub parent:*mut device} #[repr(C)] pub struct cdev; #[repr(C)] pub struct work_struct; #[repr(C)] pub struct cxl_dev_state{pub type_:u32,pub dev:device} #[repr(C)] pub struct cxl_nvdimm_bridge; #[repr(C)] pub struct cxl_nvdimm; #[repr(C)] pub struct cxl_port{pub uport_dev:*mut device,pub endpoints:usize} #[repr(C)] pub struct cxl_endpoint_decoder{pub cxld:cxl_decoder} #[repr(C)] pub struct cxl_root_decoder{pub cxlsd:cxl_switch_decoder} #[repr(C)] pub struct cxl_decoder{pub dev:device} #[repr(C)] pub struct cxl_switch_decoder{pub cxld:cxl_decoder} #[repr(C)] pub struct cxl_ep; #[repr(C)] pub struct cxl_mem_err_rec; #[repr(C)] pub struct range; #[repr(C)] pub struct mutex; #[repr(C)] pub struct delayed_work; #[repr(C)] pub struct kernfs_node; #[repr(C)] pub struct cxl_component_regs; #[repr(C)] pub struct cxl_mailbox{pub host:*mut device} #[repr(C)] pub struct cxl_mbox_cmd; #[repr(C)] pub struct cxl_mbox_poison_out; #[repr(C)] pub struct dentry;
pub const CXL_NR_PARTITIONS_MAX:u32=2; pub const CXL_DEVTYPE_CLASSMEM:u32=3;
extern "C" { fn container_of<T,U>(p:*mut T, n:usize)->*mut U; fn to_cxl_port(p:*mut device)->*mut cxl_port; fn xa_load(x:*mut usize,k:usize)->*mut cxl_ep; }

#[repr(C, packed)] pub struct cxl_mbox_get_partition_info{pub active_volatile_cap:u64,pub active_persistent_cap:u64,pub next_volatile_cap:u64,pub next_persistent_cap:u64}
#[repr(C, packed)] pub struct cxl_mbox_get_lsa{pub offset:u32,pub length:u32}
#[repr(C, packed)] pub struct cxl_mbox_set_lsa{pub offset:u32,pub reserved:u32,pub data:[u8;0]}
#[repr(C, packed)] pub struct cxl_mbox_set_partition_info{pub volatile_capacity:u64,pub flags:u8}
pub const CXL_SET_PARTITION_IMMEDIATE_FLAG:u8=1;
#[repr(C, packed)] pub struct cxl_mbox_get_health_info_out{pub health_status:u8,pub media_status:u8,pub additional_status:u8,pub life_used:u8,pub device_temperature:u16,pub dirty_shutdown_cnt:u32,pub corrected_volatile_error_cnt:u32,pub corrected_persistent_error_cnt:u32}
#[repr(C, packed)] pub struct cxl_mbox_set_shutdown_state_in{pub state:u8}
#[repr(C, packed)] pub struct cxl_mbox_set_timestamp_in{pub timestamp:u64}
#[repr(C, packed)] pub struct cxl_mbox_poison_in{pub offset:u64,pub length:u64}
#[repr(C, packed)] pub struct cxl_mbox_inject_poison{pub address:u64}
#[repr(C, packed)] pub struct cxl_mbox_clear_poison{pub address:u64,pub write_data:[u8;64]}
pub const CXL_POISON_START_MASK:u64=(!0u64)<<6; pub const CXL_POISON_SOURCE_MASK:u64=7; pub const CXL_POISON_LEN_MULT:usize=64; pub const CXL_POISON_LIST_MAX:usize=1024;
pub const CXL_PMEM_SEC_STATE_USER_PASS_SET:u8=1; pub const CXL_PMEM_SEC_STATE_MASTER_PASS_SET:u8=2; pub const CXL_PMEM_SEC_STATE_LOCKED:u8=4; pub const CXL_PMEM_SEC_STATE_FROZEN:u8=8; pub const CXL_PMEM_SEC_STATE_USER_PLIMIT:u8=16; pub const CXL_PMEM_SEC_STATE_MASTER_PLIMIT:u8=32;
#[repr(C, packed)] pub struct cxl_set_pass{pub type_:u8,pub reserved:[u8;31],pub old_pass:[u8;32],pub new_pass:[u8;32]}
#[repr(C, packed)] pub struct cxl_disable_pass{pub type_:u8,pub reserved:[u8;31],pub pass:[u8;32]}
#[repr(C, packed)] pub struct cxl_pass_erase{pub type_:u8,pub reserved:[u8;31],pub pass:[u8;32]}
pub const CXL_PMEM_SEC_PASS_MASTER:u32=0; pub const CXL_PMEM_SEC_PASS_USER:u32=1;
#[repr(C)] pub struct cxl_mem_command{pub info:cxl_command_info,pub opcode:u32,pub flags:u32}
#[repr(C)] pub struct cxl_command_info;
pub const CXL_CMD_FLAG_FORCE_ENABLE:u32=1;
#[repr(C)] pub struct cxl_get_event_payload{pub flags:u8,pub reserved1:u8,pub overflow_err_count:u16,pub first_overflow_timestamp:u64,pub last_overflow_timestamp:u64,pub record_count:u16,pub reserved2:[u8;10],pub records:[u8;0]}
pub const CXL_GET_EVENT_FLAG_OVERFLOW:u8=1; pub const CXL_GET_EVENT_FLAG_MORE_RECORDS:u8=2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
