// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble ICE1712 (Envy24)
 *
 *   Lowlevel functions for M-Audio Delta 1010, 1010E, 44, 66, 66E, Dio2496,
 *			    Audiophile, Digigram VX442
 *
 *	Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
 */

/* Dependencies from linux/delay.h, linux/interrupt.h, linux/init.h,
 * linux/slab.h, linux/mutex.h, sound/core.h, sound/cs8427.h,
 * sound/asoundef.h, ice1712.h, delta.h, and sound/cs8403.h are expected
 * to be supplied by the translated surrounding repository.
 */

/*
 * CS8427 via SPI mode (for Audiophile), emulated I2C
 */

/* send 8 bits */
unsafe fn ap_cs8427_write_byte(ice: *mut snd_ice1712, data: u8, mut tmp: u8) {
    let mut idx: i32 = 7;

    while idx >= 0 {
        tmp &= !(ICE1712_DELTA_AP_DOUT | ICE1712_DELTA_AP_CCLK) as u8;
        if (data as i32 & (1 << idx)) != 0 {
            tmp |= ICE1712_DELTA_AP_DOUT as u8;
        }
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
        udelay(5);
        tmp |= ICE1712_DELTA_AP_CCLK as u8;
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
        udelay(5);
        idx -= 1;
    }
}

/* read 8 bits */
unsafe fn ap_cs8427_read_byte(ice: *mut snd_ice1712, mut tmp: u8) -> u8 {
    let mut data: u8 = 0;
    let mut idx: i32 = 7;

    while idx >= 0 {
        tmp &= !(ICE1712_DELTA_AP_CCLK as u8);
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
        udelay(5);
        if (snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA) & ICE1712_DELTA_AP_DIN as u8) != 0 {
            data |= (1 << idx) as u8;
        }
        tmp |= ICE1712_DELTA_AP_CCLK as u8;
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
        udelay(5);
        idx -= 1;
    }
    data
}

/* assert chip select */
unsafe fn ap_cs8427_codec_select(ice: *mut snd_ice1712) -> u8 {
    let mut tmp: u8;
    tmp = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA);
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010E | ICE1712_SUBDEVICE_DELTA1010LT => {
            tmp &= !(ICE1712_DELTA_1010LT_CS as u8);
            tmp |= (ICE1712_DELTA_1010LT_CCLK | ICE1712_DELTA_1010LT_CS_CS8427) as u8;
        }
        ICE1712_SUBDEVICE_AUDIOPHILE | ICE1712_SUBDEVICE_DELTA410 => {
            tmp |= (ICE1712_DELTA_AP_CCLK | ICE1712_DELTA_AP_CS_CODEC) as u8;
            tmp &= !(ICE1712_DELTA_AP_CS_DIGITAL as u8);
        }
        ICE1712_SUBDEVICE_DELTA66E => {
            tmp |= (ICE1712_DELTA_66E_CCLK
                | ICE1712_DELTA_66E_CS_CHIP_A
                | ICE1712_DELTA_66E_CS_CHIP_B) as u8;
            tmp &= !(ICE1712_DELTA_66E_CS_CS8427 as u8);
        }
        ICE1712_SUBDEVICE_VX442 => {
            tmp |= (ICE1712_VX442_CCLK | ICE1712_VX442_CODEC_CHIP_A | ICE1712_VX442_CODEC_CHIP_B)
                as u8;
            tmp &= !(ICE1712_VX442_CS_DIGITAL as u8);
        }
        _ => {}
    }
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
    udelay(5);
    tmp
}

/* deassert chip select */
unsafe fn ap_cs8427_codec_deassert(ice: *mut snd_ice1712, mut tmp: u8) {
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010E | ICE1712_SUBDEVICE_DELTA1010LT => {
            tmp &= !(ICE1712_DELTA_1010LT_CS as u8);
            tmp |= ICE1712_DELTA_1010LT_CS_NONE as u8;
        }
        ICE1712_SUBDEVICE_AUDIOPHILE | ICE1712_SUBDEVICE_DELTA410 => {
            tmp |= ICE1712_DELTA_AP_CS_DIGITAL as u8;
        }
        ICE1712_SUBDEVICE_DELTA66E => {
            tmp |= ICE1712_DELTA_66E_CS_CS8427 as u8;
        }
        ICE1712_SUBDEVICE_VX442 => {
            tmp |= ICE1712_VX442_CS_DIGITAL as u8;
        }
        _ => {}
    }
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
}

/* sequential write */
unsafe fn ap_cs8427_sendbytes(device: *mut snd_i2c_device, mut bytes: *mut u8, mut count: i32) -> i32 {
    let ice: *mut snd_ice1712 = (*(*device).bus).private_data as *mut snd_ice1712;
    let res: i32 = count;
    let tmp: u8;

    guard_mutex(&mut (*ice).gpio_mutex);
    tmp = ap_cs8427_codec_select(ice);
    ap_cs8427_write_byte(ice, (((*device).addr << 1) | 0) as u8, tmp); /* address + write mode */
    while count > 0 {
        ap_cs8427_write_byte(ice, *bytes, tmp);
        bytes = bytes.add(1);
        count -= 1;
    }
    ap_cs8427_codec_deassert(ice, tmp);
    res
}

/* sequential read */
unsafe fn ap_cs8427_readbytes(device: *mut snd_i2c_device, mut bytes: *mut u8, mut count: i32) -> i32 {
    let ice: *mut snd_ice1712 = (*(*device).bus).private_data as *mut snd_ice1712;
    let res: i32 = count;
    let tmp: u8;

    guard_mutex(&mut (*ice).gpio_mutex);
    tmp = ap_cs8427_codec_select(ice);
    ap_cs8427_write_byte(ice, (((*device).addr << 1) | 1) as u8, tmp); /* address + read mode */
    while count > 0 {
        *bytes = ap_cs8427_read_byte(ice, tmp);
        bytes = bytes.add(1);
        count -= 1;
    }
    ap_cs8427_codec_deassert(ice, tmp);
    res
}

unsafe fn ap_cs8427_probeaddr(_bus: *mut snd_i2c_bus, addr: u16) -> i32 {
    if addr == 0x10 {
        return 1;
    }
    -ENOENT
}

static ap_cs8427_i2c_ops: snd_i2c_ops = snd_i2c_ops {
    sendbytes: Some(ap_cs8427_sendbytes),
    readbytes: Some(ap_cs8427_readbytes),
    probeaddr: Some(ap_cs8427_probeaddr),
};

/*
 */

unsafe fn snd_ice1712_delta_cs8403_spdif_write(ice: *mut snd_ice1712, bits: u8) {
    let mut tmp: u8;
    let mask1: u8;
    let mask2: u8;
    let mut idx: i32;
    /* send byte to transmitter */
    mask1 = ICE1712_DELTA_SPDIF_OUT_STAT_CLOCK as u8;
    mask2 = ICE1712_DELTA_SPDIF_OUT_STAT_DATA as u8;
    guard_mutex(&mut (*ice).gpio_mutex);
    tmp = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA);
    idx = 7;
    while idx >= 0 {
        tmp &= !(mask1 | mask2);
        if (bits as i32 & (1 << idx)) != 0 {
            tmp |= mask2;
        }
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
        udelay(100);
        tmp |= mask1;
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
        udelay(100);
        idx -= 1;
    }
    tmp &= !mask1;
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
}

unsafe fn delta_spdif_default_get(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) {
    snd_cs8403_decode_spdif_bits(&mut (*ucontrol).value.iec958, (*ice).spdif.cs8403_bits);
}

unsafe fn delta_spdif_default_put(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let val: u32;
    let change: i32;

    val = snd_cs8403_encode_spdif_bits(&mut (*ucontrol).value.iec958);
    scoped_spinlock_irq(&mut (*ice).reg_lock, || {
        change = ((*ice).spdif.cs8403_bits != val) as i32;
        (*ice).spdif.cs8403_bits = val;
        if change == 0 || !(*ice).playback_pro_substream.is_null() {
            return change;
        }
        change
    });
    if change == 0 || !(*ice).playback_pro_substream.is_null() {
        return change;
    }
    snd_ice1712_delta_cs8403_spdif_write(ice, val as u8);
    change
}

unsafe fn delta_spdif_stream_get(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) {
    snd_cs8403_decode_spdif_bits(&mut (*ucontrol).value.iec958, (*ice).spdif.cs8403_stream_bits);
}

unsafe fn delta_spdif_stream_put(ice: *mut snd_ice1712, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    let val: u32;
    let change: i32;

    val = snd_cs8403_encode_spdif_bits(&mut (*ucontrol).value.iec958);
    scoped_spinlock_irq(&mut (*ice).reg_lock, || {
        change = ((*ice).spdif.cs8403_stream_bits != val) as i32;
        (*ice).spdif.cs8403_stream_bits = val;
        if change == 0 || !(*ice).playback_pro_substream.is_null() {
            return change;
        }
        change
    });
    if change == 0 || !(*ice).playback_pro_substream.is_null() {
        return change;
    }
    snd_ice1712_delta_cs8403_spdif_write(ice, val as u8);
    change
}

/*
 * AK4524 on Delta 44 and 66 to choose the chip mask
 */
unsafe fn delta_ak4524_lock(ak: *mut snd_akm4xxx, chip: i32) {
    let priv_: *mut snd_ak4xxx_private = (*ak).private_value[0] as *mut snd_ak4xxx_private;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    snd_ice1712_save_gpio_status(ice);
    (*priv_).cs_mask = if chip == 0 {
        ICE1712_DELTA_CODEC_CHIP_A
    } else {
        ICE1712_DELTA_CODEC_CHIP_B
    };
    (*priv_).cs_addr = (*priv_).cs_mask;
}

/*
 * AK4524 on Delta1010LT to choose the chip address
 */
unsafe fn delta1010lt_ak4524_lock(ak: *mut snd_akm4xxx, chip: i32) {
    let priv_: *mut snd_ak4xxx_private = (*ak).private_value[0] as *mut snd_ak4xxx_private;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    snd_ice1712_save_gpio_status(ice);
    (*priv_).cs_mask = ICE1712_DELTA_1010LT_CS;
    (*priv_).cs_addr = chip << 4;
}

/*
 * AK4524 on Delta66 rev E to choose the chip address
 */
unsafe fn delta66e_ak4524_lock(ak: *mut snd_akm4xxx, chip: i32) {
    let priv_: *mut snd_ak4xxx_private = (*ak).private_value[0] as *mut snd_ak4xxx_private;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    snd_ice1712_save_gpio_status(ice);
    (*priv_).cs_mask = if chip == 0 {
        ICE1712_DELTA_66E_CS_CHIP_A
    } else {
        ICE1712_DELTA_66E_CS_CHIP_B
    };
    (*priv_).cs_addr = (*priv_).cs_mask;
}

/*
 * AK4528 on VX442 to choose the chip mask
 */
unsafe fn vx442_ak4524_lock(ak: *mut snd_akm4xxx, chip: i32) {
    let priv_: *mut snd_ak4xxx_private = (*ak).private_value[0] as *mut snd_ak4xxx_private;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    snd_ice1712_save_gpio_status(ice);
    (*priv_).cs_mask = if chip == 0 {
        ICE1712_VX442_CODEC_CHIP_A
    } else {
        ICE1712_VX442_CODEC_CHIP_B
    };
    (*priv_).cs_addr = (*priv_).cs_mask;
}

/*
 * change the DFS bit according rate for Delta1010
 */
unsafe fn delta_1010_set_rate_val(ice: *mut snd_ice1712, rate: u32) {
    let tmp: u8;
    let mut tmp2: u8;

    if rate == 0 {
        /* no hint - S/PDIF input is master, simply return */
        return;
    }

    guard_mutex(&mut (*ice).gpio_mutex);
    tmp = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA);
    tmp2 = tmp & !(ICE1712_DELTA_DFS as u8);
    if rate > 48000 {
        tmp2 |= ICE1712_DELTA_DFS as u8;
    }
    if tmp != tmp2 {
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp2);
    }
}

/*
 * change the rate of AK4524 on Delta 44/66, AP, 1010LT
 */
unsafe fn delta_ak4524_set_rate_val(ak: *mut snd_akm4xxx, rate: u32) {
    let mut tmp: u8;
    let mut tmp2: u8;
    let ice: *mut snd_ice1712 = (*ak).private_data[0] as *mut snd_ice1712;

    if rate == 0 {
        /* no hint - S/PDIF input is master, simply return */
        return;
    }

    /* check before reset ak4524 to avoid unnecessary clicks */
    scoped_mutex(&mut (*ice).gpio_mutex, || {
        tmp = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA);
    });
    tmp2 = tmp & !(ICE1712_DELTA_DFS as u8);
    if rate > 48000 {
        tmp2 |= ICE1712_DELTA_DFS as u8;
    }
    if tmp == tmp2 {
        return;
    }

    /* do it again */
    snd_akm4xxx_reset(ak, 1);
    scoped_mutex(&mut (*ice).gpio_mutex, || {
        tmp = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA) & !(ICE1712_DELTA_DFS as u8);
        if rate > 48000 {
            tmp |= ICE1712_DELTA_DFS as u8;
        }
        snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
    });
    snd_akm4xxx_reset(ak, 0);
}

/*
 * change the rate of AK4524 on VX442
 */
unsafe fn vx442_ak4524_set_rate_val(ak: *mut snd_akm4xxx, rate: u32) {
    let val: u8;

    val = if rate > 48000 { 0x65 } else { 0x60 };
    if snd_akm4xxx_get(ak, 0, 0x02) != val
        || snd_akm4xxx_get(ak, 1, 0x02) != val
    {
        snd_akm4xxx_reset(ak, 1);
        snd_akm4xxx_write(ak, 0, 0x02, val);
        snd_akm4xxx_write(ak, 1, 0x02, val);
        snd_akm4xxx_reset(ak, 0);
    }
}

/*
 * SPDIF ops for Delta 1010, Dio, 66
 */

/* open callback */
unsafe fn delta_open_spdif(ice: *mut snd_ice1712, _substream: *mut snd_pcm_substream) {
    (*ice).spdif.cs8403_stream_bits = (*ice).spdif.cs8403_bits;
}

/* set up */
unsafe fn delta_setup_spdif(ice: *mut snd_ice1712, rate: i32) {
    let mut tmp: u32;
    let change: i32;

    scoped_spinlock_irqsave(&mut (*ice).reg_lock, || {
        tmp = (*ice).spdif.cs8403_stream_bits;
        if (tmp & 0x01) != 0 {
            /* consumer */
            tmp &= if (tmp & 0x01) != 0 { !0x06 } else { !0x18 };
        }
        match rate {
            32000 => tmp |= if (tmp & 0x01) != 0 { 0x04 } else { 0x00 },
            44100 => tmp |= if (tmp & 0x01) != 0 { 0x00 } else { 0x10 },
            48000 => tmp |= if (tmp & 0x01) != 0 { 0x02 } else { 0x08 },
            _ => tmp |= if (tmp & 0x01) != 0 { 0x00 } else { 0x18 },
        }
        change = ((*ice).spdif.cs8403_stream_bits != tmp) as i32;
        (*ice).spdif.cs8403_stream_bits = tmp;
    });
    if change != 0 {
        snd_ctl_notify(
            (*ice).card,
            SNDRV_CTL_EVENT_MASK_VALUE,
            &mut (*(*ice).spdif.stream_ctl).id,
        );
    }
    snd_ice1712_delta_cs8403_spdif_write(ice, tmp as u8);
}

/* #define snd_ice1712_delta1010lt_wordclock_status_info snd_ctl_boolean_mono_info */
const snd_ice1712_delta1010lt_wordclock_status_info: snd_kcontrol_info_t =
    snd_ctl_boolean_mono_info;

unsafe fn snd_ice1712_delta1010lt_wordclock_status_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let mut reg: i8 = 0x10; /* CS8427 receiver error register */
    let ice: *mut snd_ice1712 = snd_kcontrol_chip(kcontrol);

    if snd_i2c_sendbytes(ice.cs8427, &mut reg as *mut i8 as *mut u8, 1) != 1 {
        dev_err(
            (*(*ice).card).dev,
            "unable to send register 0x%x byte to CS8427\n",
            reg,
        );
    }
    snd_i2c_readbytes((*ice).cs8427, &mut reg as *mut i8 as *mut u8, 1);
    (*ucontrol).value.integer.value[0] = if (reg & CS8427_UNLOCK as i8) != 0 { 1 } else { 0 };
    0
}

static snd_ice1712_delta1010lt_wordclock_status: snd_kcontrol_new = snd_kcontrol_new {
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Word Clock Status".as_ptr(),
    info: Some(snd_ice1712_delta1010lt_wordclock_status_info),
    get: Some(snd_ice1712_delta1010lt_wordclock_status_get),
    ..snd_kcontrol_new::zeroed()
};

/*
 * initialize the chips on M-Audio cards
 */

static akm_audiophile: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4528,
    num_adcs: 2,
    num_dacs: 2,
    ops: snd_akm4xxx_ops {
        set_rate_val: Some(delta_ak4524_set_rate_val),
        ..snd_akm4xxx_ops::zeroed()
    },
    ..snd_akm4xxx::zeroed()
};

static akm_audiophile_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0,
    data_mask: ICE1712_DELTA_AP_DOUT,
    clk_mask: ICE1712_DELTA_AP_CCLK,
    cs_mask: ICE1712_DELTA_AP_CS_CODEC,
    cs_addr: ICE1712_DELTA_AP_CS_CODEC,
    cs_none: 0,
    add_flags: ICE1712_DELTA_AP_CS_DIGITAL,
    mask_flags: 0,
};

static akm_delta410: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4529,
    num_adcs: 2,
    num_dacs: 8,
    ops: snd_akm4xxx_ops {
        set_rate_val: Some(delta_ak4524_set_rate_val),
        ..snd_akm4xxx_ops::zeroed()
    },
    ..snd_akm4xxx::zeroed()
};

static akm_delta410_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 0,
    cif: 0,
    data_mask: ICE1712_DELTA_AP_DOUT,
    clk_mask: ICE1712_DELTA_AP_CCLK,
    cs_mask: ICE1712_DELTA_AP_CS_CODEC,
    cs_addr: ICE1712_DELTA_AP_CS_CODEC,
    cs_none: 0,
    add_flags: ICE1712_DELTA_AP_CS_DIGITAL,
    mask_flags: 0,
};

static akm_delta1010lt: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4524,
    num_adcs: 8,
    num_dacs: 8,
    ops: snd_akm4xxx_ops {
        lock: Some(delta1010lt_ak4524_lock),
        set_rate_val: Some(delta_ak4524_set_rate_val),
        ..snd_akm4xxx_ops::zeroed()
    },
    ..snd_akm4xxx::zeroed()
};

static akm_delta1010lt_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0, /* the default level of the CIF pin from AK4524 */
    data_mask: ICE1712_DELTA_1010LT_DOUT,
    clk_mask: ICE1712_DELTA_1010LT_CCLK,
    cs_mask: 0,
    cs_addr: 0, /* set later */
    cs_none: ICE1712_DELTA_1010LT_CS_NONE,
    add_flags: 0,
    mask_flags: 0,
};

static akm_delta66e: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4524,
    num_adcs: 4,
    num_dacs: 4,
    ops: snd_akm4xxx_ops {
        lock: Some(delta66e_ak4524_lock),
        set_rate_val: Some(delta_ak4524_set_rate_val),
        ..snd_akm4xxx_ops::zeroed()
    },
    ..snd_akm4xxx::zeroed()
};

static akm_delta66e_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0, /* the default level of the CIF pin from AK4524 */
    data_mask: ICE1712_DELTA_66E_DOUT,
    clk_mask: ICE1712_DELTA_66E_CCLK,
    cs_mask: 0,
    cs_addr: 0, /* set later */
    cs_none: 0,
    add_flags: 0,
    mask_flags: 0,
};

static akm_delta44: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4524,
    num_adcs: 4,
    num_dacs: 4,
    ops: snd_akm4xxx_ops {
        lock: Some(delta_ak4524_lock),
        set_rate_val: Some(delta_ak4524_set_rate_val),
        ..snd_akm4xxx_ops::zeroed()
    },
    ..snd_akm4xxx::zeroed()
};

static akm_delta44_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0, /* the default level of the CIF pin from AK4524 */
    data_mask: ICE1712_DELTA_CODEC_SERIAL_DATA,
    clk_mask: ICE1712_DELTA_CODEC_SERIAL_CLOCK,
    cs_mask: 0,
    cs_addr: 0, /* set later */
    cs_none: 0,
    add_flags: 0,
    mask_flags: 0,
};

static akm_vx442: snd_akm4xxx = snd_akm4xxx {
    type_: SND_AK4524,
    num_adcs: 4,
    num_dacs: 4,
    ops: snd_akm4xxx_ops {
        lock: Some(vx442_ak4524_lock),
        set_rate_val: Some(vx442_ak4524_set_rate_val),
        ..snd_akm4xxx_ops::zeroed()
    },
    ..snd_akm4xxx::zeroed()
};

static akm_vx442_priv: snd_ak4xxx_private = snd_ak4xxx_private {
    caddr: 2,
    cif: 0,
    data_mask: ICE1712_VX442_DOUT,
    clk_mask: ICE1712_VX442_CCLK,
    cs_mask: 0,
    cs_addr: 0, /* set later */
    cs_none: 0,
    add_flags: 0,
    mask_flags: 0,
};

/* CONFIG_PM_SLEEP */
unsafe fn snd_ice1712_delta_resume(ice: *mut snd_ice1712) -> i32 {
    let mut akm_img_bak: [u8; AK4XXX_IMAGE_SIZE] = [0; AK4XXX_IMAGE_SIZE];
    let mut akm_vol_bak: [u8; AK4XXX_IMAGE_SIZE] = [0; AK4XXX_IMAGE_SIZE];

    /* init spdif */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_AUDIOPHILE
        | ICE1712_SUBDEVICE_DELTA410
        | ICE1712_SUBDEVICE_DELTA1010E
        | ICE1712_SUBDEVICE_DELTA1010LT
        | ICE1712_SUBDEVICE_VX442
        | ICE1712_SUBDEVICE_DELTA66E => {
            snd_cs8427_init((*ice).i2c, (*ice).cs8427);
        }
        ICE1712_SUBDEVICE_DELTA1010 | ICE1712_SUBDEVICE_MEDIASTATION => {
            /* nothing */
        }
        ICE1712_SUBDEVICE_DELTADIO2496 | ICE1712_SUBDEVICE_DELTA66 => {
            /* Set spdif defaults */
            snd_ice1712_delta_cs8403_spdif_write(ice, (*ice).spdif.cs8403_bits as u8);
        }
        _ => {}
    }

    /* init codec and restore registers */
    if (*ice).akm_codecs != 0 {
        memcpy(
            akm_img_bak.as_mut_ptr() as *mut c_void,
            (*(*ice).akm).images.as_ptr() as *const c_void,
            akm_img_bak.len(),
        );
        memcpy(
            akm_vol_bak.as_mut_ptr() as *mut c_void,
            (*(*ice).akm).volumes.as_ptr() as *const c_void,
            akm_vol_bak.len(),
        );
        snd_akm4xxx_init((*ice).akm);
        memcpy(
            (*(*ice).akm).images.as_mut_ptr() as *mut c_void,
            akm_img_bak.as_ptr() as *const c_void,
            akm_img_bak.len(),
        );
        memcpy(
            (*(*ice).akm).volumes.as_mut_ptr() as *mut c_void,
            akm_vol_bak.as_ptr() as *const c_void,
            akm_vol_bak.len(),
        );
        snd_akm4xxx_reset((*ice).akm, 0);
    }

    0
}

unsafe fn snd_ice1712_delta_suspend(ice: *mut snd_ice1712) -> i32 {
    if (*ice).akm_codecs != 0 {
        /* reset & mute codec */
        snd_akm4xxx_reset((*ice).akm, 1);
    }

    0
}

unsafe fn snd_ice1712_delta_init(ice: *mut snd_ice1712) -> i32 {
    let mut err: i32;
    let ak: *mut snd_akm4xxx;
    let mut tmp: u8;

    if (*ice).eeprom.subvendor == ICE1712_SUBDEVICE_DELTA1010 && (*ice).eeprom.gpiodir == 0x7b {
        (*ice).eeprom.subvendor = ICE1712_SUBDEVICE_DELTA1010E;
    }

    if (*ice).eeprom.subvendor == ICE1712_SUBDEVICE_DELTA66 && (*ice).eeprom.gpiodir == 0xfb {
        (*ice).eeprom.subvendor = ICE1712_SUBDEVICE_DELTA66E;
    }

    /* determine I2C, DACs and ADCs */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_AUDIOPHILE => {
            (*ice).num_total_dacs = 2;
            (*ice).num_total_adcs = 2;
        }
        ICE1712_SUBDEVICE_DELTA410 => {
            (*ice).num_total_dacs = 8;
            (*ice).num_total_adcs = 2;
        }
        ICE1712_SUBDEVICE_DELTA44 | ICE1712_SUBDEVICE_DELTA66 => {
            (*ice).num_total_dacs = if (*ice).omni != 0 { 8 } else { 4 };
            (*ice).num_total_adcs = if (*ice).omni != 0 { 8 } else { 4 };
        }
        ICE1712_SUBDEVICE_DELTA1010
        | ICE1712_SUBDEVICE_DELTA1010E
        | ICE1712_SUBDEVICE_DELTA1010LT
        | ICE1712_SUBDEVICE_MEDIASTATION
        | ICE1712_SUBDEVICE_EDIROLDA2496 => {
            (*ice).num_total_dacs = 8;
            (*ice).num_total_adcs = 8;
        }
        ICE1712_SUBDEVICE_DELTADIO2496 => {
            (*ice).num_total_dacs = 4; /* two AK4324 codecs */
        }
        ICE1712_SUBDEVICE_VX442 | ICE1712_SUBDEVICE_DELTA66E => {
            /* omni not supported yet */
            (*ice).num_total_dacs = 4;
            (*ice).num_total_adcs = 4;
        }
        _ => {}
    }
    /* CONFIG_PM_SLEEP */
    (*ice).pm_resume = Some(snd_ice1712_delta_resume);
    (*ice).pm_suspend = Some(snd_ice1712_delta_suspend);
    (*ice).pm_suspend_enabled = 1;

    /* initialize the SPI clock to high */
    tmp = snd_ice1712_read(ice, ICE1712_IREG_GPIO_DATA);
    tmp |= ICE1712_DELTA_AP_CCLK as u8;
    snd_ice1712_write(ice, ICE1712_IREG_GPIO_DATA, tmp);
    udelay(5);

    /* initialize spdif */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_AUDIOPHILE
        | ICE1712_SUBDEVICE_DELTA410
        | ICE1712_SUBDEVICE_DELTA1010E
        | ICE1712_SUBDEVICE_DELTA1010LT
        | ICE1712_SUBDEVICE_VX442
        | ICE1712_SUBDEVICE_DELTA66E => {
            err = snd_i2c_bus_create(
                (*ice).card,
                c"ICE1712 GPIO 1".as_ptr(),
                core::ptr::null_mut(),
                &mut (*ice).i2c,
            );
            if err < 0 {
                dev_err((*(*ice).card).dev, "unable to create I2C bus\n");
                return err;
            }
            (*(*ice).i2c).private_data = ice as *mut c_void;
            (*(*ice).i2c).ops = &ap_cs8427_i2c_ops;
            err = snd_ice1712_init_cs8427(ice, CS8427_BASE_ADDR);
            if err < 0 {
                return err;
            }
        }
        ICE1712_SUBDEVICE_DELTA1010 | ICE1712_SUBDEVICE_MEDIASTATION => {
            (*ice).gpio.set_pro_rate = Some(delta_1010_set_rate_val);
        }
        ICE1712_SUBDEVICE_DELTADIO2496 => {
            (*ice).gpio.set_pro_rate = Some(delta_1010_set_rate_val);
            (*ice).spdif.ops.open = Some(delta_open_spdif);
            (*ice).spdif.ops.setup_rate = Some(delta_setup_spdif);
            (*ice).spdif.ops.default_get = Some(delta_spdif_default_get);
            (*ice).spdif.ops.default_put = Some(delta_spdif_default_put);
            (*ice).spdif.ops.stream_get = Some(delta_spdif_stream_get);
            (*ice).spdif.ops.stream_put = Some(delta_spdif_stream_put);
            /* Set spdif defaults */
            snd_ice1712_delta_cs8403_spdif_write(ice, (*ice).spdif.cs8403_bits as u8);
        }
        ICE1712_SUBDEVICE_DELTA66 => {
            (*ice).spdif.ops.open = Some(delta_open_spdif);
            (*ice).spdif.ops.setup_rate = Some(delta_setup_spdif);
            (*ice).spdif.ops.default_get = Some(delta_spdif_default_get);
            (*ice).spdif.ops.default_put = Some(delta_spdif_default_put);
            (*ice).spdif.ops.stream_get = Some(delta_spdif_stream_get);
            (*ice).spdif.ops.stream_put = Some(delta_spdif_stream_put);
            /* Set spdif defaults */
            snd_ice1712_delta_cs8403_spdif_write(ice, (*ice).spdif.cs8403_bits as u8);
        }
        _ => {}
    }

    /* no analog? */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010
        | ICE1712_SUBDEVICE_DELTA1010E
        | ICE1712_SUBDEVICE_DELTADIO2496
        | ICE1712_SUBDEVICE_MEDIASTATION => {
            return 0;
        }
        _ => {}
    }

    /* second stage of initialization, analog parts and others */
    ak = kmalloc_obj::<snd_akm4xxx>();
    (*ice).akm = ak;
    if ak.is_null() {
        return -ENOMEM;
    }
    (*ice).akm_codecs = 1;

    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_AUDIOPHILE => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_audiophile, &akm_audiophile_priv, ice);
        }
        ICE1712_SUBDEVICE_DELTA410 => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_delta410, &akm_delta410_priv, ice);
        }
        ICE1712_SUBDEVICE_DELTA1010LT | ICE1712_SUBDEVICE_EDIROLDA2496 => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_delta1010lt, &akm_delta1010lt_priv, ice);
        }
        ICE1712_SUBDEVICE_DELTA66 | ICE1712_SUBDEVICE_DELTA44 => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_delta44, &akm_delta44_priv, ice);
        }
        ICE1712_SUBDEVICE_VX442 => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_vx442, &akm_vx442_priv, ice);
        }
        ICE1712_SUBDEVICE_DELTA66E => {
            err = snd_ice1712_akm4xxx_init(ak, &akm_delta66e, &akm_delta66e_priv, ice);
        }
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }

    err
}

/*
 * additional controls for M-Audio cards
 */

static snd_ice1712_delta1010_wordclock_select: snd_kcontrol_new = ICE1712_GPIO!(
    SNDRV_CTL_ELEM_IFACE_MIXER,
    c"Word Clock Sync".as_ptr(),
    0,
    ICE1712_DELTA_WORD_CLOCK_SELECT,
    1,
    0
);
static snd_ice1712_delta1010lt_wordclock_select: snd_kcontrol_new = ICE1712_GPIO!(
    SNDRV_CTL_ELEM_IFACE_MIXER,
    c"Word Clock Sync".as_ptr(),
    0,
    ICE1712_DELTA_1010LT_WORDCLOCK,
    0,
    0
);
static snd_ice1712_delta1010_wordclock_status: snd_kcontrol_new = ICE1712_GPIO!(
    SNDRV_CTL_ELEM_IFACE_MIXER,
    c"Word Clock Status".as_ptr(),
    0,
    ICE1712_DELTA_WORD_CLOCK_STATUS,
    1,
    SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE
);
static snd_ice1712_deltadio2496_spdif_in_select: snd_kcontrol_new = ICE1712_GPIO!(
    SNDRV_CTL_ELEM_IFACE_MIXER,
    c"IEC958 Input Optical".as_ptr(),
    0,
    ICE1712_DELTA_SPDIF_INPUT_SELECT,
    0,
    0
);
static snd_ice1712_delta_spdif_in_status: snd_kcontrol_new = ICE1712_GPIO!(
    SNDRV_CTL_ELEM_IFACE_MIXER,
    c"Delta IEC958 Input Status".as_ptr(),
    0,
    ICE1712_DELTA_SPDIF_IN_STAT,
    1,
    SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE
);

unsafe fn snd_ice1712_delta_add_controls(ice: *mut snd_ice1712) -> i32 {
    let mut err: i32;

    /* 1010 and dio specific controls */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010 | ICE1712_SUBDEVICE_MEDIASTATION => {
            err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&snd_ice1712_delta1010_wordclock_select, ice as *mut c_void),
            );
            if err < 0 {
                return err;
            }
            err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&snd_ice1712_delta1010_wordclock_status, ice as *mut c_void),
            );
            if err < 0 {
                return err;
            }
        }
        ICE1712_SUBDEVICE_DELTADIO2496 => {
            err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&snd_ice1712_deltadio2496_spdif_in_select, ice as *mut c_void),
            );
            if err < 0 {
                return err;
            }
        }
        ICE1712_SUBDEVICE_DELTA1010E | ICE1712_SUBDEVICE_DELTA1010LT => {
            err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&snd_ice1712_delta1010lt_wordclock_select, ice as *mut c_void),
            );
            if err < 0 {
                return err;
            }
            err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&snd_ice1712_delta1010lt_wordclock_status, ice as *mut c_void),
            );
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }

    /* normal spdif controls */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010
        | ICE1712_SUBDEVICE_DELTADIO2496
        | ICE1712_SUBDEVICE_DELTA66
        | ICE1712_SUBDEVICE_MEDIASTATION => {
            err = snd_ice1712_spdif_build_controls(ice);
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }

    /* spdif status in */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010
        | ICE1712_SUBDEVICE_DELTADIO2496
        | ICE1712_SUBDEVICE_DELTA66
        | ICE1712_SUBDEVICE_MEDIASTATION => {
            err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&snd_ice1712_delta_spdif_in_status, ice as *mut c_void),
            );
            if err < 0 {
                return err;
            }
        }
        _ => {}
    }

    /* ak4524 controls */
    match (*ice).eeprom.subvendor {
        ICE1712_SUBDEVICE_DELTA1010LT
        | ICE1712_SUBDEVICE_AUDIOPHILE
        | ICE1712_SUBDEVICE_DELTA410
        | ICE1712_SUBDEVICE_DELTA44
        | ICE1712_SUBDEVICE_DELTA66
        | ICE1712_SUBDEVICE_VX442
        | ICE1712_SUBDEVICE_DELTA66E
        | ICE1712_SUBDEVICE_EDIROLDA2496 => {
            err = snd_ice1712_akm4xxx_build_controls(ice);
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
pub static mut snd_ice1712_delta_cards: [snd_ice1712_card_info; 11] = [
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_DELTA1010,
        name: c"M Audio Delta 1010".as_ptr(),
        model: c"delta1010".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_DELTADIO2496,
        name: c"M Audio Delta DiO 2496".as_ptr(),
        model: c"dio2496".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        no_mpu401: 1,
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_DELTA66,
        name: c"M Audio Delta 66".as_ptr(),
        model: c"delta66".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        no_mpu401: 1,
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_DELTA44,
        name: c"M Audio Delta 44".as_ptr(),
        model: c"delta44".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        no_mpu401: 1,
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_AUDIOPHILE,
        name: c"M Audio Audiophile 24/96".as_ptr(),
        model: c"audiophile".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_DELTA410,
        name: c"M Audio Delta 410".as_ptr(),
        model: c"delta410".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_DELTA1010LT,
        name: c"M Audio Delta 1010LT".as_ptr(),
        model: c"delta1010lt".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_VX442,
        name: c"Digigram VX442".as_ptr(),
        model: c"vx442".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        no_mpu401: 1,
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_MEDIASTATION,
        name: c"Lionstracs Mediastation".as_ptr(),
        model: c"mediastation".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        subvendor: ICE1712_SUBDEVICE_EDIROLDA2496,
        name: c"Edirol DA2496".as_ptr(),
        model: c"da2496".as_ptr(),
        chip_init: Some(snd_ice1712_delta_init),
        build_controls: Some(snd_ice1712_delta_add_controls),
        ..snd_ice1712_card_info::zeroed()
    },
    snd_ice1712_card_info {
        ..snd_ice1712_card_info::zeroed()
    }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
