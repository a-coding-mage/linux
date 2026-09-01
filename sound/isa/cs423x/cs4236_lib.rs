// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of CS4235/4236B/4237B/4238B/4239 chips
 *
 *  Note:
 *     -----
 *
 *  Bugs:
 *     -----
 */

/*
 *  Indirect control registers (CS4236B+)
 *
 *  C0
 *     D8: WSS reset (all chips)
 *
 *  C1 (all chips except CS4236)
 *     D7-D5: version
 *     D4-D0: chip id
 *             11101 - CS4235
 *             01011 - CS4236B
 *             01000 - CS4237B
 *             01001 - CS4238B
 *             11110 - CS4239
 *
 *  C2
 *     D7-D4: 3D Space (CS4235,CS4237B,CS4238B,CS4239)
 *     D3-D0: 3D Center (CS4237B); 3D Volume (CS4238B)
 *
 *  C3
 *     D7: 3D Enable (CS4237B)
 *     D6: 3D Mono Enable (CS4237B)
 *     D5: 3D Serial Output (CS4237B,CS4238B)
 *     D4: 3D Enable (CS4235,CS4238B,CS4239)
 *
 *  C4
 *     D7: consumer serial port enable (CS4237B,CS4238B)
 *     D6: channels status block reset (CS4237B,CS4238B)
 *     D5: user bit in sub-frame of digital audio data (CS4237B,CS4238B)
 *     D4: validity bit in sub-frame of digital audio data (CS4237B,CS4238B)
 *
 *  C5  lower channel status (digital serial data description) (CS4237B,CS4238B)
 *     D7-D6: first two bits of category code
 *     D5: lock
 *     D4-D3: pre-emphasis (0 = none, 1 = 50/15us)
 *     D2: copy/copyright (0 = copy inhibited)
 *     D1: 0 = digital audio / 1 = non-digital audio
 *
 *  C6  upper channel status (digital serial data description) (CS4237B,CS4238B)
 *     D7-D6: sample frequency (0 = 44.1kHz)
 *     D5: generation status (0 = no indication, 1 = original/commercially precaptureed data)
 *     D4-D0: category code (upper bits)
 *
 *  C7  reserved (must write 0)
 *
 *  C8  wavetable control
 *     D7: volume control interrupt enable (CS4235,CS4239)
 *     D6: hardware volume control format (CS4235,CS4239)
 *     D3: wavetable serial port enable (all chips)
 *     D2: DSP serial port switch (all chips)
 *     D1: disable MCLK (all chips)
 *     D0: force BRESET low (all chips)
 *
 */

// Dependencies originally supplied by Linux/ALSA headers:
// linux/io.h, linux/delay.h, linux/init.h, linux/time.h, linux/wait.h,
// sound/core.h, sound/wss.h, sound/asoundef.h, sound/initval.h, sound/tlv.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct snd_pcm {
    pub info_flags: c_uint,
}

#[repr(C)]
pub struct snd_wss {
    pub cport: c_ulong,
    pub cimage: [u8; 32],
    pub image: [u8; 32],
    pub eimage: [u8; 32],
    pub hardware: c_ushort,
    pub rate_constraint: Option<unsafe extern "C" fn(*mut snd_pcm_runtime) -> c_int>,
    pub set_playback_format: Option<unsafe extern "C" fn(*mut snd_wss, *mut snd_pcm_hw_params, u8)>,
    pub set_capture_format: Option<unsafe extern "C" fn(*mut snd_wss, *mut snd_pcm_hw_params, u8)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_wss)>,
    pub pcm: *mut snd_pcm,
    pub card: *mut snd_card,
    pub reg_lock: c_void,
    pub mce_mutex: c_void,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub rate_den: c_uint,
}

#[repr(C)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_uint,
    pub rats: *const snd_ratnum,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_integer {
    pub min: i64,
    pub max: i64,
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub tlv: snd_kcontrol_tlv,
}

extern "C" {
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn udelay(usecs: c_ulong);
    fn snd_BUG();
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_wss_create(
        card: *mut snd_card,
        port: c_ulong,
        cport: c_ulong,
        irq: c_int,
        dma1: c_int,
        dma2: c_int,
        hardware: c_ushort,
        hwshare: c_ushort,
        rchip: *mut *mut snd_wss,
    ) -> c_int;
    fn snd_wss_pcm(chip: *mut snd_wss, device: c_int) -> c_int;
    fn snd_wss_out(chip: *mut snd_wss, reg: c_int, val: c_ushort);
    fn snd_wss_in(chip: *mut snd_wss, reg: c_int) -> u8;
    fn snd_wss_mce_up(chip: *mut snd_wss);
    fn snd_wss_mce_down(chip: *mut snd_wss);
    fn snd_cs4236_ext_out(chip: *mut snd_wss, reg: c_int, val: c_ushort);
    fn snd_cs4236_ext_in(chip: *mut snd_wss, reg: c_int) -> u8;
    fn snd_pcm_hw_constraint_ratnums(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        r: *const snd_pcm_hw_constraint_ratnums,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_wss;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut c_void) -> c_int;
    fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut snd_wss) -> *mut c_void;
    fn snd_wss_chip_id(chip: *mut snd_wss) -> *const c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0x0000_0003;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0x0004_0000;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 11;
const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 0x0020_0000;
const SNDRV_AUTO_PORT: c_ulong = !0;

const WSS_HW_DETECT: c_ushort = 0;
const WSS_HW_DETECT3: c_ushort = 3;
const WSS_HW_CS4235: c_ushort = 0x0005;
const WSS_HW_CS4237B: c_ushort = 0x0007;
const WSS_HW_CS4238B: c_ushort = 0x0008;
const WSS_HW_CS4239: c_ushort = 0x0009;
const WSS_HW_CS4236B_MASK: c_ushort = 0x00f0;

const CS4231_LEFT_INPUT: c_int = 0;
const CS4231_RIGHT_INPUT: c_int = 1;
const CS4231_LEFT_OUTPUT: c_int = 6;
const CS4231_RIGHT_OUTPUT: c_int = 7;
const CS4231_PLAYBK_FORMAT: c_int = 8;
const CS4231_REC_FORMAT: c_int = 28;
const CS4231_LOOPBACK: c_int = 13;
const CS4231_AUX1_LEFT_INPUT: c_int = 2;
const CS4231_AUX1_RIGHT_INPUT: c_int = 3;
const CS4231_AUX2_LEFT_INPUT: c_int = 4;
const CS4231_AUX2_RIGHT_INPUT: c_int = 5;
const CS4231_ALT_FEATURE_1: c_int = 16;
const CS4231_VERSION: c_int = 25;
const CS4231_MONO_CTRL: c_int = 26;
const CS4231_LEFT_LINE_IN: c_int = 18;
const CS4231_RIGHT_LINE_IN: c_int = 19;

const CS4236_EXT_REG: c_int = 23;
const CS4236_LEFT_LINE: c_int = 0;
const CS4236_RIGHT_LINE: c_int = 1;
const CS4236_LEFT_MIC: c_int = 2;
const CS4236_RIGHT_MIC: c_int = 3;
const CS4236_LEFT_MIX_CTRL: c_int = 4;
const CS4236_RIGHT_MIX_CTRL: c_int = 5;
const CS4236_LEFT_FM: c_int = 6;
const CS4236_RIGHT_FM: c_int = 7;
const CS4236_LEFT_DSP: c_int = 8;
const CS4236_RIGHT_DSP: c_int = 9;
const CS4236_RIGHT_LOOPBACK: c_int = 10;
const CS4236_DAC_MUTE: c_int = 11;
const CS4236_ADC_RATE: c_int = 12;
const CS4236_DAC_RATE: c_int = 13;
const CS4236_LEFT_MASTER: c_int = 14;
const CS4236_RIGHT_MASTER: c_int = 15;
const CS4236_LEFT_WAVE: c_int = 16;
const CS4236_RIGHT_WAVE: c_int = 17;
const CS4236_VERSION: c_int = 1;
const CS4235_LEFT_MASTER: c_int = 27;
const CS4235_RIGHT_MASTER: c_int = 29;

const IEC958_AES1_CON_PCM_CODER: c_uint = 0;
const IEC958_AES0_CON_EMPHASIS_NONE: c_uint = 0;

const fn CS4236_REG(reg: c_int) -> usize {
    (reg & 0xff) as usize
}

const fn CS4236_I23VAL(reg: c_uint) -> c_int {
    reg as c_int
}

const fn tlv_db_scale(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4] {
    [0, 2, min as c_uint, ((step as c_uint) & 0xffff) | ((mute as c_uint) << 16)]
}

static SND_CS4236_EXT_MAP: [u8; 18] = [
    0xff, 0xff, 0xdf, 0xdf, 0xe0 | 0x18, 0xe0, 0xbf, 0xbf, 0xbf, 0xbf, 0xbf, 0xe0, 0x01,
    0x01, 0xbf, 0xbf, 0xbf, 0xbf,
];

unsafe extern "C" fn snd_cs4236_ctrl_out(chip: *mut snd_wss, reg: u8, val: u8) {
    unsafe {
        outb(reg, (*chip).cport + 3);
        (*chip).cimage[reg as usize] = val;
        outb(val, (*chip).cport + 4);
    }
}

unsafe extern "C" fn snd_cs4236_ctrl_in(chip: *mut snd_wss, reg: u8) -> u8 {
    unsafe {
        outb(reg, (*chip).cport + 3);
        inb((*chip).cport + 4)
    }
}

const CLOCKS: usize = 8;

static CLOCKS_TABLE: [snd_ratnum; CLOCKS] = [
    snd_ratnum { num: 16934400, den_min: 353, den_max: 353, den_step: 1 },
    snd_ratnum { num: 16934400, den_min: 529, den_max: 529, den_step: 1 },
    snd_ratnum { num: 16934400, den_min: 617, den_max: 617, den_step: 1 },
    snd_ratnum { num: 16934400, den_min: 1058, den_max: 1058, den_step: 1 },
    snd_ratnum { num: 16934400, den_min: 1764, den_max: 1764, den_step: 1 },
    snd_ratnum { num: 16934400, den_min: 2117, den_max: 2117, den_step: 1 },
    snd_ratnum { num: 16934400, den_min: 2558, den_max: 2558, den_step: 1 },
    snd_ratnum { num: 16934400 / 16, den_min: 21, den_max: 192, den_step: 1 },
];

static HW_CONSTRAINTS_CLOCKS: snd_pcm_hw_constraint_ratnums =
    snd_pcm_hw_constraint_ratnums { nrats: CLOCKS as c_uint, rats: CLOCKS_TABLE.as_ptr() };

unsafe extern "C" fn snd_cs4236_xrate(runtime: *mut snd_pcm_runtime) -> c_int {
    unsafe { snd_pcm_hw_constraint_ratnums(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &HW_CONSTRAINTS_CLOCKS) }
}

unsafe extern "C" fn divisor_to_rate_register(divisor: c_uint) -> u8 {
    match divisor {
        353 => 1,
        529 => 2,
        617 => 3,
        1058 => 4,
        1764 => 5,
        2117 => 6,
        2558 => 7,
        _ => {
            if divisor < 21 || divisor > 192 {
                unsafe { snd_BUG() };
                return 192;
            }
            divisor as u8
        }
    }
}

unsafe extern "C" fn snd_cs4236_playback_format(
    chip: *mut snd_wss,
    params: *mut snd_pcm_hw_params,
    pdfr: u8,
) {
    unsafe {
        let rate = divisor_to_rate_register((*params).rate_den);
        snd_wss_out(chip, CS4231_ALT_FEATURE_1, ((*chip).image[CS4231_ALT_FEATURE_1 as usize] | 0x10) as c_ushort);
        snd_wss_out(chip, CS4231_PLAYBK_FORMAT, (pdfr & 0xf0) as c_ushort);
        snd_wss_out(chip, CS4231_ALT_FEATURE_1, ((*chip).image[CS4231_ALT_FEATURE_1 as usize] & !0x10) as c_ushort);
        snd_cs4236_ext_out(chip, CS4236_DAC_RATE, rate as c_ushort);
    }
}

unsafe extern "C" fn snd_cs4236_capture_format(
    chip: *mut snd_wss,
    params: *mut snd_pcm_hw_params,
    cdfr: u8,
) {
    unsafe {
        let rate = divisor_to_rate_register((*params).rate_den);
        snd_wss_out(chip, CS4231_ALT_FEATURE_1, ((*chip).image[CS4231_ALT_FEATURE_1 as usize] | 0x20) as c_ushort);
        snd_wss_out(chip, CS4231_REC_FORMAT, (cdfr & 0xf0) as c_ushort);
        snd_wss_out(chip, CS4231_ALT_FEATURE_1, ((*chip).image[CS4231_ALT_FEATURE_1 as usize] & !0x20) as c_ushort);
        snd_cs4236_ext_out(chip, CS4236_ADC_RATE, rate as c_ushort);
    }
}

// CONFIG_PM conditional code from the C source.
unsafe extern "C" fn snd_cs4236_suspend(chip: *mut snd_wss) {
    unsafe {
        for reg in 0..32 {
            (*chip).image[reg] = snd_wss_in(chip, reg as c_int);
        }
        for reg in 0..18 {
            (*chip).eimage[reg] = snd_cs4236_ext_in(chip, CS4236_I23VAL(reg as c_uint));
        }
        for reg in 2..9 {
            (*chip).cimage[reg] = snd_cs4236_ctrl_in(chip, reg as u8);
        }
    }
}

unsafe extern "C" fn snd_cs4236_resume(chip: *mut snd_wss) {
    unsafe {
        snd_wss_mce_up(chip);
        for reg in 0..32 {
            match reg as c_int {
                CS4236_EXT_REG | CS4231_VERSION | 27 | 29 => {}
                _ => snd_wss_out(chip, reg as c_int, (*chip).image[reg] as c_ushort),
            }
        }
        for reg in 0..18 {
            snd_cs4236_ext_out(chip, CS4236_I23VAL(reg as c_uint), (*chip).eimage[reg] as c_ushort);
        }
        for reg in 2..9 {
            if reg != 7 {
                snd_cs4236_ctrl_out(chip, reg as u8, (*chip).cimage[reg]);
            }
        }
        snd_wss_mce_down(chip);
    }
}

/*
 * This function does no fail if the chip is not CS4236B or compatible.
 * It just an equivalent to the snd_wss_create() then.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_cs4236_create(
    card: *mut snd_card,
    port: c_ulong,
    cport: c_ulong,
    irq: c_int,
    dma1: c_int,
    dma2: c_int,
    mut hardware: c_ushort,
    hwshare: c_ushort,
    rchip: *mut *mut snd_wss,
) -> c_int {
    unsafe {
        let mut chip: *mut snd_wss = core::ptr::null_mut();
        let ver1: u8;
        let ver2: u8;
        let mut reg: c_uint;
        let err: c_int;

        *rchip = core::ptr::null_mut();
        if hardware == WSS_HW_DETECT {
            hardware = WSS_HW_DETECT3;
        }

        err = snd_wss_create(card, port, cport, irq, dma1, dma2, hardware, hwshare, &mut chip);
        if err < 0 {
            return err;
        }

        if ((*chip).hardware & WSS_HW_CS4236B_MASK) == 0 {
            *rchip = chip;
            return 0;
        }

        // C source #if 0 debug dumps of CD and C registers are intentionally inactive.
        if cport < 0x100 || cport == SNDRV_AUTO_PORT {
            return -ENODEV;
        }
        ver1 = snd_cs4236_ctrl_in(chip, 1);
        ver2 = snd_cs4236_ext_in(chip, CS4236_VERSION);
        if ver1 != ver2 {
            return -ENODEV;
        }
        snd_cs4236_ctrl_out(chip, 0, 0x00);
        snd_cs4236_ctrl_out(chip, 2, 0xff);
        snd_cs4236_ctrl_out(chip, 3, 0x00);
        snd_cs4236_ctrl_out(chip, 4, 0x80);
        reg = ((IEC958_AES1_CON_PCM_CODER & 3) << 6) | IEC958_AES0_CON_EMPHASIS_NONE;
        snd_cs4236_ctrl_out(chip, 5, reg as u8);
        snd_cs4236_ctrl_out(chip, 6, (IEC958_AES1_CON_PCM_CODER >> 2) as u8);
        snd_cs4236_ctrl_out(chip, 7, 0x00);
        /*
         * 0x8c for C8 is valid for Turtle Beach Malibu - the IEC-958
         * output is working with this setup, other hardware should
         * have different signal paths and this value should be
         * selectable in the future
         */
        snd_cs4236_ctrl_out(chip, 8, 0x8c);
        (*chip).rate_constraint = Some(snd_cs4236_xrate);
        (*chip).set_playback_format = Some(snd_cs4236_playback_format);
        (*chip).set_capture_format = Some(snd_cs4236_capture_format);
        (*chip).suspend = Some(snd_cs4236_suspend);
        (*chip).resume = Some(snd_cs4236_resume);

        for reg in 0..SND_CS4236_EXT_MAP.len() as c_uint {
            snd_cs4236_ext_out(chip, CS4236_I23VAL(reg), SND_CS4236_EXT_MAP[reg as usize] as c_ushort);
        }

        snd_wss_out(chip, CS4231_LEFT_INPUT, 0x40);
        snd_wss_out(chip, CS4231_RIGHT_INPUT, 0x40);
        snd_wss_out(chip, CS4231_AUX1_LEFT_INPUT, 0xff);
        snd_wss_out(chip, CS4231_AUX1_RIGHT_INPUT, 0xff);
        snd_wss_out(chip, CS4231_AUX2_LEFT_INPUT, 0xdf);
        snd_wss_out(chip, CS4231_AUX2_RIGHT_INPUT, 0xdf);
        snd_wss_out(chip, CS4231_RIGHT_LINE_IN, 0xff);
        snd_wss_out(chip, CS4231_LEFT_LINE_IN, 0xff);
        snd_wss_out(chip, CS4231_RIGHT_LINE_IN, 0xff);
        match (*chip).hardware {
            WSS_HW_CS4235 | WSS_HW_CS4239 => {
                snd_wss_out(chip, CS4235_LEFT_MASTER, 0xff);
                snd_wss_out(chip, CS4235_RIGHT_MASTER, 0xff);
            }
            _ => {}
        }

        *rchip = chip;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_cs4236_pcm(chip: *mut snd_wss, device: c_int) -> c_int {
    unsafe {
        let err = snd_wss_pcm(chip, device);
        if err < 0 {
            return err;
        }
        (*(*chip).pcm).info_flags &= !SNDRV_PCM_INFO_JOINT_DUPLEX;
        0
    }
}

const fn private_single(reg: c_int, shift: c_int, mask: c_int, invert: c_int) -> c_ulong {
    (reg | (shift << 8) | (mask << 16) | (invert << 24)) as c_ulong
}

const fn private_double(
    left_reg: c_int,
    right_reg: c_int,
    shift_left: c_int,
    shift_right: c_int,
    mask: c_int,
    invert: c_int,
) -> c_ulong {
    (left_reg | (right_reg << 8) | (shift_left << 16) | (shift_right << 19) | (mask << 24) | (invert << 22)) as c_ulong
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

const fn kctl(
    name: &'static [u8],
    index: c_uint,
    access: c_uint,
    info: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int,
    get: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    put: unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    private_value: c_ulong,
    tlv: *const c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: cstr(name),
        index,
        access,
        info: Some(info),
        get: Some(get),
        put: Some(put),
        private_value,
        tlv: snd_kcontrol_tlv { p: tlv },
    }
}

unsafe extern "C" fn snd_cs4236_info_single(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        let mask = ((*kcontrol).private_value >> 16) & 0xff;
        (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = mask as i64;
        0
    }
}

unsafe extern "C" fn snd_cs4236_get_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let reg = ((*kcontrol).private_value & 0xff) as c_int;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as i64;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
        (*ucontrol).value.integer.value[0] = (((*chip).eimage[CS4236_REG(reg)] as c_int >> shift) as i64) & mask;
        if invert != 0 {
            (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
        }
        0
    }
}

unsafe extern "C" fn snd_cs4236_put_single(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let reg = ((*kcontrol).private_value & 0xff) as c_int;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
        let mut val = ((*ucontrol).value.integer.value[0] as c_int & mask) as c_ushort;
        if invert != 0 {
            val = (mask as c_ushort).wrapping_sub(val);
        }
        val <<= shift;
        val = (((*chip).eimage[CS4236_REG(reg)] as c_int & !(mask << shift)) as c_ushort) | val;
        let change = (val as u8 != (*chip).eimage[CS4236_REG(reg)]) as c_int;
        snd_cs4236_ext_out(chip, reg, val);
        change
    }
}

unsafe extern "C" fn snd_cs4236_get_singlec(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let reg = ((*kcontrol).private_value & 0xff) as usize;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as i64;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
        (*ucontrol).value.integer.value[0] = (((*chip).cimage[reg] as c_int >> shift) as i64) & mask;
        if invert != 0 {
            (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
        }
        0
    }
}

unsafe extern "C" fn snd_cs4236_put_singlec(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let reg = ((*kcontrol).private_value & 0xff) as usize;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
        let mut val = ((*ucontrol).value.integer.value[0] as c_int & mask) as c_ushort;
        if invert != 0 {
            val = (mask as c_ushort).wrapping_sub(val);
        }
        val <<= shift;
        val = (((*chip).cimage[reg] as c_int & !(mask << shift)) as c_ushort) | val;
        let change = (val as u8 != (*chip).cimage[reg]) as c_int;
        snd_cs4236_ctrl_out(chip, reg as u8, val as u8);
        change
    }
}

unsafe extern "C" fn snd_cs4236_info_double(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    unsafe {
        let mask = ((*kcontrol).private_value >> 24) & 0xff;
        (*uinfo).type_ = if mask == 1 { SNDRV_CTL_ELEM_TYPE_BOOLEAN } else { SNDRV_CTL_ELEM_TYPE_INTEGER };
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = mask as i64;
        0
    }
}

unsafe extern "C" fn snd_cs4236_get_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let left_reg = ((*kcontrol).private_value & 0xff) as c_int;
        let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
        let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as i64;
        let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
        (*ucontrol).value.integer.value[0] = (((*chip).eimage[CS4236_REG(left_reg)] as c_int >> shift_left) as i64) & mask;
        (*ucontrol).value.integer.value[1] = (((*chip).eimage[CS4236_REG(right_reg)] as c_int >> shift_right) as i64) & mask;
        if invert != 0 {
            (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
            (*ucontrol).value.integer.value[1] = mask - (*ucontrol).value.integer.value[1];
        }
        0
    }
}

unsafe extern "C" fn snd_cs4236_put_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let left_reg = ((*kcontrol).private_value & 0xff) as c_int;
        let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
        let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
        let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
        let mut val1 = ((*ucontrol).value.integer.value[0] as c_int & mask) as c_ushort;
        let mut val2 = ((*ucontrol).value.integer.value[1] as c_int & mask) as c_ushort;
        if invert != 0 {
            val1 = (mask as c_ushort).wrapping_sub(val1);
            val2 = (mask as c_ushort).wrapping_sub(val2);
        }
        val1 <<= shift_left;
        val2 <<= shift_right;
        if left_reg != right_reg {
            val1 = (((*chip).eimage[CS4236_REG(left_reg)] as c_int & !(mask << shift_left)) as c_ushort) | val1;
            val2 = (((*chip).eimage[CS4236_REG(right_reg)] as c_int & !(mask << shift_right)) as c_ushort) | val2;
            let change = (val1 as u8 != (*chip).eimage[CS4236_REG(left_reg)]
                || val2 as u8 != (*chip).eimage[CS4236_REG(right_reg)]) as c_int;
            snd_cs4236_ext_out(chip, left_reg, val1);
            snd_cs4236_ext_out(chip, right_reg, val2);
            change
        } else {
            val1 = (((*chip).eimage[CS4236_REG(left_reg)] as c_int
                & !((mask << shift_left) | (mask << shift_right))) as c_ushort)
                | val1
                | val2;
            let change = (val1 as u8 != (*chip).eimage[CS4236_REG(left_reg)]) as c_int;
            snd_cs4236_ext_out(chip, left_reg, val1);
            change
        }
    }
}

unsafe extern "C" fn snd_cs4236_get_double1(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let left_reg = ((*kcontrol).private_value & 0xff) as usize;
        let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
        let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as i64;
        let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
        (*ucontrol).value.integer.value[0] = (((*chip).image[left_reg] as c_int >> shift_left) as i64) & mask;
        (*ucontrol).value.integer.value[1] = (((*chip).eimage[CS4236_REG(right_reg)] as c_int >> shift_right) as i64) & mask;
        if invert != 0 {
            (*ucontrol).value.integer.value[0] = mask - (*ucontrol).value.integer.value[0];
            (*ucontrol).value.integer.value[1] = mask - (*ucontrol).value.integer.value[1];
        }
        0
    }
}

unsafe extern "C" fn snd_cs4236_put_double1(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let left_reg = ((*kcontrol).private_value & 0xff) as usize;
        let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
        let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
        let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
        let invert = (((*kcontrol).private_value >> 22) & 1) as c_int;
        let mut val1 = ((*ucontrol).value.integer.value[0] as c_int & mask) as c_ushort;
        let mut val2 = ((*ucontrol).value.integer.value[1] as c_int & mask) as c_ushort;
        if invert != 0 {
            val1 = (mask as c_ushort).wrapping_sub(val1);
            val2 = (mask as c_ushort).wrapping_sub(val2);
        }
        val1 <<= shift_left;
        val2 <<= shift_right;
        val1 = (((*chip).image[left_reg] as c_int & !(mask << shift_left)) as c_ushort) | val1;
        val2 = (((*chip).eimage[CS4236_REG(right_reg)] as c_int & !(mask << shift_right)) as c_ushort) | val2;
        let change = (val1 as u8 != (*chip).image[left_reg] || val2 as u8 != (*chip).eimage[CS4236_REG(right_reg)]) as c_int;
        snd_wss_out(chip, left_reg as c_int, val1);
        snd_cs4236_ext_out(chip, right_reg, val2);
        change
    }
}

#[inline]
fn snd_cs4236_mixer_master_digital_invert_volume(vol: c_int) -> c_int {
    if vol < 64 { 63 - vol } else { 64 + (71 - vol) }
}

unsafe extern "C" fn snd_cs4236_get_master_digital(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.integer.value[0] =
            snd_cs4236_mixer_master_digital_invert_volume((*chip).eimage[CS4236_REG(CS4236_LEFT_MASTER)] as c_int & 0x7f) as i64;
        (*ucontrol).value.integer.value[1] =
            snd_cs4236_mixer_master_digital_invert_volume((*chip).eimage[CS4236_REG(CS4236_RIGHT_MASTER)] as c_int & 0x7f) as i64;
        0
    }
}

unsafe extern "C" fn snd_cs4236_put_master_digital(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let mut val1 = snd_cs4236_mixer_master_digital_invert_volume((*ucontrol).value.integer.value[0] as c_int & 0x7f) as c_ushort;
        let mut val2 = snd_cs4236_mixer_master_digital_invert_volume((*ucontrol).value.integer.value[1] as c_int & 0x7f) as c_ushort;
        val1 = (((*chip).eimage[CS4236_REG(CS4236_LEFT_MASTER)] as c_int & !0x7f) as c_ushort) | val1;
        val2 = (((*chip).eimage[CS4236_REG(CS4236_RIGHT_MASTER)] as c_int & !0x7f) as c_ushort) | val2;
        let change = (val1 as u8 != (*chip).eimage[CS4236_REG(CS4236_LEFT_MASTER)]
            || val2 as u8 != (*chip).eimage[CS4236_REG(CS4236_RIGHT_MASTER)]) as c_int;
        snd_cs4236_ext_out(chip, CS4236_LEFT_MASTER, val1);
        snd_cs4236_ext_out(chip, CS4236_RIGHT_MASTER, val2);
        change
    }
}

#[inline]
fn snd_cs4235_mixer_output_accu_get_volume(vol: c_int) -> c_int {
    match (vol >> 5) & 3 {
        0 => 1,
        1 => 3,
        2 => 2,
        3 => 0,
        _ => 3,
    }
}

#[inline]
fn snd_cs4235_mixer_output_accu_set_volume(vol: c_int) -> c_int {
    match vol & 3 {
        0 => 3 << 5,
        1 => 0 << 5,
        2 => 2 << 5,
        3 => 1 << 5,
        _ => 1 << 5,
    }
}

unsafe extern "C" fn snd_cs4235_get_output_accu(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.integer.value[0] = snd_cs4235_mixer_output_accu_get_volume((*chip).image[CS4235_LEFT_MASTER as usize] as c_int) as i64;
        (*ucontrol).value.integer.value[1] = snd_cs4235_mixer_output_accu_get_volume((*chip).image[CS4235_RIGHT_MASTER as usize] as c_int) as i64;
        0
    }
}

unsafe extern "C" fn snd_cs4235_put_output_accu(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let mut val1 = snd_cs4235_mixer_output_accu_set_volume((*ucontrol).value.integer.value[0] as c_int) as c_ushort;
        let mut val2 = snd_cs4235_mixer_output_accu_set_volume((*ucontrol).value.integer.value[1] as c_int) as c_ushort;
        val1 = (((*chip).image[CS4235_LEFT_MASTER as usize] as c_int & !(3 << 5)) as c_ushort) | val1;
        val2 = (((*chip).image[CS4235_RIGHT_MASTER as usize] as c_int & !(3 << 5)) as c_ushort) | val2;
        let change = (val1 as u8 != (*chip).image[CS4235_LEFT_MASTER as usize] || val2 as u8 != (*chip).image[CS4235_RIGHT_MASTER as usize]) as c_int;
        snd_wss_out(chip, CS4235_LEFT_MASTER, val1);
        snd_wss_out(chip, CS4235_RIGHT_MASTER, val2);
        change
    }
}

static DB_SCALE_7BIT: [c_uint; 4] = tlv_db_scale(-9450, 150, 0);
static DB_SCALE_6BIT: [c_uint; 4] = tlv_db_scale(-9450, 150, 0);
static DB_SCALE_6BIT_12DB_MAX: [c_uint; 4] = tlv_db_scale(-8250, 150, 0);
static DB_SCALE_5BIT_12DB_MAX: [c_uint; 4] = tlv_db_scale(-3450, 150, 0);
static DB_SCALE_5BIT_22DB_MAX: [c_uint; 4] = tlv_db_scale(-2400, 150, 0);
static DB_SCALE_4BIT: [c_uint; 4] = tlv_db_scale(-4500, 300, 0);
static DB_SCALE_2BIT: [c_uint; 4] = tlv_db_scale(-1800, 600, 0);
static DB_SCALE_REC_GAIN: [c_uint; 4] = tlv_db_scale(0, 150, 0);
static DB_SCALE_5BIT_6DB_MAX: [c_uint; 4] = tlv_db_scale(-5600, 200, 0);
static DB_SCALE_2BIT_16DB_MAX: [c_uint; 4] = tlv_db_scale(-2400, 800, 0);

macro_rules! CS4236_SINGLE {
    ($name:expr, $index:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE, snd_cs4236_info_single, snd_cs4236_get_single, snd_cs4236_put_single, private_single($reg, $shift, $mask, $invert), core::ptr::null())
    };
}
macro_rules! CS4236_SINGLE_TLV {
    ($name:expr, $index:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr, $tlv:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, snd_cs4236_info_single, snd_cs4236_get_single, snd_cs4236_put_single, private_single($reg, $shift, $mask, $invert), $tlv.as_ptr())
    };
}
macro_rules! CS4236_SINGLEC {
    ($name:expr, $index:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE, snd_cs4236_info_single, snd_cs4236_get_singlec, snd_cs4236_put_singlec, private_single($reg, $shift, $mask, $invert), core::ptr::null())
    };
}
macro_rules! CS4236_DOUBLE {
    ($name:expr, $index:expr, $left:expr, $right:expr, $sl:expr, $sr:expr, $mask:expr, $invert:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE, snd_cs4236_info_double, snd_cs4236_get_double, snd_cs4236_put_double, private_double($left, $right, $sl, $sr, $mask, $invert), core::ptr::null())
    };
}
macro_rules! CS4236_DOUBLE_TLV {
    ($name:expr, $index:expr, $left:expr, $right:expr, $sl:expr, $sr:expr, $mask:expr, $invert:expr, $tlv:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, snd_cs4236_info_double, snd_cs4236_get_double, snd_cs4236_put_double, private_double($left, $right, $sl, $sr, $mask, $invert), $tlv.as_ptr())
    };
}
macro_rules! CS4236_DOUBLE1 {
    ($name:expr, $index:expr, $left:expr, $right:expr, $sl:expr, $sr:expr, $mask:expr, $invert:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE, snd_cs4236_info_double, snd_cs4236_get_double1, snd_cs4236_put_double1, private_double($left, $right, $sl, $sr, $mask, $invert), core::ptr::null())
    };
}
macro_rules! CS4236_DOUBLE1_TLV {
    ($name:expr, $index:expr, $left:expr, $right:expr, $sl:expr, $sr:expr, $mask:expr, $invert:expr, $tlv:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, snd_cs4236_info_double, snd_cs4236_get_double1, snd_cs4236_put_double1, private_double($left, $right, $sl, $sr, $mask, $invert), $tlv.as_ptr())
    };
}
macro_rules! WSS_DOUBLE {
    ($name:expr, $index:expr, $left:expr, $right:expr, $sl:expr, $sr:expr, $mask:expr, $invert:expr) => {
        CS4236_DOUBLE1!($name, $index, $left, $right, $sl, $sr, $mask, $invert)
    };
}
macro_rules! WSS_DOUBLE_TLV {
    ($name:expr, $index:expr, $left:expr, $right:expr, $sl:expr, $sr:expr, $mask:expr, $invert:expr, $tlv:expr) => {
        CS4236_DOUBLE1_TLV!($name, $index, $left, $right, $sl, $sr, $mask, $invert, $tlv)
    };
}
macro_rules! WSS_SINGLE {
    ($name:expr, $index:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr) => {
        CS4236_SINGLE!($name, $index, $reg, $shift, $mask, $invert)
    };
}
macro_rules! WSS_SINGLE_TLV {
    ($name:expr, $index:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr, $tlv:expr) => {
        CS4236_SINGLE_TLV!($name, $index, $reg, $shift, $mask, $invert, $tlv)
    };
}
macro_rules! CS4236_MASTER_DIGITAL {
    ($name:expr, $index:expr, $tlv:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, snd_cs4236_info_double, snd_cs4236_get_master_digital, snd_cs4236_put_master_digital, 71 << 24, $tlv.as_ptr())
    };
}
macro_rules! CS4235_OUTPUT_ACCU {
    ($name:expr, $index:expr, $tlv:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ, snd_cs4236_info_double, snd_cs4235_get_output_accu, snd_cs4235_put_output_accu, 3 << 24, $tlv.as_ptr())
    };
}
macro_rules! CS4236_IEC958_ENABLE {
    ($name:expr, $index:expr) => {
        kctl($name, $index, SNDRV_CTL_ELEM_ACCESS_READWRITE, snd_cs4236_info_single, snd_cs4236_get_iec958_switch, snd_cs4236_put_iec958_switch, 1 << 16, core::ptr::null())
    };
}

static SND_CS4236_CONTROLS: [snd_kcontrol_new; 35] = [
    CS4236_DOUBLE!(b"Master Digital Playback Switch\0", 0, CS4236_LEFT_MASTER, CS4236_RIGHT_MASTER, 7, 7, 1, 1),
    CS4236_DOUBLE!(b"Master Digital Capture Switch\0", 0, CS4236_DAC_MUTE, CS4236_DAC_MUTE, 7, 6, 1, 1),
    CS4236_MASTER_DIGITAL!(b"Master Digital Volume\0", 0, DB_SCALE_7BIT),
    CS4236_DOUBLE_TLV!(b"Capture Boost Volume\0", 0, CS4236_LEFT_MIX_CTRL, CS4236_RIGHT_MIX_CTRL, 5, 5, 3, 1, DB_SCALE_2BIT),
    WSS_DOUBLE!(b"PCM Playback Switch\0", 0, CS4231_LEFT_OUTPUT, CS4231_RIGHT_OUTPUT, 7, 7, 1, 1),
    WSS_DOUBLE_TLV!(b"PCM Playback Volume\0", 0, CS4231_LEFT_OUTPUT, CS4231_RIGHT_OUTPUT, 0, 0, 63, 1, DB_SCALE_6BIT),
    CS4236_DOUBLE!(b"DSP Playback Switch\0", 0, CS4236_LEFT_DSP, CS4236_RIGHT_DSP, 7, 7, 1, 1),
    CS4236_DOUBLE_TLV!(b"DSP Playback Volume\0", 0, CS4236_LEFT_DSP, CS4236_RIGHT_DSP, 0, 0, 63, 1, DB_SCALE_6BIT),
    CS4236_DOUBLE!(b"FM Playback Switch\0", 0, CS4236_LEFT_FM, CS4236_RIGHT_FM, 7, 7, 1, 1),
    CS4236_DOUBLE_TLV!(b"FM Playback Volume\0", 0, CS4236_LEFT_FM, CS4236_RIGHT_FM, 0, 0, 63, 1, DB_SCALE_6BIT),
    CS4236_DOUBLE!(b"Wavetable Playback Switch\0", 0, CS4236_LEFT_WAVE, CS4236_RIGHT_WAVE, 7, 7, 1, 1),
    CS4236_DOUBLE_TLV!(b"Wavetable Playback Volume\0", 0, CS4236_LEFT_WAVE, CS4236_RIGHT_WAVE, 0, 0, 63, 1, DB_SCALE_6BIT_12DB_MAX),
    WSS_DOUBLE!(b"Synth Playback Switch\0", 0, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 7, 7, 1, 1),
    WSS_DOUBLE_TLV!(b"Synth Volume\0", 0, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 0, 0, 31, 1, DB_SCALE_5BIT_12DB_MAX),
    WSS_DOUBLE!(b"Synth Capture Switch\0", 0, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 6, 6, 1, 1),
    WSS_DOUBLE!(b"Synth Capture Bypass\0", 0, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 5, 5, 1, 1),
    CS4236_DOUBLE!(b"Mic Playback Switch\0", 0, CS4236_LEFT_MIC, CS4236_RIGHT_MIC, 6, 6, 1, 1),
    CS4236_DOUBLE!(b"Mic Capture Switch\0", 0, CS4236_LEFT_MIC, CS4236_RIGHT_MIC, 7, 7, 1, 1),
    CS4236_DOUBLE_TLV!(b"Mic Volume\0", 0, CS4236_LEFT_MIC, CS4236_RIGHT_MIC, 0, 0, 31, 1, DB_SCALE_5BIT_22DB_MAX),
    CS4236_DOUBLE!(b"Mic Playback Boost (+20dB)\0", 0, CS4236_LEFT_MIC, CS4236_RIGHT_MIC, 5, 5, 1, 0),
    WSS_DOUBLE!(b"Line Playback Switch\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 7, 7, 1, 1),
    WSS_DOUBLE_TLV!(b"Line Volume\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 0, 0, 31, 1, DB_SCALE_5BIT_12DB_MAX),
    WSS_DOUBLE!(b"Line Capture Switch\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 6, 6, 1, 1),
    WSS_DOUBLE!(b"Line Capture Bypass\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 5, 5, 1, 1),
    WSS_DOUBLE!(b"CD Playback Switch\0", 0, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 7, 7, 1, 1),
    WSS_DOUBLE_TLV!(b"CD Volume\0", 0, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 0, 0, 31, 1, DB_SCALE_5BIT_12DB_MAX),
    WSS_DOUBLE!(b"CD Capture Switch\0", 0, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 6, 6, 1, 1),
    CS4236_DOUBLE1!(b"Mono Output Playback Switch\0", 0, CS4231_MONO_CTRL, CS4236_RIGHT_MIX_CTRL, 6, 7, 1, 1),
    CS4236_DOUBLE1!(b"Beep Playback Switch\0", 0, CS4231_MONO_CTRL, CS4236_LEFT_MIX_CTRL, 7, 7, 1, 1),
    WSS_SINGLE_TLV!(b"Beep Playback Volume\0", 0, CS4231_MONO_CTRL, 0, 15, 1, DB_SCALE_4BIT),
    WSS_SINGLE!(b"Beep Bypass Playback Switch\0", 0, CS4231_MONO_CTRL, 5, 1, 0),
    WSS_DOUBLE_TLV!(b"Capture Volume\0", 0, CS4231_LEFT_INPUT, CS4231_RIGHT_INPUT, 0, 0, 15, 0, DB_SCALE_REC_GAIN),
    WSS_DOUBLE!(b"Analog Loopback Capture Switch\0", 0, CS4231_LEFT_INPUT, CS4231_RIGHT_INPUT, 7, 7, 1, 0),
    WSS_SINGLE!(b"Loopback Digital Playback Switch\0", 0, CS4231_LOOPBACK, 0, 1, 0),
    CS4236_DOUBLE1_TLV!(b"Loopback Digital Playback Volume\0", 0, CS4231_LOOPBACK, CS4236_RIGHT_LOOPBACK, 2, 0, 63, 1, DB_SCALE_6BIT),
];

static SND_CS4235_CONTROLS: [snd_kcontrol_new; 25] = [
    WSS_DOUBLE!(b"Master Playback Switch\0", 0, CS4235_LEFT_MASTER, CS4235_RIGHT_MASTER, 7, 7, 1, 1),
    WSS_DOUBLE_TLV!(b"Master Playback Volume\0", 0, CS4235_LEFT_MASTER, CS4235_RIGHT_MASTER, 0, 0, 31, 1, DB_SCALE_5BIT_6DB_MAX),
    CS4235_OUTPUT_ACCU!(b"Playback Volume\0", 0, DB_SCALE_2BIT_16DB_MAX),
    WSS_DOUBLE!(b"Synth Playback Switch\0", 1, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 7, 7, 1, 1),
    WSS_DOUBLE!(b"Synth Capture Switch\0", 1, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 6, 6, 1, 1),
    WSS_DOUBLE_TLV!(b"Synth Volume\0", 1, CS4231_LEFT_LINE_IN, CS4231_RIGHT_LINE_IN, 0, 0, 31, 1, DB_SCALE_5BIT_12DB_MAX),
    CS4236_DOUBLE_TLV!(b"Capture Volume\0", 0, CS4236_LEFT_MIX_CTRL, CS4236_RIGHT_MIX_CTRL, 5, 5, 3, 1, DB_SCALE_2BIT),
    WSS_DOUBLE!(b"PCM Playback Switch\0", 0, CS4231_LEFT_OUTPUT, CS4231_RIGHT_OUTPUT, 7, 7, 1, 1),
    WSS_DOUBLE!(b"PCM Capture Switch\0", 0, CS4236_DAC_MUTE, CS4236_DAC_MUTE, 7, 6, 1, 1),
    WSS_DOUBLE_TLV!(b"PCM Volume\0", 0, CS4231_LEFT_OUTPUT, CS4231_RIGHT_OUTPUT, 0, 0, 63, 1, DB_SCALE_6BIT),
    CS4236_DOUBLE!(b"DSP Switch\0", 0, CS4236_LEFT_DSP, CS4236_RIGHT_DSP, 7, 7, 1, 1),
    CS4236_DOUBLE!(b"FM Switch\0", 0, CS4236_LEFT_FM, CS4236_RIGHT_FM, 7, 7, 1, 1),
    CS4236_DOUBLE!(b"Wavetable Switch\0", 0, CS4236_LEFT_WAVE, CS4236_RIGHT_WAVE, 7, 7, 1, 1),
    CS4236_DOUBLE!(b"Mic Capture Switch\0", 0, CS4236_LEFT_MIC, CS4236_RIGHT_MIC, 7, 7, 1, 1),
    CS4236_DOUBLE!(b"Mic Playback Switch\0", 0, CS4236_LEFT_MIC, CS4236_RIGHT_MIC, 6, 6, 1, 1),
    CS4236_SINGLE_TLV!(b"Mic Volume\0", 0, CS4236_LEFT_MIC, 0, 31, 1, DB_SCALE_5BIT_22DB_MAX),
    CS4236_SINGLE!(b"Mic Boost (+20dB)\0", 0, CS4236_LEFT_MIC, 5, 1, 0),
    WSS_DOUBLE!(b"Line Playback Switch\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 7, 7, 1, 1),
    WSS_DOUBLE!(b"Line Capture Switch\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 6, 6, 1, 1),
    WSS_DOUBLE_TLV!(b"Line Volume\0", 0, CS4231_AUX1_LEFT_INPUT, CS4231_AUX1_RIGHT_INPUT, 0, 0, 31, 1, DB_SCALE_5BIT_12DB_MAX),
    WSS_DOUBLE!(b"CD Playback Switch\0", 1, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 7, 7, 1, 1),
    WSS_DOUBLE!(b"CD Capture Switch\0", 1, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 6, 6, 1, 1),
    WSS_DOUBLE_TLV!(b"CD Volume\0", 1, CS4231_AUX2_LEFT_INPUT, CS4231_AUX2_RIGHT_INPUT, 0, 0, 31, 1, DB_SCALE_5BIT_12DB_MAX),
    CS4236_DOUBLE1!(b"Beep Playback Switch\0", 0, CS4231_MONO_CTRL, CS4236_LEFT_MIX_CTRL, 7, 7, 1, 1),
    WSS_SINGLE!(b"Beep Playback Volume\0", 0, CS4231_MONO_CTRL, 0, 15, 1),
];

unsafe extern "C" fn snd_cs4236_get_iec958_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        (*ucontrol).value.integer.value[0] = if ((*chip).image[CS4231_ALT_FEATURE_1 as usize] & 0x02) != 0 { 1 } else { 0 };
        // C source #if 0 debug logging is intentionally inactive.
        0
    }
}

unsafe extern "C" fn snd_cs4236_put_iec958_switch(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    unsafe {
        let chip = snd_kcontrol_chip(kcontrol);
        let enable = ((*ucontrol).value.integer.value[0] & 1) as c_ushort;
        snd_wss_mce_up(chip);
        let mut val = (((*chip).image[CS4231_ALT_FEATURE_1 as usize] as c_ushort & !0x0e) | (0 << 2) | (enable << 1)) as c_ushort;
        let change = (val as u8 != (*chip).image[CS4231_ALT_FEATURE_1 as usize]) as c_int;
        snd_wss_out(chip, CS4231_ALT_FEATURE_1, val);
        val = (snd_cs4236_ctrl_in(chip, 4) as c_ushort) | 0xc0;
        snd_cs4236_ctrl_out(chip, 4, val as u8);
        udelay(100);
        val &= !0x40;
        snd_cs4236_ctrl_out(chip, 4, val as u8);
        snd_wss_mce_down(chip);
        // C source #if 0 debug logging is intentionally inactive.
        change
    }
}

static SND_CS4236_IEC958_CONTROLS: [snd_kcontrol_new; 6] = [
    CS4236_IEC958_ENABLE!(b"IEC958 Output Enable\0", 0),
    CS4236_SINGLEC!(b"IEC958 Output Validity\0", 0, 4, 4, 1, 0),
    CS4236_SINGLEC!(b"IEC958 Output User\0", 0, 4, 5, 1, 0),
    CS4236_SINGLEC!(b"IEC958 Output CSBR\0", 0, 4, 6, 1, 0),
    CS4236_SINGLEC!(b"IEC958 Output Channel Status Low\0", 0, 5, 1, 127, 0),
    CS4236_SINGLEC!(b"IEC958 Output Channel Status High\0", 0, 6, 0, 255, 0),
];

static SND_CS4236_3D_CONTROLS_CS4235: [snd_kcontrol_new; 2] = [
    CS4236_SINGLEC!(b"3D Control - Switch\0", 0, 3, 4, 1, 0),
    CS4236_SINGLEC!(b"3D Control - Space\0", 0, 2, 4, 15, 1),
];

static SND_CS4236_3D_CONTROLS_CS4237: [snd_kcontrol_new; 5] = [
    CS4236_SINGLEC!(b"3D Control - Switch\0", 0, 3, 7, 1, 0),
    CS4236_SINGLEC!(b"3D Control - Space\0", 0, 2, 4, 15, 1),
    CS4236_SINGLEC!(b"3D Control - Center\0", 0, 2, 0, 15, 1),
    CS4236_SINGLEC!(b"3D Control - Mono\0", 0, 3, 6, 1, 0),
    CS4236_SINGLEC!(b"3D Control - IEC958\0", 0, 3, 5, 1, 0),
];

static SND_CS4236_3D_CONTROLS_CS4238: [snd_kcontrol_new; 4] = [
    CS4236_SINGLEC!(b"3D Control - Switch\0", 0, 3, 4, 1, 0),
    CS4236_SINGLEC!(b"3D Control - Space\0", 0, 2, 4, 15, 1),
    CS4236_SINGLEC!(b"3D Control - Volume\0", 0, 2, 0, 15, 1),
    CS4236_SINGLEC!(b"3D Control - IEC958\0", 0, 3, 5, 1, 0),
];

#[no_mangle]
pub unsafe extern "C" fn snd_cs4236_mixer(chip: *mut snd_wss) -> c_int {
    unsafe {
        let card: *mut snd_card;
        let mut idx: c_uint;
        let count: c_uint;
        let mut err: c_int;
        let mut kcontrol: *const snd_kcontrol_new;

        if snd_BUG_ON(chip.is_null() || (*chip).card.is_null()) != 0 {
            return -EINVAL;
        }
        card = (*chip).card;
        strscpy((*card).mixername, snd_wss_chip_id(chip));

        if (*chip).hardware == WSS_HW_CS4235 || (*chip).hardware == WSS_HW_CS4239 {
            idx = 0;
            while idx < SND_CS4235_CONTROLS.len() as c_uint {
                err = snd_ctl_add(card, snd_ctl_new1(&SND_CS4235_CONTROLS[idx as usize], chip));
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        } else {
            idx = 0;
            while idx < SND_CS4236_CONTROLS.len() as c_uint {
                err = snd_ctl_add(card, snd_ctl_new1(&SND_CS4236_CONTROLS[idx as usize], chip));
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        }

        match (*chip).hardware {
            WSS_HW_CS4235 | WSS_HW_CS4239 => {
                count = SND_CS4236_3D_CONTROLS_CS4235.len() as c_uint;
                kcontrol = SND_CS4236_3D_CONTROLS_CS4235.as_ptr();
            }
            WSS_HW_CS4237B => {
                count = SND_CS4236_3D_CONTROLS_CS4237.len() as c_uint;
                kcontrol = SND_CS4236_3D_CONTROLS_CS4237.as_ptr();
            }
            WSS_HW_CS4238B => {
                count = SND_CS4236_3D_CONTROLS_CS4238.len() as c_uint;
                kcontrol = SND_CS4236_3D_CONTROLS_CS4238.as_ptr();
            }
            _ => {
                count = 0;
                kcontrol = core::ptr::null();
            }
        }

        idx = 0;
        while idx < count {
            err = snd_ctl_add(card, snd_ctl_new1(kcontrol, chip));
            if err < 0 {
                return err;
            }
            idx += 1;
            kcontrol = kcontrol.add(1);
        }

        if (*chip).hardware == WSS_HW_CS4237B || (*chip).hardware == WSS_HW_CS4238B {
            idx = 0;
            while idx < SND_CS4236_IEC958_CONTROLS.len() as c_uint {
                err = snd_ctl_add(card, snd_ctl_new1(&SND_CS4236_IEC958_CONTROLS[idx as usize], chip));
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
