/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/asm/sunbpp.h
 */

#[repr(C)]
pub struct bpp_regs {
    /* DMA registers */
    pub p_csr: u32,    /* DMA Control/Status Register */
    pub p_addr: u32,   /* Address Register */
    pub p_bcnt: u32,   /* Byte Count Register */
    pub p_tst_csr: u32, /* Test Control/Status (DMA2 only) */
    /* Parallel Port registers */
    pub p_hcr: u16,    /* Hardware Configuration Register */
    pub p_ocr: u16,    /* Operation Configuration Register */
    pub p_dr: u8,      /* Parallel Data Register */
    pub p_tcr: u8,     /* Transfer Control Register */
    pub p_or: u8,      /* Output Register */
    pub p_ir: u8,      /* Input Register */
    pub p_icr: u16,    /* Interrupt Control Register */
}

/* P_HCR. Time is in increments of SBus clock. */
pub const P_HCR_TEST: u32 = 0x8000; /* Allows buried counters to be read */
pub const P_HCR_DSW: u32 = 0x7f00; /* Data strobe width (in ticks) */
pub const P_HCR_DDS: u32 = 0x007f; /* Data setup before strobe (in ticks) */

/* P_OCR. */
pub const P_OCR_MEM_CLR: u32 = 0x8000;
pub const P_OCR_DATA_SRC: u32 = 0x4000; /* )                  */
pub const P_OCR_DS_DSEL: u32 = 0x2000; /* )  Bidirectional      */
pub const P_OCR_BUSY_DSEL: u32 = 0x1000; /* )    selects            */
pub const P_OCR_ACK_DSEL: u32 = 0x0800; /* )                  */
pub const P_OCR_EN_DIAG: u32 = 0x0400;
pub const P_OCR_BUSY_OP: u32 = 0x0200; /* Busy operation */
pub const P_OCR_ACK_OP: u32 = 0x0100; /* Ack operation */
pub const P_OCR_SRST: u32 = 0x0080; /* Reset state machines. Not selfcleaning. */
pub const P_OCR_IDLE: u32 = 0x0008; /* PP data transfer state machine is idle */
pub const P_OCR_V_ILCK: u32 = 0x0002; /* Versatec faded. Zebra only. */
pub const P_OCR_EN_VER: u32 = 0x0001; /* Enable Versatec (0 - enable). Zebra only. */

/* P_TCR */
pub const P_TCR_DIR: u32 = 0x08;
pub const P_TCR_BUSY: u32 = 0x04;
pub const P_TCR_ACK: u32 = 0x02;
pub const P_TCR_DS: u32 = 0x01; /* Strobe */

/* P_OR */
pub const P_OR_V3: u32 = 0x20; /* )                 */
pub const P_OR_V2: u32 = 0x10; /* ) on Zebra only   */
pub const P_OR_V1: u32 = 0x08; /* )                 */
pub const P_OR_INIT: u32 = 0x04;
pub const P_OR_AFXN: u32 = 0x02; /* Auto Feed */
pub const P_OR_SLCT_IN: u32 = 0x01;

/* P_IR */
pub const P_IR_PE: u32 = 0x04;
pub const P_IR_SLCT: u32 = 0x02;
pub const P_IR_ERR: u32 = 0x01;

/* P_ICR */
pub const P_DS_IRQ: u32 = 0x8000; /* RW1  */
pub const P_ACK_IRQ: u32 = 0x4000; /* RW1  */
pub const P_BUSY_IRQ: u32 = 0x2000; /* RW1  */
pub const P_PE_IRQ: u32 = 0x1000; /* RW1  */
pub const P_SLCT_IRQ: u32 = 0x0800; /* RW1  */
pub const P_ERR_IRQ: u32 = 0x0400; /* RW1  */
pub const P_DS_IRQ_EN: u32 = 0x0200; /* RW   Always on rising edge */
pub const P_ACK_IRQ_EN: u32 = 0x0100; /* RW   Always on rising edge */
pub const P_BUSY_IRP: u32 = 0x0080; /* RW   1= rising edge */
pub const P_BUSY_IRQ_EN: u32 = 0x0040; /* RW   */
pub const P_PE_IRP: u32 = 0x0020; /* RW   1= rising edge */
pub const P_PE_IRQ_EN: u32 = 0x0010; /* RW   */
pub const P_SLCT_IRP: u32 = 0x0008; /* RW   1= rising edge */
pub const P_SLCT_IRQ_EN: u32 = 0x0004; /* RW   */
pub const P_ERR_IRP: u32 = 0x0002; /* RW1  1= rising edge */
pub const P_ERR_IRQ_EN: u32 = 0x0001; /* RW   */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
