// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT1724 (Envy24HT)
 *
 *   Lowlevel functions for ESI Maya44 cards
 *
 *      Copyright (c) 2009 Takashi Iwai <tiwai@suse.de>
 *      Based on the patches by Rainer Zimmermann <mail@lightshed.de>
 */

/* C dependencies:
 * linux/init.h, linux/slab.h
 * sound/core.h, sound/control.h, sound/pcm.h, sound/tlv.h
 * ice1712.h, envy24ht.h, maya44.h
 */

/* WM8776 register indexes */
const WM8776_REG_HEADPHONE_L: u8 = 0x00;
const WM8776_REG_HEADPHONE_R: u8 = 0x01;
const WM8776_REG_HEADPHONE_MASTER: u8 = 0x02;
const WM8776_REG_DAC_ATTEN_L: u8 = 0x03;
const WM8776_REG_DAC_ATTEN_R: u8 = 0x04;
const WM8776_REG_DAC_ATTEN_MASTER: u8 = 0x05;
const WM8776_REG_DAC_PHASE: u8 = 0x06;
const WM8776_REG_DAC_CONTROL: u8 = 0x07;
const WM8776_REG_DAC_MUTE: u8 = 0x08;
const WM8776_REG_DAC_DEEMPH: u8 = 0x09;
const WM8776_REG_DAC_IF_CONTROL: u8 = 0x0a;
const WM8776_REG_ADC_IF_CONTROL: u8 = 0x0b;
const WM8776_REG_MASTER_MODE_CONTROL: u8 = 0x0c;
const WM8776_REG_POWERDOWN: u8 = 0x0d;
const WM8776_REG_ADC_ATTEN_L: u8 = 0x0e;
const WM8776_REG_ADC_ATTEN_R: u8 = 0x0f;
const WM8776_REG_ADC_ALC1: u8 = 0x10;
const WM8776_REG_ADC_ALC2: u8 = 0x11;
const WM8776_REG_ADC_ALC3: u8 = 0x12;
const WM8776_REG_ADC_NOISE_GATE: u8 = 0x13;
const WM8776_REG_ADC_LIMITER: u8 = 0x14;
const WM8776_REG_ADC_MUX: u8 = 0x15;
const WM8776_REG_OUTPUT_MUX: u8 = 0x16;
const WM8776_REG_RESET: u8 = 0x17;

const WM8776_NUM_REGS: usize = 0x18;

/* clock ratio identifiers for snd_wm8776_set_rate() */
const WM8776_CLOCK_RATIO_128FS: u32 = 0;
const WM8776_CLOCK_RATIO_192FS: u32 = 1;
const WM8776_CLOCK_RATIO_256FS: u32 = 2;
const WM8776_CLOCK_RATIO_384FS: u32 = 3;
const WM8776_CLOCK_RATIO_512FS: u32 = 4;
const WM8776_CLOCK_RATIO_768FS: u32 = 5;

const WM_VOL_HP: usize = 0;
const WM_VOL_DAC: usize = 1;
const WM_VOL_ADC: usize = 2;
const WM_NUM_VOLS: usize = 3;

const WM_SW_DAC: u32 = 0;
const WM_SW_BYPASS: u32 = 1;
const WM_NUM_SWITCHES: usize = 2;

#[repr(C)]
struct snd_wm8776 {
    addr: u8,
    regs: [u16; WM8776_NUM_REGS],
    volumes: [[u8; 2]; WM_NUM_VOLS],
    switch_bits: u32,
}

#[repr(C)]
struct snd_maya44 {
    ice: *mut snd_ice1712,
    wm: [snd_wm8776; 2],
    mutex: mutex,
}

/* write the given register and save the data to the cache */
unsafe fn wm8776_write(ice: *mut snd_ice1712, wm: *mut snd_wm8776, reg: u8, val: u16) {
    /*
     * WM8776 registers are up to 9 bits wide, bit 8 is placed in the LSB
     * of the address field
     */
    unsafe {
        snd_vt1724_write_i2c(
            ice,
            (*wm).addr,
            (reg << 1) | (((val >> 8) & 1) as u8),
            (val & 0xff) as u8,
        );
        (*wm).regs[reg as usize] = val;
    }
}

/*
 * update the given register with and/or mask and save the data to the cache
 */
unsafe fn wm8776_write_bits(
    ice: *mut snd_ice1712,
    wm: *mut snd_wm8776,
    reg: u8,
    mask: u16,
    mut val: u16,
) -> i32 {
    unsafe {
        val |= (*wm).regs[reg as usize] & !mask;
        if val != (*wm).regs[reg as usize] {
            wm8776_write(ice, wm, reg, val);
            return 1;
        }
    }
    0
}

/*
 * WM8776 volume controls
 */

#[repr(C)]
struct maya_vol_info {
    maxval: u32,       /* volume range: 0..maxval */
    regs: [u8; 2],     /* left and right registers */
    mask: u16,         /* value mask */
    offset: u16,       /* zero-value offset */
    mute: u16,         /* mute bit */
    update: u16,       /* update bits */
    mux_bits: [u8; 2], /* extra bits for ADC mute */
}

static vol_info: [maya_vol_info; WM_NUM_VOLS] = [
    maya_vol_info {
        maxval: 80,
        regs: [WM8776_REG_HEADPHONE_L, WM8776_REG_HEADPHONE_R],
        mask: 0x7f,
        offset: 0x30,
        mute: 0x00,
        update: 0x180, /* update and zero-cross enable */
        mux_bits: [0, 0],
    },
    maya_vol_info {
        maxval: 255,
        regs: [WM8776_REG_DAC_ATTEN_L, WM8776_REG_DAC_ATTEN_R],
        mask: 0xff,
        offset: 0x01,
        mute: 0x00,
        update: 0x100, /* zero-cross enable */
        mux_bits: [0, 0],
    },
    maya_vol_info {
        maxval: 91,
        regs: [WM8776_REG_ADC_ATTEN_L, WM8776_REG_ADC_ATTEN_R],
        mask: 0xff,
        offset: 0xa5,
        mute: 0xa5,
        update: 0x100,           /* update */
        mux_bits: [0x80, 0x40],  /* ADCMUX bits */
    },
];

/*
 * dB tables
 */
/* headphone output: mute, -73..+6db (1db step) */
static db_scale_hp: [u32; 4] = TLV_DB_SCALE_ITEM(-7400, 100, 1);
/* DAC output: mute, -127..0db (0.5db step) */
static db_scale_dac: [u32; 4] = TLV_DB_SCALE_ITEM(-12750, 50, 1);
/* ADC gain: mute, -21..+24db (0.5db step) */
static db_scale_adc: [u32; 4] = TLV_DB_SCALE_ITEM(-2100, 50, 1);

unsafe fn maya_vol_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> i32 {
    unsafe {
        let idx = (*kcontrol).private_value as usize;
        let vol = &vol_info[idx] as *const maya_vol_info;

        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = (*vol).maxval as _;
    }
    0
}

unsafe fn maya_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let wm = &mut (*chip).wm[snd_ctl_get_ioff(kcontrol, &mut (*ucontrol).id) as usize]
            as *mut snd_wm8776;
        let idx = (*kcontrol).private_value as usize;

        let _guard = mutex_guard(&mut (*chip).mutex);
        (*ucontrol).value.integer.value[0] = (*wm).volumes[idx][0] as _;
        (*ucontrol).value.integer.value[1] = (*wm).volumes[idx][1] as _;
    }
    0
}

unsafe fn maya_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let wm = &mut (*chip).wm[snd_ctl_get_ioff(kcontrol, &mut (*ucontrol).id) as usize]
            as *mut snd_wm8776;
        let idx = (*kcontrol).private_value as usize;
        let vol = &vol_info[idx] as *const maya_vol_info;
        let mut changed: i32 = 0;

        let _guard = mutex_guard(&mut (*chip).mutex);
        for ch in 0..2usize {
            let mut val = (*ucontrol).value.integer.value[ch] as u32;
            if val > (*vol).maxval {
                val = (*vol).maxval;
            }
            if val == (*wm).volumes[idx][ch] as u32 {
                continue;
            }
            let mut data: u32 = if val == 0 {
                (*vol).mute as u32
            } else {
                (val - 1) + (*vol).offset as u32
            };
            data |= (*vol).update as u32;
            changed |= wm8776_write_bits(
                (*chip).ice,
                wm,
                (*vol).regs[ch],
                (*vol).mask | (*vol).update,
                data as u16,
            );
            if (*vol).mux_bits[ch] != 0 {
                wm8776_write_bits(
                    (*chip).ice,
                    wm,
                    WM8776_REG_ADC_MUX,
                    (*vol).mux_bits[ch] as u16,
                    if val != 0 { 0 } else { (*vol).mux_bits[ch] as u16 },
                );
            }
            (*wm).volumes[idx][ch] = val as u8;
        }
        changed
    }
}

/*
 * WM8776 switch controls
 */

const fn COMPOSE_SW_VAL(idx: u32, reg: u32, mask: u32) -> u32 {
    idx | (reg << 8) | (mask << 16)
}
const fn GET_SW_VAL_IDX(val: u32) -> u32 {
    val & 0xff
}
const fn GET_SW_VAL_REG(val: u32) -> u32 {
    (val >> 8) & 0xff
}
const fn GET_SW_VAL_MASK(val: u32) -> u32 {
    (val >> 16) & 0xff
}

const maya_sw_info: unsafe fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32 =
    snd_ctl_boolean_mono_info;

unsafe fn maya_sw_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let wm = &mut (*chip).wm[snd_ctl_get_ioff(kcontrol, &mut (*ucontrol).id) as usize]
            as *mut snd_wm8776;
        let idx = GET_SW_VAL_IDX((*kcontrol).private_value as u32);

        (*ucontrol).value.integer.value[0] = (((*wm).switch_bits >> idx) & 1) as _;
    }
    0
}

unsafe fn maya_sw_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let wm = &mut (*chip).wm[snd_ctl_get_ioff(kcontrol, &mut (*ucontrol).id) as usize]
            as *mut snd_wm8776;
        let idx = GET_SW_VAL_IDX((*kcontrol).private_value as u32);
        let mut mask: u32;

        let _guard = mutex_guard(&mut (*chip).mutex);
        mask = 1 << idx;
        (*wm).switch_bits &= !mask;
        let val = (*ucontrol).value.integer.value[0] as u32;
        if val != 0 {
            (*wm).switch_bits |= mask;
        }
        mask = GET_SW_VAL_MASK((*kcontrol).private_value as u32);
        wm8776_write_bits(
            (*chip).ice,
            wm,
            GET_SW_VAL_REG((*kcontrol).private_value as u32) as u8,
            mask as u16,
            if val != 0 { mask as u16 } else { 0 },
        )
    }
}

/*
 * GPIO pins (known ones for maya44)
 */
const GPIO_PHANTOM_OFF: u32 = 2;
const GPIO_MIC_RELAY: u32 = 4;
const GPIO_SPDIF_IN_INV: u32 = 5;
const GPIO_MUST_BE_0: u32 = 7;

/*
 * GPIO switch controls
 */

const fn COMPOSE_GPIO_VAL(shift: u32, inv: u32) -> u32 {
    shift | (inv << 8)
}
const fn GET_GPIO_VAL_SHIFT(val: u32) -> u32 {
    val & 0xff
}
const fn GET_GPIO_VAL_INV(val: u32) -> u32 {
    (val >> 8) & 1
}

unsafe fn maya_set_gpio_bits(ice: *mut snd_ice1712, mask: u32, bits: u32) -> i32 {
    unsafe {
        let data = snd_ice1712_gpio_read(ice);
        if (data & mask) == bits {
            return 0;
        }
        snd_ice1712_gpio_write(ice, (data & !mask) | bits);
    }
    1
}

const maya_gpio_sw_info: unsafe fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32 =
    snd_ctl_boolean_mono_info;

unsafe fn maya_gpio_sw_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let shift = GET_GPIO_VAL_SHIFT((*kcontrol).private_value as u32);
        let mut val = (snd_ice1712_gpio_read((*chip).ice) >> shift) & 1;
        if GET_GPIO_VAL_INV((*kcontrol).private_value as u32) != 0 {
            val = if val == 0 { 1 } else { 0 };
        }
        (*ucontrol).value.integer.value[0] = val as _;
    }
    0
}

unsafe fn maya_gpio_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let shift = GET_GPIO_VAL_SHIFT((*kcontrol).private_value as u32);

        let _guard = mutex_guard(&mut (*chip).mutex);
        let mask = 1 << shift;
        let mut val = (*ucontrol).value.integer.value[0] as u32;
        if GET_GPIO_VAL_INV((*kcontrol).private_value as u32) != 0 {
            val = if val == 0 { 1 } else { 0 };
        }
        val = if val != 0 { mask } else { 0 };
        maya_set_gpio_bits((*chip).ice, mask, val)
    }
}

/*
 * capture source selection
 */

/* known working input slots (0-4) */
const MAYA_LINE_IN: i32 = 1; /* in-2 */
const MAYA_MIC_IN: i32 = 3; /* in-4 */

unsafe fn wm8776_select_input(chip: *mut snd_maya44, idx: i32, line: i32) {
    unsafe {
        wm8776_write_bits(
            (*chip).ice,
            &mut (*chip).wm[idx as usize],
            WM8776_REG_ADC_MUX,
            0x1f,
            (1 << line) as u16,
        );
    }
}

unsafe fn maya_rec_src_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    static texts: [*const i8; 2] = [c"Line".as_ptr(), c"Mic".as_ptr()];

    unsafe { snd_ctl_enum_info(uinfo, 1, texts.len() as u32, texts.as_ptr()) }
}

unsafe fn maya_rec_src_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let sel = if (snd_ice1712_gpio_read((*chip).ice) & (1 << GPIO_MIC_RELAY)) != 0 {
            1
        } else {
            0
        };
        (*ucontrol).value.enumerated.item[0] = sel as _;
    }
    0
}

unsafe fn maya_rec_src_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let sel = (*ucontrol).value.enumerated.item[0] as i32;

        let _guard = mutex_guard(&mut (*chip).mutex);
        let changed = maya_set_gpio_bits(
            (*chip).ice,
            1 << GPIO_MIC_RELAY,
            if sel != 0 { 1 << GPIO_MIC_RELAY } else { 0 },
        );
        wm8776_select_input(
            chip,
            0,
            if sel != 0 { MAYA_MIC_IN } else { MAYA_LINE_IN },
        );
        changed
    }
}

/*
 * Maya44 routing switch settings have different meanings than the standard
 * ice1724 switches as defined in snd_vt1724_pro_route_info (ice1724.c).
 */
unsafe fn maya_pb_route_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    static texts: [*const i8; 5] = [
        c"PCM Out".as_ptr(), /* 0 */
        c"Input 1".as_ptr(),
        c"Input 2".as_ptr(),
        c"Input 3".as_ptr(),
        c"Input 4".as_ptr(),
    ];

    unsafe { snd_ctl_enum_info(uinfo, 1, texts.len() as u32, texts.as_ptr()) }
}

fn maya_pb_route_shift(idx: i32) -> i32 {
    static shift: [u8; 10] = [8, 20, 0, 3, 11, 23, 14, 26, 17, 29];
    shift[(idx % 10) as usize] as i32
}

unsafe fn maya_pb_route_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
        (*ucontrol).value.enumerated.item[0] =
            snd_ice1724_get_route_val((*chip).ice, maya_pb_route_shift(idx)) as _;
    }
    0
}

unsafe fn maya_pb_route_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol) as *mut snd_maya44;
        let idx = snd_ctl_get_ioffidx(kcontrol, &mut (*ucontrol).id);
        snd_ice1724_put_route_val(
            (*chip).ice,
            (*ucontrol).value.enumerated.item[0] as _,
            maya_pb_route_shift(idx),
        )
    }
}

/*
 * controls to be added
 */

static maya_controls: [snd_kcontrol_new; 9] = [
    snd_kcontrol_new {
        name: c"Crossmix Playback Volume".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(maya_vol_info),
        get: Some(maya_vol_get),
        put: Some(maya_vol_put),
        tlv: snd_kcontrol_tlv { p: db_scale_hp.as_ptr() },
        private_value: WM_VOL_HP as _,
        count: 2,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"PCM Playback Volume".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(maya_vol_info),
        get: Some(maya_vol_get),
        put: Some(maya_vol_put),
        tlv: snd_kcontrol_tlv { p: db_scale_dac.as_ptr() },
        private_value: WM_VOL_DAC as _,
        count: 2,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"Line Capture Volume".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        info: Some(maya_vol_info),
        get: Some(maya_vol_get),
        put: Some(maya_vol_put),
        tlv: snd_kcontrol_tlv { p: db_scale_adc.as_ptr() },
        private_value: WM_VOL_ADC as _,
        count: 2,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"PCM Playback Switch".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(maya_sw_info),
        get: Some(maya_sw_get),
        put: Some(maya_sw_put),
        private_value: COMPOSE_SW_VAL(WM_SW_DAC, WM8776_REG_OUTPUT_MUX as u32, 0x01) as _,
        count: 2,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"Bypass Playback Switch".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(maya_sw_info),
        get: Some(maya_sw_get),
        put: Some(maya_sw_put),
        private_value: COMPOSE_SW_VAL(WM_SW_BYPASS, WM8776_REG_OUTPUT_MUX as u32, 0x04) as _,
        count: 2,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"Capture Source".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(maya_rec_src_info),
        get: Some(maya_rec_src_get),
        put: Some(maya_rec_src_put),
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"Mic Phantom Power Switch".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(maya_gpio_sw_info),
        get: Some(maya_gpio_sw_get),
        put: Some(maya_gpio_sw_put),
        private_value: COMPOSE_GPIO_VAL(GPIO_PHANTOM_OFF, 1) as _,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        name: c"SPDIF Capture Switch".as_ptr(),
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        info: Some(maya_gpio_sw_info),
        get: Some(maya_gpio_sw_get),
        put: Some(maya_gpio_sw_put),
        private_value: COMPOSE_GPIO_VAL(GPIO_SPDIF_IN_INV, 1) as _,
        ..unsafe { core::mem::zeroed() }
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"H/W Playback Route".as_ptr(),
        info: Some(maya_pb_route_info),
        get: Some(maya_pb_route_get),
        put: Some(maya_pb_route_put),
        count: 4, /* FIXME: do controls 5-9 have any meaning? */
        ..unsafe { core::mem::zeroed() }
    },
];

unsafe fn maya44_add_controls(ice: *mut snd_ice1712) -> i32 {
    unsafe {
        for i in 0..maya_controls.len() {
            let err = snd_ctl_add(
                (*ice).card,
                snd_ctl_new1(&maya_controls[i] as *const snd_kcontrol_new, (*ice).spec),
            );
            if err < 0 {
                return err;
            }
        }
    }
    0
}

/*
 * initialize a wm8776 chip
 */
unsafe fn wm8776_init(ice: *mut snd_ice1712, wm: *mut snd_wm8776, addr: u32) {
    static inits_wm8776: [u16; 30] = [
        0x02, 0x100, /* R2: headphone L+R muted + update */
        0x05, 0x100, /* R5: DAC output L+R muted + update */
        0x06, 0x000, /* R6: DAC output phase normal */
        0x07, 0x091, /* R7: DAC enable zero cross detection,
                      * normal output */
        0x08, 0x000, /* R8: DAC soft mute off */
        0x09, 0x000, /* R9: no deemph, DAC zero detect disabled */
        0x0a, 0x022, /* R10: DAC I2C mode, std polarities, 24bit */
        0x0b, 0x022, /* R11: ADC I2C mode, std polarities, 24bit,
                      * highpass filter enabled */
        0x0c, 0x042, /* R12: ADC+DAC slave, ADC+DAC 44,1kHz */
        0x0d, 0x000, /* R13: all power up */
        0x0e, 0x100, /* R14: ADC left muted,
                      * enable zero cross detection */
        0x0f, 0x100, /* R15: ADC right muted,
                      * enable zero cross detection */
        /* R16: ALC...*/
        0x11, 0x000, /* R17: disable ALC */
        /* R18: ALC...*/
        /* R19: noise gate...*/
        0x15, 0x000, /* R21: ADC input mux init, mute all inputs */
        0x16, 0x001, /* R22: output mux, select DAC */
        0xff, 0xff,
    ];

    unsafe {
        (*wm).addr = addr as u8;
        /* enable DAC output; mute bypass, aux & all inputs */
        (*wm).switch_bits = 1 << WM_SW_DAC;

        let mut ptr = inits_wm8776.as_ptr();
        while *ptr != 0xff {
            let reg = *ptr as u8;
            ptr = ptr.add(1);
            let data = *ptr;
            ptr = ptr.add(1);
            wm8776_write(ice, wm, reg, data);
        }
    }
}

/*
 * change the rate on the WM8776 codecs.
 * this assumes that the VT17xx's rate is changed by the calling function.
 * NOTE: even though the WM8776's are running in slave mode and rate
 * selection is automatic, we need to call snd_wm8776_set_rate() here
 * to make sure some flags are set correctly.
 */
unsafe fn set_rate(ice: *mut snd_ice1712, rate: u32) {
    unsafe {
        let chip = (*ice).spec as *mut snd_maya44;
        let ratio: u32;

        match rate {
            192000 => ratio = WM8776_CLOCK_RATIO_128FS,
            176400 => ratio = WM8776_CLOCK_RATIO_128FS,
            96000 => ratio = WM8776_CLOCK_RATIO_256FS,
            88200 => ratio = WM8776_CLOCK_RATIO_384FS,
            48000 => ratio = WM8776_CLOCK_RATIO_512FS,
            44100 => ratio = WM8776_CLOCK_RATIO_512FS,
            32000 => ratio = WM8776_CLOCK_RATIO_768FS,
            0 => {
                /* no hint - S/PDIF input is master, simply return */
                return;
            }
            _ => {
                snd_BUG();
                return;
            }
        }

        /*
         * this currently sets the same rate for ADC and DAC, but limits
         * ADC rate to 256X (96kHz). For 256X mode (96kHz), this sets ADC
         * oversampling to 64x, as recommended by WM8776 datasheet.
         * Setting the rate is not really necessary in slave mode.
         */
        let mut adc_ratio = ratio;
        if adc_ratio < WM8776_CLOCK_RATIO_256FS {
            adc_ratio = WM8776_CLOCK_RATIO_256FS;
        }

        let mut val = adc_ratio;
        if adc_ratio == WM8776_CLOCK_RATIO_256FS {
            val |= 8;
        }
        val |= ratio << 4;

        let _guard = mutex_guard(&mut (*chip).mutex);
        for i in 0..2usize {
            wm8776_write_bits(
                ice,
                &mut (*chip).wm[i],
                WM8776_REG_MASTER_MODE_CONTROL,
                0x180,
                val as u16,
            );
        }
    }
}

/*
 * supported sample rates (to override the default one)
 */

static rates: [u32; 8] = [32000, 44100, 48000, 64000, 88200, 96000, 176400, 192000];

/* playback rates: 32..192 kHz */
static dac_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates.len() as _,
    list: rates.as_ptr(),
    mask: 0,
};

/*
 * chip addresses on I2C bus
 */
static wm8776_addr: [u8; 2] = [
    0x34, 0x36, /* codec 0 & 1 */
];

/*
 * initialize the chip
 */
unsafe fn maya44_init(ice: *mut snd_ice1712) -> i32 {
    unsafe {
        let chip: *mut snd_maya44;

        chip = kzalloc_obj::<snd_maya44>();
        if chip.is_null() {
            return -ENOMEM;
        }
        mutex_init(&mut (*chip).mutex);
        (*chip).ice = ice;
        (*ice).spec = chip as *mut _;

        /* initialise codecs */
        (*ice).num_total_dacs = 4;
        (*ice).num_total_adcs = 4;
        (*ice).akm_codecs = 0;

        for i in 0..2usize {
            wm8776_init(ice, &mut (*chip).wm[i], wm8776_addr[i] as u32);
            wm8776_select_input(chip, i as i32, MAYA_LINE_IN);
        }

        /* set card specific rates */
        (*ice).hw_rates = &dac_rates as *const snd_pcm_hw_constraint_list;

        /* register change rate notifier */
        (*ice).gpio.set_pro_rate = Some(set_rate);

        /* RDMA1 (2nd input channel) is used for ADC by default */
        (*ice).force_rdma1 = 1;

        /* have an own routing control */
        (*ice).own_routing = 1;
    }

    0
}

/*
 * Maya44 boards don't provide the EEPROM data except for the vendor IDs.
 * hence the driver needs to sets up it properly.
 */

static maya44_eeprom: [u8; 13] = {
    let mut data = [0u8; 13];
    data[ICE_EEP2_SYSCONF as usize] = 0x45;
    /* clock xin1=49.152MHz, mpu401, 2 stereo ADCs+DACs */
    data[ICE_EEP2_ACLINK as usize] = 0x80;
    /* I2S */
    data[ICE_EEP2_I2S as usize] = 0xf8;
    /* vol, 96k, 24bit, 192k */
    data[ICE_EEP2_SPDIF as usize] = 0xc3;
    /* enable spdif out, spdif out supp, spdif-in, ext spdif out */
    data[ICE_EEP2_GPIO_DIR as usize] = 0xff;
    data[ICE_EEP2_GPIO_DIR1 as usize] = 0xff;
    data[ICE_EEP2_GPIO_DIR2 as usize] = 0xff;
    data[ICE_EEP2_GPIO_MASK as usize] = 0; /*0x9f*/
    data[ICE_EEP2_GPIO_MASK1 as usize] = 0; /*0xff*/
    data[ICE_EEP2_GPIO_MASK2 as usize] = 0; /*0x7f*/
    data[ICE_EEP2_GPIO_STATE as usize] =
        ((1 << GPIO_PHANTOM_OFF) | (1 << GPIO_SPDIF_IN_INV)) as u8;
    data[ICE_EEP2_GPIO_STATE1 as usize] = 0x00;
    data[ICE_EEP2_GPIO_STATE2 as usize] = 0x00;
    data
};

/* entry point */
#[no_mangle]
static mut snd_vt1724_maya44_cards: [snd_ice1712_card_info; 2] = [
    snd_ice1712_card_info {
        subvendor: VT1724_SUBDEVICE_MAYA44,
        name: c"ESI Maya44".as_ptr(),
        model: c"maya44".as_ptr(),
        chip_init: Some(maya44_init),
        build_controls: Some(maya44_add_controls),
        eeprom_size: core::mem::size_of_val(&maya44_eeprom) as _,
        eeprom_data: maya44_eeprom.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() }, /* terminator */
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
