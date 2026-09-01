// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ALSA driver for Echoaudio soundcards.
 *  Copyright (C) 2003-2004 Giuliano Pochini <pochini@shiny.it>
 *  Copyright (C) 2020 Mark Hills <mark@xwax.org>
 *
 *  Source-level Rust translation of echoaudio.c. Kernel/ALSA/card-specific
 *  types, constants, functions, and registration macros are expected to be
 *  supplied by the surrounding driver translation.
 */

use core::ffi::{c_char, c_int, c_short, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

/* MODULE_AUTHOR("Giuliano Pochini <pochini@shiny.it>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("Echoaudio " ECHOCARD_NAME " soundcards driver"); */
/* MODULE_DEVICE_TABLE(pci, snd_echo_ids); */

static mut index: [c_int; SNDRV_CARDS as usize] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS as usize] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS as usize] = SNDRV_DEFAULT_ENABLE_PNP;

/* module_param_array(index, int, NULL, 0444); */
/* MODULE_PARM_DESC(index, "Index value for " ECHOCARD_NAME " soundcard."); */
/* module_param_array(id, charp, NULL, 0444); */
/* MODULE_PARM_DESC(id, "ID string for " ECHOCARD_NAME " soundcard."); */
/* module_param_array(enable, bool, NULL, 0444); */
/* MODULE_PARM_DESC(enable, "Enable " ECHOCARD_NAME " soundcard."); */

static channels_list: [c_uint; 10] = [1, 2, 4, 6, 8, 10, 12, 14, 16, 999999];
static db_scale_output_gain: DECLARE_TLV_DB_SCALE =
    DECLARE_TLV_DB_SCALE::new(-12800, 100, 1);

unsafe fn get_firmware(
    fw_entry: *mut *const firmware,
    chip: *mut echoaudio,
    fw_index: c_short,
) -> c_int {
    let mut err: c_int;
    let mut name: [c_char; 30] = [0; 30];
    let idx = fw_index as usize;

    if !(*chip).fw_cache[idx].is_null() {
        dev_dbg(
            (*(*chip).card).dev,
            c"firmware requested: %s is cached\n".as_ptr(),
            card_fw[idx].data,
        );
        *fw_entry = (*chip).fw_cache[idx];
        return 0;
    }

    dev_dbg(
        (*(*chip).card).dev,
        c"firmware requested: %s\n".as_ptr(),
        card_fw[idx].data,
    );
    snprintf(
        name.as_mut_ptr(),
        size_of::<[c_char; 30]>(),
        c"ea/%s".as_ptr(),
        card_fw[idx].data,
    );
    err = request_firmware(fw_entry, name.as_ptr(), addr_of_mut!((*(*chip).pci).dev));
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            c"get_firmware(): Firmware not available (%d)\n".as_ptr(),
            err,
        );
    } else {
        (*chip).fw_cache[idx] = *fw_entry;
    }
    err
}

unsafe fn free_firmware(_fw_entry: *const firmware, chip: *mut echoaudio) {
    dev_dbg(
        (*(*chip).card).dev,
        c"firmware not released (kept in cache)\n".as_ptr(),
    );
}

unsafe fn free_firmware_cache(chip: *mut echoaudio) {
    for i in 0..8 {
        if !(*chip).fw_cache[i].is_null() {
            release_firmware((*chip).fw_cache[i]);
            dev_dbg((*(*chip).card).dev, c"release_firmware(%d)\n".as_ptr(), i as c_int);
        }
    }
}

/******************************************************************************
    PCM interface
******************************************************************************/

unsafe fn audiopipe_free(runtime: *mut snd_pcm_runtime) {
    let pipe = (*runtime).private_data as *mut audiopipe;

    if !(*pipe).sgpage.area.is_null() {
        snd_dma_free_pages(addr_of_mut!((*pipe).sgpage));
    }
    kfree(pipe as *mut c_void);
}

unsafe fn hw_rule_capture_format_by_channels(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let mut fmt: snd_mask = zeroed();

    snd_mask_any(addr_of_mut!(fmt));

    #[cfg(not(ECHOCARD_HAS_STEREO_BIG_ENDIAN32))]
    {
        /* >=2 channels cannot be S32_BE */
        if (*c).min == 2 {
            fmt.bits[0] &= !SNDRV_PCM_FMTBIT_S32_BE;
            return snd_mask_refine(f, addr_of_mut!(fmt));
        }
    }

    /* > 2 channels cannot be U8 and S32_BE */
    if (*c).min > 2 {
        fmt.bits[0] &= !(SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S32_BE);
        return snd_mask_refine(f, addr_of_mut!(fmt));
    }
    /* Mono is ok with any format */
    0
}

unsafe fn hw_rule_capture_channels_by_format(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let mut ch: snd_interval = zeroed();

    snd_interval_any(addr_of_mut!(ch));

    /* S32_BE is mono (and stereo) only */
    if (*f).bits[0] == SNDRV_PCM_FMTBIT_S32_BE {
        ch.min = 1;
        #[cfg(ECHOCARD_HAS_STEREO_BIG_ENDIAN32)]
        {
            ch.max = 2;
        }
        #[cfg(not(ECHOCARD_HAS_STEREO_BIG_ENDIAN32))]
        {
            ch.max = 1;
        }
        ch.integer = 1;
        return snd_interval_refine(c, addr_of_mut!(ch));
    }
    /* U8 can be only mono or stereo */
    if (*f).bits[0] == SNDRV_PCM_FMTBIT_U8 {
        ch.min = 1;
        ch.max = 2;
        ch.integer = 1;
        return snd_interval_refine(c, addr_of_mut!(ch));
    }
    /* S16_LE, S24_3LE and S32_LE support any number of channels. */
    0
}

unsafe fn hw_rule_playback_format_by_channels(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let mut fmt: snd_mask = zeroed();
    let mut fmask: u64;

    snd_mask_any(addr_of_mut!(fmt));
    fmask = fmt.bits[0] as u64 + ((fmt.bits[1] as u64) << 32);

    /* >2 channels must be S16_LE, S24_3LE or S32_LE */
    if (*c).min > 2 {
        fmask &= (SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S32_LE) as u64;
    /* 1 channel must be S32_BE or S32_LE */
    } else if (*c).max == 1 {
        fmask &= (SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S32_BE) as u64;
    } else {
        #[cfg(not(ECHOCARD_HAS_STEREO_BIG_ENDIAN32))]
        {
            /* 2 channels cannot be S32_BE */
            if (*c).min == 2 && (*c).max == 2 {
                fmask &= !(SNDRV_PCM_FMTBIT_S32_BE as u64);
            } else {
                return 0;
            }
        }
        #[cfg(ECHOCARD_HAS_STEREO_BIG_ENDIAN32)]
        {
            return 0;
        }
    }

    fmt.bits[0] &= fmask as u32;
    fmt.bits[1] &= (fmask >> 32) as u32;
    snd_mask_refine(f, addr_of_mut!(fmt))
}

unsafe fn hw_rule_playback_channels_by_format(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let f = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let mut ch: snd_interval = zeroed();
    let fmask: u64;

    snd_interval_any(addr_of_mut!(ch));
    ch.integer = 1;
    fmask = (*f).bits[0] as u64 + (((*f).bits[1] as u64) << 32);

    /* S32_BE is mono (and stereo) only */
    if fmask == SNDRV_PCM_FMTBIT_S32_BE as u64 {
        ch.min = 1;
        #[cfg(ECHOCARD_HAS_STEREO_BIG_ENDIAN32)]
        {
            ch.max = 2;
        }
        #[cfg(not(ECHOCARD_HAS_STEREO_BIG_ENDIAN32))]
        {
            ch.max = 1;
        }
    /* U8 is stereo only */
    } else if fmask == SNDRV_PCM_FMTBIT_U8 as u64 {
        ch.min = 2;
        ch.max = 2;
    /* S16_LE and S24_3LE must be at least stereo */
    } else if (fmask & !((SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE) as u64)) == 0 {
        ch.min = 2;
    } else {
        return 0;
    }

    snd_interval_refine(c, addr_of_mut!(ch))
}

/* Since the sample rate is a global setting, do allow the user to change the
sample rate only if there is only one pcm device open. */
unsafe fn hw_rule_sample_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let chip = (*rule).private as *mut echoaudio;
    let mut fixed: snd_interval = zeroed();
    let err: c_int;

    guard_mutex(addr_of_mut!((*chip).mode_mutex));
    if (*chip).can_set_rate != 0 {
        err = 0;
    } else {
        snd_interval_any(addr_of_mut!(fixed));
        fixed.min = (*chip).sample_rate;
        fixed.max = (*chip).sample_rate;
        err = snd_interval_refine(rate, addr_of_mut!(fixed));
    }
    err
}

unsafe fn pcm_open(substream: *mut snd_pcm_substream, max_channels: i8) -> c_int {
    let chip: *mut echoaudio;
    let runtime: *mut snd_pcm_runtime;
    let pipe: *mut audiopipe;
    let mut err: c_int;
    let mut i: c_int;

    if max_channels <= 0 {
        return -EAGAIN;
    }

    chip = snd_pcm_substream_chip(substream);
    runtime = (*substream).runtime;

    pipe = kzalloc_obj_audiopipe();
    if pipe.is_null() {
        return -ENOMEM;
    }
    (*pipe).index = -1; /* Not configured yet */

    /* Set up hw capabilities and contraints */
    memcpy(
        addr_of_mut!((*pipe).hw) as *mut c_void,
        addr_of!(pcm_hardware_skel) as *const c_void,
        size_of::<snd_pcm_hardware>(),
    );
    dev_dbg((*(*chip).card).dev, c"max_channels=%d\n".as_ptr(), max_channels as c_int);
    (*pipe).constr.list = channels_list.as_ptr();
    (*pipe).constr.mask = 0;
    i = 0;
    while channels_list[i as usize] <= max_channels as c_uint {
        i += 1;
    }
    (*pipe).constr.count = i as c_uint;
    if (*pipe).hw.channels_max > max_channels as c_uint {
        (*pipe).hw.channels_max = max_channels as c_uint;
    }
    if (*chip).digital_mode == DIGITAL_MODE_ADAT {
        (*pipe).hw.rate_max = 48000;
        (*pipe).hw.rates &= SNDRV_PCM_RATE_8000_48000;
    }

    (*runtime).hw = (*pipe).hw;
    (*runtime).private_data = pipe as *mut c_void;
    (*runtime).private_free = Some(audiopipe_free);
    snd_pcm_set_sync(substream);

    /* Only mono and any even number of channels are allowed */
    err = snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, addr_of_mut!((*pipe).constr));
    if err < 0 { return err; }

    /* All periods should have the same size */
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 { return err; }

    /* The hw accesses memory in chunks 32 frames long and they should be
    32-bytes-aligned. It's not a requirement, but it seems that IRQs are
    generated with a resolution of 32 frames. Thus we need the following */
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 32);
    if err < 0 { return err; }
    err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 32);
    if err < 0 { return err; }

    err = snd_pcm_hw_rule_add(
        (*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE,
        Some(hw_rule_sample_rate), chip as *mut c_void, SNDRV_PCM_HW_PARAM_RATE, -1,
    );
    if err < 0 { return err; }

    /* Allocate a page for the scatter-gather list */
    err = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, addr_of_mut!((*(*chip).pci).dev), PAGE_SIZE, addr_of_mut!((*pipe).sgpage));
    if err < 0 {
        dev_err((*(*chip).card).dev, c"s-g list allocation failed\n".as_ptr());
        return err;
    }

    /*
     * Sole ownership required to set the rate
     */
    dev_dbg(
        (*(*chip).card).dev,
        c"pcm_open opencount=%d can_set_rate=%d, rate_set=%d".as_ptr(),
        (*chip).opencount,
        (*chip).can_set_rate,
        (*chip).rate_set,
    );

    (*chip).opencount += 1;
    if (*chip).opencount > 1 && (*chip).rate_set != 0 {
        (*chip).can_set_rate = 0;
    }

    0
}

unsafe fn pcm_analog_in_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;

    err = pcm_open(substream, (num_analog_busses_in(chip) - (*substream).number) as i8);
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(hw_rule_capture_channels_by_format), null_mut(), SNDRV_PCM_HW_PARAM_FORMAT, -1);
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT,
        Some(hw_rule_capture_format_by_channels), null_mut(), SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if err < 0 { return err; }
    0
}

unsafe fn pcm_analog_out_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let max_channels: c_int;
    let mut err: c_int;

    #[cfg(ECHOCARD_HAS_VMIXER)]
    { max_channels = num_pipes_out(chip); }
    #[cfg(not(ECHOCARD_HAS_VMIXER))]
    { max_channels = num_analog_busses_out(chip); }

    err = pcm_open(substream, (max_channels - (*substream).number) as i8);
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(hw_rule_playback_channels_by_format), null_mut(), SNDRV_PCM_HW_PARAM_FORMAT, -1);
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT,
        Some(hw_rule_playback_format_by_channels), null_mut(), SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if err < 0 { return err; }
    0
}

#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
unsafe fn pcm_digital_in_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;
    let max_channels = num_digital_busses_in(chip) - (*substream).number;

    guard_mutex(addr_of_mut!((*chip).mode_mutex));
    if (*chip).digital_mode == DIGITAL_MODE_ADAT {
        err = pcm_open(substream, max_channels as i8);
    } else {
        /* If the card has ADAT, subtract the 6 channels that S/PDIF doesn't have */
        err = pcm_open(substream, (max_channels - ECHOCARD_HAS_ADAT) as i8);
    }
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(hw_rule_capture_channels_by_format), null_mut(), SNDRV_PCM_HW_PARAM_FORMAT, -1);
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT,
        Some(hw_rule_capture_format_by_channels), null_mut(), SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if err < 0 { return err; }
    0
}

#[cfg(all(ECHOCARD_HAS_DIGITAL_IO, not(ECHOCARD_HAS_VMIXER)))]
unsafe fn pcm_digital_out_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;
    let max_channels = num_digital_busses_out(chip) - (*substream).number;

    guard_mutex(addr_of_mut!((*chip).mode_mutex));
    if (*chip).digital_mode == DIGITAL_MODE_ADAT {
        err = pcm_open(substream, max_channels as i8);
    } else {
        /* If the card has ADAT, subtract the 6 channels that S/PDIF doesn't have */
        err = pcm_open(substream, (max_channels - ECHOCARD_HAS_ADAT) as i8);
    }
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS,
        Some(hw_rule_playback_channels_by_format), null_mut(), SNDRV_PCM_HW_PARAM_FORMAT, -1);
    if err < 0 { return err; }
    err = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT,
        Some(hw_rule_playback_format_by_channels), null_mut(), SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if err < 0 { return err; }
    0
}

unsafe fn pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    /* Nothing to do here. Audio is already off and pipe will be freed by its callback */
    guard_mutex(addr_of_mut!((*chip).mode_mutex));

    dev_dbg(
        (*(*chip).card).dev,
        c"pcm_open opencount=%d can_set_rate=%d, rate_set=%d".as_ptr(),
        (*chip).opencount,
        (*chip).can_set_rate,
        (*chip).rate_set,
    );

    (*chip).opencount -= 1;
    match (*chip).opencount {
        1 => (*chip).can_set_rate = 1,
        0 => (*chip).rate_set = 0,
        _ => {}
    }
    0
}

/* Channel allocation and scatter-gather list setup */
unsafe fn init_engine(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
    pipe_index: c_int,
    interleave: c_int,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut err: c_int;
    let mut per: c_int;
    let mut rest: c_int;
    let mut page: c_int;
    let mut edge: c_int;
    let mut offs: c_int;
    let pipe = (*(*substream).runtime).private_data as *mut audiopipe;

    /* Sets up che hardware. If it's already initialized, reset and redo with the new parameters */
    spin_lock_irq(addr_of_mut!((*chip).lock));
    if (*pipe).index >= 0 {
        dev_dbg((*(*chip).card).dev, c"hwp_ie free(%d)\n".as_ptr(), (*pipe).index);
        err = free_pipes(chip, pipe);
        snd_BUG_ON(err);
        (*chip).substream[(*pipe).index as usize] = null_mut();
    }
    err = allocate_pipes(chip, pipe, pipe_index, interleave);
    if err < 0 {
        spin_unlock_irq(addr_of_mut!((*chip).lock));
        dev_err((*(*chip).card).dev, c"allocate_pipes(%d) err=%d\n".as_ptr(), pipe_index, err);
        return err;
    }
    spin_unlock_irq(addr_of_mut!((*chip).lock));

    dev_dbg((*(*chip).card).dev, c"allocate_pipes()=%d\n".as_ptr(), pipe_index);
    dev_dbg(
        (*(*chip).card).dev,
        c"pcm_hw_params (bufsize=%dB periods=%d persize=%dB)\n".as_ptr(),
        params_buffer_bytes(hw_params),
        params_periods(hw_params),
        params_period_bytes(hw_params),
    );

    sglist_init(chip, pipe);
    edge = PAGE_SIZE;
    offs = 0;
    page = 0;
    per = 0;
    while offs < params_buffer_bytes(hw_params) {
        rest = params_period_bytes(hw_params);
        if offs + rest > params_buffer_bytes(hw_params) {
            rest = params_buffer_bytes(hw_params) - offs;
        }
        while rest != 0 {
            let addr: dma_addr_t = snd_pcm_sgbuf_get_addr(substream, offs);
            if rest <= edge - offs {
                sglist_add_mapping(chip, pipe, addr, rest);
                sglist_add_irq(chip, pipe);
                offs += rest;
                rest = 0;
            } else {
                sglist_add_mapping(chip, pipe, addr, edge - offs);
                rest -= edge - offs;
                offs = edge;
            }
            if offs == edge {
                edge += PAGE_SIZE;
                page += 1;
            }
        }
        per += 1;
    }

    /* Close the ring buffer */
    sglist_wrap(chip, pipe);

    /* This stuff is used by the irq handler, so it must be initialized before chip->substream */
    (*pipe).last_period = 0;
    (*pipe).last_counter = 0;
    (*pipe).position = 0;
    smp_wmb();
    (*chip).substream[pipe_index as usize] = substream;
    (*chip).rate_set = 1;
    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    set_sample_rate(chip, (*hw_params).rate_num / (*hw_params).rate_den);
    0
}

unsafe fn pcm_analog_in_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    init_engine(substream, hw_params, px_analog_in(chip) + (*substream).number, params_channels(hw_params))
}

unsafe fn pcm_analog_out_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    init_engine(substream, hw_params, (*substream).number, params_channels(hw_params))
}

#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
unsafe fn pcm_digital_in_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    init_engine(substream, hw_params, px_digital_in(chip) + (*substream).number, params_channels(hw_params))
}

#[cfg(all(ECHOCARD_HAS_DIGITAL_IO, not(ECHOCARD_HAS_VMIXER)))]
unsafe fn pcm_digital_out_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    init_engine(substream, hw_params, px_digital_out(chip) + (*substream).number, params_channels(hw_params))
}

unsafe fn pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let pipe = (*(*substream).runtime).private_data as *mut audiopipe;

    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    if (*pipe).index >= 0 {
        dev_dbg((*(*chip).card).dev, c"pcm_hw_free(%d)\n".as_ptr(), (*pipe).index);
        free_pipes(chip, pipe);
        (*chip).substream[(*pipe).index as usize] = null_mut();
        (*pipe).index = -1;
    }
    0
}

unsafe fn pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut format: audioformat = zeroed();
    let pipe_index = (*((*runtime).private_data as *mut audiopipe)).index;

    dev_dbg(
        (*(*chip).card).dev,
        c"Prepare rate=%d format=%d channels=%d\n".as_ptr(),
        (*runtime).rate,
        (*runtime).format,
        (*runtime).channels,
    );
    format.interleave = (*runtime).channels;
    format.data_are_bigendian = 0;
    format.mono_to_stereo = 0;
    match (*runtime).format {
        SNDRV_PCM_FORMAT_U8 => format.bits_per_sample = 8,
        SNDRV_PCM_FORMAT_S16_LE => format.bits_per_sample = 16,
        SNDRV_PCM_FORMAT_S24_3LE => format.bits_per_sample = 24,
        SNDRV_PCM_FORMAT_S32_BE => {
            format.data_are_bigendian = 1;
            format.bits_per_sample = 32;
        }
        SNDRV_PCM_FORMAT_S32_LE => format.bits_per_sample = 32,
        _ => {
            dev_err((*(*chip).card).dev, c"Prepare error: unsupported format %d\n".as_ptr(), (*runtime).format);
            return -EINVAL;
        }
    }

    if snd_BUG_ON((pipe_index >= px_num(chip)) as c_int) != 0 {
        return -EINVAL;
    }

    /*
     * We passed checks we can do independently; now take exclusive control
     */
    guard_spinlock_irq(addr_of_mut!((*chip).lock));

    if snd_BUG_ON((is_pipe_allocated(chip, pipe_index) == 0) as c_int) != 0 {
        return -EINVAL;
    }

    set_audio_format(chip, pipe_index, addr_of_mut!(format));
    0
}

unsafe fn pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut pipe: *mut audiopipe;
    let mut err: c_int;
    let mut channelmask: u32 = 0;
    let mut s: *mut snd_pcm_substream;

    snd_pcm_group_for_each_entry!(s, substream, {
        for i in 0..DSP_MAXPIPES {
            if s == (*chip).substream[i as usize] {
                channelmask |= 1u32 << i;
                snd_pcm_trigger_done(s, substream);
            }
        }
    });

    guard_spinlock(addr_of_mut!((*chip).lock));
    match cmd {
        SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            for i in 0..DSP_MAXPIPES {
                if (channelmask & (1u32 << i)) != 0 {
                    pipe = (*(*(*chip).substream[i as usize]).runtime).private_data as *mut audiopipe;
                    match (*pipe).state {
                        PIPE_STATE_STOPPED => {
                            (*pipe).last_period = 0;
                            (*pipe).last_counter = 0;
                            (*pipe).position = 0;
                            *(*pipe).dma_counter = 0;
                            (*pipe).state = PIPE_STATE_STARTED;
                        }
                        PIPE_STATE_PAUSED => (*pipe).state = PIPE_STATE_STARTED,
                        PIPE_STATE_STARTED => {}
                        _ => {}
                    }
                }
            }
            err = start_transport(chip, channelmask, (*chip).pipe_cyclic_mask);
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP => {
            for i in 0..DSP_MAXPIPES {
                if (channelmask & (1u32 << i)) != 0 {
                    pipe = (*(*(*chip).substream[i as usize]).runtime).private_data as *mut audiopipe;
                    (*pipe).state = PIPE_STATE_STOPPED;
                }
            }
            err = stop_transport(chip, channelmask);
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            for i in 0..DSP_MAXPIPES {
                if (channelmask & (1u32 << i)) != 0 {
                    pipe = (*(*(*chip).substream[i as usize]).runtime).private_data as *mut audiopipe;
                    (*pipe).state = PIPE_STATE_PAUSED;
                }
            }
            err = pause_transport(chip, channelmask);
        }
        _ => err = -EINVAL,
    }
    err
}

unsafe fn pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let pipe = (*runtime).private_data as *mut audiopipe;
    let counter: u32;
    let step: u32;

    /*
     * IRQ handling runs concurrently. Do not share tracking of counter with it,
     * which would race or require locking
     */
    counter = le32_to_cpu(*(*pipe).dma_counter); /* presumed atomic */
    step = counter.wrapping_sub((*pipe).last_counter);
    (*pipe).last_counter = counter;

    /* counter doesn't neccessarily wrap on a multiple of buffer_size, so can't
     * derive the position; must accumulate */
    (*pipe).position = (*pipe).position.wrapping_add(step);
    (*pipe).position %= frames_to_bytes(runtime, (*runtime).buffer_size) as u32; /* wrap */
    bytes_to_frames(runtime, (*pipe).position)
}

/* pcm *_ops structures */
static analog_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(pcm_analog_out_open),
    close: Some(pcm_close),
    hw_params: Some(pcm_analog_out_hw_params),
    hw_free: Some(pcm_hw_free),
    prepare: Some(pcm_prepare),
    trigger: Some(pcm_trigger),
    pointer: Some(pcm_pointer),
};
static analog_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(pcm_analog_in_open),
    close: Some(pcm_close),
    hw_params: Some(pcm_analog_in_hw_params),
    hw_free: Some(pcm_hw_free),
    prepare: Some(pcm_prepare),
    trigger: Some(pcm_trigger),
    pointer: Some(pcm_pointer),
};

#[cfg(all(ECHOCARD_HAS_DIGITAL_IO, not(ECHOCARD_HAS_VMIXER)))]
static digital_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(pcm_digital_out_open),
    close: Some(pcm_close),
    hw_params: Some(pcm_digital_out_hw_params),
    hw_free: Some(pcm_hw_free),
    prepare: Some(pcm_prepare),
    trigger: Some(pcm_trigger),
    pointer: Some(pcm_pointer),
};
#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
static digital_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(pcm_digital_in_open),
    close: Some(pcm_close),
    hw_params: Some(pcm_digital_in_hw_params),
    hw_free: Some(pcm_hw_free),
    prepare: Some(pcm_prepare),
    trigger: Some(pcm_trigger),
    pointer: Some(pcm_pointer),
};

/* Preallocate memory only for the first substream because it's the most used one */
unsafe fn snd_echo_preallocate_pages(pcm: *mut snd_pcm, dev: *mut device) {
    for stream in 0..2 {
        let mut ss = (*pcm).streams[stream].substream;
        while !ss.is_null() {
            snd_pcm_set_managed_buffer(
                ss,
                SNDRV_DMA_TYPE_DEV_SG,
                dev,
                if (*ss).number != 0 { 0 } else { 128 << 10 },
                256 << 10,
            );
            ss = (*ss).next;
        }
    }
}

/*<--snd_echo_probe() */
unsafe fn snd_echo_new_pcm(chip: *mut echoaudio) -> c_int {
    let mut pcm: *mut snd_pcm = null_mut();
    let mut err: c_int;

    #[cfg(ECHOCARD_HAS_VMIXER)]
    {
        /* This card has a Vmixer, that is there is no direct mapping from PCM
        streams to physical outputs. The user can mix the streams as he wishes
        via control interface and it's possible to send any stream to any
        output, thus it makes no sense to keep analog and digital outputs
        separated */

        /* PCM#0 Virtual outputs and analog inputs */
        err = snd_pcm_new((*chip).card, c"PCM".as_ptr(), 0, num_pipes_out(chip),
            num_analog_busses_in(chip), addr_of_mut!(pcm));
        if err < 0 { return err; }
        (*pcm).private_data = chip as *mut c_void;
        (*chip).analog_pcm = pcm;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, addr_of!(analog_playback_ops));
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, addr_of!(analog_capture_ops));
        snd_echo_preallocate_pages(pcm, addr_of_mut!((*(*chip).pci).dev));

        #[cfg(ECHOCARD_HAS_DIGITAL_IO)]
        {
            /* PCM#1 Digital inputs, no outputs */
            err = snd_pcm_new((*chip).card, c"Digital PCM".as_ptr(), 1, 0,
                num_digital_busses_in(chip), addr_of_mut!(pcm));
            if err < 0 { return err; }
            (*pcm).private_data = chip as *mut c_void;
            (*chip).digital_pcm = pcm;
            strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, addr_of!(digital_capture_ops));
            snd_echo_preallocate_pages(pcm, addr_of_mut!((*(*chip).pci).dev));
        }
    }

    #[cfg(not(ECHOCARD_HAS_VMIXER))]
    {
        /* The card can manage substreams formed by analog and digital channels
        at the same time, but I prefer to keep analog and digital channels
        separated, because that mixed thing is confusing and useless. So we
        register two PCM devices: */

        /* PCM#0 Analog i/o */
        err = snd_pcm_new((*chip).card, c"Analog PCM".as_ptr(), 0,
            num_analog_busses_out(chip), num_analog_busses_in(chip), addr_of_mut!(pcm));
        if err < 0 { return err; }
        (*pcm).private_data = chip as *mut c_void;
        (*chip).analog_pcm = pcm;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, addr_of!(analog_playback_ops));
        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, addr_of!(analog_capture_ops));
        snd_echo_preallocate_pages(pcm, addr_of_mut!((*(*chip).pci).dev));

        #[cfg(ECHOCARD_HAS_DIGITAL_IO)]
        {
            /* PCM#1 Digital i/o */
            err = snd_pcm_new((*chip).card, c"Digital PCM".as_ptr(), 1,
                num_digital_busses_out(chip), num_digital_busses_in(chip), addr_of_mut!(pcm));
            if err < 0 { return err; }
            (*pcm).private_data = chip as *mut c_void;
            (*chip).digital_pcm = pcm;
            strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, addr_of!(digital_playback_ops));
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, addr_of!(digital_capture_ops));
            snd_echo_preallocate_pages(pcm, addr_of_mut!((*(*chip).pci).dev));
        }
    }

    0
}

/******************************************************************************
    Control interface
******************************************************************************/

#[cfg(any(not(ECHOCARD_HAS_VMIXER), ECHOCARD_HAS_LINE_OUT_GAIN))]
unsafe fn snd_echo_output_gain_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = num_busses_out(chip);
    (*uinfo).value.integer.min = ECHOGAIN_MINOUT;
    (*uinfo).value.integer.max = ECHOGAIN_MAXOUT;
    0
}

#[cfg(any(not(ECHOCARD_HAS_VMIXER), ECHOCARD_HAS_LINE_OUT_GAIN))]
unsafe fn snd_echo_output_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    for c in 0..num_busses_out(chip) {
        (*ucontrol).value.integer.value[c as usize] = (*chip).output_gain[c as usize];
    }
    0
}

#[cfg(any(not(ECHOCARD_HAS_VMIXER), ECHOCARD_HAS_LINE_OUT_GAIN))]
unsafe fn snd_echo_output_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    for c in 0..num_busses_out(chip) {
        let gain = (*ucontrol).value.integer.value[c as usize];
        /* Ignore out of range values */
        if gain < ECHOGAIN_MINOUT || gain > ECHOGAIN_MAXOUT { continue; }
        if (*chip).output_gain[c as usize] != gain {
            set_output_gain(chip, c, gain);
            changed = 1;
        }
    }
    if changed != 0 { update_output_line_level(chip); }
    changed
}

#[cfg(ECHOCARD_HAS_LINE_OUT_GAIN)]
static snd_echo_line_output_gain: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Line Playback Volume".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_echo_output_gain_info),
    get: Some(snd_echo_output_gain_get),
    put: Some(snd_echo_output_gain_put),
    tlv: snd_kcontrol_tlv { p: addr_of!(db_scale_output_gain) },
    ..snd_kcontrol_new::zero()
};

#[cfg(all(not(ECHOCARD_HAS_LINE_OUT_GAIN), any(not(ECHOCARD_HAS_VMIXER), ECHOCARD_HAS_LINE_OUT_GAIN)))]
static snd_echo_pcm_output_gain: snd_kcontrol_new = snd_kcontrol_new {
    name: c"PCM Playback Volume".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_echo_output_gain_info),
    get: Some(snd_echo_output_gain_get),
    put: Some(snd_echo_output_gain_put),
    tlv: snd_kcontrol_tlv { p: addr_of!(db_scale_output_gain) },
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_INPUT_GAIN)]
unsafe fn snd_echo_input_gain_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = num_analog_busses_in(chip);
    (*uinfo).value.integer.min = ECHOGAIN_MININP;
    (*uinfo).value.integer.max = ECHOGAIN_MAXINP;
    0
}

#[cfg(ECHOCARD_HAS_INPUT_GAIN)]
unsafe fn snd_echo_input_gain_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    for c in 0..num_analog_busses_in(chip) {
        (*ucontrol).value.integer.value[c as usize] = (*chip).input_gain[c as usize];
    }
    0
}

#[cfg(ECHOCARD_HAS_INPUT_GAIN)]
unsafe fn snd_echo_input_gain_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    for c in 0..num_analog_busses_in(chip) {
        let gain = (*ucontrol).value.integer.value[c as usize];
        /* Ignore out of range values */
        if gain < ECHOGAIN_MININP || gain > ECHOGAIN_MAXINP { continue; }
        if (*chip).input_gain[c as usize] != gain {
            set_input_gain(chip, c, gain);
            changed = 1;
        }
    }
    if changed != 0 { update_input_line_level(chip); }
    changed
}

#[cfg(ECHOCARD_HAS_INPUT_GAIN)]
static db_scale_input_gain: DECLARE_TLV_DB_SCALE = DECLARE_TLV_DB_SCALE::new(-2500, 50, 0);

#[cfg(ECHOCARD_HAS_INPUT_GAIN)]
static snd_echo_line_input_gain: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Line Capture Volume".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_echo_input_gain_info),
    get: Some(snd_echo_input_gain_get),
    put: Some(snd_echo_input_gain_put),
    tlv: snd_kcontrol_tlv { p: addr_of!(db_scale_input_gain) },
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL)]
unsafe fn snd_echo_output_nominal_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = num_analog_busses_out(chip);
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

#[cfg(ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL)]
unsafe fn snd_echo_output_nominal_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    for c in 0..num_analog_busses_out(chip) {
        (*ucontrol).value.integer.value[c as usize] = (*chip).nominal_level[c as usize];
    }
    0
}

#[cfg(ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL)]
unsafe fn snd_echo_output_nominal_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    for c in 0..num_analog_busses_out(chip) {
        if (*chip).nominal_level[c as usize] != (*ucontrol).value.integer.value[c as usize] {
            set_nominal_level(chip, c, (*ucontrol).value.integer.value[c as usize]);
            changed = 1;
        }
    }
    if changed != 0 { update_output_line_level(chip); }
    changed
}

#[cfg(ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL)]
static snd_echo_output_nominal_level: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Line Playback Switch (-10dBV)".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    info: Some(snd_echo_output_nominal_info),
    get: Some(snd_echo_output_nominal_get),
    put: Some(snd_echo_output_nominal_put),
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_INPUT_NOMINAL_LEVEL)]
unsafe fn snd_echo_input_nominal_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = num_analog_busses_in(chip);
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

#[cfg(ECHOCARD_HAS_INPUT_NOMINAL_LEVEL)]
unsafe fn snd_echo_input_nominal_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    for c in 0..num_analog_busses_in(chip) {
        (*ucontrol).value.integer.value[c as usize] =
            (*chip).nominal_level[(bx_analog_in(chip) + c) as usize];
    }
    0
}

#[cfg(ECHOCARD_HAS_INPUT_NOMINAL_LEVEL)]
unsafe fn snd_echo_input_nominal_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    for c in 0..num_analog_busses_in(chip) {
        let idx = bx_analog_in(chip) + c;
        if (*chip).nominal_level[idx as usize] != (*ucontrol).value.integer.value[c as usize] {
            set_nominal_level(chip, idx, (*ucontrol).value.integer.value[c as usize]);
            changed = 1;
        }
    }
    if changed != 0 {
        update_output_line_level(chip); /* "Output" is not a mistake here. */
    }
    changed
}

#[cfg(ECHOCARD_HAS_INPUT_NOMINAL_LEVEL)]
static snd_echo_intput_nominal_level: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Line Capture Switch (-10dBV)".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    info: Some(snd_echo_input_nominal_info),
    get: Some(snd_echo_input_nominal_get),
    put: Some(snd_echo_input_nominal_put),
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_MONITOR)]
unsafe fn snd_echo_mixer_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = ECHOGAIN_MINOUT;
    (*uinfo).value.integer.max = ECHOGAIN_MAXOUT;
    0
}

#[cfg(ECHOCARD_HAS_MONITOR)]
unsafe fn snd_echo_mixer_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let out = (*ucontrol).id.index / num_busses_in(chip) as c_uint;
    let input = (*ucontrol).id.index % num_busses_in(chip) as c_uint;
    if out >= ECHO_MAXAUDIOOUTPUTS || input >= ECHO_MAXAUDIOINPUTS { return -EINVAL; }
    (*ucontrol).value.integer.value[0] = (*chip).monitor_gain[out as usize][input as usize];
    0
}

#[cfg(ECHOCARD_HAS_MONITOR)]
unsafe fn snd_echo_mixer_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    let out = (*ucontrol).id.index / num_busses_in(chip) as c_uint;
    let input = (*ucontrol).id.index % num_busses_in(chip) as c_uint;
    if out >= ECHO_MAXAUDIOOUTPUTS || input >= ECHO_MAXAUDIOINPUTS { return -EINVAL; }
    let gain = (*ucontrol).value.integer.value[0];
    if gain < ECHOGAIN_MINOUT || gain > ECHOGAIN_MAXOUT { return -EINVAL; }
    if (*chip).monitor_gain[out as usize][input as usize] != gain {
        guard_spinlock_irq(addr_of_mut!((*chip).lock));
        set_monitor_gain(chip, out, input, gain);
        update_output_line_level(chip);
        changed = 1;
    }
    changed
}

#[cfg(ECHOCARD_HAS_MONITOR)]
static mut snd_echo_monitor_mixer: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Monitor Mixer Volume".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_echo_mixer_info),
    get: Some(snd_echo_mixer_get),
    put: Some(snd_echo_mixer_put),
    tlv: snd_kcontrol_tlv { p: addr_of!(db_scale_output_gain) },
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_VMIXER)]
unsafe fn snd_echo_vmixer_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = ECHOGAIN_MINOUT;
    (*uinfo).value.integer.max = ECHOGAIN_MAXOUT;
    0
}

#[cfg(ECHOCARD_HAS_VMIXER)]
unsafe fn snd_echo_vmixer_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] =
        (*chip).vmixer_gain[((*ucontrol).id.index / num_pipes_out(chip) as c_uint) as usize]
            [((*ucontrol).id.index % num_pipes_out(chip) as c_uint) as usize];
    0
}

#[cfg(ECHOCARD_HAS_VMIXER)]
unsafe fn snd_echo_vmixer_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    let out = ((*ucontrol).id.index / num_pipes_out(chip) as c_uint) as c_short;
    let vch = ((*ucontrol).id.index % num_pipes_out(chip) as c_uint) as c_short;
    let gain = (*ucontrol).value.integer.value[0];
    if gain < ECHOGAIN_MINOUT || gain > ECHOGAIN_MAXOUT { return -EINVAL; }
    if (*chip).vmixer_gain[out as usize][vch as usize] != gain {
        guard_spinlock_irq(addr_of_mut!((*chip).lock));
        set_vmixer_gain(chip, out, vch, gain);
        update_vmixer_level(chip);
        changed = 1;
    }
    changed
}

#[cfg(ECHOCARD_HAS_VMIXER)]
static mut snd_echo_vmixer: snd_kcontrol_new = snd_kcontrol_new {
    name: c"VMixer Volume".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_echo_vmixer_info),
    get: Some(snd_echo_vmixer_get),
    put: Some(snd_echo_vmixer_put),
    tlv: snd_kcontrol_tlv { p: addr_of!(db_scale_output_gain) },
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_DIGITAL_MODE_SWITCH)]
unsafe fn snd_echo_digital_mode_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 4] = [
        c"S/PDIF Coaxial".as_ptr(), c"S/PDIF Optical".as_ptr(),
        c"ADAT Optical".as_ptr(), c"S/PDIF Cdrom".as_ptr(),
    ];
    let chip = snd_kcontrol_chip(kcontrol);
    snd_ctl_enum_info(uinfo, 1, (*chip).num_digital_modes, names.as_ptr())
}

#[cfg(ECHOCARD_HAS_DIGITAL_MODE_SWITCH)]
unsafe fn snd_echo_digital_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mode = (*chip).digital_mode;
    let mut i = (*chip).num_digital_modes - 1;
    loop {
        if mode == (*chip).digital_mode_list[i as usize] {
            (*ucontrol).value.enumerated.item[0] = i as c_uint;
            break;
        }
        if i == 0 { break; }
        i -= 1;
    }
    0
}

#[cfg(ECHOCARD_HAS_DIGITAL_MODE_SWITCH)]
unsafe fn snd_echo_digital_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    let emode = (*ucontrol).value.enumerated.item[0] as u16;
    if emode >= (*chip).num_digital_modes as u16 { return -EINVAL; }
    let dmode = (*chip).digital_mode_list[emode as usize];
    if dmode != (*chip).digital_mode {
        /* mode_mutex is required to make this operation atomic wrt
        pcm_digital_*_open() and set_input_clock() functions. */
        guard_mutex(addr_of_mut!((*chip).mode_mutex));
        /* Do not allow the user to change the digital mode when a pcm device is
        open because it also changes the number of channels and the allowed sample rates */
        if (*chip).opencount != 0 {
            changed = -EAGAIN;
        } else {
            changed = set_digital_mode(chip, dmode);
            /* If we had to change the clock source, report it */
            if changed > 0 && !(*chip).clock_src_ctl.is_null() {
                snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, addr_of_mut!((*(*chip).clock_src_ctl).id));
                dev_dbg((*(*chip).card).dev, c"SDM() =%d\n".as_ptr(), changed);
            }
            if changed >= 0 { changed = 1; } /* No errors */
        }
    }
    changed
}

#[cfg(ECHOCARD_HAS_DIGITAL_MODE_SWITCH)]
static snd_echo_digital_mode_switch: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Digital mode Switch".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(snd_echo_digital_mode_info),
    get: Some(snd_echo_digital_mode_get),
    put: Some(snd_echo_digital_mode_put),
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
unsafe fn snd_echo_spdif_mode_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 2] = [c"Consumer".as_ptr(), c"Professional".as_ptr()];
    snd_ctl_enum_info(uinfo, 1, 2, names.as_ptr())
}

#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
unsafe fn snd_echo_spdif_mode_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.enumerated.item[0] = ((*chip).professional_spdif != 0) as c_uint;
    0
}

#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
unsafe fn snd_echo_spdif_mode_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mode = ((*ucontrol).value.enumerated.item[0] != 0) as c_int;
    if mode != (*chip).professional_spdif {
        guard_spinlock_irq(addr_of_mut!((*chip).lock));
        set_professional_spdif(chip, mode);
        return 1;
    }
    0
}

#[cfg(ECHOCARD_HAS_DIGITAL_IO)]
static snd_echo_spdif_mode_switch: snd_kcontrol_new = snd_kcontrol_new {
    name: c"S/PDIF mode Switch".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(snd_echo_spdif_mode_info),
    get: Some(snd_echo_spdif_mode_get),
    put: Some(snd_echo_spdif_mode_put),
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_EXTERNAL_CLOCK)]
unsafe fn snd_echo_clock_source_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    static names: [*const c_char; 8] = [
        c"Internal".as_ptr(), c"Word".as_ptr(), c"Super".as_ptr(), c"S/PDIF".as_ptr(),
        c"ADAT".as_ptr(), c"ESync".as_ptr(), c"ESync96".as_ptr(), c"MTC".as_ptr(),
    ];
    let chip = snd_kcontrol_chip(kcontrol);
    snd_ctl_enum_info(uinfo, 1, (*chip).num_clock_sources, names.as_ptr())
}

#[cfg(ECHOCARD_HAS_EXTERNAL_CLOCK)]
unsafe fn snd_echo_clock_source_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let clock = (*chip).input_clock;
    for i in 0..(*chip).num_clock_sources {
        if clock == (*chip).clock_source_list[i as usize] {
            (*ucontrol).value.enumerated.item[0] = i as c_uint;
        }
    }
    0
}

#[cfg(ECHOCARD_HAS_EXTERNAL_CLOCK)]
unsafe fn snd_echo_clock_source_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let mut changed = 0;
    let eclock = (*ucontrol).value.enumerated.item[0];
    if eclock >= (*chip).input_clock_types { return -EINVAL; }
    let dclock = (*chip).clock_source_list[eclock as usize];
    if (*chip).input_clock != dclock {
        guard_mutex(addr_of_mut!((*chip).mode_mutex));
        guard_spinlock_irq(addr_of_mut!((*chip).lock));
        changed = set_input_clock(chip, dclock);
        if changed == 0 { changed = 1; } /* no errors */
    }
    if changed < 0 {
        dev_dbg((*(*chip).card).dev, c"seticlk val%d err 0x%x\n".as_ptr(), dclock, changed);
    }
    changed
}

#[cfg(ECHOCARD_HAS_EXTERNAL_CLOCK)]
static snd_echo_clock_source_switch: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Sample Clock Source".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    info: Some(snd_echo_clock_source_info),
    get: Some(snd_echo_clock_source_get),
    put: Some(snd_echo_clock_source_put),
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_PHANTOM_POWER)]
const snd_echo_phantom_power_info: snd_kcontrol_info_t = snd_ctl_boolean_mono_info;

#[cfg(ECHOCARD_HAS_PHANTOM_POWER)]
unsafe fn snd_echo_phantom_power_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = (*chip).phantom_power;
    0
}

#[cfg(ECHOCARD_HAS_PHANTOM_POWER)]
unsafe fn snd_echo_phantom_power_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let power = ((*ucontrol).value.integer.value[0] != 0) as c_int;
    let mut changed = 0;
    if (*chip).phantom_power != power {
        guard_spinlock_irq(addr_of_mut!((*chip).lock));
        changed = set_phantom_power(chip, power);
        if changed == 0 { changed = 1; } /* no errors */
    }
    changed
}

#[cfg(ECHOCARD_HAS_PHANTOM_POWER)]
static snd_echo_phantom_power_switch: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Phantom power Switch".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(snd_echo_phantom_power_info),
    get: Some(snd_echo_phantom_power_get),
    put: Some(snd_echo_phantom_power_put),
    ..snd_kcontrol_new::zero()
};

#[cfg(ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE)]
const snd_echo_automute_info: snd_kcontrol_info_t = snd_ctl_boolean_mono_info;

#[cfg(ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE)]
unsafe fn snd_echo_automute_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = (*chip).digital_in_automute;
    0
}

#[cfg(ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE)]
unsafe fn snd_echo_automute_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let automute = ((*ucontrol).value.integer.value[0] != 0) as c_int;
    let mut changed = 0;
    if (*chip).digital_in_automute != automute {
        guard_spinlock_irq(addr_of_mut!((*chip).lock));
        changed = set_input_auto_mute(chip, automute);
        if changed == 0 { changed = 1; } /* no errors */
    }
    changed
}

#[cfg(ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE)]
static snd_echo_automute_switch: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Digital Capture Switch (automute)".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    info: Some(snd_echo_automute_info),
    get: Some(snd_echo_automute_get),
    put: Some(snd_echo_automute_put),
    ..snd_kcontrol_new::zero()
};

const snd_echo_vumeters_switch_info: snd_kcontrol_info_t = snd_ctl_boolean_mono_info;

unsafe fn snd_echo_vumeters_switch_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    guard_spinlock_irq(addr_of_mut!((*chip).lock));
    set_meters_on(chip, (*ucontrol).value.integer.value[0]);
    1
}

static snd_echo_vumeters_switch: snd_kcontrol_new = snd_kcontrol_new {
    name: c"VU-meters Switch".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    access: SNDRV_CTL_ELEM_ACCESS_WRITE,
    info: Some(snd_echo_vumeters_switch_info),
    put: Some(snd_echo_vumeters_switch_put),
    ..snd_kcontrol_new::zero()
};

unsafe fn snd_echo_vumeters_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 96;
    (*uinfo).value.integer.min = ECHOGAIN_MINOUT;
    (*uinfo).value.integer.max = 0;
    0
}

unsafe fn snd_echo_vumeters_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    get_audio_meters(chip, (*ucontrol).value.integer.value.as_mut_ptr());
    0
}

static snd_echo_vumeters: snd_kcontrol_new = snd_kcontrol_new {
    name: c"VU-meters".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    info: Some(snd_echo_vumeters_info),
    get: Some(snd_echo_vumeters_get),
    tlv: snd_kcontrol_tlv { p: addr_of!(db_scale_output_gain) },
    ..snd_kcontrol_new::zero()
};

/*** Channels info - it exports informations about the number of channels ***/
unsafe fn snd_echo_channels_info_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 6;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1 << ECHO_CLOCK_NUMBER;
    0
}

unsafe fn snd_echo_channels_info_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let detected: c_int;
    let mut clocks: c_int;
    let mut src: c_int;

    (*ucontrol).value.integer.value[0] = num_busses_in(chip);
    (*ucontrol).value.integer.value[1] = num_analog_busses_in(chip);
    (*ucontrol).value.integer.value[2] = num_busses_out(chip);
    (*ucontrol).value.integer.value[3] = num_analog_busses_out(chip);
    (*ucontrol).value.integer.value[4] = num_pipes_out(chip);

    /* Compute the bitmask of the currently valid input clocks */
    detected = detect_input_clocks(chip);
    clocks = 0;
    src = (*chip).num_clock_sources - 1;
    let mut bit = ECHO_CLOCK_NUMBER - 1;
    loop {
        if (detected & (1 << bit)) != 0 {
            while src >= 0 {
                if bit == (*chip).clock_source_list[src as usize] {
                    clocks |= 1 << src;
                    break;
                }
                src -= 1;
            }
        }
        if bit == 0 { break; }
        bit -= 1;
    }
    (*ucontrol).value.integer.value[5] = clocks;
    0
}

static snd_echo_channels_info: snd_kcontrol_new = snd_kcontrol_new {
    name: c"Channels info".as_ptr(),
    iface: SNDRV_CTL_ELEM_IFACE_HWDEP,
    access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE,
    info: Some(snd_echo_channels_info_info),
    get: Some(snd_echo_channels_info_get),
    ..snd_kcontrol_new::zero()
};

/******************************************************************************
    IRQ Handling
******************************************************************************/
/* Check if a period has elapsed since last interrupt
 *
 * Don't make any updates to state; PCM core handles this with the
 * correct locks.
 *
 * \return true if a period has elapsed, otherwise false
 */
unsafe fn period_has_elapsed(substream: *mut snd_pcm_substream) -> bool {
    let runtime = (*substream).runtime;
    let pipe = (*runtime).private_data as *mut audiopipe;
    let counter: u32;
    let mut step: u32;
    let period_bytes: usize;

    if (*pipe).state != PIPE_STATE_STARTED {
        return false;
    }

    period_bytes = frames_to_bytes(runtime, (*runtime).period_size) as usize;
    counter = le32_to_cpu(*(*pipe).dma_counter); /* presumed atomic */
    step = counter.wrapping_sub((*pipe).last_period); /* handles wrapping */
    step -= step % period_bytes as u32; /* acknowledge whole periods only */

    if step == 0 {
        return false; /* haven't advanced a whole period yet */
    }

    (*pipe).last_period = (*pipe).last_period.wrapping_add(step); /* used exclusively by us */
    true
}

unsafe fn snd_echo_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut echoaudio;
    let mut st: c_int;

    spin_lock(addr_of_mut!((*chip).lock));
    st = service_irq(chip);
    if st < 0 {
        spin_unlock(addr_of_mut!((*chip).lock));
        return IRQ_NONE;
    }
    /* The hardware doesn't tell us which substream caused the irq,
    thus we have to check all running substreams. */
    for ss in 0..DSP_MAXPIPES {
        let substream = (*chip).substream[ss as usize];
        if !substream.is_null() && period_has_elapsed(substream) {
            spin_unlock(addr_of_mut!((*chip).lock));
            snd_pcm_period_elapsed(substream);
            spin_lock(addr_of_mut!((*chip).lock));
        }
    }
    spin_unlock(addr_of_mut!((*chip).lock));

    #[cfg(ECHOCARD_HAS_MIDI)]
    {
        if st > 0 && !(*chip).midi_in.is_null() {
            snd_rawmidi_receive((*chip).midi_in, (*chip).midi_buffer.as_mut_ptr(), st);
            dev_dbg((*(*chip).card).dev, c"rawmidi_iread=%d\n".as_ptr(), st);
        }
    }
    IRQ_HANDLED
}

/******************************************************************************
    Module construction / destruction
******************************************************************************/

unsafe fn snd_echo_free(card: *mut snd_card) {
    let chip = (*card).private_data as *mut echoaudio;

    if !(*chip).comm_page.is_null() {
        rest_in_peace(chip);
    }
    if (*chip).irq >= 0 {
        free_irq((*chip).irq, chip as *mut c_void);
    }
    /* release chip data */
    free_firmware_cache(chip);
}

/* <--snd_echo_probe() */
unsafe fn snd_echo_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip = (*card).private_data as *mut echoaudio;
    let mut err: c_int;
    let mut sz: usize;

    pci_write_config_byte(pci, PCI_LATENCY_TIMER, 0xC0);
    err = pcim_enable_device(pci);
    if err < 0 { return err; }
    pci_set_master(pci);

    /* Allocate chip if needed */
    spin_lock_init(addr_of_mut!((*chip).lock));
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;
    (*chip).opencount = 0;
    mutex_init(addr_of_mut!((*chip).mode_mutex));
    (*chip).can_set_rate = 1;

    /* PCI resource allocation */
    err = pcim_request_all_regions(pci, ECHOCARD_NAME);
    if err < 0 { return err; }

    (*chip).dsp_registers_phys = pci_resource_start(pci, 0);
    sz = pci_resource_len(pci, 0);
    if sz > PAGE_SIZE as usize {
        sz = PAGE_SIZE as usize; /* We map only the required part */
    }

    (*chip).dsp_registers = devm_ioremap(addr_of_mut!((*pci).dev), (*chip).dsp_registers_phys, sz);
    if (*chip).dsp_registers.is_null() {
        dev_err((*(*chip).card).dev, c"ioremap failed\n".as_ptr());
        return -ENOMEM;
    }

    if request_irq((*pci).irq, Some(snd_echo_interrupt), IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*(*chip).card).dev, c"cannot grab irq\n".as_ptr());
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    dev_dbg(
        (*card).dev,
        c"pci=%p irq=%d subdev=%04x Init hardware...\n".as_ptr(),
        (*chip).pci,
        (*chip).irq,
        (*(*chip).pci).subsystem_device,
    );

    (*card).private_free = Some(snd_echo_free);

    /* Create the DSP comm page - this is the area of memory used for most
    of the communication with the DSP, which accesses it via bus mastering */
    (*chip).commpage_dma_buf = snd_devm_alloc_pages(addr_of_mut!((*pci).dev), SNDRV_DMA_TYPE_DEV, size_of::<comm_page>());
    if (*chip).commpage_dma_buf.is_null() {
        return -ENOMEM;
    }
    (*chip).comm_page_phys = (*(*chip).commpage_dma_buf).addr;
    (*chip).comm_page = (*(*chip).commpage_dma_buf).area as *mut comm_page;

    err = init_hw(chip, (*(*chip).pci).device, (*(*chip).pci).subsystem_device);
    if err >= 0 {
        err = set_mixer_defaults(chip);
    }
    if err < 0 {
        dev_err((*card).dev, c"init_hw err=%d\n".as_ptr(), err);
        return err;
    }
    0
}

/* constructor */
unsafe fn __snd_echo_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = null_mut();
    let mut chip: *mut echoaudio;
    let mut dsp: *const c_char;
    let mut err: c_int;

    if dev >= SNDRV_CARDS { return -ENODEV; }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(addr_of_mut!((*pci).dev), index[dev as usize], id[dev as usize],
        THIS_MODULE, size_of_val_ptr::<echoaudio>(), addr_of_mut!(card));
    if err < 0 { return err; }
    chip = (*card).private_data as *mut echoaudio;

    err = snd_echo_create(card, pci);
    if err < 0 { return err; }

    strscpy((*card).driver.as_mut_ptr(), concat_cstr(c"Echo_", ECHOCARD_NAME));
    strscpy((*card).shortname.as_mut_ptr(), (*chip).card_name.as_ptr());

    dsp = c"56301".as_ptr();
    if (*pci_id).device == 0x3410 {
        dsp = c"56361".as_ptr();
    }

    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s rev.%d (DSP%s) at 0x%lx irq %i".as_ptr(),
        (*card).shortname.as_ptr(),
        (*pci_id).subdevice & 0x000f,
        dsp,
        (*chip).dsp_registers_phys as c_ulong,
        (*chip).irq,
    );

    err = snd_echo_new_pcm(chip);
    if err < 0 {
        dev_err((*(*chip).card).dev, c"new pcm error %d\n".as_ptr(), err);
        return err;
    }

    #[cfg(ECHOCARD_HAS_MIDI)]
    {
        if (*chip).has_midi != 0 {
            /* Some Mia's do not have midi */
            err = snd_echo_midi_create(card, chip);
            if err < 0 {
                dev_err((*(*chip).card).dev, c"new midi error %d\n".as_ptr(), err);
                return err;
            }
        }
    }

    #[cfg(ECHOCARD_HAS_VMIXER)]
    {
        snd_echo_vmixer.count = num_pipes_out(chip) * num_busses_out(chip);
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of_mut!(snd_echo_vmixer), chip as *mut c_void));
        if err < 0 { return err; }
        #[cfg(ECHOCARD_HAS_LINE_OUT_GAIN)]
        {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_line_output_gain), chip as *mut c_void));
            if err < 0 { return err; }
        }
    }
    #[cfg(not(ECHOCARD_HAS_VMIXER))]
    {
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_pcm_output_gain), chip as *mut c_void));
        if err < 0 { return err; }
    }

    #[cfg(ECHOCARD_HAS_INPUT_GAIN)]
    {
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_line_input_gain), chip as *mut c_void));
        if err < 0 { return err; }
    }

    #[cfg(ECHOCARD_HAS_INPUT_NOMINAL_LEVEL)]
    {
        if (*chip).hasnt_input_nominal_level == 0 {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_intput_nominal_level), chip as *mut c_void));
            if err < 0 { return err; }
        }
    }

    #[cfg(ECHOCARD_HAS_OUTPUT_NOMINAL_LEVEL)]
    {
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_output_nominal_level), chip as *mut c_void));
        if err < 0 { return err; }
    }

    err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_vumeters_switch), chip as *mut c_void));
    if err < 0 { return err; }
    err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_vumeters), chip as *mut c_void));
    if err < 0 { return err; }

    #[cfg(ECHOCARD_HAS_MONITOR)]
    {
        snd_echo_monitor_mixer.count = num_busses_in(chip) * num_busses_out(chip);
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of_mut!(snd_echo_monitor_mixer), chip as *mut c_void));
        if err < 0 { return err; }
    }

    #[cfg(ECHOCARD_HAS_DIGITAL_IN_AUTOMUTE)]
    {
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_automute_switch), chip as *mut c_void));
        if err < 0 { return err; }
    }

    err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_channels_info), chip as *mut c_void));
    if err < 0 { return err; }

    #[cfg(ECHOCARD_HAS_DIGITAL_MODE_SWITCH)]
    {
        /* Creates a list of available digital modes */
        (*chip).num_digital_modes = 0;
        for i in 0..6 {
            if ((*chip).digital_modes & (1 << i)) != 0 {
                (*chip).digital_mode_list[(*chip).num_digital_modes as usize] = i;
                (*chip).num_digital_modes += 1;
            }
        }
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_digital_mode_switch), chip as *mut c_void));
        if err < 0 { return err; }
    }

    #[cfg(ECHOCARD_HAS_EXTERNAL_CLOCK)]
    {
        /* Creates a list of available clock sources */
        (*chip).num_clock_sources = 0;
        for i in 0..10 {
            if ((*chip).input_clock_types & (1 << i)) != 0 {
                (*chip).clock_source_list[(*chip).num_clock_sources as usize] = i;
                (*chip).num_clock_sources += 1;
            }
        }
        if (*chip).num_clock_sources > 1 {
            (*chip).clock_src_ctl = snd_ctl_new1(addr_of!(snd_echo_clock_source_switch), chip as *mut c_void);
            err = snd_ctl_add((*chip).card, (*chip).clock_src_ctl);
            if err < 0 { return err; }
        }
    }

    #[cfg(ECHOCARD_HAS_DIGITAL_IO)]
    {
        err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_spdif_mode_switch), chip as *mut c_void));
        if err < 0 { return err; }
    }

    #[cfg(ECHOCARD_HAS_PHANTOM_POWER)]
    {
        if (*chip).has_phantom_power != 0 {
            err = snd_ctl_add((*chip).card, snd_ctl_new1(addr_of!(snd_echo_phantom_power_switch), chip as *mut c_void));
            if err < 0 { return err; }
        }
    }

    err = snd_card_register(card);
    if err < 0 { return err; }
    dev_info((*card).dev, c"Card registered: %s\n".as_ptr(), (*card).longname.as_ptr());

    pci_set_drvdata(pci, chip as *mut c_void);
    dev += 1;
    0
}

unsafe fn snd_echo_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    snd_card_free_on_error(addr_of_mut!((*pci).dev), __snd_echo_probe(pci, pci_id))
}

unsafe fn snd_echo_suspend(dev: *mut device) -> c_int {
    let chip = dev_get_drvdata(dev) as *mut echoaudio;

    #[cfg(ECHOCARD_HAS_MIDI)]
    {
        /* This call can sleep */
        if !(*chip).midi_out.is_null() {
            snd_echo_midi_output_trigger((*chip).midi_out, 0);
        }
    }
    spin_lock_irq(addr_of_mut!((*chip).lock));
    if wait_handshake(chip) != 0 {
        spin_unlock_irq(addr_of_mut!((*chip).lock));
        return -EIO;
    }
    clear_handshake(chip);
    if send_vector(chip, DSP_VC_GO_COMATOSE) < 0 {
        spin_unlock_irq(addr_of_mut!((*chip).lock));
        return -EIO;
    }
    spin_unlock_irq(addr_of_mut!((*chip).lock));

    (*chip).dsp_code = null_mut();
    free_irq((*chip).irq, chip as *mut c_void);
    (*chip).irq = -1;
    (*(*chip).card).sync_irq = -1;
    0
}

unsafe fn snd_echo_resume(dev: *mut device) -> c_int {
    let pci = to_pci_dev(dev);
    let chip = dev_get_drvdata(dev) as *mut echoaudio;
    let commpage: *mut comm_page;
    let commpage_bak: *mut comm_page;
    let pipe_alloc_mask: u32;
    let mut err: c_int;

    commpage = (*chip).comm_page;
    commpage_bak = kmemdup(commpage as *const c_void, size_of::<comm_page>(), GFP_KERNEL) as *mut comm_page;
    if commpage_bak.is_null() {
        return -ENOMEM;
    }

    err = init_hw(chip, (*(*chip).pci).device, (*(*chip).pci).subsystem_device);
    if err < 0 {
        kfree(commpage_bak as *mut c_void);
        dev_err(dev, c"resume init_hw err=%d\n".as_ptr(), err);
        return err;
    }

    /* Temporarily set chip->pipe_alloc_mask=0 otherwise restore_dsp_settings() fails. */
    pipe_alloc_mask = (*chip).pipe_alloc_mask;
    (*chip).pipe_alloc_mask = 0;
    err = restore_dsp_settings(chip);
    (*chip).pipe_alloc_mask = pipe_alloc_mask;
    if err < 0 {
        kfree(commpage_bak as *mut c_void);
        return err;
    }

    memcpy(addr_of_mut!((*commpage).audio_format) as *mut c_void,
        addr_of_mut!((*commpage_bak).audio_format) as *const c_void,
        size_of_val(addr_of_mut!((*commpage).audio_format)));
    memcpy(addr_of_mut!((*commpage).sglist_addr) as *mut c_void,
        addr_of_mut!((*commpage_bak).sglist_addr) as *const c_void,
        size_of_val(addr_of_mut!((*commpage).sglist_addr)));
    memcpy(addr_of_mut!((*commpage).midi_output) as *mut c_void,
        addr_of_mut!((*commpage_bak).midi_output) as *const c_void,
        size_of_val(addr_of_mut!((*commpage).midi_output)));
    kfree(commpage_bak as *mut c_void);

    if request_irq((*pci).irq, Some(snd_echo_interrupt), IRQF_SHARED, KBUILD_MODNAME, chip as *mut c_void) != 0 {
        dev_err((*(*chip).card).dev, c"cannot grab irq\n".as_ptr());
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*(*chip).card).sync_irq = (*chip).irq;
    dev_dbg(dev, c"resume irq=%d\n".as_ptr(), (*chip).irq);

    #[cfg(ECHOCARD_HAS_MIDI)]
    {
        if (*chip).midi_input_enabled != 0 {
            enable_midi_input(chip, true);
        }
        if !(*chip).midi_out.is_null() {
            snd_echo_midi_output_trigger((*chip).midi_out, 1);
        }
    }

    0
}

static snd_echo_pm: dev_pm_ops = DEFINE_SIMPLE_DEV_PM_OPS!(snd_echo_suspend, snd_echo_resume);

/******************************************************************************
    Everything starts and ends here
******************************************************************************/

/* pci_driver definition */
static mut echo_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_echo_ids,
    probe: Some(snd_echo_probe),
    driver: device_driver {
        pm: addr_of!(snd_echo_pm),
        ..device_driver::zero()
    },
    ..pci_driver::zero()
};

module_pci_driver!(echo_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
