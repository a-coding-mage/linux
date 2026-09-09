/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <asm/irq.h> in the original header.
// `IRQ_USER` is expected to be provided by the surrounding translation unit.

#[repr(C)]
pub struct MVMElp {
    pub ack_icr: u8,
    pub flt_icr: u8,
    pub sel_icr: u8,
    pub pe_icr: u8,
    pub bsy_icr: u8,
    pub spare1: u8,
    pub isr: u8,
    pub cr: u8,
    pub spare2: u8,
    pub spare3: u8,
    pub spare4: u8,
    pub data: u8,
}

pub type MVMElpPtr = *mut MVMElp;

pub const MVME_LPR_BASE: usize = 0xfff4_2030;

// C macro: ((*(volatile MVMElpPtr)(MVME_LPR_BASE)))
pub const mvmelp: MVMElpPtr = MVME_LPR_BASE as *mut MVMElp;

pub const MVME_RTC_BASE: usize = 0xfffc_0000;

pub const MVME_I596_BASE: usize = 0xfff4_6000;

pub const MVME_SCC_A_ADDR: usize = 0xfff4_5005;
pub const MVME_SCC_B_ADDR: usize = 0xfff4_5001;
pub const MVME_SCC_PCLK: u32 = 10_000_000;

pub const MVME162_IRQ_TYPE_PRIO: i32 = 0;

pub const MVME167_IRQ_PRN: i32 = IRQ_USER + 20;
pub const MVME16x_IRQ_I596: i32 = IRQ_USER + 23;
pub const MVME16x_IRQ_SCSI: i32 = IRQ_USER + 21;
pub const MVME16x_IRQ_FLY: i32 = IRQ_USER + 63;
pub const MVME167_IRQ_SER_ERR: i32 = IRQ_USER + 28;
pub const MVME167_IRQ_SER_MODEM: i32 = IRQ_USER + 29;
pub const MVME167_IRQ_SER_TX: i32 = IRQ_USER + 30;
pub const MVME167_IRQ_SER_RX: i32 = IRQ_USER + 31;
pub const MVME16x_IRQ_TIMER: i32 = IRQ_USER + 25;
pub const MVME167_IRQ_ABORT: i32 = IRQ_USER + 46;
pub const MVME162_IRQ_ABORT: i32 = IRQ_USER + 30;

/* SCC interrupts, for MVME162 */
pub const MVME162_IRQ_SCC_BASE: i32 = IRQ_USER + 0;
pub const MVME162_IRQ_SCCB_TX: i32 = IRQ_USER + 0;
pub const MVME162_IRQ_SCCB_STAT: i32 = IRQ_USER + 2;
pub const MVME162_IRQ_SCCB_RX: i32 = IRQ_USER + 4;
pub const MVME162_IRQ_SCCB_SPCOND: i32 = IRQ_USER + 6;
pub const MVME162_IRQ_SCCA_TX: i32 = IRQ_USER + 8;
pub const MVME162_IRQ_SCCA_STAT: i32 = IRQ_USER + 10;
pub const MVME162_IRQ_SCCA_RX: i32 = IRQ_USER + 12;
pub const MVME162_IRQ_SCCA_SPCOND: i32 = IRQ_USER + 14;

/* MVME162 version register */
pub const MVME162_VERSION_REG: usize = 0xfff4_202e;

extern "C" {
    pub static mut mvme16x_config: u16;
}

/* Lower 8 bits must match the revision register in the MC2 chip */

pub const MVME16x_CONFIG_SPEED_32: u16 = 0x0001;
pub const MVME16x_CONFIG_NO_VMECHIP2: u16 = 0x0002;
pub const MVME16x_CONFIG_NO_SCSICHIP: u16 = 0x0004;
pub const MVME16x_CONFIG_NO_ETHERNET: u16 = 0x0008;
pub const MVME16x_CONFIG_GOT_FPU: u16 = 0x0010;

pub const MVME16x_CONFIG_GOT_LP: u16 = 0x0100;
pub const MVME16x_CONFIG_GOT_CD2401: u16 = 0x0200;
pub const MVME16x_CONFIG_GOT_SCCA: u16 = 0x0400;
pub const MVME16x_CONFIG_GOT_SCCB: u16 = 0x0800;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
