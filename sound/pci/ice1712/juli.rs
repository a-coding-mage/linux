// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for ESI Juli@ cards
 *
 *	Copyright (c) 2004 Jaroslav Kysela <perex@perex.cz>
 *	              2008 Pavel Hofman <dustin@seznam.cz>
 */

/* Dependencies from the original C includes:
 * linux/delay.h, linux/interrupt.h, linux/init.h, linux/slab.h,
 * linux/string.h, sound/core.h, sound/tlv.h, ice1712.h, envy24ht.h, juli.h.
 */

#[repr(C)]
pub struct juli_spec {
    pub ak4114: *mut ak4114,
    pub analog: c_uint,
}

/*
 * chip addresses on I2C bus
 */
pub const AK4114_ADDR: c_uint = 0x20; /* S/PDIF receiver */
pub const AK4358_ADDR: c_uint = 0x22; /* DAC */

/*
 * Juli does not use the standard ICE1724 clock scheme. Juli's ice1724 chip is
 * supplied by external clock provided by Xilinx array and MK73-1 PLL frequency
 * multiplier. Actual frequency is set by ice1724 GPIOs hooked to the Xilinx.
 *
 * The clock circuitry is supplied by the two ice1724 crystals. This
 * arrangement allows to generate independent clock signal for AK4114's input
 * rate detection circuit. As a result, Juli, unlike most other
 * ice1724+ak4114-based cards, detects spdif input rate correctly.
 * This fact is applied in the driver, allowing to modify PCM stream rate
 * parameter according to the actual input rate.
 *
 * Juli uses the remaining three stereo-channels of its DAC to optionally
 * monitor analog input, digital input, and digital output. The corresponding
 * I2S signals are routed by Xilinx, controlled by GPIOs.
 *
 * The master mute is implemented using output muting transistors (GPIO) in
 * combination with smuting the DAC.
 *
 * The card itself has no HW master volume control, implemented using the
 * vmaster control.
 *
 * TODO:
 * researching and fixing the input monitors
 */

/*
 * GPIO pins
 */
pub const GPIO_FREQ_MASK: c_uint = 3 << 0;
pub const GPIO_FREQ_32KHZ: c_uint = 0 << 0;
pub const GPIO_FREQ_44KHZ: c_uint = 1 << 0;
pub const GPIO_FREQ_48KHZ: c_uint = 2 << 0;
pub const GPIO_MULTI_MASK: c_uint = 3 << 2;
pub const GPIO_MULTI_4X: c_uint = 0 << 2;
pub const GPIO_MULTI_2X: c_uint = 1 << 2;
pub const GPIO_MULTI_1X: c_uint = 2 << 2; /* also external */
pub const GPIO_MULTI_HALF: c_uint = 3 << 2;
pub const GPIO_INTERNAL_CLOCK: c_uint = 1 << 4; /* 0 = external, 1 = internal */
pub const GPIO_CLOCK_MASK: c_uint = 1 << 4;
pub const GPIO_ANALOG_PRESENT: c_uint = 1 << 5; /* RO only: 0 = present */
pub const GPIO_RXMCLK_SEL: c_uint = 1 << 7; /* must be 0 */
pub const GPIO_AK5385A_CKS0: c_uint = 1 << 8;
pub const GPIO_AK5385A_DFS1: c_uint = 1 << 9;
pub const GPIO_AK5385A_DFS0: c_uint = 1 << 10;
pub const GPIO_DIGOUT_MONITOR: c_uint = 1 << 11; /* 1 = active */
pub const GPIO_DIGIN_MONITOR: c_uint = 1 << 12; /* 1 = active */
pub const GPIO_ANAIN_MONITOR: c_uint = 1 << 13; /* 1 = active */
pub const GPIO_AK5385A_CKS1: c_uint = 1 << 14; /* must be 0 */
pub const GPIO_MUTE_CONTROL: c_uint = 1 << 15; /* output mute, 1 = muted */

pub const GPIO_RATE_MASK: c_uint = GPIO_FREQ_MASK | GPIO_MULTI_MASK | GPIO_CLOCK_MASK;
pub const GPIO_AK5385A_MASK: c_uint =
    GPIO_AK5385A_CKS0 | GPIO_AK5385A_DFS0 | GPIO_AK5385A_DFS1 | GPIO_AK5385A_CKS1;

pub const JULI_PCM_RATE: c_uint = SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_64000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

pub const GPIO_RATE_16000: c_uint = GPIO_FREQ_32KHZ | GPIO_MULTI_HALF | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_22050: c_uint = GPIO_FREQ_44KHZ | GPIO_MULTI_HALF | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_24000: c_uint = GPIO_FREQ_48KHZ | GPIO_MULTI_HALF | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_32000: c_uint = GPIO_FREQ_32KHZ | GPIO_MULTI_1X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_44100: c_uint = GPIO_FREQ_44KHZ | GPIO_MULTI_1X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_48000: c_uint = GPIO_FREQ_48KHZ | GPIO_MULTI_1X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_64000: c_uint = GPIO_FREQ_32KHZ | GPIO_MULTI_2X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_88200: c_uint = GPIO_FREQ_44KHZ | GPIO_MULTI_2X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_96000: c_uint = GPIO_FREQ_48KHZ | GPIO_MULTI_2X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_176400: c_uint = GPIO_FREQ_44KHZ | GPIO_MULTI_4X | GPIO_INTERNAL_CLOCK;
pub const GPIO_RATE_192000: c_uint = GPIO_FREQ_48KHZ | GPIO_MULTI_4X | GPIO_INTERNAL_CLOCK;

/*
 * Initial setup of the conversion array GPIO <-> rate
 */
static juli_rates: [c_uint; 11] = [
    16000, 22050, 24000, 32000, 44100, 48000, 64000, 88200, 96000, 176400, 192000,
];

static gpio_vals: [c_uint; 11] = [
    GPIO_RATE_16000,
    GPIO_RATE_22050,
    GPIO_RATE_24000,
    GPIO_RATE_32000,
    GPIO_RATE_44100,
    GPIO_RATE_48000,
    GPIO_RATE_64000,
    GPIO_RATE_88200,
    GPIO_RATE_96000,
    GPIO_RATE_176400,
    GPIO_RATE_192000,
];

static juli_rates_info: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: juli_rates.len() as c_uint,
    list: juli_rates.as_ptr(),
    mask: 0,
};

unsafe fn get_gpio_val(rate: c_int) -> c_int {
    let mut i: usize = 0;
    while i < juli_rates.len() {
        if juli_rates[i] as c_int == rate {
            return gpio_vals[i] as c_int;
        }
        i += 1;
    }
    0
}

unsafe fn juli_ak4114_write(private_data: *mut c_void, reg: c_uchar, val: c_uchar) {
    snd_vt1724_write_i2c(private_data as *mut snd_ice1712, AK4114_ADDR as c_int, reg, val);
}

unsafe fn juli_ak4114_read(private_data: *mut c_void, reg: c_uchar) -> c_uchar {
    snd_vt1724_read_i2c(private_data as *mut snd_ice1712, AK4114_ADDR as c_int, reg)
}

/*
 * If SPDIF capture and slaved to SPDIF-IN, setting runtime rate
 * to the external rate
 */
unsafe fn juli_spdif_in_open(ice: *mut snd_ice1712, substream: *mut snd_pcm_substream) {
    let spec: *mut juli_spec = (*ice).spec as *mut juli_spec;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let rate: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK || ((*ice).is_spdif_master.unwrap())(ice) == 0 {
        return;
    }
    rate = snd_ak4114_external_rate((*spec).ak4114);
    if rate as c_uint >= (*runtime).hw.rate_min && rate as c_uint <= (*runtime).hw.rate_max {
        (*runtime).hw.rate_min = rate as c_uint;
        (*runtime).hw.rate_max = rate as c_uint;
    }
}

/*
 * AK4358 section
 */

unsafe fn juli_akm_lock(_ak: *mut snd_akm4xxx, _chip: c_int) {}

unsafe fn juli_akm_unlock(_ak: *mut snd_akm4xxx, _chip: c_int) {}

unsafe fn juli_akm_write(ak: *mut snd_akm4xxx, chip: c_int, addr: c_uchar, data: c_uchar) {
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    if snd_BUG_ON(chip) != 0 {
        return;
    }
    snd_vt1724_write_i2c(ice, AK4358_ADDR as c_int, addr, data);
}

/*
 * change the rate of envy24HT, AK4358, AK5385
 */
unsafe fn juli_akm_set_rate_val(ak: *mut snd_akm4xxx, rate: c_uint) {
    let mut old: c_uchar;
    let mut tmp: c_uchar;
    let ak4358_dfs: c_uchar;
    let ak5385_pins: c_uint;
    let old_gpio: c_uint;
    let new_gpio: c_uint;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;
    let spec: *mut juli_spec = (*ice).spec as *mut juli_spec;

    if rate == 0 {
        /* no hint - S/PDIF input is master or the new spdif
           input rate undetected, simply return */
        return;
    }

    /* adjust DFS on codecs */
    if rate > 96000 {
        ak4358_dfs = 2;
        ak5385_pins = GPIO_AK5385A_DFS1 | GPIO_AK5385A_CKS0;
    } else if rate > 48000 {
        ak4358_dfs = 1;
        ak5385_pins = GPIO_AK5385A_DFS0;
    } else {
        ak4358_dfs = 0;
        ak5385_pins = 0;
    }
    /* AK5385 first, since it requires cold reset affecting both codecs */
    old_gpio = ((*ice).gpio.get_data.unwrap())(ice);
    new_gpio = (old_gpio & !GPIO_AK5385A_MASK) | ak5385_pins;
    /* dev_dbg(ice->card->dev, "JULI - ak5385 set_rate_val: new gpio 0x%x\n",
        new_gpio); */
    ((*ice).gpio.set_data.unwrap())(ice, new_gpio);

    /* cold reset */
    old = inb(ICEMT1724(ice, AC97_CMD)) as c_uchar;
    outb((old as c_int | VT1724_AC97_COLD) as c_uchar, ICEMT1724(ice, AC97_CMD));
    udelay(1);
    outb((old as c_int & !VT1724_AC97_COLD) as c_uchar, ICEMT1724(ice, AC97_CMD));

    /* AK4358 */
    /* set new value, reset DFS */
    tmp = snd_akm4xxx_get(ak, 0, 2);
    snd_akm4xxx_reset(ak, 1);
    tmp = snd_akm4xxx_get(ak, 0, 2);
    tmp &= !(0x03 << 4);
    tmp |= ak4358_dfs << 4;
    snd_akm4xxx_set(ak, 0, 2, tmp);
    snd_akm4xxx_reset(ak, 0);

    /* reinit ak4114 */
    snd_ak4114_reinit((*spec).ak4114);
}

pub const PCM_VOLUME: &[u8] = b"PCM Playback Volume\0";
pub const MONITOR_AN_IN_VOLUME: &[u8] = b"Monitor Analog In Volume\0";
pub const MONITOR_DIG_IN_VOLUME: &[u8] = b"Monitor Digital In Volume\0";
pub const MONITOR_DIG_OUT_VOLUME: &[u8] = b"Monitor Digital Out Volume\0";

static juli_dac: [snd_akm4xxx_dac_channel; 4] = [
    snd_akm4xxx_dac_channel {
        name: PCM_VOLUME.as_ptr() as *const c_char,
        num_channels: 2,
    },
    snd_akm4xxx_dac_channel {
        name: MONITOR_AN_IN_VOLUME.as_ptr() as *const c_char,
        num_channels: 2,
    },
    snd_akm4xxx_dac_channel {
        name: MONITOR_DIG_OUT_VOLUME.as_ptr() as *const c_char,
        num_channels: 2,
    },
    snd_akm4xxx_dac_channel {
        name: MONITOR_DIG_IN_VOLUME.as_ptr() as *const c_char,
        num_channels: 2,
    },
];

static akm_juli_dac: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4358,
    num_dacs: 8, /* DAC1 - analog out
                  * DAC2 - analog in monitor
                  * DAC3 - digital out monitor
                  * DAC4 - digital in monitor
                  */
    ops: snd_akm4xxx_ops {
        lock: Some(juli_akm_lock),
        unlock: Some(juli_akm_unlock),
        write: Some(juli_akm_write),
        set_rate_val: Some(juli_akm_set_rate_val),
    },
    dac_info: juli_dac.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

/* #define juli_mute_info snd_ctl_boolean_mono_info */

unsafe fn juli_mute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let val: c_uint;
    val = ((*ice).gpio.get_data.unwrap())(ice) & (*kcontrol).private_value as c_uint;
    if (*kcontrol).private_value as c_uint == GPIO_MUTE_CONTROL {
        /* val 0 = signal on */
        (*ucontrol).value.integer.value[0] = if val != 0 { 0 } else { 1 };
    } else {
        /* val 1 = signal on */
        (*ucontrol).value.integer.value[0] = if val != 0 { 1 } else { 0 };
    }
    0
}

unsafe fn juli_mute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol) as *mut snd_ice1712;
    let old_gpio: c_uint;
    let new_gpio: c_uint;
    old_gpio = ((*ice).gpio.get_data.unwrap())(ice);
    if (*ucontrol).value.integer.value[0] != 0 {
        /* unmute */
        if (*kcontrol).private_value as c_uint == GPIO_MUTE_CONTROL {
            /* 0 = signal on */
            new_gpio = old_gpio & !GPIO_MUTE_CONTROL;
            /* un-smuting DAC */
            snd_akm4xxx_write((*ice).akm, 0, 0x01, 0x01);
        } else {
            /* 1 = signal on */
            new_gpio = old_gpio | (*kcontrol).private_value as c_uint;
        }
    } else {
        /* mute */
        if (*kcontrol).private_value as c_uint == GPIO_MUTE_CONTROL {
            /* 1 = signal off */
            new_gpio = old_gpio | GPIO_MUTE_CONTROL;
            /* smuting DAC */
            snd_akm4xxx_write((*ice).akm, 0, 0x01, 0x03);
        } else {
            /* 0 = signal off */
            new_gpio = old_gpio & !((*kcontrol).private_value as c_uint);
        }
    }
    /* dev_dbg(ice->card->dev,
        "JULI - mute/unmute: control_value: 0x%x, old_gpio: 0x%x, "
        "new_gpio 0x%x\n",
        (unsigned int)ucontrol->value.integer.value[0], old_gpio,
        new_gpio); */
    if old_gpio != new_gpio {
        ((*ice).gpio.set_data.unwrap())(ice, new_gpio);
        return 1;
    }
    /* no change */
    0
}

static juli_mute_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Master Playback Switch\0".as_ptr() as *const c_char,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(juli_mute_get),
        put: Some(juli_mute_put),
        private_value: GPIO_MUTE_CONTROL as c_ulong,
        ..unsafe { core::mem::zeroed() }
    },
    /* Although the following functionality respects the succint NDA'd
     * documentation from the card manufacturer, and the same way of
     * operation is coded in OSS Juli driver, only Digital Out monitor
     * seems to work. Surprisingly, Analog input monitor outputs Digital
     * output data. The two are independent, as enabling both doubles
     * volume of the monitor sound.
     *
     * Checking traces on the board suggests the functionality described
     * by the manufacturer is correct - I2S from ADC and AK4114
     * go to ICE as well as to Xilinx, I2S inputs of DAC2,3,4 (the monitor
     * inputs) are fed from Xilinx.
     *
     * I even checked traces on board and coded a support in driver for
     * an alternative possibility - the unused I2S ICE output channels
     * switched to HW-IN/SPDIF-IN and providing the monitoring signal to
     * the DAC - to no avail. The I2S outputs seem to be unconnected.
     *
     * The windows driver supports the monitoring correctly.
     */
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Monitor Analog In Switch\0".as_ptr() as *const c_char,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(juli_mute_get),
        put: Some(juli_mute_put),
        private_value: GPIO_ANAIN_MONITOR as c_ulong,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Monitor Digital Out Switch\0".as_ptr() as *const c_char,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(juli_mute_get),
        put: Some(juli_mute_put),
        private_value: GPIO_DIGOUT_MONITOR as c_ulong,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"Monitor Digital In Switch\0".as_ptr() as *const c_char,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(juli_mute_get),
        put: Some(juli_mute_put),
        private_value: GPIO_DIGIN_MONITOR as c_ulong,
        ..unsafe { core::mem::zeroed() }
    },
];

static follower_vols: [*const c_char; 5] = [
    PCM_VOLUME.as_ptr() as *const c_char,
    MONITOR_AN_IN_VOLUME.as_ptr() as *const c_char,
    MONITOR_DIG_IN_VOLUME.as_ptr() as *const c_char,
    MONITOR_DIG_OUT_VOLUME.as_ptr() as *const c_char,
    core::ptr::null(),
];

/* static DECLARE_TLV_DB_SCALE(juli_master_db_scale, -6350, 50, 1); */
static juli_master_db_scale: [c_uint; 4] = [TLV_DB_SCALE_ITEM(-6350, 50, 1)];

unsafe fn juli_add_controls(ice: *mut snd_ice1712) -> c_int {
    let spec: *mut juli_spec = (*ice).spec as *mut juli_spec;
    let mut err: c_int;
    let mut i: c_uint;
    let vmaster: *mut snd_kcontrol;

    err = snd_ice1712_akm4xxx_build_controls(ice);
    if err < 0 {
        return err;
    }

    i = 0;
    while (i as usize) < juli_mute_controls.len() {
        err = snd_ctl_add(
            (*ice).card,
            snd_ctl_new1(&juli_mute_controls[i as usize], ice as *mut c_void),
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }
    /* Create virtual master control */
    vmaster = snd_ctl_make_virtual_master(
        b"Master Playback Volume\0".as_ptr() as *const c_char,
        juli_master_db_scale.as_ptr(),
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

    /* only capture SPDIF over AK4114 */
    snd_ak4114_build(
        (*spec).ak4114,
        core::ptr::null_mut(),
        (*(*(*ice).pcm).streams.as_ptr().add(SNDRV_PCM_STREAM_CAPTURE as usize)).substream,
    )
}

/*
 * suspend/resume
 * */

/* CONFIG_PM_SLEEP */
unsafe fn juli_resume(ice: *mut snd_ice1712) -> c_int {
    let ak: *mut snd_akm4xxx = (*ice).akm;
    let spec: *mut juli_spec = (*ice).spec as *mut juli_spec;
    /* akm4358 un-reset, un-mute */
    snd_akm4xxx_reset(ak, 0);
    /* reinit ak4114 */
    snd_ak4114_resume((*spec).ak4114);
    0
}

/* CONFIG_PM_SLEEP */
unsafe fn juli_suspend(ice: *mut snd_ice1712) -> c_int {
    let ak: *mut snd_akm4xxx = (*ice).akm;
    let spec: *mut juli_spec = (*ice).spec as *mut juli_spec;
    /* akm4358 reset and soft-mute */
    snd_akm4xxx_reset(ak, 1);
    snd_ak4114_suspend((*spec).ak4114);
    0
}

/*
 * initialize the chip
 */

unsafe fn juli_is_spdif_master(ice: *mut snd_ice1712) -> c_int {
    if ((*ice).gpio.get_data.unwrap())(ice) & GPIO_INTERNAL_CLOCK != 0 {
        0
    } else {
        1
    }
}

unsafe fn juli_get_rate(ice: *mut snd_ice1712) -> c_uint {
    let mut i: c_int;
    let result: c_uchar;

    result = (((*ice).gpio.get_data.unwrap())(ice) & GPIO_RATE_MASK) as c_uchar;
    i = 0;
    while (i as usize) < gpio_vals.len() {
        if gpio_vals[i as usize] == result as c_uint {
            return juli_rates[i as usize];
        }
        i += 1;
    }
    0
}

/* setting new rate */
unsafe fn juli_set_rate(ice: *mut snd_ice1712, rate: c_uint) {
    let old: c_uint;
    let new: c_uint;
    let val: c_uchar;

    old = ((*ice).gpio.get_data.unwrap())(ice);
    new = (old & !GPIO_RATE_MASK) | get_gpio_val(rate as c_int) as c_uint;
    /* dev_dbg(ice->card->dev, "JULI - set_rate: old %x, new %x\n",
            old & GPIO_RATE_MASK,
            new & GPIO_RATE_MASK); */

    ((*ice).gpio.set_data.unwrap())(ice, new);
    /* switching to external clock - supplied by external circuits */
    val = inb(ICEMT1724(ice, RATE)) as c_uchar;
    outb((val as c_int | VT1724_SPDIF_MASTER) as c_uchar, ICEMT1724(ice, RATE));
}

unsafe fn juli_set_mclk(_ice: *mut snd_ice1712, _rate: c_uint) -> c_uchar {
    /* no change in master clock */
    0
}

/* setting clock to external - SPDIF */
unsafe fn juli_set_spdif_clock(ice: *mut snd_ice1712, _type: c_int) -> c_int {
    let old: c_uint;
    old = ((*ice).gpio.get_data.unwrap())(ice);
    /* external clock (= 0), multiply 1x, 48kHz */
    ((*ice).gpio.set_data.unwrap())(
        ice,
        (old & !GPIO_RATE_MASK) | GPIO_MULTI_1X | GPIO_FREQ_48KHZ,
    );
    0
}

/* Called when ak4114 detects change in the input SPDIF stream */
unsafe fn juli_ak4114_change(ak4114: *mut ak4114, _c0: c_uchar, c1: c_uchar) {
    let ice: *mut snd_ice1712 = (*ak4114).change_callback_private as *mut snd_ice1712;
    let rate: c_int;
    if ((*ice).is_spdif_master.unwrap())(ice) != 0 && c1 != 0 {
        /* only for SPDIF master mode, rate was changed */
        rate = snd_ak4114_external_rate(ak4114);
        /* dev_dbg(ice->card->dev, "ak4114 - input rate changed to %d\n",
                rate); */
        juli_akm_set_rate_val((*ice).akm, rate as c_uint);
    }
}

unsafe fn juli_init(ice: *mut snd_ice1712) -> c_int {
    static ak4114_init_vals: [c_uchar; 6] = [
        /* AK4117_REG_PWRDN */ AK4114_RST | AK4114_PWN | AK4114_OCKS0 | AK4114_OCKS1,
        /* AK4114_REQ_FORMAT */ AK4114_DIF_I24I2S,
        /* AK4114_REG_IO0 */ AK4114_TX1E,
        /* AK4114_REG_IO1 */ AK4114_EFH_1024 | AK4114_DIT | AK4114_IPS(1),
        /* AK4114_REG_INT0_MASK */ 0,
        /* AK4114_REG_INT1_MASK */ 0,
    ];
    static ak4114_init_txcsb: [c_uchar; 5] = [0x41, 0x02, 0x2c, 0x00, 0x00];
    let mut err: c_int;
    let spec: *mut juli_spec;
    let ak: *mut snd_akm4xxx;

    spec = kzalloc_obj::<juli_spec>();
    if spec.is_null() {
        return -ENOMEM;
    }
    (*ice).spec = spec as *mut c_void;

    err = snd_ak4114_create(
        (*ice).card,
        Some(juli_ak4114_read),
        Some(juli_ak4114_write),
        ak4114_init_vals.as_ptr(),
        ak4114_init_txcsb.as_ptr(),
        ice as *mut c_void,
        &mut (*spec).ak4114,
    );
    if err < 0 {
        return err;
    }
    /* callback for codecs rate setting */
    (*(*spec).ak4114).change_callback = Some(juli_ak4114_change);
    (*(*spec).ak4114).change_callback_private = ice as *mut c_void;
    /* AK4114 in Juli can detect external rate correctly */
    (*(*spec).ak4114).check_flags = 0;

    /* #if 0
     *
     * it seems that the analog doughter board detection does not work reliably, so
     * force the analog flag; it should be very rare (if ever) to come at Juli@
     * used without the analog daughter board
     *
     * spec->analog = (ice->gpio.get_data(ice) & GPIO_ANALOG_PRESENT) ? 0 : 1;
     * #else
     */
    (*spec).analog = 1;
    /* #endif */

    if (*spec).analog != 0 {
        dev_info((*(*ice).card).dev, b"juli@: analog I/O detected\n\0".as_ptr() as *const c_char);
        (*ice).num_total_dacs = 2;
        (*ice).num_total_adcs = 2;

        (*ice).akm = kzalloc_obj::<snd_akm4xxx>();
        ak = (*ice).akm;
        if ak.is_null() {
            return -ENOMEM;
        }
        (*ice).akm_codecs = 1;
        err = snd_ice1712_akm4xxx_init(ak, &akm_juli_dac, core::ptr::null_mut(), ice);
        if err < 0 {
            return err;
        }
    }

    /* juli is clocked by Xilinx array */
    (*ice).hw_rates = &juli_rates_info;
    (*ice).is_spdif_master = Some(juli_is_spdif_master);
    (*ice).get_rate = Some(juli_get_rate);
    (*ice).set_rate = Some(juli_set_rate);
    (*ice).set_mclk = Some(juli_set_mclk);
    (*ice).set_spdif_clock = Some(juli_set_spdif_clock);

    (*ice).spdif.ops.open = Some(juli_spdif_in_open);

    /* CONFIG_PM_SLEEP */
    (*ice).pm_resume = Some(juli_resume);
    (*ice).pm_suspend = Some(juli_suspend);
    (*ice).pm_suspend_enabled = 1;

    0
}

/*
 * Juli@ boards don't provide the EEPROM data except for the vendor IDs.
 * hence the driver needs to sets up it properly.
 */

static juli_eeprom: [c_uchar; ICE_EEP2_GPIO_STATE2 as usize + 1] = {
    let mut data = [0 as c_uchar; ICE_EEP2_GPIO_STATE2 as usize + 1];
    data[ICE_EEP2_SYSCONF as usize] = 0x2b; /* clock 512, mpu401, 1xADC, 1xDACs,
                                             * SPDIF in */
    data[ICE_EEP2_ACLINK as usize] = 0x80; /* I2S */
    data[ICE_EEP2_I2S as usize] = 0xf8; /* vol, 96k, 24bit, 192k */
    data[ICE_EEP2_SPDIF as usize] = 0xc3; /* out-en, out-int, spdif-in */
    data[ICE_EEP2_GPIO_DIR as usize] = 0x9f; /* 5, 6:inputs; 7, 4-0 outputs*/
    data[ICE_EEP2_GPIO_DIR1 as usize] = 0xff;
    data[ICE_EEP2_GPIO_DIR2 as usize] = 0x7f;
    data[ICE_EEP2_GPIO_MASK as usize] = 0x60; /* 5, 6: locked; 7, 4-0 writable */
    data[ICE_EEP2_GPIO_MASK1 as usize] = 0x00; /* 0-7 writable */
    data[ICE_EEP2_GPIO_MASK2 as usize] = 0x7f;
    data[ICE_EEP2_GPIO_STATE as usize] =
        (GPIO_FREQ_48KHZ | GPIO_MULTI_1X | GPIO_INTERNAL_CLOCK) as c_uchar; /* internal clock, multiple 1x, 48kHz*/
    data[ICE_EEP2_GPIO_STATE1 as usize] = 0x00; /* unmuted */
    data[ICE_EEP2_GPIO_STATE2 as usize] = 0x00;
    data
};

/* entry point */
#[no_mangle]
pub static mut snd_vt1724_juli_cards: [snd_ice1712_card_info; 2] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_JULI,
        name: b"ESI Juli@\0".as_ptr() as *const c_char,
        model: b"juli\0".as_ptr() as *const c_char,
        chip_init: Some(juli_init),
        build_controls: Some(juli_add_controls),
        eeprom_size: core::mem::size_of_val(&juli_eeprom) as c_uint,
        eeprom_data: juli_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
