// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/sh/kernel/adc.c -- SH3 on-chip ADC support
 *
 *  Copyright (C) 2004  Andriy Skulysh <askulysh@image.kiev.ua>
 */

// Dependency declarations corresponding to <linux/module.h>, <asm/adc.h>,
// and <asm/io.h>.  The address and bit definitions are supplied externally.
extern "C" {
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_writeb(value: u8, addr: usize);

    static ADCSR: usize;
    static ADDRAH: usize;
    static ADDRAL: usize;
    static ADCSR_ADST: u8;
    static ADCSR_CKS: u8;
    static ADCSR_ADF: u8;
}

pub unsafe fn adc_single(channel: u32) -> i32 {
    let off: i32;
    let mut csr: u8;

    if channel >= 8 {
        return -1;
    }

    off = ((channel & 0x03) << 2) as i32;

    csr = __raw_readb(ADCSR);
    csr = channel as u8 | ADCSR_ADST | ADCSR_CKS;
    __raw_writeb(csr, ADCSR);

    loop {
        csr = __raw_readb(ADCSR);
        if (csr & ADCSR_ADF) != 0 {
            break;
        }
    }

    csr &= !(ADCSR_ADF | ADCSR_ADST);
    __raw_writeb(csr, ADCSR);

    ((((__raw_readb(ADDRAH + off as usize) as i32) << 8)
        | __raw_readb(ADDRAL + off as usize) as i32)
        >> 6)
}

// EXPORT_SYMBOL(adc_single);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
