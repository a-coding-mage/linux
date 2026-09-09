/*
 * HP i8042 System Device Controller -- header
 *
 * Copyright (c) 2001 Brian S. Julin
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions, and the following disclaimer,
 *    without modification.
 * 2. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL").
 */

// C header dependencies: linux/interrupt.h, linux/types.h, linux/time.h,
// linux/timer.h, and arch-specific hardware definitions are supplied elsewhere.

pub const HP_SDC_MAX_REG_DELAY: u32 = 20000;

pub type HpSdcIrqhook = unsafe extern "C" fn(irq: i32, dev_id: *mut core::ffi::c_void, status: u8, data: u8);

unsafe extern "C" {
    pub fn hp_sdc_request_timer_irq(callback: Option<HpSdcIrqhook>) -> i32;
    pub fn hp_sdc_request_hil_irq(callback: Option<HpSdcIrqhook>) -> i32;
    pub fn hp_sdc_request_cooked_irq(callback: Option<HpSdcIrqhook>) -> i32;
    pub fn hp_sdc_release_timer_irq(callback: Option<HpSdcIrqhook>) -> i32;
    pub fn hp_sdc_release_hil_irq(callback: Option<HpSdcIrqhook>) -> i32;
    pub fn hp_sdc_release_cooked_irq(callback: Option<HpSdcIrqhook>) -> i32;
}

#[repr(C)]
pub union HpSdcTransactionAct {
    pub irqhook: Option<HpSdcIrqhook>,
    pub semaphore: *mut Semaphore,
}

#[repr(C)]
pub struct HpSdcTransaction {
    pub actidx: i32,
    pub idx: i32,
    pub endidx: i32,
    pub seq: *mut u8,
    pub act: HpSdcTransactionAct,
}

unsafe extern "C" {
    pub fn __hp_sdc_enqueue_transaction(this: *mut HpSdcTransaction) -> i32;
    pub fn hp_sdc_enqueue_transaction(this: *mut HpSdcTransaction) -> i32;
    pub fn hp_sdc_dequeue_transaction(this: *mut HpSdcTransaction) -> i32;
}

// The following opaque types are supplied by the kernel dependencies.
#[repr(C)] pub struct Semaphore { _private: [u8; 0] }
#[repr(C)] pub struct Rwlock { _private: [u8; 0] }
#[repr(C)] pub struct Ktime { _private: [u8; 0] }
#[repr(C)] pub struct TimerList { _private: [u8; 0] }
#[repr(C)] pub struct TaskletStruct { _private: [u8; 0] }
#[repr(C)] pub struct PariscDevice { _private: [u8; 0] }

pub const HP_SDC_ACT_PRECMD: u8 = 0x01;
pub const HP_SDC_ACT_DATAREG: u8 = 0x02;
pub const HP_SDC_ACT_DATAOUT: u8 = 0x04;
pub const HP_SDC_ACT_POSTCMD: u8 = 0x08;
pub const HP_SDC_ACT_DATAIN: u8 = 0x10;
pub const HP_SDC_ACT_DURING: u8 = 0x1f;
pub const HP_SDC_ACT_SEMAPHORE: u8 = 0x20;
pub const HP_SDC_ACT_CALLBACK: u8 = 0x40;
pub const HP_SDC_ACT_DEALLOC: u8 = 0x80;
pub const HP_SDC_ACT_AFTER: u8 = 0xe0;
pub const HP_SDC_ACT_DEAD: u8 = 0x60;

pub const HP_SDC_STATUS_IBF: u8 = 0x02;
pub const HP_SDC_STATUS_IRQMASK: u8 = 0xf0;
pub const HP_SDC_STATUS_PERIODIC: u8 = 0x10;
pub const HP_SDC_STATUS_USERTIMER: u8 = 0x20;
pub const HP_SDC_STATUS_TIMER: u8 = 0x30;
pub const HP_SDC_STATUS_REG: u8 = 0x40;
pub const HP_SDC_STATUS_HILCMD: u8 = 0x50;
pub const HP_SDC_STATUS_HILDATA: u8 = 0x60;
pub const HP_SDC_STATUS_PUP: u8 = 0x70;
pub const HP_SDC_STATUS_KCOOKED: u8 = 0x80;
pub const HP_SDC_STATUS_KRPG: u8 = 0xc0;
pub const HP_SDC_STATUS_KMOD_SUP: u8 = 0x10;
pub const HP_SDC_STATUS_KMOD_CUP: u8 = 0x20;
pub const HP_SDC_NMISTATUS_FHS: u8 = 0x40;

pub const HP_SDC_USE: u8 = 0x02; pub const HP_SDC_IM: u8 = 0x04;
pub const HP_SDC_CFG: u8 = 0x11; pub const HP_SDC_KBLANGUAGE: u8 = 0x12;
pub const HP_SDC_D0: u8 = 0x70; pub const HP_SDC_D1: u8 = 0x71; pub const HP_SDC_D2: u8 = 0x72; pub const HP_SDC_D3: u8 = 0x73;
pub const HP_SDC_VT1: u8 = 0x74; pub const HP_SDC_VT2: u8 = 0x75; pub const HP_SDC_VT3: u8 = 0x76; pub const HP_SDC_VT4: u8 = 0x77;
pub const HP_SDC_KBN: u8 = 0x78; pub const HP_SDC_KBC: u8 = 0x79; pub const HP_SDC_LPS: u8 = 0x7a; pub const HP_SDC_LPC: u8 = 0x7b;
pub const HP_SDC_RSV: u8 = 0x7c; pub const HP_SDC_LPR: u8 = 0x7d; pub const HP_SDC_XTD: u8 = 0x7e; pub const HP_SDC_STR: u8 = 0x7f;

pub const HP_SDC_USE_LOOP: u8 = 0x04;
pub const HP_SDC_IM_MASK: u8 = 0x1f; pub const HP_SDC_IM_FH: u8 = 0x10; pub const HP_SDC_IM_PT: u8 = 0x08; pub const HP_SDC_IM_TIMERS: u8 = 0x04; pub const HP_SDC_IM_RESET: u8 = 0x02; pub const HP_SDC_IM_HIL: u8 = 0x01;
pub const HP_SDC_CFG_ROLLOVER: u8 = 0x08; pub const HP_SDC_CFG_KBD: u8 = 0x10; pub const HP_SDC_CFG_NEW: u8 = 0x20; pub const HP_SDC_CFG_KBD_OLD: u8 = 0x03; pub const HP_SDC_CFG_KBD_NEW: u8 = 0x07; pub const HP_SDC_CFG_REV: u8 = 0x40; pub const HP_SDC_CFG_IDPROM: u8 = 0x80;
pub const HP_SDC_LPS_NDEV: u8 = 0x07; pub const HP_SDC_LPS_ACSUCC: u8 = 0x08; pub const HP_SDC_LPS_ACFAIL: u8 = 0x80;
pub const HP_SDC_LPC_APE_IPF: u8 = 0x01; pub const HP_SDC_LPC_ARCONERR: u8 = 0x02; pub const HP_SDC_LPC_ARCQUIET: u8 = 0x03; pub const HP_SDC_LPC_COOK: u8 = 0x10; pub const HP_SDC_LPC_RC: u8 = 0x80;
pub const HP_SDC_XTD_REV: u8 = 0x07;

pub fn hp_sdc_xtd_rev_strings(val: u8) -> &'static str {
    match val { 0x1 => "1820-3712", 0x2 => "1820-4379", 0x3 => "1820-4784", _ => "unknown" }
}
pub const HP_SDC_XTD_BEEPER: u8 = 0x08; pub const HP_SDC_XTD_BBRTC: u8 = 0x20;

pub const HP_SDC_CMD_LOAD_RT: u8 = 0x31; pub const HP_SDC_CMD_LOAD_FHS: u8 = 0x36; pub const HP_SDC_CMD_LOAD_MT: u8 = 0x38; pub const HP_SDC_CMD_LOAD_DT: u8 = 0x3B; pub const HP_SDC_CMD_LOAD_CT: u8 = 0x3E;
pub const HP_SDC_CMD_SET_IM: u8 = 0x40;
pub const HP_SDC_CMD_READ_RAM: u8 = 0x00; pub const HP_SDC_CMD_READ_USE: u8 = 0x02; pub const HP_SDC_CMD_READ_IM: u8 = 0x04; pub const HP_SDC_CMD_READ_KCC: u8 = 0x11; pub const HP_SDC_CMD_READ_KLC: u8 = 0x12;
pub const HP_SDC_CMD_READ_T1: u8 = 0x13; pub const HP_SDC_CMD_READ_T2: u8 = 0x14; pub const HP_SDC_CMD_READ_T3: u8 = 0x15; pub const HP_SDC_CMD_READ_T4: u8 = 0x16; pub const HP_SDC_CMD_READ_T5: u8 = 0x17;
pub const HP_SDC_CMD_READ_D0: u8 = 0xf0; pub const HP_SDC_CMD_READ_D1: u8 = 0xf1; pub const HP_SDC_CMD_READ_D2: u8 = 0xf2; pub const HP_SDC_CMD_READ_D3: u8 = 0xf3; pub const HP_SDC_CMD_READ_VT1: u8 = 0xf4; pub const HP_SDC_CMD_READ_VT2: u8 = 0xf5; pub const HP_SDC_CMD_READ_VT3: u8 = 0xf6; pub const HP_SDC_CMD_READ_VT4: u8 = 0xf7; pub const HP_SDC_CMD_READ_KBN: u8 = 0xf8; pub const HP_SDC_CMD_READ_KBC: u8 = 0xf9; pub const HP_SDC_CMD_READ_LPS: u8 = 0xfa; pub const HP_SDC_CMD_READ_LPC: u8 = 0xfb; pub const HP_SDC_CMD_READ_RSV: u8 = 0xfc; pub const HP_SDC_CMD_READ_LPR: u8 = 0xfd; pub const HP_SDC_CMD_READ_XTD: u8 = 0xfe; pub const HP_SDC_CMD_READ_STR: u8 = 0xff;
pub const HP_SDC_CMD_SET_ARD: u8 = 0xA0; pub const HP_SDC_CMD_SET_ARR: u8 = 0xA2; pub const HP_SDC_CMD_SET_BELL: u8 = 0xA3; pub const HP_SDC_CMD_SET_RPGR: u8 = 0xA6; pub const HP_SDC_CMD_SET_RTMS: u8 = 0xAD; pub const HP_SDC_CMD_SET_RTD: u8 = 0xAF; pub const HP_SDC_CMD_SET_FHS: u8 = 0xB2; pub const HP_SDC_CMD_SET_MT: u8 = 0xB4; pub const HP_SDC_CMD_SET_DT: u8 = 0xB7; pub const HP_SDC_CMD_SET_CT: u8 = 0xBA; pub const HP_SDC_CMD_SET_RAMP: u8 = 0xC1;
pub const HP_SDC_CMD_SET_D0: u8 = 0xe0; pub const HP_SDC_CMD_SET_D1: u8 = 0xe1; pub const HP_SDC_CMD_SET_D2: u8 = 0xe2; pub const HP_SDC_CMD_SET_D3: u8 = 0xe3; pub const HP_SDC_CMD_SET_VT1: u8 = 0xe4; pub const HP_SDC_CMD_SET_VT2: u8 = 0xe5; pub const HP_SDC_CMD_SET_VT3: u8 = 0xe6; pub const HP_SDC_CMD_SET_VT4: u8 = 0xe7; pub const HP_SDC_CMD_SET_KBN: u8 = 0xe8; pub const HP_SDC_CMD_SET_KBC: u8 = 0xe9; pub const HP_SDC_CMD_SET_LPS: u8 = 0xea; pub const HP_SDC_CMD_SET_LPC: u8 = 0xeb; pub const HP_SDC_CMD_SET_RSV: u8 = 0xec; pub const HP_SDC_CMD_SET_LPR: u8 = 0xed; pub const HP_SDC_CMD_SET_XTD: u8 = 0xee; pub const HP_SDC_CMD_SET_STR: u8 = 0xef;
pub const HP_SDC_CMD_DO_RTCW: u8 = 0xc2; pub const HP_SDC_CMD_DO_RTCR: u8 = 0xc3; pub const HP_SDC_CMD_DO_BEEP: u8 = 0xc4; pub const HP_SDC_CMD_DO_HIL: u8 = 0xc5;
pub const HP_SDC_DATA: u8 = 0x40; pub const HP_SDC_HIL_CMD: u8 = 0x50; pub const HP_SDC_HIL_R1MASK: u8 = 0x0f; pub const HP_SDC_HIL_AUTO: u8 = 0x10; pub const HP_SDC_HIL_ISERR: u8 = 0x80; pub const HP_SDC_HIL_RC_DONE: u8 = 0x80; pub const HP_SDC_HIL_ERR: u8 = 0x81; pub const HP_SDC_HIL_TO: u8 = 0x82; pub const HP_SDC_HIL_RC: u8 = 0x84; pub const HP_SDC_HIL_DAT: u8 = 0x60;

#[repr(C)]
pub struct HpI8042Sdc {
    pub ibf_lock: Rwlock, pub lock: Rwlock, pub rtq_lock: Rwlock, pub hook_lock: Rwlock,
    pub irq: u32, pub nmi: u32, pub base_io: usize, pub status_io: usize, pub data_io: usize,
    pub im: u8, pub set_im: i32, pub ibf: i32, pub wi: u8, pub r7: [u8; 4], pub r11: u8, pub r7e: u8,
    pub timer: Option<HpSdcIrqhook>, pub reg: Option<HpSdcIrqhook>, pub hil: Option<HpSdcIrqhook>, pub pup: Option<HpSdcIrqhook>, pub cooked: Option<HpSdcIrqhook>,
    pub tq: [*mut HpSdcTransaction; 16], pub rcurr: i32, pub rqty: i32, pub rtime: Ktime, pub wcurr: i32, pub dev_err: i32,
    // __hppa__: *mut PariscDevice; __mc68000__: *mut c_void; other architectures are unsupported.
    pub dev: *mut core::ffi::c_void,
    pub kicker: TimerList, pub task: TaskletStruct,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
