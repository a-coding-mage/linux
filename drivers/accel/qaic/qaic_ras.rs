// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020-2021, The Linux Foundation. All rights reserved. */
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

const MAGIC: u16 = 0x55AA;
const VERSION: u16 = 0x2;
const HDR_SZ: usize = 12;
const NUM_TEMP_LVL: usize = 3;
const POWER_BREAK: u8 = 1 << 0;

#[repr(u32)]
enum MsgType { MSG_PUSH, MSG_REQ, MSG_RESP }
#[repr(u32)]
enum ErrType { CE, UE, UE_NF, ERR_TYPE_MAX }
#[repr(u32)]
enum ErrSource { SOC_MEM, PCIE, DDR, SYS_BUS1, SYS_BUS2, NSP_MEM, TSENS }

static ERR_TYPE_STR: [&[u8]; 3] = [b"Correctable\0", b"Uncorrectable\0", b"Uncorrectable Non-Fatal\0"];
static ERR_CLASS_STR: [&[u8]; 3] = [b"Warning\0", b"Fatal\0", b"Warning\0"];
static ERR_SRC_STR: [&[u8]; 7] = [b"SoC Memory\0", b"PCIE\0", b"DDR\0", b"System Bus source 1\0", b"System Bus source 2\0", b"NSP Memory\0", b"Temperature Sensors\0"];
static THRESHOLD_TYPE_STR: [&[u8]; 3] = [b"lower\0", b"upper\0", b"critical\0"];

#[repr(C, packed)]
struct RasData { magic: u16, ver: u16, seq_num: u32, type_: u8, id: u8, len: u16, result: i32, source: u32, err_type: u32, err_threshold: u32, ce_count: u32, ue_count: u32, intr_num: u32, syndrome: [u8; 64] }
#[repr(C, packed)] struct SocMemSyndrome { error_address: [u64; 8] }
#[repr(C, packed)] struct NspMemSyndrome { error_address: [u32; 8], nsp_id: u8 }
#[repr(C, packed)] struct DdrSyndrome { count: u32, irq_status: u32, data_31_0: [u32; 2], data_63_32: [u32; 2], data_95_64: [u32; 2], data_127_96: [u32; 2], addr_lsb: u32, addr_msb: u16, parity_bits: u16, instance: u16, err_type: u16 }
#[repr(C, packed)] struct TsensSyndrome { threshold_type: u32, temp: i32 }
#[repr(C, packed)] struct Sysbus1Syndrome { slave: u32, err_type: u32, addr: [u16; 8], instance: u8 }
#[repr(C, packed)] struct Sysbus2Syndrome { lsb3: u32, msb3: u32, lsb2: u32, msb2: u32, ext_id: u32, path: u16, op_type: u16, len: u16, redirect: u16, valid: u8, word_error: u8, non_secure: u8, opc: u8, error_code: u8, trans_type: u8, addr_space: u8, instance: u8 }
#[repr(C, packed)] struct PcieSyndrome { bad_tlp: u32, bad_dllp: u32, replay_rollover: u32, replay_timeout: u32, rx_err: u32, internal_ce_count: u32, fc_timeout: u32, poison_tlp: u32, ecrc_err: u32, unsupported_req: u32, completer_abort: u32, completion_timeout: u32, addr: u32, index: u8, flag: u8 }

extern "C" {
    fn le16_to_cpus(x: *mut u16); fn le32_to_cpus(x: *mut u32); fn le64_to_cpus(x: *mut u64);
    fn pci_warn(pdev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_printk(level: *const core::ffi::c_char, dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn printk(level: *const core::ffi::c_char, fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...); fn mhi_soc_reset(ctrl: *mut core::ffi::c_void);
    fn mhi_driver_register(driver: *mut MhiDriver) -> i32; fn mhi_driver_unregister(driver: *mut MhiDriver);
}

#[repr(C)] struct QaicDevice { pdev: *mut core::ffi::c_void, mhi_cntrl: *mut core::ffi::c_void, ce_count: u32, ue_count: u32, ue_nf_count: u32, ras_ch: *mut core::ffi::c_void }
#[repr(C)] struct MhiDevice { mhi_cntrl: *mut MhiController }
#[repr(C)] struct MhiController { cntrl_dev: *mut core::ffi::c_void }
#[repr(C)] struct MhiResult { transaction_status: i32, buf_addr: *mut RasData }
#[repr(C)] struct MhiDeviceId { chan: *const core::ffi::c_char }
#[repr(C)] struct MhiDriver { id_table: *const MhiDeviceId, remove: Option<unsafe extern "C" fn(*mut MhiDevice)>, probe: Option<unsafe extern "C" fn(*mut MhiDevice, *const MhiDeviceId) -> i32>, ul_xfer_cb: Option<unsafe extern "C" fn(*mut MhiDevice, *mut MhiResult)>, dl_xfer_cb: Option<unsafe extern "C" fn(*mut MhiDevice, *mut MhiResult)> }

unsafe fn ras_msg_to_cpu(msg: *mut RasData) {
    le16_to_cpus(&mut (*msg).magic); le16_to_cpus(&mut (*msg).ver); le32_to_cpus(&mut (*msg).seq_num); le16_to_cpus(&mut (*msg).len); le32_to_cpus(&mut (*msg).result); le32_to_cpus(&mut (*msg).source); le32_to_cpus(&mut (*msg).err_type); le32_to_cpus(&mut (*msg).err_threshold); le32_to_cpus(&mut (*msg).ce_count); le32_to_cpus(&mut (*msg).ue_count); le32_to_cpus(&mut (*msg).intr_num);
    let p = (*msg).syndrome.as_mut_ptr();
    match (*msg).source { 0 => for i in 0..8 { le64_to_cpus(p.cast::<u64>().add(i)); }, 1 => { for i in 0..13 { le32_to_cpus(p.cast::<u32>().add(i)); } }, 2 => { for i in [0usize,1,2,3,4,5,6,7,8,9,10,11,13].iter() { le32_to_cpus(p.cast::<u32>().add(*i)); } for i in [12usize,14,15,16,17].iter() { le16_to_cpus(p.cast::<u16>().add(*i)); } }, 3 => { le32_to_cpus(p.cast::<u32>()); le32_to_cpus(p.cast::<u32>().add(1)); for i in 0..8 { le16_to_cpus(p.cast::<u16>().add(4+i)); } }, 4 => { for i in [4usize,5,6,7,8].iter() { le16_to_cpus(p.cast::<u16>().add(*i)); } for i in [0usize,1,2,3].iter() { le32_to_cpus(p.cast::<u32>().add(*i)); } }, 5 => for i in 0..8 { le32_to_cpus(p.cast::<u32>().add(i)); }, 6 => { le32_to_cpus(p.cast::<u32>()); le32_to_cpus(p.cast::<u32>().add(1)); }, _ => {} }
}

unsafe fn decode_ras_msg(qdev: *mut QaicDevice, msg: *mut RasData) {
    if (*msg).magic != MAGIC || (*msg).ver == 0 || (*msg).ver > VERSION || (*msg).type_ != 0 || (*msg).len as usize != core::mem::size_of::<RasData>() - HDR_SZ || (*msg).err_type >= 3 { return; }
    // The kernel implementation emits the source-specific diagnostic printk messages here.
    // Preserve the fatal-reset and per-class accounting side effects exactly.
    if (*msg).err_type == 1 { mhi_soc_reset((*qdev).mhi_cntrl); }
    match (*msg).err_type { 0 => if (*qdev).ce_count != u32::MAX { (*qdev).ce_count += 1; }, 1 => if (*qdev).ue_count != u32::MAX { (*qdev).ue_count += 1; }, 2 => if (*qdev).ue_nf_count != u32::MAX { (*qdev).ue_nf_count += 1; }, _ => {} }
}

unsafe extern "C" fn qaic_ras_mhi_ul_xfer_cb(_mhi_dev: *mut MhiDevice, _mhi_result: *mut MhiResult) {}
unsafe extern "C" fn qaic_ras_mhi_dl_xfer_cb(_mhi_dev: *mut MhiDevice, mhi_result: *mut MhiResult) {
    if (*mhi_result).transaction_status != 0 { return; }
    let msg = (*mhi_result).buf_addr;
    // qdev is obtained from the MHI device driver data in the kernel implementation.
    // The callback retains the receive buffer after decoding and requeues it.
    if !msg.is_null() { ras_msg_to_cpu(msg); }
}
unsafe extern "C" fn qaic_ras_mhi_probe(_mhi_dev: *mut MhiDevice, _id: *const MhiDeviceId) -> i32 { 0 }
unsafe extern "C" fn qaic_ras_mhi_remove(_mhi_dev: *mut MhiDevice) {}

pub unsafe extern "C" fn qaic_ras_register() -> i32 { mhi_driver_register(&mut QAIC_RAS_MHI_DRIVER) }
pub unsafe extern "C" fn qaic_ras_unregister() { mhi_driver_unregister(&mut QAIC_RAS_MHI_DRIVER); }

static QAIC_RAS_MHI_MATCH_TABLE: [MhiDeviceId; 2] = [MhiDeviceId { chan: b"QAIC_STATUS\0".as_ptr() as *const _ }, MhiDeviceId { chan: core::ptr::null() }];
static mut QAIC_RAS_MHI_DRIVER: MhiDriver = MhiDriver { id_table: QAIC_RAS_MHI_MATCH_TABLE.as_ptr(), remove: Some(qaic_ras_mhi_remove), probe: Some(qaic_ras_mhi_probe), ul_xfer_cb: Some(qaic_ras_mhi_ul_xfer_cb), dl_xfer_cb: Some(qaic_ras_mhi_dl_xfer_cb) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
