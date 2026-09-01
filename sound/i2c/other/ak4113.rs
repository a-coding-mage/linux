// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for control of the AK4113 via I2C/4-wire serial interface
 *  IEC958 (S/PDIF) receiver by Asahi Kasei
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Copyright (c) by Pavel Hofman <pavel.hofman@ivitera.com>
 */

// C dependencies: linux/slab.h, linux/delay.h, linux/module.h,
// sound/core.h, sound/control.h, sound/pcm.h, sound/ak4113.h,
// sound/asoundef.h, sound/info.h.

const AK4113_ADDR: u8 = 0x00; /* fixed address */

unsafe fn ak4113_stats(work: *mut work_struct);
unsafe fn ak4113_init_regs(chip: *mut ak4113);

unsafe fn reg_write(ak4113: *mut ak4113, reg: u8, val: u8) {
    unsafe {
        ((*ak4113).write).expect("non-null function pointer")((*ak4113).private_data, reg, val);
        if (reg as usize) < core::mem::size_of_val(&(*ak4113).regmap) {
            (*ak4113).regmap[reg as usize] = val;
        }
    }
}

#[inline]
unsafe fn reg_read(ak4113: *mut ak4113, reg: u8) -> u8 {
    unsafe { ((*ak4113).read).expect("non-null function pointer")((*ak4113).private_data, reg) }
}

unsafe fn snd_ak4113_free(chip: *mut ak4113) {
    unsafe {
        atomic_inc(&mut (*chip).wq_processing); /* don't schedule new work */
        cancel_delayed_work_sync(&mut (*chip).work);
        kfree(chip as *mut core::ffi::c_void);
    }
}

unsafe extern "C" fn snd_ak4113_dev_free(device: *mut snd_device) -> i32 {
    unsafe {
        let chip: *mut ak4113 = (*device).device_data as *mut ak4113;
        snd_ak4113_free(chip);
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_create(
    card: *mut snd_card,
    read: ak4113_read_t,
    write: ak4113_write_t,
    pgm: *const u8,
    private_data: *mut core::ffi::c_void,
    r_ak4113: *mut *mut ak4113,
) -> i32 {
    unsafe {
        let mut chip: *mut ak4113;
        let mut err: i32;
        let mut reg: u8;
        static OPS: snd_device_ops = snd_device_ops {
            dev_free: Some(snd_ak4113_dev_free),
        };

        chip = kzalloc_obj::<ak4113>();
        if chip.is_null() {
            return -ENOMEM;
        }
        spin_lock_init(&mut (*chip).lock);
        (*chip).card = card;
        (*chip).read = read;
        (*chip).write = write;
        (*chip).private_data = private_data;
        INIT_DELAYED_WORK(&mut (*chip).work, Some(ak4113_stats));
        atomic_set(&mut (*chip).wq_processing, 0);
        mutex_init(&mut (*chip).reinit_mutex);

        reg = 0;
        while reg < AK4113_WRITABLE_REGS {
            (*chip).regmap[reg as usize] = *pgm.add(reg as usize);
            reg = reg.wrapping_add(1);
        }
        ak4113_init_regs(chip);

        (*chip).rcs0 = reg_read(chip, AK4113_REG_RCS0) & !(AK4113_QINT | AK4113_CINT | AK4113_STC);
        (*chip).rcs1 = reg_read(chip, AK4113_REG_RCS1);
        (*chip).rcs2 = reg_read(chip, AK4113_REG_RCS2);
        err = snd_device_new(card, SNDRV_DEV_CODEC, chip as *mut core::ffi::c_void, &OPS);
        if err < 0 {
            snd_ak4113_free(chip);
            return err;
        }

        if !r_ak4113.is_null() {
            *r_ak4113 = chip;
        }
        0
    }
}
// EXPORT_SYMBOL_GPL(snd_ak4113_create);

#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_reg_write(chip: *mut ak4113, reg: u8, mask: u8, val: u8) {
    unsafe {
        if reg >= AK4113_WRITABLE_REGS {
            return;
        }
        reg_write(chip, reg, ((*chip).regmap[reg as usize] & !mask) | val);
    }
}
// EXPORT_SYMBOL_GPL(snd_ak4113_reg_write);

unsafe fn ak4113_init_regs(chip: *mut ak4113) {
    unsafe {
        let old: u8 = (*chip).regmap[AK4113_REG_PWRDN as usize];
        let mut reg: u8;

        /* bring the chip to reset state and powerdown state */
        reg_write(chip, AK4113_REG_PWRDN, old & !(AK4113_RST | AK4113_PWN));
        udelay(200);
        /* release reset, but leave powerdown */
        reg_write(chip, AK4113_REG_PWRDN, (old | AK4113_RST) & !AK4113_PWN);
        udelay(200);
        reg = 1;
        while reg < AK4113_WRITABLE_REGS {
            reg_write(chip, reg, (*chip).regmap[reg as usize]);
            reg = reg.wrapping_add(1);
        }
        /* release powerdown, everything is initialized now */
        reg_write(chip, AK4113_REG_PWRDN, old | AK4113_RST | AK4113_PWN);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_reinit(chip: *mut ak4113) {
    unsafe {
        if atomic_inc_return(&mut (*chip).wq_processing) == 1 {
            cancel_delayed_work_sync(&mut (*chip).work);
        }
        mutex_lock(&mut (*chip).reinit_mutex);
        ak4113_init_regs(chip);
        mutex_unlock(&mut (*chip).reinit_mutex);
        /* bring up statistics / event queing */
        if atomic_dec_and_test(&mut (*chip).wq_processing) {
            schedule_delayed_work(&mut (*chip).work, HZ / 10);
        }
    }
}
// EXPORT_SYMBOL_GPL(snd_ak4113_reinit);

fn external_rate(rcs1: u8) -> u32 {
    match rcs1 & (AK4113_FS0 | AK4113_FS1 | AK4113_FS2 | AK4113_FS3) {
        AK4113_FS_8000HZ => 8000,
        AK4113_FS_11025HZ => 11025,
        AK4113_FS_16000HZ => 16000,
        AK4113_FS_22050HZ => 22050,
        AK4113_FS_24000HZ => 24000,
        AK4113_FS_32000HZ => 32000,
        AK4113_FS_44100HZ => 44100,
        AK4113_FS_48000HZ => 48000,
        AK4113_FS_64000HZ => 64000,
        AK4113_FS_88200HZ => 88200,
        AK4113_FS_96000HZ => 96000,
        AK4113_FS_176400HZ => 176400,
        AK4113_FS_192000HZ => 192000,
        _ => 0,
    }
}

unsafe extern "C" fn snd_ak4113_in_error_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = LONG_MAX;
        0
    }
}

unsafe extern "C" fn snd_ak4113_in_error_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;

        spin_lock_irq(&mut (*chip).lock);
        (*ucontrol).value.integer.value[0] =
            (*chip).errors[(*kcontrol).private_value as usize] as _;
        (*chip).errors[(*kcontrol).private_value as usize] = 0;
        spin_unlock_irq(&mut (*chip).lock);
        0
    }
}

// #define snd_ak4113_in_bit_info snd_ctl_boolean_mono_info

unsafe extern "C" fn snd_ak4113_in_bit_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;
        let reg: u8 = ((*kcontrol).private_value & 0xff) as u8;
        let bit: u8 = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let inv: u8 = (((*kcontrol).private_value >> 31) & 1) as u8;

        (*ucontrol).value.integer.value[0] =
            (((if (reg_read(chip, reg) & (1u8 << bit)) != 0 { 1 } else { 0 }) ^ inv) as _);
        0
    }
}

unsafe extern "C" fn snd_ak4113_rx_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 5;
        0
    }
}

unsafe extern "C" fn snd_ak4113_rx_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;

        (*ucontrol).value.integer.value[0] = AK4113_IPS((*chip).regmap[AK4113_REG_IO1 as usize]) as _;
        0
    }
}

unsafe extern "C" fn snd_ak4113_rx_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;
        let change: i32;
        let old_val: u8;

        spin_lock_irq(&mut (*chip).lock);
        old_val = (*chip).regmap[AK4113_REG_IO1 as usize];
        change = ((*ucontrol).value.integer.value[0] != AK4113_IPS(old_val) as _) as i32;
        if change != 0 {
            reg_write(
                chip,
                AK4113_REG_IO1,
                (old_val & !AK4113_IPS(0xff)) | AK4113_IPS((*ucontrol).value.integer.value[0] as u8),
            );
        }
        spin_unlock_irq(&mut (*chip).lock);
        change
    }
}

unsafe extern "C" fn snd_ak4113_rate_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 192000;
        0
    }
}

unsafe extern "C" fn snd_ak4113_rate_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;

        (*ucontrol).value.integer.value[0] = external_rate(reg_read(chip, AK4113_REG_RCS1)) as _;
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;
        let mut i: u32 = 0;

        while i < AK4113_REG_RXCSB_SIZE {
            (*ucontrol).value.iec958.status[i as usize] =
                reg_read(chip, AK4113_REG_RXCSB0 + i as u8);
            i += 1;
        }
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_mask_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
        (*uinfo).count = 1;
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_mask_get(
    _kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        memset(
            (*ucontrol).value.iec958.status.as_mut_ptr() as *mut core::ffi::c_void,
            0xff,
            AK4113_REG_RXCSB_SIZE as usize,
        );
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_pinfo(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 0xffff;
        (*uinfo).count = 4;
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_pget(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;
        let mut tmp: u16;

        (*ucontrol).value.integer.value[0] = 0xf8f2;
        (*ucontrol).value.integer.value[1] = 0x4e1f;
        tmp = (reg_read(chip, AK4113_REG_Pc0) as u16)
            | ((reg_read(chip, AK4113_REG_Pc1) as u16) << 8);
        (*ucontrol).value.integer.value[2] = tmp as _;
        tmp = (reg_read(chip, AK4113_REG_Pd0) as u16)
            | ((reg_read(chip, AK4113_REG_Pd1) as u16) << 8);
        (*ucontrol).value.integer.value[3] = tmp as _;
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_qinfo(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> i32 {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
        (*uinfo).count = AK4113_REG_QSUB_SIZE;
        0
    }
}

unsafe extern "C" fn snd_ak4113_spdif_qget(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    unsafe {
        let chip: *mut ak4113 = snd_kcontrol_chip(kcontrol) as *mut ak4113;
        let mut i: u32 = 0;

        while i < AK4113_REG_QSUB_SIZE {
            (*ucontrol).value.bytes.data[i as usize] =
                reg_read(chip, AK4113_REG_QSUB_ADDR + i as u8);
            i += 1;
        }
        0
    }
}

/* Don't forget to change AK4113_CONTROLS define!!! */
static SND_AK4113_IEC958_CONTROLS: [snd_kcontrol_new; AK4113_CONTROLS as usize] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Parity Errors".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_in_error_info),
        get: Some(snd_ak4113_in_error_get),
        private_value: AK4113_PARITY_ERRORS as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 V-Bit Errors".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_in_error_info),
        get: Some(snd_ak4113_in_error_get),
        private_value: AK4113_V_BIT_ERRORS as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 C-CRC Errors".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_in_error_info),
        get: Some(snd_ak4113_in_error_get),
        private_value: AK4113_CCRC_ERRORS as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Q-CRC Errors".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_in_error_info),
        get: Some(snd_ak4113_in_error_get),
        private_value: AK4113_QCRC_ERRORS as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 External Rate".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_rate_info),
        get: Some(snd_ak4113_rate_get),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958(c"".as_ptr(), CAPTURE, MASK),
        access: SNDRV_CTL_ELEM_ACCESS_READ,
        info: Some(snd_ak4113_spdif_mask_info),
        get: Some(snd_ak4113_spdif_mask_get),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: SNDRV_CTL_NAME_IEC958(c"".as_ptr(), CAPTURE, DEFAULT),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_spdif_info),
        get: Some(snd_ak4113_spdif_get),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Preamble Capture Default".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_spdif_pinfo),
        get: Some(snd_ak4113_spdif_pget),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Q-subcode Capture Default".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ak4113_spdif_qinfo),
        get: Some(snd_ak4113_spdif_qget),
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Audio".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(snd_ak4113_in_bit_get),
        private_value: ((1 << 31) | (1 << 8) | AK4113_REG_RCS0 as i64) as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 Non-PCM Bitstream".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(snd_ak4113_in_bit_get),
        private_value: ((0 << 8) | AK4113_REG_RCS1 as i64) as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"IEC958 DTS Bitstream".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
        info: Some(snd_ctl_boolean_mono_info),
        get: Some(snd_ak4113_in_bit_get),
        private_value: ((1 << 8) | AK4113_REG_RCS1 as i64) as _,
        ..snd_kcontrol_new::zeroed()
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_PCM,
        name: c"AK4113 Input Select".as_ptr(),
        access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE,
        info: Some(snd_ak4113_rx_info),
        get: Some(snd_ak4113_rx_get),
        put: Some(snd_ak4113_rx_put),
        ..snd_kcontrol_new::zeroed()
    },
];

unsafe extern "C" fn snd_ak4113_proc_regs_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    unsafe {
        let ak4113: *mut ak4113 = (*entry).private_data as *mut ak4113;
        let mut reg: i32;
        let mut val: i32;
        /* all ak4113 registers 0x00 - 0x1c */
        reg = 0;
        while reg < 0x1d {
            val = reg_read(ak4113, reg as u8) as i32;
            snd_iprintf(buffer, c"0x%02x = 0x%02x\n".as_ptr(), reg, val);
            reg += 1;
        }
    }
}

unsafe fn snd_ak4113_proc_init(ak4113: *mut ak4113) {
    unsafe {
        snd_card_ro_proc_new(
            (*ak4113).card,
            c"ak4113".as_ptr(),
            ak4113 as *mut core::ffi::c_void,
            Some(snd_ak4113_proc_regs_read),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_build(
    ak4113: *mut ak4113,
    cap_substream: *mut snd_pcm_substream,
) -> i32 {
    unsafe {
        let mut kctl: *mut snd_kcontrol;
        let mut idx: u32;
        let mut err: i32;

        if snd_BUG_ON(cap_substream.is_null()) {
            return -EINVAL;
        }
        (*ak4113).substream = cap_substream;
        idx = 0;
        while idx < AK4113_CONTROLS {
            kctl = snd_ctl_new1(&SND_AK4113_IEC958_CONTROLS[idx as usize], ak4113 as *mut core::ffi::c_void);
            if kctl.is_null() {
                return -ENOMEM;
            }
            (*kctl).id.device = (*(*cap_substream).pcm).device;
            (*kctl).id.subdevice = (*cap_substream).number;
            err = snd_ctl_add((*ak4113).card, kctl);
            if err < 0 {
                return err;
            }
            (*ak4113).kctls[idx as usize] = kctl;
            idx += 1;
        }
        snd_ak4113_proc_init(ak4113);
        /* trigger workq */
        schedule_delayed_work(&mut (*ak4113).work, HZ / 10);
        0
    }
}
// EXPORT_SYMBOL_GPL(snd_ak4113_build);

#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_external_rate(ak4113: *mut ak4113) -> i32 {
    unsafe {
        let rcs1: u8;

        rcs1 = reg_read(ak4113, AK4113_REG_RCS1);
        external_rate(rcs1) as i32
    }
}
// EXPORT_SYMBOL_GPL(snd_ak4113_external_rate);

#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_check_rate_and_errors(
    ak4113: *mut ak4113,
    flags: u32,
) -> i32 {
    unsafe {
        let runtime: *mut snd_pcm_runtime = if !(*ak4113).substream.is_null() {
            (*(*ak4113).substream).runtime
        } else {
            core::ptr::null_mut()
        };
        let mut _flags: core::ffi::c_ulong = 0;
        let mut res: i32 = 0;
        let mut rcs0: u8 = 0;
        let rcs1: u8;
        let mut rcs2: u8 = 0;
        let mut c0: u8 = 0;
        let mut c1: u8 = 0;

        rcs1 = reg_read(ak4113, AK4113_REG_RCS1);
        if (flags & AK4113_CHECK_NO_STAT) == 0 {
            rcs0 = reg_read(ak4113, AK4113_REG_RCS0);
            rcs2 = reg_read(ak4113, AK4113_REG_RCS2);
            spin_lock_irqsave(&mut (*ak4113).lock, &mut _flags);
            if (rcs0 & AK4113_PAR) != 0 {
                (*ak4113).errors[AK4113_PARITY_ERRORS as usize] += 1;
            }
            if (rcs0 & AK4113_V) != 0 {
                (*ak4113).errors[AK4113_V_BIT_ERRORS as usize] += 1;
            }
            if (rcs2 & AK4113_CCRC) != 0 {
                (*ak4113).errors[AK4113_CCRC_ERRORS as usize] += 1;
            }
            if (rcs2 & AK4113_QCRC) != 0 {
                (*ak4113).errors[AK4113_QCRC_ERRORS as usize] += 1;
            }
            c0 = ((*ak4113).rcs0
                & (AK4113_QINT | AK4113_CINT | AK4113_STC | AK4113_AUDION | AK4113_AUTO | AK4113_UNLCK))
                ^ (rcs0
                    & (AK4113_QINT | AK4113_CINT | AK4113_STC | AK4113_AUDION | AK4113_AUTO | AK4113_UNLCK));
            c1 = ((*ak4113).rcs1 & (AK4113_DTSCD | AK4113_NPCM | AK4113_PEM | AK4113_DAT | 0xf0))
                ^ (rcs1 & (AK4113_DTSCD | AK4113_NPCM | AK4113_PEM | AK4113_DAT | 0xf0));
            (*ak4113).rcs0 = rcs0 & !(AK4113_QINT | AK4113_CINT | AK4113_STC);
            (*ak4113).rcs1 = rcs1;
            (*ak4113).rcs2 = rcs2;
            spin_unlock_irqrestore(&mut (*ak4113).lock, _flags);

            if (rcs0 & AK4113_PAR) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[0]).id);
            }
            if (rcs0 & AK4113_V) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[1]).id);
            }
            if (rcs2 & AK4113_CCRC) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[2]).id);
            }
            if (rcs2 & AK4113_QCRC) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[3]).id);
            }

            /* rate change */
            if (c1 & 0xf0) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[4]).id);
            }

            if ((c1 & AK4113_PEM) | (c0 & AK4113_CINT)) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[6]).id);
            }
            if (c0 & AK4113_QINT) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[8]).id);
            }

            if (c0 & AK4113_AUDION) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[9]).id);
            }
            if (c1 & AK4113_NPCM) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[10]).id);
            }
            if (c1 & AK4113_DTSCD) != 0 {
                snd_ctl_notify((*ak4113).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*(*ak4113).kctls[11]).id);
            }

            if (*ak4113).change_callback.is_some() && (c0 | c1) != 0 {
                ((*ak4113).change_callback).expect("non-null function pointer")(ak4113, c0, c1);
            }
        }

        /* compare rate */
        res = external_rate(rcs1) as i32;
        if (flags & AK4113_CHECK_NO_RATE) == 0 && !runtime.is_null() && (*runtime).rate != res as _ {
            snd_pcm_stream_lock_irqsave((*ak4113).substream, &mut _flags);
            if snd_pcm_running((*ak4113).substream) {
                snd_pcm_stop((*ak4113).substream, SNDRV_PCM_STATE_DRAINING);
                wake_up(&mut (*runtime).sleep);
                res = 1;
            }
            snd_pcm_stream_unlock_irqrestore((*ak4113).substream, _flags);
        }
        res
    }
}
// EXPORT_SYMBOL_GPL(snd_ak4113_check_rate_and_errors);

unsafe fn ak4113_stats(work: *mut work_struct) {
    unsafe {
        let chip: *mut ak4113 = container_of!(work, ak4113, work.work);

        if atomic_inc_return(&mut (*chip).wq_processing) == 1 {
            snd_ak4113_check_rate_and_errors(chip, (*chip).check_flags);
        }

        if atomic_dec_and_test(&mut (*chip).wq_processing) {
            schedule_delayed_work(&mut (*chip).work, HZ / 10);
        }
    }
}

// CONFIG_PM conditional code from the C source.
#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_suspend(chip: *mut ak4113) {
    unsafe {
        atomic_inc(&mut (*chip).wq_processing); /* don't schedule new work */
        cancel_delayed_work_sync(&mut (*chip).work);
    }
}
// EXPORT_SYMBOL(snd_ak4113_suspend);

#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn snd_ak4113_resume(chip: *mut ak4113) {
    unsafe {
        atomic_dec(&mut (*chip).wq_processing);
        snd_ak4113_reinit(chip);
    }
}
// EXPORT_SYMBOL(snd_ak4113_resume);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
