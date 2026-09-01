// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PMac AWACS lowlevel functions
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 * code based on dmasound.c.
 */

// C dependencies: linux/io.h, asm/nvram.h, linux/init.h, linux/delay.h,
// linux/of.h, linux/slab.h, sound/core.h, and "pmac.h".

#[cfg(CONFIG_ADB_CUDA)]
const PMAC_AMP_AVAIL: bool = true;

#[cfg(PMAC_AMP_AVAIL)]
#[repr(C)]
struct awacs_amp {
    amp_master: ::core::ffi::c_uchar,
    amp_vol: [[::core::ffi::c_uchar; 2]; 2],
    amp_tone: [::core::ffi::c_uchar; 2],
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn CHECK_CUDA_AMP() -> bool {
    sys_ctrler == SYS_CTRLER_CUDA
}

unsafe fn snd_pmac_screamer_wait(chip: *mut snd_pmac) {
    let mut timeout: libc::c_long = 2000;
    while in_le32(&mut (*(*chip).awacs).codec_stat) & MASK_VALID == 0 {
        mdelay(1);
        timeout -= 1;
        if timeout == 0 {
            dev_dbg((*(*chip).card).dev, "%s timeout\n\0".as_ptr() as *const _, "__func__\0".as_ptr() as *const _);
            break;
        }
    }
}

/*
 * write AWACS register
 */
unsafe fn snd_pmac_awacs_write(chip: *mut snd_pmac, val: libc::c_int) {
    let mut timeout: libc::c_long = 5000000;

    if (*chip).model == PMAC_SCREAMER {
        snd_pmac_screamer_wait(chip);
    }
    out_le32(
        &mut (*(*chip).awacs).codec_ctrl,
        val | ((*chip).subframe << 22),
    );
    while in_le32(&mut (*(*chip).awacs).codec_ctrl) & MASK_NEWECMD != 0 {
        timeout -= 1;
        if timeout == 0 {
            dev_dbg((*(*chip).card).dev, "%s timeout\n\0".as_ptr() as *const _, "__func__\0".as_ptr() as *const _);
            break;
        }
    }
}

unsafe fn snd_pmac_awacs_write_reg(chip: *mut snd_pmac, reg: libc::c_int, val: libc::c_int) {
    snd_pmac_awacs_write(chip, val | (reg << 12));
    (*chip).awacs_reg[reg as usize] = val;
}

unsafe fn snd_pmac_awacs_write_noreg(chip: *mut snd_pmac, reg: libc::c_int, val: libc::c_int) {
    snd_pmac_awacs_write(chip, val | (reg << 12));
}

#[cfg(CONFIG_PM)]
/* Recalibrate chip */
unsafe fn screamer_recalibrate(chip: *mut snd_pmac) {
    if (*chip).model != PMAC_SCREAMER {
        return;
    }

    /* Sorry for the horrible delays... I hope to get that improved
     * by making the whole PM process asynchronous in a future version
     */
    snd_pmac_awacs_write_noreg(chip, 1, (*chip).awacs_reg[1]);
    if (*chip).manufacturer == 0x1 {
        /* delay for broken crystal part */
        msleep(750);
    }
    snd_pmac_awacs_write_noreg(
        chip,
        1,
        (*chip).awacs_reg[1] | MASK_RECALIBRATE | MASK_CMUTE | MASK_AMUTE,
    );
    snd_pmac_awacs_write_noreg(chip, 1, (*chip).awacs_reg[1]);
    snd_pmac_awacs_write_noreg(chip, 6, (*chip).awacs_reg[6]);
}

#[cfg(not(CONFIG_PM))]
unsafe fn screamer_recalibrate(_chip: *mut snd_pmac) {}

/*
 * additional callback to set the pcm format
 */
unsafe fn snd_pmac_awacs_set_format(chip: *mut snd_pmac) {
    (*chip).awacs_reg[1] &= !MASK_SAMPLERATE;
    (*chip).awacs_reg[1] |= (*chip).rate_index << 3;
    snd_pmac_awacs_write_reg(chip, 1, (*chip).awacs_reg[1]);
}

/*
 * AWACS volume callbacks
 */
/*
 * volumes: 0-15 stereo
 */
unsafe fn snd_pmac_awacs_info_volume(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> libc::c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 15;
    0
}

unsafe fn snd_pmac_awacs_get_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let reg = (*kcontrol).private_value & 0xff;
    let lshift = ((*kcontrol).private_value >> 8) & 0xff;
    let inverted = ((*kcontrol).private_value >> 16) & 1;
    let mut vol = [0 as libc::c_int; 2];

    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    vol[0] = ((*chip).awacs_reg[reg as usize] >> lshift) & 0xf;
    vol[1] = (*chip).awacs_reg[reg as usize] & 0xf;
    if inverted != 0 {
        vol[0] = 0x0f - vol[0];
        vol[1] = 0x0f - vol[1];
    }
    (*ucontrol).value.integer.value[0] = vol[0] as _;
    (*ucontrol).value.integer.value[1] = vol[1] as _;
    0
}

unsafe fn snd_pmac_awacs_put_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let reg = (*kcontrol).private_value & 0xff;
    let lshift = ((*kcontrol).private_value >> 8) & 0xff;
    let inverted = ((*kcontrol).private_value >> 16) & 1;
    let mut vol = [0 as libc::c_uint; 2];

    vol[0] = (*ucontrol).value.integer.value[0] as libc::c_uint;
    vol[1] = (*ucontrol).value.integer.value[1] as libc::c_uint;
    if vol[0] > 0x0f || vol[1] > 0x0f {
        return -EINVAL;
    }
    if inverted != 0 {
        vol[0] = 0x0f - vol[0];
        vol[1] = 0x0f - vol[1];
    }
    vol[0] &= 0x0f;
    vol[1] &= 0x0f;
    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    let oldval = (*chip).awacs_reg[reg as usize];
    let mut val = oldval & !(0xf | (0xf << lshift));
    val |= (vol[0] as libc::c_int) << lshift;
    val |= vol[1] as libc::c_int;
    if oldval != val {
        snd_pmac_awacs_write_reg(chip, reg, val);
    }
    (oldval != reg) as libc::c_int
}

const fn AWACS_VOLUME_PRIVATE(xreg: libc::c_long, xshift: libc::c_long, xinverted: libc::c_long) -> libc::c_long {
    xreg | (xshift << 8) | (xinverted << 16)
}

/*
 * mute master/ogain for AWACS: mono
 */
unsafe fn snd_pmac_awacs_get_switch(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let reg = (*kcontrol).private_value & 0xff;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let invert = ((*kcontrol).private_value >> 16) & 1;

    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    let mut val = ((*chip).awacs_reg[reg as usize] >> shift) & 1;
    if invert != 0 {
        val = 1 - val;
    }
    (*ucontrol).value.integer.value[0] = val as _;
    0
}

unsafe fn snd_pmac_awacs_put_switch(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let reg = (*kcontrol).private_value & 0xff;
    let shift = ((*kcontrol).private_value >> 8) & 0xff;
    let invert = ((*kcontrol).private_value >> 16) & 1;
    let mask = 1 << shift;

    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    let mut val = (*chip).awacs_reg[reg as usize] & !mask;
    if ((*ucontrol).value.integer.value[0] as libc::c_long) != invert {
        val |= mask;
    }
    let changed = ((*chip).awacs_reg[reg as usize] != val) as libc::c_int;
    if changed != 0 {
        snd_pmac_awacs_write_reg(chip, reg, val);
    }
    changed
}

const fn AWACS_SWITCH_PRIVATE(xreg: libc::c_long, xshift: libc::c_long, xinvert: libc::c_long) -> libc::c_long {
    xreg | (xshift << 8) | (xinvert << 16)
}

#[cfg(PMAC_AMP_AVAIL)]
/*
 * controls for perch/whisper extension cards, e.g. G3 desktop
 *
 * TDA7433 connected via i2c address 0x45 (= 0x8a),
 * accessed through cuda
 */
unsafe fn awacs_set_cuda(reg: libc::c_int, val: libc::c_int) {
    let mut req: adb_request = ::core::mem::zeroed();
    cuda_request(&mut req, core::ptr::null_mut(), 5, CUDA_PACKET, CUDA_GET_SET_IIC, 0x8a, reg, val);
    while !req.complete {
        cuda_poll();
    }
}

#[cfg(PMAC_AMP_AVAIL)]
/*
 * level = 0 - 14, 7 = 0 dB
 */
unsafe fn awacs_amp_set_tone(amp: *mut awacs_amp, mut bass: libc::c_int, mut treble: libc::c_int) {
    (*amp).amp_tone[0] = bass as _;
    (*amp).amp_tone[1] = treble as _;
    if bass > 7 {
        bass = (14 - bass) + 8;
    }
    if treble > 7 {
        treble = (14 - treble) + 8;
    }
    awacs_set_cuda(2, (bass << 4) | treble);
}

#[cfg(PMAC_AMP_AVAIL)]
/*
 * vol = 0 - 31 (attenuation), 32 = mute bit, stereo
 */
unsafe fn awacs_amp_set_vol(
    amp: *mut awacs_amp,
    index: libc::c_int,
    lvol: libc::c_int,
    rvol: libc::c_int,
    do_check: libc::c_int,
) -> libc::c_int {
    if do_check != 0
        && (*amp).amp_vol[index as usize][0] as libc::c_int == lvol
        && (*amp).amp_vol[index as usize][1] as libc::c_int == rvol
    {
        return 0;
    }
    awacs_set_cuda(3 + index, lvol);
    awacs_set_cuda(5 + index, rvol);
    (*amp).amp_vol[index as usize][0] = lvol as _;
    (*amp).amp_vol[index as usize][1] = rvol as _;
    1
}

#[cfg(PMAC_AMP_AVAIL)]
/*
 * 0 = -79 dB, 79 = 0 dB, 99 = +20 dB
 */
unsafe fn awacs_amp_set_master(amp: *mut awacs_amp, mut vol: libc::c_int) {
    (*amp).amp_master = vol as _;
    if vol <= 79 {
        vol = 32 + (79 - vol);
    } else {
        vol = 32 - (vol - 79);
    }
    awacs_set_cuda(1, vol);
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn awacs_amp_free(chip: *mut snd_pmac) {
    let amp = (*chip).mixer_data as *mut awacs_amp;
    if amp.is_null() {
        return;
    }
    kfree(amp as *mut _);
    (*chip).mixer_data = core::ptr::null_mut();
    (*chip).mixer_free = None;
}

#[cfg(PMAC_AMP_AVAIL)]
/*
 * mixer controls
 */
unsafe fn snd_pmac_awacs_info_volume_amp(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> libc::c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 31;
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_get_volume_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let index = (*kcontrol).private_value;
    let amp = (*chip).mixer_data as *mut awacs_amp;

    (*ucontrol).value.integer.value[0] = (31 - ((*amp).amp_vol[index as usize][0] & 31)) as _;
    (*ucontrol).value.integer.value[1] = (31 - ((*amp).amp_vol[index as usize][1] & 31)) as _;
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_put_volume_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let index = (*kcontrol).private_value;
    let amp = (*chip).mixer_data as *mut awacs_amp;
    let mut vol = [0 as libc::c_int; 2];

    vol[0] = (31 - ((*ucontrol).value.integer.value[0] as libc::c_int & 31))
        | ((*amp).amp_vol[index as usize][0] as libc::c_int & 32);
    vol[1] = (31 - ((*ucontrol).value.integer.value[1] as libc::c_int & 31))
        | ((*amp).amp_vol[index as usize][1] as libc::c_int & 32);
    awacs_amp_set_vol(amp, index as _, vol[0], vol[1], 1)
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_get_switch_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let index = (*kcontrol).private_value;
    let amp = (*chip).mixer_data as *mut awacs_amp;

    (*ucontrol).value.integer.value[0] = if (*amp).amp_vol[index as usize][0] & 32 != 0 { 0 } else { 1 };
    (*ucontrol).value.integer.value[1] = if (*amp).amp_vol[index as usize][1] & 32 != 0 { 0 } else { 1 };
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_put_switch_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let index = (*kcontrol).private_value;
    let amp = (*chip).mixer_data as *mut awacs_amp;
    let mut vol = [0 as libc::c_int; 2];

    vol[0] = if (*ucontrol).value.integer.value[0] != 0 { 0 } else { 32 }
        | ((*amp).amp_vol[index as usize][0] as libc::c_int & 31);
    vol[1] = if (*ucontrol).value.integer.value[1] != 0 { 0 } else { 32 }
        | ((*amp).amp_vol[index as usize][1] as libc::c_int & 31);
    awacs_amp_set_vol(amp, index as _, vol[0], vol[1], 1)
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_info_tone_amp(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> libc::c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 14;
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_get_tone_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let index = (*kcontrol).private_value;
    let amp = (*chip).mixer_data as *mut awacs_amp;

    (*ucontrol).value.integer.value[0] = (*amp).amp_tone[index as usize] as _;
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_put_tone_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let index = (*kcontrol).private_value;
    let amp = (*chip).mixer_data as *mut awacs_amp;
    let val = (*ucontrol).value.integer.value[0] as libc::c_uint;

    if val > 14 {
        return -EINVAL;
    }
    if val as libc::c_uchar != (*amp).amp_tone[index as usize] {
        (*amp).amp_tone[index as usize] = val as _;
        awacs_amp_set_tone(amp, (*amp).amp_tone[0] as _, (*amp).amp_tone[1] as _);
        return 1;
    }
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_info_master_amp(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> libc::c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 99;
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_get_master_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let amp = (*chip).mixer_data as *mut awacs_amp;

    (*ucontrol).value.integer.value[0] = (*amp).amp_master as _;
    0
}

#[cfg(PMAC_AMP_AVAIL)]
unsafe fn snd_pmac_awacs_put_master_amp(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let amp = (*chip).mixer_data as *mut awacs_amp;
    let val = (*ucontrol).value.integer.value[0] as libc::c_uint;

    if val > 99 {
        return -EINVAL;
    }
    if val as libc::c_uchar != (*amp).amp_master {
        (*amp).amp_master = val as _;
        awacs_amp_set_master(amp, (*amp).amp_master as _);
        return 1;
    }
    0
}

#[cfg(PMAC_AMP_AVAIL)]
const AMP_CH_SPK: libc::c_long = 0;
#[cfg(PMAC_AMP_AVAIL)]
const AMP_CH_HD: libc::c_long = 1;

#[cfg(PMAC_AMP_AVAIL)]
static snd_pmac_awacs_amp_vol: [snd_kcontrol_new; 5] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Speaker Playback Volume".as_ptr(), info: Some(snd_pmac_awacs_info_volume_amp), get: Some(snd_pmac_awacs_get_volume_amp), put: Some(snd_pmac_awacs_put_volume_amp), private_value: AMP_CH_SPK, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Headphone Playback Volume".as_ptr(), info: Some(snd_pmac_awacs_info_volume_amp), get: Some(snd_pmac_awacs_get_volume_amp), put: Some(snd_pmac_awacs_put_volume_amp), private_value: AMP_CH_HD, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Tone Control - Bass".as_ptr(), info: Some(snd_pmac_awacs_info_tone_amp), get: Some(snd_pmac_awacs_get_tone_amp), put: Some(snd_pmac_awacs_put_tone_amp), private_value: 0, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Tone Control - Treble".as_ptr(), info: Some(snd_pmac_awacs_info_tone_amp), get: Some(snd_pmac_awacs_get_tone_amp), put: Some(snd_pmac_awacs_put_tone_amp), private_value: 1, ..unsafe { core::mem::zeroed() } },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: c"Amp Master Playback Volume".as_ptr(), info: Some(snd_pmac_awacs_info_master_amp), get: Some(snd_pmac_awacs_get_master_amp), put: Some(snd_pmac_awacs_put_master_amp), ..unsafe { core::mem::zeroed() } },
];

#[cfg(PMAC_AMP_AVAIL)]
static snd_pmac_awacs_amp_hp_sw: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Headphone Playback Switch".as_ptr(),
    info: Some(snd_pmac_boolean_stereo_info),
    get: Some(snd_pmac_awacs_get_switch_amp),
    put: Some(snd_pmac_awacs_put_switch_amp),
    private_value: AMP_CH_HD,
    ..unsafe { core::mem::zeroed() }
};

#[cfg(PMAC_AMP_AVAIL)]
static snd_pmac_awacs_amp_spk_sw: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: c"Speaker Playback Switch".as_ptr(),
    info: Some(snd_pmac_boolean_stereo_info),
    get: Some(snd_pmac_awacs_get_switch_amp),
    put: Some(snd_pmac_awacs_put_switch_amp),
    private_value: AMP_CH_SPK,
    ..unsafe { core::mem::zeroed() }
};

/*
 * mic boost for screamer
 */
unsafe fn snd_pmac_screamer_mic_boost_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> libc::c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 3;
    0
}

unsafe fn snd_pmac_screamer_mic_boost_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mut val = 0;

    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    if (*chip).awacs_reg[6] & MASK_MIC_BOOST != 0 {
        val |= 2;
    }
    if (*chip).awacs_reg[0] & MASK_GAINLINE != 0 {
        val |= 1;
    }
    (*ucontrol).value.integer.value[0] = val as _;
    0
}

unsafe fn snd_pmac_screamer_mic_boost_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> libc::c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;

    guard_spinlock_irqsave(&mut (*chip).reg_lock);
    let mut val0 = (*chip).awacs_reg[0] & !MASK_GAINLINE;
    let mut val6 = (*chip).awacs_reg[6] & !MASK_MIC_BOOST;
    if (*ucontrol).value.integer.value[0] & 1 != 0 {
        val0 |= MASK_GAINLINE;
    }
    if (*ucontrol).value.integer.value[0] & 2 != 0 {
        val6 |= MASK_MIC_BOOST;
    }
    if val0 != (*chip).awacs_reg[0] {
        snd_pmac_awacs_write_reg(chip, 0, val0);
        changed = 1;
    }
    if val6 != (*chip).awacs_reg[6] {
        snd_pmac_awacs_write_reg(chip, 6, val6);
        changed = 1;
    }
    changed
}

const fn AWACS_VOLUME(
    xname: *const libc::c_char,
    xreg: libc::c_long,
    xshift: libc::c_long,
    xinverted: libc::c_long,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: 0,
        info: Some(snd_pmac_awacs_info_volume),
        get: Some(snd_pmac_awacs_get_volume),
        put: Some(snd_pmac_awacs_put_volume),
        private_value: AWACS_VOLUME_PRIVATE(xreg, xshift, xinverted),
        ..unsafe { core::mem::zeroed() }
    }
}

const fn AWACS_SWITCH(
    xname: *const libc::c_char,
    xreg: libc::c_long,
    xshift: libc::c_long,
    xinvert: libc::c_long,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: 0,
        info: Some(snd_pmac_boolean_mono_info),
        get: Some(snd_pmac_awacs_get_switch),
        put: Some(snd_pmac_awacs_put_switch),
        private_value: AWACS_SWITCH_PRIVATE(xreg, xshift, xinvert),
        ..unsafe { core::mem::zeroed() }
    }
}

/*
 * lists of mixer elements
 */
static snd_pmac_awacs_mixers: [snd_kcontrol_new; 2] = [
    AWACS_SWITCH(c"Master Capture Switch".as_ptr(), 1, SHIFT_LOOPTHRU, 0),
    AWACS_VOLUME(c"Master Capture Volume".as_ptr(), 0, 4, 0),
    /* AWACS_SWITCH("Unknown Playback Switch", 6, SHIFT_PAROUT0, 0), */
];

static snd_pmac_screamer_mixers_beige: [snd_kcontrol_new; 4] = [
    AWACS_VOLUME(c"Master Playback Volume".as_ptr(), 2, 6, 1),
    AWACS_VOLUME(c"Play-through Playback Volume".as_ptr(), 5, 6, 1),
    AWACS_SWITCH(c"Line Capture Switch".as_ptr(), 0, SHIFT_MUX_MIC, 0),
    AWACS_SWITCH(c"CD Capture Switch".as_ptr(), 0, SHIFT_MUX_LINE, 0),
];

static snd_pmac_screamer_mixers_lo: [snd_kcontrol_new; 1] = [
    AWACS_VOLUME(c"Line out Playback Volume".as_ptr(), 2, 6, 1),
];

static snd_pmac_screamer_mixers_imac: [snd_kcontrol_new; 2] = [
    AWACS_VOLUME(c"Play-through Playback Volume".as_ptr(), 5, 6, 1),
    AWACS_SWITCH(c"CD Capture Switch".as_ptr(), 0, SHIFT_MUX_CD, 0),
];

static snd_pmac_screamer_mixers_g4agp: [snd_kcontrol_new; 4] = [
    AWACS_VOLUME(c"Line out Playback Volume".as_ptr(), 2, 6, 1),
    AWACS_VOLUME(c"Master Playback Volume".as_ptr(), 5, 6, 1),
    AWACS_SWITCH(c"CD Capture Switch".as_ptr(), 0, SHIFT_MUX_CD, 0),
    AWACS_SWITCH(c"Line Capture Switch".as_ptr(), 0, SHIFT_MUX_MIC, 0),
];

static snd_pmac_awacs_mixers_pmac7500: [snd_kcontrol_new; 3] = [
    AWACS_VOLUME(c"Line out Playback Volume".as_ptr(), 2, 6, 1),
    AWACS_SWITCH(c"CD Capture Switch".as_ptr(), 0, SHIFT_MUX_CD, 0),
    AWACS_SWITCH(c"Line Capture Switch".as_ptr(), 0, SHIFT_MUX_MIC, 0),
];

static snd_pmac_awacs_mixers_pmac5500: [snd_kcontrol_new; 1] = [
    AWACS_VOLUME(c"Headphone Playback Volume".as_ptr(), 2, 6, 1),
];

static snd_pmac_awacs_mixers_pmac: [snd_kcontrol_new; 2] = [
    AWACS_VOLUME(c"Master Playback Volume".as_ptr(), 2, 6, 1),
    AWACS_SWITCH(c"CD Capture Switch".as_ptr(), 0, SHIFT_MUX_CD, 0),
];

/* FIXME: is this correct order?
 * screamer (powerbook G3 pismo) seems to have different bits...
 */
static snd_pmac_awacs_mixers2: [snd_kcontrol_new; 2] = [
    AWACS_SWITCH(c"Line Capture Switch".as_ptr(), 0, SHIFT_MUX_LINE, 0),
    AWACS_SWITCH(c"Mic Capture Switch".as_ptr(), 0, SHIFT_MUX_MIC, 0),
];

static snd_pmac_screamer_mixers2: [snd_kcontrol_new; 2] = [
    AWACS_SWITCH(c"Line Capture Switch".as_ptr(), 0, SHIFT_MUX_MIC, 0),
    AWACS_SWITCH(c"Mic Capture Switch".as_ptr(), 0, SHIFT_MUX_LINE, 0),
];

static snd_pmac_awacs_mixers2_pmac5500: [snd_kcontrol_new; 1] = [
    AWACS_SWITCH(c"CD Capture Switch".as_ptr(), 0, SHIFT_MUX_CD, 0),
];

static snd_pmac_awacs_master_sw: snd_kcontrol_new =
    AWACS_SWITCH(c"Master Playback Switch".as_ptr(), 1, SHIFT_HDMUTE, 1);

static snd_pmac_awacs_master_sw_imac: snd_kcontrol_new =
    AWACS_SWITCH(c"Line out Playback Switch".as_ptr(), 1, SHIFT_HDMUTE, 1);

static snd_pmac_awacs_master_sw_pmac5500: snd_kcontrol_new =
    AWACS_SWITCH(c"Headphone Playback Switch".as_ptr(), 1, SHIFT_HDMUTE, 1);

static snd_pmac_awacs_mic_boost: [snd_kcontrol_new; 1] = [
    AWACS_SWITCH(c"Mic Boost Capture Switch".as_ptr(), 0, SHIFT_GAINLINE, 0),
];

static snd_pmac_screamer_mic_boost: [snd_kcontrol_new; 1] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Mic Boost Capture Volume".as_ptr(),
        info: Some(snd_pmac_screamer_mic_boost_info),
        get: Some(snd_pmac_screamer_mic_boost_get),
        put: Some(snd_pmac_screamer_mic_boost_put),
        ..unsafe { core::mem::zeroed() }
    },
];

static snd_pmac_awacs_mic_boost_pmac7500: [snd_kcontrol_new; 1] = [
    AWACS_SWITCH(c"Line Boost Capture Switch".as_ptr(), 0, SHIFT_GAINLINE, 0),
];

static snd_pmac_screamer_mic_boost_beige: [snd_kcontrol_new; 2] = [
    AWACS_SWITCH(c"Line Boost Capture Switch".as_ptr(), 0, SHIFT_GAINLINE, 0),
    AWACS_SWITCH(c"CD Boost Capture Switch".as_ptr(), 6, SHIFT_MIC_BOOST, 0),
];

static snd_pmac_screamer_mic_boost_imac: [snd_kcontrol_new; 2] = [
    AWACS_SWITCH(c"Line Boost Capture Switch".as_ptr(), 0, SHIFT_GAINLINE, 0),
    AWACS_SWITCH(c"Mic Boost Capture Switch".as_ptr(), 6, SHIFT_MIC_BOOST, 0),
];

static snd_pmac_awacs_speaker_vol: [snd_kcontrol_new; 1] = [
    AWACS_VOLUME(c"Speaker Playback Volume".as_ptr(), 4, 6, 1),
];

static snd_pmac_awacs_speaker_sw: snd_kcontrol_new =
    AWACS_SWITCH(c"Speaker Playback Switch".as_ptr(), 1, SHIFT_SPKMUTE, 1);

static snd_pmac_awacs_speaker_sw_imac1: snd_kcontrol_new =
    AWACS_SWITCH(c"Speaker Playback Switch".as_ptr(), 1, SHIFT_PAROUT1, 1);

static snd_pmac_awacs_speaker_sw_imac2: snd_kcontrol_new =
    AWACS_SWITCH(c"Speaker Playback Switch".as_ptr(), 1, SHIFT_PAROUT1, 0);

/*
 * add new mixer elements to the card
 */
unsafe fn build_mixers(
    chip: *mut snd_pmac,
    nums: libc::c_int,
    mixers: *const snd_kcontrol_new,
) -> libc::c_int {
    let mut i = 0;
    while i < nums {
        let err = snd_ctl_add((*chip).card, snd_ctl_new1(mixers.add(i as usize), chip as *mut _));
        if err < 0 {
            return err;
        }
        i += 1;
    }
    0
}

/*
 * restore all registers
 */
unsafe fn awacs_restore_all_regs(chip: *mut snd_pmac) {
    snd_pmac_awacs_write_noreg(chip, 0, (*chip).awacs_reg[0]);
    snd_pmac_awacs_write_noreg(chip, 1, (*chip).awacs_reg[1]);
    snd_pmac_awacs_write_noreg(chip, 2, (*chip).awacs_reg[2]);
    snd_pmac_awacs_write_noreg(chip, 4, (*chip).awacs_reg[4]);
    if (*chip).model == PMAC_SCREAMER {
        snd_pmac_awacs_write_noreg(chip, 5, (*chip).awacs_reg[5]);
        snd_pmac_awacs_write_noreg(chip, 6, (*chip).awacs_reg[6]);
        snd_pmac_awacs_write_noreg(chip, 7, (*chip).awacs_reg[7]);
    }
}

#[cfg(CONFIG_PM)]
unsafe fn snd_pmac_awacs_suspend(chip: *mut snd_pmac) {
    snd_pmac_awacs_write_noreg(chip, 1, (*chip).awacs_reg[1] | MASK_AMUTE | MASK_CMUTE);
}

#[cfg(CONFIG_PM)]
unsafe fn snd_pmac_awacs_resume(chip: *mut snd_pmac) {
    if of_machine_is_compatible(c"PowerBook3,1".as_ptr()) || of_machine_is_compatible(c"PowerBook3,2".as_ptr()) {
        msleep(100);
        snd_pmac_awacs_write_reg(chip, 1, (*chip).awacs_reg[1] & !MASK_PAROUT);
        msleep(300);
    }

    awacs_restore_all_regs(chip);
    if (*chip).model == PMAC_SCREAMER {
        /* reset power bits in reg 6 */
        mdelay(5);
        snd_pmac_awacs_write_noreg(chip, 6, (*chip).awacs_reg[6]);
    }
    screamer_recalibrate(chip);
    #[cfg(PMAC_AMP_AVAIL)]
    if !(*chip).mixer_data.is_null() {
        let amp = (*chip).mixer_data as *mut awacs_amp;
        awacs_amp_set_vol(amp, 0, (*amp).amp_vol[0][0] as _, (*amp).amp_vol[0][1] as _, 0);
        awacs_amp_set_vol(amp, 1, (*amp).amp_vol[1][0] as _, (*amp).amp_vol[1][1] as _, 0);
        awacs_amp_set_tone(amp, (*amp).amp_tone[0] as _, (*amp).amp_tone[1] as _);
        awacs_amp_set_master(amp, (*amp).amp_master as _);
    }
}

unsafe fn IS_PM7500() -> bool {
    of_machine_is_compatible(c"AAPL,7500".as_ptr())
        || of_machine_is_compatible(c"AAPL,8500".as_ptr())
        || of_machine_is_compatible(c"AAPL,9500".as_ptr())
}
unsafe fn IS_PM5500() -> bool {
    of_machine_is_compatible(c"AAPL,e411".as_ptr())
}
unsafe fn IS_BEIGE() -> bool {
    of_machine_is_compatible(c"AAPL,Gossamer".as_ptr())
}
unsafe fn IS_IMAC1() -> bool {
    of_machine_is_compatible(c"PowerMac2,1".as_ptr())
}
unsafe fn IS_IMAC2() -> bool {
    of_machine_is_compatible(c"PowerMac2,2".as_ptr())
        || of_machine_is_compatible(c"PowerMac4,1".as_ptr())
}
unsafe fn IS_G4AGP() -> bool {
    of_machine_is_compatible(c"PowerMac3,1".as_ptr())
}
unsafe fn IS_LOMBARD() -> bool {
    of_machine_is_compatible(c"PowerBook1,1".as_ptr())
}

static mut imac1: libc::c_int = 0;
static mut imac2: libc::c_int = 0;

#[cfg(PMAC_SUPPORT_AUTOMUTE)]
/*
 * auto-mute stuffs
 */
unsafe fn snd_pmac_awacs_detect_headphone(chip: *mut snd_pmac) -> libc::c_int {
    if in_le32(&mut (*(*chip).awacs).codec_stat) & (*chip).hp_stat_mask != 0 { 1 } else { 0 }
}

#[cfg(all(PMAC_SUPPORT_AUTOMUTE, PMAC_AMP_AVAIL))]
unsafe fn toggle_amp_mute(amp: *mut awacs_amp, index: libc::c_int, mute: libc::c_int) -> libc::c_int {
    let mut vol = [0 as libc::c_int; 2];
    vol[0] = (*amp).amp_vol[index as usize][0] as libc::c_int & 31;
    vol[1] = (*amp).amp_vol[index as usize][1] as libc::c_int & 31;
    if mute != 0 {
        vol[0] |= 32;
        vol[1] |= 32;
    }
    awacs_amp_set_vol(amp, index, vol[0], vol[1], 1)
}

#[cfg(PMAC_SUPPORT_AUTOMUTE)]
unsafe fn snd_pmac_awacs_update_automute(chip: *mut snd_pmac, do_notify: libc::c_int) {
    if (*chip).auto_mute != 0 {
        #[cfg(PMAC_AMP_AVAIL)]
        if !(*chip).mixer_data.is_null() {
            let amp = (*chip).mixer_data as *mut awacs_amp;
            let mut changed;
            if snd_pmac_awacs_detect_headphone(chip) != 0 {
                changed = toggle_amp_mute(amp, AMP_CH_HD as _, 0);
                changed |= toggle_amp_mute(amp, AMP_CH_SPK as _, 1);
            } else {
                changed = toggle_amp_mute(amp, AMP_CH_HD as _, 1);
                changed |= toggle_amp_mute(amp, AMP_CH_SPK as _, 0);
            }
            if do_notify != 0 && changed == 0 {
                return;
            }
        } else
        {
            let mut reg = (*chip).awacs_reg[1] | (MASK_HDMUTE | MASK_SPKMUTE);
            if imac1 != 0 {
                reg &= !MASK_SPKMUTE;
                reg |= MASK_PAROUT1;
            } else if imac2 != 0 {
                reg &= !MASK_SPKMUTE;
                reg &= !MASK_PAROUT1;
            }
            if snd_pmac_awacs_detect_headphone(chip) != 0 {
                reg &= !MASK_HDMUTE;
            } else if imac1 != 0 {
                reg &= !MASK_PAROUT1;
            } else if imac2 != 0 {
                reg |= MASK_PAROUT1;
            } else {
                reg &= !MASK_SPKMUTE;
            }
            if do_notify != 0 && reg == (*chip).awacs_reg[1] {
                return;
            }
            snd_pmac_awacs_write_reg(chip, 1, reg);
        }
        if do_notify != 0 {
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).master_sw_ctl).id);
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).speaker_sw_ctl).id);
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*chip).hp_detect_ctl).id);
        }
    }
}

/*
 * initialize chip
 */
pub unsafe fn snd_pmac_awacs_init(chip: *mut snd_pmac) -> libc::c_int {
    let pm7500 = IS_PM7500() as libc::c_int;
    let pm5500 = IS_PM5500() as libc::c_int;
    let beige = IS_BEIGE() as libc::c_int;
    let g4agp = IS_G4AGP() as libc::c_int;
    let lombard = IS_LOMBARD() as libc::c_int;
    let mut err: libc::c_int;
    let mut vol: libc::c_int;
    let mut vmaster_sw: *mut snd_kcontrol;
    let mut vmaster_vol: *mut snd_kcontrol;
    let mut master_vol: *mut snd_kcontrol;
    let mut speaker_vol: *mut snd_kcontrol;

    imac1 = IS_IMAC1() as libc::c_int;
    imac2 = IS_IMAC2() as libc::c_int;
    let imac = imac1 != 0 || imac2 != 0;
    /* looks like MASK_GAINLINE triggers something, so we set here
     * as start-up
     */
    (*chip).awacs_reg[0] = MASK_MUX_CD | 0xff | MASK_GAINLINE;
    (*chip).awacs_reg[1] = MASK_CMUTE | MASK_AMUTE;
    /* FIXME: Only machines with external SRS module need MASK_PAROUT */
    if (*chip).has_iic != 0 || (*chip).device_id == 0x5 ||
        /* chip->_device_id == 0x8 || */
        (*chip).device_id == 0xb
    {
        (*chip).awacs_reg[1] |= MASK_PAROUT;
    }
    /* get default volume from nvram */
    // vol = (~nvram_read_byte(0x1308) & 7) << 1;
    // vol = ((pmac_xpram_read( 8 ) & 7 ) << 1 );
    vol = 0x0f; /* no, on alsa, muted as default */
    vol = vol + (vol << 6);
    (*chip).awacs_reg[2] = vol;
    (*chip).awacs_reg[4] = vol;
    if (*chip).model == PMAC_SCREAMER {
        /* FIXME: screamer has loopthru vol control */
        (*chip).awacs_reg[5] = vol;
        /* FIXME: maybe should be vol << 3 for PCMCIA speaker */
        (*chip).awacs_reg[6] = MASK_MIC_BOOST;
        (*chip).awacs_reg[7] = 0;
    }

    awacs_restore_all_regs(chip);
    (*chip).manufacturer = ((in_le32(&mut (*(*chip).awacs).codec_stat) >> 8) & 0xf) as _;
    screamer_recalibrate(chip);

    (*chip).revision = ((in_le32(&mut (*(*chip).awacs).codec_stat) >> 12) & 0xf) as _;
    #[cfg(PMAC_AMP_AVAIL)]
    if (*chip).revision == 3 && (*chip).has_iic != 0 && CHECK_CUDA_AMP() {
        let amp: *mut awacs_amp = kzalloc_obj();
        if amp.is_null() {
            return -ENOMEM;
        }
        (*chip).mixer_data = amp as *mut _;
        (*chip).mixer_free = Some(awacs_amp_free);
        /* mute and zero vol */
        awacs_amp_set_vol(amp, 0, 63, 63, 0);
        awacs_amp_set_vol(amp, 1, 63, 63, 0);
        awacs_amp_set_tone(amp, 7, 7); /* 0 dB */
        awacs_amp_set_master(amp, 79); /* 0 dB */
    }

    if (*chip).hp_stat_mask == 0 {
        /* set headphone-jack detection bit */
        match (*chip).model {
            PMAC_AWACS => {
                (*chip).hp_stat_mask = if pm7500 != 0 || pm5500 != 0 { MASK_HDPCONN } else { MASK_LOCONN };
            }
            PMAC_SCREAMER => {
                match (*chip).device_id {
                    0x08 | 0x0B => {
                        (*chip).hp_stat_mask = if imac {
                            MASK_LOCONN_IMAC | MASK_HDPLCONN_IMAC | MASK_HDPRCONN_IMAC
                        } else {
                            MASK_HDPCONN
                        };
                    }
                    0x00 | 0x05 => {
                        (*chip).hp_stat_mask = MASK_LOCONN;
                    }
                    _ => {
                        (*chip).hp_stat_mask = MASK_HDPCONN;
                    }
                }
            }
            _ => {
                snd_BUG();
            }
        }
    }

    /*
     * build mixers
     */
    strscpy((*(*chip).card).mixername.as_mut_ptr(), c"PowerMac AWACS".as_ptr());

    err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mixers) as _, snd_pmac_awacs_mixers.as_ptr());
    if err < 0 {
        return err;
    }
    if beige != 0 || g4agp != 0 {
        ;
    } else if (*chip).model == PMAC_SCREAMER || pm5500 != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mixers2) as _, snd_pmac_screamer_mixers2.as_ptr());
    } else if pm7500 == 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mixers2) as _, snd_pmac_awacs_mixers2.as_ptr());
    }
    if err < 0 {
        return err;
    }
    if pm5500 != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mixers2_pmac5500) as _, snd_pmac_awacs_mixers2_pmac5500.as_ptr());
        if err < 0 {
            return err;
        }
    }
    master_vol = core::ptr::null_mut();
    speaker_vol = core::ptr::null_mut();
    if pm7500 != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mixers_pmac7500) as _, snd_pmac_awacs_mixers_pmac7500.as_ptr());
    } else if pm5500 != 0 {
        master_vol = snd_ctl_new1(snd_pmac_awacs_mixers_pmac5500.as_ptr(), chip as *mut _);
        err = snd_ctl_add((*chip).card, master_vol);
    } else if beige != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mixers_beige) as _, snd_pmac_screamer_mixers_beige.as_ptr());
    } else if imac || lombard != 0 {
        master_vol = snd_ctl_new1(snd_pmac_screamer_mixers_lo.as_ptr(), chip as *mut _);
        err = snd_ctl_add((*chip).card, master_vol);
        if err < 0 {
            return err;
        }
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mixers_imac) as _, snd_pmac_screamer_mixers_imac.as_ptr());
    } else if g4agp != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mixers_g4agp) as _, snd_pmac_screamer_mixers_g4agp.as_ptr());
    } else {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mixers_pmac) as _, snd_pmac_awacs_mixers_pmac.as_ptr());
    }
    if err < 0 {
        return err;
    }
    (*chip).master_sw_ctl = snd_ctl_new1(
        if pm7500 != 0 || imac || g4agp != 0 || lombard != 0 {
            &snd_pmac_awacs_master_sw_imac
        } else if pm5500 != 0 {
            &snd_pmac_awacs_master_sw_pmac5500
        } else {
            &snd_pmac_awacs_master_sw
        },
        chip as *mut _,
    );
    err = snd_ctl_add((*chip).card, (*chip).master_sw_ctl);
    if err < 0 {
        return err;
    }
    #[cfg(PMAC_AMP_AVAIL)]
    if !(*chip).mixer_data.is_null() {
        /* use amplifier.  the signal is connected from route A
         * to the amp.  the amp has its headphone and speaker
         * volumes and mute switches, so we use them instead of
         * screamer registers.
         * in this case, it seems the route C is not used.
         */
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_amp_vol) as _, snd_pmac_awacs_amp_vol.as_ptr());
        if err < 0 {
            return err;
        }
        /* overwrite */
        (*chip).master_sw_ctl = snd_ctl_new1(&snd_pmac_awacs_amp_hp_sw, chip as *mut _);
        err = snd_ctl_add((*chip).card, (*chip).master_sw_ctl);
        if err < 0 {
            return err;
        }
        (*chip).speaker_sw_ctl = snd_ctl_new1(&snd_pmac_awacs_amp_spk_sw, chip as *mut _);
        err = snd_ctl_add((*chip).card, (*chip).speaker_sw_ctl);
        if err < 0 {
            return err;
        }
    } else
    {
        /* route A = headphone, route C = speaker */
        speaker_vol = snd_ctl_new1(snd_pmac_awacs_speaker_vol.as_ptr(), chip as *mut _);
        err = snd_ctl_add((*chip).card, speaker_vol);
        if err < 0 {
            return err;
        }
        (*chip).speaker_sw_ctl = snd_ctl_new1(
            if imac1 != 0 {
                &snd_pmac_awacs_speaker_sw_imac1
            } else if imac2 != 0 {
                &snd_pmac_awacs_speaker_sw_imac2
            } else {
                &snd_pmac_awacs_speaker_sw
            },
            chip as *mut _,
        );
        err = snd_ctl_add((*chip).card, (*chip).speaker_sw_ctl);
        if err < 0 {
            return err;
        }
    }

    if pm5500 != 0 || imac || lombard != 0 {
        vmaster_sw = snd_ctl_make_virtual_master(c"Master Playback Switch".as_ptr(), core::ptr::null_mut());
        err = snd_ctl_add_follower_uncached(vmaster_sw, (*chip).master_sw_ctl);
        if err < 0 {
            return err;
        }
        err = snd_ctl_add_follower_uncached(vmaster_sw, (*chip).speaker_sw_ctl);
        if err < 0 {
            return err;
        }
        err = snd_ctl_add((*chip).card, vmaster_sw);
        if err < 0 {
            return err;
        }
        vmaster_vol = snd_ctl_make_virtual_master(c"Master Playback Volume".as_ptr(), core::ptr::null_mut());
        err = snd_ctl_add_follower(vmaster_vol, master_vol);
        if err < 0 {
            return err;
        }
        err = snd_ctl_add_follower(vmaster_vol, speaker_vol);
        if err < 0 {
            return err;
        }
        err = snd_ctl_add((*chip).card, vmaster_vol);
        if err < 0 {
            return err;
        }
    }

    if beige != 0 || g4agp != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mic_boost_beige) as _, snd_pmac_screamer_mic_boost_beige.as_ptr());
    } else if imac {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mic_boost_imac) as _, snd_pmac_screamer_mic_boost_imac.as_ptr());
    } else if (*chip).model == PMAC_SCREAMER {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_screamer_mic_boost) as _, snd_pmac_screamer_mic_boost.as_ptr());
    } else if pm7500 != 0 {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mic_boost_pmac7500) as _, snd_pmac_awacs_mic_boost_pmac7500.as_ptr());
    } else {
        err = build_mixers(chip, ARRAY_SIZE(&snd_pmac_awacs_mic_boost) as _, snd_pmac_awacs_mic_boost.as_ptr());
    }
    if err < 0 {
        return err;
    }

    /*
     * set lowlevel callbacks
     */
    (*chip).set_format = Some(snd_pmac_awacs_set_format);
    #[cfg(CONFIG_PM)]
    {
        (*chip).suspend = Some(snd_pmac_awacs_suspend);
        (*chip).resume = Some(snd_pmac_awacs_resume);
    }
    #[cfg(PMAC_SUPPORT_AUTOMUTE)]
    {
        err = snd_pmac_add_automute(chip);
        if err < 0 {
            return err;
        }
        (*chip).detect_headphone = Some(snd_pmac_awacs_detect_headphone);
        (*chip).update_automute = Some(snd_pmac_awacs_update_automute);
        snd_pmac_awacs_update_automute(chip, 0); /* update the status only */
    }
    if (*chip).model == PMAC_SCREAMER {
        snd_pmac_awacs_write_noreg(chip, 6, (*chip).awacs_reg[6]);
        snd_pmac_awacs_write_noreg(chip, 0, (*chip).awacs_reg[0]);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
