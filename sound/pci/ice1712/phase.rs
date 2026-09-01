// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble ICE1724 (Envy24)
 *
 *   Lowlevel functions for Terratec PHASE 22
 *
 *	Copyright (c) 2005 Misha Zhilin <misha@epiphan.com>
 */

/* PHASE 22 overview:
 *   Audio controller: VIA Envy24HT-S (slightly trimmed down Envy24HT, 4in/4out)
 *   Analog chip: AK4524 (partially via Philip's 74HCT125)
 *   Digital receiver: CS8414-CS (supported in this release)
 *		PHASE 22 revision 2.0 and Terrasoniq/Musonik TS22PCI have CS8416
 *		(support status unknown, please test and report)
 *
 *   Envy connects to AK4524
 *	- CS directly from GPIO 10
 *	- CCLK via 74HCT125's gate #4 from GPIO 4
 *	- CDTI via 74HCT125's gate #2 from GPIO 5
 *		CDTI may be completely blocked by 74HCT125's gate #1
 *		controlled by GPIO 3
 */

/* PHASE 28 overview:
 *   Audio controller: VIA Envy24HT (full untrimmed version, 4in/8out)
 *   Analog chip: WM8770 (8 channel 192k DAC, 2 channel 96k ADC)
 *   Digital receiver: CS8414-CS (supported in this release)
 */

// C includes removed: linux/delay.h, linux/interrupt.h, linux/init.h,
// linux/slab.h, linux/mutex.h, sound/core.h, ice1712.h, envy24ht.h,
// phase.h, sound/tlv.h.
use crate::*;

/* AC97 register cache for Phase28 */
#[repr(C)]
pub struct phase28_spec {
    pub master: [c_ushort; 2],
    pub vol: [c_ushort; 8],
}

/* WM8770 registers */
pub const WM_DAC_ATTEN: c_int = 0x00; /* DAC1-8 analog attenuation */
pub const WM_DAC_MASTER_ATTEN: c_int = 0x08; /* DAC master analog attenuation */
pub const WM_DAC_DIG_ATTEN: c_int = 0x09; /* DAC1-8 digital attenuation */
pub const WM_DAC_DIG_MASTER_ATTEN: c_int = 0x11; /* DAC master digital attenuation */
pub const WM_PHASE_SWAP: c_int = 0x12; /* DAC phase */
pub const WM_DAC_CTRL1: c_int = 0x13; /* DAC control bits */
pub const WM_MUTE: c_int = 0x14; /* mute controls */
pub const WM_DAC_CTRL2: c_int = 0x15; /* de-emphasis and zefo-flag */
pub const WM_INT_CTRL: c_int = 0x16; /* interface control */
pub const WM_MASTER: c_int = 0x17; /* master clock and mode */
pub const WM_POWERDOWN: c_int = 0x18; /* power-down controls */
pub const WM_ADC_GAIN: c_int = 0x19; /* ADC gain L(19)/R(1a) */
pub const WM_ADC_MUX: c_int = 0x1b; /* input MUX */
pub const WM_OUT_MUX1: c_int = 0x1c; /* output MUX */
pub const WM_OUT_MUX2: c_int = 0x1e; /* output MUX */
pub const WM_RESET: c_int = 0x1f; /* software reset */

/*
 * Logarithmic volume values for WM8770
 * Computed as 20 * Log10(255 / x)
 */
static wm_vol: [c_uchar; 256] = [
    127, 48, 42, 39, 36, 34, 33, 31, 30, 29, 28, 27, 27, 26, 25, 25, 24, 24,
    23, 23, 22, 22, 21, 21, 21, 20, 20, 20, 19, 19, 19, 18, 18, 18, 18, 17,
    17, 17, 17, 16, 16, 16, 16, 15, 15, 15, 15, 15, 15, 14, 14, 14, 14, 14,
    13, 13, 13, 13, 13, 13, 13, 12, 12, 12, 12, 12, 12, 12, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 10, 10, 10, 10, 10, 10, 10, 10, 10, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];

pub const WM_VOL_MAX: usize = wm_vol.len() - 1;
pub const WM_VOL_MUTE: c_ushort = 0x8000;

static akm_phase22: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4524,
    num_dacs: 2,
    num_adcs: 2,
    ..unsafe { core::mem::zeroed() }
};

static akm_phase22_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 1,
    data_mask: 1 << 4,
    clk_mask: 1 << 5,
    cs_mask: 1 << 10,
    cs_addr: 1 << 10,
    cs_none: 0,
    add_flags: 1 << 3,
    mask_flags: 0,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn phase22_init(ice: *mut snd_ice1712) -> c_int {
    let mut ak: *mut snd_akm4xxx;
    let mut err: c_int;

    /* Configure DAC/ADC description for generic part of ice1724 */
    match (*ice).eeprom.subvendor {
        VT1724_SUBDEVICE_PHASE22 | VT1724_SUBDEVICE_TS22 => {
            (*ice).num_total_dacs = 2;
            (*ice).num_total_adcs = 2;
            (*ice).vt1720 = 1; /* Envy24HT-S have 16 bit wide GPIO */
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }

    /* Initialize analog chips */
    (*ice).akm = kzalloc_obj::<snd_akm4xxx>();
    ak = (*ice).akm;
    if ak.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;
    match (*ice).eeprom.subvendor {
        VT1724_SUBDEVICE_PHASE22 | VT1724_SUBDEVICE_TS22 => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_phase22, &akm_phase22_priv, ice);
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn phase22_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut err: c_int = 0;

    match (*ice).eeprom.subvendor {
        VT1724_SUBDEVICE_PHASE22 | VT1724_SUBDEVICE_TS22 => {
            err = snd_ice1712_akm4xxx_build_controls(ice);
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }
    0
}

static phase22_eeprom: [c_uchar; ICE_EEP2_GPIO_STATE2 + 1] = {
    let mut a = [0 as c_uchar; ICE_EEP2_GPIO_STATE2 + 1];
    a[ICE_EEP2_SYSCONF] = 0x28; /* clock 512, mpu 401, spdif-in/1xADC, 1xDACs */
    a[ICE_EEP2_ACLINK] = 0x80; /* I2S */
    a[ICE_EEP2_I2S] = 0xf0; /* vol, 96k, 24bit */
    a[ICE_EEP2_SPDIF] = 0xc3; /* out-en, out-int, spdif-in */
    a[ICE_EEP2_GPIO_DIR] = 0xff;
    a[ICE_EEP2_GPIO_DIR1] = 0xff;
    a[ICE_EEP2_GPIO_DIR2] = 0xff;
    a[ICE_EEP2_GPIO_MASK] = 0x00;
    a[ICE_EEP2_GPIO_MASK1] = 0x00;
    a[ICE_EEP2_GPIO_MASK2] = 0x00;
    a[ICE_EEP2_GPIO_STATE] = 0x00;
    a[ICE_EEP2_GPIO_STATE1] = 0x00;
    a[ICE_EEP2_GPIO_STATE2] = 0x00;
    a
};

static phase28_eeprom: [c_uchar; ICE_EEP2_GPIO_STATE2 + 1] = {
    let mut a = [0 as c_uchar; ICE_EEP2_GPIO_STATE2 + 1];
    a[ICE_EEP2_SYSCONF] = 0x2b; /* clock 512, mpu401, spdif-in/1xADC, 4xDACs */
    a[ICE_EEP2_ACLINK] = 0x80; /* I2S */
    a[ICE_EEP2_I2S] = 0xfc; /* vol, 96k, 24bit, 192k */
    a[ICE_EEP2_SPDIF] = 0xc3; /* out-en, out-int, spdif-in */
    a[ICE_EEP2_GPIO_DIR] = 0xff;
    a[ICE_EEP2_GPIO_DIR1] = 0xff;
    a[ICE_EEP2_GPIO_DIR2] = 0x5f;
    a[ICE_EEP2_GPIO_MASK] = 0x00;
    a[ICE_EEP2_GPIO_MASK1] = 0x00;
    a[ICE_EEP2_GPIO_MASK2] = 0x00;
    a[ICE_EEP2_GPIO_STATE] = 0x00;
    a[ICE_EEP2_GPIO_STATE1] = 0x00;
    a[ICE_EEP2_GPIO_STATE2] = 0x00;
    a
};

/*
 * write data in the SPI mode
 */
unsafe fn phase28_spi_write(ice: *mut snd_ice1712, cs: c_uint, data: c_uint, bits: c_int) {
    let mut tmp: c_uint;
    let mut i: c_int;

    tmp = snd_ice1712_gpio_read(ice);

    snd_ice1712_gpio_set_mask(ice, !(PHASE28_WM_RW | PHASE28_SPI_MOSI | PHASE28_SPI_CLK | PHASE28_WM_CS));
    tmp |= PHASE28_WM_RW;
    tmp &= !cs;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);

    i = bits - 1;
    while i >= 0 {
        tmp &= !PHASE28_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        if data & (1 << i) != 0 {
            tmp |= PHASE28_SPI_MOSI;
        } else {
            tmp &= !PHASE28_SPI_MOSI;
        }
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        tmp |= PHASE28_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        i -= 1;
    }

    tmp &= !PHASE28_SPI_CLK;
    tmp |= cs;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp |= PHASE28_SPI_CLK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
}

/*
 * get the current register value of WM codec
 */
unsafe fn wm_get(ice: *mut snd_ice1712, mut reg: c_int) -> c_ushort {
    reg <<= 1;
    (((*(*ice).akm.add(0)).images[reg as usize] as c_ushort) << 8)
        | (*(*ice).akm.add(0)).images[(reg + 1) as usize] as c_ushort
}

/*
 * set the register value of WM codec
 */
unsafe fn wm_put_nocache(ice: *mut snd_ice1712, reg: c_int, val: c_ushort) {
    phase28_spi_write(ice, PHASE28_WM_CS, ((reg << 9) as c_uint) | ((val & 0x1ff) as c_uint), 16);
}

/*
 * set the register value of WM codec and remember it
 */
unsafe fn wm_put(ice: *mut snd_ice1712, mut reg: c_int, val: c_ushort) {
    wm_put_nocache(ice, reg, val);
    reg <<= 1;
    (*(*ice).akm.add(0)).images[reg as usize] = (val >> 8) as c_uchar;
    (*(*ice).akm.add(0)).images[(reg + 1) as usize] = val as c_uchar;
}

unsafe fn wm_set_vol(ice: *mut snd_ice1712, index: c_uint, vol: c_ushort, master: c_ushort) {
    let nvol: c_uchar;

    if (master & WM_VOL_MUTE) != 0 || (vol & WM_VOL_MUTE) != 0 {
        nvol = 0;
    } else {
        nvol = 127 - wm_vol[((((vol & !WM_VOL_MUTE) as c_uint)
            * ((master & !WM_VOL_MUTE) as c_uint)) / 127 & WM_VOL_MAX as c_uint) as usize];
    }

    wm_put(ice, index as c_int, nvol as c_ushort);
    wm_put_nocache(ice, index as c_int, 0x180 | nvol as c_ushort);
}

/*
 * DAC mute control
 */
// #define wm_pcm_mute_info snd_ctl_boolean_mono_info
const wm_pcm_mute_info: snd_kcontrol_info_t = snd_ctl_boolean_mono_info;

unsafe extern "C" fn wm_pcm_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    guard_mutex(&mut (*ice).gpio_mutex);
    (*ucontrol).value.integer.value[0] = if (wm_get(ice, WM_MUTE) & 0x10) != 0 { 0 } else { 1 };
    0
}

unsafe extern "C" fn wm_pcm_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let mut nval: c_ushort;
    let oval: c_ushort;
    let change: c_int;

    snd_ice1712_save_gpio_status(ice);
    oval = wm_get(ice, WM_MUTE);
    nval = (oval & !0x10) | if (*ucontrol).value.integer.value[0] != 0 { 0 } else { 0x10 };
    change = (nval != oval) as c_int;
    if change != 0 {
        wm_put(ice, WM_MUTE, nval);
    }
    snd_ice1712_restore_gpio_status(ice);

    change
}

/*
 * Master volume attenuation mixer control
 */
unsafe extern "C" fn wm_master_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = WM_VOL_MAX as c_long;
    0
}

unsafe extern "C" fn wm_master_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let mut i: c_int;
    i = 0;
    while i < 2 {
        (*ucontrol).value.integer.value[i as usize] = ((*spec).master[i as usize] & !WM_VOL_MUTE) as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn wm_master_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let mut ch: c_int;
    let mut change: c_int = 0;

    snd_ice1712_save_gpio_status(ice);
    ch = 0;
    while ch < 2 {
        let mut vol: c_uint = (*ucontrol).value.integer.value[ch as usize] as c_uint;
        if vol <= WM_VOL_MAX as c_uint {
            vol |= ((*spec).master[ch as usize] & WM_VOL_MUTE) as c_uint;
            if vol as c_ushort != (*spec).master[ch as usize] {
                let mut dac: c_int;
                (*spec).master[ch as usize] = vol as c_ushort;
                dac = 0;
                while dac < (*ice).num_total_dacs {
                    wm_set_vol(
                        ice,
                        (WM_DAC_ATTEN + dac + ch) as c_uint,
                        (*spec).vol[(dac + ch) as usize],
                        (*spec).master[ch as usize],
                    );
                    dac += 2;
                }
                change = 1;
            }
        }
        ch += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe extern "C" fn phase28_init(ice: *mut snd_ice1712) -> c_int {
    static wm_inits_phase28: [c_ushort; 61] = [
        /* These come first to reduce init pop noise */
        0x1b, 0x044, /* ADC Mux (AC'97 source) */
        0x1c, 0x00B, /* Out Mux1 (VOUT1 = DAC+AUX, VOUT2 = DAC) */
        0x1d, 0x009, /* Out Mux2 (VOUT2 = DAC, VOUT3 = DAC) */
        0x18, 0x000, /* All power-up */
        0x16, 0x122, /* I2S, normal polarity, 24bit */
        0x17, 0x022, /* 256fs, slave mode */
        0x00, 0, 0x01, 0, 0x02, 0, 0x03, 0, 0x04, 0, 0x05, 0, 0x06, 0, 0x07, 0,
        0x08, 0x100, /* master analog mute */
        0x09, 0xff, 0x0a, 0xff, 0x0b, 0xff, 0x0c, 0xff, 0x0d, 0xff, 0x0e, 0xff,
        0x0f, 0xff, 0x10, 0xff,
        0x11, 0x1ff, /* master digital full */
        0x12, 0x000, /* phase normal */
        0x13, 0x090, /* unmute DAC L/R */
        0x14, 0x000, /* all unmute */
        0x15, 0x000, /* no deemphasis, no ZFLG */
        0x19, 0x000, /* -12dB ADC/L */
        0x1a, 0x000, /* -12dB ADC/R */
        -1i16 as c_ushort,
    ];

    let mut tmp: c_uint;
    let mut ak: *mut snd_akm4xxx;
    let mut spec: *mut phase28_spec;
    let mut p: *const c_ushort;
    let mut i: c_int;

    (*ice).num_total_dacs = 8;
    (*ice).num_total_adcs = 2;

    spec = kzalloc_obj::<phase28_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut _;

    /* Initialize analog chips */
    (*ice).akm = kzalloc_obj::<snd_akm4xxx>();
    ak = (*ice).akm;
    if ak.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;

    snd_ice1712_gpio_set_dir(ice, 0x5fffff); /* fix this for time being */

    /* reset the wm codec as the SPI mode */
    snd_ice1712_save_gpio_status(ice);
    snd_ice1712_gpio_set_mask(ice, !(PHASE28_WM_RESET | PHASE28_WM_CS | PHASE28_HP_SEL));

    tmp = snd_ice1712_gpio_read(ice);
    tmp &= !PHASE28_WM_RESET;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp |= PHASE28_WM_CS;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp |= PHASE28_WM_RESET;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);

    p = wm_inits_phase28.as_ptr();
    while *p != (-1i16 as c_ushort) {
        wm_put(ice, *p as c_int, *p.add(1));
        p = p.add(2);
    }

    snd_ice1712_restore_gpio_status(ice);

    (*spec).master[0] = WM_VOL_MUTE;
    (*spec).master[1] = WM_VOL_MUTE;
    i = 0;
    while i < (*ice).num_total_dacs {
        (*spec).vol[i as usize] = WM_VOL_MUTE;
        wm_set_vol(ice, i as c_uint, (*spec).vol[i as usize], (*spec).master[(i % 2) as usize]);
        i += 1;
    }

    0
}

/*
 * DAC volume attenuation mixer control
 */
unsafe extern "C" fn wm_vol_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let voices: c_int = ((*kcontrol).private_value >> 8) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = voices as c_uint;
    (*uinfo).value.integer.min = 0; /* mute (-101dB) */
    (*uinfo).value.integer.max = 0x7F; /* 0dB */
    0
}

unsafe extern "C" fn wm_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let mut i: c_int;
    let ofs: c_int;
    let voices: c_int;

    voices = ((*kcontrol).private_value >> 8) as c_int;
    ofs = ((*kcontrol).private_value & 0xff) as c_int;
    i = 0;
    while i < voices {
        (*ucontrol).value.integer.value[i as usize] = ((*spec).vol[(ofs + i) as usize] & !WM_VOL_MUTE) as c_long;
        i += 1;
    }
    0
}

unsafe extern "C" fn wm_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let mut i: c_int;
    let mut idx: c_int;
    let ofs: c_int;
    let voices: c_int;
    let mut change: c_int = 0;

    voices = ((*kcontrol).private_value >> 8) as c_int;
    ofs = ((*kcontrol).private_value & 0xff) as c_int;
    snd_ice1712_save_gpio_status(ice);
    i = 0;
    while i < voices {
        let mut vol: c_uint;
        vol = (*ucontrol).value.integer.value[i as usize] as c_uint;
        if vol <= 0x7f {
            vol |= ((*spec).vol[(ofs + i) as usize] & WM_VOL_MUTE) as c_uint;
            if vol as c_ushort != (*spec).vol[(ofs + i) as usize] {
                (*spec).vol[(ofs + i) as usize] = vol as c_ushort;
                idx = WM_DAC_ATTEN + ofs + i;
                wm_set_vol(ice, idx as c_uint, (*spec).vol[(ofs + i) as usize], (*spec).master[i as usize]);
                change = 1;
            }
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

/*
 * WM8770 mute control
 */
unsafe extern "C" fn wm_mute_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = ((*kcontrol).private_value >> 8) as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn wm_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let voices: c_int;
    let ofs: c_int;
    let mut i: c_int;

    voices = ((*kcontrol).private_value >> 8) as c_int;
    ofs = ((*kcontrol).private_value & 0xFF) as c_int;

    i = 0;
    while i < voices {
        (*ucontrol).value.integer.value[i as usize] =
            if ((*spec).vol[(ofs + i) as usize] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
        i += 1;
    }
    0
}

unsafe extern "C" fn wm_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let mut change: c_int = 0;
    let voices: c_int;
    let ofs: c_int;
    let mut i: c_int;

    voices = ((*kcontrol).private_value >> 8) as c_int;
    ofs = ((*kcontrol).private_value & 0xFF) as c_int;

    snd_ice1712_save_gpio_status(ice);
    i = 0;
    while i < voices {
        let val: c_int = if ((*spec).vol[(ofs + i) as usize] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
        if (*ucontrol).value.integer.value[i as usize] != val as c_long {
            (*spec).vol[(ofs + i) as usize] &= !WM_VOL_MUTE;
            (*spec).vol[(ofs + i) as usize] |= if (*ucontrol).value.integer.value[i as usize] != 0 { 0 } else { WM_VOL_MUTE };
            wm_set_vol(ice, (ofs + i) as c_uint, (*spec).vol[(ofs + i) as usize], (*spec).master[i as usize]);
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);

    change
}

/*
 * WM8770 master mute control
 */
// #define wm_master_mute_info snd_ctl_boolean_stereo_info
const wm_master_mute_info: snd_kcontrol_info_t = snd_ctl_boolean_stereo_info;

unsafe extern "C" fn wm_master_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;

    (*ucontrol).value.integer.value[0] = if ((*spec).master[0] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
    (*ucontrol).value.integer.value[1] = if ((*spec).master[1] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
    0
}

unsafe extern "C" fn wm_master_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let spec: *mut phase28_spec = (*ice).spec as *mut phase28_spec;
    let mut change: c_int = 0;
    let mut i: c_int;

    snd_ice1712_save_gpio_status(ice);
    i = 0;
    while i < 2 {
        let val: c_int = if ((*spec).master[i as usize] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
        if (*ucontrol).value.integer.value[i as usize] != val as c_long {
            let mut dac: c_int;
            (*spec).master[i as usize] &= !WM_VOL_MUTE;
            (*spec).master[i as usize] |= if (*ucontrol).value.integer.value[i as usize] != 0 { 0 } else { WM_VOL_MUTE };
            dac = 0;
            while dac < (*ice).num_total_dacs {
                wm_set_vol(
                    ice,
                    (WM_DAC_ATTEN + dac + i) as c_uint,
                    (*spec).vol[(dac + i) as usize],
                    (*spec).master[i as usize],
                );
                dac += 2;
            }
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);

    change
}

/* digital master volume */
pub const PCM_0dB: c_ushort = 0xff;
pub const PCM_RES: c_ushort = 128; /* -64dB */
pub const PCM_MIN: c_ushort = PCM_0dB - PCM_RES;

unsafe extern "C" fn wm_pcm_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0; /* mute (-64dB) */
    (*uinfo).value.integer.max = PCM_RES as c_long; /* 0dB */
    0
}

unsafe extern "C" fn wm_pcm_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let mut val: c_ushort;

    guard_mutex(&mut (*ice).gpio_mutex);
    val = wm_get(ice, WM_DAC_DIG_MASTER_ATTEN) & 0xff;
    val = if val > PCM_MIN { val - PCM_MIN } else { 0 };
    (*ucontrol).value.integer.value[0] = val as c_long;
    0
}

unsafe extern "C" fn wm_pcm_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let ovol: c_ushort;
    let mut nvol: c_ushort;
    let mut change: c_int = 0;

    nvol = (*ucontrol).value.integer.value[0] as c_ushort;
    if nvol > PCM_RES {
        return -EINVAL;
    }
    snd_ice1712_save_gpio_status(ice);
    nvol = (if nvol != 0 { nvol + PCM_MIN } else { 0 }) & 0xff;
    ovol = wm_get(ice, WM_DAC_DIG_MASTER_ATTEN) & 0xff;
    if ovol != nvol {
        wm_put(ice, WM_DAC_DIG_MASTER_ATTEN, nvol); /* prelatch */
        /* update */
        wm_put_nocache(ice, WM_DAC_DIG_MASTER_ATTEN, nvol | 0x100);
        change = 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

/*
 * Deemphasis
 */
// #define phase28_deemp_info snd_ctl_boolean_mono_info
const phase28_deemp_info: snd_kcontrol_info_t = snd_ctl_boolean_mono_info;

unsafe extern "C" fn phase28_deemp_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = ((wm_get(ice, WM_DAC_CTRL2) & 0xf) == 0xf) as c_long;
    0
}

unsafe extern "C" fn phase28_deemp_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    let mut temp: c_int;
    let temp2: c_int;
    temp = wm_get(ice, WM_DAC_CTRL2) as c_int;
    temp2 = temp;
    if (*ucontrol).value.integer.value[0] != 0 {
        temp |= 0xf;
    } else {
        temp &= !0xf;
    }
    if temp != temp2 {
        wm_put(ice, WM_DAC_CTRL2, temp as c_ushort);
        return 1;
    }
    0
}

/*
 * ADC Oversampling
 */
unsafe extern "C" fn phase28_oversampling_info(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static texts: [*const c_char; 2] = [c_str!("128x"), c_str!("64x")];

    snd_ctl_enum_info(uinfo, 1, 2, texts.as_ptr())
}

unsafe extern "C" fn phase28_oversampling_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = ((wm_get(ice, WM_MASTER) & 0x8) == 0x8) as c_uint;
    0
}

unsafe extern "C" fn phase28_oversampling_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let mut temp: c_int;
    let temp2: c_int;
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    temp = wm_get(ice, WM_MASTER) as c_int;
    temp2 = temp;

    if (*ucontrol).value.enumerated.item[0] != 0 {
        temp |= 0x8;
    } else {
        temp &= !0x8;
    }

    if temp != temp2 {
        wm_put(ice, WM_MASTER, temp as c_ushort);
        return 1;
    }
    0
}

static db_scale_wm_dac: [c_uint; 4] = declare_tlv_db_scale!(-12700, 100, 1);
static db_scale_wm_pcm: [c_uint; 4] = declare_tlv_db_scale!(-6400, 50, 1);

static phase28_dac_controls: [snd_kcontrol_new; 12] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c_str!("Master Playback Switch"),
        info: Some(wm_master_mute_info),
        get: Some(wm_master_mute_get),
        put: Some(wm_master_mute_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: c_str!("Master Playback Volume"),
        info: Some(wm_master_vol_info),
        get: Some(wm_master_vol_get),
        put: Some(wm_master_vol_put),
        tlv: snd_kcontrol_new_tlv { p: db_scale_wm_dac.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c_str!("Front Playback Switch"), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (2 << 8) | 0, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c_str!("Front Playback Volume"), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (2 << 8) | 0, tlv: snd_kcontrol_new_tlv { p: db_scale_wm_dac.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c_str!("Rear Playback Switch"), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (2 << 8) | 2, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c_str!("Rear Playback Volume"), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (2 << 8) | 2, tlv: snd_kcontrol_new_tlv { p: db_scale_wm_dac.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c_str!("Center Playback Switch"), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (1 << 8) | 4, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c_str!("Center Playback Volume"), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (1 << 8) | 4, tlv: snd_kcontrol_new_tlv { p: db_scale_wm_dac.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c_str!("LFE Playback Switch"), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (1 << 8) | 5, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c_str!("LFE Playback Volume"), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (1 << 8) | 5, tlv: snd_kcontrol_new_tlv { p: db_scale_wm_dac.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c_str!("Side Playback Switch"), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (2 << 8) | 6, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c_str!("Side Playback Volume"), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (2 << 8) | 6, tlv: snd_kcontrol_new_tlv { p: db_scale_wm_dac.as_ptr() }, ..unsafe { core::mem::zeroed() } },
];

static wm_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c_str!("PCM Playback Switch"),
        info: Some(wm_pcm_mute_info),
        get: Some(wm_pcm_mute_get),
        put: Some(wm_pcm_mute_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: c_str!("PCM Playback Volume"),
        info: Some(wm_pcm_vol_info),
        get: Some(wm_pcm_vol_get),
        put: Some(wm_pcm_vol_put),
        tlv: snd_kcontrol_new_tlv { p: db_scale_wm_pcm.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c_str!("DAC Deemphasis Switch"),
        info: Some(phase28_deemp_info),
        get: Some(phase28_deemp_get),
        put: Some(phase28_deemp_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c_str!("ADC Oversampling"),
        info: Some(phase28_oversampling_info),
        get: Some(phase28_oversampling_get),
        put: Some(phase28_oversampling_put),
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe extern "C" fn phase28_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut i: c_uint;
    let counts: c_uint;
    let mut err: c_int;

    counts = phase28_dac_controls.len() as c_uint;
    i = 0;
    while i < counts {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&phase28_dac_controls[i as usize], ice as *mut _));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    i = 0;
    while i < wm_controls.len() as c_uint {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&wm_controls[i as usize], ice as *mut _));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

#[no_mangle]
pub static mut snd_vt1724_phase_cards: [snd_ice1712_card_info; 4] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PHASE22,
        name: c_str!("Terratec PHASE 22"),
        model: c_str!("phase22"),
        chip_init: Some(phase22_init),
        build_controls: Some(phase22_add_controls),
        eeprom_size: phase22_eeprom.len(),
        eeprom_data: phase22_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PHASE28,
        name: c_str!("Terratec PHASE 28"),
        model: c_str!("phase28"),
        chip_init: Some(phase28_init),
        build_controls: Some(phase28_add_controls),
        eeprom_size: phase28_eeprom.len(),
        eeprom_data: phase28_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_TS22,
        name: c_str!("Terrasoniq TS22 PCI"),
        model: c_str!("TS22"),
        chip_init: Some(phase22_init),
        build_controls: Some(phase22_add_controls),
        eeprom_size: phase22_eeprom.len(),
        eeprom_data: phase22_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
