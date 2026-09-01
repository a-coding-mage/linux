// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Sound Core PDAudioCF soundcard
 *
 * Copyright (c) 2003 by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies:
// <linux/delay.h>, <linux/slab.h>, <sound/core.h>, <sound/info.h>,
// "pdaudiocf.h", <sound/initval.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type c_int = i32;
type c_uint = u32;
type c_uchar = u8;
type c_ushort = u16;
type c_ulong = u64;
type c_void = core::ffi::c_void;
type u16 = c_ushort;

#[repr(C)]
pub struct snd_pdacf {
    pub card: *mut snd_card,
    pub reg_lock: mutex,
    pub ak4117_lock: spinlock_t,
    pub port: c_ulong,
    pub regmap: [u16; 0],
    pub suspend_reg_scr: u16,
    pub ak4117: *mut ak4117,
    pub chip_status: c_uint,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ak4117 {
    pub change_callback_private: *mut c_void,
    pub change_callback: Option<unsafe extern "C" fn(*mut ak4117, c_uchar, c_uchar)>,
    pub rcs0: c_uchar,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    static PDAUDIOCF_REG_SCR: c_ulong;
    static PDAUDIOCF_REG_AK_IFR: c_ulong;
    static PDAUDIOCF_REG_WDP: c_ulong;
    static PDAUDIOCF_REG_RDP: c_ulong;
    static PDAUDIOCF_REG_TCR: c_ulong;
    static PDAUDIOCF_REG_ISR: c_ulong;
    static PDAUDIOCF_REG_IER: c_ulong;

    static PDAUDIOCF_AK_SBP: u16;
    static PDAUDIOCF_PDN: u16;
    static PDAUDIOCF_RECORD: u16;
    static PDAUDIOCF_RST: u16;
    static PDAUDIOCF_BLUE_LED_OFF: u16;
    static PDAUDIOCF_RED_LED_OFF: u16;
    static PDAUDIOCF_ELIMAKMBIT: u16;
    static PDAUDIOCF_TESTDATASEL: u16;
    static PDAUDIOCF_CLKDIV0: u16;
    static PDAUDIOCF_CLKDIV1: u16;
    static PDAUDIOCF_DATAFMT0: u16;
    static PDAUDIOCF_DATAFMT1: u16;
    static PDAUDIOCF_IRQLVLEN0: u16;
    static PDAUDIOCF_IRQLVLEN1: u16;
    static PDAUDIOCF_BLUEDUTY0: u16;
    static PDAUDIOCF_REDDUTY0: u16;
    static PDAUDIOCF_REDDUTY1: u16;
    static PDAUDIOCF_BLUEDUTY1: u16;
    static PDAUDIOCF_HALFRATE: u16;
    static PDAUDIOCF_IRQOVREN: u16;
    static PDAUDIOCF_IRQAKMEN: u16;
    static PDAUDIOCF_STAT_IS_SUSPENDED: c_uint;

    static AK4117_UNLCK: c_uchar;
    static AK4117_XTL_24_576M: c_uchar;
    static AK4117_EXCT: c_uchar;
    static AK4117_CM_PLL_XTAL: c_uchar;
    static AK4117_PKCS_128fs: c_uchar;
    static AK4117_XCKS_128fs: c_uchar;
    static AK4117_EFH_1024LRCLK: c_uchar;
    static AK4117_DIF_24R: c_uchar;
    static AK4117_IPS: c_uchar;
    static AK4117_MAUTO: c_uchar;
    static AK4117_MAUD: c_uchar;
    static AK4117_MULK: c_uchar;
    static AK4117_MPAR: c_uchar;
    static AK4117_MV: c_uchar;

    static SNDRV_CTL_POWER_D3hot: c_int;
    static SNDRV_CTL_POWER_D0: c_int;

    fn pdacf_reg_read(chip: *mut snd_pdacf, reg: c_ulong) -> u16;
    fn pdacf_reg_write(chip: *mut snd_pdacf, reg: c_ulong, val: u16);
    fn inw(port: c_ulong) -> u16;
    fn outw(val: u16, port: c_ulong);
    fn udelay(usecs: c_ulong);
    fn mdelay(msecs: c_ulong);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const u8, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const u8,
        private_data: *mut snd_pdacf,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn kzalloc_obj_snd_pdacf() -> *mut snd_pdacf;
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_ak4117_reinit(ak4117: *mut ak4117);
    fn snd_ak4117_create(
        card: *mut snd_card,
        read: Option<unsafe extern "C" fn(*mut c_void, c_uchar) -> c_uchar>,
        write: Option<unsafe extern "C" fn(*mut c_void, c_uchar, c_uchar)>,
        pgm: *const c_uchar,
        private_data: *mut snd_pdacf,
        ak4117: *mut *mut ak4117,
    ) -> c_int;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_ak4117_external_rate(ak4117: *mut ak4117) -> c_int;
    fn PDAUDIOCF_FPGAREV(val: u16) -> u16;
}

/*
 *
 */
unsafe extern "C" fn pdacf_ak4117_read(private_data: *mut c_void, reg: c_uchar) -> c_uchar {
    let chip: *mut snd_pdacf = private_data as *mut snd_pdacf;
    let mut timeout: c_ulong;
    let mut flags: c_ulong = 0;
    let res: c_uchar;

    spin_lock_irqsave(core::ptr::addr_of_mut!((*chip).ak4117_lock), &mut flags);
    timeout = 1000;
    while (pdacf_reg_read(chip, PDAUDIOCF_REG_SCR) & PDAUDIOCF_AK_SBP) != 0 {
        udelay(5);
        timeout = timeout.wrapping_sub(1);
        if timeout == 0 {
            spin_unlock_irqrestore(core::ptr::addr_of_mut!((*chip).ak4117_lock), flags);
            dev_err((*(*chip).card).dev, b"AK4117 ready timeout (read)\n\0".as_ptr());
            return 0;
        }
    }
    pdacf_reg_write(chip, PDAUDIOCF_REG_AK_IFR, (reg as u16) << 8);
    timeout = 1000;
    while (pdacf_reg_read(chip, PDAUDIOCF_REG_SCR) & PDAUDIOCF_AK_SBP) != 0 {
        udelay(5);
        timeout = timeout.wrapping_sub(1);
        if timeout == 0 {
            spin_unlock_irqrestore(core::ptr::addr_of_mut!((*chip).ak4117_lock), flags);
            dev_err((*(*chip).card).dev, b"AK4117 read timeout (read2)\n\0".as_ptr());
            return 0;
        }
    }
    res = pdacf_reg_read(chip, PDAUDIOCF_REG_AK_IFR) as c_uchar;
    spin_unlock_irqrestore(core::ptr::addr_of_mut!((*chip).ak4117_lock), flags);
    res
}

unsafe extern "C" fn pdacf_ak4117_write(
    private_data: *mut c_void,
    reg: c_uchar,
    val: c_uchar,
) {
    let chip: *mut snd_pdacf = private_data as *mut snd_pdacf;
    let mut timeout: c_ulong;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(core::ptr::addr_of_mut!((*chip).ak4117_lock), &mut flags);
    timeout = 1000;
    while (inw((*chip).port + PDAUDIOCF_REG_SCR) & PDAUDIOCF_AK_SBP) != 0 {
        udelay(5);
        timeout = timeout.wrapping_sub(1);
        if timeout == 0 {
            spin_unlock_irqrestore(core::ptr::addr_of_mut!((*chip).ak4117_lock), flags);
            dev_err((*(*chip).card).dev, b"AK4117 ready timeout (write)\n\0".as_ptr());
            return;
        }
    }
    outw(((reg as u16) << 8) | val as u16 | (1u16 << 13), (*chip).port + PDAUDIOCF_REG_AK_IFR);
    spin_unlock_irqrestore(core::ptr::addr_of_mut!((*chip).ak4117_lock), flags);
}

// Disabled C debug block preserved from `#if 0`.
#[cfg(any())]
unsafe extern "C" fn pdacf_dump(chip: *mut snd_pdacf) {
    dev_dbg((*(*chip).card).dev, b"PDAUDIOCF DUMP (0x%lx):\n\0".as_ptr(), (*chip).port);
    dev_dbg((*(*chip).card).dev, b"WPD         : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_WDP));
    dev_dbg((*(*chip).card).dev, b"RDP         : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_RDP));
    dev_dbg((*(*chip).card).dev, b"TCR         : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_TCR));
    dev_dbg((*(*chip).card).dev, b"SCR         : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_SCR));
    dev_dbg((*(*chip).card).dev, b"ISR         : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_ISR));
    dev_dbg((*(*chip).card).dev, b"IER         : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_IER));
    dev_dbg((*(*chip).card).dev, b"AK_IFR      : 0x%x\n\0".as_ptr(), inw((*chip).port + PDAUDIOCF_REG_AK_IFR));
}

unsafe extern "C" fn pdacf_reset(chip: *mut snd_pdacf, powerdown: c_int) -> c_int {
    let mut val: u16;

    val = pdacf_reg_read(chip, PDAUDIOCF_REG_SCR);
    val |= PDAUDIOCF_PDN;
    val &= !PDAUDIOCF_RECORD; /* for sure */
    pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);
    udelay(5);
    val |= PDAUDIOCF_RST;
    pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);
    udelay(200);
    val &= !PDAUDIOCF_RST;
    pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);
    udelay(5);
    if powerdown == 0 {
        val &= !PDAUDIOCF_PDN;
        pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);
        udelay(200);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn pdacf_reinit(chip: *mut snd_pdacf, resume: c_int) {
    pdacf_reset(chip, 0);
    if resume != 0 {
        pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, (*chip).suspend_reg_scr);
    }
    snd_ak4117_reinit((*chip).ak4117);
    pdacf_reg_write(
        chip,
        PDAUDIOCF_REG_TCR,
        *(*chip).regmap.as_ptr().offset((PDAUDIOCF_REG_TCR >> 1) as isize),
    );
    pdacf_reg_write(
        chip,
        PDAUDIOCF_REG_IER,
        *(*chip).regmap.as_ptr().offset((PDAUDIOCF_REG_IER >> 1) as isize),
    );
}

unsafe extern "C" fn pdacf_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip: *mut snd_pdacf = (*entry).private_data as *mut snd_pdacf;
    let mut tmp: u16;

    snd_iprintf(buffer, b"PDAudioCF\n\n\0".as_ptr());
    tmp = pdacf_reg_read(chip, PDAUDIOCF_REG_SCR);
    snd_iprintf(
        buffer,
        b"FPGA revision      : 0x%x\n\0".as_ptr(),
        PDAUDIOCF_FPGAREV(tmp) as c_uint,
    );
}

unsafe extern "C" fn pdacf_proc_init(chip: *mut snd_pdacf) {
    snd_card_ro_proc_new((*chip).card, b"pdaudiocf\0".as_ptr(), chip, Some(pdacf_proc_read));
}

#[no_mangle]
pub unsafe extern "C" fn snd_pdacf_create(card: *mut snd_card) -> *mut snd_pdacf {
    let chip: *mut snd_pdacf;

    chip = kzalloc_obj_snd_pdacf();
    if chip.is_null() {
        return core::ptr::null_mut();
    }
    (*chip).card = card;
    mutex_init(core::ptr::addr_of_mut!((*chip).reg_lock));
    spin_lock_init(core::ptr::addr_of_mut!((*chip).ak4117_lock));
    (*card).private_data = chip as *mut c_void;

    pdacf_proc_init(chip);
    chip
}

unsafe extern "C" fn snd_pdacf_ak4117_change(
    ak4117: *mut ak4117,
    c0: c_uchar,
    _c1: c_uchar,
) {
    let chip: *mut snd_pdacf = (*ak4117).change_callback_private as *mut snd_pdacf;
    let mut val: u16;

    if (c0 & AK4117_UNLCK) == 0 {
        return;
    }
    mutex_lock(core::ptr::addr_of_mut!((*chip).reg_lock));
    val = *(*chip).regmap.as_ptr().offset((PDAUDIOCF_REG_SCR >> 1) as isize);
    if ((*ak4117).rcs0 & AK4117_UNLCK) != 0 {
        val |= PDAUDIOCF_BLUE_LED_OFF;
    } else {
        val &= !PDAUDIOCF_BLUE_LED_OFF;
    }
    pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);
    mutex_unlock(core::ptr::addr_of_mut!((*chip).reg_lock));
}

#[no_mangle]
pub unsafe extern "C" fn snd_pdacf_ak4117_create(chip: *mut snd_pdacf) -> c_int {
    let mut err: c_int;
    let mut val: u16;
    /* design note: if we unmask PLL unlock, parity, valid, audio or auto bit interrupts */
    /* from AK4117 then INT1 pin from AK4117 will be high all time, because PCMCIA interrupts are */
    /* egde based and FPGA does logical OR for all interrupt sources, we cannot use these */
    /* high-rate sources */
    let pgm: [c_uchar; 5] = [
        AK4117_XTL_24_576M | AK4117_EXCT, /* AK4117_REG_PWRDN */
        AK4117_CM_PLL_XTAL | AK4117_PKCS_128fs | AK4117_XCKS_128fs, /* AK4117_REQ_CLOCK */
        AK4117_EFH_1024LRCLK | AK4117_DIF_24R | AK4117_IPS, /* AK4117_REG_IO */
        0xff, /* AK4117_REG_INT0_MASK */
        AK4117_MAUTO | AK4117_MAUD | AK4117_MULK | AK4117_MPAR | AK4117_MV, /* AK4117_REG_INT1_MASK */
    ];

    err = pdacf_reset(chip, 0);
    if err < 0 {
        return err;
    }
    err = snd_ak4117_create(
        (*chip).card,
        Some(pdacf_ak4117_read),
        Some(pdacf_ak4117_write),
        pgm.as_ptr(),
        chip,
        core::ptr::addr_of_mut!((*chip).ak4117),
    );
    if err < 0 {
        return err;
    }

    val = pdacf_reg_read(chip, PDAUDIOCF_REG_TCR);
    // `#if 1 /* normal operation */`
    val &= !(PDAUDIOCF_ELIMAKMBIT | PDAUDIOCF_TESTDATASEL);
    // `#else /* debug */`
    // val |= PDAUDIOCF_ELIMAKMBIT;
    // val &= !PDAUDIOCF_TESTDATASEL;
    pdacf_reg_write(chip, PDAUDIOCF_REG_TCR, val);

    /* setup the FPGA to match AK4117 setup */
    val = pdacf_reg_read(chip, PDAUDIOCF_REG_SCR);
    val &= !(PDAUDIOCF_CLKDIV0 | PDAUDIOCF_CLKDIV1); /* use 24.576Mhz clock */
    val &= !(PDAUDIOCF_RED_LED_OFF | PDAUDIOCF_BLUE_LED_OFF);
    val |= PDAUDIOCF_DATAFMT0 | PDAUDIOCF_DATAFMT1; /* 24-bit data */
    pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);

    /* setup LEDs and IRQ */
    val = pdacf_reg_read(chip, PDAUDIOCF_REG_IER);
    val &= !(PDAUDIOCF_IRQLVLEN0 | PDAUDIOCF_IRQLVLEN1);
    val &= !(PDAUDIOCF_BLUEDUTY0 | PDAUDIOCF_REDDUTY0 | PDAUDIOCF_REDDUTY1);
    val |= PDAUDIOCF_BLUEDUTY1 | PDAUDIOCF_HALFRATE;
    val |= PDAUDIOCF_IRQOVREN | PDAUDIOCF_IRQAKMEN;
    pdacf_reg_write(chip, PDAUDIOCF_REG_IER, val);

    (*(*chip).ak4117).change_callback_private = chip as *mut c_void;
    (*(*chip).ak4117).change_callback = Some(snd_pdacf_ak4117_change);

    /* update LED status */
    snd_pdacf_ak4117_change((*chip).ak4117, AK4117_UNLCK, 0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pdacf_powerdown(chip: *mut snd_pdacf) {
    let mut val: u16;

    val = pdacf_reg_read(chip, PDAUDIOCF_REG_SCR);
    (*chip).suspend_reg_scr = val;
    val |= PDAUDIOCF_RED_LED_OFF | PDAUDIOCF_BLUE_LED_OFF;
    pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, val);
    /* disable interrupts, but use direct write to preserve old register value in chip->regmap */
    val = inw((*chip).port + PDAUDIOCF_REG_IER);
    val &= !(PDAUDIOCF_IRQOVREN | PDAUDIOCF_IRQAKMEN | PDAUDIOCF_IRQLVLEN0 | PDAUDIOCF_IRQLVLEN1);
    outw(val, (*chip).port + PDAUDIOCF_REG_IER);
    pdacf_reset(chip, 1);
}

// `#ifdef CONFIG_PM`
#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn snd_pdacf_suspend(chip: *mut snd_pdacf) -> c_int {
    let mut val: u16;

    snd_power_change_state((*chip).card, SNDRV_CTL_POWER_D3hot);
    /* disable interrupts, but use direct write to preserve old register value in chip->regmap */
    val = inw((*chip).port + PDAUDIOCF_REG_IER);
    val &= !(PDAUDIOCF_IRQOVREN | PDAUDIOCF_IRQAKMEN | PDAUDIOCF_IRQLVLEN0 | PDAUDIOCF_IRQLVLEN1);
    outw(val, (*chip).port + PDAUDIOCF_REG_IER);
    (*chip).chip_status |= PDAUDIOCF_STAT_IS_SUSPENDED; /* ignore interrupts from now */
    snd_pdacf_powerdown(chip);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn check_signal(chip: *mut snd_pdacf) -> c_int {
    (((*(*chip).ak4117).rcs0 & AK4117_UNLCK) == 0) as c_int
}

#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn snd_pdacf_resume(chip: *mut snd_pdacf) -> c_int {
    let mut timeout: c_int = 40;

    pdacf_reinit(chip, 1);
    /* wait for AK4117's PLL */
    while {
        let cond = timeout > 0
            && (snd_ak4117_external_rate((*chip).ak4117) <= 0 || check_signal(chip) == 0);
        timeout -= 1;
        cond
    } {
        mdelay(1);
    }
    (*chip).chip_status &= !PDAUDIOCF_STAT_IS_SUSPENDED;
    snd_power_change_state((*chip).card, SNDRV_CTL_POWER_D0);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
