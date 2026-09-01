// SPDX-License-Identifier: GPL-2.0-only
/*
 * C-Media CMI8788 driver - PCM code
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

type snd_pcm_uframes_t = usize;

/* Dependencies from linux/pci.h, sound/*.h, and oxygen.h are external. */

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
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
    pub fifo_size: usize,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub private_data: *mut c_void,
    pub hw: snd_pcm_hardware,
    pub dma_addr: usize,
    pub no_period_wakeup: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_volatile {
    pub access: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub vd: *mut snd_kcontrol_volatile,
    pub id: snd_ctl_elem_id,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_stream {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub name: *mut c_char,
    pub streams: [snd_pcm_stream; 2],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct oxygen_model {
    pub device_config: c_uint,
    pub dac_channels_pcm: c_uint,
    pub pcm_hardware_filter: Option<unsafe extern "C" fn(c_uint, *mut snd_pcm_hardware)>,
    pub adc_mclks: c_uint,
    pub dac_mclks: c_uint,
    pub adc_i2s_format: c_uint,
    pub dac_i2s_format: c_uint,
    pub set_adc_params: unsafe extern "C" fn(*mut oxygen, *mut snd_pcm_hw_params),
    pub set_dac_params: unsafe extern "C" fn(*mut oxygen, *mut snd_pcm_hw_params),
}

#[repr(C)]
pub struct oxygen {
    pub has_ac97_1: c_int,
    pub model: oxygen_model,
    pub streams: [*mut snd_pcm_substream; PCM_COUNT],
    pub mutex: c_void,
    pub pcm_active: c_uint,
    pub spdif_pcm_bits: c_uint,
    pub spdif_bits: c_uint,
    pub controls: [*mut snd_kcontrol; CONTROL_COUNT],
    pub card: *mut snd_card,
    pub reg_lock: c_void,
    pub interrupt_mask: c_uint,
    pub pcm_running: c_uint,
    pub pci: *mut pci_dev,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

const PCM_A: usize = 0;
const PCM_B: usize = 1;
const PCM_C: usize = 2;
const PCM_SPDIF: usize = 3;
const PCM_MULTICH: usize = 4;
const PCM_AC97: usize = 5;
const PCM_COUNT: usize = 6;

const CONTROL_SPDIF_PCM: usize = 0;
const CONTROL_COUNT: usize = 1;

extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_INFO_SYNC_START: c_uint;
    static SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_64000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
}

const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_CTL_ELEM_ACCESS_INACTIVE: c_uint = 1 << 8;
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 1;
const SNDRV_CTL_EVENT_MASK_INFO: c_uint = 2;
const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 1;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const EINVAL: c_int = 22;
const SNDRV_PCM_STREAM_PLAYBACK: usize = 0;
const SNDRV_PCM_STREAM_CAPTURE: usize = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;

const OXYGEN_FORMAT_24: c_uint = 1;
const OXYGEN_FORMAT_16: c_uint = 0;
const OXYGEN_RATE_32000: c_uint = 0;
const OXYGEN_RATE_44100: c_uint = 1;
const OXYGEN_RATE_48000: c_uint = 2;
const OXYGEN_RATE_64000: c_uint = 3;
const OXYGEN_RATE_88200: c_uint = 4;
const OXYGEN_RATE_96000: c_uint = 5;
const OXYGEN_RATE_176400: c_uint = 6;
const OXYGEN_RATE_192000: c_uint = 7;
const OXYGEN_I2S_BITS_24: c_uint = 1 << 4;
const OXYGEN_I2S_BITS_16: c_uint = 0;
const OXYGEN_PLAY_CHANNELS_2: c_uint = 0;
const OXYGEN_PLAY_CHANNELS_4: c_uint = 1;
const OXYGEN_PLAY_CHANNELS_6: c_uint = 2;
const OXYGEN_PLAY_CHANNELS_8: c_uint = 3;

const OXYGEN_DMA_A_ADDRESS: c_uint = 0;
const OXYGEN_DMA_B_ADDRESS: c_uint = 0;
const OXYGEN_DMA_C_ADDRESS: c_uint = 0;
const OXYGEN_DMA_SPDIF_ADDRESS: c_uint = 0;
const OXYGEN_DMA_MULTICH_ADDRESS: c_uint = 0;
const OXYGEN_DMA_AC97_ADDRESS: c_uint = 0;
const OXYGEN_DMA_MULTICH_COUNT: c_uint = 0;
const OXYGEN_DMA_MULTICH_TCOUNT: c_uint = 0;
const OXYGEN_REC_FORMAT: c_uint = 0;
const OXYGEN_REC_FORMAT_A_SHIFT: c_uint = 0;
const OXYGEN_REC_FORMAT_A_MASK: c_uint = 0;
const OXYGEN_REC_FORMAT_B_SHIFT: c_uint = 0;
const OXYGEN_REC_FORMAT_B_MASK: c_uint = 0;
const OXYGEN_REC_FORMAT_C_SHIFT: c_uint = 0;
const OXYGEN_REC_FORMAT_C_MASK: c_uint = 0;
const OXYGEN_I2S_A_FORMAT: c_uint = 0;
const OXYGEN_I2S_B_FORMAT: c_uint = 0;
const OXYGEN_I2S_C_FORMAT: c_uint = 0;
const OXYGEN_I2S_MULTICH_FORMAT: c_uint = 0;
const OXYGEN_I2S_RATE_MASK: c_uint = 0;
const OXYGEN_I2S_FORMAT_MASK: c_uint = 0;
const OXYGEN_I2S_MCLK_MASK: c_uint = 0;
const OXYGEN_I2S_BITS_MASK: c_uint = 0;
const OXYGEN_SPDIF_CONTROL: c_uint = 0;
const OXYGEN_SPDIF_OUT_ENABLE: c_uint = 0;
const OXYGEN_PLAY_FORMAT: c_uint = 0;
const OXYGEN_SPDIF_FORMAT_SHIFT: c_uint = 0;
const OXYGEN_SPDIF_FORMAT_MASK: c_uint = 0;
const OXYGEN_SPDIF_OUT_RATE_SHIFT: c_uint = 0;
const OXYGEN_SPDIF_OUT_RATE_MASK: c_uint = 0;
const OXYGEN_PLAY_CHANNELS: c_uint = 0;
const OXYGEN_PLAY_CHANNELS_MASK: c_uint = 0;
const OXYGEN_MULTICH_FORMAT_SHIFT: c_uint = 0;
const OXYGEN_MULTICH_FORMAT_MASK: c_uint = 0;
const OXYGEN_INTERRUPT_MASK: c_uint = 0;
const OXYGEN_DMA_FLUSH: c_uint = 0;
const OXYGEN_DMA_STATUS: c_uint = 0;
const OXYGEN_DMA_PAUSE: c_uint = 0;
const OXYGEN_REC_ROUTING: c_uint = 0;
const OXYGEN_REC_B_ROUTE_AC97_1: c_uint = 0;
const OXYGEN_REC_B_ROUTE_MASK: c_uint = 0;
const OXYGEN_REC_C_ROUTE_I2S_ADC_3: c_uint = 0;
const OXYGEN_REC_C_ROUTE_MASK: c_uint = 0;

const PLAYBACK_0_TO_I2S: c_uint = 1 << 0;
const CAPTURE_0_FROM_I2S_1: c_uint = 1 << 1;
const CAPTURE_0_FROM_I2S_2: c_uint = 1 << 2;
const PLAYBACK_1_TO_SPDIF: c_uint = 1 << 3;
const CAPTURE_1_FROM_SPDIF: c_uint = 1 << 4;
const PLAYBACK_2_TO_AC97_1: c_uint = 1 << 5;
const CAPTURE_2_FROM_AC97_1: c_uint = 1 << 6;
const CAPTURE_2_FROM_I2S_2: c_uint = 1 << 7;
const CAPTURE_3_FROM_I2S_3: c_uint = 1 << 8;

extern "C" {
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut oxygen;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, step: c_uint) -> c_int;
    fn snd_pcm_hw_constraint_msbits(runtime: *mut snd_pcm_runtime, cond: c_uint, width: c_uint, msbits: c_uint) -> c_int;
    fn snd_pcm_set_sync(substream: *mut snd_pcm_substream);
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *const snd_ctl_elem_id);
    fn oxygen_update_spdif_source(chip: *mut oxygen);
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_buffer_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn oxygen_write32(chip: *mut oxygen, reg: c_uint, value: u32);
    fn oxygen_write16(chip: *mut oxygen, reg: c_uint, value: u16);
    fn oxygen_write8_masked(chip: *mut oxygen, reg: c_uint, value: c_uint, mask: c_uint);
    fn oxygen_write16_masked(chip: *mut oxygen, reg: c_uint, value: c_uint, mask: c_uint);
    fn oxygen_write32_masked(chip: *mut oxygen, reg: c_uint, value: c_uint, mask: c_uint);
    fn oxygen_clear_bits32(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_set_bits8(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_clear_bits8(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn oxygen_write8(chip: *mut oxygen, reg: c_uint, value: c_uint);
    fn snd_pcm_trigger_done(s: *mut snd_pcm_substream, master: *mut snd_pcm_substream);
    fn oxygen_read32(chip: *mut oxygen, reg: c_uint) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> snd_pcm_uframes_t;
    fn snd_pcm_new(card: *mut snd_card, id: *const c_char, device: c_int, playback_count: c_int, capture_count: c_int, rpcm: *mut *mut snd_pcm) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: usize, ops: *const snd_pcm_ops);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snd_pcm_set_managed_buffer(substream: *mut snd_pcm_substream, ty: c_int, dev: *mut device, size: usize, max: usize);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, size: usize, max: usize);
    fn oxygen_update_dac_routing(chip: *mut oxygen);
}

const fn OXYGEN_I2S_MCLK(value: c_uint) -> u16 {
    value as u16
}

/* most DMA channels have a 16-bit counter for 32-bit words */
const BUFFER_BYTES_MAX: usize = (1usize << 16) * 4;
/* the multichannel DMA channel has a 24-bit counter */
const BUFFER_BYTES_MAX_MULTICH: usize = (1usize << 24) * 4;

const FIFO_BYTES: usize = 256;
const FIFO_BYTES_MULTICH: usize = 1024;

const PERIOD_BYTES_MIN: usize = 64;

const DEFAULT_BUFFER_BYTES: usize = BUFFER_BYTES_MAX / 2;
const DEFAULT_BUFFER_BYTES_MULTICH: usize = 1024 * 1024;

static oxygen_stereo_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe { SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP },
    formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    rates: unsafe { SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_64000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000 },
    rate_min: 32000,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: BUFFER_BYTES_MAX,
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: BUFFER_BYTES_MAX,
    periods_min: 1,
    periods_max: (BUFFER_BYTES_MAX / PERIOD_BYTES_MIN) as c_uint,
    fifo_size: FIFO_BYTES,
};

static oxygen_multichannel_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe { SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP },
    formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    rates: unsafe { SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_64000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000 },
    rate_min: 32000,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 8,
    buffer_bytes_max: BUFFER_BYTES_MAX_MULTICH,
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: BUFFER_BYTES_MAX_MULTICH,
    periods_min: 1,
    periods_max: (BUFFER_BYTES_MAX_MULTICH / PERIOD_BYTES_MIN) as c_uint,
    fifo_size: FIFO_BYTES_MULTICH,
};

static oxygen_ac97_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe { SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_SYNC_START | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP },
    formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    rates: unsafe { SNDRV_PCM_RATE_48000 },
    rate_min: 48000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: BUFFER_BYTES_MAX,
    period_bytes_min: PERIOD_BYTES_MIN,
    period_bytes_max: BUFFER_BYTES_MAX,
    periods_min: 1,
    periods_max: (BUFFER_BYTES_MAX / PERIOD_BYTES_MIN) as c_uint,
    fifo_size: FIFO_BYTES,
};

static oxygen_hardware: [*const snd_pcm_hardware; PCM_COUNT] = [
    &oxygen_stereo_hardware,
    &oxygen_stereo_hardware,
    &oxygen_stereo_hardware,
    &oxygen_stereo_hardware,
    &oxygen_multichannel_hardware,
    &oxygen_ac97_hardware,
];

#[inline]
unsafe fn oxygen_substream_channel(substream: *mut snd_pcm_substream) -> c_uint {
    (*(*substream).runtime).private_data as usize as c_uint
}

unsafe fn oxygen_open(substream: *mut snd_pcm_substream, channel: c_uint) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;

    (*runtime).private_data = channel as usize as *mut c_void;
    if channel as usize == PCM_B
        && (*chip).has_ac97_1 != 0
        && ((*chip).model.device_config & CAPTURE_2_FROM_AC97_1) != 0
    {
        (*runtime).hw = oxygen_ac97_hardware;
    } else {
        (*runtime).hw = *oxygen_hardware[channel as usize];
    }
    match channel as usize {
        PCM_C => {
            if ((*chip).model.device_config & CAPTURE_1_FROM_SPDIF) != 0 {
                (*runtime).hw.rates &= !(SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_64000);
                (*runtime).hw.rate_min = 44100;
            }
            (*runtime).hw.fifo_size = 0;
        }
        PCM_A | PCM_B => {
            (*runtime).hw.fifo_size = 0;
        }
        PCM_MULTICH => {
            (*runtime).hw.channels_max = (*chip).model.dac_channels_pcm;
        }
        _ => {}
    }
    if let Some(filter) = (*chip).model.pcm_hardware_filter {
        filter(channel, &mut (*runtime).hw);
    }
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 32);
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 32);
    if err < 0 {
        return err;
    }
    if ((*runtime).hw.formats & SNDRV_PCM_FMTBIT_S32_LE) != 0 {
        err = snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24);
        if err < 0 {
            return err;
        }
    }
    if (*runtime).hw.channels_max > 2 {
        err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
        if err < 0 {
            return err;
        }
    }
    snd_pcm_set_sync(substream);
    (*chip).streams[channel as usize] = substream;

    /* guard(mutex)(&chip->mutex); */
    (*chip).pcm_active |= 1 << channel;
    if channel as usize == PCM_SPDIF {
        (*chip).spdif_pcm_bits = (*chip).spdif_bits;
        (*(*(*chip).controls[CONTROL_SPDIF_PCM]).vd.add(0)).access &= !SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        snd_ctl_notify(
            (*chip).card,
            SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO,
            &(*(*chip).controls[CONTROL_SPDIF_PCM]).id,
        );
    }

    0
}

unsafe extern "C" fn oxygen_rec_a_open(substream: *mut snd_pcm_substream) -> c_int {
    oxygen_open(substream, PCM_A as c_uint)
}

unsafe extern "C" fn oxygen_rec_b_open(substream: *mut snd_pcm_substream) -> c_int {
    oxygen_open(substream, PCM_B as c_uint)
}

unsafe extern "C" fn oxygen_rec_c_open(substream: *mut snd_pcm_substream) -> c_int {
    oxygen_open(substream, PCM_C as c_uint)
}

unsafe extern "C" fn oxygen_spdif_open(substream: *mut snd_pcm_substream) -> c_int {
    oxygen_open(substream, PCM_SPDIF as c_uint)
}

unsafe extern "C" fn oxygen_multich_open(substream: *mut snd_pcm_substream) -> c_int {
    oxygen_open(substream, PCM_MULTICH as c_uint)
}

unsafe extern "C" fn oxygen_ac97_open(substream: *mut snd_pcm_substream) -> c_int {
    oxygen_open(substream, PCM_AC97 as c_uint)
}

unsafe extern "C" fn oxygen_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let channel = oxygen_substream_channel(substream);

    /* guard(mutex)(&chip->mutex); */
    (*chip).pcm_active &= !(1 << channel);
    if channel as usize == PCM_SPDIF {
        (*(*(*chip).controls[CONTROL_SPDIF_PCM]).vd.add(0)).access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
        snd_ctl_notify(
            (*chip).card,
            SNDRV_CTL_EVENT_MASK_VALUE | SNDRV_CTL_EVENT_MASK_INFO,
            &(*(*chip).controls[CONTROL_SPDIF_PCM]).id,
        );
    }
    if channel as usize == PCM_SPDIF || channel as usize == PCM_MULTICH {
        oxygen_update_spdif_source(chip);
    }

    (*chip).streams[channel as usize] = core::ptr::null_mut();
    0
}

unsafe fn oxygen_format(hw_params: *mut snd_pcm_hw_params) -> c_uint {
    if params_format(hw_params) == SNDRV_PCM_FORMAT_S32_LE {
        OXYGEN_FORMAT_24
    } else {
        OXYGEN_FORMAT_16
    }
}

unsafe fn oxygen_rate(hw_params: *mut snd_pcm_hw_params) -> c_uint {
    match params_rate(hw_params) {
        32000 => OXYGEN_RATE_32000,
        44100 => OXYGEN_RATE_44100,
        64000 => OXYGEN_RATE_64000,
        88200 => OXYGEN_RATE_88200,
        96000 => OXYGEN_RATE_96000,
        176400 => OXYGEN_RATE_176400,
        192000 => OXYGEN_RATE_192000,
        _ => OXYGEN_RATE_48000,
    }
}

unsafe fn oxygen_i2s_bits(hw_params: *mut snd_pcm_hw_params) -> c_uint {
    if params_format(hw_params) == SNDRV_PCM_FORMAT_S32_LE {
        OXYGEN_I2S_BITS_24
    } else {
        OXYGEN_I2S_BITS_16
    }
}

unsafe fn oxygen_play_channels(hw_params: *mut snd_pcm_hw_params) -> c_uint {
    match params_channels(hw_params) {
        4 => OXYGEN_PLAY_CHANNELS_4,
        6 => OXYGEN_PLAY_CHANNELS_6,
        8 => OXYGEN_PLAY_CHANNELS_8,
        _ => OXYGEN_PLAY_CHANNELS_2,
    }
}

static channel_base_registers: [c_uint; PCM_COUNT] = [
    OXYGEN_DMA_A_ADDRESS,
    OXYGEN_DMA_B_ADDRESS,
    OXYGEN_DMA_C_ADDRESS,
    OXYGEN_DMA_SPDIF_ADDRESS,
    OXYGEN_DMA_MULTICH_ADDRESS,
    OXYGEN_DMA_AC97_ADDRESS,
];

unsafe extern "C" fn oxygen_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let channel = oxygen_substream_channel(substream);

    oxygen_write32(
        chip,
        channel_base_registers[channel as usize],
        (*(*substream).runtime).dma_addr as u32,
    );
    if channel as usize == PCM_MULTICH {
        oxygen_write32(chip, OXYGEN_DMA_MULTICH_COUNT, params_buffer_bytes(hw_params) / 4 - 1);
        oxygen_write32(chip, OXYGEN_DMA_MULTICH_TCOUNT, params_period_bytes(hw_params) / 4 - 1);
    } else {
        oxygen_write16(
            chip,
            channel_base_registers[channel as usize] + 4,
            (params_buffer_bytes(hw_params) / 4 - 1) as u16,
        );
        oxygen_write16(
            chip,
            channel_base_registers[channel as usize] + 6,
            (params_period_bytes(hw_params) / 4 - 1) as u16,
        );
    }
    0
}

unsafe fn get_mclk(
    chip: *mut oxygen,
    channel: c_uint,
    params: *mut snd_pcm_hw_params,
) -> u16 {
    let mclks: c_uint;
    let shift: c_uint;

    if channel as usize == PCM_MULTICH {
        mclks = (*chip).model.dac_mclks;
    } else {
        mclks = (*chip).model.adc_mclks;
    }

    if params_rate(params) <= 48000 {
        shift = 0;
    } else if params_rate(params) <= 96000 {
        shift = 2;
    } else {
        shift = 4;
    }

    OXYGEN_I2S_MCLK(mclks >> shift)
}

unsafe extern "C" fn oxygen_rec_a_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;

    err = oxygen_hw_params(substream, hw_params);
    if err < 0 {
        return err;
    }

    /* scoped_guard(spinlock_irq, &chip->reg_lock) */
    oxygen_write8_masked(
        chip,
        OXYGEN_REC_FORMAT,
        oxygen_format(hw_params) << OXYGEN_REC_FORMAT_A_SHIFT,
        OXYGEN_REC_FORMAT_A_MASK,
    );
    oxygen_write16_masked(
        chip,
        OXYGEN_I2S_A_FORMAT,
        oxygen_rate(hw_params)
            | (*chip).model.adc_i2s_format
            | get_mclk(chip, PCM_A as c_uint, hw_params) as c_uint
            | oxygen_i2s_bits(hw_params),
        OXYGEN_I2S_RATE_MASK | OXYGEN_I2S_FORMAT_MASK | OXYGEN_I2S_MCLK_MASK | OXYGEN_I2S_BITS_MASK,
    );

    /* guard(mutex)(&chip->mutex); */
    ((*chip).model.set_adc_params)(chip, hw_params);
    0
}

unsafe extern "C" fn oxygen_rec_b_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let is_ac97: c_int;
    let mut err: c_int;

    err = oxygen_hw_params(substream, hw_params);
    if err < 0 {
        return err;
    }

    is_ac97 = ((*chip).has_ac97_1 != 0 && ((*chip).model.device_config & CAPTURE_2_FROM_AC97_1) != 0) as c_int;

    /* scoped_guard(spinlock_irq, &chip->reg_lock) */
    oxygen_write8_masked(
        chip,
        OXYGEN_REC_FORMAT,
        oxygen_format(hw_params) << OXYGEN_REC_FORMAT_B_SHIFT,
        OXYGEN_REC_FORMAT_B_MASK,
    );
    if is_ac97 == 0 {
        oxygen_write16_masked(
            chip,
            OXYGEN_I2S_B_FORMAT,
            oxygen_rate(hw_params)
                | (*chip).model.adc_i2s_format
                | get_mclk(chip, PCM_B as c_uint, hw_params) as c_uint
                | oxygen_i2s_bits(hw_params),
            OXYGEN_I2S_RATE_MASK | OXYGEN_I2S_FORMAT_MASK | OXYGEN_I2S_MCLK_MASK | OXYGEN_I2S_BITS_MASK,
        );
    }

    if is_ac97 == 0 {
        /* guard(mutex)(&chip->mutex); */
        ((*chip).model.set_adc_params)(chip, hw_params);
    }
    0
}

unsafe extern "C" fn oxygen_rec_c_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let is_spdif: bool;
    let mut err: c_int;

    err = oxygen_hw_params(substream, hw_params);
    if err < 0 {
        return err;
    }

    is_spdif = ((*chip).model.device_config & CAPTURE_1_FROM_SPDIF) != 0;

    /* scoped_guard(spinlock_irq, &chip->reg_lock) */
    oxygen_write8_masked(
        chip,
        OXYGEN_REC_FORMAT,
        oxygen_format(hw_params) << OXYGEN_REC_FORMAT_C_SHIFT,
        OXYGEN_REC_FORMAT_C_MASK,
    );
    if !is_spdif {
        oxygen_write16_masked(
            chip,
            OXYGEN_I2S_C_FORMAT,
            oxygen_rate(hw_params)
                | (*chip).model.adc_i2s_format
                | get_mclk(chip, PCM_B as c_uint, hw_params) as c_uint
                | oxygen_i2s_bits(hw_params),
            OXYGEN_I2S_RATE_MASK | OXYGEN_I2S_FORMAT_MASK | OXYGEN_I2S_MCLK_MASK | OXYGEN_I2S_BITS_MASK,
        );
    }

    if !is_spdif {
        /* guard(mutex)(&chip->mutex); */
        ((*chip).model.set_adc_params)(chip, hw_params);
    }
    0
}

unsafe extern "C" fn oxygen_spdif_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;

    err = oxygen_hw_params(substream, hw_params);
    if err < 0 {
        return err;
    }

    /* guard(mutex)(&chip->mutex); guard(spinlock_irq)(&chip->reg_lock); */
    oxygen_clear_bits32(chip, OXYGEN_SPDIF_CONTROL, OXYGEN_SPDIF_OUT_ENABLE);
    oxygen_write8_masked(
        chip,
        OXYGEN_PLAY_FORMAT,
        oxygen_format(hw_params) << OXYGEN_SPDIF_FORMAT_SHIFT,
        OXYGEN_SPDIF_FORMAT_MASK,
    );
    oxygen_write32_masked(
        chip,
        OXYGEN_SPDIF_CONTROL,
        oxygen_rate(hw_params) << OXYGEN_SPDIF_OUT_RATE_SHIFT,
        OXYGEN_SPDIF_OUT_RATE_MASK,
    );
    oxygen_update_spdif_source(chip);
    0
}

unsafe extern "C" fn oxygen_multich_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;

    err = oxygen_hw_params(substream, hw_params);
    if err < 0 {
        return err;
    }

    /* guard(mutex)(&chip->mutex); scoped_guard(spinlock_irq, &chip->reg_lock) */
    oxygen_write8_masked(chip, OXYGEN_PLAY_CHANNELS, oxygen_play_channels(hw_params), OXYGEN_PLAY_CHANNELS_MASK);
    oxygen_write8_masked(
        chip,
        OXYGEN_PLAY_FORMAT,
        oxygen_format(hw_params) << OXYGEN_MULTICH_FORMAT_SHIFT,
        OXYGEN_MULTICH_FORMAT_MASK,
    );
    oxygen_write16_masked(
        chip,
        OXYGEN_I2S_MULTICH_FORMAT,
        oxygen_rate(hw_params)
            | (*chip).model.dac_i2s_format
            | get_mclk(chip, PCM_MULTICH as c_uint, hw_params) as c_uint
            | oxygen_i2s_bits(hw_params),
        OXYGEN_I2S_RATE_MASK | OXYGEN_I2S_FORMAT_MASK | OXYGEN_I2S_MCLK_MASK | OXYGEN_I2S_BITS_MASK,
    );
    oxygen_update_spdif_source(chip);

    ((*chip).model.set_dac_params)(chip, hw_params);
    oxygen_update_dac_routing(chip);
    0
}

unsafe extern "C" fn oxygen_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let channel = oxygen_substream_channel(substream);
    let channel_mask = 1 << channel;

    /* guard(spinlock_irq)(&chip->reg_lock); */
    (*chip).interrupt_mask &= !channel_mask;
    oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, (*chip).interrupt_mask as u16);

    oxygen_set_bits8(chip, OXYGEN_DMA_FLUSH, channel_mask);
    oxygen_clear_bits8(chip, OXYGEN_DMA_FLUSH, channel_mask);

    0
}

unsafe extern "C" fn oxygen_spdif_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    /* scoped_guard(spinlock_irq, &chip->reg_lock) */
    oxygen_clear_bits32(chip, OXYGEN_SPDIF_CONTROL, OXYGEN_SPDIF_OUT_ENABLE);
    oxygen_hw_free(substream)
}

unsafe extern "C" fn oxygen_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let channel = oxygen_substream_channel(substream);
    let channel_mask = 1 << channel;

    /* guard(spinlock_irq)(&chip->reg_lock); */
    oxygen_set_bits8(chip, OXYGEN_DMA_FLUSH, channel_mask);
    oxygen_clear_bits8(chip, OXYGEN_DMA_FLUSH, channel_mask);

    if (*(*substream).runtime).no_period_wakeup != 0 {
        (*chip).interrupt_mask &= !channel_mask;
    } else {
        (*chip).interrupt_mask |= channel_mask;
    }
    oxygen_write16(chip, OXYGEN_INTERRUPT_MASK, (*chip).interrupt_mask as u16);
    0
}

unsafe extern "C" fn oxygen_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut s: *mut snd_pcm_substream;
    let mut mask: c_uint = 0;
    let pausing: c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_SUSPEND => {
            pausing = 0;
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            pausing = 1;
        }
        _ => {
            return -EINVAL;
        }
    }

    /*
     * snd_pcm_group_for_each_entry(s, substream) {
     *     if (snd_pcm_substream_chip(s) == chip) {
     *         mask |= 1 << oxygen_substream_channel(s);
     *         snd_pcm_trigger_done(s, substream);
     *     }
     * }
     */
    s = substream;
    if snd_pcm_substream_chip(s) == chip {
        mask |= 1 << oxygen_substream_channel(s);
        snd_pcm_trigger_done(s, substream);
    }

    /* guard(spinlock)(&chip->reg_lock); */
    if pausing == 0 {
        if cmd == SNDRV_PCM_TRIGGER_START {
            (*chip).pcm_running |= mask;
        } else {
            (*chip).pcm_running &= !mask;
        }
        oxygen_write8(chip, OXYGEN_DMA_STATUS, (*chip).pcm_running);
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH {
        oxygen_set_bits8(chip, OXYGEN_DMA_PAUSE, mask);
    } else {
        oxygen_clear_bits8(chip, OXYGEN_DMA_PAUSE, mask);
    }
    0
}

unsafe extern "C" fn oxygen_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let channel = oxygen_substream_channel(substream);
    let curr_addr: u32;

    /* no spinlock, this read should be atomic */
    curr_addr = oxygen_read32(chip, channel_base_registers[channel as usize]);
    bytes_to_frames(runtime, curr_addr.wrapping_sub((*runtime).dma_addr as u32))
}

static oxygen_rec_a_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(oxygen_rec_a_open),
    close: Some(oxygen_close),
    hw_params: Some(oxygen_rec_a_hw_params),
    hw_free: Some(oxygen_hw_free),
    prepare: Some(oxygen_prepare),
    trigger: Some(oxygen_trigger),
    pointer: Some(oxygen_pointer),
};

static oxygen_rec_b_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(oxygen_rec_b_open),
    close: Some(oxygen_close),
    hw_params: Some(oxygen_rec_b_hw_params),
    hw_free: Some(oxygen_hw_free),
    prepare: Some(oxygen_prepare),
    trigger: Some(oxygen_trigger),
    pointer: Some(oxygen_pointer),
};

static oxygen_rec_c_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(oxygen_rec_c_open),
    close: Some(oxygen_close),
    hw_params: Some(oxygen_rec_c_hw_params),
    hw_free: Some(oxygen_hw_free),
    prepare: Some(oxygen_prepare),
    trigger: Some(oxygen_trigger),
    pointer: Some(oxygen_pointer),
};

static oxygen_spdif_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(oxygen_spdif_open),
    close: Some(oxygen_close),
    hw_params: Some(oxygen_spdif_hw_params),
    hw_free: Some(oxygen_spdif_hw_free),
    prepare: Some(oxygen_prepare),
    trigger: Some(oxygen_trigger),
    pointer: Some(oxygen_pointer),
};

static oxygen_multich_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(oxygen_multich_open),
    close: Some(oxygen_close),
    hw_params: Some(oxygen_multich_hw_params),
    hw_free: Some(oxygen_hw_free),
    prepare: Some(oxygen_prepare),
    trigger: Some(oxygen_trigger),
    pointer: Some(oxygen_pointer),
};

static oxygen_ac97_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(oxygen_ac97_open),
    close: Some(oxygen_close),
    hw_params: Some(oxygen_hw_params),
    hw_free: Some(oxygen_hw_free),
    prepare: Some(oxygen_prepare),
    trigger: Some(oxygen_trigger),
    pointer: Some(oxygen_pointer),
};

#[no_mangle]
pub unsafe extern "C" fn oxygen_pcm_init(chip: *mut oxygen) -> c_int {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut outs: c_int;
    let mut ins: c_int;
    let mut err: c_int;

    outs = (((*chip).model.device_config & PLAYBACK_0_TO_I2S) != 0) as c_int;
    ins = (((*chip).model.device_config & (CAPTURE_0_FROM_I2S_1 | CAPTURE_0_FROM_I2S_2)) != 0) as c_int;
    if (outs | ins) != 0 {
        err = snd_pcm_new((*chip).card, b"Multichannel\0".as_ptr() as *const c_char, 0, outs, ins, &mut pcm);
        if err < 0 {
            return err;
        }
        if outs != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &oxygen_multich_ops);
        }
        if ((*chip).model.device_config & CAPTURE_0_FROM_I2S_1) != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &oxygen_rec_a_ops);
        } else if ((*chip).model.device_config & CAPTURE_0_FROM_I2S_2) != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &oxygen_rec_b_ops);
        }
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name, b"Multichannel\0".as_ptr() as *const c_char);
        if outs != 0 {
            snd_pcm_set_managed_buffer(
                (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK].substream,
                SNDRV_DMA_TYPE_DEV,
                &mut (*(*chip).pci).dev,
                DEFAULT_BUFFER_BYTES_MULTICH,
                BUFFER_BYTES_MAX_MULTICH,
            );
        }
        if ins != 0 {
            snd_pcm_set_managed_buffer(
                (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE].substream,
                SNDRV_DMA_TYPE_DEV,
                &mut (*(*chip).pci).dev,
                DEFAULT_BUFFER_BYTES,
                BUFFER_BYTES_MAX,
            );
        }
    }

    outs = (((*chip).model.device_config & PLAYBACK_1_TO_SPDIF) != 0) as c_int;
    ins = (((*chip).model.device_config & CAPTURE_1_FROM_SPDIF) != 0) as c_int;
    if (outs | ins) != 0 {
        err = snd_pcm_new((*chip).card, b"Digital\0".as_ptr() as *const c_char, 1, outs, ins, &mut pcm);
        if err < 0 {
            return err;
        }
        if outs != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &oxygen_spdif_ops);
        }
        if ins != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &oxygen_rec_c_ops);
        }
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name, b"Digital\0".as_ptr() as *const c_char);
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*chip).pci).dev,
            DEFAULT_BUFFER_BYTES,
            BUFFER_BYTES_MAX,
        );
    }

    if (*chip).has_ac97_1 != 0 {
        outs = (((*chip).model.device_config & PLAYBACK_2_TO_AC97_1) != 0) as c_int;
        ins = (((*chip).model.device_config & CAPTURE_2_FROM_AC97_1) != 0) as c_int;
    } else {
        outs = 0;
        ins = (((*chip).model.device_config & CAPTURE_2_FROM_I2S_2) != 0) as c_int;
    }
    if (outs | ins) != 0 {
        err = snd_pcm_new(
            (*chip).card,
            if outs != 0 { b"AC97\0".as_ptr() } else { b"Analog2\0".as_ptr() } as *const c_char,
            2,
            outs,
            ins,
            &mut pcm,
        );
        if err < 0 {
            return err;
        }
        if outs != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &oxygen_ac97_ops);
            oxygen_write8_masked(
                chip,
                OXYGEN_REC_ROUTING,
                OXYGEN_REC_B_ROUTE_AC97_1,
                OXYGEN_REC_B_ROUTE_MASK,
            );
        }
        if ins != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &oxygen_rec_b_ops);
        }
        (*pcm).private_data = chip as *mut c_void;
        strscpy(
            (*pcm).name,
            if outs != 0 { b"Front Panel\0".as_ptr() } else { b"Analog 2\0".as_ptr() } as *const c_char,
        );
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*chip).pci).dev,
            DEFAULT_BUFFER_BYTES,
            BUFFER_BYTES_MAX,
        );
    }

    ins = (((*chip).model.device_config & CAPTURE_3_FROM_I2S_3) != 0) as c_int;
    if ins != 0 {
        err = snd_pcm_new((*chip).card, b"Analog3\0".as_ptr() as *const c_char, 3, 0, ins, &mut pcm);
        if err < 0 {
            return err;
        }
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &oxygen_rec_c_ops);
        oxygen_write8_masked(
            chip,
            OXYGEN_REC_ROUTING,
            OXYGEN_REC_C_ROUTE_I2S_ADC_3,
            OXYGEN_REC_C_ROUTE_MASK,
        );
        (*pcm).private_data = chip as *mut c_void;
        strscpy((*pcm).name, b"Analog 3\0".as_ptr() as *const c_char);
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_DEV,
            &mut (*(*chip).pci).dev,
            DEFAULT_BUFFER_BYTES,
            BUFFER_BYTES_MAX,
        );
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
