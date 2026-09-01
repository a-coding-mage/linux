// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Sound Core PDAudioCF soundcards
 *
 * PCM part
 *
 * Copyright (c) 2003 by Jaroslav Kysela <perex@perex.cz>
 */

// Dependencies from <linux/delay.h>, <sound/core.h>, <sound/asoundef.h>,
// and "pdaudiocf.h" are expected to be supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u16 = u16;
type snd_pcm_uframes_t = c_uint;

extern "C" {
    static pdacf_pcm_capture_hw: snd_pcm_hardware;

    fn inw(port: c_uint) -> c_uint;
    fn snd_pcm_substream_chip(subs: *mut snd_pcm_substream) -> *mut snd_pdacf;
    fn snd_ak4117_check_rate_and_errors(ak4117: *mut ak4117, flags: c_int) -> c_int;
    fn pdacf_reg_read(chip: *mut snd_pdacf, reg: c_uint) -> u16;
    fn pdacf_reg_write(chip: *mut snd_pdacf, reg: c_uint, val: u16);
    fn snd_pcm_format_little_endian(format: c_int) -> c_int;
    fn snd_pcm_format_big_endian(format: c_int) -> c_int;
    fn snd_pcm_format_unsigned(format: c_int) -> c_int;
    fn snd_ak4117_reg_write(ak4117: *mut ak4117, reg: c_uint, mask: c_uint, val: c_uint);
    fn pdacf_reinit(chip: *mut snd_pdacf, resume: c_int);
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_ak4117_build(ak4117: *mut ak4117, capture_substream: *mut snd_pcm_substream) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub shortname: *const c_char,
}

#[repr(C)]
pub struct ak4117 {
    pub rcs0: c_uint,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub private_data: *mut c_void,
    pub rate: c_uint,
    pub channels: c_uint,
    pub format: c_int,
    pub buffer_size: snd_pcm_uframes_t,
    pub period_size: snd_pcm_uframes_t,
    pub dma_area: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_stream {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub nonatomic: bool,
    pub name: *mut c_char,
    pub streams: [snd_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct snd_pdacf {
    pub port: c_uint,
    pub chip_status: c_uint,
    pub reg_lock: mutex,
    pub pcm_hwptr: snd_pcm_uframes_t,
    pub pcm_tdone: c_uint,
    pub pcm_running: c_int,
    pub ak4117: *mut ak4117,
    pub pcm_channels: c_uint,
    pub pcm_little: bool,
    pub pcm_swab: bool,
    pub pcm_xor: c_uint,
    pub pcm_sample: c_uint,
    pub pcm_frame: c_uint,
    pub pcm_size: snd_pcm_uframes_t,
    pub pcm_period: snd_pcm_uframes_t,
    pub pcm_area: *mut c_void,
    pub pcm_substream: *mut snd_pcm_substream,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
}

extern "C" {
    static PDAUDIOCF_REG_RDP: c_uint;
    static PDAUDIOCF_REG_WDP: c_uint;
    static PDAUDIOCF_REG_MD: c_uint;
    static PDAUDIOCF_REG_SCR: c_uint;
    static PDAUDIOCF_REG_IER: c_uint;
    static PDAUDIOCF_STAT_IS_STALE: c_uint;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static PDAUDIOCF_RECORD: u16;
    static AK4117_CHECK_NO_STAT: c_int;
    static AK4117_CHECK_NO_RATE: c_int;
    static AK4117_UNLCK: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: c_int;
    static SNDRV_PCM_FORMAT_S16_BE: c_int;
    static SNDRV_PCM_FORMAT_S24_3LE: c_int;
    static SNDRV_PCM_FORMAT_S24_3BE: c_int;
    static PDAUDIOCF_DATAFMT0: u16;
    static PDAUDIOCF_DATAFMT1: u16;
    static AK4117_DIF_16R: c_uint;
    static AK4117_DIF_24R: c_uint;
    static AK4117_REG_IO: c_uint;
    static AK4117_DIF2: c_uint;
    static AK4117_DIF1: c_uint;
    static AK4117_DIF0: c_uint;
    static PDAUDIOCF_IRQLVLEN1: u16;
    static PDAUDIOCF_IRQLVLEN0: u16;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_BATCH: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S16_BE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3BE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_BE: u64;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_DMA_TYPE_VMALLOC: c_int;
    static EIO: c_int;
    static EBUSY: c_int;
    static EINVAL: c_int;
}

/*
 * clear the SRAM contents
 */
unsafe extern "C" fn pdacf_pcm_clear_sram(chip: *mut snd_pdacf) -> c_int {
    let mut max_loop: c_int = 64 * 1024;

    while inw((*chip).port + PDAUDIOCF_REG_RDP) != inw((*chip).port + PDAUDIOCF_REG_WDP) {
        if {
            let old = max_loop;
            max_loop -= 1;
            old < 0
        } {
            return -EIO;
        }
        inw((*chip).port + PDAUDIOCF_REG_MD);
    }
    0
}

/*
 * pdacf_pcm_trigger - trigger callback for capture
 */
unsafe extern "C" fn pdacf_pcm_trigger(subs: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut snd_pdacf = snd_pcm_substream_chip(subs);
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let mut inc: c_int;
    let mut ret: c_int = 0;
    let mut rate: c_int;
    let mut mask: u16;
    let mut val: u16;
    let mut tmp: u16;

    if ((*chip).chip_status & PDAUDIOCF_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    if cmd == SNDRV_PCM_TRIGGER_START {
        (*chip).pcm_hwptr = 0;
        (*chip).pcm_tdone = 0;
        mask = 0;
        val = PDAUDIOCF_RECORD;
        inc = 1;
        rate = snd_ak4117_check_rate_and_errors(
            (*chip).ak4117,
            AK4117_CHECK_NO_STAT | AK4117_CHECK_NO_RATE,
        );
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE || cmd == SNDRV_PCM_TRIGGER_RESUME {
        mask = 0;
        val = PDAUDIOCF_RECORD;
        inc = 1;
        rate = snd_ak4117_check_rate_and_errors(
            (*chip).ak4117,
            AK4117_CHECK_NO_STAT | AK4117_CHECK_NO_RATE,
        );
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
    {
        mask = PDAUDIOCF_RECORD;
        val = 0;
        inc = -1;
        rate = 0;
    } else {
        return -EINVAL;
    }

    mutex_lock(&mut (*chip).reg_lock);
    loop {
        (*chip).pcm_running += inc;
        tmp = pdacf_reg_read(chip, PDAUDIOCF_REG_SCR);
        if (*chip).pcm_running != 0 {
            if ((*(*chip).ak4117).rcs0 & AK4117_UNLCK) != 0 || (*runtime).rate != rate as c_uint {
                (*chip).pcm_running -= inc;
                ret = -EIO;
                break;
            }
        }
        tmp &= !mask;
        tmp |= val;
        pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, tmp);
        break;
    }
    mutex_unlock(&mut (*chip).reg_lock);
    snd_ak4117_check_rate_and_errors((*chip).ak4117, AK4117_CHECK_NO_RATE);
    ret
}

/*
 * pdacf_pcm_prepare - prepare callback for playback and capture
 */
unsafe extern "C" fn pdacf_pcm_prepare(subs: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_pdacf = snd_pcm_substream_chip(subs);
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let mut val: u16;
    let mut nval: u16;
    let mut aval: u16;

    if ((*chip).chip_status & PDAUDIOCF_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    (*chip).pcm_channels = (*runtime).channels;

    (*chip).pcm_little = snd_pcm_format_little_endian((*runtime).format) > 0;
    // Original C selected this branch under SNDRV_LITTLE_ENDIAN.
    (*chip).pcm_swab = snd_pcm_format_big_endian((*runtime).format) > 0;

    if snd_pcm_format_unsigned((*runtime).format) != 0 {
        (*chip).pcm_xor = 0x80008000;
    }

    if pdacf_pcm_clear_sram(chip) < 0 {
        return -EIO;
    }

    val = pdacf_reg_read(chip, PDAUDIOCF_REG_SCR);
    nval = val;
    nval &= !(PDAUDIOCF_DATAFMT0 | PDAUDIOCF_DATAFMT1);
    if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE || (*runtime).format == SNDRV_PCM_FORMAT_S16_BE {
    } else {
        /* 24-bit */
        nval |= PDAUDIOCF_DATAFMT0 | PDAUDIOCF_DATAFMT1;
    }
    aval = 0;
    (*chip).pcm_sample = 4;
    if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE || (*runtime).format == SNDRV_PCM_FORMAT_S16_BE {
        aval = AK4117_DIF_16R as u16;
        (*chip).pcm_frame = 2;
        (*chip).pcm_sample = 2;
    } else {
        if (*runtime).format == SNDRV_PCM_FORMAT_S24_3LE || (*runtime).format == SNDRV_PCM_FORMAT_S24_3BE {
            (*chip).pcm_sample = 3;
        }
        /* 24-bit */
        aval = AK4117_DIF_24R as u16;
        (*chip).pcm_frame = 3;
        (*chip).pcm_xor &= 0xffff0000;
    }

    if val != nval {
        snd_ak4117_reg_write(
            (*chip).ak4117,
            AK4117_REG_IO,
            AK4117_DIF2 | AK4117_DIF1 | AK4117_DIF0,
            aval as c_uint,
        );
        pdacf_reg_write(chip, PDAUDIOCF_REG_SCR, nval);
    }

    val = pdacf_reg_read(chip, PDAUDIOCF_REG_IER);
    val &= !PDAUDIOCF_IRQLVLEN1;
    val |= PDAUDIOCF_IRQLVLEN0;
    pdacf_reg_write(chip, PDAUDIOCF_REG_IER, val);

    (*chip).pcm_size = (*runtime).buffer_size;
    (*chip).pcm_period = (*runtime).period_size;
    (*chip).pcm_area = (*runtime).dma_area;

    0
}

/*
 * capture hw information
 */

#[no_mangle]
pub static pdacf_pcm_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BATCH,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S16_BE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S24_3BE
        | SNDRV_PCM_FMTBIT_S32_LE
        | SNDRV_PCM_FMTBIT_S32_BE,
    rates: SNDRV_PCM_RATE_32000
        | SNDRV_PCM_RATE_44100
        | SNDRV_PCM_RATE_48000
        | SNDRV_PCM_RATE_88200
        | SNDRV_PCM_RATE_96000
        | SNDRV_PCM_RATE_176400
        | SNDRV_PCM_RATE_192000,
    rate_min: 32000,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 512 * 1024,
    period_bytes_min: 8 * 1024,
    period_bytes_max: 64 * 1024,
    periods_min: 2,
    periods_max: 128,
    fifo_size: 0,
};

/*
 * pdacf_pcm_capture_open - open callback for capture
 */
unsafe extern "C" fn pdacf_pcm_capture_open(subs: *mut snd_pcm_substream) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let chip: *mut snd_pdacf = snd_pcm_substream_chip(subs);

    if ((*chip).chip_status & PDAUDIOCF_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    (*runtime).hw = pdacf_pcm_capture_hw;
    (*runtime).private_data = chip as *mut c_void;
    (*chip).pcm_substream = subs;

    0
}

/*
 * pdacf_pcm_capture_close - close callback for capture
 */
unsafe extern "C" fn pdacf_pcm_capture_close(subs: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_pdacf = snd_pcm_substream_chip(subs);

    if chip.is_null() {
        return -EINVAL;
    }
    pdacf_reinit(chip, 0);
    (*chip).pcm_substream = ptr::null_mut();
    0
}

/*
 * pdacf_pcm_capture_pointer - pointer callback for capture
 */
unsafe extern "C" fn pdacf_pcm_capture_pointer(
    subs: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip: *mut snd_pdacf = snd_pcm_substream_chip(subs);
    (*chip).pcm_hwptr
}

/*
 * operators for PCM capture
 */
#[no_mangle]
pub static pdacf_pcm_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(pdacf_pcm_capture_open),
    close: Some(pdacf_pcm_capture_close),
    prepare: Some(pdacf_pcm_prepare),
    trigger: Some(pdacf_pcm_trigger),
    pointer: Some(pdacf_pcm_capture_pointer),
};

/*
 * snd_pdacf_pcm_new - create and initialize a pcm
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pdacf_pcm_new(chip: *mut snd_pdacf) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let mut err: c_int;

    err = snd_pcm_new(
        (*chip).card,
        b"PDAudioCF\0".as_ptr() as *const c_char,
        0,
        0,
        1,
        &mut pcm,
    );
    if err < 0 {
        return err;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &pdacf_pcm_capture_ops);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name, (*(*chip).card).shortname);
    (*chip).pcm = pcm;

    err = snd_ak4117_build(
        (*chip).ak4117,
        (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream,
    );
    if err < 0 {
        return err;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
