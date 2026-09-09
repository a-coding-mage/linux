/* SPDX-License-Identifier: GPL-2.0 */
/*
 * I/O Processor (IOP) defines and structures, mostly snagged from A/UX
 * header files.
 *
 * The original header from which this was taken is copyrighted. I've done some
 * rewriting (in fact my changes make this a bit more readable, IMHO) but some
 * more should be done.
 */

/* This is the base address of the IOPs. */
pub const SCC_IOP_BASE_IIFX: u32 = 0x50F04000;
pub const ISM_IOP_BASE_IIFX: u32 = 0x50F12000;
pub const SCC_IOP_BASE_QUADRA: u32 = 0x50F0C000;
pub const ISM_IOP_BASE_QUADRA: u32 = 0x50F1E000;

/* IOP status/control register bits */
pub const IOP_BYPASS: u8 = 0x01;
pub const IOP_AUTOINC: u8 = 0x02;
pub const IOP_RUN: u8 = 0x04;
pub const IOP_IRQ: u8 = 0x08;
pub const IOP_INT0: u8 = 0x10;
pub const IOP_INT1: u8 = 0x20;
pub const IOP_HWINT: u8 = 0x40;
pub const IOP_DMAINACTIVE: u8 = 0x80;

pub const NUM_IOPS: u32 = 2;
pub const NUM_IOP_CHAN: u32 = 7;
pub const NUM_IOP_MSGS: u32 = NUM_IOP_CHAN * 8;
pub const IOP_MSG_LEN: usize = 32;

pub const IOP_NUM_SCC: u32 = 0;
pub const IOP_NUM_ISM: u32 = 1;

pub const IOP_MSG_IDLE: u32 = 0;
pub const IOP_MSG_NEW: u32 = 1;
pub const IOP_MSG_RCVD: u32 = 2;
pub const IOP_MSG_COMPLETE: u32 = 3;

pub const IOP_MSGSTATUS_UNUSED: u32 = 0;
pub const IOP_MSGSTATUS_WAITING: u32 = 1;
pub const IOP_MSGSTATUS_SENT: u32 = 2;
pub const IOP_MSGSTATUS_COMPLETE: u32 = 3;
pub const IOP_MSGSTATUS_UNSOL: u32 = 6;

pub const IOP_ADDR_MAX_SEND_CHAN: u32 = 0x0200;
pub const IOP_ADDR_SEND_STATE: u32 = 0x0201;
pub const IOP_ADDR_PATCH_CTRL: u32 = 0x021F;
pub const IOP_ADDR_SEND_MSG: u32 = 0x0220;
pub const IOP_ADDR_MAX_RECV_CHAN: u32 = 0x0300;
pub const IOP_ADDR_RECV_STATE: u32 = 0x0301;
pub const IOP_ADDR_ALIVE: u32 = 0x031F;
pub const IOP_ADDR_RECV_MSG: u32 = 0x0320;

#[repr(C)]
pub struct MacIopSccRegs {
    pub sccb_cmd: u8,
    pub pad4: u8,
    pub scca_cmd: u8,
    pub pad5: u8,
    pub sccb_data: u8,
    pub pad6: u8,
    pub scca_data: u8,
}

#[repr(C)]
pub struct MacIopIsmRegs {
    pub wdata: u8, pub pad7: u8, pub wmark: u8, pub pad8: u8,
    pub wcrc: u8, pub pad9: u8, pub wparams: u8, pub pad10: u8,
    pub wphase: u8, pub pad11: u8, pub wsetup: u8, pub pad12: u8,
    pub wzeroes: u8, pub pad13: u8, pub wones: u8, pub pad14: u8,
    pub rdata: u8, pub pad15: u8, pub rmark: u8, pub pad16: u8,
    pub rerror: u8, pub pad17: u8, pub rparams: u8, pub pad18: u8,
    pub rphase: u8, pub pad19: u8, pub rsetup: u8, pub pad20: u8,
    pub rmode: u8, pub pad21: u8, pub rhandshake: u8,
}

#[repr(C)]
pub union MacIopBypass {
    pub scc_regs: MacIopSccRegs,
    pub ism_regs: MacIopIsmRegs,
}

#[repr(C)]
pub struct MacIop {
    pub ram_addr_hi: u8,
    pub pad0: u8,
    pub ram_addr_lo: u8,
    pub pad1: u8,
    pub status_ctrl: u8,
    pub pad2: [u8; 3],
    pub ram_data: u8,
    pub pad3: [u8; 23],
    pub b: MacIopBypass,
}

#[repr(C)]
pub struct IopMsg {
    pub next: *mut IopMsg,
    pub iop_num: u32,
    pub channel: u32,
    pub caller_priv: *mut core::ffi::c_void,
    pub status: i32,
    pub message: [u8; IOP_MSG_LEN],
    pub reply: [u8; IOP_MSG_LEN],
    pub handler: Option<unsafe extern "C" fn(*mut IopMsg)>,
}

extern "C" {
    pub static mut iop_scc_present: i32;
    pub static mut iop_ism_present: i32;
    pub fn iop_listen(iop_num: u32, channel: u32, handler: Option<unsafe extern "C" fn(*mut IopMsg)>, name: *const core::ffi::c_char) -> i32;
    pub fn iop_send_message(iop_num: u32, channel: u32, priv_data: *mut core::ffi::c_void, length: u32, message: *mut u8, handler: Option<unsafe extern "C" fn(*mut IopMsg)>) -> i32;
    pub fn iop_complete_message(message: *mut IopMsg);
    pub fn iop_upload_code(iop_num: u32, code: *mut u8, length: u32, offset: u16);
    pub fn iop_download_code(iop_num: u32, code: *mut u8, length: u32, offset: u16);
    pub fn iop_compare_code(iop_num: u32, code: *mut u8, length: u32, offset: u16) -> *mut u8;
    pub fn iop_ism_irq_poll(iop_num: u32);
    pub fn iop_register_interrupts();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
