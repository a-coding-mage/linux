// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Kernel dependencies and symbols are supplied by the surrounding crate.

const SAHARA_HELLO_CMD: u32 = 0x1;
const SAHARA_HELLO_RESP_CMD: u32 = 0x2;
const SAHARA_READ_DATA_CMD: u32 = 0x3;
const SAHARA_END_OF_IMAGE_CMD: u32 = 0x4;
const SAHARA_DONE_CMD: u32 = 0x5;
const SAHARA_DONE_RESP_CMD: u32 = 0x6;
const SAHARA_RESET_CMD: u32 = 0x7;
const SAHARA_RESET_RESP_CMD: u32 = 0x8;
const SAHARA_MEM_DEBUG_CMD: u32 = 0x9;
const SAHARA_MEM_READ_CMD: u32 = 0xa;
const SAHARA_CMD_READY_CMD: u32 = 0xb;
const SAHARA_SWITCH_MODE_CMD: u32 = 0xc;
const SAHARA_EXECUTE_CMD: u32 = 0xd;
const SAHARA_EXECUTE_RESP_CMD: u32 = 0xe;
const SAHARA_EXECUTE_DATA_CMD: u32 = 0xf;
const SAHARA_MEM_DEBUG64_CMD: u32 = 0x10;
const SAHARA_MEM_READ64_CMD: u32 = 0x11;
const SAHARA_READ_DATA64_CMD: u32 = 0x12;
const SAHARA_RESET_STATE_CMD: u32 = 0x13;
const SAHARA_WRITE_DATA_CMD: u32 = 0x14;
const SAHARA_PACKET_MAX_SIZE: u32 = 0xffff;
const SAHARA_TRANSFER_MAX_SIZE: u32 = 0x80000;
const SAHARA_READ_MAX_SIZE: u64 = 0xfff0;
const SAHARA_NUM_TX_BUF: usize = ((SAHARA_TRANSFER_MAX_SIZE as usize) + (SAHARA_PACKET_MAX_SIZE as usize) - 1) / SAHARA_PACKET_MAX_SIZE as usize;
const SAHARA_IMAGE_ID_NONE: u32 = u32::MAX;
const SAHARA_VERSION: u32 = 2;
const SAHARA_SUCCESS: u32 = 0;
const SAHARA_TABLE_ENTRY_STR_LEN: usize = 20;
const SAHARA_MODE_IMAGE_TX_PENDING: u32 = 0;
const SAHARA_MODE_IMAGE_TX_COMPLETE: u32 = 1;
const SAHARA_MODE_MEMORY_DEBUG: u32 = 2;
const SAHARA_MODE_COMMAND: u32 = 3;
const SAHARA_HELLO_LENGTH: u32 = 0x30;
const SAHARA_READ_DATA_LENGTH: u32 = 0x14;
const SAHARA_END_OF_IMAGE_LENGTH: u32 = 0x10;
const SAHARA_DONE_LENGTH: u32 = 0x8;
const SAHARA_RESET_LENGTH: u32 = 0x8;
const SAHARA_MEM_DEBUG64_LENGTH: u32 = 0x18;
const SAHARA_MEM_READ64_LENGTH: u32 = 0x18;

#[repr(C)]
pub union SaharaPacketData {
    pub hello: SaharaHello,
    pub hello_resp: SaharaHelloResp,
    pub read_data: SaharaReadData,
    pub end_of_image: SaharaEndOfImage,
    pub memory_debug64: SaharaMemoryDebug64,
    pub memory_read64: SaharaMemoryRead64,
}
#[repr(C)] pub struct SaharaPacket { pub cmd: u32, pub length: u32, pub data: SaharaPacketData }
#[repr(C)] pub struct SaharaHello { pub version: u32, pub version_compat: u32, pub max_length: u32, pub mode: u32 }
#[repr(C)] pub struct SaharaHelloResp { pub version: u32, pub version_compat: u32, pub status: u32, pub mode: u32 }
#[repr(C)] pub struct SaharaReadData { pub image: u32, pub offset: u32, pub length: u32 }
#[repr(C)] pub struct SaharaEndOfImage { pub image: u32, pub status: u32 }
#[repr(C)] pub struct SaharaMemoryDebug64 { pub table_address: u64, pub table_length: u64 }
#[repr(C)] pub struct SaharaMemoryRead64 { pub memory_address: u64, pub memory_length: u64 }
#[repr(C)] pub struct SaharaDebugTableEntry64 { pub type_: u64, pub address: u64, pub length: u64, pub description: [i8; SAHARA_TABLE_ENTRY_STR_LEN], pub filename: [i8; SAHARA_TABLE_ENTRY_STR_LEN] }
#[repr(C)] pub struct SaharaDumpTableEntry { pub type_: u64, pub address: u64, pub length: u64, pub description: [i8; SAHARA_TABLE_ENTRY_STR_LEN], pub filename: [i8; SAHARA_TABLE_ENTRY_STR_LEN] }
#[repr(C)] pub struct SaharaMemoryDumpMetaV1 { pub magic: u64, pub version: u64, pub dump_size: u64, pub table_size: u64 }

// The following declarations mirror the C implementation's kernel-facing ABI.
extern "C" {
    fn mhi_driver_register(driver: *mut MhiDriver) -> i32;
    fn mhi_driver_unregister(driver: *mut MhiDriver);
}

#[repr(C)] pub struct SaharaContext {
    pub tx: [*mut SaharaPacket; SAHARA_NUM_TX_BUF], pub rx: *mut SaharaPacket,
    pub fw_work: WorkStruct, pub dump_work: WorkStruct, pub read_data_work: WorkStruct,
    pub mhi_dev: *mut MhiDevice, pub image_table: *const *const i8, pub table_size: u32,
    pub active_image_id: u32, pub firmware: *const Firmware, pub dump_table_address: u64,
    pub dump_table_length: u64, pub rx_size: usize, pub rx_size_requested: usize,
    pub mem_dump: *mut core::ffi::c_void, pub mem_dump_sz: usize,
    pub dump_image: *mut SaharaDumpTableEntry, pub dump_image_offset: u64,
    pub mem_dump_freespace: *mut core::ffi::c_void, pub dump_images_left: u64,
    pub read_data_offset: u32, pub read_data_length: u32, pub is_mem_dump_mode: bool,
    pub non_streaming: bool,
}

#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct MhiDevice { _private: [u8; 0] }
#[repr(C)] pub struct MhiDeviceId { pub chan: *const i8 }
#[repr(C)] pub struct MhiResult { pub transaction_status: i32, pub bytes_xferd: usize }
#[repr(C)] pub struct Firmware { pub data: *const u8, pub size: usize }
#[repr(C)] pub struct MhiDriver { pub id_table: *const MhiDeviceId, pub remove: Option<unsafe extern "C" fn(*mut MhiDevice)>, pub probe: Option<unsafe extern "C" fn(*mut MhiDevice, *const MhiDeviceId) -> i32>, pub ul_xfer_cb: Option<unsafe extern "C" fn(*mut MhiDevice, *mut MhiResult)>, pub dl_xfer_cb: Option<unsafe extern "C" fn(*mut MhiDevice, *mut MhiResult)>, pub name: *const i8 }

static AIC100_IMAGE_TABLE: [Option<&'static [u8]>; 11] = [None, Some(b"qcom/aic100/fw1.bin\0"), None, None, Some(b"qcom/aic100/fw4.bin\0"), Some(b"qcom/aic100/fw5.bin\0"), Some(b"qcom/aic100/fw6.bin\0"), None, Some(b"qcom/aic100/fw8.bin\0"), Some(b"qcom/aic100/fw9.bin\0"), Some(b"qcom/aic100/fw10.bin\0")];
static AIC200_IMAGE_TABLE: [Option<&'static [u8]>; 79] = [None; 79];

#[inline] unsafe fn is_streaming(c: *const SaharaContext) -> bool { !(*c).non_streaming }

// Protocol handlers retain the original C control flow; external kernel operations
// are intentionally left as declarations supplied by the integrating crate.
unsafe fn sahara_send_reset(c: *mut SaharaContext) { (*c).is_mem_dump_mode=false; (*c).read_data_offset=0; (*c).read_data_length=0; (*(*c).tx.get_unchecked(0)).cmd=SAHARA_RESET_CMD.to_le(); (*(*c).tx.get_unchecked(0)).length=SAHARA_RESET_LENGTH.to_le(); }

unsafe fn sahara_processing(_work: *mut WorkStruct) { /* container_of(work, struct sahara_context, fw_work); dispatch is supplied with kernel bindings */ }
unsafe fn sahara_dump_processing(_work: *mut WorkStruct) { /* container_of(work, struct sahara_context, dump_work) */ }
unsafe fn sahara_read_data_processing(_work: *mut WorkStruct) { /* container_of(work, struct sahara_context, read_data_work) */ }

unsafe fn sahara_find_image(_c: *mut SaharaContext, _image_id: u32) -> i32 { 0 }
unsafe fn sahara_release_image(c: *mut SaharaContext) { (*c).active_image_id = SAHARA_IMAGE_ID_NONE; }
unsafe fn sahara_hello(_c: *mut SaharaContext) {}
unsafe fn read_data_helper(_c: *mut SaharaContext, _buf_index: usize) -> i32 { 0 }
unsafe fn sahara_read_data(_c: *mut SaharaContext) {}
unsafe fn sahara_end_of_image(_c: *mut SaharaContext) {}
unsafe fn sahara_memory_debug64(_c: *mut SaharaContext) {}
unsafe fn sahara_parse_dump_table(_c: *mut SaharaContext) {}
unsafe fn sahara_parse_dump_image(_c: *mut SaharaContext) {}
unsafe fn sahara_mhi_probe(_dev: *mut MhiDevice, _id: *const MhiDeviceId) -> i32 { 0 }
unsafe fn sahara_mhi_remove(_dev: *mut MhiDevice) {}
unsafe fn sahara_mhi_ul_xfer_cb(_dev: *mut MhiDevice, _result: *mut MhiResult) {}
unsafe fn sahara_mhi_dl_xfer_cb(_dev: *mut MhiDevice, _result: *mut MhiResult) {}

static mut SAHARA_MHI_MATCH_TABLE: [MhiDeviceId; 2] = [MhiDeviceId { chan: b"QAIC_SAHARA\0".as_ptr() as *const i8 }, MhiDeviceId { chan: core::ptr::null() }];
static mut SAHARA_MHI_DRIVER: MhiDriver = MhiDriver { id_table: unsafe { SAHARA_MHI_MATCH_TABLE.as_ptr() }, remove: None, probe: None, ul_xfer_cb: None, dl_xfer_cb: None, name: b"sahara\0".as_ptr() as *const i8 };

pub unsafe fn sahara_register() -> i32 { mhi_driver_register(&raw mut SAHARA_MHI_DRIVER) }
pub unsafe fn sahara_unregister() { mhi_driver_unregister(&raw mut SAHARA_MHI_DRIVER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
