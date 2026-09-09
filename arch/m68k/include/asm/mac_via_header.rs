/* SPDX-License-Identifier: GPL-2.0 */
/* 6522 Versatile Interface Adapter (VIA) declarations for Macintosh. */

// Base addresses for the VIAs.
pub const VIA1_BASE: u32 = 0x50F00000;
pub const VIA2_BASE: u32 = 0x50F02000;
pub const RBV_BASE: u32 = 0x50F26000;

pub const VIA1A_vSccWrReq: u8 = 0x80;
pub const VIA1A_vRev8: u8 = 0x40;
pub const VIA1A_vHeadSel: u8 = 0x20;
pub const VIA1A_vOverlay: u8 = 0x10;
pub const VIA1A_vSync: u8 = 0x08;
pub const VIA1A_vVolume: u8 = 0x07;
pub const VIA1A_CPUID0: u8 = 0x02;
pub const VIA1A_CPUID1: u8 = 0x04;
pub const VIA1A_CPUID2: u8 = 0x10;
pub const VIA1A_CPUID3: u8 = 0x40;

pub const VIA1B_vSound: u8 = 0x80;
pub const VIA1B_vMystery: u8 = 0x40;
pub const VIA1B_vADBS2: u8 = 0x20;
pub const VIA1B_vADBS1: u8 = 0x10;
pub const VIA1B_vADBInt: u8 = 0x08;
pub const VIA1B_vRTCEnb: u8 = 0x04;
pub const VIA1B_vRTCClk: u8 = 0x02;
pub const VIA1B_vRTCData: u8 = 0x01;

pub const EVRB_XCVR: u8 = 0x08;
pub const EVRB_FULL: u8 = 0x10;
pub const EVRB_SYSES: u8 = 0x20;
pub const EVRB_AUXIE: u8 = 0x00;
pub const EVRB_AUXID: u8 = 0x40;
pub const EVRB_SFTWRIE: u8 = 0x00;
pub const EVRB_SFTWRID: u8 = 0x80;

pub const VIA2A_vRAM1: u8 = 0x80;
pub const VIA2A_vRAM0: u8 = 0x40;
pub const VIA2A_vIRQE: u8 = 0x20;
pub const VIA2A_vIRQD: u8 = 0x10;
pub const VIA2A_vIRQC: u8 = 0x08;
pub const VIA2A_vIRQB: u8 = 0x04;
pub const VIA2A_vIRQA: u8 = 0x02;
pub const VIA2A_vIRQ9: u8 = 0x01;

pub const VIA2B_vVBL: u8 = 0x80;
pub const VIA2B_vSndJck: u8 = 0x40;
pub const VIA2B_vTfr0: u8 = 0x20;
pub const VIA2B_vTfr1: u8 = 0x10;
pub const VIA2B_vMode32: u8 = 0x08;
pub const VIA2B_vPower: u8 = 0x04;
pub const VIA2B_vBusLk: u8 = 0x02;
pub const VIA2B_vCDis: u8 = 0x01;

pub const vBufB: u16 = 0x0000;
pub const vBufAH: u16 = 0x0200;
pub const vDirB: u16 = 0x0400;
pub const vDirA: u16 = 0x0600;
pub const vT1CL: u16 = 0x0800;
pub const vT1CH: u16 = 0x0a00;
pub const vT1LL: u16 = 0x0c00;
pub const vT1LH: u16 = 0x0e00;
pub const vT2CL: u16 = 0x1000;
pub const vT2CH: u16 = 0x1200;
pub const vSR: u16 = 0x1400;
pub const vACR: u16 = 0x1600;
pub const vPCR: u16 = 0x1800;
pub const vIFR: u16 = 0x1a00;
pub const vIER: u16 = 0x1c00;
pub const vBufA: u16 = 0x1e00;

pub const rBufB: u16 = 0x0000;
pub const rExp: u16 = 0x0001;
pub const rSIFR: u16 = 0x0002;
pub const rIFR: u16 = 0x1a03;
pub const rMonP: u16 = 0x0010;
pub const rChpT: u16 = 0x0011;
pub const rSIER: u16 = 0x0012;
pub const rIER: u16 = 0x1c13;
pub const rBufA: u16 = rSIFR;

pub const RBV_DEPTH: u8 = 0x07;
pub const RBV_MONID: u8 = 0x38;
pub const RBV_VIDOFF: u8 = 0x40;
pub const MON_15BW: u8 = 1 << 3;
pub const MON_IIGS: u8 = 2 << 3;
pub const MON_15RGB: u8 = 5 << 3;
pub const MON_12OR13: u8 = 6 << 3;
pub const MON_NONE: u8 = 7 << 3;

#[inline]
pub const fn IER_SET_BIT(b: u32) -> u32 { 0x80 | (1 << b) }
#[inline]
pub const fn IER_CLR_BIT(b: u32) -> u32 { 0x7F & (1 << b) }

extern "C" {
    pub static mut via1: *mut u8;
    pub static mut via2: *mut u8;
    pub static mut rbv_present: i32;
    pub static mut via_alt_mapping: i32;

    pub fn via_l2_flush(writeback: i32);
    pub fn via_register_interrupts();
    pub fn via_irq_enable(_: i32);
    pub fn via_irq_disable(_: i32);
    pub fn via_nubus_irq_startup(irq: i32);
    pub fn via_nubus_irq_shutdown(irq: i32);
    pub fn via1_irq(desc: *mut irq_desc);
    pub fn via1_set_head(_: i32);
    pub fn via2_scsi_drq_pending() -> i32;
}

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
