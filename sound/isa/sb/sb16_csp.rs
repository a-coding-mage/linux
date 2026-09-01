// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) 1999 by Uros Bizjak <uros@kss-loka.si>
 *                        Takashi Iwai <tiwai@suse.de>
 *
 *  SB16ASP/AWE32 CSP control
 *
 *  CSP microcode loader:
 *   alsa-tools/sb16_csp/
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __le16 = u16;
type __le32 = u32;
type snd_pcm_format_t = c_int;

const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const EFAULT: c_int = 14;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ENOTTY: c_int = 25;
const ENXIO: c_int = 6;

const SNDRV_LITTLE_ENDIAN: bool = cfg!(target_endian = "little");

const fn csp_hdr_value(a: u32, b: u32, c: u32, d: u32) -> u32 {
    if SNDRV_LITTLE_ENDIAN {
        a | (b << 8) | (c << 16) | (d << 24)
    } else {
        d | (c << 8) | (b << 16) | (a << 24)
    }
}

const RIFF_HEADER: u32 = csp_hdr_value(b'R' as u32, b'I' as u32, b'F' as u32, b'F' as u32);
const CSP__HEADER: u32 = csp_hdr_value(b'C' as u32, b'S' as u32, b'P' as u32, b' ' as u32);
const LIST_HEADER: u32 = csp_hdr_value(b'L' as u32, b'I' as u32, b'S' as u32, b'T' as u32);
const FUNC_HEADER: u32 = csp_hdr_value(b'f' as u32, b'u' as u32, b'n' as u32, b'c' as u32);
const CODE_HEADER: u32 = csp_hdr_value(b'c' as u32, b'o' as u32, b'd' as u32, b'e' as u32);
const INIT_HEADER: u32 = csp_hdr_value(b'i' as u32, b'n' as u32, b'i' as u32, b't' as u32);
const MAIN_HEADER: u32 = csp_hdr_value(b'm' as u32, b'a' as u32, b'i' as u32, b'n' as u32);

/*
 * RIFF data format
 */
#[repr(C)]
struct riff_header {
    name: __le32,
    len: __le32,
}

#[repr(C)]
struct desc_header {
    info: riff_header,
    func_nr: __le16,
    VOC_type: __le16,
    flags_play_rec: __le16,
    flags_16bit_8bit: __le16,
    flags_stereo_mono: __le16,
    flags_rates: __le16,
}

#[repr(C)]
struct snd_sb {
    card: *mut snd_card,
    reg_lock: spinlock_t,
    mixer_lock: spinlock_t,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct file {
    _private: [u8; 0],
}

#[repr(C)]
struct firmware {
    size: usize,
    data: *const u8,
}

#[repr(C)]
struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_hwdep_ops {
    open: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
    ioctl: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file, c_uint, c_ulong) -> c_int>,
    release: Option<unsafe extern "C" fn(*mut snd_hwdep, *mut file) -> c_int>,
}

#[repr(C)]
struct snd_hwdep {
    name: [c_char; 32],
    iface: c_int,
    private_data: *mut c_void,
    private_free: Option<unsafe extern "C" fn(*mut snd_hwdep)>,
    ops: snd_hwdep_ops,
}

#[repr(C)]
struct snd_sb_csp_ops {
    csp_use: Option<unsafe extern "C" fn(*mut snd_sb_csp) -> c_int>,
    csp_unuse: Option<unsafe extern "C" fn(*mut snd_sb_csp) -> c_int>,
    csp_autoload: Option<unsafe extern "C" fn(*mut snd_sb_csp, snd_pcm_format_t, c_int) -> c_int>,
    csp_start: Option<unsafe extern "C" fn(*mut snd_sb_csp, c_int, c_int) -> c_int>,
    csp_stop: Option<unsafe extern "C" fn(*mut snd_sb_csp) -> c_int>,
    csp_qsound_transfer: Option<unsafe extern "C" fn(*mut snd_sb_csp) -> c_int>,
}

#[repr(C)]
struct snd_sb_csp {
    chip: *mut snd_sb,
    version: c_int,
    ops: snd_sb_csp_ops,
    access_mutex: mutex,
    used: c_int,
    running: c_int,
    codec_name: [c_char; 16],
    func_nr: c_uint,
    acc_format: c_uint,
    acc_channels: c_int,
    acc_width: c_int,
    acc_rates: c_int,
    mode: c_int,
    run_channels: c_int,
    run_width: c_int,
    csp_programs: [*const firmware; CSP_PROGRAM_COUNT],
    q_enabled: u8,
    q_lock: spinlock_t,
    qpos_left: u8,
    qpos_right: u8,
    qpos_changed: c_int,
    qsound_switch: *mut snd_kcontrol,
    qsound_space: *mut snd_kcontrol,
}

#[repr(C)]
struct snd_sb_csp_mc_header {
    codec_name: [c_char; 16],
    func_req: u16,
    data: *mut u8,
}

#[repr(C)]
struct snd_sb_csp_microcode {
    codec_name: [c_char; 16],
    func_req: u16,
    data: *mut u8,
}

#[repr(C)]
struct snd_sb_csp_info {
    codec_name: [c_char; 16],
    func_nr: c_uint,
    acc_format: c_uint,
    acc_channels: c_int,
    acc_width: c_int,
    acc_rates: c_int,
    csp_mode: c_int,
    run_channels: c_int,
    run_width: c_int,
    version: c_int,
    state: c_int,
}

#[repr(C)]
struct snd_sb_csp_start {
    sample_width: c_int,
    channels: c_int,
}

#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_ctl_elem_info_integer {
    min: i64,
    max: i64,
}

#[repr(C)]
union snd_ctl_elem_info_value {
    integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
struct snd_ctl_elem_info {
    type_: c_uint,
    count: c_uint,
    value: snd_ctl_elem_info_value,
}

#[repr(C)]
struct snd_ctl_elem_value_integer {
    value: [i64; 128],
}

#[repr(C)]
union snd_ctl_elem_value_value {
    integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct snd_info_entry {
    private_data: *mut c_void,
}

#[repr(C)]
struct snd_info_buffer {
    _private: [u8; 0],
}

unsafe extern "C" {
    static snd_ctl_boolean_mono_info:
        unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int;

    fn snd_hwdep_new(card: *mut snd_card, id: *const c_char, device: c_int, rhwdep: *mut *mut snd_hwdep) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn kfree(ptr: *const c_void);
    fn release_firmware(fw: *const firmware);
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn memdup_user(src: *const c_void, len: usize) -> *mut c_void;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, device: *mut device) -> c_int;
    fn snd_sbdsp_command(chip: *mut snd_sb, val: u8) -> c_int;
    fn snd_sbdsp_get_byte(chip: *mut snd_sb) -> c_int;
    fn snd_sbdsp_reset(chip: *mut snd_sb);
    fn snd_sbmixer_read(chip: *mut snd_sb, reg: c_int) -> c_int;
    fn snd_sbmixer_write(chip: *mut snd_sb, reg: c_int, val: u8);
    fn udelay(usecs: c_ulong);
    fn snd_ctl_new1(kcontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        private_data: *mut c_void,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    ) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn str_enabled_disabled(enabled: c_int) -> *const c_char;
}

const SNDRV_HWDEP_IFACE_SB16CSP: c_int = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_SB_CSP_IOCTL_INFO: c_uint = 0;
const SNDRV_SB_CSP_IOCTL_LOAD_CODE: c_uint = 1;
const SNDRV_SB_CSP_IOCTL_UNLOAD_CODE: c_uint = 2;
const SNDRV_SB_CSP_IOCTL_START: c_uint = 3;
const SNDRV_SB_CSP_IOCTL_STOP: c_uint = 4;
const SNDRV_SB_CSP_IOCTL_PAUSE: c_uint = 5;
const SNDRV_SB_CSP_IOCTL_RESTART: c_uint = 6;
const SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE: usize = 0x3000;
const SNDRV_SB_CSP_LOAD_INITBLOCK: c_int = 1;
const SNDRV_SB_CSP_ST_RUNNING: c_int = 0x01;
const SNDRV_SB_CSP_ST_LOADED: c_int = 0x02;
const SNDRV_SB_CSP_ST_AUTO: c_int = 0x04;
const SNDRV_SB_CSP_ST_QSOUND: c_int = 0x08;
const SNDRV_SB_CSP_ST_PAUSED: c_int = 0x10;
const SNDRV_SB_CSP_MODE_DSP_READ: c_int = 0x01;
const SNDRV_SB_CSP_MODE_DSP_WRITE: c_int = 0x02;
const SNDRV_SB_CSP_MODE_QSOUND: c_int = 0x04;
const SNDRV_PCM_FMTBIT_A_LAW: c_uint = 0x01;
const SNDRV_PCM_FMTBIT_MU_LAW: c_uint = 0x02;
const SNDRV_PCM_FMTBIT_IMA_ADPCM: c_uint = 0x04;
const SNDRV_PCM_FMTBIT_SPECIAL: c_uint = 0x08;
const SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t = 0;
const SNDRV_PCM_FORMAT_A_LAW: snd_pcm_format_t = 1;
const SNDRV_PCM_FORMAT_IMA_ADPCM: snd_pcm_format_t = 2;
const SNDRV_SB_CSP_SAMPLE_16BIT: c_int = 0x01;
const SNDRV_SB_CSP_SAMPLE_8BIT: c_int = 0x02;
const SNDRV_SB_CSP_MONO: c_int = 0x01;
const SNDRV_SB_CSP_STEREO: c_int = 0x02;
const SNDRV_SB_CSP_RATE_ALL: c_int = 0x0f;
const SNDRV_SB_CSP_RATE_8000: c_int = 0x01;
const SNDRV_SB_CSP_RATE_11025: c_int = 0x02;
const SNDRV_SB_CSP_RATE_22050: c_int = 0x04;
const SNDRV_SB_CSP_RATE_44100: c_int = 0x08;
const SNDRV_SB_CSP_QSOUND_MAX_RIGHT: u8 = 0xff;
const SB_DSP4_DMASETUP: c_int = 0;
const SB_DMASETUP_DMA7: c_int = 0x80;
const SB_DMASETUP_DMA6: c_int = 0x40;
const SB_DMASETUP_DMA5: c_int = 0x20;
const SB_DSP4_PCM_DEV: c_int = 0;
const CSP_PROGRAM_MULAW: usize = 0;
const CSP_PROGRAM_ALAW: usize = 1;
const CSP_PROGRAM_ADPCM_INIT: usize = 2;
const CSP_PROGRAM_ADPCM_PLAYBACK: usize = 3;
const CSP_PROGRAM_ADPCM_CAPTURE: usize = 4;
const CSP_PROGRAM_COUNT: usize = 5;

unsafe fn le16_to_cpu(v: __le16) -> u16 {
    u16::from_le(v)
}

unsafe fn le32_to_cpu(v: __le32) -> u32 {
    u32::from_le(v)
}

unsafe fn snd_BUG_ON(cond: bool) -> bool {
    cond
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) > -4096
}

unsafe fn PTR_ERR(ptr: *const c_void) -> c_int {
    ptr as isize as c_int
}

unsafe fn kzalloc_obj<T>() -> *mut T {
    let layout = std::alloc::Layout::new::<T>();
    let ptr = std::alloc::alloc_zeroed(layout) as *mut T;
    ptr
}

/*
 * Detect CSP chip and create a new instance
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sb_csp_new(chip: *mut snd_sb, device: c_int, rhwdep: *mut *mut snd_hwdep) -> c_int {
    let mut p: *mut snd_sb_csp;
    let mut version: c_int = 0;
    let mut err: c_int;
    let mut hw: *mut snd_hwdep = ptr::null_mut();

    if !rhwdep.is_null() {
        *rhwdep = ptr::null_mut();
    }

    if csp_detect(chip, &mut version) != 0 {
        return -ENODEV;
    }

    err = snd_hwdep_new((*chip).card, b"SB16-CSP\0".as_ptr() as *const c_char, device, &mut hw);
    if err < 0 {
        return err;
    }

    p = kzalloc_obj::<snd_sb_csp>();
    if p.is_null() {
        snd_device_free((*chip).card, hw as *mut c_void);
        return -ENOMEM;
    }
    (*p).chip = chip;
    (*p).version = version;

    /* CSP operators */
    (*p).ops.csp_use = Some(snd_sb_csp_use);
    (*p).ops.csp_unuse = Some(snd_sb_csp_unuse);
    (*p).ops.csp_autoload = Some(snd_sb_csp_autoload);
    (*p).ops.csp_start = Some(snd_sb_csp_start);
    (*p).ops.csp_stop = Some(snd_sb_csp_stop);
    (*p).ops.csp_qsound_transfer = Some(snd_sb_csp_qsound_transfer);

    mutex_init(&mut (*p).access_mutex);
    sprintf((*hw).name.as_mut_ptr(), b"CSP v%d.%d\0".as_ptr() as *const c_char, version >> 4, version & 0x0f);
    (*hw).iface = SNDRV_HWDEP_IFACE_SB16CSP;
    (*hw).private_data = p as *mut c_void;
    (*hw).private_free = Some(snd_sb_csp_free);

    /* operators - only write/ioctl */
    (*hw).ops.open = Some(snd_sb_csp_open);
    (*hw).ops.ioctl = Some(snd_sb_csp_ioctl);
    (*hw).ops.release = Some(snd_sb_csp_release);

    /* create a proc entry */
    init_proc_entry(p, device);
    if !rhwdep.is_null() {
        *rhwdep = hw;
    }
    0
}

/*
 * free_private for hwdep instance
 */
unsafe extern "C" fn snd_sb_csp_free(hwdep: *mut snd_hwdep) {
    let mut i: usize;
    let p = (*hwdep).private_data as *mut snd_sb_csp;
    if !p.is_null() {
        if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 {
            snd_sb_csp_stop(p);
        }
        i = 0;
        while i < (*p).csp_programs.len() {
            release_firmware((*p).csp_programs[i]);
            i += 1;
        }
        kfree(p as *mut c_void);
    }
}

/* ------------------------------ */

/*
 * open the device exclusively
 */
unsafe extern "C" fn snd_sb_csp_open(hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    let p = (*hw).private_data as *mut snd_sb_csp;
    snd_sb_csp_use(p)
}

/*
 * ioctl for hwdep device:
 */
unsafe extern "C" fn snd_sb_csp_ioctl(hw: *mut snd_hwdep, _file: *mut file, cmd: c_uint, arg: c_ulong) -> c_int {
    let p = (*hw).private_data as *mut snd_sb_csp;
    let mut info: snd_sb_csp_info = zeroed();
    let mut start_info: snd_sb_csp_start = zeroed();
    let err: c_int;

    if snd_BUG_ON(p.is_null()) {
        return -EINVAL;
    }

    if snd_sb_csp_check_version(p) != 0 {
        return -ENODEV;
    }

    match cmd {
        /* get information */
        SNDRV_SB_CSP_IOCTL_INFO => {
            info.codec_name[0] = (*p).codec_name[0];
            info.func_nr = (*p).func_nr;
            info.acc_format = (*p).acc_format;
            info.acc_channels = (*p).acc_channels;
            info.acc_width = (*p).acc_width;
            info.acc_rates = (*p).acc_rates;
            info.csp_mode = (*p).mode;
            info.run_channels = (*p).run_channels;
            info.run_width = (*p).run_width;
            info.version = (*p).version;
            info.state = (*p).running;
            if copy_to_user(arg as *mut c_void, &info as *const _ as *const c_void, size_of::<snd_sb_csp_info>()) != 0 {
                err = -EFAULT;
            } else {
                err = 0;
            }
        }

        /* load CSP microcode */
        SNDRV_SB_CSP_IOCTL_LOAD_CODE => {
            err = if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 {
                -EBUSY
            } else {
                snd_sb_csp_riff_load(p, arg as *mut snd_sb_csp_microcode)
            };
        }
        SNDRV_SB_CSP_IOCTL_UNLOAD_CODE => {
            err = if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 {
                -EBUSY
            } else {
                snd_sb_csp_unload(p)
            };
        }

        /* change CSP running state */
        SNDRV_SB_CSP_IOCTL_START => {
            if copy_from_user(&mut start_info as *mut _ as *mut c_void, arg as *const c_void, size_of::<snd_sb_csp_start>()) != 0 {
                err = -EFAULT;
            } else {
                err = snd_sb_csp_start(p, start_info.sample_width, start_info.channels);
            }
        }
        SNDRV_SB_CSP_IOCTL_STOP => {
            err = snd_sb_csp_stop(p);
        }
        SNDRV_SB_CSP_IOCTL_PAUSE => {
            err = snd_sb_csp_pause(p);
        }
        SNDRV_SB_CSP_IOCTL_RESTART => {
            err = snd_sb_csp_restart(p);
        }
        _ => {
            err = -ENOTTY;
        }
    }

    err
}

/*
 * close the device
 */
unsafe extern "C" fn snd_sb_csp_release(hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    let p = (*hw).private_data as *mut snd_sb_csp;
    snd_sb_csp_unuse(p)
}

/* ------------------------------ */

/*
 * acquire device
 */
unsafe extern "C" fn snd_sb_csp_use(p: *mut snd_sb_csp) -> c_int {
    if (*p).used != 0 {
        return -EAGAIN;
    }
    (*p).used += 1;
    0
}

/*
 * release device
 */
unsafe extern "C" fn snd_sb_csp_unuse(p: *mut snd_sb_csp) -> c_int {
    (*p).used -= 1;
    0
}

/*
 * load microcode via ioctl:
 * code is user-space pointer
 */
unsafe fn snd_sb_csp_riff_load(p: *mut snd_sb_csp, mcode: *mut snd_sb_csp_microcode) -> c_int {
    let mut info: snd_sb_csp_mc_header = zeroed();
    let dev = (*(*(*p).chip).card).dev;

    let mut data_ptr: *mut u8;
    let data_end: *mut u8;
    let mut func_nr: u16 = 0;

    let mut file_h: riff_header = zeroed();
    let mut item_h: riff_header = zeroed();
    let mut code_h: riff_header = zeroed();
    let mut item_type: __le32 = 0;
    let mut funcdesc_h: desc_header = zeroed();

    let mut err: c_int;

    if copy_from_user(&mut info as *mut _ as *mut c_void, mcode as *const c_void, size_of::<snd_sb_csp_mc_header>()) != 0 {
        return -EFAULT;
    }
    data_ptr = (*mcode).data;

    if copy_from_user(&mut file_h as *mut _ as *mut c_void, data_ptr as *const c_void, size_of::<riff_header>()) != 0 {
        return -EFAULT;
    }
    if le32_to_cpu(file_h.name) != RIFF_HEADER
        || le32_to_cpu(file_h.len) as usize >= SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE - size_of::<riff_header>()
    {
        dev_dbg(dev, b"%s: Invalid RIFF header\n\0".as_ptr() as *const c_char, b"snd_sb_csp_riff_load\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    data_ptr = data_ptr.add(size_of::<riff_header>());
    data_end = data_ptr.add(le32_to_cpu(file_h.len) as usize);

    if copy_from_user(&mut item_type as *mut _ as *mut c_void, data_ptr as *const c_void, size_of::<__le32>()) != 0 {
        return -EFAULT;
    }
    if le32_to_cpu(item_type) != CSP__HEADER {
        dev_dbg(dev, b"%s: Invalid RIFF file type\n\0".as_ptr() as *const c_char, b"snd_sb_csp_riff_load\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    data_ptr = data_ptr.add(size_of::<__le32>());

    while data_ptr < data_end {
        if copy_from_user(&mut item_h as *mut _ as *mut c_void, data_ptr as *const c_void, size_of::<riff_header>()) != 0 {
            return -EFAULT;
        }
        data_ptr = data_ptr.add(size_of::<riff_header>());
        if le32_to_cpu(item_h.name) != LIST_HEADER {
            data_ptr = data_ptr.add(le32_to_cpu(item_h.len) as usize);
            continue;
        }

        if copy_from_user(&mut item_type as *mut _ as *mut c_void, data_ptr as *const c_void, size_of::<__le32>()) != 0 {
            return -EFAULT;
        }
        match le32_to_cpu(item_type) {
            FUNC_HEADER => {
                if copy_from_user(
                    &mut funcdesc_h as *mut _ as *mut c_void,
                    data_ptr.add(size_of::<__le32>()) as *const c_void,
                    size_of::<desc_header>(),
                ) != 0
                {
                    return -EFAULT;
                }
                func_nr = le16_to_cpu(funcdesc_h.func_nr);
            }
            CODE_HEADER => {
                if func_nr != info.func_req {
                    data_ptr = data_ptr.add(le32_to_cpu(item_h.len) as usize);
                    continue; /* not required function, try next */
                }
                data_ptr = data_ptr.add(size_of::<__le32>());

                /* destroy QSound mixer element */
                if (*p).mode == SNDRV_SB_CSP_MODE_QSOUND {
                    snd_sb_qsound_destroy(p);
                }
                /* Clear all flags */
                (*p).running = 0;
                (*p).mode = 0;

                /* load microcode blocks */
                loop {
                    if data_ptr >= data_end {
                        return -EINVAL;
                    }
                    if copy_from_user(&mut code_h as *mut _ as *mut c_void, data_ptr as *const c_void, size_of::<riff_header>()) != 0 {
                        return -EFAULT;
                    }

                    /* init microcode blocks */
                    if le32_to_cpu(code_h.name) != INIT_HEADER {
                        break;
                    }
                    data_ptr = data_ptr.add(size_of::<riff_header>());
                    err = snd_sb_csp_load_user(
                        p,
                        data_ptr as *const u8,
                        le32_to_cpu(code_h.len) as c_int,
                        SNDRV_SB_CSP_LOAD_INITBLOCK,
                    );
                    if err != 0 {
                        return err;
                    }
                    data_ptr = data_ptr.add(le32_to_cpu(code_h.len) as usize);
                }
                /* main microcode block */
                if copy_from_user(&mut code_h as *mut _ as *mut c_void, data_ptr as *const c_void, size_of::<riff_header>()) != 0 {
                    return -EFAULT;
                }

                if le32_to_cpu(code_h.name) != MAIN_HEADER {
                    dev_dbg(dev, b"%s: Missing 'main' microcode\n\0".as_ptr() as *const c_char, b"snd_sb_csp_riff_load\0".as_ptr() as *const c_char);
                    return -EINVAL;
                }
                data_ptr = data_ptr.add(size_of::<riff_header>());
                err = snd_sb_csp_load_user(p, data_ptr as *const u8, le32_to_cpu(code_h.len) as c_int, 0);
                if err != 0 {
                    return err;
                }

                /* fill in codec header */
                strscpy((*p).codec_name.as_mut_ptr(), info.codec_name.as_ptr(), (*p).codec_name.len());
                (*p).func_nr = func_nr as c_uint;
                (*p).mode = le16_to_cpu(funcdesc_h.flags_play_rec) as c_int;
                match le16_to_cpu(funcdesc_h.VOC_type) {
                    0x0001 => {
                        /* QSound decoder */
                        if le16_to_cpu(funcdesc_h.flags_play_rec) as c_int == SNDRV_SB_CSP_MODE_DSP_WRITE {
                            if snd_sb_qsound_build(p) == 0 {
                                /* set QSound flag and clear all other mode flags */
                                (*p).mode = SNDRV_SB_CSP_MODE_QSOUND;
                            }
                        }
                        (*p).acc_format = 0;
                    }
                    0x0006 => {
                        /* A Law codec */
                        (*p).acc_format = SNDRV_PCM_FMTBIT_A_LAW;
                    }
                    0x0007 => {
                        /* Mu Law codec */
                        (*p).acc_format = SNDRV_PCM_FMTBIT_MU_LAW;
                    }
                    0x0011 | 0x0200 => {
                        /* what Creative thinks is IMA ADPCM codec */
                        /* Creative ADPCM codec */
                        (*p).acc_format = SNDRV_PCM_FMTBIT_IMA_ADPCM;
                    }
                    201 => {
                        /* Text 2 Speech decoder */
                        /* TODO: Text2Speech handling routines */
                        (*p).acc_format = 0;
                    }
                    0x0202 | 0x0203 => {
                        /* Fast Speech 8 codec */
                        /* Fast Speech 10 codec */
                        (*p).acc_format = SNDRV_PCM_FMTBIT_SPECIAL;
                    }
                    _ => {
                        /* other codecs are unsupported */
                        (*p).acc_format = 0;
                        (*p).acc_width = 0;
                        (*p).acc_rates = 0;
                        (*p).mode = 0;
                        dev_dbg(
                            dev,
                            b"%s: Unsupported CSP codec type: 0x%04x\n\0".as_ptr() as *const c_char,
                            b"snd_sb_csp_riff_load\0".as_ptr() as *const c_char,
                            le16_to_cpu(funcdesc_h.VOC_type) as c_int,
                        );
                        return -EINVAL;
                    }
                }
                (*p).acc_channels = le16_to_cpu(funcdesc_h.flags_stereo_mono) as c_int;
                (*p).acc_width = le16_to_cpu(funcdesc_h.flags_16bit_8bit) as c_int;
                (*p).acc_rates = le16_to_cpu(funcdesc_h.flags_rates) as c_int;

                /* Decouple CSP from IRQ and DMAREQ lines */
                set_mode_register((*p).chip, 0xfc);
                set_mode_register((*p).chip, 0x00);

                /* finished loading successfully */
                (*p).running = SNDRV_SB_CSP_ST_LOADED; /* set LOADED flag */
                return 0;
            }
            _ => {}
        }
        data_ptr = data_ptr.add(le32_to_cpu(item_h.len) as usize);
    }
    dev_dbg(
        dev,
        b"%s: Function #%d not found\n\0".as_ptr() as *const c_char,
        b"snd_sb_csp_riff_load\0".as_ptr() as *const c_char,
        info.func_req as c_int,
    );
    -EINVAL
}

/*
 * unload CSP microcode
 */
unsafe fn snd_sb_csp_unload(p: *mut snd_sb_csp) -> c_int {
    if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 {
        return -EBUSY;
    }
    if !((*p).running & SNDRV_SB_CSP_ST_LOADED != 0) {
        return -ENXIO;
    }

    /* clear supported formats */
    (*p).acc_format = 0;
    (*p).acc_rates = 0;
    (*p).acc_width = 0;
    (*p).acc_channels = 0;
    /* destroy QSound mixer element */
    if (*p).mode == SNDRV_SB_CSP_MODE_QSOUND {
        snd_sb_qsound_destroy(p);
    }
    /* clear all flags */
    (*p).running = 0;
    (*p).mode = 0;
    0
}

/*
 * send command sequence to DSP
 */
unsafe fn command_seq(chip: *mut snd_sb, seq: *const u8, size: c_int) -> c_int {
    let mut i = 0;
    while i < size {
        if snd_sbdsp_command(chip, *seq.offset(i as isize)) == 0 {
            return -EIO;
        }
        i += 1;
    }
    0
}

/*
 * set CSP codec parameter
 */
unsafe fn set_codec_parameter(chip: *mut snd_sb, par: u8, val: u8) -> c_int {
    let mut dsp_cmd = [0u8; 3];

    dsp_cmd[0] = 0x05; /* CSP set codec parameter */
    dsp_cmd[1] = val; /* Parameter value */
    dsp_cmd[2] = par; /* Parameter */
    command_seq(chip, dsp_cmd.as_ptr(), 3);
    snd_sbdsp_command(chip, 0x03); /* DSP read? */
    if snd_sbdsp_get_byte(chip) != par as c_int {
        return -EIO;
    }
    0
}

/*
 * set CSP register
 */
unsafe fn set_register(chip: *mut snd_sb, reg: u8, val: u8) -> c_int {
    let mut dsp_cmd = [0u8; 3];

    dsp_cmd[0] = 0x0e; /* CSP set register */
    dsp_cmd[1] = reg; /* CSP Register */
    dsp_cmd[2] = val; /* value */
    command_seq(chip, dsp_cmd.as_ptr(), 3)
}

/*
 * read CSP register
 * return < 0 -> error
 */
unsafe fn read_register(chip: *mut snd_sb, reg: u8) -> c_int {
    let mut dsp_cmd = [0u8; 2];

    dsp_cmd[0] = 0x0f; /* CSP read register */
    dsp_cmd[1] = reg; /* CSP Register */
    command_seq(chip, dsp_cmd.as_ptr(), 2);
    snd_sbdsp_get_byte(chip) /* Read DSP value */
}

/*
 * set CSP mode register
 */
unsafe fn set_mode_register(chip: *mut snd_sb, mode: u8) -> c_int {
    let mut dsp_cmd = [0u8; 2];

    dsp_cmd[0] = 0x04; /* CSP set mode register */
    dsp_cmd[1] = mode; /* mode */
    command_seq(chip, dsp_cmd.as_ptr(), 2)
}

/*
 * Detect CSP
 * return 0 if CSP exists.
 */
unsafe fn csp_detect(chip: *mut snd_sb, version: *mut c_int) -> c_int {
    let csp_test1: u8;
    let csp_test2: u8;

    set_codec_parameter(chip, 0x00, 0x00);
    set_mode_register(chip, 0xfc); /* 0xfc = ?? */

    csp_test1 = read_register(chip, 0x83) as u8;
    set_register(chip, 0x83, !csp_test1);
    csp_test2 = read_register(chip, 0x83) as u8;
    if csp_test2 != (csp_test1 ^ 0xff) {
        return -ENODEV;
    }

    set_register(chip, 0x83, csp_test1);
    let csp_test2_again = read_register(chip, 0x83) as u8;
    if csp_test2_again != csp_test1 {
        return -ENODEV;
    }

    set_mode_register(chip, 0x00); /* 0x00 = ? */

    *version = get_version(chip);
    snd_sbdsp_reset(chip); /* reset DSP after getversion! */
    if *version >= 0x10 && *version <= 0x1f {
        return 0; /* valid version id */
    }

    -ENODEV
}

/*
 * get CSP version number
 */
unsafe fn get_version(chip: *mut snd_sb) -> c_int {
    let mut dsp_cmd = [0u8; 2];

    dsp_cmd[0] = 0x08; /* SB_DSP_!something! */
    dsp_cmd[1] = 0x03; /* get chip version id? */
    command_seq(chip, dsp_cmd.as_ptr(), 2);

    snd_sbdsp_get_byte(chip)
}

/*
 * check if the CSP version is valid
 */
unsafe fn snd_sb_csp_check_version(p: *mut snd_sb_csp) -> c_int {
    if (*p).version < 0x10 || (*p).version > 0x1f {
        dev_dbg(
            (*(*(*p).chip).card).dev,
            b"%s: Invalid CSP version: 0x%x\n\0".as_ptr() as *const c_char,
            b"snd_sb_csp_check_version\0".as_ptr() as *const c_char,
            (*p).version,
        );
        return 1;
    }
    0
}

/*
 * download microcode to CSP (microcode should have one "main" block).
 */
unsafe fn snd_sb_csp_load(p: *mut snd_sb_csp, mut buf: *const u8, mut size: c_int, load_flags: c_int) -> c_int {
    let mut status: c_int;
    let mut i: c_int;
    let err: c_int;

    snd_sbdsp_command((*p).chip, 0x01); /* CSP download command */
    if snd_sbdsp_get_byte((*p).chip) != 0 {
        dev_dbg((*(*(*p).chip).card).dev, b"%s: Download command failed\n\0".as_ptr() as *const c_char, b"snd_sb_csp_load\0".as_ptr() as *const c_char);
        return -EIO;
    }
    /* Send CSP low byte (size - 1) */
    snd_sbdsp_command((*p).chip, (size - 1) as u8);
    /* Send high byte */
    snd_sbdsp_command((*p).chip, ((size - 1) >> 8) as u8);
    /* send microcode sequence */
    /* load from kernel space */
    while size != 0 {
        size -= 1;
        if snd_sbdsp_command((*p).chip, *buf) == 0 {
            return -EIO;
        }
        buf = buf.add(1);
    }
    if snd_sbdsp_get_byte((*p).chip) != 0 {
        return -EIO;
    }

    if load_flags & SNDRV_SB_CSP_LOAD_INITBLOCK != 0 {
        i = 0;
        /* some codecs (FastSpeech) take some time to initialize */
        loop {
            snd_sbdsp_command((*p).chip, 0x03);
            status = snd_sbdsp_get_byte((*p).chip);
            i += 1;
            if status == 0x55 || i >= 10 {
                break;
            }
            udelay(10);
        }
        if status != 0x55 {
            dev_dbg(
                (*(*(*p).chip).card).dev,
                b"%s: Microcode initialization failed\n\0".as_ptr() as *const c_char,
                b"snd_sb_csp_load\0".as_ptr() as *const c_char,
            );
            return -EIO;
        }
    } else {
        /*
         * Read mixer register SB_DSP4_DMASETUP after loading 'main' code.
         * Start CSP chip if no 16bit DMA channel is set - some kind
         * of autorun or perhaps a bugfix?
         */
        status = snd_sbmixer_read((*p).chip, SB_DSP4_DMASETUP);
        if status & (SB_DMASETUP_DMA7 | SB_DMASETUP_DMA6 | SB_DMASETUP_DMA5) == 0 {
            err = if set_codec_parameter((*p).chip, 0xaa, 0x00) != 0
                || set_codec_parameter((*p).chip, 0xff, 0x00) != 0
            {
                1
            } else {
                0
            };
            snd_sbdsp_reset((*p).chip); /* really! */
            if err != 0 {
                return -EIO;
            }
            set_mode_register((*p).chip, 0xc0); /* c0 = STOP */
            set_mode_register((*p).chip, 0x70); /* 70 = RUN */
        }
    }

    0
}

unsafe fn snd_sb_csp_load_user(p: *mut snd_sb_csp, buf: *const u8, size: c_int, load_flags: c_int) -> c_int {
    let err: c_int;
    let kbuf: *mut u8;

    kbuf = memdup_user(buf as *const c_void, size as usize) as *mut u8;
    if IS_ERR(kbuf as *const c_void) {
        return PTR_ERR(kbuf as *const c_void);
    }

    err = snd_sb_csp_load(p, kbuf, size, load_flags);

    kfree(kbuf as *mut c_void);
    err
}

unsafe fn snd_sb_csp_firmware_load(p: *mut snd_sb_csp, index: c_int, flags: c_int) -> c_int {
    static names: [&[u8]; CSP_PROGRAM_COUNT] = [
        b"sb16/mulaw_main.csp\0",
        b"sb16/alaw_main.csp\0",
        b"sb16/ima_adpcm_init.csp\0",
        b"sb16/ima_adpcm_playback.csp\0",
        b"sb16/ima_adpcm_capture.csp\0",
    ];
    let mut program: *const firmware;

    program = (*p).csp_programs[index as usize];
    if program.is_null() {
        let err = request_firmware(&mut program, names[index as usize].as_ptr() as *const c_char, (*(*(*p).chip).card).dev);
        if err < 0 {
            return err;
        }
        (*p).csp_programs[index as usize] = program;
    }
    snd_sb_csp_load(p, (*program).data, (*program).size as c_int, flags)
}

/*
 * autoload hardware codec if necessary
 * return 0 if CSP is loaded and ready to run (p->running != 0)
 */
unsafe extern "C" fn snd_sb_csp_autoload(p: *mut snd_sb_csp, pcm_sfmt: snd_pcm_format_t, play_rec_mode: c_int) -> c_int {
    let mut err: c_int = 0;

    /* if CSP is running or manually loaded then exit */
    if (*p).running & (SNDRV_SB_CSP_ST_RUNNING | SNDRV_SB_CSP_ST_LOADED) != 0 {
        return -EBUSY;
    }

    /* autoload microcode only if requested hardware codec is not already loaded */
    if (((1u32 << pcm_sfmt) & (*p).acc_format) != 0) && (play_rec_mode & (*p).mode != 0) {
        (*p).running = SNDRV_SB_CSP_ST_AUTO;
    } else {
        match pcm_sfmt {
            SNDRV_PCM_FORMAT_MU_LAW => {
                err = snd_sb_csp_firmware_load(p, CSP_PROGRAM_MULAW as c_int, 0);
                (*p).acc_format = SNDRV_PCM_FMTBIT_MU_LAW;
                (*p).mode = SNDRV_SB_CSP_MODE_DSP_READ | SNDRV_SB_CSP_MODE_DSP_WRITE;
            }
            SNDRV_PCM_FORMAT_A_LAW => {
                err = snd_sb_csp_firmware_load(p, CSP_PROGRAM_ALAW as c_int, 0);
                (*p).acc_format = SNDRV_PCM_FMTBIT_A_LAW;
                (*p).mode = SNDRV_SB_CSP_MODE_DSP_READ | SNDRV_SB_CSP_MODE_DSP_WRITE;
            }
            SNDRV_PCM_FORMAT_IMA_ADPCM => {
                err = snd_sb_csp_firmware_load(p, CSP_PROGRAM_ADPCM_INIT as c_int, SNDRV_SB_CSP_LOAD_INITBLOCK);
                if err == 0 {
                    if play_rec_mode == SNDRV_SB_CSP_MODE_DSP_WRITE {
                        err = snd_sb_csp_firmware_load(p, CSP_PROGRAM_ADPCM_PLAYBACK as c_int, 0);
                        (*p).mode = SNDRV_SB_CSP_MODE_DSP_WRITE;
                    } else {
                        err = snd_sb_csp_firmware_load(p, CSP_PROGRAM_ADPCM_CAPTURE as c_int, 0);
                        (*p).mode = SNDRV_SB_CSP_MODE_DSP_READ;
                    }
                    (*p).acc_format = SNDRV_PCM_FMTBIT_IMA_ADPCM;
                }
            }
            _ => {
                /* Decouple CSP from IRQ and DMAREQ lines */
                if (*p).running & SNDRV_SB_CSP_ST_AUTO != 0 {
                    set_mode_register((*p).chip, 0xfc);
                    set_mode_register((*p).chip, 0x00);
                    (*p).running = 0; /* clear autoloaded flag */
                }
                return -EINVAL;
            }
        }
        if err != 0 {
            (*p).acc_format = 0;
            (*p).acc_rates = 0;
            (*p).acc_width = 0;
            (*p).acc_channels = 0;

            (*p).running = 0; /* clear autoloaded flag */
            (*p).mode = 0;
            return err;
        } else {
            (*p).running = SNDRV_SB_CSP_ST_AUTO; /* set autoloaded flag */
            (*p).acc_width = SNDRV_SB_CSP_SAMPLE_16BIT; /* only 16 bit data */
            (*p).acc_channels = SNDRV_SB_CSP_MONO | SNDRV_SB_CSP_STEREO;
            (*p).acc_rates = SNDRV_SB_CSP_RATE_ALL; /* HW codecs accept all rates */
        }
    }
    if (*p).running & SNDRV_SB_CSP_ST_AUTO != 0 {
        0
    } else {
        -ENXIO
    }
}

/*
 * start CSP
 */
unsafe extern "C" fn snd_sb_csp_start(p: *mut snd_sb_csp, sample_width: c_int, channels: c_int) -> c_int {
    let dev = (*(*(*p).chip).card).dev;
    let mut s_type: u8; /* sample type */
    let mixL: u8;
    let mixR: u8;
    let mut result: c_int = -EIO;

    if !((*p).running & (SNDRV_SB_CSP_ST_LOADED | SNDRV_SB_CSP_ST_AUTO) != 0) {
        dev_dbg(dev, b"%s: Microcode not loaded\n\0".as_ptr() as *const c_char, b"snd_sb_csp_start\0".as_ptr() as *const c_char);
        return -ENXIO;
    }
    if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 {
        dev_dbg(dev, b"%s: CSP already running\n\0".as_ptr() as *const c_char, b"snd_sb_csp_start\0".as_ptr() as *const c_char);
        return -EBUSY;
    }
    if sample_width & (*p).acc_width == 0 {
        dev_dbg(dev, b"%s: Unsupported PCM sample width\n\0".as_ptr() as *const c_char, b"snd_sb_csp_start\0".as_ptr() as *const c_char);
        return -EINVAL;
    }
    if channels & (*p).acc_channels == 0 {
        dev_dbg(dev, b"%s: Invalid number of channels\n\0".as_ptr() as *const c_char, b"snd_sb_csp_start\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    /* Mute PCM volume */
    mixL = snd_sbmixer_read((*p).chip, SB_DSP4_PCM_DEV) as u8;
    mixR = snd_sbmixer_read((*p).chip, SB_DSP4_PCM_DEV + 1) as u8;
    snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV, mixL & 0x7);
    snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV + 1, mixR & 0x7);

    set_mode_register((*p).chip, 0xc0); /* c0 = STOP */
    set_mode_register((*p).chip, 0x70); /* 70 = RUN */

    s_type = 0x00;
    if channels == SNDRV_SB_CSP_MONO {
        s_type = 0x11; /* 000n 000n    (n = 1 if mono) */
    }
    if sample_width == SNDRV_SB_CSP_SAMPLE_8BIT {
        s_type |= 0x22; /* 00dX 00dX    (d = 1 if 8 bit samples) */
    }

    if set_codec_parameter((*p).chip, 0x81, s_type) != 0 {
        dev_dbg(dev, b"%s: Set sample type command failed\n\0".as_ptr() as *const c_char, b"snd_sb_csp_start\0".as_ptr() as *const c_char);
    } else if set_codec_parameter((*p).chip, 0x80, 0x00) != 0 {
        dev_dbg(dev, b"%s: Codec start command failed\n\0".as_ptr() as *const c_char, b"snd_sb_csp_start\0".as_ptr() as *const c_char);
    } else {
        (*p).run_width = sample_width;
        (*p).run_channels = channels;

        (*p).running |= SNDRV_SB_CSP_ST_RUNNING;

        if (*p).mode & SNDRV_SB_CSP_MODE_QSOUND != 0 {
            set_codec_parameter((*p).chip, 0xe0, 0x01);
            /* enable QSound decoder */
            set_codec_parameter((*p).chip, 0x00, 0xff);
            set_codec_parameter((*p).chip, 0x01, 0xff);
            (*p).running |= SNDRV_SB_CSP_ST_QSOUND;
            /* set QSound startup value */
            snd_sb_csp_qsound_transfer(p);
        }
        result = 0;
    }

    /* restore PCM volume */
    if result < 0 {
        snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV, mixL);
        snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV + 1, mixR);
    }

    result
}

/*
 * stop CSP
 */
unsafe extern "C" fn snd_sb_csp_stop(p: *mut snd_sb_csp) -> c_int {
    let result: c_int;
    let mixL: u8;
    let mixR: u8;

    if !((*p).running & SNDRV_SB_CSP_ST_RUNNING != 0) {
        return 0;
    }

    /* Mute PCM volume */
    mixL = snd_sbmixer_read((*p).chip, SB_DSP4_PCM_DEV) as u8;
    mixR = snd_sbmixer_read((*p).chip, SB_DSP4_PCM_DEV + 1) as u8;
    snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV, mixL & 0x7);
    snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV + 1, mixR & 0x7);

    if (*p).running & SNDRV_SB_CSP_ST_QSOUND != 0 {
        set_codec_parameter((*p).chip, 0xe0, 0x01);
        /* disable QSound decoder */
        set_codec_parameter((*p).chip, 0x00, 0x00);
        set_codec_parameter((*p).chip, 0x01, 0x00);

        (*p).running &= !SNDRV_SB_CSP_ST_QSOUND;
    }
    result = set_mode_register((*p).chip, 0xc0); /* c0 = STOP */

    /* restore PCM volume */
    snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV, mixL);
    snd_sbmixer_write((*p).chip, SB_DSP4_PCM_DEV + 1, mixR);

    if result == 0 {
        (*p).running &= !(SNDRV_SB_CSP_ST_PAUSED | SNDRV_SB_CSP_ST_RUNNING);
    }
    result
}

/*
 * pause CSP codec and hold DMA transfer
 */
unsafe fn snd_sb_csp_pause(p: *mut snd_sb_csp) -> c_int {
    let result: c_int;

    if !((*p).running & SNDRV_SB_CSP_ST_RUNNING != 0) {
        return -EBUSY;
    }

    result = set_codec_parameter((*p).chip, 0x80, 0xff);
    if result == 0 {
        (*p).running |= SNDRV_SB_CSP_ST_PAUSED;
    }

    result
}

/*
 * restart CSP codec and resume DMA transfer
 */
unsafe fn snd_sb_csp_restart(p: *mut snd_sb_csp) -> c_int {
    let result: c_int;

    if !((*p).running & SNDRV_SB_CSP_ST_PAUSED != 0) {
        return -EBUSY;
    }

    result = set_codec_parameter((*p).chip, 0x80, 0x00);
    if result == 0 {
        (*p).running &= !SNDRV_SB_CSP_ST_PAUSED;
    }

    result
}

/* ------------------------------ */

/*
 * QSound mixer control for PCM
 */

unsafe extern "C" fn snd_sb_qsound_switch_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let p = snd_kcontrol_chip(kcontrol) as *mut snd_sb_csp;

    (*ucontrol).value.integer.value[0] = if (*p).q_enabled != 0 { 1 } else { 0 };
    0
}

unsafe extern "C" fn snd_sb_qsound_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let p = snd_kcontrol_chip(kcontrol) as *mut snd_sb_csp;
    let change: c_int;
    let nval: u8;

    nval = ((*ucontrol).value.integer.value[0] & 0x01) as u8;
    change = ((*p).q_enabled != nval) as c_int;
    (*p).q_enabled = nval;
    change
}

unsafe extern "C" fn snd_sb_qsound_space_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = SNDRV_SB_CSP_QSOUND_MAX_RIGHT as i64;
    0
}

unsafe extern "C" fn snd_sb_qsound_space_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let p = snd_kcontrol_chip(kcontrol) as *mut snd_sb_csp;

    (*ucontrol).value.integer.value[0] = (*p).qpos_left as i64;
    (*ucontrol).value.integer.value[1] = (*p).qpos_right as i64;
    0
}

unsafe extern "C" fn snd_sb_qsound_space_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let p = snd_kcontrol_chip(kcontrol) as *mut snd_sb_csp;
    let change: c_int;
    let mut nval1: u8;
    let mut nval2: u8;

    nval1 = (*ucontrol).value.integer.value[0] as u8;
    if nval1 > SNDRV_SB_CSP_QSOUND_MAX_RIGHT {
        nval1 = SNDRV_SB_CSP_QSOUND_MAX_RIGHT;
    }
    nval2 = (*ucontrol).value.integer.value[1] as u8;
    if nval2 > SNDRV_SB_CSP_QSOUND_MAX_RIGHT {
        nval2 = SNDRV_SB_CSP_QSOUND_MAX_RIGHT;
    }
    change = ((*p).qpos_left != nval1 || (*p).qpos_right != nval2) as c_int;
    (*p).qpos_left = nval1;
    (*p).qpos_right = nval2;
    (*p).qpos_changed = change;
    change
}

static snd_sb_qsound_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"3D Control - Switch\0".as_ptr() as *const c_char,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(snd_sb_qsound_switch_get),
    put: Some(snd_sb_qsound_switch_put),
};

static snd_sb_qsound_space: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"3D Control - Space\0".as_ptr() as *const c_char,
    info: Some(snd_sb_qsound_space_info),
    get: Some(snd_sb_qsound_space_get),
    put: Some(snd_sb_qsound_space_put),
};

unsafe fn snd_sb_qsound_build(p: *mut snd_sb_csp) -> c_int {
    let card: *mut snd_card;
    let mut kctl: *mut snd_kcontrol;
    let mut err: c_int;

    if snd_BUG_ON(p.is_null()) {
        return -EINVAL;
    }

    card = (*(*p).chip).card;
    (*p).qpos_right = SNDRV_SB_CSP_QSOUND_MAX_RIGHT / 2;
    (*p).qpos_left = (*p).qpos_right;
    (*p).qpos_changed = 0;

    spin_lock_init(&mut (*p).q_lock);

    kctl = snd_ctl_new1(&snd_sb_qsound_switch, p as *mut c_void);
    err = snd_ctl_add(card, kctl);
    if err < 0 {
        snd_sb_qsound_destroy(p);
        return err;
    }
    (*p).qsound_switch = kctl;
    kctl = snd_ctl_new1(&snd_sb_qsound_space, p as *mut c_void);
    err = snd_ctl_add(card, kctl);
    if err < 0 {
        snd_sb_qsound_destroy(p);
        return err;
    }
    (*p).qsound_space = kctl;

    0
}

unsafe fn snd_sb_qsound_destroy(p: *mut snd_sb_csp) {
    let card: *mut snd_card;

    if snd_BUG_ON(p.is_null()) {
        return;
    }

    card = (*(*p).chip).card;

    snd_ctl_remove(card, (*p).qsound_switch);
    (*p).qsound_switch = ptr::null_mut();
    snd_ctl_remove(card, (*p).qsound_space);
    (*p).qsound_space = ptr::null_mut();

    /* cancel pending transfer of QSound parameters */
    (*p).qpos_changed = 0;
}

/*
 * Transfer qsound parameters to CSP,
 * function should be called from interrupt routine
 */
unsafe extern "C" fn snd_sb_csp_qsound_transfer(p: *mut snd_sb_csp) -> c_int {
    let mut err: c_int = -ENXIO;

    if (*p).running & SNDRV_SB_CSP_ST_QSOUND != 0 {
        set_codec_parameter((*p).chip, 0xe0, 0x01);
        /* left channel */
        set_codec_parameter((*p).chip, 0x00, (*p).qpos_left);
        set_codec_parameter((*p).chip, 0x02, 0x00);
        /* right channel */
        set_codec_parameter((*p).chip, 0x00, (*p).qpos_right);
        set_codec_parameter((*p).chip, 0x03, 0x00);
        err = 0;
    }
    (*p).qpos_changed = 0;
    err
}

/* ------------------------------ */

/*
 * proc interface
 */
unsafe fn init_proc_entry(p: *mut snd_sb_csp, device: c_int) -> c_int {
    let mut name = [0 as c_char; 16];

    sprintf(name.as_mut_ptr(), b"cspD%d\0".as_ptr() as *const c_char, device);
    snd_card_ro_proc_new((*(*p).chip).card, name.as_ptr(), p as *mut c_void, Some(info_read));
    0
}

unsafe extern "C" fn info_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let p = (*entry).private_data as *mut snd_sb_csp;

    snd_iprintf(
        buffer,
        b"Creative Signal Processor [v%d.%d]\n\0".as_ptr() as *const c_char,
        (*p).version >> 4,
        (*p).version & 0x0f,
    );
    snd_iprintf(
        buffer,
        b"State: %cx%c%c%c\n\0".as_ptr() as *const c_char,
        if (*p).running & SNDRV_SB_CSP_ST_QSOUND != 0 { b'Q' as c_int } else { b'-' as c_int },
        if (*p).running & SNDRV_SB_CSP_ST_PAUSED != 0 { b'P' as c_int } else { b'-' as c_int },
        if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 { b'R' as c_int } else { b'-' as c_int },
        if (*p).running & SNDRV_SB_CSP_ST_LOADED != 0 { b'L' as c_int } else { b'-' as c_int },
    );
    if (*p).running & SNDRV_SB_CSP_ST_LOADED != 0 {
        snd_iprintf(
            buffer,
            b"Codec: %s [func #%d]\n\0".as_ptr() as *const c_char,
            (*p).codec_name.as_ptr(),
            (*p).func_nr,
        );
        snd_iprintf(buffer, b"Sample rates: \0".as_ptr() as *const c_char);
        if (*p).acc_rates == SNDRV_SB_CSP_RATE_ALL {
            snd_iprintf(buffer, b"All\n\0".as_ptr() as *const c_char);
        } else {
            snd_iprintf(
                buffer,
                b"%s%s%s%s\n\0".as_ptr() as *const c_char,
                if (*p).acc_rates & SNDRV_SB_CSP_RATE_8000 != 0 { b"8000Hz \0".as_ptr() } else { b"\0".as_ptr() },
                if (*p).acc_rates & SNDRV_SB_CSP_RATE_11025 != 0 { b"11025Hz \0".as_ptr() } else { b"\0".as_ptr() },
                if (*p).acc_rates & SNDRV_SB_CSP_RATE_22050 != 0 { b"22050Hz \0".as_ptr() } else { b"\0".as_ptr() },
                if (*p).acc_rates & SNDRV_SB_CSP_RATE_44100 != 0 { b"44100Hz\0".as_ptr() } else { b"\0".as_ptr() },
            );
        }
        if (*p).mode == SNDRV_SB_CSP_MODE_QSOUND {
            snd_iprintf(
                buffer,
                b"QSound decoder %s\n\0".as_ptr() as *const c_char,
                str_enabled_disabled((*p).q_enabled as c_int),
            );
        } else {
            snd_iprintf(
                buffer,
                b"PCM format ID: 0x%x (%s/%s) [%s/%s] [%s/%s]\n\0".as_ptr() as *const c_char,
                (*p).acc_format,
                if (*p).acc_width & SNDRV_SB_CSP_SAMPLE_16BIT != 0 { b"16bit\0".as_ptr() } else { b"-\0".as_ptr() },
                if (*p).acc_width & SNDRV_SB_CSP_SAMPLE_8BIT != 0 { b"8bit\0".as_ptr() } else { b"-\0".as_ptr() },
                if (*p).acc_channels & SNDRV_SB_CSP_MONO != 0 { b"mono\0".as_ptr() } else { b"-\0".as_ptr() },
                if (*p).acc_channels & SNDRV_SB_CSP_STEREO != 0 { b"stereo\0".as_ptr() } else { b"-\0".as_ptr() },
                if (*p).mode & SNDRV_SB_CSP_MODE_DSP_WRITE != 0 { b"playback\0".as_ptr() } else { b"-\0".as_ptr() },
                if (*p).mode & SNDRV_SB_CSP_MODE_DSP_READ != 0 { b"capture\0".as_ptr() } else { b"-\0".as_ptr() },
            );
        }
    }
    if (*p).running & SNDRV_SB_CSP_ST_AUTO != 0 {
        snd_iprintf(buffer, b"Autoloaded Mu-Law, A-Law or Ima-ADPCM hardware codec\n\0".as_ptr() as *const c_char);
    }
    if (*p).running & SNDRV_SB_CSP_ST_RUNNING != 0 {
        snd_iprintf(
            buffer,
            b"Processing %dbit %s PCM samples\n\0".as_ptr() as *const c_char,
            if (*p).run_width & SNDRV_SB_CSP_SAMPLE_16BIT != 0 { 16 } else { 8 },
            if (*p).run_channels & SNDRV_SB_CSP_MONO != 0 { b"mono\0".as_ptr() } else { b"stereo\0".as_ptr() },
        );
    }
    if (*p).running & SNDRV_SB_CSP_ST_QSOUND != 0 {
        snd_iprintf(
            buffer,
            b"Qsound position: left = 0x%x, right = 0x%x\n\0".as_ptr() as *const c_char,
            (*p).qpos_left as c_int,
            (*p).qpos_right as c_int,
        );
    }
}

/* */

/* EXPORT_SYMBOL(snd_sb_csp_new); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
