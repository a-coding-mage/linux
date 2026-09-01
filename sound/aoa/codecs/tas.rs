// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio driver for tas codec
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 *
 * Open questions:
 *  - How to distinguish between 3004 and versions?
 *
 * FIXMEs:
 *  - This codec driver doesn't honour the 'connected'
 *    property of the aoa_codec struct, hence if
 *    it is used in machines where not everything is
 *    connected it will display wrong mixer elements.
 *  - Driver assumes that the microphone is always
 *    monaureal and connected to the right channel of
 *    the input. This should also be a codec-dependent
 *    flag, maybe the codec should have 3 different
 *    bits for the three different possibilities how
 *    it can be hooked up...
 *    But as long as I don't see any hardware hooked
 *    up that way...
 *  - As Apple notes in their code, the tas3004 seems
 *    to delay the right channel by one sample. You can
 *    see this when for example recording stereo in
 *    audacity, or recording the tas output via cable
 *    on another machine (use a sinus generator or so).
 *    I tried programming the BiQuads but couldn't
 *    make the delay work, maybe someone can read the
 *    datasheet and fix it. The relevant Apple comment
 *    is in AppleTAS3004Audio.cpp lines 1637 ff. Note
 *    that their comment describing how they program
 *    the filters sucks...
 *
 * Other things:
 *  - this should actually register *two* aoa_codec
 *    structs since it has two inputs. Then it must
 *    use the prepare callback to forbid running the
 *    secondary output on a different clock.
 *    Also, whatever bus knows how to do this must
 *    provide two soundbus_dev devices and the fabric
 *    must be able to link them correctly.
 *
 *    I don't even know if Apple ever uses the second
 *    port on the tas3004 though, I don't think their
 *    i2s controllers can even do it. OTOH, they all
 *    derive the clocks from common clocks, so it
 *    might just be possible. The framework allows the
 *    codec to refine the transfer_info items in the
 *    usable callback, so we can simply remove the
 *    rates the second instance is not using when it
 *    actually is in use.
 *    Maybe we'll need to make the sound busses have
 *    a 'clock group id' value so the codec can
 *    determine if the two outputs can be driven at
 *    the same time. But that is likely overkill, up
 *    to the fabric to not link them up incorrectly,
 *    and up to the hardware designer to not wire
 *    them up in some weird unusable way.
 */

/* C dependencies removed from executable Rust:
 * linux/i2c.h, asm/pmac_low_i2c.h, linux/delay.h, linux/module.h,
 * linux/mutex.h, linux/of.h, linux/slab.h, tas.h, tas-gain-table.h,
 * tas-basstreble.h, ../aoa.h, ../soundbus/soundbus.h.
 */

const PFX: &[u8] = b"snd-aoa-codec-tas: \0";

#[repr(C)]
pub struct tas {
    pub codec: aoa_codec,
    pub i2c: *mut i2c_client,
    pub mute_l: u32,
    pub mute_r: u32,
    pub controls_created: u32,
    pub drc_enabled: u32,
    pub hw_enabled: u32,
    pub cached_volume_l: u8,
    pub cached_volume_r: u8,
    pub mixer_l: [u8; 3],
    pub mixer_r: [u8; 3],
    pub bass: u8,
    pub treble: u8,
    pub acr: u8,
    pub drc_range: i32,
    /* protects hardware access against concurrency from
     * userspace when hitting controls and during
     * codec init/suspend/resume */
    pub mtx: mutex,
}

unsafe fn codec_to_tas(codec: *mut aoa_codec) -> *mut tas {
    container_of!(codec, tas, codec)
}

unsafe fn tas_write_reg(tas: *mut tas, reg: u8, len: u8, data: *mut u8) -> i32 {
    if len == 1 {
        i2c_smbus_write_byte_data((*tas).i2c, reg, *data)
    } else {
        i2c_smbus_write_i2c_block_data((*tas).i2c, reg, len, data)
    }
}

unsafe fn tas3004_set_drc(tas: *mut tas) {
    let mut val: [u8; 6] = [0; 6];

    if (*tas).drc_enabled != 0 {
        val[0] = 0x50; /* 3:1 above threshold */
    } else {
        val[0] = 0x51; /* disabled */
    }
    val[1] = 0x02; /* 1:1 below threshold */
    if (*tas).drc_range > 0xef {
        val[2] = 0xef;
    } else if (*tas).drc_range < 0 {
        val[2] = 0x00;
    } else {
        val[2] = (*tas).drc_range as u8;
    }
    val[3] = 0xb0;
    val[4] = 0x60;
    val[5] = 0xa0;

    tas_write_reg(tas, TAS_REG_DRC, 6, val.as_mut_ptr());
}

unsafe fn tas_set_treble(tas: *mut tas) {
    let mut tmp: u8;

    tmp = tas3004_treble((*tas).treble);
    tas_write_reg(tas, TAS_REG_TREBLE, 1, &mut tmp);
}

unsafe fn tas_set_bass(tas: *mut tas) {
    let mut tmp: u8;

    tmp = tas3004_bass((*tas).bass);
    tas_write_reg(tas, TAS_REG_BASS, 1, &mut tmp);
}

unsafe fn tas_set_volume(tas: *mut tas) {
    let mut block: [u8; 6] = [0; 6];
    let mut tmp: i32;
    let mut left: u8;
    let mut right: u8;

    left = (*tas).cached_volume_l;
    right = (*tas).cached_volume_r;

    if left > 177 {
        left = 177;
    }
    if right > 177 {
        right = 177;
    }

    if (*tas).mute_l != 0 {
        left = 0;
    }
    if (*tas).mute_r != 0 {
        right = 0;
    }

    /* analysing the volume and mixer tables shows
     * that they are similar enough when we shift
     * the mixer table down by 4 bits. The error
     * is miniscule, in just one item the error
     * is 1, at a value of 0x07f17b (mixer table
     * value is 0x07f17a) */
    tmp = tas_gaintable[left as usize];
    block[0] = (tmp >> 20) as u8;
    block[1] = (tmp >> 12) as u8;
    block[2] = (tmp >> 4) as u8;
    tmp = tas_gaintable[right as usize];
    block[3] = (tmp >> 20) as u8;
    block[4] = (tmp >> 12) as u8;
    block[5] = (tmp >> 4) as u8;
    tas_write_reg(tas, TAS_REG_VOL, 6, block.as_mut_ptr());
}

unsafe fn tas_set_mixer(tas: *mut tas) {
    let mut block: [u8; 9] = [0; 9];
    let mut tmp: i32;
    let mut i: i32;
    let mut val: u8;

    i = 0;
    while i < 3 {
        val = (*tas).mixer_l[i as usize];
        if val > 177 {
            val = 177;
        }
        tmp = tas_gaintable[val as usize];
        block[(3 * i + 0) as usize] = (tmp >> 16) as u8;
        block[(3 * i + 1) as usize] = (tmp >> 8) as u8;
        block[(3 * i + 2) as usize] = tmp as u8;
        i += 1;
    }
    tas_write_reg(tas, TAS_REG_LMIX, 9, block.as_mut_ptr());

    i = 0;
    while i < 3 {
        val = (*tas).mixer_r[i as usize];
        if val > 177 {
            val = 177;
        }
        tmp = tas_gaintable[val as usize];
        block[(3 * i + 0) as usize] = (tmp >> 16) as u8;
        block[(3 * i + 1) as usize] = (tmp >> 8) as u8;
        block[(3 * i + 2) as usize] = tmp as u8;
        i += 1;
    }
    tas_write_reg(tas, TAS_REG_RMIX, 9, block.as_mut_ptr());
}

/* alsa stuff */

unsafe extern "C" fn tas_dev_register(_dev: *mut snd_device) -> i32 {
    0
}

static ops: snd_device_ops = snd_device_ops {
    dev_register: Some(tas_dev_register),
};

unsafe extern "C" fn tas_snd_vol_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 177;
    0
}

unsafe extern "C" fn tas_snd_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = (*tas).cached_volume_l as _;
    (*ucontrol).value.integer.value[1] = (*tas).cached_volume_r as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    if (*ucontrol).value.integer.value[0] < 0 || (*ucontrol).value.integer.value[0] > 177 {
        return -EINVAL;
    }
    if (*ucontrol).value.integer.value[1] < 0 || (*ucontrol).value.integer.value[1] > 177 {
        return -EINVAL;
    }

    mutex_lock(&mut (*tas).mtx);
    if (*tas).cached_volume_l as c_long == (*ucontrol).value.integer.value[0]
        && (*tas).cached_volume_r as c_long == (*ucontrol).value.integer.value[1]
    {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).cached_volume_l = (*ucontrol).value.integer.value[0] as u8;
    (*tas).cached_volume_r = (*ucontrol).value.integer.value[1] as u8;
    if (*tas).hw_enabled != 0 {
        tas_set_volume(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static volume_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Master Playback Volume\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_vol_info),
    get: Some(tas_snd_vol_get),
    put: Some(tas_snd_vol_put),
    private_value: 0,
};

const tas_snd_mute_info: snd_kcontrol_info_t = snd_ctl_boolean_stereo_info;

unsafe extern "C" fn tas_snd_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = ((*tas).mute_l == 0) as _;
    (*ucontrol).value.integer.value[1] = ((*tas).mute_r == 0) as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    if (*tas).mute_l == ((*ucontrol).value.integer.value[0] == 0) as u32
        && (*tas).mute_r == ((*ucontrol).value.integer.value[1] == 0) as u32
    {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).mute_l = ((*ucontrol).value.integer.value[0] == 0) as u32;
    (*tas).mute_r = ((*ucontrol).value.integer.value[1] == 0) as u32;
    if (*tas).hw_enabled != 0 {
        tas_set_volume(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static mute_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Master Playback Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_mute_info),
    get: Some(tas_snd_mute_get),
    put: Some(tas_snd_mute_put),
    private_value: 0,
};

unsafe extern "C" fn tas_snd_mixer_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 177;
    0
}

unsafe extern "C" fn tas_snd_mixer_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);
    let idx: i32 = (*kcontrol).private_value as i32;

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = (*tas).mixer_l[idx as usize] as _;
    (*ucontrol).value.integer.value[1] = (*tas).mixer_r[idx as usize] as _;
    mutex_unlock(&mut (*tas).mtx);

    0
}

unsafe extern "C" fn tas_snd_mixer_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);
    let idx: i32 = (*kcontrol).private_value as i32;

    mutex_lock(&mut (*tas).mtx);
    if (*tas).mixer_l[idx as usize] as c_long == (*ucontrol).value.integer.value[0]
        && (*tas).mixer_r[idx as usize] as c_long == (*ucontrol).value.integer.value[1]
    {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).mixer_l[idx as usize] = (*ucontrol).value.integer.value[0] as u8;
    (*tas).mixer_r[idx as usize] = (*ucontrol).value.integer.value[1] as u8;

    if (*tas).hw_enabled != 0 {
        tas_set_mixer(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static pcm1_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"PCM Playback Volume\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_mixer_info),
    get: Some(tas_snd_mixer_get),
    put: Some(tas_snd_mixer_put),
    private_value: 0,
};

static monitor_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Monitor Playback Volume\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_mixer_info),
    get: Some(tas_snd_mixer_get),
    put: Some(tas_snd_mixer_put),
    private_value: 2,
};

unsafe extern "C" fn tas_snd_drc_range_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = TAS3004_DRC_MAX as _;
    0
}

unsafe extern "C" fn tas_snd_drc_range_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = (*tas).drc_range as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_drc_range_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    if (*ucontrol).value.integer.value[0] < 0
        || (*ucontrol).value.integer.value[0] > TAS3004_DRC_MAX as _
    {
        return -EINVAL;
    }

    mutex_lock(&mut (*tas).mtx);
    if (*tas).drc_range as c_long == (*ucontrol).value.integer.value[0] {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).drc_range = (*ucontrol).value.integer.value[0] as i32;
    if (*tas).hw_enabled != 0 {
        tas3004_set_drc(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static drc_range_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"DRC Range\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_drc_range_info),
    get: Some(tas_snd_drc_range_get),
    put: Some(tas_snd_drc_range_put),
    private_value: 0,
};

const tas_snd_drc_switch_info: snd_kcontrol_info_t = snd_ctl_boolean_mono_info;

unsafe extern "C" fn tas_snd_drc_switch_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = (*tas).drc_enabled as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_drc_switch_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    if (*tas).drc_enabled as c_long == (*ucontrol).value.integer.value[0] {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).drc_enabled = ((*ucontrol).value.integer.value[0] != 0) as u32;
    if (*tas).hw_enabled != 0 {
        tas3004_set_drc(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static drc_switch_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"DRC Range Switch\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_drc_switch_info),
    get: Some(tas_snd_drc_switch_get),
    put: Some(tas_snd_drc_switch_put),
    private_value: 0,
};

unsafe extern "C" fn tas_snd_capture_source_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    static TEXTS: [*const c_char; 2] = [
        b"Line-In\0".as_ptr() as *const c_char,
        b"Microphone\0".as_ptr() as *const c_char,
    ];

    snd_ctl_enum_info(uinfo, 1, 2, TEXTS.as_ptr())
}

unsafe extern "C" fn tas_snd_capture_source_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.enumerated.item[0] = (((*tas).acr & TAS_ACR_INPUT_B) != 0) as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_capture_source_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);
    let oldacr: i32;

    if (*ucontrol).value.enumerated.item[0] > 1 {
        return -EINVAL;
    }
    mutex_lock(&mut (*tas).mtx);
    oldacr = (*tas).acr as i32;

    /*
     * Despite what the data sheet says in one place, the
     * TAS_ACR_B_MONAUREAL bit forces mono output even when
     * input A (line in) is selected.
     */
    (*tas).acr &= !(TAS_ACR_INPUT_B | TAS_ACR_B_MONAUREAL);
    if (*ucontrol).value.enumerated.item[0] != 0 {
        (*tas).acr |= TAS_ACR_INPUT_B | TAS_ACR_B_MONAUREAL | TAS_ACR_B_MON_SEL_RIGHT;
    }
    if oldacr == (*tas).acr as i32 {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }
    if (*tas).hw_enabled != 0 {
        tas_write_reg(tas, TAS_REG_ACR, 1, &mut (*tas).acr);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static capture_source_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    /* If we name this 'Input Source', it properly shows up in
     * alsamixer as a selection, * but it's shown under the
     * 'Playback' category.
     * If I name it 'Capture Source', it shows up in strange
     * ways (two bools of which one can be selected at a
     * time) but at least it's shown in the 'Capture'
     * category.
     * I was told that this was due to backward compatibility,
     * but I don't understand then why the mangling is *not*
     * done when I name it "Input Source".....
     */
    name: b"Capture Source\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_capture_source_info),
    get: Some(tas_snd_capture_source_get),
    put: Some(tas_snd_capture_source_put),
    private_value: 0,
};

unsafe extern "C" fn tas_snd_treble_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = TAS3004_TREBLE_MIN as _;
    (*uinfo).value.integer.max = TAS3004_TREBLE_MAX as _;
    0
}

unsafe extern "C" fn tas_snd_treble_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = (*tas).treble as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_treble_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    if (*ucontrol).value.integer.value[0] < TAS3004_TREBLE_MIN as _
        || (*ucontrol).value.integer.value[0] > TAS3004_TREBLE_MAX as _
    {
        return -EINVAL;
    }
    mutex_lock(&mut (*tas).mtx);
    if (*tas).treble as c_long == (*ucontrol).value.integer.value[0] {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).treble = (*ucontrol).value.integer.value[0] as u8;
    if (*tas).hw_enabled != 0 {
        tas_set_treble(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static treble_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Treble\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_treble_info),
    get: Some(tas_snd_treble_get),
    put: Some(tas_snd_treble_put),
    private_value: 0,
};

unsafe extern "C" fn tas_snd_bass_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = TAS3004_BASS_MIN as _;
    (*uinfo).value.integer.max = TAS3004_BASS_MAX as _;
    0
}

unsafe extern "C" fn tas_snd_bass_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    mutex_lock(&mut (*tas).mtx);
    (*ucontrol).value.integer.value[0] = (*tas).bass as _;
    mutex_unlock(&mut (*tas).mtx);
    0
}

unsafe extern "C" fn tas_snd_bass_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let tas: *mut tas = snd_kcontrol_chip(kcontrol);

    if (*ucontrol).value.integer.value[0] < TAS3004_BASS_MIN as _
        || (*ucontrol).value.integer.value[0] > TAS3004_BASS_MAX as _
    {
        return -EINVAL;
    }
    mutex_lock(&mut (*tas).mtx);
    if (*tas).bass as c_long == (*ucontrol).value.integer.value[0] {
        mutex_unlock(&mut (*tas).mtx);
        return 0;
    }

    (*tas).bass = (*ucontrol).value.integer.value[0] as u8;
    if (*tas).hw_enabled != 0 {
        tas_set_bass(tas);
    }
    mutex_unlock(&mut (*tas).mtx);
    1
}

static bass_control: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"Bass\0".as_ptr() as *const c_char,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    info: Some(tas_snd_bass_info),
    get: Some(tas_snd_bass_get),
    put: Some(tas_snd_bass_put),
    private_value: 0,
};

static mut tas_transfers: [transfer_info; 3] = [
    transfer_info {
        /* input */
        formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
        transfer_in: 1,
    },
    transfer_info {
        /* output */
        formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S24_BE,
        rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
        transfer_in: 0,
    },
    transfer_info::zeroed(),
];

unsafe extern "C" fn tas_usable(
    _cii: *mut codec_info_item,
    _ti: *mut transfer_info,
    _out: *mut transfer_info,
) -> i32 {
    1
}

unsafe fn tas_reset_init(tas: *mut tas) -> i32 {
    let mut tmp: u8;

    ((*(*(*tas).codec.gpio).methods).all_amps_off.unwrap())((*tas).codec.gpio);
    msleep(5);
    ((*(*(*tas).codec.gpio).methods).set_hw_reset.unwrap())((*tas).codec.gpio, 0);
    msleep(5);
    ((*(*(*tas).codec.gpio).methods).set_hw_reset.unwrap())((*tas).codec.gpio, 1);
    msleep(20);
    ((*(*(*tas).codec.gpio).methods).set_hw_reset.unwrap())((*tas).codec.gpio, 0);
    msleep(10);
    ((*(*(*tas).codec.gpio).methods).all_amps_restore.unwrap())((*tas).codec.gpio);

    tmp = TAS_MCS_SCLK64 | TAS_MCS_SPORT_MODE_I2S | TAS_MCS_SPORT_WL_24BIT;
    if tas_write_reg(tas, TAS_REG_MCS, 1, &mut tmp) != 0 {
        return -ENODEV;
    }

    (*tas).acr |= TAS_ACR_ANALOG_PDOWN;
    if tas_write_reg(tas, TAS_REG_ACR, 1, &mut (*tas).acr) != 0 {
        return -ENODEV;
    }

    tmp = 0;
    if tas_write_reg(tas, TAS_REG_MCS2, 1, &mut tmp) != 0 {
        return -ENODEV;
    }

    tas3004_set_drc(tas);

    /* Set treble & bass to 0dB */
    (*tas).treble = TAS3004_TREBLE_ZERO;
    (*tas).bass = TAS3004_BASS_ZERO;
    tas_set_treble(tas);
    tas_set_bass(tas);

    (*tas).acr &= !TAS_ACR_ANALOG_PDOWN;
    if tas_write_reg(tas, TAS_REG_ACR, 1, &mut (*tas).acr) != 0 {
        return -ENODEV;
    }

    0
}

unsafe extern "C" fn tas_switch_clock(cii: *mut codec_info_item, clock: clock_switch) -> i32 {
    let tas: *mut tas = (*cii).codec_data as *mut tas;

    match clock {
        CLOCK_SWITCH_PREPARE_SLAVE => {
            /* Clocks are going away, mute mute mute */
            ((*(*(*tas).codec.gpio).methods).all_amps_off.unwrap())((*tas).codec.gpio);
            (*tas).hw_enabled = 0;
        }
        CLOCK_SWITCH_SLAVE => {
            /* Clocks are back, re-init the codec */
            mutex_lock(&mut (*tas).mtx);
            tas_reset_init(tas);
            tas_set_volume(tas);
            tas_set_mixer(tas);
            (*tas).hw_enabled = 1;
            ((*(*(*tas).codec.gpio).methods).all_amps_restore.unwrap())((*tas).codec.gpio);
            mutex_unlock(&mut (*tas).mtx);
        }
        _ => {
            /* doesn't happen as of now */
            return -EINVAL;
        }
    }
    0
}

/* CONFIG_PM conditional preserved from C. */
#[cfg(CONFIG_PM)]
unsafe fn tas_suspend(tas: *mut tas) -> i32 {
    mutex_lock(&mut (*tas).mtx);
    (*tas).hw_enabled = 0;
    (*tas).acr |= TAS_ACR_ANALOG_PDOWN;
    tas_write_reg(tas, TAS_REG_ACR, 1, &mut (*tas).acr);
    mutex_unlock(&mut (*tas).mtx);
    0
}

#[cfg(CONFIG_PM)]
unsafe fn tas_resume(tas: *mut tas) -> i32 {
    /* reset codec */
    mutex_lock(&mut (*tas).mtx);
    tas_reset_init(tas);
    tas_set_volume(tas);
    tas_set_mixer(tas);
    (*tas).hw_enabled = 1;
    mutex_unlock(&mut (*tas).mtx);
    0
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn _tas_suspend(cii: *mut codec_info_item, state: pm_message_t) -> i32 {
    let _ = state;
    tas_suspend((*cii).codec_data as *mut tas)
}

#[cfg(CONFIG_PM)]
unsafe extern "C" fn _tas_resume(cii: *mut codec_info_item) -> i32 {
    tas_resume((*cii).codec_data as *mut tas)
}

#[cfg(CONFIG_PM)]
const TAS_SUSPEND_CB: Option<unsafe extern "C" fn(*mut codec_info_item, pm_message_t) -> i32> =
    Some(_tas_suspend);
#[cfg(not(CONFIG_PM))]
const TAS_SUSPEND_CB: Option<unsafe extern "C" fn(*mut codec_info_item, pm_message_t) -> i32> =
    None;
#[cfg(CONFIG_PM)]
const TAS_RESUME_CB: Option<unsafe extern "C" fn(*mut codec_info_item) -> i32> = Some(_tas_resume);
#[cfg(not(CONFIG_PM))]
const TAS_RESUME_CB: Option<unsafe extern "C" fn(*mut codec_info_item) -> i32> = None;

static mut tas_codec_info: codec_info = codec_info {
    transfers: unsafe { tas_transfers.as_mut_ptr() },
    /* in theory, we can drive it at 512 too...
     * but so far the framework doesn't allow
     * for that and I don't see much point in it. */
    sysclock_factor: 256,
    /* same here, could be 32 for just one 16 bit format */
    bus_factor: 64,
    owner: THIS_MODULE,
    usable: Some(tas_usable),
    switch_clock: Some(tas_switch_clock),
    suspend: TAS_SUSPEND_CB,
    resume: TAS_RESUME_CB,
};

unsafe extern "C" fn tas_init_codec(codec: *mut aoa_codec) -> i32 {
    let tas: *mut tas = codec_to_tas(codec);
    let mut err: i32;

    if (*tas).codec.gpio.is_null() || (*(*tas).codec.gpio).methods.is_null() {
        printk(KERN_ERR, PFX.as_ptr(), b"gpios not assigned!!\n\0".as_ptr());
        return -EINVAL;
    }

    mutex_lock(&mut (*tas).mtx);
    if tas_reset_init(tas) != 0 {
        printk(KERN_ERR, PFX.as_ptr(), b"tas failed to initialise\n\0".as_ptr());
        mutex_unlock(&mut (*tas).mtx);
        return -ENXIO;
    }
    (*tas).hw_enabled = 1;
    mutex_unlock(&mut (*tas).mtx);

    if ((*(*tas).codec.soundbus_dev).attach_codec.unwrap())(
        (*tas).codec.soundbus_dev,
        aoa_get_card(),
        &mut tas_codec_info,
        tas as *mut c_void,
    ) != 0
    {
        printk(KERN_ERR, PFX.as_ptr(), b"error attaching tas to soundbus\n\0".as_ptr());
        return -ENODEV;
    }

    if aoa_snd_device_new(SNDRV_DEV_CODEC, tas as *mut c_void, &ops) != 0 {
        printk(KERN_ERR, PFX.as_ptr(), b"failed to create tas snd device!\n\0".as_ptr());
        return -ENODEV;
    }
    err = aoa_snd_ctl_add(snd_ctl_new1(&volume_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&mute_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&pcm1_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&monitor_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&capture_source_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&drc_range_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&drc_switch_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&treble_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    err = aoa_snd_ctl_add(snd_ctl_new1(&bass_control, tas as *mut c_void));
    if err != 0 {
        goto_error(tas, err);
        return err;
    }

    0
}

unsafe fn goto_error(tas: *mut tas, err: i32) {
    let _ = err;
    ((*(*tas).codec.soundbus_dev).detach_codec.unwrap())((*tas).codec.soundbus_dev, tas as *mut c_void);
    snd_device_free(aoa_get_card(), tas as *mut c_void);
}

unsafe extern "C" fn tas_exit_codec(codec: *mut aoa_codec) {
    let tas: *mut tas = codec_to_tas(codec);

    if (*tas).codec.soundbus_dev.is_null() {
        return;
    }
    ((*(*tas).codec.soundbus_dev).detach_codec.unwrap())((*tas).codec.soundbus_dev, tas as *mut c_void);
}

unsafe extern "C" fn tas_i2c_probe(client: *mut i2c_client) -> i32 {
    let node: *mut device_node = (*client).dev.of_node;
    let tas: *mut tas;

    tas = kzalloc_obj::<tas>();

    if tas.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*tas).mtx);
    (*tas).i2c = client;
    i2c_set_clientdata(client, tas as *mut c_void);

    /* seems that half is a saner default */
    (*tas).drc_range = TAS3004_DRC_MAX / 2;

    strscpy((*tas).codec.name.as_mut_ptr(), b"tas\0".as_ptr() as *const c_char);
    (*tas).codec.owner = THIS_MODULE;
    (*tas).codec.init = Some(tas_init_codec);
    (*tas).codec.exit = Some(tas_exit_codec);
    (*tas).codec.node = of_node_get(node);

    if aoa_codec_register(&mut (*tas).codec) != 0 {
        mutex_destroy(&mut (*tas).mtx);
        of_node_put((*tas).codec.node);
        kfree(tas as *mut c_void);
        return -EINVAL;
    }
    printk(
        KERN_DEBUG,
        b"snd-aoa-codec-tas: tas found, addr 0x%02x on %pOF\n\0".as_ptr(),
        (*client).addr as c_uint,
        node,
    );
    0
}

unsafe extern "C" fn tas_i2c_remove(client: *mut i2c_client) {
    let tas: *mut tas = i2c_get_clientdata(client) as *mut tas;
    let mut tmp: u8 = TAS_ACR_ANALOG_PDOWN;

    aoa_codec_unregister(&mut (*tas).codec);
    of_node_put((*tas).codec.node);

    /* power down codec chip */
    tas_write_reg(tas, TAS_REG_ACR, 1, &mut tmp);

    mutex_destroy(&mut (*tas).mtx);
    kfree(tas as *mut c_void);
}

static tas_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"MAC,tas3004\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        driver_data: 0,
    },
    i2c_device_id::zeroed(),
];
module_device_table!(i2c, tas_i2c_id);

static mut tas_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"aoa_codec_tas\0".as_ptr() as *const c_char,
    },
    probe: Some(tas_i2c_probe),
    remove: Some(tas_i2c_remove),
    id_table: tas_i2c_id.as_ptr(),
};

module_i2c_driver!(tas_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
