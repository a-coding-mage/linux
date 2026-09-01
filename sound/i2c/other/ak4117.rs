// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for control of the AK4117 via 4-wire serial interface
 *  IEC958 (S/PDIF) receiver by Asahi Kasei
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies: linux/slab.h, linux/delay.h, linux/module.h,
// sound/core.h, sound/control.h, sound/pcm.h, sound/ak4117.h,
// sound/asoundef.h.

pub const AK4117_ADDR: u8 = 0x00; /* fixed address */

extern "C" {
    fn timer_shutdown_sync(timer: *mut timer_list);
    fn timer_delete(timer: *mut timer_list);
    fn timer_setup(timer: *mut timer_list, func: Option<unsafe extern "C" fn(*mut timer_list)>, flags: u32);
    fn mod_timer(timer: *mut timer_list, expires: c_ulong) -> c_int;
    fn udelay(usecs: c_ulong);
    fn kfree(ptr: *mut c_void);
    fn snd_device_new(
        card: *mut snd_card,
        ty: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_pcm_stream_lock_irqsave(substream: *mut snd_pcm_substream, flags: c_ulong);
    fn snd_pcm_stream_unlock_irqrestore(substream: *mut snd_pcm_substream, flags: c_ulong);
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int) -> c_int;
    fn wake_up(wait: *mut wait_queue_head_t);
    fn snd_BUG_ON(condition: bool) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;

    static mut jiffies: c_ulong;
}

unsafe extern "C" fn reg_write(ak4117: *mut ak4117, reg: c_uchar, val: c_uchar) {
    ((*ak4117).write).unwrap()((*ak4117).private_data, reg, val);
    if (reg as usize) < (*ak4117).regmap.len() {
        (*ak4117).regmap[reg as usize] = val;
    }
}

#[inline]
unsafe fn reg_read(ak4117: *mut ak4117, reg: c_uchar) -> c_uchar {
    ((*ak4117).read).unwrap()((*ak4117).private_data, reg)
}

unsafe extern "C" fn snd_ak4117_free(chip: *mut ak4117) {
    timer_shutdown_sync(&mut (*chip).timer);
    kfree(chip as *mut c_void);
}

unsafe extern "C" fn snd_ak4117_dev_free(device: *mut snd_device) -> c_int {
    let chip: *mut ak4117 = (*device).device_data as *mut ak4117;
    snd_ak4117_free(chip);
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4117_create(
    card: *mut snd_card,
    read: ak4117_read_t,
    write: ak4117_write_t,
    pgm: *const c_uchar,
    private_data: *mut c_void,
    r_ak4117: *mut *mut ak4117,
) -> c_int {
    let mut err: c_int = 0;
    let mut reg: c_uchar;
    static OPS: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_ak4117_dev_free),
    };

    let chip = kzalloc(core::mem::size_of::<ak4117>(), GFP_KERNEL) as *mut ak4117;
    if chip.is_null() {
        return -ENOMEM;
    }
    spin_lock_init(&mut (*chip).lock);
    (*chip).card = card;
    (*chip).read = read;
    (*chip).write = write;
    (*chip).private_data = private_data;
    timer_setup(&mut (*chip).timer, Some(snd_ak4117_timer), 0);

    reg = 0;
    while reg < 5 {
        (*chip).regmap[reg as usize] = *pgm.add(reg as usize);
        reg = reg.wrapping_add(1);
    }
    snd_ak4117_reinit(chip);

    (*chip).rcs0 = reg_read(chip, AK4117_REG_RCS0) & !(AK4117_QINT | AK4117_CINT | AK4117_STC);
    (*chip).rcs1 = reg_read(chip, AK4117_REG_RCS1);
    (*chip).rcs2 = reg_read(chip, AK4117_REG_RCS2);

    err = snd_device_new(card, SNDRV_DEV_CODEC, chip as *mut c_void, &OPS);
    if err < 0 {
        snd_ak4117_free(chip);
        return err;
    }

    if !r_ak4117.is_null() {
        *r_ak4117 = chip;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4117_reg_write(
    chip: *mut ak4117,
    reg: c_uchar,
    mask: c_uchar,
    val: c_uchar,
) {
    if reg >= 5 {
        return;
    }
    reg_write(chip, reg, ((*chip).regmap[reg as usize] & !mask) | val);
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4117_reinit(chip: *mut ak4117) {
    let old: c_uchar = (*chip).regmap[AK4117_REG_PWRDN as usize];
    let mut reg: c_uchar;

    timer_delete(&mut (*chip).timer);
    (*chip).init = 1;
    /* bring the chip to reset state and powerdown state */
    reg_write(chip, AK4117_REG_PWRDN, 0);
    udelay(200);
    /* release reset, but leave powerdown */
    reg_write(chip, AK4117_REG_PWRDN, (old | AK4117_RST) & !AK4117_PWN);
    udelay(200);
    reg = 1;
    while reg < 5 {
        reg_write(chip, reg, (*chip).regmap[reg as usize]);
        reg = reg.wrapping_add(1);
    }
    /* release powerdown, everything is initialized now */
    reg_write(chip, AK4117_REG_PWRDN, old | AK4117_RST | AK4117_PWN);
    (*chip).init = 0;
    mod_timer(&mut (*chip).timer, 1 + jiffies);
}

unsafe fn external_rate(rcs1: c_uchar) -> c_uint {
    match rcs1 & (AK4117_FS0 | AK4117_FS1 | AK4117_FS2 | AK4117_FS3) {
        AK4117_FS_32000HZ => 32000,
        AK4117_FS_44100HZ => 44100,
        AK4117_FS_48000HZ => 48000,
        AK4117_FS_88200HZ => 88200,
        AK4117_FS_96000HZ => 96000,
        AK4117_FS_176400HZ => 176400,
        AK4117_FS_192000HZ => 192000,
        _ => 0,
    }
}

unsafe extern "C" fn snd_ak4117_in_error_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = LONG_MAX;
    0
}

unsafe extern "C" fn snd_ak4117_in_error_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;

    spin_lock_irq(&mut (*chip).lock);
    (*ucontrol).value.integer.value[0] = (*chip).errors[(*kcontrol).private_value as usize] as c_long;
    (*chip).errors[(*kcontrol).private_value as usize] = 0;
    spin_unlock_irq(&mut (*chip).lock);
    0
}

unsafe extern "C" fn snd_ak4117_in_bit_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;
    let reg: c_uchar = ((*kcontrol).private_value & 0xff) as c_uchar;
    let bit: c_uchar = (((*kcontrol).private_value >> 8) & 0xff) as c_uchar;
    let inv: c_uchar = (((*kcontrol).private_value >> 31) & 1) as c_uchar;

    (*ucontrol).value.integer.value[0] =
        ((((reg_read(chip, reg) & ((1 as c_uchar) << bit)) != 0) as c_uchar) ^ inv) as c_long;
    0
}

unsafe extern "C" fn snd_ak4117_rx_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn snd_ak4117_rx_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;

    (*ucontrol).value.integer.value[0] =
        if ((*chip).regmap[AK4117_REG_IO as usize] & AK4117_IPS) != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn snd_ak4117_rx_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;
    let change: c_int;
    let old_val: u8;

    spin_lock_irq(&mut (*chip).lock);
    old_val = (*chip).regmap[AK4117_REG_IO as usize];
    change = (((*ucontrol).value.integer.value[0] != 0) != ((old_val & AK4117_IPS) != 0)) as c_int;
    if change != 0 {
        reg_write(
            chip,
            AK4117_REG_IO,
            (old_val & !AK4117_IPS)
                | if (*ucontrol).value.integer.value[0] != 0 { AK4117_IPS } else { 0 },
        );
    }
    spin_unlock_irq(&mut (*chip).lock);
    change
}

unsafe extern "C" fn snd_ak4117_rate_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 192000;
    0
}

unsafe extern "C" fn snd_ak4117_rate_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;

    (*ucontrol).value.integer.value[0] = external_rate(reg_read(chip, AK4117_REG_RCS1)) as c_long;
    0
}

unsafe extern "C" fn snd_ak4117_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_ak4117_spdif_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;
    let mut i: c_uint = 0;

    while i < AK4117_REG_RXCSB_SIZE {
        (*ucontrol).value.iec958.status[i as usize] =
            reg_read(chip, AK4117_REG_RXCSB0 + i as c_uchar);
        i += 1;
    }
    0
}

unsafe extern "C" fn snd_ak4117_spdif_mask_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn snd_ak4117_spdif_mask_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    core::ptr::write_bytes(
        (*ucontrol).value.iec958.status.as_mut_ptr(),
        0xff,
        AK4117_REG_RXCSB_SIZE as usize,
    );
    0
}

unsafe extern "C" fn snd_ak4117_spdif_pinfo(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 0xffff;
    (*uinfo).count = 4;
    0
}

unsafe extern "C" fn snd_ak4117_spdif_pget(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;
    let mut tmp: c_ushort;

    (*ucontrol).value.integer.value[0] = 0xf8f2;
    (*ucontrol).value.integer.value[1] = 0x4e1f;
    tmp = (reg_read(chip, AK4117_REG_PC0) as c_ushort)
        | ((reg_read(chip, AK4117_REG_PC1) as c_ushort) << 8);
    (*ucontrol).value.integer.value[2] = tmp as c_long;
    tmp = (reg_read(chip, AK4117_REG_PD0) as c_ushort)
        | ((reg_read(chip, AK4117_REG_PD1) as c_ushort) << 8);
    (*ucontrol).value.integer.value[3] = tmp as c_long;
    0
}

unsafe extern "C" fn snd_ak4117_spdif_qinfo(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = AK4117_REG_QSUB_SIZE;
    0
}

unsafe extern "C" fn snd_ak4117_spdif_qget(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut ak4117 = snd_kcontrol_chip(kcontrol) as *mut ak4117;
    let mut i: c_uint = 0;

    while i < AK4117_REG_QSUB_SIZE {
        (*ucontrol).value.bytes.data[i as usize] =
            reg_read(chip, AK4117_REG_QSUB_ADDR + i as c_uchar);
        i += 1;
    }
    0
}

/* Don't forget to change AK4117_CONTROLS define!!! */
static SND_AK4117_IEC958_CONTROLS: [snd_kcontrol_new; AK4117_CONTROLS as usize] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Parity Errors\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_in_error_info),
        get: Some(snd_ak4117_in_error_get),
        put: None,
        private_value: AK4117_PARITY_ERRORS as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 V-Bit Errors\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_in_error_info),
        get: Some(snd_ak4117_in_error_get),
        put: None,
        private_value: AK4117_V_BIT_ERRORS as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 C-CRC Errors\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_in_error_info),
        get: Some(snd_ak4117_in_error_get),
        put: None,
        private_value: AK4117_CCRC_ERRORS as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Q-CRC Errors\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_in_error_info),
        get: Some(snd_ak4117_in_error_get),
        put: None,
        private_value: AK4117_QCRC_ERRORS as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 External Rate\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_rate_info),
        get: Some(snd_ak4117_rate_get),
        put: None,
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_CAPTURE_MASK,
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(snd_ak4117_spdif_mask_info),
        get: Some(snd_ak4117_spdif_mask_get),
        put: None,
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958_CAPTURE_DEFAULT,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_spdif_info),
        get: Some(snd_ak4117_spdif_get),
        put: None,
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Preamble Capture Default\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_spdif_pinfo),
        get: Some(snd_ak4117_spdif_pget),
        put: None,
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Q-subcode Capture Default\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4117_spdif_qinfo),
        get: Some(snd_ak4117_spdif_qget),
        put: None,
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Audio\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(snd_ak4117_in_bit_get),
        put: None,
        private_value: ((1u32 << 31) | (3 << 8) | AK4117_REG_RCS0 as u32) as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 Non-PCM Bitstream\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(snd_ak4117_in_bit_get),
        put: None,
        private_value: ((5 << 8) | AK4117_REG_RCS1 as u32) as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"IEC958 DTS Bitstream\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(snd_ak4117_in_bit_get),
        put: None,
        private_value: ((6 << 8) | AK4117_REG_RCS1 as u32) as c_ulong,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: b"AK4117 Input Select\0".as_ptr() as *const c_char,
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE,
        info: Some(snd_ak4117_rx_info),
        get: Some(snd_ak4117_rx_get),
        put: Some(snd_ak4117_rx_put),
        private_value: 0,
    },
];

#[no_mangle]
pub unsafe extern "C" fn snd_ak4117_build(
    ak4117: *mut ak4117,
    cap_substream: *mut snd_pcm_substream,
) -> c_int {
    let mut kctl: *mut snd_kcontrol;
    let mut idx: c_uint;
    let mut err: c_int;

    if snd_BUG_ON(cap_substream.is_null()) != 0 {
        return -EINVAL;
    }
    (*ak4117).substream = cap_substream;
    idx = 0;
    while idx < AK4117_CONTROLS {
        kctl = snd_ctl_new1(
            &SND_AK4117_IEC958_CONTROLS[idx as usize],
            ak4117 as *mut c_void,
        );
        if kctl.is_null() {
            return -ENOMEM;
        }
        (*kctl).id.device = (*(*cap_substream).pcm).device;
        (*kctl).id.subdevice = (*cap_substream).number;
        err = snd_ctl_add((*ak4117).card, kctl);
        if err < 0 {
            return err;
        }
        (*ak4117).kctls[idx as usize] = kctl;
        idx += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4117_external_rate(ak4117: *mut ak4117) -> c_int {
    let rcs1: c_uchar;

    rcs1 = reg_read(ak4117, AK4117_REG_RCS1);
    external_rate(rcs1) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4117_check_rate_and_errors(
    ak4117: *mut ak4117,
    flags: c_uint,
) -> c_int {
    let runtime: *mut snd_pcm_runtime = if !(*ak4117).substream.is_null() {
        (*(*ak4117).substream).runtime
    } else {
        core::ptr::null_mut()
    };
    let mut _flags: c_ulong = 0;
    let mut res: c_int = 0;
    let mut rcs0: c_uchar = 0;
    let rcs1: c_uchar;
    let mut rcs2: c_uchar = 0;
    let mut c0: c_uchar = 0;
    let mut c1: c_uchar = 0;

    rcs1 = reg_read(ak4117, AK4117_REG_RCS1);
    if (flags & AK4117_CHECK_NO_STAT) == 0 {
        rcs0 = reg_read(ak4117, AK4117_REG_RCS0);
        rcs2 = reg_read(ak4117, AK4117_REG_RCS2);
        spin_lock_irqsave(&mut (*ak4117).lock, &mut _flags);
        if (rcs0 & AK4117_PAR) != 0 {
            (*ak4117).errors[AK4117_PARITY_ERRORS as usize] =
                (*ak4117).errors[AK4117_PARITY_ERRORS as usize].wrapping_add(1);
        }
        if (rcs0 & AK4117_V) != 0 {
            (*ak4117).errors[AK4117_V_BIT_ERRORS as usize] =
                (*ak4117).errors[AK4117_V_BIT_ERRORS as usize].wrapping_add(1);
        }
        if (rcs2 & AK4117_CCRC) != 0 {
            (*ak4117).errors[AK4117_CCRC_ERRORS as usize] =
                (*ak4117).errors[AK4117_CCRC_ERRORS as usize].wrapping_add(1);
        }
        if (rcs2 & AK4117_QCRC) != 0 {
            (*ak4117).errors[AK4117_QCRC_ERRORS as usize] =
                (*ak4117).errors[AK4117_QCRC_ERRORS as usize].wrapping_add(1);
        }
        c0 = ((*ak4117).rcs0
            & (AK4117_QINT | AK4117_CINT | AK4117_STC | AK4117_AUDION | AK4117_AUTO | AK4117_UNLCK))
            ^ (rcs0
                & (AK4117_QINT | AK4117_CINT | AK4117_STC | AK4117_AUDION | AK4117_AUTO | AK4117_UNLCK));
        c1 = ((*ak4117).rcs1 & (AK4117_DTSCD | AK4117_NPCM | AK4117_PEM | 0x0f))
            ^ (rcs1 & (AK4117_DTSCD | AK4117_NPCM | AK4117_PEM | 0x0f));
        (*ak4117).rcs0 = rcs0 & !(AK4117_QINT | AK4117_CINT | AK4117_STC);
        (*ak4117).rcs1 = rcs1;
        (*ak4117).rcs2 = rcs2;
        spin_unlock_irqrestore(&mut (*ak4117).lock, _flags);

        if (rcs0 & AK4117_PAR) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[0]).id);
        }
        if (rcs0 & AK4117_V) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[1]).id);
        }
        if (rcs2 & AK4117_CCRC) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[2]).id);
        }
        if (rcs2 & AK4117_QCRC) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[3]).id);
        }

        /* rate change */
        if (c1 & 0x0f) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[4]).id);
        }

        if ((c1 & AK4117_PEM) | (c0 & AK4117_CINT)) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[6]).id);
        }
        if (c0 & AK4117_QINT) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[8]).id);
        }

        if (c0 & AK4117_AUDION) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[9]).id);
        }
        if (c1 & AK4117_NPCM) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[10]).id);
        }
        if (c1 & AK4117_DTSCD) != 0 {
            snd_ctl_notify((*ak4117).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4117).kctls[11]).id);
        }

        if (*ak4117).change_callback.is_some() && (c0 | c1) != 0 {
            ((*ak4117).change_callback).unwrap()(ak4117, c0, c1);
        }
    }

    /* compare rate */
    res = external_rate(rcs1) as c_int;
    if (flags & AK4117_CHECK_NO_RATE) == 0 && !runtime.is_null() && (*runtime).rate != res {
        snd_pcm_stream_lock_irqsave((*ak4117).substream, _flags);
        if snd_pcm_running((*ak4117).substream) != 0 {
            snd_pcm_stop((*ak4117).substream, SNDRV_PCM_STATE_DRAINING);
            wake_up(&mut (*runtime).sleep);
            res = 1;
        }
        snd_pcm_stream_unlock_irqrestore((*ak4117).substream, _flags);
    }
    res
}

unsafe extern "C" fn snd_ak4117_timer(t: *mut timer_list) {
    let chip: *mut ak4117 = timer_container_of_ak4117(t);

    if (*chip).init != 0 {
        return;
    }
    snd_ak4117_check_rate_and_errors(chip, 0);
    mod_timer(&mut (*chip).timer, 1 + jiffies);
}

// EXPORT_SYMBOL(snd_ak4117_create);
// EXPORT_SYMBOL(snd_ak4117_reg_write);
// EXPORT_SYMBOL(snd_ak4117_reinit);
// EXPORT_SYMBOL(snd_ak4117_build);
// EXPORT_SYMBOL(snd_ak4117_external_rate);
// EXPORT_SYMBOL(snd_ak4117_check_rate_and_errors);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
