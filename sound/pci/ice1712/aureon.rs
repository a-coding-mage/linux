// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for Terratec Aureon cards
 *
 *      Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
 *
 * NOTES:
 *
 * - we reuse the struct snd_akm4xxx record for storing the wm8770 codec data.
 *   both wm and akm codecs are pretty similar, so we can integrate
 *   both controls in the future, once if wm codecs are reused in
 *   many boards.
 *
 * - DAC digital volumes are not implemented in the mixer.
 *   if they show better response than DAC analog volumes, we can use them
 *   instead.
 *
 *   Lowlevel functions for AudioTrak Prodigy 7.1 (and possibly 192) cards
 *      Copyright (c) 2003 Dimitromanolakis Apostolos <apostol@cs.utoronto.ca>
 *
 *   version 0.82: Stable / not all features work yet (no communication with AC97 secondary)
 *       added 64x/128x oversampling switch (should be 64x only for 96khz)
 *       fixed some recording labels (still need to check the rest)
 *       recording is working probably thanks to correct wm8770 initialization
 *
 *   version 0.5: Initial release:
 *           working: analog output, mixer, headphone amplifier switch
 *       not working: prety much everything else, at least i could verify that
 *                    we have no digital output, no capture, pretty bad clicks and poops
 *                    on mixer switch and other coll stuff.
 */

/* C dependencies removed from executable Rust:
 * linux/delay.h, linux/interrupt.h, linux/init.h, linux/slab.h, linux/mutex.h,
 * sound/core.h, ice1712.h, envy24ht.h, aureon.h, sound/tlv.h.
 * The names below intentionally reference symbols supplied by those files.
 */

#[repr(C)]
pub struct aureon_spec {
    pub stac9744: [u16; 64],
    pub cs8415_mux: u32,
    pub master: [u16; 2],
    pub vol: [u16; 8],
    pub pca9554_out: u8,
}

pub const WM_DAC_ATTEN: i32 = 0x00; /* DAC1-8 analog attenuation */
pub const WM_DAC_MASTER_ATTEN: i32 = 0x08; /* DAC master analog attenuation */
pub const WM_DAC_DIG_ATTEN: i32 = 0x09; /* DAC1-8 digital attenuation */
pub const WM_DAC_DIG_MASTER_ATTEN: i32 = 0x11; /* DAC master digital attenuation */
pub const WM_PHASE_SWAP: i32 = 0x12; /* DAC phase */
pub const WM_DAC_CTRL1: i32 = 0x13; /* DAC control bits */
pub const WM_MUTE: i32 = 0x14; /* mute controls */
pub const WM_DAC_CTRL2: i32 = 0x15; /* de-emphasis and zefo-flag */
pub const WM_INT_CTRL: i32 = 0x16; /* interface control */
pub const WM_MASTER: i32 = 0x17; /* master clock and mode */
pub const WM_POWERDOWN: i32 = 0x18; /* power-down controls */
pub const WM_ADC_GAIN: i32 = 0x19; /* ADC gain L(19)/R(1a) */
pub const WM_ADC_MUX: i32 = 0x1b; /* input MUX */
pub const WM_OUT_MUX1: i32 = 0x1c; /* output MUX */
pub const WM_OUT_MUX2: i32 = 0x1e; /* output MUX */
pub const WM_RESET: i32 = 0x1f; /* software reset */

pub const CS8415_CTRL1: i32 = 0x01;
pub const CS8415_CTRL2: i32 = 0x02;
pub const CS8415_QSUB: i32 = 0x14;
pub const CS8415_RATIO: i32 = 0x1E;
pub const CS8415_C_BUFFER: i32 = 0x20;
pub const CS8415_ID: i32 = 0x7F;

pub const PCA9554_DEV: u8 = 0x40; /* I2C device address */
pub const PCA9554_IN: u8 = 0x00; /* input port */
pub const PCA9554_OUT: u8 = 0x01; /* output port */
pub const PCA9554_INVERT: u8 = 0x02; /* input invert */
pub const PCA9554_DIR: u8 = 0x03; /* port directions */

pub const AUREON_AC97_STEREO: i64 = 0x80;
pub const WM_VOL_MAX: u16 = 100;
pub const WM_VOL_CNT: u16 = 101; /* 0dB .. -100dB */
pub const WM_VOL_MUTE: u16 = 0x8000;
pub const PCM_0dB: u16 = 0xff;
pub const PCM_RES: u16 = 128; /* -64dB */
pub const PCM_MIN: u16 = PCM_0dB - PCM_RES;

extern "C" {
    fn snd_ice1712_gpio_read(ice: *mut snd_ice1712) -> u32;
    fn snd_ice1712_gpio_write(ice: *mut snd_ice1712, data: u32);
    fn snd_ice1712_gpio_set_mask(ice: *mut snd_ice1712, mask: u32);
    fn snd_ice1712_gpio_set_dir(ice: *mut snd_ice1712, dir: u32);
    fn snd_ice1712_save_gpio_status(ice: *mut snd_ice1712);
    fn snd_ice1712_restore_gpio_status(ice: *mut snd_ice1712);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_ice1712;
    fn snd_ctl_enum_info(uinfo: *mut snd_ctl_elem_info, channels: u32, items: u32, texts: *const *const i8) -> i32;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, private_data: *mut snd_ice1712) -> *mut snd_kcontrol;
    fn dev_info(dev: *mut device, fmt: *const i8, ...);
    fn udelay(usecs: u32);
}

/* External types, constants, helper macros, TLV objects, and kernel allocation
 * helpers are intentionally dependency references supplied by translated headers.
 */

unsafe fn spec_mut(ice: *mut snd_ice1712) -> *mut aureon_spec {
    (*ice).spec as *mut aureon_spec
}

/*
 * Aureon Universe additional controls using PCA9554
 */

/*
 * Send data to pca9554
 */
unsafe fn aureon_pca9554_write(ice: *mut snd_ice1712, reg: u8, data: u8) {
    let mut tmp: u32;
    let mut i: i32;
    let mut j: i32;
    let dev: u8 = PCA9554_DEV; /* ID 0100000, write */
    let mut val: u8 = 0;

    tmp = snd_ice1712_gpio_read(ice);
    snd_ice1712_gpio_set_mask(
        ice,
        !(AUREON_SPI_MOSI | AUREON_SPI_CLK | AUREON_WM_RW | AUREON_WM_CS | AUREON_CS8415_CS),
    );
    tmp |= AUREON_WM_RW;
    tmp |= AUREON_CS8415_CS | AUREON_WM_CS; /* disable SPI devices */

    tmp &= !AUREON_SPI_MOSI;
    tmp &= !AUREON_SPI_CLK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(50);

    /*
     * send i2c stop condition and start condition
     * to obtain sane state
     */
    tmp |= AUREON_SPI_CLK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(50);
    tmp |= AUREON_SPI_MOSI;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(100);
    tmp &= !AUREON_SPI_MOSI;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(50);
    tmp &= !AUREON_SPI_CLK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(100);

    /*
     * send device address, command and value,
     * skipping ack cycles in between
     */
    j = 0;
    while j < 3 {
        match j {
            0 => val = dev,
            1 => val = reg,
            2 => val = data,
            _ => {}
        }
        i = 7;
        while i >= 0 {
            tmp &= !AUREON_SPI_CLK;
            snd_ice1712_gpio_write(ice, tmp);
            udelay(40);
            if (val as u32 & (1u32 << i)) != 0 {
                tmp |= AUREON_SPI_MOSI;
            } else {
                tmp &= !AUREON_SPI_MOSI;
            }
            snd_ice1712_gpio_write(ice, tmp);
            udelay(40);
            tmp |= AUREON_SPI_CLK;
            snd_ice1712_gpio_write(ice, tmp);
            udelay(40);
            i -= 1;
        }
        tmp &= !AUREON_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(40);
        tmp |= AUREON_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(40);
        tmp &= !AUREON_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(40);
        j += 1;
    }
    tmp &= !AUREON_SPI_CLK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(40);
    tmp &= !AUREON_SPI_MOSI;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(40);
    tmp |= AUREON_SPI_CLK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(50);
    tmp |= AUREON_SPI_MOSI;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(100);
}

unsafe fn aureon_universe_inmux_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    static TEXTS: [*const i8; 3] = [c"Internal Aux".as_ptr(), c"Wavetable".as_ptr(), c"Rear Line-In".as_ptr()];
    snd_ctl_enum_info(uinfo, 1, 3, TEXTS.as_ptr())
}

unsafe fn aureon_universe_inmux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    (*ucontrol).value.enumerated.item[0] = (*spec).pca9554_out as _;
    0
}

unsafe fn aureon_universe_inmux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let oval: u8;
    let nval: u8;
    let change: i32;

    nval = (*ucontrol).value.enumerated.item[0] as u8;
    if nval >= 3 {
        return -EINVAL;
    }
    snd_ice1712_save_gpio_status(ice);
    oval = (*spec).pca9554_out;
    change = (oval != nval) as i32;
    if change != 0 {
        aureon_pca9554_write(ice, PCA9554_OUT, nval);
        (*spec).pca9554_out = nval;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn aureon_ac97_write(ice: *mut snd_ice1712, reg: u16, val: u16) {
    let spec = spec_mut(ice);
    let mut tmp: u32;

    /* Send address to XILINX chip */
    tmp = (snd_ice1712_gpio_read(ice) & !0xFF) | ((reg & 0x7F) as u32);
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp |= AUREON_AC97_ADDR;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp &= !AUREON_AC97_ADDR;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);

    /* Send low-order byte to XILINX chip */
    tmp &= !AUREON_AC97_DATA_MASK;
    tmp |= (val as u32) & AUREON_AC97_DATA_MASK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp |= AUREON_AC97_DATA_LOW;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp &= !AUREON_AC97_DATA_LOW;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);

    /* Send high-order byte to XILINX chip */
    tmp &= !AUREON_AC97_DATA_MASK;
    tmp |= ((val >> 8) as u32) & AUREON_AC97_DATA_MASK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp |= AUREON_AC97_DATA_HIGH;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp &= !AUREON_AC97_DATA_HIGH;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);

    /* Instruct XILINX chip to parse the data to the STAC9744 chip */
    tmp |= AUREON_AC97_COMMIT;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);
    tmp &= !AUREON_AC97_COMMIT;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(10);

    /* Store the data in out private buffer */
    (*spec).stac9744[((reg & 0x7F) >> 1) as usize] = val;
}

unsafe fn aureon_ac97_read(ice: *mut snd_ice1712, reg: u16) -> u16 {
    let spec = spec_mut(ice);
    (*spec).stac9744[((reg & 0x7F) >> 1) as usize]
}

/*
 * Initialize STAC9744 chip
 */
unsafe fn aureon_ac97_init(ice: *mut snd_ice1712) -> i32 {
    let spec = spec_mut(ice);
    let mut i: usize;
    static AC97_DEFAULTS: [u16; 37] = [
        0x00, 0x9640, 0x02, 0x8000, 0x04, 0x8000, 0x06, 0x8000, 0x0C, 0x8008,
        0x0E, 0x8008, 0x10, 0x8808, 0x12, 0x8808, 0x14, 0x8808, 0x16, 0x8808,
        0x18, 0x8808, 0x1C, 0x8000, 0x26, 0x000F, 0x28, 0x0201, 0x2C, 0xBB80,
        0x32, 0xBB80, 0x7C, 0x8384, 0x7E, 0x7644, u16::MAX,
    ];
    let mut tmp: u32;

    /* Cold reset */
    tmp = (snd_ice1712_gpio_read(ice) | AUREON_AC97_RESET) & !AUREON_AC97_DATA_MASK;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(3);
    tmp &= !AUREON_AC97_RESET;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(3);
    tmp |= AUREON_AC97_RESET;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(3);

    (*spec).stac9744 = [0; 64];
    i = 0;
    while AC97_DEFAULTS[i] != u16::MAX {
        (*spec).stac9744[(AC97_DEFAULTS[i] >> 1) as usize] = AC97_DEFAULTS[i + 1];
        i += 2;
    }

    /* Unmute AC'97 master volume permanently - muting is done by WM8770 */
    aureon_ac97_write(ice, AC97_MASTER as u16, 0x0000);
    0
}

unsafe fn aureon_ac97_vol_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = if ((*kcontrol).private_value & AUREON_AC97_STEREO) != 0 { 2 } else { 1 };
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 31;
    0
}

unsafe fn aureon_ac97_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    let vol = aureon_ac97_read(ice, ((*kcontrol).private_value & 0x7F) as u16);
    (*ucontrol).value.integer.value[0] = (0x1F - (vol & 0x1F)) as _;
    if ((*kcontrol).private_value & AUREON_AC97_STEREO) != 0 {
        (*ucontrol).value.integer.value[1] = (0x1F - ((vol >> 8) & 0x1F)) as _;
    }
    0
}

unsafe fn aureon_ac97_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let ovol: u16;
    let mut nvol: u16;
    let change: i32;

    snd_ice1712_save_gpio_status(ice);
    ovol = aureon_ac97_read(ice, ((*kcontrol).private_value & 0x7F) as u16);
    nvol = ((0x1F - (*ucontrol).value.integer.value[0]) as u16) & 0x001F;
    if ((*kcontrol).private_value & AUREON_AC97_STEREO) != 0 {
        nvol |= (((0x1F - (*ucontrol).value.integer.value[1]) as u16) << 8) & 0x1F00;
    }
    nvol |= ovol & !0x1F1F;
    change = (ovol != nvol) as i32;
    if change != 0 {
        aureon_ac97_write(ice, ((*kcontrol).private_value & 0x7F) as u16, nvol);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

/* AC'97 mute controls */
/* #define aureon_ac97_mute_info snd_ctl_boolean_mono_info */
/* AC'97 mute controls */
/* #define aureon_ac97_micboost_info snd_ctl_boolean_mono_info */
/* AC'97 master playback mute controls (Mute on WM8770 chip) */
/* #define aureon_ac97_mmute_info snd_ctl_boolean_mono_info */
/* #define aureon_mono_bool_info snd_ctl_boolean_mono_info */
/* #define wm_pcm_mute_info snd_ctl_boolean_mono_info */
/* #define wm_master_mute_info snd_ctl_boolean_stereo_info */
/* #define wm_adc_mute_info snd_ctl_boolean_stereo_info */
/* #define aureon_cs8415_mute_info snd_ctl_boolean_mono_info */
/* #define aureon_hpamp_info snd_ctl_boolean_mono_info */
/* #define aureon_deemp_info snd_ctl_boolean_mono_info */

unsafe fn aureon_ac97_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    (*ucontrol).value.integer.value[0] =
        if (aureon_ac97_read(ice, ((*kcontrol).private_value & 0x7F) as u16) & 0x8000) != 0 { 0 } else { 1 };
    0
}

unsafe fn aureon_ac97_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let ovol: u16;
    let nvol: u16;
    let change: i32;
    snd_ice1712_save_gpio_status(ice);
    ovol = aureon_ac97_read(ice, ((*kcontrol).private_value & 0x7F) as u16);
    nvol = (if (*ucontrol).value.integer.value[0] != 0 { 0x0000 } else { 0x8000 }) | (ovol & !0x8000);
    change = (ovol != nvol) as i32;
    if change != 0 {
        aureon_ac97_write(ice, ((*kcontrol).private_value & 0x7F) as u16, nvol);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn aureon_ac97_micboost_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    (*ucontrol).value.integer.value[0] = if (aureon_ac97_read(ice, AC97_MIC as u16) & 0x0020) != 0 { 0 } else { 1 };
    0
}

unsafe fn aureon_ac97_micboost_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let ovol: u16;
    let nvol: u16;
    let change: i32;
    snd_ice1712_save_gpio_status(ice);
    ovol = aureon_ac97_read(ice, AC97_MIC as u16);
    nvol = (if (*ucontrol).value.integer.value[0] != 0 { 0x0000 } else { 0x0020 }) | (ovol & !0x0020);
    change = (ovol != nvol) as i32;
    if change != 0 {
        aureon_ac97_write(ice, AC97_MIC as u16, nvol);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

/*
 * write data in the SPI mode
 */
unsafe fn aureon_spi_write(ice: *mut snd_ice1712, cs: u32, data: u32, bits: i32) {
    let mut tmp: u32;
    let mut i: i32;
    let mosi: u32;
    let clk: u32;

    tmp = snd_ice1712_gpio_read(ice);
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71LT
        || (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71XT
    {
        snd_ice1712_gpio_set_mask(ice, !(PRODIGY_SPI_MOSI | PRODIGY_SPI_CLK | PRODIGY_WM_CS));
        mosi = PRODIGY_SPI_MOSI;
        clk = PRODIGY_SPI_CLK;
    } else {
        snd_ice1712_gpio_set_mask(
            ice,
            !(AUREON_WM_RW | AUREON_SPI_MOSI | AUREON_SPI_CLK | AUREON_WM_CS | AUREON_CS8415_CS),
        );
        mosi = AUREON_SPI_MOSI;
        clk = AUREON_SPI_CLK;
        tmp |= AUREON_WM_RW;
    }

    tmp &= !cs;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    i = bits - 1;
    while i >= 0 {
        tmp &= !clk;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        if (data & (1u32 << i)) != 0 {
            tmp |= mosi;
        } else {
            tmp &= !mosi;
        }
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        tmp |= clk;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        i -= 1;
    }
    tmp &= !clk;
    tmp |= cs;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp |= clk;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
}

/*
 * Read data in SPI mode
 */
unsafe fn aureon_spi_read(ice: *mut snd_ice1712, cs: u32, data: u32, bits: i32, buffer: *mut u8, size: i32) {
    let mut i: i32;
    let mut j: i32;
    let mut tmp: u32;

    tmp = (snd_ice1712_gpio_read(ice) & !AUREON_SPI_CLK) | AUREON_CS8415_CS | AUREON_WM_CS;
    snd_ice1712_gpio_write(ice, tmp);
    tmp &= !cs;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);

    i = bits - 1;
    while i >= 0 {
        if (data & (1u32 << i)) != 0 {
            tmp |= AUREON_SPI_MOSI;
        } else {
            tmp &= !AUREON_SPI_MOSI;
        }
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        tmp |= AUREON_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        tmp &= !AUREON_SPI_CLK;
        snd_ice1712_gpio_write(ice, tmp);
        udelay(1);
        i -= 1;
    }

    j = 0;
    while j < size {
        let mut outdata: u8 = 0;
        i = 7;
        while i >= 0 {
            tmp = snd_ice1712_gpio_read(ice);
            outdata <<= 1;
            outdata |= if (tmp & AUREON_SPI_MISO) != 0 { 1 } else { 0 };
            udelay(1);
            tmp |= AUREON_SPI_CLK;
            snd_ice1712_gpio_write(ice, tmp);
            udelay(1);
            tmp &= !AUREON_SPI_CLK;
            snd_ice1712_gpio_write(ice, tmp);
            udelay(1);
            i -= 1;
        }
        *buffer.offset(j as isize) = outdata;
        j += 1;
    }
    tmp |= cs;
    snd_ice1712_gpio_write(ice, tmp);
}

unsafe fn aureon_cs8415_get(ice: *mut snd_ice1712, reg: i32) -> u8 {
    let mut val: u8 = 0;
    aureon_spi_write(ice, AUREON_CS8415_CS, 0x2000 | reg as u32, 16);
    aureon_spi_read(ice, AUREON_CS8415_CS, 0x21, 8, &mut val, 1);
    val
}

unsafe fn aureon_cs8415_read(ice: *mut snd_ice1712, reg: i32, buffer: *mut u8, size: i32) {
    aureon_spi_write(ice, AUREON_CS8415_CS, 0x2000 | reg as u32, 16);
    aureon_spi_read(ice, AUREON_CS8415_CS, 0x21, 8, buffer, size);
}

unsafe fn aureon_cs8415_put(ice: *mut snd_ice1712, reg: i32, val: u8) {
    aureon_spi_write(ice, AUREON_CS8415_CS, 0x200000 | ((reg as u32) << 8) | val as u32, 24);
}

/*
 * get the current register value of WM codec
 */
unsafe fn wm_get(ice: *mut snd_ice1712, mut reg: i32) -> u16 {
    reg <<= 1;
    (((*ice).akm.offset(0)).as_ref().unwrap().images[reg as usize] as u16) << 8
        | (*ice).akm.offset(0).as_ref().unwrap().images[(reg + 1) as usize] as u16
}

/*
 * set the register value of WM codec
 */
unsafe fn wm_put_nocache(ice: *mut snd_ice1712, reg: i32, val: u16) {
    aureon_spi_write(
        ice,
        if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71LT
            || (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71XT
        {
            PRODIGY_WM_CS
        } else {
            AUREON_WM_CS
        },
        ((reg as u32) << 9) | ((val & 0x1ff) as u32),
        16,
    );
}

/*
 * set the register value of WM codec and remember it
 */
unsafe fn wm_put(ice: *mut snd_ice1712, mut reg: i32, val: u16) {
    wm_put_nocache(ice, reg, val);
    reg <<= 1;
    (*(*ice).akm).images[reg as usize] = (val >> 8) as u8;
    (*(*ice).akm).images[(reg + 1) as usize] = val as u8;
}

unsafe fn aureon_ac97_mmute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    (*ucontrol).value.integer.value[0] = ((wm_get(ice, WM_OUT_MUX1) >> 1) & 0x01) as _;
    0
}

unsafe fn aureon_ac97_mmute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let ovol: u16;
    let nvol: u16;
    let change: i32;
    snd_ice1712_save_gpio_status(ice);
    ovol = wm_get(ice, WM_OUT_MUX1);
    nvol = (ovol & !0x02) | if (*ucontrol).value.integer.value[0] != 0 { 0x02 } else { 0x00 };
    change = (ovol != nvol) as i32;
    if change != 0 {
        wm_put(ice, WM_OUT_MUX1, nvol);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

/* static const DECLARE_TLV_DB_SCALE(...) */
static DB_SCALE_WM_DAC: [u32; 4] = declare_tlv_db_scale!(-10000, 100, 1);
static DB_SCALE_WM_PCM: [u32; 4] = declare_tlv_db_scale!(-6400, 50, 1);
static DB_SCALE_WM_ADC: [u32; 4] = declare_tlv_db_scale!(-1200, 100, 0);
static DB_SCALE_AC97_MASTER: [u32; 4] = declare_tlv_db_scale!(-4650, 150, 0);
static DB_SCALE_AC97_GAIN: [u32; 4] = declare_tlv_db_scale!(-3450, 150, 0);

unsafe fn wm_set_vol(ice: *mut snd_ice1712, index: u32, vol: u16, master: u16) {
    let mut nvol: u8;
    if (master & WM_VOL_MUTE) != 0 || (vol & WM_VOL_MUTE) != 0 {
        nvol = 0;
    } else {
        nvol = (((vol % WM_VOL_CNT) * (master % WM_VOL_CNT)) / WM_VOL_MAX) as u8;
        nvol = nvol.wrapping_add(0x1b);
    }
    wm_put(ice, index as i32, nvol as u16);
    wm_put_nocache(ice, index as i32, 0x180 | nvol as u16);
}

unsafe fn wm_pcm_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    (*ucontrol).value.integer.value[0] = if (wm_get(ice, WM_MUTE) & 0x10) != 0 { 0 } else { 1 };
    0
}

unsafe fn wm_pcm_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let nval: u16;
    let oval: u16;
    let change: i32;
    snd_ice1712_save_gpio_status(ice);
    oval = wm_get(ice, WM_MUTE);
    nval = (oval & !0x10) | if (*ucontrol).value.integer.value[0] != 0 { 0 } else { 0x10 };
    change = (oval != nval) as i32;
    if change != 0 {
        wm_put(ice, WM_MUTE, nval);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_master_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = WM_VOL_MAX as _;
    0
}

unsafe fn wm_master_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let mut i = 0;
    while i < 2 {
        (*ucontrol).value.integer.value[i] = ((*spec).master[i] & !WM_VOL_MUTE) as _;
        i += 1;
    }
    0
}

unsafe fn wm_master_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let mut ch: usize;
    let mut change: i32 = 0;
    snd_ice1712_save_gpio_status(ice);
    ch = 0;
    while ch < 2 {
        let mut vol = (*ucontrol).value.integer.value[ch] as u16;
        if vol > WM_VOL_MAX {
            vol = WM_VOL_MAX;
        }
        vol |= (*spec).master[ch] & WM_VOL_MUTE;
        if vol != (*spec).master[ch] {
            let mut dac: i32;
            (*spec).master[ch] = vol;
            dac = 0;
            while dac < (*ice).num_total_dacs {
                wm_set_vol(ice, (WM_DAC_ATTEN + dac + ch as i32) as u32, (*spec).vol[(dac + ch as i32) as usize], (*spec).master[ch]);
                dac += 2;
            }
            change = 1;
        }
        ch += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_vol_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    let voices = (*kcontrol).private_value >> 8;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = voices as _;
    (*uinfo).value.integer.min = 0; /* mute (-101dB) */
    (*uinfo).value.integer.max = WM_VOL_MAX as _; /* 0dB */
    0
}

unsafe fn wm_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let voices = ((*kcontrol).private_value >> 8) as usize;
    let ofs = ((*kcontrol).private_value & 0xff) as usize;
    let mut i = 0;
    while i < voices {
        (*ucontrol).value.integer.value[i] = ((*spec).vol[ofs + i] & !WM_VOL_MUTE) as _;
        i += 1;
    }
    0
}

unsafe fn wm_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let voices = ((*kcontrol).private_value >> 8) as usize;
    let ofs = ((*kcontrol).private_value & 0xff) as usize;
    let mut change = 0;
    let mut i = 0;
    snd_ice1712_save_gpio_status(ice);
    while i < voices {
        let mut vol = (*ucontrol).value.integer.value[i] as u16;
        if vol > WM_VOL_MAX {
            vol = WM_VOL_MAX;
        }
        vol |= (*spec).vol[ofs + i] & WM_VOL_MUTE;
        if vol != (*spec).vol[ofs + i] {
            let idx = WM_DAC_ATTEN + ofs as i32 + i as i32;
            (*spec).vol[ofs + i] = vol;
            wm_set_vol(ice, idx as u32, (*spec).vol[ofs + i], (*spec).master[i]);
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_mute_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = ((*kcontrol).private_value >> 8) as _;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe fn wm_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let voices = ((*kcontrol).private_value >> 8) as usize;
    let ofs = ((*kcontrol).private_value & 0xFF) as usize;
    let mut i = 0;
    while i < voices {
        (*ucontrol).value.integer.value[i] = if ((*spec).vol[ofs + i] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
        i += 1;
    }
    0
}

unsafe fn wm_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let voices = ((*kcontrol).private_value >> 8) as usize;
    let ofs = ((*kcontrol).private_value & 0xFF) as usize;
    let mut change = 0;
    let mut i = 0;
    snd_ice1712_save_gpio_status(ice);
    while i < voices {
        let val = if ((*spec).vol[ofs + i] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
        if (*ucontrol).value.integer.value[i] != val {
            (*spec).vol[ofs + i] &= !WM_VOL_MUTE;
            (*spec).vol[ofs + i] |= if (*ucontrol).value.integer.value[i] != 0 { 0 } else { WM_VOL_MUTE };
            wm_set_vol(ice, (ofs + i) as u32, (*spec).vol[ofs + i], (*spec).master[i]);
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_master_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    (*ucontrol).value.integer.value[0] = if ((*spec).master[0] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
    (*ucontrol).value.integer.value[1] = if ((*spec).master[1] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
    0
}

unsafe fn wm_master_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    let mut change = 0;
    let mut i = 0usize;
    snd_ice1712_save_gpio_status(ice);
    while i < 2 {
        let val = if ((*spec).master[i] & WM_VOL_MUTE) != 0 { 0 } else { 1 };
        if (*ucontrol).value.integer.value[i] != val {
            let mut dac = 0;
            (*spec).master[i] &= !WM_VOL_MUTE;
            (*spec).master[i] |= if (*ucontrol).value.integer.value[i] != 0 { 0 } else { WM_VOL_MUTE };
            while dac < (*ice).num_total_dacs {
                wm_set_vol(ice, (WM_DAC_ATTEN + dac + i as i32) as u32, (*spec).vol[(dac + i as i32) as usize], (*spec).master[i]);
                dac += 2;
            }
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_pcm_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0; /* mute (-64dB) */
    (*uinfo).value.integer.max = PCM_RES as _; /* 0dB */
    0
}

unsafe fn wm_pcm_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    let mut val = wm_get(ice, WM_DAC_DIG_MASTER_ATTEN) & 0xff;
    val = if val > PCM_MIN { val - PCM_MIN } else { 0 };
    (*ucontrol).value.integer.value[0] = val as _;
    0
}

unsafe fn wm_pcm_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let ovol: u16;
    let mut nvol: u16 = (*ucontrol).value.integer.value[0] as u16;
    let mut change = 0;
    if nvol > PCM_RES {
        return -EINVAL;
    }
    snd_ice1712_save_gpio_status(ice);
    nvol = (if nvol != 0 { nvol + PCM_MIN } else { 0 }) & 0xff;
    ovol = wm_get(ice, WM_DAC_DIG_MASTER_ATTEN) & 0xff;
    if ovol != nvol {
        wm_put(ice, WM_DAC_DIG_MASTER_ATTEN, nvol); /* prelatch */
        wm_put_nocache(ice, WM_DAC_DIG_MASTER_ATTEN, nvol | 0x100); /* update */
        change = 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_adc_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let mut i = 0;
    /* C used guard(mutex)(&ice->gpio_mutex). */
    while i < 2 {
        let val = wm_get(ice, WM_ADC_GAIN + i as i32);
        (*ucontrol).value.integer.value[i] = ((!val >> 5) & 0x1) as _;
        i += 1;
    }
    0
}

unsafe fn wm_adc_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let mut i = 0;
    let mut change = 0;
    snd_ice1712_save_gpio_status(ice);
    while i < 2 {
        let old = wm_get(ice, WM_ADC_GAIN + i as i32);
        let new = (((!(*ucontrol).value.integer.value[i] as u16) << 5) & 0x20) | (old & !0x20);
        if new != old {
            wm_put(ice, WM_ADC_GAIN + i as i32, new);
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_adc_vol_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0; /* -12dB */
    (*uinfo).value.integer.max = 0x1f; /* 19dB */
    0
}

unsafe fn wm_adc_vol_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let mut i = 0;
    /* C used guard(mutex)(&ice->gpio_mutex). */
    while i < 2 {
        let idx = WM_ADC_GAIN + i as i32;
        let vol = wm_get(ice, idx) & 0x1f;
        (*ucontrol).value.integer.value[i] = vol as _;
        i += 1;
    }
    0
}

unsafe fn wm_adc_vol_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let mut i = 0;
    let mut change = 0;
    snd_ice1712_save_gpio_status(ice);
    while i < 2 {
        let idx = WM_ADC_GAIN + i as i32;
        let nvol = ((*ucontrol).value.integer.value[i] as u16) & 0x1f;
        let ovol = wm_get(ice, idx);
        if (ovol & 0x1f) != nvol {
            wm_put(ice, idx, nvol | (ovol & !0x1f));
            change = 1;
        }
        i += 1;
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn wm_adc_mux_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    static TEXTS: [*const i8; 5] = [c"CD".as_ptr(), c"Aux".as_ptr(), c"Line".as_ptr(), c"Mic".as_ptr(), c"AC97".as_ptr()];
    static UNIVERSE_TEXTS: [*const i8; 8] = [
        c"Aux1".as_ptr(), c"CD".as_ptr(), c"Phono".as_ptr(), c"Line".as_ptr(),
        c"Aux2".as_ptr(), c"Mic".as_ptr(), c"Aux3".as_ptr(), c"AC97".as_ptr(),
    ];
    let ice = snd_kcontrol_chip(kcontrol);
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_AUREON71_UNIVERSE {
        snd_ctl_enum_info(uinfo, 2, 8, UNIVERSE_TEXTS.as_ptr())
    } else {
        snd_ctl_enum_info(uinfo, 2, 5, TEXTS.as_ptr())
    }
}

unsafe fn wm_adc_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    /* C used guard(mutex)(&ice->gpio_mutex). */
    let val = wm_get(ice, WM_ADC_MUX);
    (*ucontrol).value.enumerated.item[0] = (val & 7) as _;
    (*ucontrol).value.enumerated.item[1] = ((val >> 4) & 7) as _;
    0
}

unsafe fn wm_adc_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    snd_ice1712_save_gpio_status(ice);
    let oval = wm_get(ice, WM_ADC_MUX);
    let mut nval = oval & !0x77;
    nval |= ((*ucontrol).value.enumerated.item[0] as u16) & 7;
    nval |= (((*ucontrol).value.enumerated.item[1] as u16) & 7) << 4;
    let change = (oval != nval) as i32;
    if change != 0 {
        wm_put(ice, WM_ADC_MUX, nval);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

/* CS8415 Input mux */
unsafe fn aureon_cs8415_mux_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    static AUREON_TEXTS: [*const i8; 2] = [c"CD".as_ptr(), c"Optical".as_ptr()];
    static PRODIGY_TEXTS: [*const i8; 2] = [c"CD".as_ptr(), c"Coax".as_ptr()];
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71 {
        snd_ctl_enum_info(uinfo, 1, 2, PRODIGY_TEXTS.as_ptr())
    } else {
        snd_ctl_enum_info(uinfo, 1, 2, AUREON_TEXTS.as_ptr())
    }
}

unsafe fn aureon_cs8415_mux_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    /* snd_ice1712_save_gpio_status(ice); */
    /* val = aureon_cs8415_get(ice, CS8415_CTRL2); */
    (*ucontrol).value.enumerated.item[0] = (*spec).cs8415_mux as _;
    /* snd_ice1712_restore_gpio_status(ice); */
    0
}

unsafe fn aureon_cs8415_mux_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let spec = spec_mut(ice);
    snd_ice1712_save_gpio_status(ice);
    let oval = aureon_cs8415_get(ice, CS8415_CTRL2) as u16;
    let mut nval = oval & !0x07;
    nval |= ((*ucontrol).value.enumerated.item[0] as u16) & 7;
    let change = (oval != nval) as i32;
    if change != 0 {
        aureon_cs8415_put(ice, CS8415_CTRL2, nval as u8);
    }
    snd_ice1712_restore_gpio_status(ice);
    (*spec).cs8415_mux = (*ucontrol).value.enumerated.item[0] as u32;
    change
}

unsafe fn aureon_cs8415_rate_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 192000;
    0
}

unsafe fn aureon_cs8415_rate_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let ratio = aureon_cs8415_get(ice, CS8415_RATIO);
    (*ucontrol).value.integer.value[0] = ratio as i32 as i64 * 750;
    0
}

unsafe fn aureon_cs8415_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    snd_ice1712_save_gpio_status(ice);
    (*ucontrol).value.integer.value[0] = if (aureon_cs8415_get(ice, CS8415_CTRL1) & 0x20) != 0 { 0 } else { 1 };
    snd_ice1712_restore_gpio_status(ice);
    0
}

unsafe fn aureon_cs8415_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    snd_ice1712_save_gpio_status(ice);
    let oval = aureon_cs8415_get(ice, CS8415_CTRL1);
    let nval = if (*ucontrol).value.integer.value[0] != 0 { oval & !0x20 } else { oval | 0x20 };
    let change = (oval != nval) as i32;
    if change != 0 {
        aureon_cs8415_put(ice, CS8415_CTRL1, nval);
    }
    snd_ice1712_restore_gpio_status(ice);
    change
}

unsafe fn aureon_cs8415_qsub_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = 10;
    0
}

unsafe fn aureon_cs8415_qsub_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    snd_ice1712_save_gpio_status(ice);
    aureon_cs8415_read(ice, CS8415_QSUB, (*ucontrol).value.bytes.data.as_mut_ptr(), 10);
    snd_ice1712_restore_gpio_status(ice);
    0
}

unsafe fn aureon_cs8415_spdif_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe fn aureon_cs8415_mask_get(_kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    core::ptr::write_bytes((*ucontrol).value.iec958.status.as_mut_ptr(), 0xFF, 24);
    0
}

unsafe fn aureon_cs8415_spdif_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    snd_ice1712_save_gpio_status(ice);
    aureon_cs8415_read(ice, CS8415_C_BUFFER, (*ucontrol).value.iec958.status.as_mut_ptr(), 24);
    snd_ice1712_restore_gpio_status(ice);
    0
}

/*
 * Headphone Amplifier
 */
unsafe fn aureon_set_headphone_amp(ice: *mut snd_ice1712, enable: i32) -> i32 {
    let mut tmp: u32;
    let tmp2: u32;
    tmp = snd_ice1712_gpio_read(ice);
    tmp2 = tmp;
    if enable != 0 {
        if (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71LT
            && (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71XT
        {
            tmp |= AUREON_HP_SEL;
        } else {
            tmp |= PRODIGY_HP_SEL;
        }
    } else if (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71LT
        && (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71XT
    {
        tmp &= !AUREON_HP_SEL;
    } else {
        tmp &= !PRODIGY_HP_SEL;
    }
    if tmp != tmp2 {
        snd_ice1712_gpio_write(ice, tmp);
        return 1;
    }
    0
}

unsafe fn aureon_get_headphone_amp(ice: *mut snd_ice1712) -> i32 {
    let tmp = snd_ice1712_gpio_read(ice);
    ((tmp & AUREON_HP_SEL) != 0) as i32
}

unsafe fn aureon_hpamp_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = aureon_get_headphone_amp(ice) as _;
    0
}

unsafe fn aureon_hpamp_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    aureon_set_headphone_amp(ice, (*ucontrol).value.integer.value[0] as i32)
}

/*
 * Deemphasis
 */
unsafe fn aureon_deemp_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = ((wm_get(ice, WM_DAC_CTRL2) & 0xf) == 0xf) as _;
    0
}

unsafe fn aureon_deemp_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let mut temp = wm_get(ice, WM_DAC_CTRL2) as i32;
    let temp2 = temp;
    if (*ucontrol).value.integer.value[0] != 0 {
        temp |= 0xf;
    } else {
        temp &= !0xf;
    }
    if temp != temp2 {
        wm_put(ice, WM_DAC_CTRL2, temp as u16);
        return 1;
    }
    0
}

/*
 * ADC Oversampling
 */
unsafe fn aureon_oversampling_info(_k: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    static TEXTS: [*const i8; 2] = [c"128x".as_ptr(), c"64x".as_ptr()];
    snd_ctl_enum_info(uinfo, 1, 2, TEXTS.as_ptr())
}

unsafe fn aureon_oversampling_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = ((wm_get(ice, WM_MASTER) & 0x8) == 0x8) as _;
    0
}

unsafe fn aureon_oversampling_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let ice = snd_kcontrol_chip(kcontrol);
    let mut temp = wm_get(ice, WM_MASTER) as i32;
    let temp2 = temp;
    if (*ucontrol).value.enumerated.item[0] != 0 {
        temp |= 0x8;
    } else {
        temp &= !0x8;
    }
    if temp != temp2 {
        wm_put(ice, WM_MASTER, temp as u16);
        return 1;
    }
    0
}

/*
 * mixers
 */
static AUREON_DAC_CONTROLS: [snd_kcontrol_new; 12] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Master Playback Switch".as_ptr(), info: Some(wm_master_mute_info), get: Some(wm_master_mute_get), put: Some(wm_master_mute_put), ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Master Playback Volume".as_ptr(), info: Some(wm_master_vol_info), get: Some(wm_master_vol_get), put: Some(wm_master_vol_put), tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_DAC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Front Playback Switch".as_ptr(), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (2 << 8) | 0, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Front Playback Volume".as_ptr(), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (2 << 8) | 0, tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_DAC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Rear Playback Switch".as_ptr(), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (2 << 8) | 2, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Rear Playback Volume".as_ptr(), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (2 << 8) | 2, tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_DAC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Center Playback Switch".as_ptr(), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (1 << 8) | 4, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Center Playback Volume".as_ptr(), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (1 << 8) | 4, tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_DAC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"LFE Playback Switch".as_ptr(), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (1 << 8) | 5, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"LFE Playback Volume".as_ptr(), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (1 << 8) | 5, tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_DAC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Side Playback Switch".as_ptr(), info: Some(wm_mute_info), get: Some(wm_mute_get), put: Some(wm_mute_put), private_value: (2 << 8) | 6, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Side Playback Volume".as_ptr(), info: Some(wm_vol_info), get: Some(wm_vol_get), put: Some(wm_vol_put), private_value: (2 << 8) | 6, tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_DAC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
];

static WM_CONTROLS: [snd_kcontrol_new; 8] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"PCM Playback Switch".as_ptr(), info: Some(wm_pcm_mute_info), get: Some(wm_pcm_mute_get), put: Some(wm_pcm_mute_put), ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"PCM Playback Volume".as_ptr(), info: Some(wm_pcm_vol_info), get: Some(wm_pcm_vol_get), put: Some(wm_pcm_vol_put), tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_PCM.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Capture Switch".as_ptr(), info: Some(wm_adc_mute_info), get: Some(wm_adc_mute_get), put: Some(wm_adc_mute_put), ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, name: c"Capture Volume".as_ptr(), info: Some(wm_adc_vol_info), get: Some(wm_adc_vol_get), put: Some(wm_adc_vol_put), tlv: snd_kcontrol_tlv { p: DB_SCALE_WM_ADC.as_ptr() }, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Capture Source".as_ptr(), info: Some(wm_adc_mux_info), get: Some(wm_adc_mux_get), put: Some(wm_adc_mux_put), private_value: 5, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"External Amplifier".as_ptr(), info: Some(aureon_hpamp_info), get: Some(aureon_hpamp_get), put: Some(aureon_hpamp_put), ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"DAC Deemphasis Switch".as_ptr(), info: Some(aureon_deemp_info), get: Some(aureon_deemp_get), put: Some(aureon_deemp_put), ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"ADC Oversampling".as_ptr(), info: Some(aureon_oversampling_info), get: Some(aureon_oversampling_get), put: Some(aureon_oversampling_put), ..unsafe { core::mem::zeroed() } },
];

static AC97_CONTROLS: [snd_kcontrol_new; 11] = ac97_controls_array!();
static UNIVERSE_AC97_CONTROLS: [snd_kcontrol_new; 13] = universe_ac97_controls_array!();
static CS8415_CONTROLS: [snd_kcontrol_new; 6] = cs8415_controls_array!();

unsafe fn aureon_add_controls(ice: *mut snd_ice1712) -> i32 {
    let mut i: u32;
    let mut counts: u32;
    let mut err: i32;

    counts = AUREON_DAC_CONTROLS.len() as u32;
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_AUREON51_SKY {
        counts -= 2; /* no side */
    }
    i = 0;
    while i < counts {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&AUREON_DAC_CONTROLS[i as usize], ice));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    i = 0;
    while i < WM_CONTROLS.len() as u32 {
        err = snd_ctl_add((*ice).card, snd_ctl_new1(&WM_CONTROLS[i as usize], ice));
        if err < 0 {
            return err;
        }
        i += 1;
    }

    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_AUREON71_UNIVERSE {
        i = 0;
        while i < UNIVERSE_AC97_CONTROLS.len() as u32 {
            err = snd_ctl_add((*ice).card, snd_ctl_new1(&UNIVERSE_AC97_CONTROLS[i as usize], ice));
            if err < 0 {
                return err;
            }
            i += 1;
        }
    } else if (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71LT
        && (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71XT
    {
        i = 0;
        while i < AC97_CONTROLS.len() as u32 {
            err = snd_ctl_add((*ice).card, snd_ctl_new1(&AC97_CONTROLS[i as usize], ice));
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }

    if (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71LT
        && (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71XT
    {
        let id: u8;
        snd_ice1712_save_gpio_status(ice);
        id = aureon_cs8415_get(ice, CS8415_ID);
        snd_ice1712_restore_gpio_status(ice);
        if id != 0x41 {
            dev_info((*(*ice).card).dev, c"No CS8415 chip. Skipping CS8415 controls.\n".as_ptr());
        } else {
            i = 0;
            while i < CS8415_CONTROLS.len() as u32 {
                let kctl = snd_ctl_new1(&CS8415_CONTROLS[i as usize], ice);
                if kctl.is_null() {
                    return -ENOMEM;
                }
                if i > 1 {
                    (*kctl).id.device = (*(*ice).pcm).device;
                }
                err = snd_ctl_add((*ice).card, kctl);
                if err < 0 {
                    return err;
                }
                i += 1;
            }
        }
    }

    0
}

/*
 * reset the chip
 */
unsafe fn aureon_reset(ice: *mut snd_ice1712) -> i32 {
    static WM_INITS_AUREON: [u16; 63] = [
        0x1b, 0x044, 0x1c, 0x00B, 0x1d, 0x009, 0x18, 0x000, 0x16, 0x122,
        0x17, 0x022, 0x00, 0, 0x01, 0, 0x02, 0, 0x03, 0, 0x04, 0, 0x05, 0,
        0x06, 0, 0x07, 0, 0x08, 0x100, 0x09, 0xff, 0x0a, 0xff, 0x0b, 0xff,
        0x0c, 0xff, 0x0d, 0xff, 0x0e, 0xff, 0x0f, 0xff, 0x10, 0xff, 0x11,
        0x1ff, 0x12, 0x000, 0x13, 0x090, 0x14, 0x000, 0x15, 0x000, 0x19,
        0x000, 0x1a, 0x000, u16::MAX,
    ];
    static WM_INITS_PRODIGY: [u16; 63] = [
        0x1b, 0x000, 0x1c, 0x009, 0x1d, 0x009, 0x18, 0x000, 0x16, 0x022,
        0x17, 0x006, 0x00, 0, 0x01, 0, 0x02, 0, 0x03, 0, 0x04, 0, 0x05, 0,
        0x06, 0, 0x07, 0, 0x08, 0x100, 0x09, 0x7f, 0x0a, 0x7f, 0x0b, 0x7f,
        0x0c, 0x7f, 0x0d, 0x7f, 0x0e, 0x7f, 0x0f, 0x7f, 0x10, 0x7f, 0x11,
        0x1FF, 0x12, 0x000, 0x13, 0x090, 0x14, 0x000, 0x15, 0x000, 0x19,
        0x000, 0x1a, 0x000, u16::MAX,
    ];
    static CS_INITS: [u16; 5] = [0x0441, 0x0180, 0x0201, 0x0605, u16::MAX];
    let mut tmp: u32;
    let mut p: *const u16;
    let err: i32;
    let spec = spec_mut(ice);

    err = aureon_ac97_init(ice);
    if err != 0 {
        return err;
    }
    snd_ice1712_gpio_set_dir(ice, 0x5fffff); /* fix this for the time being */

    /* reset the wm codec as the SPI mode */
    snd_ice1712_save_gpio_status(ice);
    snd_ice1712_gpio_set_mask(ice, !(AUREON_WM_RESET | AUREON_WM_CS | AUREON_CS8415_CS | AUREON_HP_SEL));
    tmp = snd_ice1712_gpio_read(ice);
    tmp &= !AUREON_WM_RESET;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp |= AUREON_WM_CS | AUREON_CS8415_CS;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp |= AUREON_WM_RESET;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);

    /* initialize WM8770 codec */
    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71
        || (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71LT
        || (*ice).eeprom.subvendor == VT1724_SUBDEVICE_PRODIGY71XT
    {
        p = WM_INITS_PRODIGY.as_ptr();
    } else {
        p = WM_INITS_AUREON.as_ptr();
    }
    while *p != u16::MAX {
        wm_put(ice, *p as i32, *p.add(1));
        p = p.add(2);
    }

    /* initialize CS8415A codec */
    if (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71LT
        && (*ice).eeprom.subvendor != VT1724_SUBDEVICE_PRODIGY71XT
    {
        p = CS_INITS.as_ptr();
        while *p != u16::MAX {
            aureon_spi_write(ice, AUREON_CS8415_CS, (*p as u32) | 0x200000, 24);
            p = p.add(1);
        }
        (*spec).cs8415_mux = 1;
        aureon_set_headphone_amp(ice, 1);
    }

    snd_ice1712_restore_gpio_status(ice);
    /* initialize PCA9554 pin directions & set default input */
    aureon_pca9554_write(ice, PCA9554_DIR, 0x00);
    aureon_pca9554_write(ice, PCA9554_OUT, 0x00); /* internal AUX */
    0
}

/*
 * suspend/resume
 */
/* CONFIG_PM_SLEEP */
unsafe fn aureon_resume(ice: *mut snd_ice1712) -> i32 {
    let spec = spec_mut(ice);
    let mut i: i32;
    let err = aureon_reset(ice);
    if err != 0 {
        return err;
    }
    /* workaround for poking volume with alsamixer after resume:
     * just set stored volume again */
    i = 0;
    while i < (*ice).num_total_dacs {
        wm_set_vol(ice, i as u32, (*spec).vol[i as usize], (*spec).master[(i % 2) as usize]);
        i += 1;
    }
    0
}

/*
 * initialize the chip
 */
unsafe fn aureon_init(ice: *mut snd_ice1712) -> i32 {
    let spec: *mut aureon_spec;
    let mut i: i32;
    let err: i32;

    spec = kzalloc_obj!(*spec);
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut _;

    if (*ice).eeprom.subvendor == VT1724_SUBDEVICE_AUREON51_SKY {
        (*ice).num_total_dacs = 6;
        (*ice).num_total_adcs = 2;
    } else {
        /* aureon 7.1 and prodigy 7.1 */
        (*ice).num_total_dacs = 8;
        (*ice).num_total_adcs = 2;
    }

    /* to remember the register values of CS8415 */
    (*ice).akm = kzalloc_obj!(snd_akm4xxx);
    if (*ice).akm.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;

    err = aureon_reset(ice);
    if err != 0 {
        return err;
    }

    (*spec).master[0] = WM_VOL_MUTE;
    (*spec).master[1] = WM_VOL_MUTE;
    i = 0;
    while i < (*ice).num_total_dacs {
        (*spec).vol[i as usize] = WM_VOL_MUTE;
        wm_set_vol(ice, i as u32, (*spec).vol[i as usize], (*spec).master[(i % 2) as usize]);
        i += 1;
    }

    /* CONFIG_PM_SLEEP:
     * ice->pm_resume = aureon_resume;
     * ice->pm_suspend_enabled = 1;
     */
    #[cfg(CONFIG_PM_SLEEP)]
    {
        (*ice).pm_resume = Some(aureon_resume);
        (*ice).pm_suspend_enabled = 1;
    }

    0
}

/*
 * Aureon boards don't provide the EEPROM data except for the vendor IDs.
 * hence the driver needs to sets up it properly.
 */
static AUREON51_EEPROM: [u8; 13] = eeprom_data! {
    ICE_EEP2_SYSCONF => 0x0a, ICE_EEP2_ACLINK => 0x80, ICE_EEP2_I2S => 0xfc,
    ICE_EEP2_SPDIF => 0xc3, ICE_EEP2_GPIO_DIR => 0xff, ICE_EEP2_GPIO_DIR1 => 0xff,
    ICE_EEP2_GPIO_DIR2 => 0x5f, ICE_EEP2_GPIO_MASK => 0x00, ICE_EEP2_GPIO_MASK1 => 0x00,
    ICE_EEP2_GPIO_MASK2 => 0x00, ICE_EEP2_GPIO_STATE => 0x00, ICE_EEP2_GPIO_STATE1 => 0x00,
    ICE_EEP2_GPIO_STATE2 => 0x00
};

static AUREON71_EEPROM: [u8; 13] = eeprom_data! {
    ICE_EEP2_SYSCONF => 0x0b, ICE_EEP2_ACLINK => 0x80, ICE_EEP2_I2S => 0xfc,
    ICE_EEP2_SPDIF => 0xc3, ICE_EEP2_GPIO_DIR => 0xff, ICE_EEP2_GPIO_DIR1 => 0xff,
    ICE_EEP2_GPIO_DIR2 => 0x5f, ICE_EEP2_GPIO_MASK => 0x00, ICE_EEP2_GPIO_MASK1 => 0x00,
    ICE_EEP2_GPIO_MASK2 => 0x00, ICE_EEP2_GPIO_STATE => 0x00, ICE_EEP2_GPIO_STATE1 => 0x00,
    ICE_EEP2_GPIO_STATE2 => 0x00
};
/* #define prodigy71_eeprom aureon71_eeprom */
static PRODIGY71_EEPROM: &[u8] = &AUREON71_EEPROM;

static AUREON71_UNIVERSE_EEPROM: [u8; 13] = eeprom_data! {
    ICE_EEP2_SYSCONF => 0x2b, ICE_EEP2_ACLINK => 0x80, ICE_EEP2_I2S => 0xfc,
    ICE_EEP2_SPDIF => 0xc3, ICE_EEP2_GPIO_DIR => 0xff, ICE_EEP2_GPIO_DIR1 => 0xff,
    ICE_EEP2_GPIO_DIR2 => 0x5f, ICE_EEP2_GPIO_MASK => 0x00, ICE_EEP2_GPIO_MASK1 => 0x00,
    ICE_EEP2_GPIO_MASK2 => 0x00, ICE_EEP2_GPIO_STATE => 0x00, ICE_EEP2_GPIO_STATE1 => 0x00,
    ICE_EEP2_GPIO_STATE2 => 0x00
};

static PRODIGY71LT_EEPROM: [u8; 13] = eeprom_data! {
    ICE_EEP2_SYSCONF => 0x4b, ICE_EEP2_ACLINK => 0x80, ICE_EEP2_I2S => 0xfc,
    ICE_EEP2_SPDIF => 0xc3, ICE_EEP2_GPIO_DIR => 0xff, ICE_EEP2_GPIO_DIR1 => 0xff,
    ICE_EEP2_GPIO_DIR2 => 0x5f, ICE_EEP2_GPIO_MASK => 0x00, ICE_EEP2_GPIO_MASK1 => 0x00,
    ICE_EEP2_GPIO_MASK2 => 0x00, ICE_EEP2_GPIO_STATE => 0x00, ICE_EEP2_GPIO_STATE1 => 0x00,
    ICE_EEP2_GPIO_STATE2 => 0x00
};
/* #define prodigy71xt_eeprom prodigy71lt_eeprom */
static PRODIGY71XT_EEPROM: &[u8] = &PRODIGY71LT_EEPROM;

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_aureon_cards: [snd_ice1712_card_info; 7] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_AUREON51_SKY,
        name: c"Terratec Aureon 5.1-Sky".as_ptr(),
        model: c"aureon51".as_ptr(),
        chip_init: Some(aureon_init),
        build_controls: Some(aureon_add_controls),
        eeprom_size: core::mem::size_of_val(&AUREON51_EEPROM) as _,
        eeprom_data: AUREON51_EEPROM.as_ptr(),
        driver: c"Aureon51".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_AUREON71_SPACE,
        name: c"Terratec Aureon 7.1-Space".as_ptr(),
        model: c"aureon71".as_ptr(),
        chip_init: Some(aureon_init),
        build_controls: Some(aureon_add_controls),
        eeprom_size: core::mem::size_of_val(&AUREON71_EEPROM) as _,
        eeprom_data: AUREON71_EEPROM.as_ptr(),
        driver: c"Aureon71".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_AUREON71_UNIVERSE,
        name: c"Terratec Aureon 7.1-Universe".as_ptr(),
        model: c"universe".as_ptr(),
        chip_init: Some(aureon_init),
        build_controls: Some(aureon_add_controls),
        eeprom_size: core::mem::size_of_val(&AUREON71_UNIVERSE_EEPROM) as _,
        eeprom_data: AUREON71_UNIVERSE_EEPROM.as_ptr(),
        driver: c"Aureon71Univ".as_ptr(), /* keep in 15 letters */
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PRODIGY71,
        name: c"Audiotrak Prodigy 7.1".as_ptr(),
        model: c"prodigy71".as_ptr(),
        chip_init: Some(aureon_init),
        build_controls: Some(aureon_add_controls),
        eeprom_size: core::mem::size_of_val(&AUREON71_EEPROM) as _,
        eeprom_data: PRODIGY71_EEPROM.as_ptr(),
        driver: c"Prodigy71".as_ptr(), /* should be identical with Aureon71 */
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PRODIGY71LT,
        name: c"Audiotrak Prodigy 7.1 LT".as_ptr(),
        model: c"prodigy71lt".as_ptr(),
        chip_init: Some(aureon_init),
        build_controls: Some(aureon_add_controls),
        eeprom_size: core::mem::size_of_val(&PRODIGY71LT_EEPROM) as _,
        eeprom_data: PRODIGY71LT_EEPROM.as_ptr(),
        driver: c"Prodigy71LT".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_PRODIGY71XT,
        name: c"Audiotrak Prodigy 7.1 XT".as_ptr(),
        model: c"prodigy71xt".as_ptr(),
        chip_init: Some(aureon_init),
        build_controls: Some(aureon_add_controls),
        eeprom_size: core::mem::size_of_val(&PRODIGY71LT_EEPROM) as _,
        eeprom_data: PRODIGY71XT_EEPROM.as_ptr(),
        driver: c"Prodigy71LT".as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info { ..unsafe { core::mem::zeroed() } }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
