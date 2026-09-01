/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Sound Cors PDAudioCF soundcard
 *
 * Copyright (c) 2003 by Jaroslav Kysela <perex@perex.cz>
 */

/* Original C header dependencies:
 * <sound/pcm.h>, <linux/io.h>, <linux/interrupt.h>,
 * <pcmcia/cistpl.h>, <pcmcia/ds.h>, <sound/ak4117.h>
 */

use core::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

/* PDAUDIOCF registers */
pub const PDAUDIOCF_REG_MD: c_uchar = 0x00; /* music data, R/O */
pub const PDAUDIOCF_REG_WDP: c_uchar = 0x02; /* write data pointer / 2, R/O */
pub const PDAUDIOCF_REG_RDP: c_uchar = 0x04; /* read data pointer / 2, R/O */
pub const PDAUDIOCF_REG_TCR: c_uchar = 0x06; /* test control register W/O */
pub const PDAUDIOCF_REG_SCR: c_uchar = 0x08; /* status and control, R/W (see bit description) */
pub const PDAUDIOCF_REG_ISR: c_uchar = 0x0a; /* interrupt status, R/O */
pub const PDAUDIOCF_REG_IER: c_uchar = 0x0c; /* interrupt enable, R/W */
pub const PDAUDIOCF_REG_AK_IFR: c_uchar = 0x0e; /* AK interface register, R/W */

/* PDAUDIOCF_REG_TCR */
pub const PDAUDIOCF_ELIMAKMBIT: c_ushort = 1 << 0; /* simulate AKM music data */
pub const PDAUDIOCF_TESTDATASEL: c_ushort = 1 << 1; /* test data selection, 0 = 0x55, 1 = pseudo-random */

/* PDAUDIOCF_REG_SCR */
pub const PDAUDIOCF_AK_SBP: c_ushort = 1 << 0; /* serial port busy flag */
pub const PDAUDIOCF_RST: c_ushort = 1 << 2; /* FPGA, AKM + SRAM buffer reset */
pub const PDAUDIOCF_PDN: c_ushort = 1 << 3; /* power down bit */
pub const PDAUDIOCF_CLKDIV0: c_ushort = 1 << 4; /* choose 24.576Mhz clock divided by 1,2,3 or 4 */
pub const PDAUDIOCF_CLKDIV1: c_ushort = 1 << 5;
pub const PDAUDIOCF_RECORD: c_ushort = 1 << 6; /* start capturing to SRAM */
pub const PDAUDIOCF_AK_SDD: c_ushort = 1 << 7; /* music data detected */
pub const PDAUDIOCF_RED_LED_OFF: c_ushort = 1 << 8; /* red LED off override */
pub const PDAUDIOCF_BLUE_LED_OFF: c_ushort = 1 << 9; /* blue LED off override */
pub const PDAUDIOCF_DATAFMT0: c_ushort = 1 << 10; /* data format bits: 00 = 16-bit, 01 = 18-bit */
pub const PDAUDIOCF_DATAFMT1: c_ushort = 1 << 11; /* 10 = 20-bit, 11 = 24-bit, all right justified */
pub const fn PDAUDIOCF_FPGAREV(x: c_ushort) -> c_ushort {
    (x >> 12) & 0x0f
} /* FPGA revision */

/* PDAUDIOCF_REG_ISR */
pub const PDAUDIOCF_IRQLVL: c_ushort = 1 << 0; /* Buffer level IRQ */
pub const PDAUDIOCF_IRQOVR: c_ushort = 1 << 1; /* Overrun IRQ */
pub const PDAUDIOCF_IRQAKM: c_ushort = 1 << 2; /* AKM IRQ */

/* PDAUDIOCF_REG_IER */
pub const PDAUDIOCF_IRQLVLEN0: c_ushort = 1 << 0; /* fill threshold levels; 00 = none, 01 = 1/8th of buffer */
pub const PDAUDIOCF_IRQLVLEN1: c_ushort = 1 << 1; /* 10 = 1/4th of buffer, 11 = 1/2th of buffer */
pub const PDAUDIOCF_IRQOVREN: c_ushort = 1 << 2; /* enable overrun IRQ */
pub const PDAUDIOCF_IRQAKMEN: c_ushort = 1 << 3; /* enable AKM IRQ */
pub const PDAUDIOCF_BLUEDUTY0: c_ushort = 1 << 8; /* blue LED duty cycle; 00 = 100%, 01 = 50% */
pub const PDAUDIOCF_BLUEDUTY1: c_ushort = 1 << 9; /* 02 = 25%, 11 = 12% */
pub const PDAUDIOCF_REDDUTY0: c_ushort = 1 << 10; /* red LED duty cycle; 00 = 100%, 01 = 50% */
pub const PDAUDIOCF_REDDUTY1: c_ushort = 1 << 11; /* 02 = 25%, 11 = 12% */
pub const PDAUDIOCF_BLUESDD: c_ushort = 1 << 12; /* blue LED against SDD bit */
pub const PDAUDIOCF_BLUEMODULATE: c_ushort = 1 << 13; /* save power when 100% duty cycle selected */
pub const PDAUDIOCF_REDMODULATE: c_ushort = 1 << 14; /* save power when 100% duty cycle selected */
pub const PDAUDIOCF_HALFRATE: c_ushort = 1 << 15; /* slow both LED blinks by half (also spdif detect rate) */

/* chip status */
pub const PDAUDIOCF_STAT_IS_STALE: c_uint = 1 << 0;
pub const PDAUDIOCF_STAT_IS_CONFIGURED: c_uint = 1 << 1;
pub const PDAUDIOCF_STAT_IS_SUSPENDED: c_uint = 1 << 2;

#[repr(C)]
pub struct snd_pdacf {
    pub card: *mut snd_card,
    pub index: c_int,

    pub port: c_ulong,
    pub irq: c_int,

    pub reg_lock: mutex,
    pub regmap: [c_ushort; 8],
    pub suspend_reg_scr: c_ushort,

    pub ak4117_lock: spinlock_t,
    pub ak4117: *mut ak4117,

    pub chip_status: c_uint,

    pub pcm: *mut snd_pcm,
    pub pcm_substream: *mut snd_pcm_substream,
    /* C bit-field: unsigned int pcm_running: 1; */
    pub pcm_running: c_uint,
    pub pcm_channels: c_uint,
    pub pcm_swab: c_uint,
    pub pcm_little: c_uint,
    pub pcm_frame: c_uint,
    pub pcm_sample: c_uint,
    pub pcm_xor: c_uint,
    pub pcm_size: c_uint,
    pub pcm_period: c_uint,
    pub pcm_tdone: c_uint,
    pub pcm_hwptr: c_uint,
    pub pcm_area: *mut c_void,

    /* pcmcia stuff */
    pub p_dev: *mut pcmcia_device,
}

#[inline]
pub unsafe fn pdacf_reg_write(chip: *mut snd_pdacf, reg: c_uchar, val: c_ushort) {
    unsafe {
        (*chip).regmap[(reg >> 1) as usize] = val;
        outw(val, (*chip).port.wrapping_add(reg as c_ulong));
    }
}

#[inline]
pub unsafe fn pdacf_reg_read(chip: *mut snd_pdacf, reg: c_uchar) -> c_ushort {
    unsafe { inw((*chip).port.wrapping_add(reg as c_ulong)) }
}

unsafe extern "C" {
    pub fn snd_pdacf_create(card: *mut snd_card) -> *mut snd_pdacf;
    pub fn snd_pdacf_ak4117_create(pdacf: *mut snd_pdacf) -> c_int;
    pub fn snd_pdacf_powerdown(chip: *mut snd_pdacf);

    /* CONFIG_PM conditional declarations in the original C header. */
    pub fn snd_pdacf_suspend(chip: *mut snd_pdacf) -> c_int;
    pub fn snd_pdacf_resume(chip: *mut snd_pdacf) -> c_int;

    pub fn snd_pdacf_pcm_new(chip: *mut snd_pdacf) -> c_int;
    pub fn pdacf_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t;
    pub fn pdacf_threaded_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t;
    pub fn pdacf_reinit(chip: *mut snd_pdacf, resume: c_int);

    pub fn outw(value: c_ushort, port: c_ulong);
    pub fn inw(port: c_ulong) -> c_ushort;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
