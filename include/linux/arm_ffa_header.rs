/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2021 ARM Ltd. */

// Kernel dependencies supplied by the surrounding translation unit.

pub const fn FFA_SMC(calling_convention: u32, func_num: u32) -> u32 { ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL, calling_convention, ARM_SMCCC_OWNER_STANDARD, func_num) }
pub const fn FFA_SMC_32(func_num: u32) -> u32 { FFA_SMC(ARM_SMCCC_SMC_32, func_num) }
pub const fn FFA_SMC_64(func_num: u32) -> u32 { FFA_SMC(ARM_SMCCC_SMC_64, func_num) }
pub const FFA_ERROR: u32 = FFA_SMC_32(0x60);
pub const FFA_SUCCESS: u32 = FFA_SMC_32(0x61);
pub const FFA_FN64_SUCCESS: u32 = FFA_SMC_64(0x61);
pub const FFA_INTERRUPT: u32 = FFA_SMC_32(0x62);
pub const FFA_VERSION: u32 = FFA_SMC_32(0x63);
pub const FFA_FEATURES: u32 = FFA_SMC_32(0x64);
pub const FFA_RX_RELEASE: u32 = FFA_SMC_32(0x65);
pub const FFA_RXTX_MAP: u32 = FFA_SMC_32(0x66);
pub const FFA_FN64_RXTX_MAP: u32 = FFA_SMC_64(0x66);
pub const FFA_RXTX_UNMAP: u32 = FFA_SMC_32(0x67);
pub const FFA_PARTITION_INFO_GET: u32 = FFA_SMC_32(0x68);
pub const FFA_ID_GET: u32 = FFA_SMC_32(0x69);
pub const FFA_MSG_POLL: u32 = FFA_SMC_32(0x6a);
pub const FFA_MSG_WAIT: u32 = FFA_SMC_32(0x6b);
pub const FFA_YIELD: u32 = FFA_SMC_32(0x6c);
pub const FFA_RUN: u32 = FFA_SMC_32(0x6d);
pub const FFA_MSG_SEND: u32 = FFA_SMC_32(0x6e);
pub const FFA_MSG_SEND_DIRECT_REQ: u32 = FFA_SMC_32(0x6f);
pub const FFA_FN64_MSG_SEND_DIRECT_REQ: u32 = FFA_SMC_64(0x6f);
pub const FFA_MSG_SEND_DIRECT_RESP: u32 = FFA_SMC_32(0x70);
pub const FFA_FN64_MSG_SEND_DIRECT_RESP: u32 = FFA_SMC_64(0x70);
pub const FFA_MEM_DONATE: u32 = FFA_SMC_32(0x71);
pub const FFA_FN64_MEM_DONATE: u32 = FFA_SMC_64(0x71);
pub const FFA_MEM_LEND: u32 = FFA_SMC_32(0x72);
pub const FFA_FN64_MEM_LEND: u32 = FFA_SMC_64(0x72);
pub const FFA_MEM_SHARE: u32 = FFA_SMC_32(0x73);
pub const FFA_FN64_MEM_SHARE: u32 = FFA_SMC_64(0x73);
pub const FFA_MEM_RETRIEVE_REQ: u32 = FFA_SMC_32(0x74);
pub const FFA_FN64_MEM_RETRIEVE_REQ: u32 = FFA_SMC_64(0x74);
pub const FFA_MEM_RETRIEVE_RESP: u32 = FFA_SMC_32(0x75);
pub const FFA_MEM_RELINQUISH: u32 = FFA_SMC_32(0x76);
pub const FFA_MEM_RECLAIM: u32 = FFA_SMC_32(0x77);
pub const FFA_MEM_OP_PAUSE: u32 = FFA_SMC_32(0x78);
pub const FFA_MEM_OP_RESUME: u32 = FFA_SMC_32(0x79);
pub const FFA_MEM_FRAG_RX: u32 = FFA_SMC_32(0x7a);
pub const FFA_MEM_FRAG_TX: u32 = FFA_SMC_32(0x7b);
pub const FFA_NORMAL_WORLD_RESUME: u32 = FFA_SMC_32(0x7c);
pub const FFA_NOTIFICATION_BITMAP_CREATE: u32 = FFA_SMC_32(0x7d);
pub const FFA_NOTIFICATION_BITMAP_DESTROY: u32 = FFA_SMC_32(0x7e);
pub const FFA_NOTIFICATION_BIND: u32 = FFA_SMC_32(0x7f);
pub const FFA_NOTIFICATION_UNBIND: u32 = FFA_SMC_32(0x80);
pub const FFA_NOTIFICATION_SET: u32 = FFA_SMC_32(0x81);
pub const FFA_NOTIFICATION_GET: u32 = FFA_SMC_32(0x82);
pub const FFA_NOTIFICATION_INFO_GET: u32 = FFA_SMC_32(0x83);
pub const FFA_FN64_NOTIFICATION_INFO_GET: u32 = FFA_SMC_64(0x83);
pub const FFA_RX_ACQUIRE: u32 = FFA_SMC_32(0x84);
pub const FFA_SPM_ID_GET: u32 = FFA_SMC_32(0x85);
pub const FFA_MSG_SEND2: u32 = FFA_SMC_32(0x86);
pub const FFA_SECONDARY_EP_REGISTER: u32 = FFA_SMC_32(0x87);
pub const FFA_FN64_SECONDARY_EP_REGISTER: u32 = FFA_SMC_64(0x87);
pub const FFA_MEM_PERM_GET: u32 = FFA_SMC_32(0x88);
pub const FFA_FN64_MEM_PERM_GET: u32 = FFA_SMC_64(0x88);
pub const FFA_MEM_PERM_SET: u32 = FFA_SMC_32(0x89);
pub const FFA_FN64_MEM_PERM_SET: u32 = FFA_SMC_64(0x89);
pub const FFA_CONSOLE_LOG: u32 = FFA_SMC_32(0x8a);
pub const FFA_PARTITION_INFO_GET_REGS: u32 = FFA_SMC_64(0x8b);
pub const FFA_EL3_INTR_HANDLE: u32 = FFA_SMC_32(0x8c);
pub const FFA_MSG_SEND_DIRECT_REQ2: u32 = FFA_SMC_64(0x8d);
pub const FFA_MSG_SEND_DIRECT_RESP2: u32 = FFA_SMC_64(0x8e);

pub const FFA_RET_SUCCESS: i32 = 0;
pub const FFA_RET_NOT_SUPPORTED: i32 = -1;
pub const FFA_RET_INVALID_PARAMETERS: i32 = -2;
pub const FFA_RET_NO_MEMORY: i32 = -3;
pub const FFA_RET_BUSY: i32 = -4;
pub const FFA_RET_INTERRUPTED: i32 = -5;
pub const FFA_RET_DENIED: i32 = -6;
pub const FFA_RET_RETRY: i32 = -7;
pub const FFA_RET_ABORTED: i32 = -8;
pub const FFA_RET_NO_DATA: i32 = -9;

pub const FFA_MAJOR_VERSION_MASK: u32 = GENMASK(30, 16);
pub const FFA_MINOR_VERSION_MASK: u32 = GENMASK(15, 0);
pub const fn FFA_MAJOR_VERSION(x: u32) -> u16 { FIELD_GET(FFA_MAJOR_VERSION_MASK, x) as u16 }
pub const fn FFA_MINOR_VERSION(x: u32) -> u16 { FIELD_GET(FFA_MINOR_VERSION_MASK, x) as u16 }
pub const fn FFA_PACK_VERSION_INFO(major: u32, minor: u32) -> u32 { FIELD_PREP(FFA_MAJOR_VERSION_MASK, major) | FIELD_PREP(FFA_MINOR_VERSION_MASK, minor) }
pub const FFA_VERSION_1_0: u32 = FFA_PACK_VERSION_INFO(1, 0);
pub const FFA_VERSION_1_1: u32 = FFA_PACK_VERSION_INFO(1, 1);
pub const FFA_VERSION_1_2: u32 = FFA_PACK_VERSION_INFO(1, 2);
pub const FFA_PAGE_SIZE: usize = SZ_4K;
pub const FFA_FEAT_RXTX_MIN_SZ_4K: u32 = 0;
pub const FFA_FEAT_RXTX_MIN_SZ_64K: u32 = 1;
pub const FFA_FEAT_RXTX_MIN_SZ_16K: u32 = 2;
pub const FFA_FEAT_RXTX_MIN_SZ_MASK: u32 = GENMASK(1, 0);

pub const FFA_MEM_RETRIEVE_ADDR_ALIGN: fn(u32) -> u32 = |x| x << 5;
pub const fn PACK_HANDLE(l: u32, h: u32) -> u64 { FIELD_PREP(HANDLE_LOW_MASK, l as u64) | FIELD_PREP(HANDLE_HIGH_MASK, h as u64) }
pub const fn HANDLE_LOW(x: u64) -> u32 { FIELD_GET(HANDLE_LOW_MASK, x) as u32 }
pub const fn HANDLE_HIGH(x: u64) -> u32 { FIELD_GET(HANDLE_HIGH_MASK, x) as u32 }

#[repr(C)]
pub struct ffa_device { pub id: u32, pub properties: u32, pub vm_id: i32, pub mode_32bit: bool, pub uuid: uuid_t, pub dev: device, pub ops: *const ffa_ops }
#[repr(C)] pub struct ffa_device_id { pub uuid: uuid_t }
#[repr(C)] pub struct ffa_driver { pub name: *const i8, pub probe: Option<unsafe extern "C" fn(*mut ffa_device) -> i32>, pub remove: Option<unsafe extern "C" fn(*mut ffa_device)>, pub id_table: *const ffa_device_id, pub driver: device_driver }
pub unsafe fn ffa_dev_set_drvdata(fdev: *mut ffa_device, data: *mut core::ffi::c_void) { dev_set_drvdata(&mut (*fdev).dev, data); }
pub unsafe fn ffa_dev_get_drvdata(fdev: *mut ffa_device) -> *mut core::ffi::c_void { dev_get_drvdata(&mut (*fdev).dev) }
pub unsafe extern "C" fn ffa_device_register(_part_info: *const ffa_partition_info, _ops: *const ffa_ops, _parent: *mut device) -> *mut ffa_device { core::ptr::null_mut() }
pub unsafe extern "C" fn ffa_device_unregister(_dev: *mut ffa_device) {}
pub unsafe extern "C" fn ffa_driver_register(_driver: *mut ffa_driver, _owner: *mut module, _mod_name: *const i8) -> i32 { -22 }
pub unsafe extern "C" fn ffa_driver_unregister(_driver: *mut ffa_driver) {}
pub unsafe extern "C" fn ffa_devices_unregister() {}
pub unsafe extern "C" fn ffa_device_is_valid(_dev: *mut ffa_device) -> bool { false }
pub static mut ffa_bus_type: bus_type = unsafe { core::mem::zeroed() };
pub const FFA_1_0_PARTITON_INFO_SZ: usize = 8;

pub const FFA_PARTITION_DIRECT_RECV: u32 = BIT(0);
pub const FFA_PARTITION_DIRECT_SEND: u32 = BIT(1);
pub const FFA_PARTITION_INDIRECT_MSG: u32 = BIT(2);
pub const FFA_PARTITION_NOTIFICATION_RECV: u32 = BIT(3);
pub const FFA_PARTITION_AARCH64_EXEC: u32 = BIT(8);
pub const FFA_PARTITION_DIRECT_REQ2_RECV: u32 = BIT(9);
pub const FFA_PARTITION_DIRECT_REQ2_SEND: u32 = BIT(10);

#[repr(C)] pub struct ffa_partition_info { pub id: u16, pub exec_ctxt: u16, pub properties: u32, pub uuid: uuid_t }
pub unsafe fn ffa_partition_check_property(dev: *const ffa_device, property: u32) -> bool { ((*dev).properties & property) != 0 }
pub unsafe fn ffa_partition_supports_notify_recv(dev: *const ffa_device) -> bool { ffa_partition_check_property(dev, FFA_PARTITION_NOTIFICATION_RECV) }
pub unsafe fn ffa_partition_supports_indirect_msg(dev: *const ffa_device) -> bool { ffa_partition_check_property(dev, FFA_PARTITION_INDIRECT_MSG) }
pub unsafe fn ffa_partition_supports_direct_recv(dev: *const ffa_device) -> bool { ffa_partition_check_property(dev, FFA_PARTITION_DIRECT_RECV) }
pub unsafe fn ffa_partition_supports_direct_req2_recv(dev: *const ffa_device) -> bool { ffa_partition_check_property(dev, FFA_PARTITION_DIRECT_REQ2_RECV) && !(*dev).mode_32bit }

#[repr(C)] pub struct ffa_send_direct_data { pub data0: usize, pub data1: usize, pub data2: usize, pub data3: usize, pub data4: usize }
#[repr(C)] pub struct ffa_indirect_msg_hdr { pub flags: u32, pub res0: u32, pub offset: u32, pub send_recv_id: u32, pub size: u32, pub res1: u32, pub uuid: uuid_t }
#[repr(C)] pub struct ffa_send_direct_data2 { pub data: [usize; 14] }
#[repr(C)] pub struct ffa_mem_region_addr_range { pub address: u64, pub pg_cnt: u32, pub reserved: u32 }
#[repr(C)] pub struct ffa_composite_mem_region { pub total_pg_cnt: u32, pub addr_range_cnt: u32, pub reserved: u64, pub constituents: [ffa_mem_region_addr_range; 0] }
#[repr(C)] pub struct ffa_mem_region_attributes { pub receiver: u16, pub attrs: u8, pub flag: u8, pub composite_off: u32, pub impdef_val: [u8; 16], pub reserved: u64 }
#[repr(C)] pub struct ffa_mem_region { pub sender_id: u16, pub attributes: u16, pub flags: u32, pub handle: u64, pub tag: u64, pub ep_mem_size: u32, pub ep_count: u32, pub ep_mem_offset: u32, pub reserved: [u32; 3] }

pub const FFA_MEM_EXEC: u8 = BIT(3) as u8; pub const FFA_MEM_NO_EXEC: u8 = BIT(2) as u8; pub const FFA_MEM_RW: u8 = BIT(1) as u8; pub const FFA_MEM_RO: u8 = BIT(0) as u8;
pub const FFA_MEM_RETRIEVE_SELF_BORROWER: u8 = BIT(0) as u8;
pub const FFA_MEM_NORMAL: u16 = BIT(5) as u16; pub const FFA_MEM_DEVICE: u16 = BIT(4) as u16;
pub const FFA_MEM_WRITE_BACK: u16 = 3 << 2; pub const FFA_MEM_NON_CACHEABLE: u16 = 1 << 2;
pub const FFA_DEV_nGnRnE: u16 = 0 << 2; pub const FFA_DEV_nGnRE: u16 = 1 << 2; pub const FFA_DEV_nGRE: u16 = 2 << 2; pub const FFA_DEV_GRE: u16 = 3 << 2;
pub const FFA_MEM_NON_SHAREABLE: u16 = 0; pub const FFA_MEM_OUTER_SHAREABLE: u16 = 2; pub const FFA_MEM_INNER_SHAREABLE: u16 = 3;
pub const FFA_MEM_CLEAR: u32 = BIT(0); pub const FFA_TIME_SLICE_ENABLE: u32 = BIT(1);
pub const FFA_MEM_RETRIEVE_TYPE_IN_RESP: u32 = 0 << 3; pub const FFA_MEM_RETRIEVE_TYPE_SHARE: u32 = 1 << 3; pub const FFA_MEM_RETRIEVE_TYPE_LEND: u32 = 2 << 3; pub const FFA_MEM_RETRIEVE_TYPE_DONATE: u32 = 3 << 3;
pub const FFA_MEM_RETRIEVE_ADDR_ALIGN_HINT: u32 = BIT(9);
pub const HANDLE_LOW_MASK: u64 = GENMASK_ULL(31, 0); pub const HANDLE_HIGH_MASK: u64 = GENMASK_ULL(63, 32);

pub const fn FFA_EMAD_HAS_IMPDEF_FIELD(version: u32) -> bool { version >= FFA_VERSION_1_2 }
pub const fn FFA_MEM_REGION_HAS_EP_MEM_OFFSET(version: u32) -> bool { version > FFA_VERSION_1_0 }
pub const fn FFA_MEM_REGION_SZ(_version: u32) -> usize { core::mem::size_of::<ffa_mem_region>() }
pub fn ffa_emad_size_get(ffa_version: u32) -> u32 { if FFA_EMAD_HAS_IMPDEF_FIELD(ffa_version) { core::mem::size_of::<ffa_mem_region_attributes>() as u32 } else { (core::mem::size_of::<ffa_mem_region_attributes>() - 16) as u32 } }
pub unsafe fn ffa_mem_desc_offset(buf: *const ffa_mem_region, count: i32, ffa_version: u32) -> u32 { let mut offset = count as u32 * ffa_emad_size_get(ffa_version); if !FFA_MEM_REGION_HAS_EP_MEM_OFFSET(ffa_version) { offset += 32; } else { offset += (*buf).ep_mem_offset; } offset }

#[repr(C)] pub struct ffa_mem_ops_args { pub use_txbuf: bool, pub nattrs: u32, pub flags: u32, pub tag: u64, pub g_handle: u64, pub sg: *mut scatterlist, pub attrs: *mut ffa_mem_region_attributes }
#[repr(C)] pub struct ffa_info_ops { pub api_version_get: Option<unsafe extern "C" fn() -> u32>, pub partition_info_get: Option<unsafe extern "C" fn(*const i8, *mut ffa_partition_info) -> i32> }
#[repr(C)] pub struct ffa_msg_ops { pub mode_32bit_set: Option<unsafe extern "C" fn(*mut ffa_device)>, pub sync_send_receive: Option<unsafe extern "C" fn(*mut ffa_device, *mut ffa_send_direct_data) -> i32>, pub indirect_send: Option<unsafe extern "C" fn(*mut ffa_device, *mut core::ffi::c_void, usize) -> i32>, pub sync_send_receive2: Option<unsafe extern "C" fn(*mut ffa_device, *mut ffa_send_direct_data2) -> i32> }
#[repr(C)] pub struct ffa_mem_ops { pub memory_reclaim: Option<unsafe extern "C" fn(u64, u32) -> i32>, pub memory_share: Option<unsafe extern "C" fn(*mut ffa_mem_ops_args) -> i32>, pub memory_lend: Option<unsafe extern "C" fn(*mut ffa_mem_ops_args) -> i32> }
#[repr(C)] pub struct ffa_cpu_ops { pub run: Option<unsafe extern "C" fn(*mut ffa_device, u16) -> i32> }
pub type ffa_sched_recv_cb = Option<unsafe extern "C" fn(u16, bool, *mut core::ffi::c_void)>;
pub type ffa_notifier_cb = Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void)>;
pub type ffa_fwk_notifier_cb = Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void, *mut core::ffi::c_void)>;
#[repr(C)] pub struct ffa_notifier_ops { pub sched_recv_cb_register: Option<unsafe extern "C" fn(*mut ffa_device, ffa_sched_recv_cb, *mut core::ffi::c_void) -> i32>, pub sched_recv_cb_unregister: Option<unsafe extern "C" fn(*mut ffa_device) -> i32>, pub notify_request: Option<unsafe extern "C" fn(*mut ffa_device, bool, ffa_notifier_cb, *mut core::ffi::c_void, i32) -> i32>, pub notify_relinquish: Option<unsafe extern "C" fn(*mut ffa_device, i32) -> i32>, pub fwk_notify_request: Option<unsafe extern "C" fn(*mut ffa_device, ffa_fwk_notifier_cb, *mut core::ffi::c_void, i32) -> i32>, pub fwk_notify_relinquish: Option<unsafe extern "C" fn(*mut ffa_device, i32) -> i32>, pub notify_send: Option<unsafe extern "C" fn(*mut ffa_device, i32, bool, u16) -> i32> }
#[repr(C)] pub struct ffa_ops { pub info_ops: *const ffa_info_ops, pub msg_ops: *const ffa_msg_ops, pub mem_ops: *const ffa_mem_ops, pub cpu_ops: *const ffa_cpu_ops, pub notifier_ops: *const ffa_notifier_ops }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
