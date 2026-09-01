// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Infrasonic Quartet
 *
 *	Copyright (c) 2009 Pavel Hofman <pavel.hofman@ivitera.com>
 */

// Dependencies from the original C includes:
// linux/delay.h, linux/interrupt.h, linux/init.h, linux/slab.h,
// linux/string.h, sound/core.h, sound/tlv.h, sound/info.h,
// ice1712.h, envy24ht.h, sound/ak4113.h, quartet.h.

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

#[repr(C)]
pub struct qtet_spec {
    ak4113: *mut ak4113,
    scr: c_uint,  /* system control register */
    mcr: c_uint,  /* monitoring control register */
    cpld: c_uint, /* cpld register */
}

#[repr(C)]
pub struct qtet_kcontrol_private {
    bit: c_uint,
    set_register: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
    get_register: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_uint>,
    texts: [*const c_char; 2],
}

const IN12_SEL: usize = 0;
const IN34_SEL: usize = 1;
const AIN34_SEL: usize = 2;
const COAX_OUT: usize = 3;
const IN12_MON12: usize = 4;
const IN12_MON34: usize = 5;
const IN34_MON12: usize = 6;
const IN34_MON34: usize = 7;
const OUT12_MON34: usize = 8;
const OUT34_MON12: usize = 9;

static ext_clock_names: [*const c_char; 3] = [
    b"IEC958 In\0".as_ptr() as *const c_char,
    b"Word Clock 1xFS\0".as_ptr() as *const c_char,
    b"Word Clock 256xFS\0".as_ptr() as *const c_char,
];

/* chip address on I2C bus */
const AK4113_ADDR: c_uint = 0x26; /* S/PDIF receiver */

/* chip address on SPI bus */
const AK4620_ADDR: c_uint = 0x02; /* ADC/DAC */

/*
 * GPIO pins
 */

/* GPIO0 - O - DATA0, def. 0 */
const GPIO_D0: c_uint = 1 << 0;
/* GPIO1 - I/O - DATA1, Jack Detect Input0 (0:present, 1:missing), def. 1 */
const GPIO_D1_JACKDTC0: c_uint = 1 << 1;
/* GPIO2 - I/O - DATA2, Jack Detect Input1 (0:present, 1:missing), def. 1 */
const GPIO_D2_JACKDTC1: c_uint = 1 << 2;
/* GPIO3 - I/O - DATA3, def. 1 */
const GPIO_D3: c_uint = 1 << 3;
/* GPIO4 - I/O - DATA4, SPI CDTO, def. 1 */
const GPIO_D4_SPI_CDTO: c_uint = 1 << 4;
/* GPIO5 - I/O - DATA5, SPI CCLK, def. 1 */
const GPIO_D5_SPI_CCLK: c_uint = 1 << 5;
/* GPIO6 - I/O - DATA6, Cable Detect Input (0:detected, 1:not detected */
const GPIO_D6_CD: c_uint = 1 << 6;
/* GPIO7 - I/O - DATA7, Device Detect Input (0:detected, 1:not detected */
const GPIO_D7_DD: c_uint = 1 << 7;
/* GPIO8 - O - CPLD Chip Select, def. 1 */
const GPIO_CPLD_CSN: c_uint = 1 << 8;
/* GPIO9 - O - CPLD register read/write (0:write, 1:read), def. 0 */
const GPIO_CPLD_RW: c_uint = 1 << 9;
/* GPIO10 - O - SPI Chip Select for CODEC#0, def. 1 */
const GPIO_SPI_CSN0: c_uint = 1 << 10;
/* GPIO11 - O - SPI Chip Select for CODEC#1, def. 1 */
const GPIO_SPI_CSN1: c_uint = 1 << 11;
/* GPIO12 - O - Ex. Register Output Enable (0:enable, 1:disable), def. 1,
 * init 0 */
const GPIO_EX_GPIOE: c_uint = 1 << 12;
/* GPIO13 - O - Ex. Register0 Chip Select for System Control Register,
 * def. 1 */
const GPIO_SCR: c_uint = 1 << 13;
/* GPIO14 - O - Ex. Register1 Chip Select for Monitor Control Register,
 * def. 1 */
const GPIO_MCR: c_uint = 1 << 14;

const GPIO_SPI_ALL: c_uint = GPIO_D4_SPI_CDTO | GPIO_D5_SPI_CCLK | GPIO_SPI_CSN0 | GPIO_SPI_CSN1;

const GPIO_DATA_MASK: c_uint = GPIO_D0
    | GPIO_D1_JACKDTC0
    | GPIO_D2_JACKDTC1
    | GPIO_D3
    | GPIO_D4_SPI_CDTO
    | GPIO_D5_SPI_CCLK
    | GPIO_D6_CD
    | GPIO_D7_DD;

/* System Control Register GPIO_SCR data bits */
/* Mic/Line select relay (0:line, 1:mic) */
const SCR_RELAY: c_uint = GPIO_D0;
/* Phantom power drive control (0:5V, 1:48V) */
const SCR_PHP_V: c_uint = GPIO_D1_JACKDTC0;
/* H/W mute control (0:Normal, 1:Mute) */
const SCR_MUTE: c_uint = GPIO_D2_JACKDTC1;
/* Phantom power control (0:Phantom on, 1:off) */
const SCR_PHP: c_uint = GPIO_D3;
/* Analog input 1/2 Source Select */
const SCR_AIN12_SEL0: c_uint = GPIO_D4_SPI_CDTO;
const SCR_AIN12_SEL1: c_uint = GPIO_D5_SPI_CCLK;
/* Analog input 3/4 Source Select (0:line, 1:hi-z) */
const SCR_AIN34_SEL: c_uint = GPIO_D6_CD;
/* Codec Power Down (0:power down, 1:normal) */
const SCR_CODEC_PDN: c_uint = GPIO_D7_DD;

const SCR_AIN12_LINE: c_uint = 0;
const SCR_AIN12_MIC: c_uint = SCR_AIN12_SEL0;
const SCR_AIN12_LOWCUT: c_uint = SCR_AIN12_SEL1 | SCR_AIN12_SEL0;

/* Monitor Control Register GPIO_MCR data bits */
/* Input 1/2 to Monitor 1/2 (0:off, 1:on) */
const MCR_IN12_MON12: c_uint = GPIO_D0;
/* Input 1/2 to Monitor 3/4 (0:off, 1:on) */
const MCR_IN12_MON34: c_uint = GPIO_D1_JACKDTC0;
/* Input 3/4 to Monitor 1/2 (0:off, 1:on) */
const MCR_IN34_MON12: c_uint = GPIO_D2_JACKDTC1;
/* Input 3/4 to Monitor 3/4 (0:off, 1:on) */
const MCR_IN34_MON34: c_uint = GPIO_D3;
/* Output to Monitor 1/2 (0:off, 1:on) */
const MCR_OUT34_MON12: c_uint = GPIO_D4_SPI_CDTO;
/* Output to Monitor 3/4 (0:off, 1:on) */
const MCR_OUT12_MON34: c_uint = GPIO_D5_SPI_CCLK;

/* CPLD Register DATA bits */
/* Clock Rate Select */
const CPLD_CKS0: c_uint = GPIO_D0;
const CPLD_CKS1: c_uint = GPIO_D1_JACKDTC0;
const CPLD_CKS2: c_uint = GPIO_D2_JACKDTC1;
/* Sync Source Select (0:Internal, 1:External) */
const CPLD_SYNC_SEL: c_uint = GPIO_D3;
/* Word Clock FS Select (0:FS, 1:256FS) */
const CPLD_WORD_SEL: c_uint = GPIO_D4_SPI_CDTO;
/* Coaxial Output Source (IS-Link) (0:SPDIF, 1:I2S) */
const CPLD_COAX_OUT: c_uint = GPIO_D5_SPI_CCLK;
/* Input 1/2 Source Select (0:Analog12, 1:An34) */
const CPLD_IN12_SEL: c_uint = GPIO_D6_CD;
/* Input 3/4 Source Select (0:Analog34, 1:Digital In) */
const CPLD_IN34_SEL: c_uint = GPIO_D7_DD;

/* internal clock (CPLD_SYNC_SEL = 0) options */
const CPLD_CKS_44100HZ: c_uint = 0;
const CPLD_CKS_48000HZ: c_uint = CPLD_CKS0;
const CPLD_CKS_88200HZ: c_uint = CPLD_CKS1;
const CPLD_CKS_96000HZ: c_uint = CPLD_CKS1 | CPLD_CKS0;
const CPLD_CKS_176400HZ: c_uint = CPLD_CKS2;
const CPLD_CKS_192000HZ: c_uint = CPLD_CKS2 | CPLD_CKS0;

const CPLD_CKS_MASK: c_uint = CPLD_CKS0 | CPLD_CKS1 | CPLD_CKS2;

/* external clock (CPLD_SYNC_SEL = 1) options */
/* external clock - SPDIF */
const CPLD_EXT_SPDIF: c_uint = 0 | CPLD_SYNC_SEL;
/* external clock - WordClock 1xfs */
const CPLD_EXT_WORDCLOCK_1FS: c_uint = CPLD_CKS1 | CPLD_SYNC_SEL;
/* external clock - WordClock 256xfs */
const CPLD_EXT_WORDCLOCK_256FS: c_uint = CPLD_CKS1 | CPLD_WORD_SEL | CPLD_SYNC_SEL;

const EXT_SPDIF_TYPE: c_int = 0;
const EXT_WORDCLOCK_1FS_TYPE: c_int = 1;
const EXT_WORDCLOCK_256FS_TYPE: c_int = 2;

const AK4620_DFS0: c_uchar = 1 << 0;
const AK4620_DFS1: c_uchar = 1 << 1;
const AK4620_CKS0: c_uchar = 1 << 2;
const AK4620_CKS1: c_uchar = 1 << 3;
/* Clock and Format Control register */
const AK4620_DFS_REG: c_uchar = 0x02;

/* Deem and Volume Control register */
const AK4620_DEEMVOL_REG: c_uchar = 0x03;
const AK4620_SMUTE: c_uchar = 1 << 7;

/*
 * Conversion from int value to its binary form. Used for debugging.
 * The output buffer must be allocated prior to calling the function.
 */
unsafe extern "C" fn get_binary(buffer: *mut c_char, value: c_int) -> *mut c_char {
    let mut pos: c_int = 0;
    for i in 0..4 {
        for j in 0..8 {
            if value & (1 << (31 - (i * 8 + j))) != 0 {
                *buffer.offset(pos as isize) = b'1' as c_char;
            } else {
                *buffer.offset(pos as isize) = b'0' as c_char;
            }
            pos += 1;
        }
        if i < 3 {
            *buffer.offset(pos as isize) = b' ' as c_char;
            pos += 1;
        }
    }
    *buffer.offset(pos as isize) = 0;
    buffer
}

/*
 * Initial setup of the conversion array GPIO <-> rate
 */
static qtet_rates: [c_uint; 6] = [44100, 48000, 88200, 96000, 176400, 192000];

static cks_vals: [c_uint; 6] = [
    CPLD_CKS_44100HZ,
    CPLD_CKS_48000HZ,
    CPLD_CKS_88200HZ,
    CPLD_CKS_96000HZ,
    CPLD_CKS_176400HZ,
    CPLD_CKS_192000HZ,
];

static qtet_rates_info: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: qtet_rates.len() as c_uint,
    list: qtet_rates.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn qtet_ak4113_write(private_data: *mut c_void, reg: c_uchar, val: c_uchar) {
    snd_vt1724_write_i2c(private_data as *mut snd_ice1712, AK4113_ADDR, reg, val);
}

unsafe extern "C" fn qtet_ak4113_read(private_data: *mut c_void, reg: c_uchar) -> c_uchar {
    snd_vt1724_read_i2c(private_data as *mut snd_ice1712, AK4113_ADDR, reg)
}

/*
 * AK4620 section
 */

/*
 * Write data to addr register of ak4620
 */
unsafe extern "C" fn qtet_akm_write(
    ak: *mut snd_akm4xxx,
    chip: c_int,
    addr: c_uchar,
    data: c_uchar,
) {
    let mut tmp: c_uint;
    let orig_dir: c_uint;
    let mut idx: c_int;
    let mut addrdata: c_uint;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    if snd_BUG_ON(chip < 0 || chip >= 4) != 0 {
        return;
    }
    /*dev_dbg(ice->card->dev, "Writing to AK4620: chip=%d, addr=0x%x,
      data=0x%x\n", chip, addr, data);*/
    orig_dir = ((*ice).gpio.get_dir.unwrap())(ice);
    ((*ice).gpio.set_dir.unwrap())(ice, orig_dir | GPIO_SPI_ALL);
    /* set mask - only SPI bits */
    ((*ice).gpio.set_mask.unwrap())(ice, !GPIO_SPI_ALL);

    tmp = ((*ice).gpio.get_data.unwrap())(ice);
    /* high all */
    tmp |= GPIO_SPI_ALL;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);
    /* drop chip select */
    if chip != 0 {
        /* CODEC 1 */
        tmp &= !GPIO_SPI_CSN1;
    } else {
        tmp &= !GPIO_SPI_CSN0;
    }
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);

    /* build I2C address + data byte */
    addrdata = (AK4620_ADDR << 6) | 0x20 | ((addr as c_uint) & 0x1f);
    addrdata = (addrdata << 8) | data as c_uint;
    idx = 15;
    while idx >= 0 {
        /* drop clock */
        tmp &= !GPIO_D5_SPI_CCLK;
        ((*ice).gpio.set_data.unwrap())(ice, tmp);
        udelay(100);
        /* set data */
        if addrdata & (1 << idx) != 0 {
            tmp |= GPIO_D4_SPI_CDTO;
        } else {
            tmp &= !GPIO_D4_SPI_CDTO;
        }
        ((*ice).gpio.set_data.unwrap())(ice, tmp);
        udelay(100);
        /* raise clock */
        tmp |= GPIO_D5_SPI_CCLK;
        ((*ice).gpio.set_data.unwrap())(ice, tmp);
        udelay(100);
        idx -= 1;
    }
    /* all back to 1 */
    tmp |= GPIO_SPI_ALL;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);

    /* return all gpios to non-writable */
    ((*ice).gpio.set_mask.unwrap())(ice, 0xffffff);
    /* restore GPIOs direction */
    ((*ice).gpio.set_dir.unwrap())(ice, orig_dir);
}

unsafe extern "C" fn qtet_akm_set_regs(
    ak: *mut snd_akm4xxx,
    addr: c_uchar,
    mask: c_uchar,
    value: c_uchar,
) {
    let mut tmp: c_uchar;
    let mut chip: c_int = 0;
    while chip < (*ak).num_chips {
        tmp = snd_akm4xxx_get(ak, chip, addr);
        /* clear the bits */
        tmp &= !mask;
        /* set the new bits */
        tmp |= value;
        snd_akm4xxx_write(ak, chip, addr, tmp);
        chip += 1;
    }
}

/*
 * change the rate of AK4620
 */
unsafe extern "C" fn qtet_akm_set_rate_val(ak: *mut snd_akm4xxx, rate: c_uint) {
    let ak4620_dfs: c_uchar;

    if rate == 0 {
        /* no hint - S/PDIF input is master or the new spdif
           input rate undetected, simply return */
        return;
    }

    /* adjust DFS on codecs - see datasheet */
    if rate > 108000 {
        ak4620_dfs = AK4620_DFS1 | AK4620_CKS1;
    } else if rate > 54000 {
        ak4620_dfs = AK4620_DFS0 | AK4620_CKS0;
    } else {
        ak4620_dfs = 0;
    }

    /* set new value */
    qtet_akm_set_regs(
        ak,
        AK4620_DFS_REG,
        AK4620_DFS0 | AK4620_DFS1 | AK4620_CKS0 | AK4620_CKS1,
        ak4620_dfs,
    );
}

const PCM_12_PLAYBACK_VOLUME: *const c_char = b"PCM 1/2 Playback Volume\0".as_ptr() as *const c_char;
const PCM_34_PLAYBACK_VOLUME: *const c_char = b"PCM 3/4 Playback Volume\0".as_ptr() as *const c_char;
const PCM_12_CAPTURE_VOLUME: *const c_char = b"PCM 1/2 Capture Volume\0".as_ptr() as *const c_char;
const PCM_34_CAPTURE_VOLUME: *const c_char = b"PCM 3/4 Capture Volume\0".as_ptr() as *const c_char;

static qtet_dac: [snd_akm4xxx_dac_channel; 2] = [
    snd_akm4xxx_dac_channel { name: PCM_12_PLAYBACK_VOLUME, num_channels: 2 },
    snd_akm4xxx_dac_channel { name: PCM_34_PLAYBACK_VOLUME, num_channels: 2 },
];

static qtet_adc: [snd_akm4xxx_adc_channel; 2] = [
    snd_akm4xxx_adc_channel { name: PCM_12_CAPTURE_VOLUME, num_channels: 2 },
    snd_akm4xxx_adc_channel { name: PCM_34_CAPTURE_VOLUME, num_channels: 2 },
];

static akm_qtet_dac: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4620,
    num_dacs: 4, /* DAC1 - Output 12
                  */
    num_adcs: 4, /* ADC1 - Input 12
                  */
    ops: snd_akm4xxx_ops {
        write: Some(qtet_akm_write),
        set_rate_val: Some(qtet_akm_set_rate_val),
    },
    dac_info: qtet_dac.as_ptr(),
    adc_info: qtet_adc.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

/* Communication routines with the CPLD */

/* Writes data to external register reg, both reg and data are
 * GPIO representations */
unsafe extern "C" fn reg_write(ice: *mut snd_ice1712, reg: c_uint, data: c_uint) {
    let mut tmp: c_uint;

    mutex_lock(&mut (*ice).gpio_mutex);
    /* set direction of used GPIOs*/
    /* all outputs */
    tmp = 0x00ffff;
    ((*ice).gpio.set_dir.unwrap())(ice, tmp);
    /* mask - writable bits */
    ((*ice).gpio.set_mask.unwrap())(ice, !tmp);
    /* write the data */
    tmp = ((*ice).gpio.get_data.unwrap())(ice);
    tmp &= !GPIO_DATA_MASK;
    tmp |= data;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);
    /* drop output enable */
    tmp &= !GPIO_EX_GPIOE;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);
    /* drop the register gpio */
    tmp &= !reg;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);
    /* raise the register GPIO */
    tmp |= reg;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    udelay(100);

    /* raise all data gpios */
    tmp |= GPIO_DATA_MASK;
    ((*ice).gpio.set_data.unwrap())(ice, tmp);
    /* mask - immutable bits */
    ((*ice).gpio.set_mask.unwrap())(ice, 0xffffff);
    /* outputs only 8-15 */
    ((*ice).gpio.set_dir.unwrap())(ice, 0x00ff00);
    mutex_unlock(&mut (*ice).gpio_mutex);
}

unsafe extern "C" fn get_scr(ice: *mut snd_ice1712) -> c_uint {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    (*spec).scr
}

unsafe extern "C" fn get_mcr(ice: *mut snd_ice1712) -> c_uint {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    (*spec).mcr
}

unsafe extern "C" fn get_cpld(ice: *mut snd_ice1712) -> c_uint {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    (*spec).cpld
}

unsafe extern "C" fn set_scr(ice: *mut snd_ice1712, val: c_uint) {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    reg_write(ice, GPIO_SCR, val);
    (*spec).scr = val;
}

unsafe extern "C" fn set_mcr(ice: *mut snd_ice1712, val: c_uint) {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    reg_write(ice, GPIO_MCR, val);
    (*spec).mcr = val;
}

unsafe extern "C" fn set_cpld(ice: *mut snd_ice1712, val: c_uint) {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    reg_write(ice, GPIO_CPLD_CSN, val);
    (*spec).cpld = val;
}

unsafe extern "C" fn proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ice: *mut snd_ice1712 = (*entry).private_data as *mut snd_ice1712;
    let mut bin_buffer: [c_char; 36] = [0; 36];

    snd_iprintf(buffer, b"SCR:\t%s\n\0".as_ptr() as *const c_char, get_binary(bin_buffer.as_mut_ptr(), get_scr(ice) as c_int));
    snd_iprintf(buffer, b"MCR:\t%s\n\0".as_ptr() as *const c_char, get_binary(bin_buffer.as_mut_ptr(), get_mcr(ice) as c_int));
    snd_iprintf(buffer, b"CPLD:\t%s\n\0".as_ptr() as *const c_char, get_binary(bin_buffer.as_mut_ptr(), get_cpld(ice) as c_int));
}

unsafe extern "C" fn proc_init(ice: *mut snd_ice1712) {
    snd_card_ro_proc_new((*ice).card, b"quartet\0".as_ptr() as *const c_char, ice as *mut c_void, Some(proc_regs_read));
}

unsafe extern "C" fn qtet_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let val: c_uint = get_scr(ice) & SCR_MUTE;
    (*ucontrol).value.integer.value[0] = if val != 0 { 0 } else { 1 };
    0
}

unsafe extern "C" fn qtet_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let old: c_uint;
    let new: c_uint;
    let smute: c_uchar;
    old = get_scr(ice) & SCR_MUTE;
    if (*ucontrol).value.integer.value[0] != 0 {
        /* unmute */
        new = 0;
        /* un-smuting DAC */
        smute = 0;
    } else {
        /* mute */
        new = SCR_MUTE;
        /* smuting DAC */
        smute = AK4620_SMUTE;
    }
    if old != new {
        let ak: *mut snd_akm4xxx = (*ice).akm;
        set_scr(ice, (get_scr(ice) & !SCR_MUTE) | new);
        /* set smute */
        qtet_akm_set_regs(ak, AK4620_DEEMVOL_REG, AK4620_SMUTE, smute);
        return 1;
    }
    /* no change */
    0
}

unsafe extern "C" fn qtet_ain12_enum_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static texts: [*const c_char; 3] = [
        b"Line In 1/2\0".as_ptr() as *const c_char,
        b"Mic\0".as_ptr() as *const c_char,
        b"Mic + Low-cut\0".as_ptr() as *const c_char,
    ];
    snd_ctl_enum_info(uinfo, 1, texts.len() as c_uint, texts.as_ptr())
}

unsafe extern "C" fn qtet_ain12_sw_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let val: c_uint;
    let result: c_uint;
    val = get_scr(ice) & (SCR_AIN12_SEL1 | SCR_AIN12_SEL0);
    match val {
        SCR_AIN12_LINE => result = 0,
        SCR_AIN12_MIC => result = 1,
        SCR_AIN12_LOWCUT => result = 2,
        _ => {
            /* BUG - no other combinations allowed */
            snd_BUG();
            result = 0;
        }
    }
    (*ucontrol).value.integer.value[0] = result as _;
    0
}

unsafe extern "C" fn qtet_ain12_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let old: c_uint;
    let mut new: c_uint;
    let mut tmp: c_uint;
    let masked_old: c_uint;
    old = get_scr(ice);
    masked_old = old & (SCR_AIN12_SEL1 | SCR_AIN12_SEL0);
    tmp = (*ucontrol).value.integer.value[0] as c_uint;
    if tmp == 2 {
        tmp = 3; /* binary 10 is not supported */
    }
    tmp <<= 4; /* shifting to SCR_AIN12_SEL0 */
    if tmp != masked_old {
        /* change requested */
        match tmp {
            SCR_AIN12_LINE => {
                new = old & !(SCR_AIN12_SEL1 | SCR_AIN12_SEL0);
                set_scr(ice, new);
                /* turn off relay */
                new &= !SCR_RELAY;
                set_scr(ice, new);
            }
            SCR_AIN12_MIC => {
                /* turn on relay */
                new = old | SCR_RELAY;
                set_scr(ice, new);
                new = (new & !SCR_AIN12_SEL1) | SCR_AIN12_SEL0;
                set_scr(ice, new);
            }
            SCR_AIN12_LOWCUT => {
                /* turn on relay */
                new = old | SCR_RELAY;
                set_scr(ice, new);
                new |= SCR_AIN12_SEL1 | SCR_AIN12_SEL0;
                set_scr(ice, new);
            }
            _ => snd_BUG(),
        }
        return 1;
    }
    /* no change */
    0
}

unsafe extern "C" fn qtet_php_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let val: c_uint;
    /* if phantom voltage =48V, phantom on */
    val = get_scr(ice) & SCR_PHP_V;
    (*ucontrol).value.integer.value[0] = if val != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn qtet_php_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let old: c_uint;
    let mut new: c_uint;
    old = get_scr(ice);
    new = old;
    if (*ucontrol).value.integer.value[0] != 0
        /* phantom on requested */
        && (!old & SCR_PHP_V) != 0
    /* 0 = voltage 5V */
    {
        /* is off, turn on */
        /* turn voltage on first, = 1 */
        new = old | SCR_PHP_V;
        set_scr(ice, new);
        /* turn phantom on, = 0 */
        new &= !SCR_PHP;
        set_scr(ice, new);
    } else if (*ucontrol).value.integer.value[0] == 0 && (old & SCR_PHP_V) != 0 {
        /* phantom off requested and 1 = voltage 48V */
        /* is on, turn off */
        /* turn voltage off first, = 0 */
        new = old & !SCR_PHP_V;
        set_scr(ice, new);
        /* turn phantom off, = 1 */
        new |= SCR_PHP;
        set_scr(ice, new);
    }
    if old != new {
        return 1;
    }
    /* no change */
    0
}

static qtet_privates: [qtet_kcontrol_private; 10] = [
    qtet_kcontrol_private { bit: CPLD_IN12_SEL, set_register: Some(set_cpld), get_register: Some(get_cpld), texts: [b"An In 1/2\0".as_ptr() as *const c_char, b"An In 3/4\0".as_ptr() as *const c_char] },
    qtet_kcontrol_private { bit: CPLD_IN34_SEL, set_register: Some(set_cpld), get_register: Some(get_cpld), texts: [b"An In 3/4\0".as_ptr() as *const c_char, b"IEC958 In\0".as_ptr() as *const c_char] },
    qtet_kcontrol_private { bit: SCR_AIN34_SEL, set_register: Some(set_scr), get_register: Some(get_scr), texts: [b"Line In 3/4\0".as_ptr() as *const c_char, b"Hi-Z\0".as_ptr() as *const c_char] },
    qtet_kcontrol_private { bit: CPLD_COAX_OUT, set_register: Some(set_cpld), get_register: Some(get_cpld), texts: [b"IEC958\0".as_ptr() as *const c_char, b"I2S\0".as_ptr() as *const c_char] },
    qtet_kcontrol_private { bit: MCR_IN12_MON12, set_register: Some(set_mcr), get_register: Some(get_mcr), texts: [core::ptr::null(), core::ptr::null()] },
    qtet_kcontrol_private { bit: MCR_IN12_MON34, set_register: Some(set_mcr), get_register: Some(get_mcr), texts: [core::ptr::null(), core::ptr::null()] },
    qtet_kcontrol_private { bit: MCR_IN34_MON12, set_register: Some(set_mcr), get_register: Some(get_mcr), texts: [core::ptr::null(), core::ptr::null()] },
    qtet_kcontrol_private { bit: MCR_IN34_MON34, set_register: Some(set_mcr), get_register: Some(get_mcr), texts: [core::ptr::null(), core::ptr::null()] },
    qtet_kcontrol_private { bit: MCR_OUT12_MON34, set_register: Some(set_mcr), get_register: Some(get_mcr), texts: [core::ptr::null(), core::ptr::null()] },
    qtet_kcontrol_private { bit: MCR_OUT34_MON12, set_register: Some(set_mcr), get_register: Some(get_mcr), texts: [core::ptr::null(), core::ptr::null()] },
];

unsafe extern "C" fn qtet_enum_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let private: &qtet_kcontrol_private = &qtet_privates[(*kcontrol).private_value as usize];
    snd_ctl_enum_info(uinfo, 1, private.texts.len() as c_uint, private.texts.as_ptr())
}

unsafe extern "C" fn qtet_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let private: &qtet_kcontrol_private = &qtet_privates[(*kcontrol).private_value as usize];
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    (*ucontrol).value.integer.value[0] =
        if (private.get_register.unwrap())(ice) & private.bit != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn qtet_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let private: &qtet_kcontrol_private = &qtet_privates[(*kcontrol).private_value as usize];
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let old: c_uint;
    let new: c_uint;
    old = (private.get_register.unwrap())(ice);
    if (*ucontrol).value.integer.value[0] != 0 {
        new = old | private.bit;
    } else {
        new = old & !private.bit;
    }
    if old != new {
        (private.set_register.unwrap())(ice, new);
        return 1;
    }
    /* no change */
    0
}

const qtet_sw_info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int> =
    Some(snd_ctl_boolean_mono_info);

static qtet_controls: [snd_kcontrol_new; 13] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Master Playback Switch\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_mute_get), put: Some(qtet_mute_put), private_value: 0, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Phantom Power\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_php_get), put: Some(qtet_php_put), private_value: 0, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog In 1/2 Capture Switch\0".as_ptr() as *const c_char, info: Some(qtet_ain12_enum_info), get: Some(qtet_ain12_sw_get), put: Some(qtet_ain12_sw_put), private_value: 0, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog In 3/4 Capture Switch\0".as_ptr() as *const c_char, info: Some(qtet_enum_info), get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: AIN34_SEL as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"PCM In 1/2 Capture Switch\0".as_ptr() as *const c_char, info: Some(qtet_enum_info), get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: IN12_SEL as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"PCM In 3/4 Capture Switch\0".as_ptr() as *const c_char, info: Some(qtet_enum_info), get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: IN34_SEL as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Coax Output Source\0".as_ptr() as *const c_char, info: Some(qtet_enum_info), get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: COAX_OUT as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog In 1/2 to Monitor 1/2\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: IN12_MON12 as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog In 1/2 to Monitor 3/4\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: IN12_MON34 as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog In 3/4 to Monitor 1/2\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: IN34_MON12 as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Analog In 3/4 to Monitor 3/4\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: IN34_MON34 as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Output 1/2 to Monitor 3/4\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: OUT12_MON34 as _, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"Output 3/4 to Monitor 1/2\0".as_ptr() as *const c_char, info: qtet_sw_info, get: Some(qtet_sw_get), put: Some(qtet_sw_put), private_value: OUT34_MON12 as _, ..unsafe { core::mem::zeroed() } },
];

static follower_vols: [*const c_char; 3] = [
    PCM_12_PLAYBACK_VOLUME,
    PCM_34_PLAYBACK_VOLUME,
    core::ptr::null(),
];

/* static DECLARE_TLV_DB_SCALE(qtet_master_db_scale, -6350, 50, 1); */
static qtet_master_db_scale: [c_uint; 4] = TLV_DB_SCALE_ITEM(-6350, 50, 1);

unsafe extern "C" fn qtet_add_controls(ice: *mut snd_ice1712) -> c_int {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    let mut err: c_int;
    let mut i: usize;
    let vmaster: *mut snd_kcontrol;
    err = snd_ice1712_akm4xxx_build_controls(ice);
    if err < 0 {
        return err;
    }
    i = 0;
    while i < qtet_controls.len() {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&qtet_controls[i], ice as *mut c_void));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    /* Create virtual master control */
    vmaster = snd_ctl_make_virtual_master(
        b"Master Playback Volume\0".as_ptr() as *const c_char,
        qtet_master_db_scale.as_ptr(),
    );
    if vmaster.is_null() {
        return -ENOMEM;
    }
    err = snd_ctl_add((*ice).card, vmaster);
    if err < 0 {
        return err;
    }
    err = snd_ctl_add_followers((*ice).card, vmaster, follower_vols.as_ptr());
    if err < 0 {
        return err;
    }
    /* only capture SPDIF over AK4113 */
    snd_ak4113_build(
        (*spec).ak4113,
        (*(*(*ice).pcm).streams.as_ptr().offset(SNDRV_PCM_STREAM_CAPTURE as isize)).substream,
    )
}

unsafe extern "C" fn qtet_is_spdif_master(ice: *mut snd_ice1712) -> c_int {
    /* CPLD_SYNC_SEL: 0 = internal, 1 = external (i.e. spdif master) */
    if get_cpld(ice) & CPLD_SYNC_SEL != 0 { 1 } else { 0 }
}

unsafe extern "C" fn qtet_get_rate(ice: *mut snd_ice1712) -> c_uint {
    let mut i: usize;
    let result: c_uchar;

    result = (get_cpld(ice) & CPLD_CKS_MASK) as c_uchar;
    i = 0;
    while i < cks_vals.len() {
        if cks_vals[i] == result as c_uint {
            return qtet_rates[i];
        }
        i += 1;
    }
    0
}

fn get_cks_val(rate: c_int) -> c_int {
    let mut i: usize = 0;
    while i < qtet_rates.len() {
        if qtet_rates[i] == rate as c_uint {
            return cks_vals[i] as c_int;
        }
        i += 1;
    }
    0
}

/* setting new rate */
unsafe extern "C" fn qtet_set_rate(ice: *mut snd_ice1712, rate: c_uint) {
    let mut new: c_uint;
    let val: c_uchar;
    /* switching ice1724 to external clock - supplied by ext. circuits */
    val = inb(ICEMT1724(ice, RATE));
    outb(val | VT1724_SPDIF_MASTER, ICEMT1724(ice, RATE));

    new = (get_cpld(ice) & !CPLD_CKS_MASK) | get_cks_val(rate as c_int) as c_uint;
    /* switch to internal clock, drop CPLD_SYNC_SEL */
    new &= !CPLD_SYNC_SEL;
    /* dev_dbg(ice->card->dev, "QT - set_rate: old %x, new %x\n",
       get_cpld(ice), new); */
    set_cpld(ice, new);
}

unsafe extern "C" fn qtet_set_mclk(_ice: *mut snd_ice1712, _rate: c_uint) -> c_uchar {
    /* no change in master clock */
    0
}

/* setting clock to external - SPDIF */
unsafe extern "C" fn qtet_set_spdif_clock(ice: *mut snd_ice1712, type_: c_int) -> c_int {
    let old: c_uint;
    let mut new: c_uint;

    old = get_cpld(ice);
    new = old;
    new &= !(CPLD_CKS_MASK | CPLD_WORD_SEL);
    match type_ {
        EXT_SPDIF_TYPE => new |= CPLD_EXT_SPDIF,
        EXT_WORDCLOCK_1FS_TYPE => new |= CPLD_EXT_WORDCLOCK_1FS,
        EXT_WORDCLOCK_256FS_TYPE => new |= CPLD_EXT_WORDCLOCK_256FS,
        _ => snd_BUG(),
    }
    if old != new {
        set_cpld(ice, new);
        /* changed */
        return 1;
    }
    0
}

unsafe extern "C" fn qtet_get_spdif_master_type(ice: *mut snd_ice1712) -> c_int {
    let mut val: c_uint;
    let result: c_int;
    val = get_cpld(ice);
    /* checking only rate/clock-related bits */
    val &= CPLD_CKS_MASK | CPLD_WORD_SEL | CPLD_SYNC_SEL;
    if val & CPLD_SYNC_SEL == 0 {
        /* switched to internal clock, is not any external type */
        result = -1;
    } else {
        match val {
            CPLD_EXT_SPDIF => result = EXT_SPDIF_TYPE,
            CPLD_EXT_WORDCLOCK_1FS => result = EXT_WORDCLOCK_1FS_TYPE,
            CPLD_EXT_WORDCLOCK_256FS => result = EXT_WORDCLOCK_256FS_TYPE,
            _ => {
                /* undefined combination of external clock setup */
                snd_BUG();
                result = 0;
            }
        }
    }
    result
}

/* Called when ak4113 detects change in the input SPDIF stream */
unsafe extern "C" fn qtet_ak4113_change(ak4113_: *mut ak4113, _c0: c_uchar, c1: c_uchar) {
    let ice: *mut snd_ice1712 = (*ak4113_).change_callback_private as *mut snd_ice1712;
    let rate: c_int;
    if qtet_get_spdif_master_type(ice) == EXT_SPDIF_TYPE && c1 != 0 {
        /* only for SPDIF master mode, rate was changed */
        rate = snd_ak4113_external_rate(ak4113_);
        /* dev_dbg(ice->card->dev, "ak4113 - input rate changed to %d\n",
           rate); */
        qtet_akm_set_rate_val((*ice).akm, rate as c_uint);
    }
}

/*
 * If clock slaved to SPDIF-IN, setting runtime rate
 * to the detected external rate
 */
unsafe extern "C" fn qtet_spdif_in_open(ice: *mut snd_ice1712, substream: *mut snd_pcm_substream) {
    let spec: *mut qtet_spec = (*ice).spec as *mut qtet_spec;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rate: c_int;

    if qtet_get_spdif_master_type(ice) != EXT_SPDIF_TYPE {
        /* not external SPDIF, no rate limitation */
        return;
    }
    /* only external SPDIF can detect incoming sample rate */
    rate = snd_ak4113_external_rate((*spec).ak4113);
    if rate >= (*runtime).hw.rate_min && rate <= (*runtime).hw.rate_max {
        (*runtime).hw.rate_min = rate;
        (*runtime).hw.rate_max = rate;
    }
}

/*
 * initialize the chip
 */
unsafe extern "C" fn qtet_init(ice: *mut snd_ice1712) -> c_int {
    static ak4113_init_vals: [c_uchar; 7] = [
        /* AK4113_REG_PWRDN */ AK4113_RST | AK4113_PWN | AK4113_OCKS0 | AK4113_OCKS1,
        /* AK4113_REQ_FORMAT */ AK4113_DIF_I24I2S | AK4113_VTX | AK4113_DEM_OFF | AK4113_DEAU,
        /* AK4113_REG_IO0 */ AK4113_OPS2 | AK4113_TXE | AK4113_XTL_24_576M,
        /* AK4113_REG_IO1 */ AK4113_EFH_1024LRCLK | AK4113_IPS(0),
        /* AK4113_REG_INT0_MASK */ 0,
        /* AK4113_REG_INT1_MASK */ 0,
        /* AK4113_REG_DATDTS */ 0,
    ];
    let mut err: c_int;
    let spec: *mut qtet_spec;
    let ak: *mut snd_akm4xxx;
    let val: c_uchar;

    /* switching ice1724 to external clock - supplied by ext. circuits */
    val = inb(ICEMT1724(ice, RATE));
    outb(val | VT1724_SPDIF_MASTER, ICEMT1724(ice, RATE));

    spec = kzalloc_obj::<qtet_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    /* qtet is clocked by Xilinx array */
    (*ice).hw_rates = &qtet_rates_info;
    (*ice).is_spdif_master = Some(qtet_is_spdif_master);
    (*ice).get_rate = Some(qtet_get_rate);
    (*ice).set_rate = Some(qtet_set_rate);
    (*ice).set_mclk = Some(qtet_set_mclk);
    (*ice).set_spdif_clock = Some(qtet_set_spdif_clock);
    (*ice).get_spdif_master_type = Some(qtet_get_spdif_master_type);
    (*ice).ext_clock_names = ext_clock_names.as_ptr();
    (*ice).ext_clock_count = ext_clock_names.len() as c_uint;
    /* since Qtet can detect correct SPDIF-in rate, all streams can be
     * limited to this specific rate */
    (*ice).spdif.ops.open = Some(qtet_spdif_in_open);
    (*ice).pro_open = Some(qtet_spdif_in_open);
    (*ice).spec = spec as *mut c_void;

    /* Mute Off */
    /* SCR Initialize*/
    /* keep codec power down first */
    set_scr(ice, SCR_PHP);
    udelay(1);
    /* codec power up */
    set_scr(ice, SCR_PHP | SCR_CODEC_PDN);

    /* MCR Initialize */
    set_mcr(ice, 0);

    /* CPLD Initialize */
    set_cpld(ice, 0);

    (*ice).num_total_dacs = 2;
    (*ice).num_total_adcs = 2;

    (*ice).akm = kzalloc_objs::<snd_akm4xxx>(2);
    ak = (*ice).akm;
    if ak.is_null() {
        return -ENOMEM;
    }
    /* only one codec with two chips */
    (*ice).akm_codecs = 1;
    err = snd_ice1712_akm4xxx_init(ak, &akm_qtet_dac, core::ptr::null_mut(), ice);
    if err < 0 {
        return err;
    }
    err = snd_ak4113_create(
        (*ice).card,
        Some(qtet_ak4113_read),
        Some(qtet_ak4113_write),
        ak4113_init_vals.as_ptr(),
        ice as *mut c_void,
        &mut (*spec).ak4113,
    );
    if err < 0 {
        return err;
    }
    /* callback for codecs rate setting */
    (*(*spec).ak4113).change_callback = Some(qtet_ak4113_change);
    (*(*spec).ak4113).change_callback_private = ice as *mut c_void;
    /* AK41143 in Quartet can detect external rate correctly
     * (i.e. check_flags = 0) */
    (*(*spec).ak4113).check_flags = 0;

    proc_init(ice);

    qtet_set_rate(ice, 44100);
    0
}

static qtet_eeprom: [c_uchar; 13] = {
    let mut a = [0 as c_uchar; 13];
    a[ICE_EEP2_SYSCONF as usize] = 0x28; /* clock 256(24MHz), mpu401, 1xADC,
                                           1xDACs, SPDIF in */
    a[ICE_EEP2_ACLINK as usize] = 0x80; /* I2S */
    a[ICE_EEP2_I2S as usize] = 0x78; /* 96k, 24bit, 192k */
    a[ICE_EEP2_SPDIF as usize] = 0xc3; /* out-en, out-int, in, out-ext */
    a[ICE_EEP2_GPIO_DIR as usize] = 0x00; /* 0-7 inputs, switched to output
                                            only during output operations */
    a[ICE_EEP2_GPIO_DIR1 as usize] = 0xff; /* 8-15 outputs */
    a[ICE_EEP2_GPIO_DIR2 as usize] = 0x00;
    a[ICE_EEP2_GPIO_MASK as usize] = 0xff; /* changed only for OUT operations */
    a[ICE_EEP2_GPIO_MASK1 as usize] = 0x00;
    a[ICE_EEP2_GPIO_MASK2 as usize] = 0xff;

    a[ICE_EEP2_GPIO_STATE as usize] = 0x00; /* inputs */
    a[ICE_EEP2_GPIO_STATE1 as usize] = 0x7d; /* all 1, but GPIO_CPLD_RW
                                               and GPIO15 always zero */
    a[ICE_EEP2_GPIO_STATE2 as usize] = 0x00; /* inputs */
    a
};

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_qtet_cards: [snd_ice1712_card_info; 2] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_QTET,
        name: b"Infrasonic Quartet\0".as_ptr() as *const c_char,
        model: b"quartet\0".as_ptr() as *const c_char,
        chip_init: Some(qtet_init),
        build_controls: Some(qtet_add_controls),
        eeprom_size: core::mem::size_of_val(&qtet_eeprom) as c_uint,
        eeprom_data: qtet_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() }, /* terminator */
];

extern "C" {
    static SND_AK4620: c_int;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_uint;
    static ENOMEM: c_int;
    static VT1724_SPDIF_MASTER: c_uchar;
    static RATE: c_uint;
    static AK4113_RST: c_uchar;
    static AK4113_PWN: c_uchar;
    static AK4113_OCKS0: c_uchar;
    static AK4113_OCKS1: c_uchar;
    static AK4113_DIF_I24I2S: c_uchar;
    static AK4113_VTX: c_uchar;
    static AK4113_DEM_OFF: c_uchar;
    static AK4113_DEAU: c_uchar;
    static AK4113_OPS2: c_uchar;
    static AK4113_TXE: c_uchar;
    static AK4113_XTL_24_576M: c_uchar;
    static AK4113_EFH_1024LRCLK: c_uchar;
    static ICE_EEP2_SYSCONF: c_uint;
    static ICE_EEP2_ACLINK: c_uint;
    static ICE_EEP2_I2S: c_uint;
    static ICE_EEP2_SPDIF: c_uint;
    static ICE_EEP2_GPIO_DIR: c_uint;
    static ICE_EEP2_GPIO_DIR1: c_uint;
    static ICE_EEP2_GPIO_DIR2: c_uint;
    static ICE_EEP2_GPIO_MASK: c_uint;
    static ICE_EEP2_GPIO_MASK1: c_uint;
    static ICE_EEP2_GPIO_MASK2: c_uint;
    static ICE_EEP2_GPIO_STATE: c_uint;
    static ICE_EEP2_GPIO_STATE1: c_uint;
    static ICE_EEP2_GPIO_STATE2: c_uint;
    static VT1724_SUBDEVICE_QTET: c_uint;

    fn AK4113_IPS(x: c_uint) -> c_uchar;
    fn ICEMT1724(ice: *mut snd_ice1712, reg: c_uint) -> c_uint;
    fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4];
    fn snd_vt1724_write_i2c(ice: *mut snd_ice1712, addr: c_uint, reg: c_uchar, val: c_uchar);
    fn snd_vt1724_read_i2c(ice: *mut snd_ice1712, addr: c_uint, reg: c_uchar) -> c_uchar;
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_BUG();
    fn udelay(usecs: c_uint);
    fn snd_akm4xxx_get(ak: *mut snd_akm4xxx, chip: c_int, addr: c_uchar) -> c_uchar;
    fn snd_akm4xxx_write(ak: *mut snd_akm4xxx, chip: c_int, addr: c_uchar, data: c_uchar);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_ice1712_akm4xxx_build_controls(ice: *mut snd_ice1712) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_make_virtual_master(name: *const c_char, tlv: *const c_uint) -> *mut snd_kcontrol;
    fn snd_ctl_add_followers(
        card: *mut snd_card,
        master: *mut snd_kcontrol,
        followers: *const *const c_char,
    ) -> c_int;
    fn snd_ak4113_build(ak4113: *mut ak4113, substream: *mut snd_pcm_substream) -> c_int;
    fn inb(port: c_uint) -> c_uchar;
    fn outb(value: c_uchar, port: c_uint);
    fn snd_ak4113_external_rate(ak4113: *mut ak4113) -> c_int;
    fn snd_ice1712_akm4xxx_init(
        ak: *mut snd_akm4xxx,
        template: *const snd_akm4xxx,
        priv_: *mut c_void,
        ice: *mut snd_ice1712,
    ) -> c_int;
    fn snd_ak4113_create(
        card: *mut snd_card,
        read: Option<unsafe extern "C" fn(*mut c_void, c_uchar) -> c_uchar>,
        write: Option<unsafe extern "C" fn(*mut c_void, c_uchar, c_uchar)>,
        init_vals: *const c_uchar,
        private_data: *mut c_void,
        r_ak4113: *mut *mut ak4113,
    ) -> c_int;
}

extern "Rust" {
    fn kzalloc_obj<T>() -> *mut T;
    fn kzalloc_objs<T>(count: usize) -> *mut T;
}

#[repr(C)]
pub struct snd_ice1712 {
    gpio: snd_ice1712_gpio,
    gpio_mutex: mutex,
    spec: *mut c_void,
    card: *mut snd_card,
    hw_rates: *const snd_pcm_hw_constraint_list,
    is_spdif_master: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    get_rate: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_uint>,
    set_rate: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
    set_mclk: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint) -> c_uchar>,
    set_spdif_clock: Option<unsafe extern "C" fn(*mut snd_ice1712, c_int) -> c_int>,
    get_spdif_master_type: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    ext_clock_names: *const *const c_char,
    ext_clock_count: c_uint,
    spdif: snd_ice1712_spdif,
    pro_open: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_pcm_substream)>,
    num_total_dacs: c_uint,
    num_total_adcs: c_uint,
    akm: *mut snd_akm4xxx,
    akm_codecs: c_uint,
    pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_ice1712_gpio {
    get_dir: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_uint>,
    set_dir: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
    set_mask: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
    get_data: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_uint>,
    set_data: Option<unsafe extern "C" fn(*mut snd_ice1712, c_uint)>,
}

#[repr(C)]
pub struct snd_ice1712_spdif {
    ops: snd_ice1712_spdif_ops,
}

#[repr(C)]
pub struct snd_ice1712_spdif_ops {
    open: Option<unsafe extern "C" fn(*mut snd_ice1712, *mut snd_pcm_substream)>,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    count: c_uint,
    list: *const c_uint,
    mask: c_uint,
}

#[repr(C)]
pub struct snd_akm4xxx {
    type_: c_int,
    num_dacs: c_int,
    num_adcs: c_int,
    ops: snd_akm4xxx_ops,
    dac_info: *const snd_akm4xxx_dac_channel,
    adc_info: *const snd_akm4xxx_adc_channel,
    private_data: [*mut c_void; 4],
    num_chips: c_int,
}

#[repr(C)]
pub struct snd_akm4xxx_ops {
    write: Option<unsafe extern "C" fn(*mut snd_akm4xxx, c_int, c_uchar, c_uchar)>,
    set_rate_val: Option<unsafe extern "C" fn(*mut snd_akm4xxx, c_uint)>,
}

#[repr(C)]
pub struct snd_akm4xxx_dac_channel {
    name: *const c_char,
    num_channels: c_int,
}

#[repr(C)]
pub struct snd_akm4xxx_adc_channel {
    name: *const c_char,
    num_channels: c_int,
}

#[repr(C)]
pub struct snd_info_entry {
    private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: usize,
}

#[repr(C)]
pub struct ak4113 {
    change_callback: Option<unsafe extern "C" fn(*mut ak4113, c_uchar, c_uchar)>,
    change_callback_private: *mut c_void,
    check_flags: c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    rate_min: c_int,
    rate_max: c_int,
}

#[repr(C)]
pub struct snd_pcm {
    streams: [snd_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_pcm_stream {
    substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_ice1712_card_info {
    subvendor: c_uint,
    name: *const c_char,
    model: *const c_char,
    chip_init: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    build_controls: Option<unsafe extern "C" fn(*mut snd_ice1712) -> c_int>,
    eeprom_size: c_uint,
    eeprom_data: *const c_uchar,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
type c_long = core::ffi::c_long;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
