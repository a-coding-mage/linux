/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Apple Peripheral System Controller (PSC)
 *
 * The PSC is used on the AV Macs to control IO functions not handled
 * by the VIAs (Ethernet, DSP, SCC, Sound). This includes nine DMA
 * channels. See the original header for the hardware notes and register
 * descriptions.
 */

pub const PSC_BASE: u32 = 0x50F31000;

/* IER/IFR register bases; add 0xn0 to the base for register sets 3-6. */
pub const pIFRbase: u32 = 0x100;
pub const pIERbase: u32 = 0x104;

/* One-shot DMA control registers. */
pub const PSC_MYSTERY: u32 = 0x804;
pub const PSC_CTL_BASE: u32 = 0xC00;
pub const PSC_SCSI_CTL: u32 = 0xC00;
pub const PSC_ENETRD_CTL: u32 = 0xC10;
pub const PSC_ENETWR_CTL: u32 = 0xC20;
pub const PSC_FDC_CTL: u32 = 0xC30;
pub const PSC_SCCA_CTL: u32 = 0xC40;
pub const PSC_SCCB_CTL: u32 = 0xC50;
pub const PSC_SCCATX_CTL: u32 = 0xC60;

/* DMA channels; add 0x10 for the second channel in each set. */
pub const PSC_ADDR_BASE: u32 = 0x1000;
pub const PSC_LEN_BASE: u32 = 0x1004;
pub const PSC_CMD_BASE: u32 = 0x1008;
pub const PSC_SET0: u32 = 0x00;
pub const PSC_SET1: u32 = 0x10;

pub const PSC_SCSI_ADDR: u32 = 0x1000; /* confirmed */
pub const PSC_SCSI_LEN: u32 = 0x1004; /* confirmed */
pub const PSC_SCSI_CMD: u32 = 0x1008; /* confirmed */
pub const PSC_ENETRD_ADDR: u32 = 0x1020; /* confirmed */
pub const PSC_ENETRD_LEN: u32 = 0x1024; /* confirmed */
pub const PSC_ENETRD_CMD: u32 = 0x1028; /* confirmed */
pub const PSC_ENETWR_ADDR: u32 = 0x1040; /* confirmed */
pub const PSC_ENETWR_LEN: u32 = 0x1044; /* confirmed */
pub const PSC_ENETWR_CMD: u32 = 0x1048; /* confirmed */
pub const PSC_FDC_ADDR: u32 = 0x1060; /* strongly suspected */
pub const PSC_FDC_LEN: u32 = 0x1064; /* strongly suspected */
pub const PSC_FDC_CMD: u32 = 0x1068; /* strongly suspected */
pub const PSC_SCCA_ADDR: u32 = 0x1080; /* confirmed */
pub const PSC_SCCA_LEN: u32 = 0x1084; /* confirmed */
pub const PSC_SCCA_CMD: u32 = 0x1088; /* confirmed */
pub const PSC_SCCB_ADDR: u32 = 0x10A0; /* confirmed */
pub const PSC_SCCB_LEN: u32 = 0x10A4; /* confirmed */
pub const PSC_SCCB_CMD: u32 = 0x10A8; /* confirmed */
pub const PSC_SCCATX_ADDR: u32 = 0x10C0; /* confirmed */
pub const PSC_SCCATX_LEN: u32 = 0x10C4; /* confirmed */
pub const PSC_SCCATX_CMD: u32 = 0x10C8; /* confirmed */

/* Free-running sound DMA registers. */
pub const PSC_SND_CTL: u32 = 0x200;
pub const PSC_SND_SOURCE: u32 = 0x204;
pub const PSC_SND_STATUS1: u32 = 0x208;
pub const PSC_SND_HUH3: u32 = 0x20C;
pub const PSC_SND_BITS2GO: u32 = 0x20E;
pub const PSC_SND_INADDR: u32 = 0x210;
pub const PSC_SND_OUTADDR: u32 = 0x214;
pub const PSC_SND_LEN: u32 = 0x218;
pub const PSC_SND_HUH4: u32 = 0x21A;
pub const PSC_SND_STATUS2: u32 = 0x21C;
pub const PSC_SND_HUH5: u32 = 0x21E;

extern "C" {
    pub static mut psc: *mut u8;
    pub fn psc_register_interrupts();
    pub fn psc_irq_enable(irq: i32);
    pub fn psc_irq_disable(irq: i32);
}

/* Access functions. */
#[inline]
pub unsafe fn psc_write_byte(offset: i32, data: u8) {
    core::ptr::write_volatile(psc.add(offset as usize), data);
}

#[inline]
pub unsafe fn psc_write_word(offset: i32, data: u16) {
    core::ptr::write_volatile(psc.add(offset as usize) as *mut u16, data);
}

#[inline]
pub unsafe fn psc_write_long(offset: i32, data: u32) {
    core::ptr::write_volatile(psc.add(offset as usize) as *mut u32, data);
}

#[inline]
pub unsafe fn psc_read_byte(offset: i32) -> u8 {
    core::ptr::read_volatile(psc.add(offset as usize))
}

#[inline]
pub unsafe fn psc_read_word(offset: i32) -> u16 {
    core::ptr::read_volatile(psc.add(offset as usize) as *const u16)
}

#[inline]
pub unsafe fn psc_read_long(offset: i32) -> u32 {
    core::ptr::read_volatile(psc.add(offset as usize) as *const u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
