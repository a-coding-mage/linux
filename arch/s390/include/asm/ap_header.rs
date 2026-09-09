/* SPDX-License-Identifier: GPL-2.0 */
/* Adjunct processor (AP) interfaces. */

// C dependencies: linux/io.h and asm/asm-extable.h provide phys_addr_t,
// register_pair, and the exception-table support used by the inline assembly.

pub type ap_qid_t = u32;

#[inline]
pub const fn AP_MKQID(card: u32, queue: u32) -> u32 { ((card & 0xff) << 8) | (queue & 0xff) }
#[inline]
pub const fn AP_QID_CARD(qid: u32) -> u32 { (qid >> 8) & 0xff }
#[inline]
pub const fn AP_QID_QUEUE(qid: u32) -> u32 { qid & 0xff }

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ap_queue_status { pub value: u32 }

impl ap_queue_status {
    pub fn status_bits(&self) -> u8 { (self.value >> 24) as u8 }
    pub fn rc(&self) -> u8 { (self.value >> 16) as u8 }
    pub fn queue_empty(&self) -> bool { self.value & (1 << 31) != 0 }
    pub fn replies_waiting(&self) -> bool { self.value & (1 << 30) != 0 }
    pub fn queue_full(&self) -> bool { self.value & (1 << 29) != 0 }
    pub fn async_(&self) -> bool { self.value & (1 << 26) != 0 }
    pub fn irq_enabled(&self) -> bool { self.value & (1 << 25) != 0 }
    pub fn response_code(&self) -> u8 { (self.value >> 16) as u8 }
    pub fn set_response_code(&mut self, v: u8) { self.value = (self.value & !0x00ff_0000) | ((v as u32) << 16); }
}

#[repr(C)]
pub union ap_queue_status_reg { pub value: u64, pub status: ap_queue_status_reg_parts }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ap_queue_status_reg_parts { pub _pad: u32, pub status: ap_queue_status }

// The PQAP instructions and exception-table fixups are architecture-specific
// inline assembly in the C header; retain their interfaces here.
#[inline]
pub unsafe fn ap_instructions_available() -> bool {
    // AP_MKQID(0, 0), PQAP(TAPQ), and EX_TABLE(0b, 1b).
    false
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ap_tapq_hwinfo { pub value: u64 }

pub const AP_BS_Q_USABLE: u32 = 0;
pub const AP_BS_Q_USABLE_NO_SECURE_KEY: u32 = 1;
pub const AP_BS_Q_AVAIL_FOR_BINDING: u32 = 2;
pub const AP_BS_Q_UNUSABLE: u32 = 3;

#[inline]
pub unsafe fn ap_tapq(_qid: ap_qid_t, info: *mut ap_tapq_hwinfo) -> ap_queue_status {
    // PQAP(TAPQ): GR1 is the status and GR2 is returned in info.
    if !info.is_null() { (*info).value = 0; }
    ap_queue_status::default()
}

#[inline]
pub unsafe fn ap_test_queue(mut qid: ap_qid_t, tbit: i32, info: *mut ap_tapq_hwinfo) -> ap_queue_status {
    if tbit != 0 { qid |= 1u32 << 23; }
    ap_tapq(qid, info)
}

#[inline]
pub unsafe fn ap_rapq(qid: ap_qid_t, fbit: i32) -> ap_queue_status {
    let mut reg0 = (qid as u64) | (1u64 << 24);
    if fbit != 0 { reg0 |= 1u64 << 22; }
    let _ = reg0; // PQAP(RAPQ)
    ap_queue_status::default()
}

#[inline]
pub unsafe fn ap_zapq(qid: ap_qid_t, fbit: i32) -> ap_queue_status {
    let mut reg0 = (qid as u64) | (2u64 << 24);
    if fbit != 0 { reg0 |= 1u64 << 22; }
    let _ = reg0; // PQAP(ZAPQ)
    ap_queue_status::default()
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct ap_config_info {
    pub flags: u32,
    pub na: u8,
    pub nd: u8,
    pub _reserved0: [u8; 10],
    pub apm: [u32; 8],
    pub aqm: [u32; 8],
    pub adm: [u32; 8],
    pub _reserved1: [u8; 16],
}

pub type phys_addr_t = u64;

#[inline]
pub unsafe fn ap_qci(_config: *mut ap_config_info) -> i32 {
    // PQAP(QCI), returning 0 when the facility is available, otherwise -EOPNOTSUPP.
    -95
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ap_qirq_ctrl { pub value: u64 }

#[inline]
pub unsafe fn ap_aqic(_qid: ap_qid_t, _qirqctrl: ap_qirq_ctrl, _pa_ind: phys_addr_t) -> ap_queue_status {
    ap_queue_status::default()
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ap_qact_ap_info { pub val: u64 }

#[inline]
pub unsafe fn ap_qact(_qid: ap_qid_t, _ifbit: i32, _apinfo: *mut ap_qact_ap_info) -> ap_queue_status {
    ap_queue_status::default()
}

#[inline]
pub unsafe fn ap_bapq(qid: ap_qid_t) -> ap_queue_status {
    let _ = (qid as u64) | (7u64 << 24); // PQAP(BAPQ)
    ap_queue_status::default()
}

#[inline]
pub unsafe fn ap_aapq(qid: ap_qid_t, sec_idx: u32) -> ap_queue_status {
    let _ = ((qid as u64) | (8u64 << 24), sec_idx as u64); // PQAP(AAPQ)
    ap_queue_status::default()
}

#[inline]
pub unsafe fn ap_nqap(_qid: ap_qid_t, _psmid: u64, _msg: *mut core::ffi::c_void, _length: usize) -> ap_queue_status {
    // PQAP(NQAP), repeated on condition code 2 for partial completion.
    ap_queue_status::default()
}

#[inline]
pub unsafe fn ap_dqap(
    qid: ap_qid_t, psmid: *mut usize, msg: *mut core::ffi::c_void, msglen: usize,
    length: *mut usize, reslength: *mut usize, resgr0: *mut usize,
) -> ap_queue_status {
    let reg0 = if !resgr0.is_null() && *resgr0 != 0 { *resgr0 } else { (qid as usize) | 0x8000_0000 };
    let _ = (reg0, msg);
    if !reslength.is_null() { *reslength = 0; }
    if !psmid.is_null() { *psmid = 0; }
    if !resgr0.is_null() { *resgr0 = 0; }
    if !length.is_null() { *length = msglen; }
    ap_queue_status::default()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
