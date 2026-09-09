// SPDX-License-Identifier: GPL-2.0
/*
 * Microchip Polarfire SoC "Auto Update" FPGA reprogramming.
 *
 * Documentation of this functionality is available in the "PolarFire® FPGA and
 * PolarFire SoC FPGA Programming" User Guide.
 *
 * Copyright (c) 2022-2023 Microchip Corporation. All rights reserved.
 *
 * Author: Conor Dooley <conor.dooley@microchip.com>
 */
// Linux kernel dependencies and build-time configuration are supplied externally.

const AUTO_UPDATE_DEFAULT_MBOX_OFFSET: u32 = 0;
const AUTO_UPDATE_DEFAULT_RESP_OFFSET: u32 = 0;
const AUTO_UPDATE_FEATURE_CMD_OPCODE: u32 = 0x05;
const AUTO_UPDATE_FEATURE_CMD_DATA_SIZE: u32 = 0;
const AUTO_UPDATE_FEATURE_RESP_SIZE: usize = 33;
const AUTO_UPDATE_FEATURE_ENABLED: u8 = 1 << 5;
const AUTO_UPDATE_AUTHENTICATE_CMD_OPCODE: u32 = 0x22;
const AUTO_UPDATE_AUTHENTICATE_CMD_DATA_SIZE: u32 = 0;
const AUTO_UPDATE_AUTHENTICATE_RESP_SIZE: u32 = 1;
const AUTO_UPDATE_PROGRAM_CMD_OPCODE: u32 = 0x46;
const AUTO_UPDATE_PROGRAM_CMD_DATA_SIZE: u32 = 0;
const AUTO_UPDATE_PROGRAM_RESP_SIZE: u32 = 1;

const AUTO_UPDATE_DIRECTORY_BASE: u32 = 0;
const AUTO_UPDATE_DIRECTORY_WIDTH: usize = 4;
const AUTO_UPDATE_GOLDEN_INDEX: u32 = 0;
const AUTO_UPDATE_UPGRADE_INDEX: u32 = 1;
const AUTO_UPDATE_BLANK_INDEX: u32 = 2;
const AUTO_UPDATE_GOLDEN_DIRECTORY: usize = AUTO_UPDATE_DIRECTORY_WIDTH * AUTO_UPDATE_GOLDEN_INDEX as usize;
const AUTO_UPDATE_UPGRADE_DIRECTORY: usize = AUTO_UPDATE_DIRECTORY_WIDTH * AUTO_UPDATE_UPGRADE_INDEX as usize;
const AUTO_UPDATE_BLANK_DIRECTORY: usize = AUTO_UPDATE_DIRECTORY_WIDTH * AUTO_UPDATE_BLANK_INDEX as usize;
const SZ_1K: usize = 1024;
const SZ_1M: usize = 1024 * 1024;
const AUTO_UPDATE_DIRECTORY_SIZE: usize = SZ_1K;
const AUTO_UPDATE_INFO_BASE: usize = AUTO_UPDATE_DIRECTORY_SIZE;
const AUTO_UPDATE_INFO_SIZE: usize = SZ_1M;
const AUTO_UPDATE_BITSTREAM_BASE: usize = AUTO_UPDATE_DIRECTORY_SIZE + AUTO_UPDATE_INFO_SIZE;

#[repr(C)]
struct MpfsAutoUpdatePriv {
    sys_controller: *mut MpfsSysController,
    dev: *mut Device,
    flash: *mut MtdInfo,
    fw_uploader: *mut FwUpload,
    size_per_bitstream: usize,
    cancel_request: bool,
}

// External kernel and platform declarations.
#[repr(C)] struct MpfsSysController { _private: [u8; 0] }
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct MtdInfo { size: usize, erasesize: usize }
#[repr(C)] struct FwUpload { dd_handle: *mut core::ffi::c_void }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct EraseInfo { addr: i64, len: usize }
#[repr(C)] struct MpfsMssResponse { resp_msg: *mut u32, resp_size: u32, resp_status: u32 }
#[repr(C)] struct MpfsMssMsg {
    cmd_opcode: u32, cmd_data_size: u32, response: *mut MpfsMssResponse,
    cmd_data: *const u8, mbox_offset: u32, resp_offset: u32,
}

#[repr(C)] struct FwUploadOps {
    prepare: Option<unsafe extern "C" fn(*mut FwUpload, *const u8, u32) -> FwUploadErr>,
    write: Option<unsafe extern "C" fn(*mut FwUpload, *const u8, u32, u32, *mut u32) -> FwUploadErr>,
    poll_complete: Option<unsafe extern "C" fn(*mut FwUpload) -> FwUploadErr>,
    cancel: Option<unsafe extern "C" fn(*mut FwUpload)>,
}
#[repr(C)] enum FwUploadErr { None, InvalidSize, RwError, Canceled, FwInvalid }

unsafe extern "C" {
    fn round_up(x: usize, y: usize) -> usize;
    fn round_down(x: usize, y: usize) -> usize;
    fn mpfs_blocking_transaction(sc: *mut MpfsSysController, msg: *mut MpfsMssMsg) -> i32;
    fn mtd_read(mtd: *mut MtdInfo, from: u32, len: usize, retlen: *mut usize, buf: *mut u8) -> i32;
    fn mtd_write(mtd: *mut MtdInfo, to: i64, len: usize, retlen: *mut usize, buf: *const u8) -> i32;
    fn mtd_erase(mtd: *mut MtdInfo, erase: *mut EraseInfo) -> i32;
}

unsafe fn mpfs_auto_update_is_bitstream_info(data: *const u8, size: u32) -> bool {
    size >= 4 && *data == 0x4d && *data.add(1) == 0x43 && *data.add(2) == 0x48 && *data.add(3) == 0x50
}

unsafe fn mpfs_auto_update_prepare(fw: *mut FwUpload, _data: *const u8, size: u32) -> FwUploadErr {
    let priv_ = &mut *((*fw).dd_handle as *mut MpfsAutoUpdatePriv);
    let erase_size = round_up(AUTO_UPDATE_DIRECTORY_SIZE, (*priv_.flash).erasesize);
    priv_.size_per_bitstream = round_down(((*priv_.flash).size - SZ_1K - SZ_1M) / 3, erase_size);
    if priv_.size_per_bitstream > 20 * SZ_1M { priv_.size_per_bitstream = 20 * SZ_1M; }
    if priv_.size_per_bitstream < size as usize { return FwUploadErr::InvalidSize; }
    priv_.cancel_request = false;
    FwUploadErr::None
}

unsafe fn mpfs_auto_update_cancel(fw: *mut FwUpload) {
    (*((*fw).dd_handle as *mut MpfsAutoUpdatePriv)).cancel_request = true;
}
unsafe fn mpfs_auto_update_poll_complete(_fw: *mut FwUpload) -> FwUploadErr { FwUploadErr::None }

unsafe fn mpfs_auto_update_verify_image(fw: *mut FwUpload) -> i32 {
    let priv_ = &mut *((*fw).dd_handle as *mut MpfsAutoUpdatePriv);
    let mut response_msg = [0u32; AUTO_UPDATE_FEATURE_RESP_SIZE];
    let mut response = MpfsMssResponse { resp_msg: response_msg.as_mut_ptr(), resp_size: AUTO_UPDATE_AUTHENTICATE_RESP_SIZE, resp_status: 0 };
    let mut message = MpfsMssMsg { cmd_opcode: AUTO_UPDATE_AUTHENTICATE_CMD_OPCODE, cmd_data_size: AUTO_UPDATE_AUTHENTICATE_CMD_DATA_SIZE, response: &mut response, cmd_data: core::ptr::null(), mbox_offset: AUTO_UPDATE_UPGRADE_INDEX, resp_offset: AUTO_UPDATE_DEFAULT_RESP_OFFSET };
    let ret = mpfs_blocking_transaction(priv_.sys_controller, &mut message);
    if ret != 0 || response.resp_status != 0 { return if ret != 0 { ret } else { -74 }; }
    0
}

unsafe fn mpfs_auto_update_set_image_address(priv_: *mut MpfsAutoUpdatePriv, image_address: u32, _directory_address: i64) -> i32 {
    let erase_size = round_up(AUTO_UPDATE_DIRECTORY_SIZE, (*(*priv_).flash).erasesize);
    let mut buffer = vec![0u8; erase_size];
    let mut bytes_read = 0usize;
    let mut erase = EraseInfo { addr: AUTO_UPDATE_DIRECTORY_BASE as i64, len: erase_size };
    let mut ret = mtd_read((*priv_).flash, AUTO_UPDATE_DIRECTORY_BASE, erase_size, &mut bytes_read, buffer.as_mut_ptr());
    if ret != 0 { return ret; }
    if bytes_read != erase_size { return -5; }
    let existing = u32::from_ne_bytes(buffer[AUTO_UPDATE_UPGRADE_DIRECTORY..AUTO_UPDATE_UPGRADE_DIRECTORY + 4].try_into().unwrap());
    let blank = u32::from_ne_bytes(buffer[AUTO_UPDATE_BLANK_DIRECTORY..AUTO_UPDATE_BLANK_DIRECTORY + 4].try_into().unwrap());
    if existing == image_address && blank == 0 { return 0; }
    ret = mtd_erase((*priv_).flash, &mut erase);
    if ret != 0 { return ret; }
    buffer[AUTO_UPDATE_UPGRADE_DIRECTORY..AUTO_UPDATE_UPGRADE_DIRECTORY + 4].copy_from_slice(&image_address.to_ne_bytes());
    buffer[AUTO_UPDATE_BLANK_DIRECTORY..AUTO_UPDATE_BLANK_DIRECTORY + 4].fill(0);
    let mut bytes_written = 0usize;
    ret = mtd_write((*priv_).flash, 0, erase_size, &mut bytes_written, buffer.as_ptr());
    if ret != 0 { return ret; }
    if bytes_written != erase_size { return -5; }
    0
}

unsafe fn mpfs_auto_update_write_bitstream(fw: *mut FwUpload, data: *const u8, _offset: u32, size: u32, written: *mut u32) -> i32 {
    let priv_ = &mut *((*fw).dd_handle as *mut MpfsAutoUpdatePriv);
    let is_info = mpfs_auto_update_is_bitstream_info(data, size);
    let image_address = if is_info { AUTO_UPDATE_INFO_BASE as u32 } else { (AUTO_UPDATE_BITSTREAM_BASE + AUTO_UPDATE_UPGRADE_INDEX as usize * priv_.size_per_bitstream) as u32 };
    if !is_info { let ret = mpfs_auto_update_set_image_address(priv_, image_address, AUTO_UPDATE_UPGRADE_DIRECTORY as i64); if ret != 0 { return ret; } }
    else if size as usize > AUTO_UPDATE_INFO_SIZE { return -28; }
    let mut erase = EraseInfo { addr: image_address as i64, len: round_up(size as usize, (*priv_.flash).erasesize) };
    let mut ret = mtd_erase(priv_.flash, &mut erase); if ret != 0 { return ret; }
    let mut bytes_written = 0usize;
    ret = mtd_write(priv_.flash, image_address as i64, size as usize, &mut bytes_written, data);
    if ret != 0 { return ret; }
    if bytes_written != size as usize { return -5; }
    *written = bytes_written as u32; 0
}

unsafe fn mpfs_auto_update_write(fw: *mut FwUpload, data: *const u8, offset: u32, size: u32, written: *mut u32) -> FwUploadErr {
    let priv_ = &mut *((*fw).dd_handle as *mut MpfsAutoUpdatePriv);
    if mpfs_auto_update_write_bitstream(fw, data, offset, size, written) != 0 { return FwUploadErr::RwError; }
    if priv_.cancel_request { return FwUploadErr::Canceled; }
    if mpfs_auto_update_is_bitstream_info(data, size) { return FwUploadErr::None; }
    if mpfs_auto_update_verify_image(fw) != 0 { return FwUploadErr::FwInvalid; }
    FwUploadErr::None
}

static MPFS_AUTO_UPDATE_OPS: FwUploadOps = FwUploadOps { prepare: Some(mpfs_auto_update_prepare), write: Some(mpfs_auto_update_write), poll_complete: Some(mpfs_auto_update_poll_complete), cancel: Some(mpfs_auto_update_cancel) };

unsafe fn mpfs_auto_update_available(priv_: *mut MpfsAutoUpdatePriv) -> i32 {
    let mut response_msg = [0u32; AUTO_UPDATE_FEATURE_RESP_SIZE];
    let mut response = MpfsMssResponse { resp_msg: response_msg.as_mut_ptr(), resp_size: AUTO_UPDATE_FEATURE_RESP_SIZE as u32, resp_status: 0 };
    let mut message = MpfsMssMsg { cmd_opcode: AUTO_UPDATE_FEATURE_CMD_OPCODE, cmd_data_size: AUTO_UPDATE_FEATURE_CMD_DATA_SIZE, response: &mut response, cmd_data: core::ptr::null(), mbox_offset: AUTO_UPDATE_DEFAULT_MBOX_OFFSET, resp_offset: AUTO_UPDATE_DEFAULT_RESP_OFFSET };
    let ret = mpfs_blocking_transaction((*priv_).sys_controller, &mut message);
    if ret != 0 { return ret; }
    if response.resp_status != 0 { return -5; }
    let bytes = response_msg.as_ptr() as *const u8;
    if *bytes.add(1) & AUTO_UPDATE_FEATURE_ENABLED != 0 { return -1; }
    0
}

unsafe fn mpfs_auto_update_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut priv_ = Box::new(MpfsAutoUpdatePriv { sys_controller: core::ptr::null_mut(), dev: &mut (*pdev).dev, flash: core::ptr::null_mut(), fw_uploader: core::ptr::null_mut(), size_per_bitstream: 0, cancel_request: false });
    let ptr = &mut *priv_ as *mut MpfsAutoUpdatePriv;
    let ret = mpfs_auto_update_available(ptr);
    if ret != 0 { return ret; }
    core::mem::forget(priv_);
    0
}

unsafe fn mpfs_auto_update_remove(_pdev: *mut PlatformDevice) {}

#[repr(C)] struct PlatformDriver {
    name: &'static str,
    probe: Option<unsafe fn(*mut PlatformDevice) -> i32>,
    remove: Option<unsafe fn(*mut PlatformDevice)>,
}
static mut MPFS_AUTO_UPDATE_DRIVER: PlatformDriver = PlatformDriver {
    name: "mpfs-auto-update", probe: Some(mpfs_auto_update_probe), remove: Some(mpfs_auto_update_remove),
};

// module_platform_driver(mpfs_auto_update_driver)
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Conor Dooley <conor.dooley@microchip.com>");
// MODULE_DESCRIPTION("PolarFire SoC Auto Update FPGA reprogramming");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
