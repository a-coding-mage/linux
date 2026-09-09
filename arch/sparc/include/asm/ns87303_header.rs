/* SPDX-License-Identifier: GPL-2.0 */
/* ns87303.h: Configuration Register Description for the
 *            National Semiconductor PC87303 (SuperIO).
 *
 * Copyright (C) 1997  Eddie C. Dost  (ecd@skynet.be)
 */

/* Control Register Index Values */
pub const FER: u8 = 0x00;
pub const FAR: u8 = 0x01;
pub const PTR: u8 = 0x02;
pub const FCR: u8 = 0x03;
pub const PCR: u8 = 0x04;
pub const KRR: u8 = 0x05;
pub const PMC: u8 = 0x06;
pub const TUP: u8 = 0x07;
pub const SID: u8 = 0x08;
pub const ASC: u8 = 0x09;
pub const CS0CF0: u8 = 0x0a;
pub const CS0CF1: u8 = 0x0b;
pub const CS1CF0: u8 = 0x0c;
pub const CS1CF1: u8 = 0x0d;

/* Function Enable Register (FER) bits */
pub const FER_EDM: u8 = 0x10;

/* Function Address Register (FAR) bits */
pub const FAR_LPT_MASK: u8 = 0x03;
pub const FAR_LPTB: u8 = 0x00;
pub const FAR_LPTA: u8 = 0x01;
pub const FAR_LPTC: u8 = 0x02;

/* Power and Test Register (PTR) bits */
pub const PTR_LPTB_IRQ7: u8 = 0x08;
pub const PTR_LEVEL_IRQ: u8 = 0x80;
pub const PTR_LPT_REG_DIR: u8 = 0x80;

/* Function Control Register (FCR) bits */
pub const FCR_LDE: u8 = 0x10;
pub const FCR_ZWS_ENA: u8 = 0x20;

/* Printer Control Register (PCR) bits */
pub const PCR_EPP_ENABLE: u8 = 0x01;
pub const PCR_EPP_IEEE: u8 = 0x02;
pub const PCR_ECP_ENABLE: u8 = 0x04;
pub const PCR_ECP_CLK_ENA: u8 = 0x08;
pub const PCR_IRQ_POLAR: u8 = 0x20;
pub const PCR_IRQ_ODRAIN: u8 = 0x40;

/* Tape UARTs and Parallel Port Config Register (TUP) bits */
pub const TUP_EPP_TIMO: u8 = 0x02;

/* Advanced SuperIO Config Register (ASC) bits */
pub const ASC_LPT_IRQ7: u8 = 0x01;
pub const ASC_DRV2_SEL: u8 = 0x02;

pub const FER_RESERVED: u8 = 0x00;
pub const FAR_RESERVED: u8 = 0x00;
pub const PTR_RESERVED: u8 = 0x73;
pub const FCR_RESERVED: u8 = 0xc4;
pub const PCR_RESERVED: u8 = 0x10;
pub const KRR_RESERVED: u8 = 0x00;
pub const PMC_RESERVED: u8 = 0x98;
pub const TUP_RESERVED: u8 = 0xfb;
pub const SIP_RESERVED: u8 = 0x00;
pub const ASC_RESERVED: u8 = 0x18;
pub const CS0CF0_RESERVED: u8 = 0x00;
pub const CS0CF1_RESERVED: u8 = 0x08;
pub const CS1CF0_RESERVED: u8 = 0x00;
pub const CS1CF1_RESERVED: u8 = 0x08;

/* The following declarations and inline function are present only under __KERNEL__. */
#[cfg(feature = "kernel")]
extern "C" {
    pub static mut ns87303_lock: spinlock_t;
    fn outb(value: u8, port: usize);
    fn inb(port: usize) -> u8;
}

#[cfg(feature = "kernel")]
#[inline]
pub unsafe fn ns87303_modify(port: usize, index: u32, clr: u8, set: u8) -> i32 {
    const RESERVED: [u8; 14] = [
        FER_RESERVED, FAR_RESERVED, PTR_RESERVED, FCR_RESERVED,
        PCR_RESERVED, KRR_RESERVED, PMC_RESERVED, TUP_RESERVED,
        SIP_RESERVED, ASC_RESERVED, CS0CF0_RESERVED, CS0CF1_RESERVED,
        CS1CF0_RESERVED, CS1CF1_RESERVED,
    ];
    if index > 0x0d {
        return -EINVAL;
    }

    let _flags: usize = 0;
    spin_lock_irqsave(&raw mut ns87303_lock, &raw mut _flags);
    outb(index as u8, port);
    let mut value = inb(port + 1);
    value &= !(RESERVED[index as usize] | clr);
    value |= set;
    outb(value, port + 1);
    outb(value, port + 1);
    spin_unlock_irqrestore(&raw mut ns87303_lock, _flags);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
