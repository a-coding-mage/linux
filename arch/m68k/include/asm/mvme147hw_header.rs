/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: IRQ_USER is supplied by asm/irq.h.

pub const MVME147_RTC_BASE: u32 = 0xfffe0000;

#[repr(C)]
pub struct pcc_regs {
    // C volatile fields; accesses must preserve volatile semantics.
    pub dma_tadr: u32,
    pub dma_dadr: u32,
    pub dma_bcr: u32,
    pub dma_hr: u32,
    pub t1_preload: u16,
    pub t1_count: u16,
    pub t2_preload: u16,
    pub t2_count: u16,
    pub t1_int_cntrl: u8,
    pub t1_cntrl: u8,
    pub t2_int_cntrl: u8,
    pub t2_cntrl: u8,
    pub ac_fail: u8,
    pub watchdog: u8,
    pub lpt_intr: u8,
    pub lpt_cntrl: u8,
    pub dma_intr: u8,
    pub dma_cntrl: u8,
    pub bus_error: u8,
    pub dma_status: u8,
    pub abort: u8,
    pub ta_fnctl: u8,
    pub serial_cntrl: u8,
    pub general_cntrl: u8,
    pub lan_cntrl: u8,
    pub general_status: u8,
    pub scsi_interrupt: u8,
    pub slave: u8,
    pub soft1_cntrl: u8,
    pub int_base: u8,
    pub soft2_cntrl: u8,
    pub revision_level: u8,
    pub lpt_data: u8,
    pub lpt_status: u8,
}

pub const m147_pcc: *mut pcc_regs = 0xfffe1000 as *mut pcc_regs;

pub const PCC_INT_ENAB: u8 = 0x08;
pub const PCC_TIMER_INT_CLR: u8 = 0x80;
pub const PCC_TIMER_TIC_EN: u8 = 0x01;
pub const PCC_TIMER_COC_EN: u8 = 0x02;
pub const PCC_TIMER_CLR_OVF: u8 = 0x04;

pub const PCC_LEVEL_ABORT: u8 = 0x07;
pub const PCC_LEVEL_SERIAL: u8 = 0x04;
pub const PCC_LEVEL_ETH: u8 = 0x04;
pub const PCC_LEVEL_TIMER1: u8 = 0x04;
pub const PCC_LEVEL_SCSI_PORT: u8 = 0x04;
pub const PCC_LEVEL_SCSI_DMA: u8 = 0x04;

pub const PCC_IRQ_AC_FAIL: _ = IRQ_USER + 0;
pub const PCC_IRQ_BERR: _ = IRQ_USER + 1;
pub const PCC_IRQ_ABORT: _ = IRQ_USER + 2;
// #define PCC_IRQ_SERIAL (IRQ_USER+3)
pub const PCC_IRQ_PRINTER: _ = IRQ_USER + 7;
pub const PCC_IRQ_TIMER1: _ = IRQ_USER + 8;
pub const PCC_IRQ_TIMER2: _ = IRQ_USER + 9;
pub const PCC_IRQ_SOFTWARE1: _ = IRQ_USER + 10;
pub const PCC_IRQ_SOFTWARE2: _ = IRQ_USER + 11;

pub const M147_SCC_A_ADDR: u32 = 0xfffe3002;
pub const M147_SCC_B_ADDR: u32 = 0xfffe3000;
pub const M147_SCC_PCLK: u32 = 5000000;

pub const MVME147_IRQ_SCSI_PORT: _ = IRQ_USER + 5;
pub const MVME147_IRQ_SCSI_DMA: _ = IRQ_USER + 6;

/* SCC interrupts, for MVME147 */
pub const MVME147_IRQ_TYPE_PRIO: i32 = 0;
pub const MVME147_IRQ_SCC_BASE: _ = IRQ_USER + 32;
pub const MVME147_IRQ_SCCB_TX: _ = IRQ_USER + 32;
pub const MVME147_IRQ_SCCB_STAT: _ = IRQ_USER + 34;
pub const MVME147_IRQ_SCCB_RX: _ = IRQ_USER + 36;
pub const MVME147_IRQ_SCCB_SPCOND: _ = IRQ_USER + 38;
pub const MVME147_IRQ_SCCA_TX: _ = IRQ_USER + 40;
pub const MVME147_IRQ_SCCA_STAT: _ = IRQ_USER + 42;
pub const MVME147_IRQ_SCCA_RX: _ = IRQ_USER + 44;
pub const MVME147_IRQ_SCCA_SPCOND: _ = IRQ_USER + 46;

pub const MVME147_LANCE_BASE: u32 = 0xfffe1800;
pub const MVME147_LANCE_IRQ: _ = IRQ_USER + 4;

pub const ETHERNET_ADDRESS: u32 = 0xfffe0778;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
