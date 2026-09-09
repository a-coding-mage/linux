/*
** atariints.h -- Atari Linux interrupt handling structs and prototypes
**
** Copyright 1994 by Björn Brauel
**
** 5/2/94 Roman Hodek:
**   TT interrupt definitions added.
**
** 12/02/96: (Roman)
**   Adapted to new int handling scheme (see ataints.c); revised numbering
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
*/

// Dependencies supplied by the surrounding translation unit: asm/irq.h, asm/atarihw.h.

pub const STMFP_SOURCE_BASE: i32 = 8;
pub const TTMFP_SOURCE_BASE: i32 = 24;
pub const SCC_SOURCE_BASE: i32 = 40;
pub const VME_SOURCE_BASE: i32 = 56;
pub const VME_MAX_SOURCES: i32 = 16;
pub const NUM_ATARI_SOURCES: i32 = 141;

pub const fn irq_vector_to_source(v: i32) -> i32 {
    v - if v < 0x20 { 0x18 } else { 0x40 - 8 }
}
pub const fn irq_source_to_vector(i: i32) -> i32 {
    i + if i < 8 { 0x18 } else { 0x40 - 8 }
}

pub const IRQ_MFP_BUSY: i32 = 8;
pub const IRQ_MFP_DCD: i32 = 9;
pub const IRQ_MFP_CTS: i32 = 10;
pub const IRQ_MFP_GPU: i32 = 11;
pub const IRQ_MFP_TIMD: i32 = 12;
pub const IRQ_MFP_TIMC: i32 = 13;
pub const IRQ_MFP_ACIA: i32 = 14;
pub const IRQ_MFP_FDC: i32 = 15;
pub const IRQ_MFP_ACSI: i32 = IRQ_MFP_FDC;
pub const IRQ_MFP_FSCSI: i32 = IRQ_MFP_FDC;
pub const IRQ_MFP_IDE: i32 = IRQ_MFP_FDC;
pub const IRQ_MFP_TIMB: i32 = 16;
pub const IRQ_MFP_SERERR: i32 = 17;
pub const IRQ_MFP_SEREMPT: i32 = 18;
pub const IRQ_MFP_RECERR: i32 = 19;
pub const IRQ_MFP_RECFULL: i32 = 20;
pub const IRQ_MFP_TIMA: i32 = 21;
pub const IRQ_MFP_RI: i32 = 22;
pub const IRQ_MFP_MMD: i32 = 23;

pub const IRQ_TT_MFP_IO0: i32 = 24;
pub const IRQ_TT_MFP_IO1: i32 = 25;
pub const IRQ_TT_MFP_SCC: i32 = 26;
pub const IRQ_TT_MFP_RI: i32 = 27;
pub const IRQ_TT_MFP_TIMD: i32 = 28;
pub const IRQ_TT_MFP_TIMC: i32 = 29;
pub const IRQ_TT_MFP_DRVRDY: i32 = 30;
pub const IRQ_TT_MFP_SCSIDMA: i32 = 31;
pub const IRQ_TT_MFP_TIMB: i32 = 32;
pub const IRQ_TT_MFP_SERERR: i32 = 33;
pub const IRQ_TT_MFP_SEREMPT: i32 = 34;
pub const IRQ_TT_MFP_RECERR: i32 = 35;
pub const IRQ_TT_MFP_RECFULL: i32 = 36;
pub const IRQ_TT_MFP_TIMA: i32 = 37;
pub const IRQ_TT_MFP_RTC: i32 = 38;
pub const IRQ_TT_MFP_SCSI: i32 = 39;

pub const IRQ_SCCB_TX: i32 = 40;
pub const IRQ_SCCB_STAT: i32 = 42;
pub const IRQ_SCCB_RX: i32 = 44;
pub const IRQ_SCCB_SPCOND: i32 = 46;
pub const IRQ_SCCA_TX: i32 = 48;
pub const IRQ_SCCA_STAT: i32 = 50;
pub const IRQ_SCCA_RX: i32 = 52;
pub const IRQ_SCCA_SPCOND: i32 = 54;

pub const IRQ_MFP_TIMER1: i32 = 64;
pub const IRQ_MFP_TIMER2: i32 = 65;
pub const IRQ_MFP_TIMER3: i32 = 66;
pub const IRQ_MFP_TIMER4: i32 = 67;
pub const IRQ_MFP_TIMER5: i32 = 68;
pub const IRQ_MFP_TIMER6: i32 = 69;
pub const IRQ_MFP_TIMER7: i32 = 70;
pub const IRQ_MFP_TIMER8: i32 = 71;

pub const INT_CLK: i32 = 24576;
pub const INT_TICKS: i32 = 246;

pub const MFP_ENABLE: i32 = 0;
pub const MFP_PENDING: i32 = 1;
pub const MFP_SERVICE: i32 = 2;
pub const MFP_MASK: i32 = 3;

#[repr(C)]
pub struct AtariMfp { pub int_en_a: u8 }
extern "C" { pub static mut st_mfp: AtariMfp; }

#[inline]
pub unsafe fn get_mfp_bit(irq: u32, typ: i32) -> i32 {
    let mask = 1u8 << (irq & 7);
    let reg = (&mut st_mfp.int_en_a as *mut u8).offset((typ * 4 + ((irq & 8) >> 2) as i32 + ((((irq - 8) & 16) << 3) as i32)) as isize);
    core::ptr::read_volatile(reg) as i32 & mask as i32
}

#[inline]
pub unsafe fn set_mfp_bit(irq: u32, typ: i32) {
    let mask = 1u8 << (irq & 7);
    let reg = (&mut st_mfp.int_en_a as *mut u8).offset((typ * 4 + ((irq & 8) >> 2) as i32 + ((((irq - 8) & 16) << 3) as i32)) as isize);
    core::ptr::write_volatile(reg, core::ptr::read_volatile(reg) | mask);
}

#[inline]
pub unsafe fn clear_mfp_bit(irq: u32, typ: i32) {
    let mask = !(1u8 << (irq & 7));
    let reg = (&mut st_mfp.int_en_a as *mut u8).offset((typ * 4 + ((irq & 8) >> 2) as i32 + ((((irq - 8) & 16) << 3) as i32)) as isize);
    core::ptr::write_volatile(reg, if typ == MFP_PENDING || typ == MFP_SERVICE { mask } else { core::ptr::read_volatile(reg) & mask });
}

#[inline] pub unsafe fn atari_enable_irq(irq: u32) { if irq >= STMFP_SOURCE_BASE as u32 && irq < SCC_SOURCE_BASE as u32 { set_mfp_bit(irq, MFP_MASK); } }
#[inline] pub unsafe fn atari_disable_irq(irq: u32) { if irq >= STMFP_SOURCE_BASE as u32 && irq < SCC_SOURCE_BASE as u32 { clear_mfp_bit(irq, MFP_MASK); } }
#[inline] pub unsafe fn atari_turnon_irq(irq: u32) { if irq >= STMFP_SOURCE_BASE as u32 && irq < SCC_SOURCE_BASE as u32 { set_mfp_bit(irq, MFP_ENABLE); } }
#[inline] pub unsafe fn atari_turnoff_irq(irq: u32) { if irq >= STMFP_SOURCE_BASE as u32 && irq < SCC_SOURCE_BASE as u32 { clear_mfp_bit(irq, MFP_ENABLE); clear_mfp_bit(irq, MFP_PENDING); } }
#[inline] pub unsafe fn atari_clear_pending_irq(irq: u32) { if irq >= STMFP_SOURCE_BASE as u32 && irq < SCC_SOURCE_BASE as u32 { clear_mfp_bit(irq, MFP_PENDING); } }
#[inline] pub unsafe fn atari_irq_pending(irq: u32) -> i32 { if irq < STMFP_SOURCE_BASE as u32 || irq >= SCC_SOURCE_BASE as u32 { 0 } else { get_mfp_bit(irq, MFP_PENDING) } }

extern "C" {
    pub fn atari_register_vme_int() -> u32;
    pub fn atari_unregister_vme_int(_: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
