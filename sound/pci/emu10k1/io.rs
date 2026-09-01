// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Lee Revell <rlrevell@joe-job.com>
 *                   James Courtier-Dutton <James@superbug.co.uk>
 *                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
 *                   Creative Labs, Inc.
 *
 *  Routines for control of EMU10K1 chips
 */

// Dependencies from Linux, ALSA, emu10k1 headers, and p17v.h are expected
// to be supplied by the surrounding translation unit.

type u8 = core::ffi::c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

#[repr(C)]
pub struct snd_emu10k1 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut snd_emu10k1,
}

#[repr(C)]
pub struct firmware {
    pub size: usize,
    pub data: *const u8,
}

unsafe extern "C" {
    static PTR: u32;
    static DATA: u32;
    static PTR2: u32;
    static DATA2: u32;
    static INTE: u32;
    static A_GPIO: u32;
    static WC: u32;
    static AC97ADDRESS: u32;
    static AC97DATA: u32;

    static PTR_ADDRESS_MASK: u32;
    static A_PTR_ADDRESS_MASK: u32;
    static PTR_CHANNELNUM_MASK: u32;
    static REGLIST_END: u32;

    static P17V_SPI: u32;
    static P17V_I2C_1: u32;
    static P17V_I2C_ADDR: u32;
    static I2C_A_ADC_LAST: u32;
    static I2C_A_ADC_START: u32;
    static I2C_A_ADC_ADD: u32;
    static I2C_A_ADC_ABORT: u32;

    static EMU_HANA_DESTHI: u32;
    static EMU_HANA_DESTLO: u32;
    static EMU_HANA_SRCHI: u32;
    static EMU_HANA_SRCLO: u32;
    static EMU_HANA_WCLOCK_HANA_SPDIF_IN: u8;
    static EMU_HANA_WCLOCK_HANA_ADAT_IN: u8;
    static EMU_HANA_WCLOCK_SYNC_BNC: u8;
    static EMU_HANA_WCLOCK_2ND_HANA: u8;
    static EMU_HANA_SPDIF_MODE: u32;
    static EMU_HANA_SPDIF_MODE_RX_INVALID: u32;
    static EMU_HANA_WC_SPDIF_LO: u32;
    static EMU_HANA_WC_SPDIF_HI: u32;
    static EMU_HANA_WC_ADAT_LO: u32;
    static EMU_HANA_WC_ADAT_HI: u32;
    static EMU_HANA_WC_BNC_LO: u32;
    static EMU_HANA_WC_BNC_HI: u32;
    static EMU_HANA2_WC_SPDIF_LO: u32;
    static EMU_HANA2_WC_SPDIF_HI: u32;
    static EMU_HANA_WCLOCK_INT_44_1K: u32;
    static EMU_HANA_WCLOCK_INT_48K: u32;
    static EMU_HANA_WCLOCK_1X: u32;
    static EMU_HANA_WCLOCK_SRC_MASK: u32;
    static EMU_HANA_DOCK_LEDS_2_44K: u32;
    static EMU_HANA_DOCK_LEDS_2_48K: u32;
    static EMU_HANA_DOCK_LEDS_2_EXT: u32;
    static EMU_HANA_DOCK_LEDS_2_LOCK: u32;
    static EMU_HANA_DOCK_LEDS_2: u32;
    static EMU_HANA_FPGA_CONFIG: u32;
    static EMU_HANA_FPGA_CONFIG_AUDIODOCK: u32;
    static EMU_HANA_FPGA_CONFIG_HANA: u32;

    static CLIEH: u32;
    static CLIEL: u32;
    static CLIPH: u32;
    static CLIPL: u32;
    static HLIEH: u32;
    static HLIEL: u32;
    static HLIPH: u32;
    static HLIPL: u32;
    static SOLEH: u32;
    static SOLEL: u32;
    static EINVAL: i32;
    static EIO: i32;
    static EAGAIN: i32;
    static WC_CURRENTCHANNEL: u32;
    static WC_SAMPLECOUNTER: u32;

    fn outl(value: u32, port: u32);
    fn inl(port: u32) -> u32;
    fn outw(value: u16, port: u32);
    fn inw(port: u32) -> u16;
    fn outb(value: u8, port: u32);
    fn udelay(usecs: u32);
    fn mdelay(msecs: u32);
    fn snd_BUG_ON(cond: bool) -> bool;
    fn mutex_is_locked(lock: *mut core::ffi::c_void) -> bool;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_warn(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: core::ffi::c_ulong);
    fn REG_SIZE(reg: u32) -> u32;
    fn REG_VAL_GET(reg: u32, value: u32) -> u32;
    fn REG_MASK0(reg: u32) -> u32;
}

unsafe fn emu_audigy(_emu: *mut snd_emu10k1) -> bool {
    todo!("external field access: emu->audigy")
}
unsafe fn emu_port(_emu: *mut snd_emu10k1) -> u32 {
    todo!("external field access: emu->port")
}
unsafe fn emu_emu_lock(_emu: *mut snd_emu10k1) -> *mut core::ffi::c_void {
    todo!("external field access: &emu->emu_lock")
}
unsafe fn emu_spi_lock(_emu: *mut snd_emu10k1) -> *mut core::ffi::c_void {
    todo!("external field access: &emu->spi_lock")
}
unsafe fn emu_i2c_lock(_emu: *mut snd_emu10k1) -> *mut core::ffi::c_void {
    todo!("external field access: &emu->i2c_lock")
}
unsafe fn emu_card_dev(_emu: *mut snd_emu10k1) -> *mut core::ffi::c_void {
    todo!("external field access: emu->card->dev")
}
unsafe fn emu_ca0108_chip(_emu: *mut snd_emu10k1) -> bool {
    todo!("external field access: emu->card_capabilities->ca0108_chip")
}
unsafe fn emu1010_lock(_emu: *mut snd_emu10k1) -> *mut core::ffi::c_void {
    todo!("external field access: &emu->emu1010.lock")
}
unsafe fn emu1010_wclock(_emu: *mut snd_emu10k1) -> u32 {
    todo!("external field access: emu->emu1010.wclock")
}
unsafe fn emu1010_set_word_clock(_emu: *mut snd_emu10k1, _clock: i32) {
    todo!("external field access: emu->emu1010.word_clock")
}

unsafe fn guard_spinlock_irqsave(_lock: *mut core::ffi::c_void) {
    todo!("external scoped guard: spinlock_irqsave")
}
unsafe fn guard_spinlock(_lock: *mut core::ffi::c_void) {
    todo!("external scoped guard: spinlock")
}
unsafe fn guard_snd_emu1010_fpga_lock(_emu: *mut snd_emu10k1) {
    todo!("external scoped guard: snd_emu1010_fpga_lock")
}

unsafe fn check_ptr_reg(emu: *mut snd_emu10k1, reg: u32) -> bool {
    if snd_BUG_ON(emu.is_null()) {
        return false;
    }
    if snd_BUG_ON(
        (reg
            & if emu_audigy(emu) {
                0xffff0000 & !A_PTR_ADDRESS_MASK
            } else {
                0xffff0000 & !PTR_ADDRESS_MASK
            }) != 0,
    ) {
        return false;
    }
    if snd_BUG_ON((reg & 0x0000ffff & !PTR_CHANNELNUM_MASK) != 0) {
        return false;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ptr_read(
    emu: *mut snd_emu10k1,
    reg: u32,
    chn: u32,
) -> u32 {
    let regptr: u32;
    let val: u32;
    let mask: u32;

    regptr = (reg << 16) | chn;
    if !check_ptr_reg(emu, regptr) {
        return 0;
    }

    {
        guard_spinlock_irqsave(emu_emu_lock(emu));
        outl(regptr, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
    }

    if (reg & 0xff000000) != 0 {
        let size: u8;
        let offset: u8;

        size = ((reg >> 24) & 0x3f) as u8;
        offset = ((reg >> 16) & 0x1f) as u8;
        mask = (1u32 << size) - 1;

        (val >> offset) & mask
    } else {
        val
    }
}

// EXPORT_SYMBOL(snd_emu10k1_ptr_read);

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ptr_write(
    emu: *mut snd_emu10k1,
    reg: u32,
    chn: u32,
    mut data: u32,
) {
    let regptr: u32;
    let mut mask: u32;

    regptr = (reg << 16) | chn;
    if !check_ptr_reg(emu, regptr) {
        return;
    }

    guard_spinlock_irqsave(emu_emu_lock(emu));
    if (reg & 0xff000000) != 0 {
        let size: u8;
        let offset: u8;

        size = ((reg >> 24) & 0x3f) as u8;
        offset = ((reg >> 16) & 0x1f) as u8;
        mask = (1u32 << size) - 1;
        if snd_BUG_ON((data & !mask) != 0) {
            return;
        }
        mask <<= offset;
        data <<= offset;

        outl(regptr, emu_port(emu) + PTR);
        data |= inl(emu_port(emu) + DATA) & !mask;
    } else {
        outl(regptr, emu_port(emu) + PTR);
    }
    outl(data, emu_port(emu) + DATA);
}

// EXPORT_SYMBOL(snd_emu10k1_ptr_write);

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ptr_write_multiple(
    emu: *mut snd_emu10k1,
    chn: u32,
    mut args: ...
) {
    let addr_mask: u32;

    if snd_BUG_ON(emu.is_null()) {
        return;
    }
    if snd_BUG_ON((chn & !PTR_CHANNELNUM_MASK) != 0) {
        return;
    }
    addr_mask = !((if emu_audigy(emu) { A_PTR_ADDRESS_MASK } else { PTR_ADDRESS_MASK }) >> 16);

    guard_spinlock_irqsave(emu_emu_lock(emu));
    loop {
        let data: u32;
        let reg: u32 = args.arg::<u32>();
        if reg == REGLIST_END {
            break;
        }
        data = args.arg::<u32>();
        if snd_BUG_ON((reg & addr_mask) != 0) {
            // Only raw registers supported here
            continue;
        }
        outl((reg << 16) | chn, emu_port(emu) + PTR);
        outl(data, emu_port(emu) + DATA);
    }
}

// EXPORT_SYMBOL(snd_emu10k1_ptr_write_multiple);

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ptr20_read(
    emu: *mut snd_emu10k1,
    reg: u32,
    chn: u32,
) -> u32 {
    let regptr: u32;

    regptr = (reg << 16) | chn;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    outl(regptr, emu_port(emu) + PTR2);
    inl(emu_port(emu) + DATA2)
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ptr20_write(
    emu: *mut snd_emu10k1,
    reg: u32,
    chn: u32,
    data: u32,
) {
    let regptr: u32;

    regptr = (reg << 16) | chn;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    outl(regptr, emu_port(emu) + PTR2);
    outl(data, emu_port(emu) + DATA2);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_spi_write(emu: *mut snd_emu10k1, data: u32) -> i32 {
    let reset: u32;
    let set: u32;
    let reg: u32;
    let mut tmp: u32;
    let mut n: i32;
    let mut result: i32;

    /* This function is not re-entrant, so protect against it. */
    guard_spinlock(emu_spi_lock(emu));
    if emu_ca0108_chip(emu) {
        reg = P17V_SPI;
    } else {
        /* For other chip types the SPI register
         * is currently unknown. */
        return 1;
    }
    if data > 0xffff {
        /* Only 16bit values allowed */
        return 1;
    }

    tmp = snd_emu10k1_ptr20_read(emu, reg, 0);
    reset = (tmp & !0x3ffff) | 0x20000; /* Set xxx20000 */
    set = reset | 0x10000; /* Set xxx1xxxx */
    snd_emu10k1_ptr20_write(emu, reg, 0, reset | data);
    tmp = snd_emu10k1_ptr20_read(emu, reg, 0); /* write post */
    snd_emu10k1_ptr20_write(emu, reg, 0, set | data);
    result = 1;
    /* Wait for status bit to return to 0 */
    n = 0;
    while n < 100 {
        udelay(10);
        tmp = snd_emu10k1_ptr20_read(emu, reg, 0);
        if (tmp & 0x10000) == 0 {
            result = 0;
            break;
        }
        n += 1;
    }
    if result != 0 {
        /* Timed out */
        return 1;
    }
    snd_emu10k1_ptr20_write(emu, reg, 0, reset | data);
    tmp = snd_emu10k1_ptr20_read(emu, reg, 0); /* Write post */
    let _ = tmp;
    0
}

/* The ADC does not support i2c read, so only write is implemented */
#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_i2c_write(emu: *mut snd_emu10k1, reg: u32, value: u32) -> i32 {
    let mut tmp: u32;
    let mut timeout: i32 = 0;
    let mut status: i32 = 0;
    let mut retry: i32;

    if reg > 0x7f || value > 0x1ff {
        dev_err(emu_card_dev(emu), b"i2c_write: invalid values.\n\0".as_ptr());
        return -EINVAL;
    }

    /* This function is not re-entrant, so protect against it. */
    guard_spinlock(emu_i2c_lock(emu));

    tmp = reg << 25 | value << 16;

    /* This controls the I2C connected to the WM8775 ADC Codec */
    snd_emu10k1_ptr20_write(emu, P17V_I2C_1, 0, tmp);
    tmp = snd_emu10k1_ptr20_read(emu, P17V_I2C_1, 0); /* write post */
    let _ = tmp;

    retry = 0;
    while retry < 10 {
        /* Send the data to i2c */
        tmp = 0;
        tmp = tmp | (I2C_A_ADC_LAST | I2C_A_ADC_START | I2C_A_ADC_ADD);
        snd_emu10k1_ptr20_write(emu, P17V_I2C_ADDR, 0, tmp);

        /* Wait till the transaction ends */
        loop {
            mdelay(1);
            status = snd_emu10k1_ptr20_read(emu, P17V_I2C_ADDR, 0) as i32;
            timeout += 1;
            if (status as u32 & I2C_A_ADC_START) == 0 {
                break;
            }

            if timeout > 1000 {
                dev_warn(
                    emu_card_dev(emu),
                    b"emu10k1:I2C:timeout status=0x%x\n\0".as_ptr(),
                    status,
                );
                break;
            }
        }
        //Read back and see if the transaction is successful
        if (status as u32 & I2C_A_ADC_ABORT) == 0 {
            break;
        }
        retry += 1;
    }

    if retry == 10 {
        dev_err(emu_card_dev(emu), b"Writing to ADC failed!\n\0".as_ptr());
        dev_err(
            emu_card_dev(emu),
            b"status=0x%x, reg=%d, value=%d\n\0".as_ptr(),
            status,
            reg,
            value,
        );
        /* dump_stack(); */
        return -EINVAL;
    }

    0
}

unsafe fn snd_emu1010_fpga_write_locked(emu: *mut snd_emu10k1, mut reg: u32, value: u32) {
    if snd_BUG_ON(reg > 0x3f) {
        return;
    }
    reg += 0x40; /* 0x40 upwards are registers. */
    if snd_BUG_ON(value > 0x3f) {
        /* 0 to 0x3f are values */
        return;
    }
    outw(reg as u16, emu_port(emu) + A_GPIO);
    udelay(10);
    outw((reg | 0x80) as u16, emu_port(emu) + A_GPIO); /* High bit clocks the value into the fpga. */
    udelay(10);
    outw(value as u16, emu_port(emu) + A_GPIO);
    udelay(10);
    outw((value | 0x80) as u16, emu_port(emu) + A_GPIO); /* High bit clocks the value into the fpga. */
    udelay(10);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_fpga_write(emu: *mut snd_emu10k1, reg: u32, value: u32) {
    if snd_BUG_ON(!mutex_is_locked(emu1010_lock(emu))) {
        return;
    }
    snd_emu1010_fpga_write_locked(emu, reg, value);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_fpga_write_lock(emu: *mut snd_emu10k1, reg: u32, value: u32) {
    guard_snd_emu1010_fpga_lock(emu);
    snd_emu1010_fpga_write_locked(emu, reg, value);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_fpga_read(emu: *mut snd_emu10k1, mut reg: u32, value: *mut u32) {
    // The higest input pin is used as the designated interrupt trigger,
    // so it needs to be masked out.
    // But note that any other input pin change will also cause an IRQ,
    // so using this function often causes an IRQ as a side effect.
    let mask: u32 = if emu_ca0108_chip(emu) { 0x1f } else { 0x7f };

    if snd_BUG_ON(!mutex_is_locked(emu1010_lock(emu))) {
        return;
    }
    if snd_BUG_ON(reg > 0x3f) {
        return;
    }
    reg += 0x40; /* 0x40 upwards are registers. */
    outw(reg as u16, emu_port(emu) + A_GPIO);
    udelay(10);
    outw((reg | 0x80) as u16, emu_port(emu) + A_GPIO); /* High bit clocks the value into the fpga. */
    udelay(10);
    *value = ((inw(emu_port(emu) + A_GPIO) >> 8) as u32) & mask;
}

/* Each Destination has one and only one Source,
 * but one Source can feed any number of Destinations simultaneously.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_fpga_link_dst_src_write(
    emu: *mut snd_emu10k1,
    dst: u32,
    src: u32,
) {
    if snd_BUG_ON((dst & !0x71f) != 0) {
        return;
    }
    if snd_BUG_ON((src & !0x71f) != 0) {
        return;
    }
    snd_emu1010_fpga_write(emu, EMU_HANA_DESTHI, dst >> 8);
    snd_emu1010_fpga_write(emu, EMU_HANA_DESTLO, dst & 0x1f);
    snd_emu1010_fpga_write(emu, EMU_HANA_SRCHI, src >> 8);
    snd_emu1010_fpga_write(emu, EMU_HANA_SRCLO, src & 0x1f);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_fpga_link_dst_src_read(
    emu: *mut snd_emu10k1,
    dst: u32,
) -> u32 {
    let mut hi: u32 = 0;
    let mut lo: u32 = 0;

    if snd_BUG_ON((dst & !0x71f) != 0) {
        return 0;
    }
    snd_emu1010_fpga_write(emu, EMU_HANA_DESTHI, dst >> 8);
    snd_emu1010_fpga_write(emu, EMU_HANA_DESTLO, dst & 0x1f);
    snd_emu1010_fpga_read(emu, EMU_HANA_SRCHI, &mut hi);
    snd_emu1010_fpga_read(emu, EMU_HANA_SRCLO, &mut lo);
    (hi << 8) | lo
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_get_raw_rate(emu: *mut snd_emu10k1, src: u8) -> i32 {
    let reg_lo: u32;
    let reg_hi: u32;
    let mut value: u32 = 0;
    let mut value2: u32 = 0;

    if src == EMU_HANA_WCLOCK_HANA_SPDIF_IN {
        snd_emu1010_fpga_read(emu, EMU_HANA_SPDIF_MODE, &mut value);
        if (value & EMU_HANA_SPDIF_MODE_RX_INVALID) != 0 {
            return 0;
        }
        reg_lo = EMU_HANA_WC_SPDIF_LO;
        reg_hi = EMU_HANA_WC_SPDIF_HI;
    } else if src == EMU_HANA_WCLOCK_HANA_ADAT_IN {
        reg_lo = EMU_HANA_WC_ADAT_LO;
        reg_hi = EMU_HANA_WC_ADAT_HI;
    } else if src == EMU_HANA_WCLOCK_SYNC_BNC {
        reg_lo = EMU_HANA_WC_BNC_LO;
        reg_hi = EMU_HANA_WC_BNC_HI;
    } else if src == EMU_HANA_WCLOCK_2ND_HANA {
        reg_lo = EMU_HANA2_WC_SPDIF_LO;
        reg_hi = EMU_HANA2_WC_SPDIF_HI;
    } else {
        return 0;
    }
    snd_emu1010_fpga_read(emu, reg_hi, &mut value);
    snd_emu1010_fpga_read(emu, reg_lo, &mut value2);
    // FIXME: The /4 is valid for 0404b, but contradicts all other info.
    (0x1770000u32 / 4 / (((value << 5) | value2) + 1)) as i32
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_update_clock(emu: *mut snd_emu10k1) {
    let mut clock: i32;
    let mut leds: u32;

    if emu1010_wclock(emu) == (EMU_HANA_WCLOCK_INT_44_1K | EMU_HANA_WCLOCK_1X) {
        clock = 44100;
        leds = EMU_HANA_DOCK_LEDS_2_44K;
    } else if emu1010_wclock(emu) == (EMU_HANA_WCLOCK_INT_48K | EMU_HANA_WCLOCK_1X) {
        clock = 48000;
        leds = EMU_HANA_DOCK_LEDS_2_48K;
    } else {
        clock = snd_emu1010_get_raw_rate(
            emu,
            (emu1010_wclock(emu) & EMU_HANA_WCLOCK_SRC_MASK) as u8,
        );
        // The raw rate reading is rather coarse (it cannot accurately
        // represent 44.1 kHz) and fluctuates slightly. Luckily, the
        // clock comes from digital inputs, which use standardized rates.
        // So we round to the closest standard rate and ignore discrepancies.
        if clock < 46000 {
            clock = 44100;
            leds = EMU_HANA_DOCK_LEDS_2_EXT | EMU_HANA_DOCK_LEDS_2_44K;
        } else {
            clock = 48000;
            leds = EMU_HANA_DOCK_LEDS_2_EXT | EMU_HANA_DOCK_LEDS_2_48K;
        }
    }
    emu1010_set_word_clock(emu, clock);

    // FIXME: this should probably represent the AND of all currently
    // used sources' lock status. But we don't know how to get that ...
    leds |= EMU_HANA_DOCK_LEDS_2_LOCK;

    snd_emu1010_fpga_write(emu, EMU_HANA_DOCK_LEDS_2, leds);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu1010_load_firmware_entry(
    emu: *mut snd_emu10k1,
    dock: i32,
    fw_entry: *const firmware,
) {
    let mut write_post: u16;

    // On E-MU 1010 rev1 the FPGA is a Xilinx Spartan IIE XC2S50E.
    // On E-MU 0404b it is a Xilinx Spartan III XC3S50.
    // The wiring is as follows:
    // GPO7 -> FPGA input & 1K resistor -> FPGA /PGMN <- FPGA output
    //   In normal operation, the active low reset line is held up by
    //   an FPGA output, while the GPO pin performs its duty as control
    //   register access strobe signal. Writing the respective bit to
    //   EMU_HANA_FPGA_CONFIG puts the FPGA output into high-Z mode, at
    //   which point the GPO pin can control the reset line through the
    //   resistor.
    // GPO6 -> FPGA CCLK & FPGA input
    // GPO5 -> FPGA DIN (dual function)

    // If the FPGA is already programmed, return it to programming mode
    snd_emu1010_fpga_write(
        emu,
        EMU_HANA_FPGA_CONFIG,
        if dock != 0 {
            EMU_HANA_FPGA_CONFIG_AUDIODOCK
        } else {
            EMU_HANA_FPGA_CONFIG_HANA
        },
    );

    // Assert reset line for 100uS
    outw(0x00, emu_port(emu) + A_GPIO);
    write_post = inw(emu_port(emu) + A_GPIO);
    let _ = write_post;
    udelay(100);
    outw(0x80, emu_port(emu) + A_GPIO);
    write_post = inw(emu_port(emu) + A_GPIO);
    let _ = write_post;
    udelay(100); // Allow FPGA memory to clean

    // Upload the netlist. Keep reset line high!
    let mut n: usize = 0;
    while n < (*fw_entry).size {
        let mut value: u8 = *(*fw_entry).data.add(n);
        let mut i: i32 = 0;
        while i < 8 {
            let mut reg: u16 = 0x80;
            if (value & 1) != 0 {
                reg |= 0x20;
            }
            value >>= 1;
            outw(reg, emu_port(emu) + A_GPIO);
            write_post = inw(emu_port(emu) + A_GPIO);
            let _ = write_post;
            outw(reg | 0x40, emu_port(emu) + A_GPIO);
            write_post = inw(emu_port(emu) + A_GPIO);
            let _ = write_post;
            i += 1;
        }
        n += 1;
    }

    // After programming, set GPIO bit 4 high again.
    // This appears to be a config word that the rev1 Hana
    // firmware reads; weird things happen without this.
    outw(0x10, emu_port(emu) + A_GPIO);
    write_post = inw(emu_port(emu) + A_GPIO);
    let _ = write_post;
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_intr_enable(emu: *mut snd_emu10k1, intrenb: u32) {
    let enable: u32;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    enable = inl(emu_port(emu) + INTE) | intrenb;
    outl(enable, emu_port(emu) + INTE);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_intr_disable(emu: *mut snd_emu10k1, intrenb: u32) {
    let enable: u32;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    enable = inl(emu_port(emu) + INTE) & !intrenb;
    outl(enable, emu_port(emu) + INTE);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_intr_enable(emu: *mut snd_emu10k1, voicenum: u32) {
    let mut val: u32;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    if voicenum >= 32 {
        outl(CLIEH << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val |= 1 << (voicenum - 32);
    } else {
        outl(CLIEL << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val |= 1 << voicenum;
    }
    outl(val, emu_port(emu) + DATA);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_intr_disable(emu: *mut snd_emu10k1, voicenum: u32) {
    let mut val: u32;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    if voicenum >= 32 {
        outl(CLIEH << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val &= !(1 << (voicenum - 32));
    } else {
        outl(CLIEL << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val &= !(1 << voicenum);
    }
    outl(val, emu_port(emu) + DATA);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_intr_ack(emu: *mut snd_emu10k1, mut voicenum: u32) {
    guard_spinlock_irqsave(emu_emu_lock(emu));
    if voicenum >= 32 {
        outl(CLIPH << 16, emu_port(emu) + PTR);
        voicenum = 1 << (voicenum - 32);
    } else {
        outl(CLIPL << 16, emu_port(emu) + PTR);
        voicenum = 1 << voicenum;
    }
    outl(voicenum, emu_port(emu) + DATA);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_half_loop_intr_enable(
    emu: *mut snd_emu10k1,
    voicenum: u32,
) {
    let mut val: u32;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    if voicenum >= 32 {
        outl(HLIEH << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val |= 1 << (voicenum - 32);
    } else {
        outl(HLIEL << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val |= 1 << voicenum;
    }
    outl(val, emu_port(emu) + DATA);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_half_loop_intr_disable(
    emu: *mut snd_emu10k1,
    voicenum: u32,
) {
    let mut val: u32;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    if voicenum >= 32 {
        outl(HLIEH << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val &= !(1 << (voicenum - 32));
    } else {
        outl(HLIEL << 16, emu_port(emu) + PTR);
        val = inl(emu_port(emu) + DATA);
        val &= !(1 << voicenum);
    }
    outl(val, emu_port(emu) + DATA);
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_half_loop_intr_ack(
    emu: *mut snd_emu10k1,
    mut voicenum: u32,
) {
    guard_spinlock_irqsave(emu_emu_lock(emu));
    if voicenum >= 32 {
        outl(HLIPH << 16, emu_port(emu) + PTR);
        voicenum = 1 << (voicenum - 32);
    } else {
        outl(HLIPL << 16, emu_port(emu) + PTR);
        voicenum = 1 << voicenum;
    }
    outl(voicenum, emu_port(emu) + DATA);
}

// Original C contained #if 0 implementations of:
// snd_emu10k1_voice_set_loop_stop()
// snd_emu10k1_voice_clear_loop_stop()

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_set_loop_stop_multiple(
    emu: *mut snd_emu10k1,
    voices: u64,
) {
    guard_spinlock_irqsave(emu_emu_lock(emu));
    outl(SOLEL << 16, emu_port(emu) + PTR);
    outl(inl(emu_port(emu) + DATA) | voices as u32, emu_port(emu) + DATA);
    outl(SOLEH << 16, emu_port(emu) + PTR);
    outl(
        inl(emu_port(emu) + DATA) | (voices >> 32) as u32,
        emu_port(emu) + DATA,
    );
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_clear_loop_stop_multiple(
    emu: *mut snd_emu10k1,
    voices: u64,
) {
    guard_spinlock_irqsave(emu_emu_lock(emu));
    outl(SOLEL << 16, emu_port(emu) + PTR);
    outl(
        inl(emu_port(emu) + DATA) & (!voices as u32),
        emu_port(emu) + DATA,
    );
    outl(SOLEH << 16, emu_port(emu) + PTR);
    outl(
        inl(emu_port(emu) + DATA) & ((!voices >> 32) as u32),
        emu_port(emu) + DATA,
    );
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_voice_clear_loop_stop_multiple_atomic(
    emu: *mut snd_emu10k1,
    voices: u64,
) -> i32 {
    let flags: core::ffi::c_ulong = 0;
    let mut soll: u32;
    let mut solh: u32;
    let mut ret: i32 = -EIO;

    spin_lock_irqsave(emu_emu_lock(emu), flags);

    outl(SOLEL << 16, emu_port(emu) + PTR);
    soll = inl(emu_port(emu) + DATA);
    outl(SOLEH << 16, emu_port(emu) + PTR);
    solh = inl(emu_port(emu) + DATA);

    soll &= !voices as u32;
    solh &= (!voices >> 32) as u32;

    let mut tries: i32 = 0;
    while tries < 1000 {
        let quart: u32 = 1u32 << (REG_SIZE(WC_CURRENTCHANNEL) - 2);
        // First we wait for the third quarter of the sample cycle ...
        let wc: u32 = inl(emu_port(emu) + WC);
        let mut cc: u32 = REG_VAL_GET(WC_CURRENTCHANNEL, wc);
        if cc >= quart * 2 && cc < quart * 3 {
            // ... and release the low voices, while the high ones are serviced.
            outl(SOLEL << 16, emu_port(emu) + PTR);
            outl(soll, emu_port(emu) + DATA);
            // Then we wait for the first quarter of the next sample cycle ...
            while tries < 1000 {
                cc = REG_VAL_GET(WC_CURRENTCHANNEL, inl(emu_port(emu) + WC));
                if cc < quart {
                    break;
                }
                // We will block for 10+ us with interrupts disabled. This is
                // not nice at all, but necessary for reasonable reliability.
                udelay(1);
                tries += 1;
            }
            if tries >= 1000 {
                break;
            }
            // ... and release the high voices, while the low ones are serviced.
            outl(SOLEH << 16, emu_port(emu) + PTR);
            outl(solh, emu_port(emu) + DATA);
            // Finally we verify that nothing interfered in fact.
            if REG_VAL_GET(WC_SAMPLECOUNTER, inl(emu_port(emu) + WC))
                == ((REG_VAL_GET(WC_SAMPLECOUNTER, wc) + 1) & REG_MASK0(WC_SAMPLECOUNTER))
            {
                ret = 0;
            } else {
                ret = -EAGAIN;
            }
            break;
        }
        // Don't block for too long
        spin_unlock_irqrestore(emu_emu_lock(emu), flags);
        udelay(1);
        spin_lock_irqsave(emu_emu_lock(emu), flags);
        tries += 1;
    }

    spin_unlock_irqrestore(emu_emu_lock(emu), flags);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_wait(emu: *mut snd_emu10k1, mut wait: u32) {
    let mut count: u32;
    let mut newtime: u32 = 0;
    let mut curtime: u32;

    curtime = inl(emu_port(emu) + WC) >> 6;
    while wait > 0 {
        wait -= 1;
        count = 0;
        while {
            let old = count;
            count = count.wrapping_add(1);
            old < 16384
        } {
            newtime = inl(emu_port(emu) + WC) >> 6;
            if newtime != curtime {
                break;
            }
        }
        if count > 16384 {
            break;
        }
        curtime = newtime;
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ac97_read(ac97: *mut snd_ac97, reg: u16) -> u16 {
    let emu: *mut snd_emu10k1 = (*ac97).private_data;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    outb(reg as u8, emu_port(emu) + AC97ADDRESS);
    inw(emu_port(emu) + AC97DATA)
}

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_ac97_write(ac97: *mut snd_ac97, reg: u16, data: u16) {
    let emu: *mut snd_emu10k1 = (*ac97).private_data;

    guard_spinlock_irqsave(emu_emu_lock(emu));
    outb(reg as u8, emu_port(emu) + AC97ADDRESS);
    outw(data, emu_port(emu) + AC97DATA);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
