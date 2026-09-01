// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for M-Audio Audiophile 192, Revolution 7.1 and 5.1
 *
 *	Copyright (c) 2003 Takashi Iwai <tiwai@suse.de>
 */

// C dependencies: linux/delay.h, linux/interrupt.h, linux/init.h,
// linux/slab.h, sound/core.h, ice1712.h, envy24ht.h, revo.h

use core::ffi::{c_int, c_uchar, c_uint, c_void};
use core::ptr::{null, null_mut};

/* a non-standard I2C device for revo51 */
#[repr(C)]
pub struct revo51_spec {
    pub dev: *mut snd_i2c_device,
    pub pt2258: *mut snd_pt2258,
    pub ak4114: *mut ak4114,
}

unsafe extern "C" fn revo_i2s_mclk_changed(ice: *mut snd_ice1712) {
    /* assert PRST# to converters; MT05 bit 7 */
    outb(
        (inb(ICEMT1724(ice, AC97_CMD)) | 0x80) as c_uchar,
        ICEMT1724(ice, AC97_CMD),
    );
    mdelay(5);
    /* deassert PRST# */
    outb(
        (inb(ICEMT1724(ice, AC97_CMD)) & !0x80) as c_uchar,
        ICEMT1724(ice, AC97_CMD),
    );
}

/*
 * change the rate of Envy24HT, AK4355 and AK4381
 */
unsafe extern "C" fn revo_set_rate_val(ak: *mut snd_akm4xxx, rate: c_uint) {
    let old: c_uchar;
    let mut tmp: c_uchar;
    let dfs: c_uchar;
    let reg: c_int;
    let shift: c_int;

    if rate == 0 {
        /* no hint - S/PDIF input is master, simply return */
        return;
    }

    /* adjust DFS on codecs */
    if rate > 96000 {
        dfs = 2;
    } else if rate > 48000 {
        dfs = 1;
    } else {
        dfs = 0;
    }

    if (*ak).type_ == SND_AK4355 || (*ak).type_ == SND_AK4358 {
        reg = 2;
        shift = 4;
    } else {
        reg = 1;
        shift = 3;
    }
    tmp = snd_akm4xxx_get(ak, 0, reg);
    old = ((tmp >> shift) & 0x03) as c_uchar;
    if old == dfs {
        return;
    }

    /* reset DFS */
    snd_akm4xxx_reset(ak, 1);
    tmp = snd_akm4xxx_get(ak, 0, reg);
    tmp &= !(0x03 << shift) as c_uchar;
    tmp |= (dfs as c_int << shift) as c_uchar;
    /* snd_akm4xxx_write(ak, 0, reg, tmp); */
    snd_akm4xxx_set(ak, 0, reg, tmp); /* value is written in reset(0) */
    snd_akm4xxx_reset(ak, 0);
}

/*
 * I2C access to the PT2258 volume controller on GPIO 6/7 (Revolution 5.1)
 */

unsafe extern "C" fn revo_i2c_start(bus: *mut snd_i2c_bus) {
    let ice = (*bus).private_data as *mut snd_ice1712;
    snd_ice1712_save_gpio_status(ice);
}

unsafe extern "C" fn revo_i2c_stop(bus: *mut snd_i2c_bus) {
    let ice = (*bus).private_data as *mut snd_ice1712;
    snd_ice1712_restore_gpio_status(ice);
}

unsafe extern "C" fn revo_i2c_direction(bus: *mut snd_i2c_bus, clock: c_int, data: c_int) {
    let ice = (*bus).private_data as *mut snd_ice1712;
    let mask: c_uint;
    let mut val: c_uint;

    val = 0;
    if clock != 0 {
        val |= VT1724_REVO_I2C_CLOCK; /* write SCL */
    }
    if data != 0 {
        val |= VT1724_REVO_I2C_DATA; /* write SDA */
    }
    mask = VT1724_REVO_I2C_CLOCK | VT1724_REVO_I2C_DATA;
    (*ice).gpio.direction &= !mask;
    (*ice).gpio.direction |= val;
    snd_ice1712_gpio_set_dir(ice, (*ice).gpio.direction);
    snd_ice1712_gpio_set_mask(ice, !mask);
}

unsafe extern "C" fn revo_i2c_setlines(bus: *mut snd_i2c_bus, clk: c_int, data: c_int) {
    let ice = (*bus).private_data as *mut snd_ice1712;
    let mut val: c_uint = 0;

    if clk != 0 {
        val |= VT1724_REVO_I2C_CLOCK;
    }
    if data != 0 {
        val |= VT1724_REVO_I2C_DATA;
    }
    snd_ice1712_gpio_write_bits(
        ice,
        VT1724_REVO_I2C_DATA | VT1724_REVO_I2C_CLOCK,
        val,
    );
    udelay(5);
}

unsafe extern "C" fn revo_i2c_getdata(bus: *mut snd_i2c_bus, ack: c_int) -> c_int {
    let ice = (*bus).private_data as *mut snd_ice1712;
    let bit: c_int;

    if ack != 0 {
        udelay(5);
    }
    bit = if snd_ice1712_gpio_read_bits(ice, VT1724_REVO_I2C_DATA) != 0 {
        1
    } else {
        0
    };
    bit
}

static mut revo51_bit_ops: snd_i2c_bit_ops = snd_i2c_bit_ops {
    start: Some(revo_i2c_start),
    stop: Some(revo_i2c_stop),
    direction: Some(revo_i2c_direction),
    setlines: Some(revo_i2c_setlines),
    getdata: Some(revo_i2c_getdata),
};

unsafe extern "C" fn revo51_i2c_init(
    ice: *mut snd_ice1712,
    pt: *mut snd_pt2258,
) -> c_int {
    let spec: *mut revo51_spec;
    let mut err: c_int;

    spec = kzalloc_obj::<revo51_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut c_void;

    /* create the I2C bus */
    err = snd_i2c_bus_create((*ice).card, b"ICE1724 GPIO6\0".as_ptr().cast(), null(), &mut (*ice).i2c);
    if err < 0 {
        return err;
    }

    (*(*ice).i2c).private_data = ice as *mut c_void;
    (*(*ice).i2c).hw_ops.bit = &mut revo51_bit_ops;

    /* create the I2C device */
    err = snd_i2c_device_create((*ice).i2c, b"PT2258\0".as_ptr().cast(), 0x40, &mut (*spec).dev);
    if err < 0 {
        return err;
    }

    (*pt).card = (*ice).card;
    (*pt).i2c_bus = (*ice).i2c;
    (*pt).i2c_dev = (*spec).dev;
    (*spec).pt2258 = pt;

    snd_pt2258_reset(pt);

    0
}

/*
 * initialize the chips on M-Audio Revolution cards
 */

macro_rules! AK_DAC {
    ($xname:expr, $xch:expr) => {
        snd_akm4xxx_dac_channel {
            name: $xname.as_ptr().cast(),
            num_channels: $xch,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static revo71_front: [snd_akm4xxx_dac_channel; 1] = [snd_akm4xxx_dac_channel {
    name: b"PCM Playback Volume\0".as_ptr().cast(),
    num_channels: 2,
    /* front channels DAC supports muting */
    switch_name: b"PCM Playback Switch\0".as_ptr().cast(),
    ..unsafe { core::mem::zeroed() }
}];

static revo71_surround: [snd_akm4xxx_dac_channel; 4] = [
    AK_DAC!(b"PCM Center Playback Volume\0", 1),
    AK_DAC!(b"PCM LFE Playback Volume\0", 1),
    AK_DAC!(b"PCM Side Playback Volume\0", 2),
    AK_DAC!(b"PCM Rear Playback Volume\0", 2),
];

static revo51_dac: [snd_akm4xxx_dac_channel; 5] = [
    AK_DAC!(b"PCM Playback Volume\0", 2),
    AK_DAC!(b"PCM Center Playback Volume\0", 1),
    AK_DAC!(b"PCM LFE Playback Volume\0", 1),
    AK_DAC!(b"PCM Rear Playback Volume\0", 2),
    AK_DAC!(b"PCM Headphone Volume\0", 2),
];

static revo51_adc_input_names: [*const i8; 4] = [
    b"Mic\0".as_ptr().cast(),
    b"Line\0".as_ptr().cast(),
    b"CD\0".as_ptr().cast(),
    null(),
];

static revo51_adc: [snd_akm4xxx_adc_channel; 1] = [snd_akm4xxx_adc_channel {
    name: b"PCM Capture Volume\0".as_ptr().cast(),
    switch_name: b"PCM Capture Switch\0".as_ptr().cast(),
    num_channels: 2,
    input_names: revo51_adc_input_names.as_ptr(),
    ..unsafe { core::mem::zeroed() }
}];

static akm_revo_front: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4381,
    num_dacs: 2,
    ops: snd_akm4xxx_ops {
        set_rate_val: Some(revo_set_rate_val),
        ..unsafe { core::mem::zeroed() }
    },
    dac_info: revo71_front.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

static akm_revo_front_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 1,
    cif: 0,
    data_mask: VT1724_REVO_CDOUT,
    clk_mask: VT1724_REVO_CCLK,
    cs_mask: VT1724_REVO_CS0 | VT1724_REVO_CS1 | VT1724_REVO_CS2,
    cs_addr: VT1724_REVO_CS0 | VT1724_REVO_CS2,
    cs_none: VT1724_REVO_CS0 | VT1724_REVO_CS1 | VT1724_REVO_CS2,
    add_flags: VT1724_REVO_CCLK, /* high at init */
    mask_flags: 0,
};

static akm_revo_surround: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4355,
    idx_offset: 1,
    num_dacs: 6,
    ops: snd_akm4xxx_ops {
        set_rate_val: Some(revo_set_rate_val),
        ..unsafe { core::mem::zeroed() }
    },
    dac_info: revo71_surround.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

static akm_revo_surround_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 3,
    cif: 0,
    data_mask: VT1724_REVO_CDOUT,
    clk_mask: VT1724_REVO_CCLK,
    cs_mask: VT1724_REVO_CS0 | VT1724_REVO_CS1 | VT1724_REVO_CS2,
    cs_addr: VT1724_REVO_CS0 | VT1724_REVO_CS1,
    cs_none: VT1724_REVO_CS0 | VT1724_REVO_CS1 | VT1724_REVO_CS2,
    add_flags: VT1724_REVO_CCLK, /* high at init */
    mask_flags: 0,
};

static akm_revo51: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4358,
    num_dacs: 8,
    ops: snd_akm4xxx_ops {
        set_rate_val: Some(revo_set_rate_val),
        ..unsafe { core::mem::zeroed() }
    },
    dac_info: revo51_dac.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

static akm_revo51_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0,
    data_mask: VT1724_REVO_CDOUT,
    clk_mask: VT1724_REVO_CCLK,
    cs_mask: VT1724_REVO_CS0 | VT1724_REVO_CS1,
    cs_addr: VT1724_REVO_CS1,
    cs_none: VT1724_REVO_CS0 | VT1724_REVO_CS1,
    add_flags: VT1724_REVO_CCLK, /* high at init */
    mask_flags: 0,
};

static akm_revo51_adc: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK5365,
    num_adcs: 2,
    adc_info: revo51_adc.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

static akm_revo51_adc_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0,
    data_mask: VT1724_REVO_CDOUT,
    clk_mask: VT1724_REVO_CCLK,
    cs_mask: VT1724_REVO_CS0 | VT1724_REVO_CS1,
    cs_addr: VT1724_REVO_CS0,
    cs_none: VT1724_REVO_CS0 | VT1724_REVO_CS1,
    add_flags: VT1724_REVO_CCLK, /* high at init */
    mask_flags: 0,
};

static mut ptc_revo51_volume: snd_pt2258 = unsafe { core::mem::zeroed() };

/* AK4358 for AP192 DAC, AK5385A for ADC */
unsafe extern "C" fn ap192_set_rate_val(ak: *mut snd_akm4xxx, rate: c_uint) {
    let ice = (*ak).private_data[0] as *mut snd_ice1712;
    let dfs: c_int;

    revo_set_rate_val(ak, rate);

    /* reset CKS */
    snd_ice1712_gpio_write_bits(ice, 1 << 8, if rate > 96000 { 1 << 8 } else { 0 });
    /* reset DFS pins of AK5385A for ADC, too */
    if rate > 96000 {
        dfs = 2;
    } else if rate > 48000 {
        dfs = 1;
    } else {
        dfs = 0;
    }
    snd_ice1712_gpio_write_bits(ice, 3 << 9, dfs << 9);
    /* reset ADC */
    snd_ice1712_gpio_write_bits(ice, 1 << 11, 0);
    snd_ice1712_gpio_write_bits(ice, 1 << 11, 1 << 11);
}

static ap192_dac: [snd_akm4xxx_dac_channel; 1] = [AK_DAC!(b"PCM Playback Volume\0", 2)];

static akm_ap192: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4358,
    num_dacs: 2,
    ops: snd_akm4xxx_ops {
        set_rate_val: Some(ap192_set_rate_val),
        ..unsafe { core::mem::zeroed() }
    },
    dac_info: ap192_dac.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

static akm_ap192_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0,
    data_mask: VT1724_REVO_CDOUT,
    clk_mask: VT1724_REVO_CCLK,
    cs_mask: VT1724_REVO_CS0 | VT1724_REVO_CS3,
    cs_addr: VT1724_REVO_CS3,
    cs_none: VT1724_REVO_CS0 | VT1724_REVO_CS3,
    add_flags: VT1724_REVO_CCLK, /* high at init */
    mask_flags: 0,
};

/* AK4114 support on Audiophile 192 */
/* CDTO (pin 32) -- GPIO2 pin 52
 * CDTI (pin 33) -- GPIO3 pin 53 (shared with AK4358)
 * CCLK (pin 34) -- GPIO1 pin 51 (shared with AK4358)
 * CSN  (pin 35) -- GPIO7 pin 59
 */
const AK4114_ADDR: c_uint = 0x00;

unsafe extern "C" fn write_data(
    ice: *mut snd_ice1712,
    mut gpio: c_uint,
    data: c_uint,
    mut idx: c_int,
) {
    while idx >= 0 {
        /* drop clock */
        gpio &= !VT1724_REVO_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        /* set data */
        if data & (1 << idx) != 0 {
            gpio |= VT1724_REVO_CDOUT;
        } else {
            gpio &= !VT1724_REVO_CDOUT;
        }
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        /* raise clock */
        gpio |= VT1724_REVO_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        idx -= 1;
    }
}

unsafe extern "C" fn read_data(
    ice: *mut snd_ice1712,
    mut gpio: c_uint,
    mut idx: c_int,
) -> c_uchar {
    let mut data: c_uchar = 0;

    while idx >= 0 {
        /* drop clock */
        gpio &= !VT1724_REVO_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        /* read data */
        if snd_ice1712_gpio_read(ice) & VT1724_REVO_CDIN != 0 {
            data |= (1 << idx) as c_uchar;
        }
        udelay(1);
        /* raise clock */
        gpio |= VT1724_REVO_CCLK;
        snd_ice1712_gpio_write(ice, gpio);
        udelay(1);
        idx -= 1;
    }
    data
}

unsafe extern "C" fn ap192_4wire_start(ice: *mut snd_ice1712) -> c_uint {
    let mut tmp: c_uint;

    snd_ice1712_save_gpio_status(ice);
    tmp = snd_ice1712_gpio_read(ice);
    tmp |= VT1724_REVO_CCLK; /* high at init */
    tmp |= VT1724_REVO_CS0;
    tmp &= !VT1724_REVO_CS3;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    tmp
}

unsafe extern "C" fn ap192_4wire_finish(ice: *mut snd_ice1712, mut tmp: c_uint) {
    tmp |= VT1724_REVO_CS3;
    tmp |= VT1724_REVO_CS0;
    snd_ice1712_gpio_write(ice, tmp);
    udelay(1);
    snd_ice1712_restore_gpio_status(ice);
}

unsafe extern "C" fn ap192_ak4114_write(
    private_data: *mut c_void,
    addr: c_uchar,
    data: c_uchar,
) {
    let ice = private_data as *mut snd_ice1712;
    let tmp: c_uint;
    let mut addrdata: c_uint;

    tmp = ap192_4wire_start(ice);
    addrdata = (AK4114_ADDR << 6) | 0x20 | ((addr as c_uint) & 0x1f);
    addrdata = (addrdata << 8) | data as c_uint;
    write_data(ice, tmp, addrdata, 15);
    ap192_4wire_finish(ice, tmp);
}

unsafe extern "C" fn ap192_ak4114_read(
    private_data: *mut c_void,
    addr: c_uchar,
) -> c_uchar {
    let ice = private_data as *mut snd_ice1712;
    let tmp: c_uint;
    let data: c_uchar;

    tmp = ap192_4wire_start(ice);
    write_data(ice, tmp, (AK4114_ADDR << 6) | ((addr as c_uint) & 0x1f), 7);
    data = read_data(ice, tmp, 7);
    ap192_4wire_finish(ice, tmp);
    data
}

unsafe extern "C" fn ap192_ak4114_init(ice: *mut snd_ice1712) -> c_int {
    static ak4114_init_vals: [c_uchar; 6] = [
        AK4114_RST | AK4114_PWN | AK4114_OCKS0,
        AK4114_DIF_I24I2S,
        AK4114_TX1E,
        AK4114_EFH_1024 | AK4114_DIT | AK4114_IPS(0),
        0,
        0,
    ];
    static ak4114_init_txcsb: [c_uchar; 5] = [0x41, 0x02, 0x2c, 0x00, 0x00];
    let mut err: c_int;

    let spec: *mut revo51_spec;
    spec = kzalloc_obj::<revo51_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut c_void;

    err = snd_ak4114_create(
        (*ice).card,
        Some(ap192_ak4114_read),
        Some(ap192_ak4114_write),
        ak4114_init_vals.as_ptr(),
        ak4114_init_txcsb.as_ptr(),
        ice as *mut c_void,
        &mut (*spec).ak4114,
    );
    if err < 0 {
        return err;
    }
    /* AK4114 in Revo cannot detect external rate correctly.
     * No reason to stop capture stream due to incorrect checks */
    (*(*spec).ak4114).check_flags = AK4114_CHECK_NO_RATE;

    0
}

unsafe extern "C" fn revo_init(ice: *mut snd_ice1712) -> c_int {
    let mut ak: *mut snd_akm4xxx;
    let mut err: c_int;

    /* determine I2C, DACs and ADCs */
    match (*ice).eeprom.subvendor {
        VT1724_SUBDEVICE_REVOLUTION71 => {
            (*ice).num_total_dacs = 8;
            (*ice).num_total_adcs = 2;
            (*ice).gpio.i2s_mclk_changed = Some(revo_i2s_mclk_changed);
        }
        VT1724_SUBDEVICE_REVOLUTION51 => {
            (*ice).num_total_dacs = 8;
            (*ice).num_total_adcs = 2;
        }
        VT1724_SUBDEVICE_AUDIOPHILE192 => {
            (*ice).num_total_dacs = 2;
            (*ice).num_total_adcs = 2;
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }

    /* second stage of initialization, analog parts and others */
    ak = kzalloc_objs::<snd_akm4xxx>(2);
    (*ice).akm = ak;
    if ak.is_null() {
        return -ENOMEM;
    }

    match (*ice).eeprom.subvendor {
        VT1724_SUBDEVICE_REVOLUTION71 => {
            (*ice).akm_codecs = 2;
            err = snd_ice1712_akm4xxx_init(
                ak,
                &akm_revo_front,
                &akm_revo_front_priv,
                ice,
            );
            if err < 0 {
                return err;
            }
            err = snd_ice1712_akm4xxx_init(
                ak.add(1),
                &akm_revo_surround,
                &akm_revo_surround_priv,
                ice,
            );
            if err < 0 {
                return err;
            }
            /* unmute all codecs */
            snd_ice1712_gpio_write_bits(ice, VT1724_REVO_MUTE, VT1724_REVO_MUTE);
        }
        VT1724_SUBDEVICE_REVOLUTION51 => {
            (*ice).akm_codecs = 2;
            err = snd_ice1712_akm4xxx_init(ak, &akm_revo51, &akm_revo51_priv, ice);
            if err < 0 {
                return err;
            }
            err = snd_ice1712_akm4xxx_init(
                ak.add(1),
                &akm_revo51_adc,
                &akm_revo51_adc_priv,
                ice,
            );
            if err < 0 {
                return err;
            }
            err = revo51_i2c_init(ice, &mut ptc_revo51_volume);
            if err < 0 {
                return err;
            }
            /* unmute all codecs */
            snd_ice1712_gpio_write_bits(ice, VT1724_REVO_MUTE, VT1724_REVO_MUTE);
        }
        VT1724_SUBDEVICE_AUDIOPHILE192 => {
            (*ice).akm_codecs = 1;
            err = snd_ice1712_akm4xxx_init(ak, &akm_ap192, &akm_ap192_priv, ice);
            if err < 0 {
                return err;
            }
            err = ap192_ak4114_init(ice);
            if err < 0 {
                return err;
            }

            /* unmute all codecs */
            snd_ice1712_gpio_write_bits(ice, VT1724_REVO_MUTE, VT1724_REVO_MUTE);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn revo_add_controls(ice: *mut snd_ice1712) -> c_int {
    let mut spec = (*ice).spec as *mut revo51_spec;
    let mut err: c_int;

    match (*ice).eeprom.subvendor {
        VT1724_SUBDEVICE_REVOLUTION71 => {
            err = snd_ice1712_akm4xxx_build_controls(ice);
            if err < 0 {
                return err;
            }
        }
        VT1724_SUBDEVICE_REVOLUTION51 => {
            err = snd_ice1712_akm4xxx_build_controls(ice);
            if err < 0 {
                return err;
            }
            spec = (*ice).spec as *mut revo51_spec;
            err = snd_pt2258_build_controls((*spec).pt2258);
            if err < 0 {
                return err;
            }
        }
        VT1724_SUBDEVICE_AUDIOPHILE192 => {
            err = snd_ice1712_akm4xxx_build_controls(ice);
            if err < 0 {
                return err;
            }
            /* only capture SPDIF over AK4114 */
            err = snd_ak4114_build(
                (*spec).ak4114,
                null_mut(),
                (*(*ice).pcm).streams[SNDRV_PCM_STREAM_CAPTURE].substream,
            );
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }
    0
}

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_revo_cards: [snd_ice1712_card_info; 4] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_REVOLUTION71,
        name: b"M Audio Revolution-7.1\0".as_ptr().cast(),
        model: b"revo71\0".as_ptr().cast(),
        chip_init: Some(revo_init),
        build_controls: Some(revo_add_controls),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_REVOLUTION51,
        name: b"M Audio Revolution-5.1\0".as_ptr().cast(),
        model: b"revo51\0".as_ptr().cast(),
        chip_init: Some(revo_init),
        build_controls: Some(revo_add_controls),
        ..unsafe { core::mem::zeroed() }
    },
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_AUDIOPHILE192,
        name: b"M Audio Audiophile192\0".as_ptr().cast(),
        model: b"ap192\0".as_ptr().cast(),
        chip_init: Some(revo_init),
        build_controls: Some(revo_add_controls),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
