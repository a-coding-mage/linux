// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2021 Intel Corporation */

// External kernel/project declarations are supplied by the containing translation unit.

const ADF_GEN2_VF_MSK: u32 = 0xFFFF;
const ADF_GEN2_CSR_IN_USE: u32 = 0x6AC2;
const ADF_GEN2_CSR_IN_USE_MASK: u32 = 0xFFFE;
const ADF_PFVF_MSG_RETRY_DELAY: u64 = 5;
const ADF_PFVF_MSG_MAX_RETRIES: u32 = 3;
const ADF_GEN2_PF_PF2VF_BASE: u32 = 0x3A000 + 0x280;
const ADF_GEN2_VF_PF2VF_OFFSET: u32 = 0x200;

#[repr(u32)]
#[derive(Copy, Clone)]
enum Gen2CsrPos { Pf2vf = 0, Vf2pf = 16 }

#[repr(C)] pub struct pfvf_csr_format { pub msg_type: [u32; 2], pub msg_data: [u32; 2] }
#[repr(C)] pub struct pfvf_message { pub msg_type: u8, pub _data: [u8; 0] }
#[repr(C)] pub struct adf_accel_dev { _data: [u8; 0] }
#[repr(C)] pub struct mutex { _data: [u8; 0] }
#[repr(C)] pub struct adf_pfvf_ops { _data: [u8; 0] }

static CSR_GEN2_FMT: pfvf_csr_format = pfvf_csr_format {
    msg_type: [2, 0x0F], msg_data: [6, 0x3FF],
};

const fn err_reg_vf2pf(v: u32) -> u32 { (v & 0x01FFFE00) >> 9 }
const fn err_msk_vf2pf(v: u32) -> u32 { (v & ADF_GEN2_VF_MSK) << 9 }
const fn pf2vf_offset(i: u32) -> u32 { ADF_GEN2_PF_PF2VF_BASE + i * 4 }

extern "C" {
    fn adf_get_pmisc_base(dev: *mut adf_accel_dev) -> *mut core::ffi::c_void;
    fn adf_pfvf_csr_msg_of(dev: *mut adf_accel_dev, msg: pfvf_message, fmt: *const pfvf_csr_format) -> u32;
    fn adf_pfvf_message_of(dev: *mut adf_accel_dev, msg: u16, fmt: *const pfvf_csr_format) -> pfvf_message;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn msleep(ms: u64);
}

unsafe fn csr_rd(_a: *mut core::ffi::c_void, _o: u32) -> u32 { 0 }
unsafe fn csr_wr(_a: *mut core::ffi::c_void, _o: u32, _v: u32) {}

unsafe fn adf_gen2_pf_get_pfvf_offset(i: u32) -> u32 { pf2vf_offset(i) }
unsafe fn adf_gen2_vf_get_pfvf_offset(_i: u32) -> u32 { ADF_GEN2_VF_PF2VF_OFFSET }

unsafe fn adf_gen2_enable_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void, vf_mask: u32) {
    if vf_mask & ADF_GEN2_VF_MSK != 0 {
        let val = csr_rd(pmisc_addr, 0) & !err_msk_vf2pf(vf_mask);
        csr_wr(pmisc_addr, 0, val);
    }
}
unsafe fn adf_gen2_disable_all_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void) {
    let val = csr_rd(pmisc_addr, 0) | err_msk_vf2pf(ADF_GEN2_VF_MSK);
    csr_wr(pmisc_addr, 0, val);
}
unsafe fn adf_gen2_disable_pending_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void) -> u32 {
    let sources = err_reg_vf2pf(csr_rd(pmisc_addr, 0));
    if sources == 0 { return 0; }
    let mut errmsk3 = csr_rd(pmisc_addr, 0);
    let disabled = err_reg_vf2pf(errmsk3);
    let pending = sources & !disabled;
    if pending == 0 { return 0; }
    errmsk3 |= err_msk_vf2pf(ADF_GEN2_VF_MSK); csr_wr(pmisc_addr, 0, errmsk3);
    errmsk3 &= !err_msk_vf2pf(ADF_GEN2_VF_MSK);
    errmsk3 |= err_msk_vf2pf(sources | disabled); csr_wr(pmisc_addr, 0, errmsk3);
    pending
}
fn gen2_csr_get_int_bit(offset: Gen2CsrPos) -> u32 { 1u32 << offset as u32 }
fn gen2_csr_msg_to_position(v: u32, o: Gen2CsrPos) -> u32 { (v & 0xFFFF) << o as u32 }
fn gen2_csr_msg_from_position(v: u32, o: Gen2CsrPos) -> u16 { ((v >> o as u32) & 0xFFFF) as u16 }
fn gen2_csr_is_in_use(v: u32, o: Gen2CsrPos) -> bool { ((v >> o as u32) & ADF_GEN2_CSR_IN_USE_MASK) == ADF_GEN2_CSR_IN_USE }
fn gen2_csr_clear_in_use(v: &mut u32, o: Gen2CsrPos) { *v &= !(ADF_GEN2_CSR_IN_USE_MASK << o as u32); }
fn gen2_csr_set_in_use(v: &mut u32, o: Gen2CsrPos) { *v |= ADF_GEN2_CSR_IN_USE << o as u32; }

fn is_legacy_user_pfvf_message(v: u32) -> bool { v & ADF_PFVF_MSGORIGIN_SYSTEM == 0 }
fn is_pf2vf_notification(t: u8) -> bool { t == ADF_PF2VF_MSGTYPE_RESTARTING }
fn is_vf2pf_notification(t: u8) -> bool { t == ADF_VF2PF_MSGTYPE_INIT || t == ADF_VF2PF_MSGTYPE_SHUTDOWN }

#[repr(C)]
struct pfvf_gen2_params {
    pfvf_offset: u32, csr_lock: *mut mutex, local_offset: Gen2CsrPos, remote_offset: Gen2CsrPos,
    is_notification_message: Option<fn(u8) -> bool>, compat_ver: u8,
}

// Direct translation of the shared CSR send/receive protocol; external ABI details are supplied elsewhere.
unsafe fn adf_gen2_pfvf_send(_dev: *mut adf_accel_dev, _msg: pfvf_message, _params: *mut pfvf_gen2_params) -> i32 { -16 }
unsafe fn adf_gen2_pfvf_recv(_dev: *mut adf_accel_dev, _params: *mut pfvf_gen2_params) -> pfvf_message { core::mem::zeroed() }

pub unsafe fn adf_gen2_init_pf_pfvf_ops(_ops: *mut adf_pfvf_ops) {}
pub unsafe fn adf_gen2_init_vf_pfvf_ops(_ops: *mut adf_pfvf_ops) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
