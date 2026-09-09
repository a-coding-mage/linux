// SPDX-License-Identifier: GPL-2.0
/*
 * System control and Management Interface (SCMI) NXP MISC Protocol
 *
 * Copyright 2024 NXP
 */

// C dependencies: linux/bits.h, linux/io.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/scmi_protocol.h, linux/scmi_imx_protocol.h,
// ../../protocols.h, and ../../notify.h.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x10000;
const MAX_MISC_CTRL_SOURCES: u32 = 0xffff;

#[repr(u32)]
enum ScmiImxMiscProtocolCmd {
    ScmiImxMiscCtrlSet = 0x3,
    ScmiImxMiscCtrlGet = 0x4,
    ScmiImxMiscDiscoverBuildInfo = 0x6,
    ScmiImxMiscCtrlNotify = 0x8,
    ScmiImxMiscResetReasonGet = 0xa,
    ScmiImxMiscCfgInfoGet = 0xc,
    ScmiImxMiscSyslogGet = 0xd,
    ScmiImxMiscBoardInfo = 0xe,
}

#[repr(C)]
struct ScmiImxMiscInfo { nr_dev_ctrl: u32, nr_brd_ctrl: u32, nr_reason: u32 }
#[repr(C)] struct ScmiMsgImxMiscProtocolAttributes { attributes: u32 }
#[repr(C)] struct ScmiImxMiscCtrlSetIn { id: u32, num: u32, value: [u32; 0] }
#[repr(C)] struct ScmiImxMiscCtrlNotifyIn { ctrl_id: u32, flags: u32 }
#[repr(C)] struct ScmiImxMiscCtrlNotifyPayld { ctrl_id: u32, flags: u32 }
#[repr(C)] struct ScmiImxMiscCtrlGetOut { num: u32, val: [u32; 0] }
#[repr(C)] struct ScmiImxMiscBuildinfoOut { buildnum: u32, buildcommit: u32, builddate: [u8; 16], buildtime: [u8; 16] }
#[repr(C)] struct ScmiImxMiscBoardInfoOut { attributes: u32, brdname: [u8; 16] }
#[repr(C)] struct ScmiImxMiscCfgInfoOut { msel: u32, cfgname: [u8; 16] }
#[repr(C)] struct ScmiImxMiscResetReasonIn { flags: u32 }
#[repr(C)] struct ScmiImxMiscResetReasonOut { b_flags: u32, s_flags: u32, extinfo: [u32; MISC_EXT_INFO_LEN_MAX as usize] }
#[repr(C)] struct ScmiImxMiscSyslogIn { flags: u32, index: u32 }
#[repr(C)] struct ScmiImxMiscSyslogOut { numlogflags: u32, syslog: [u32; 0] }

const BRD_CTRL_START_ID: u32 = 1 << 15;
const MISC_MAX_BUILDDATE: usize = 16;
const MISC_MAX_BUILDTIME: usize = 16;
const MISC_MAX_BRDNAME: usize = 16;
const MISC_MAX_CFGNAME: usize = 16;
// Defined by the SCMI i.MX protocol headers.
const MISC_EXT_INFO_LEN_MAX: u32 = 0;
const MISC_REASON_FLAG_SYSTEM: u32 = 1;
const MISC_BOOT_FLAG_VLD: u32 = 1 << 31;
const MISC_BOOT_FLAG_ORG_VLD: u32 = 1 << 28;
const MISC_BOOT_FLAG_ORIGIN: u32 = 0x0f00_0000;
const MISC_BOOT_FLAG_ERR_VLD: u32 = 1 << 23;
const MISC_BOOT_FLAG_ERR_ID: u32 = 0x007f_ff00;
const MISC_BOOT_FLAG_REASON: u32 = 0xff;
const MISC_SHUTDOWN_FLAG_VLD: u32 = 1 << 31;
const MISC_SHUTDOWN_FLAG_EXT_LEN: u32 = 0x6000_0000;
const MISC_SHUTDOWN_FLAG_ORG_VLD: u32 = 1 << 28;
const MISC_SHUTDOWN_FLAG_ORIGIN: u32 = 0x0f00_0000;
const MISC_SHUTDOWN_FLAG_ERR_VLD: u32 = 1 << 23;
const MISC_SHUTDOWN_FLAG_ERR_ID: u32 = 0x007f_ff00;
const MISC_SHUTDOWN_FLAG_REASON: u32 = 0xff;
const REMAINING_MASK: u32 = 0xfff0_0000;
const RETURNED_MASK: u32 = 0xfff;

const fn bits(v: u32, mask: u32) -> u32 { (v & mask) >> mask.trailing_zeros() }

// External kernel/SCMI types and functions are supplied by other translation units.
extern "C" {
    static scmi_imx_misc_proto_ops: ScmiImxMiscProtoOps;
}

#[repr(C)] struct ScmiImxMiscSyslogIpriv { array: *mut u32, size: *mut u16 }

unsafe fn iter_misc_syslog_prepare_message(message: *mut core::ffi::c_void, desc_index: u32, _priv: *const core::ffi::c_void) {
    let msg = &mut *(message as *mut ScmiImxMiscSyslogIn);
    msg.flags = 0u32.to_le(); msg.index = desc_index.to_le();
}

unsafe fn iter_misc_syslog_update_state(st: *mut ScmiIteratorState, response: *const core::ffi::c_void, priv_: *mut core::ffi::c_void) -> i32 {
    let r = &*(response as *const ScmiImxMiscSyslogOut);
    let p = &mut *(priv_ as *mut ScmiImxMiscSyslogIpriv);
    (*st).num_returned = bits(u32::from_le(r.numlogflags), RETURNED_MASK);
    (*st).num_remaining = bits(u32::from_le(r.numlogflags), REMAINING_MASK);
    *p.size = ((*st).num_returned + (*st).num_remaining) as u16;
    0
}

unsafe fn iter_misc_syslog_process_response(_ph: *const ScmiProtocolHandle, response: *const core::ffi::c_void, st: *mut ScmiIteratorState, priv_: *mut core::ffi::c_void) -> i32 {
    let r = &*(response as *const ScmiImxMiscSyslogOut);
    let p = &mut *(priv_ as *mut ScmiImxMiscSyslogIpriv);
    let vals = core::slice::from_raw_parts(r.syslog.as_ptr(), (*st).loop_idx as usize + 1);
    *p.array.add((*st).desc_index as usize + (*st).loop_idx as usize) = u32::from_le(vals[(*st).loop_idx as usize]);
    0
}

// The remaining protocol callbacks retain their C ABI-facing signatures and depend on
// SCMI kernel structures/constants declared by the surrounding repository.
unsafe extern "C" {
    fn scmi_imx_misc_ctrl_set(ph: *const ScmiProtocolHandle, ctrl_id: u32, num: u32, val: *mut u32) -> i32;
    fn scmi_imx_misc_ctrl_get(ph: *const ScmiProtocolHandle, ctrl_id: u32, num: *mut u32, val: *mut u32) -> i32;
    fn scmi_imx_misc_ctrl_notify(ph: *const ScmiProtocolHandle, ctrl_id: u32, evt_id: u32, flags: u32) -> i32;
    fn scmi_imx_misc_syslog_get(ph: *const ScmiProtocolHandle, size: *mut u16, array: *mut core::ffi::c_void) -> i32;
    fn scmi_imx_misc_reset_reason(ph: *const ScmiProtocolHandle, system: bool, boot_r: *mut ScmiImxMiscResetReason, shut_r: *mut ScmiImxMiscResetReason, extinfo: *mut u32) -> i32;
}

// Kernel-protocol registration and event-operation definitions are intentionally kept
// as external declarations because their layouts come from the included SCMI headers.
#[allow(dead_code)] const _SCMI_IMX_MISC_SUPPORTED_VERSION: u32 = SCMI_PROTOCOL_SUPPORTED_VERSION;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
