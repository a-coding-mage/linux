// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA driver for the digigram lx6464es interface
 *
 * Copyright (c) 2008, 2009 Tim Blechmann <tim@klingt.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ptr;

/* Dependencies supplied by the original C includes:
 * linux/module.h, linux/init.h, linux/pci.h, linux/delay.h, linux/slab.h,
 * sound/initval.h, sound/control.h, sound/info.h, and "lx6464es.h".
 */
use crate::*;

/* MODULE_AUTHOR("Tim Blechmann"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("digigram lx6464es"); */

static mut index: [c_int; SNDRV_CARDS as usize] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS as usize] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS as usize] = SNDRV_DEFAULT_ENABLE_PNP;

/* module_param_array(index, int, NULL, 0444); */
/* MODULE_PARM_DESC(index, "Index value for Digigram LX6464ES interface."); */
/* module_param_array(id, charp, NULL, 0444); */
/* MODULE_PARM_DESC(id, "ID string for  Digigram LX6464ES interface."); */
/* module_param_array(enable, bool, NULL, 0444); */
/* MODULE_PARM_DESC(enable, "Enable/disable specific Digigram LX6464ES soundcards."); */

static card_name: [c_char; 9] = *b"LX6464ES\0" as [u8; 9] as [c_char; 9];

const PCI_DEVICE_ID_PLX_LX6464ES: u32 = PCI_DEVICE_ID_PLX_9056;

static snd_lx6464es_ids: [pci_device_id; 5] = [
    PCI_DEVICE_SUB(
        PCI_VENDOR_ID_PLX,
        PCI_DEVICE_ID_PLX_LX6464ES,
        PCI_VENDOR_ID_DIGIGRAM,
        PCI_SUBDEVICE_ID_DIGIGRAM_LX6464ES_SERIAL_SUBSYSTEM,
    ),
    /* LX6464ES */
    PCI_DEVICE_SUB(
        PCI_VENDOR_ID_PLX,
        PCI_DEVICE_ID_PLX_LX6464ES,
        PCI_VENDOR_ID_DIGIGRAM,
        PCI_SUBDEVICE_ID_DIGIGRAM_LX6464ES_CAE_SERIAL_SUBSYSTEM,
    ),
    /* LX6464ES-CAE */
    PCI_DEVICE_SUB(
        PCI_VENDOR_ID_PLX,
        PCI_DEVICE_ID_PLX_LX6464ES,
        PCI_VENDOR_ID_DIGIGRAM,
        PCI_SUBDEVICE_ID_DIGIGRAM_LX6464ESE_SERIAL_SUBSYSTEM,
    ),
    /* LX6464ESe */
    PCI_DEVICE_SUB(
        PCI_VENDOR_ID_PLX,
        PCI_DEVICE_ID_PLX_LX6464ES,
        PCI_VENDOR_ID_DIGIGRAM,
        PCI_SUBDEVICE_ID_DIGIGRAM_LX6464ESE_CAE_SERIAL_SUBSYSTEM,
    ),
    /* LX6464ESe-CAE */
    pci_device_id::zeroed(),
];

/* MODULE_DEVICE_TABLE(pci, snd_lx6464es_ids); */

/* PGO pour USERo dans le registre pci_0x06/loc_0xEC */
const CHIPSC_RESET_XILINX: c_long = 1_i64 as c_long << 16;

/* alsa callbacks */
static lx_caps: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_SYNC_START,
    formats: SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S16_BE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S24_3BE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_192000,
    rate_min: 8000,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 64,
    buffer_bytes_max: 64 * 2 * 3 * MICROBLAZE_IBL_MAX * MAX_STREAM_BUFFER,
    period_bytes_min: 2 * 2 * MICROBLAZE_IBL_MIN * 2,
    period_bytes_max: 4 * 64 * MICROBLAZE_IBL_MAX * MAX_STREAM_BUFFER,
    periods_min: 2,
    periods_max: MAX_STREAM_BUFFER,
    ..snd_pcm_hardware::zeroed()
};

unsafe fn lx_set_granularity(chip: *mut lx6464es, gran: u32) -> c_int {
    let mut err: c_int = 0;
    let mut snapped_gran: u32 = MICROBLAZE_IBL_MIN;

    dev_dbg!((*(*chip).card).dev, "->lx_set_granularity\n");

    /* blocksize is a power of 2 */
    while snapped_gran < gran && snapped_gran < MICROBLAZE_IBL_MAX {
        snapped_gran = snapped_gran.wrapping_mul(2);
    }

    if snapped_gran == (*chip).pcm_granularity {
        return 0;
    }

    err = lx_dsp_set_granularity(chip, snapped_gran);
    if err < 0 {
        dev_warn!((*(*chip).card).dev, "could not set granularity\n");
        err = -EAGAIN;
    }

    if snapped_gran != gran {
        dev_err!((*(*chip).card).dev, "snapped blocksize to %d\n", snapped_gran);
    }

    dev_dbg!((*(*chip).card).dev, "set blocksize on board %d\n", snapped_gran);
    (*chip).pcm_granularity = snapped_gran;

    err
}

unsafe fn lx_hardware_open(chip: *mut lx6464es, substream: *mut snd_pcm_substream) -> c_int {
    let mut err: c_int;
    let runtime = (*substream).runtime;
    let channels: c_int = (*runtime).channels as c_int;
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;
    let period_size: snd_pcm_uframes_t = (*runtime).period_size;

    dev_dbg!((*(*chip).card).dev, "allocating pipe for %d channels\n", channels);
    err = lx_pipe_allocate(chip, 0, is_capture, channels);
    if err < 0 {
        dev_err!((*(*chip).card).dev, LXP "allocating pipe failed\n");
        return err;
    }

    err = lx_set_granularity(chip, period_size as u32);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "setting granularity to %ld failed\n", period_size);
        return err;
    }

    0
}

unsafe fn lx_hardware_start(chip: *mut lx6464es, substream: *mut snd_pcm_substream) -> c_int {
    let mut err: c_int;
    let runtime = (*substream).runtime;
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

    dev_dbg!((*(*chip).card).dev, "setting stream format\n");
    err = lx_stream_set_format(chip, runtime, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "setting stream format failed\n");
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "starting pipe\n");
    err = lx_pipe_start(chip, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "starting pipe failed\n");
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "waiting for pipe to start\n");
    err = lx_pipe_wait_for_start(chip, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "waiting for pipe failed\n");
        return err;
    }

    err
}

unsafe fn lx_hardware_stop(chip: *mut lx6464es, substream: *mut snd_pcm_substream) -> c_int {
    let mut err: c_int;
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

    dev_dbg!((*(*chip).card).dev, "pausing pipe\n");
    err = lx_pipe_pause(chip, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "pausing pipe failed\n");
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "waiting for pipe to become idle\n");
    err = lx_pipe_wait_for_idle(chip, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "waiting for pipe failed\n");
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "stopping pipe\n");
    err = lx_pipe_stop(chip, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "stopping pipe failed\n");
        return err;
    }

    err
}

unsafe fn lx_hardware_close(chip: *mut lx6464es, substream: *mut snd_pcm_substream) -> c_int {
    let mut err: c_int;
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

    dev_dbg!((*(*chip).card).dev, "releasing pipe\n");
    err = lx_pipe_release(chip, 0, is_capture);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "releasing pipe failed\n");
        return err;
    }

    err
}

unsafe extern "C" fn lx_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut lx6464es = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut err: c_int;
    let board_rate: c_int;

    dev_dbg!((*(*chip).card).dev, "->lx_pcm_open\n");
    let _guard = guard_mutex(&mut (*chip).setup_mutex);

    /* copy the struct snd_pcm_hardware struct */
    (*runtime).hw = lx_caps;

    /* #if 0: buffer-size should better be multiple of period-size */
    /*
    err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if err < 0 {
        dev_warn!((*(*chip).card).dev, "could not constrain periods\n");
        return err;
    }
    */

    /* the clock rate cannot be changed */
    board_rate = (*chip).board_sample_rate as c_int;
    err = snd_pcm_hw_constraint_single(runtime, SNDRV_PCM_HW_PARAM_RATE, board_rate);
    if err < 0 {
        dev_warn!((*(*chip).card).dev, "could not constrain periods\n");
        return err;
    }

    /* constrain period size */
    err = snd_pcm_hw_constraint_minmax(
        runtime,
        SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
        MICROBLAZE_IBL_MIN,
        MICROBLAZE_IBL_MAX,
    );
    if err < 0 {
        dev_warn!((*(*chip).card).dev, "could not constrain period size\n");
        return err;
    }

    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_SIZE, 32);
    snd_pcm_set_sync(substream);
    err = 0;
    (*runtime).private_data = chip as *mut c_void;

    dev_dbg!((*(*chip).card).dev, "<-lx_pcm_open, %d\n", err);
    err
}

unsafe extern "C" fn lx_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    dev_dbg!((*(*(*substream).pcm).card).dev, "->lx_pcm_close\n");
    0
}

unsafe extern "C" fn lx_pcm_stream_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip: *mut lx6464es = snd_pcm_substream_chip(substream);
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;
    let lx_stream: *mut lx_stream = if is_capture != 0 {
        &mut (*chip).capture_stream
    } else {
        &mut (*chip).playback_stream
    };

    dev_dbg!((*(*chip).card).dev, "->lx_pcm_stream_pointer\n");
    let _guard = guard_mutex(&mut (*chip).lock);
    let pos: snd_pcm_uframes_t = (*lx_stream).frame_pos * (*(*substream).runtime).period_size;

    dev_dbg!((*(*chip).card).dev, "stream_pointer at %ld\n", pos);
    pos
}

unsafe extern "C" fn lx_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut lx6464es = snd_pcm_substream_chip(substream);
    let mut err: c_int;
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

    dev_dbg!((*(*chip).card).dev, "->lx_pcm_prepare\n");
    let _guard = guard_mutex(&mut (*chip).setup_mutex);

    if (*chip).hardware_running[is_capture as usize] != 0 {
        err = lx_hardware_stop(chip, substream);
        if err < 0 {
            dev_err!((*(*chip).card).dev, "failed to stop hardware. Error code %d\n", err);
            return err;
        }

        err = lx_hardware_close(chip, substream);
        if err < 0 {
            dev_err!((*(*chip).card).dev, "failed to close hardware. Error code %d\n", err);
            return err;
        }
    }

    dev_dbg!((*(*chip).card).dev, "opening hardware\n");
    err = lx_hardware_open(chip, substream);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "failed to open hardware. Error code %d\n", err);
        return err;
    }

    err = lx_hardware_start(chip, substream);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "failed to start hardware. Error code %d\n", err);
        return err;
    }

    (*chip).hardware_running[is_capture as usize] = 1;

    if (*chip).board_sample_rate != (*(*substream).runtime).rate {
        if err == 0 {
            (*chip).board_sample_rate = (*(*substream).runtime).rate;
        }
    }

    err
}

unsafe fn lx_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
    is_capture: c_int,
) -> c_int {
    let chip: *mut lx6464es = snd_pcm_substream_chip(substream);

    dev_dbg!((*(*chip).card).dev, "->lx_pcm_hw_params\n");
    let _guard = guard_mutex(&mut (*chip).setup_mutex);

    if is_capture != 0 {
        (*chip).capture_stream.stream = substream;
    } else {
        (*chip).playback_stream.stream = substream;
    }

    0
}

unsafe extern "C" fn lx_pcm_hw_params_playback(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    lx_pcm_hw_params(substream, hw_params, 0)
}

unsafe extern "C" fn lx_pcm_hw_params_capture(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    lx_pcm_hw_params(substream, hw_params, 1)
}

unsafe extern "C" fn lx_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut lx6464es = snd_pcm_substream_chip(substream);
    let mut err: c_int;
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

    dev_dbg!((*(*chip).card).dev, "->lx_pcm_hw_free\n");
    let _guard = guard_mutex(&mut (*chip).setup_mutex);

    if (*chip).hardware_running[is_capture as usize] != 0 {
        err = lx_hardware_stop(chip, substream);
        if err < 0 {
            dev_err!((*(*chip).card).dev, "failed to stop hardware. Error code %d\n", err);
            return err;
        }

        err = lx_hardware_close(chip, substream);
        if err < 0 {
            dev_err!((*(*chip).card).dev, "failed to close hardware. Error code %d\n", err);
            return err;
        }

        (*chip).hardware_running[is_capture as usize] = 0;
    }

    if is_capture != 0 {
        (*chip).capture_stream.stream = ptr::null_mut();
    } else {
        (*chip).playback_stream.stream = ptr::null_mut();
    }

    0
}

unsafe fn lx_trigger_start(chip: *mut lx6464es, lx_stream: *mut lx_stream) {
    let substream = (*lx_stream).stream;
    let is_capture: c_uint = (*lx_stream).is_capture;
    let mut err: c_int;
    let periods: u32 = (*(*substream).runtime).periods;
    let period_bytes: u32 = snd_pcm_lib_period_bytes(substream);
    let mut buf: dma_addr_t = (*substream).dma_buffer.addr;
    let mut i: c_int;
    let mut needed: u32 = 0;
    let mut freed: u32 = 0;
    let mut size_array: [u32; 5] = [0; 5];

    i = 0;
    while i != periods as c_int {
        let mut buffer_index: u32 = 0;

        err = lx_buffer_ask(
            chip,
            0,
            is_capture as c_int,
            &mut needed,
            &mut freed,
            size_array.as_mut_ptr(),
        );
        let _ = err;
        dev_dbg!((*(*chip).card).dev, "starting: needed %d, freed %d\n", needed, freed);

        err = lx_buffer_give(
            chip,
            0,
            is_capture as c_int,
            period_bytes,
            lower_32_bits(buf),
            upper_32_bits(buf),
            &mut buffer_index,
        );
        let _ = err;

        dev_dbg!(
            (*(*chip).card).dev,
            "starting: buffer index %x on 0x%lx (%d bytes)\n",
            buffer_index,
            buf as c_ulong,
            period_bytes
        );
        buf = buf.wrapping_add(period_bytes as dma_addr_t);
        i += 1;
    }

    err = lx_buffer_ask(
        chip,
        0,
        is_capture as c_int,
        &mut needed,
        &mut freed,
        size_array.as_mut_ptr(),
    );
    let _ = err;
    dev_dbg!((*(*chip).card).dev, "starting: needed %d, freed %d\n", needed, freed);

    dev_dbg!((*(*chip).card).dev, "starting: starting stream\n");
    err = lx_stream_start(chip, 0, is_capture as c_int);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "couldn't start stream\n");
    } else {
        (*lx_stream).status = LX_STREAM_STATUS_RUNNING;
    }

    (*lx_stream).frame_pos = 0;
}

unsafe fn lx_trigger_stop(chip: *mut lx6464es, lx_stream: *mut lx_stream) {
    let is_capture: c_uint = (*lx_stream).is_capture;
    let err: c_int;

    dev_dbg!((*(*chip).card).dev, "stopping: stopping stream\n");
    err = lx_stream_stop(chip, 0, is_capture as c_int);
    if err < 0 {
        dev_err!((*(*chip).card).dev, "couldn't stop stream\n");
    } else {
        (*lx_stream).status = LX_STREAM_STATUS_FREE;
    }
}

unsafe fn lx_trigger_dispatch_stream(chip: *mut lx6464es, lx_stream: *mut lx_stream) {
    match (*lx_stream).status {
        LX_STREAM_STATUS_SCHEDULE_RUN => lx_trigger_start(chip, lx_stream),
        LX_STREAM_STATUS_SCHEDULE_STOP => lx_trigger_stop(chip, lx_stream),
        _ => {}
    }
}

unsafe fn lx_pcm_trigger_dispatch(
    chip: *mut lx6464es,
    lx_stream: *mut lx_stream,
    cmd: c_int,
) -> c_int {
    let _guard = guard_mutex(&mut (*chip).lock);
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            (*lx_stream).status = LX_STREAM_STATUS_SCHEDULE_RUN;
        }
        SNDRV_PCM_TRIGGER_STOP => {
            (*lx_stream).status = LX_STREAM_STATUS_SCHEDULE_STOP;
        }
        _ => return -EINVAL,
    }

    lx_trigger_dispatch_stream(chip, &mut (*chip).capture_stream);
    lx_trigger_dispatch_stream(chip, &mut (*chip).playback_stream);

    0
}

unsafe extern "C" fn lx_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int {
    let chip: *mut lx6464es = snd_pcm_substream_chip(substream);
    let is_capture: c_int = ((*substream).stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;
    let stream: *mut lx_stream = if is_capture != 0 {
        &mut (*chip).capture_stream
    } else {
        &mut (*chip).playback_stream
    };

    dev_dbg!((*(*chip).card).dev, "->lx_pcm_trigger\n");

    lx_pcm_trigger_dispatch(chip, stream, cmd)
}

unsafe extern "C" fn snd_lx6464es_free(card: *mut snd_card) {
    let chip: *mut lx6464es = (*card).private_data as *mut lx6464es;
    lx_irq_disable(chip);
}

/* reset the dsp during initialization */
unsafe fn lx_init_xilinx_reset(chip: *mut lx6464es) -> c_int {
    let mut i: c_int;
    let mut plx_reg: u32 = lx_plx_reg_read(chip, ePLX_CHIPSC);

    dev_dbg!((*(*chip).card).dev, "->lx_init_xilinx_reset\n");

    /* activate reset of xilinx */
    plx_reg &= !(CHIPSC_RESET_XILINX as u32);

    lx_plx_reg_write(chip, ePLX_CHIPSC, plx_reg);
    msleep(1);

    lx_plx_reg_write(chip, ePLX_MBOX3, 0);
    msleep(1);

    plx_reg |= CHIPSC_RESET_XILINX as u32;
    lx_plx_reg_write(chip, ePLX_CHIPSC, plx_reg);

    /* deactivate reset of xilinx */
    i = 0;
    while i != 100 {
        let reg_mbox3: u32;
        msleep(10);
        reg_mbox3 = lx_plx_reg_read(chip, ePLX_MBOX3);
        if reg_mbox3 != 0 {
            dev_dbg!((*(*chip).card).dev, "xilinx reset done\n");
            dev_dbg!((*(*chip).card).dev, "xilinx took %d loops\n", i);
            break;
        }
        i += 1;
    }

    /* todo: add some error handling? */

    /* clear mr */
    lx_dsp_reg_write(chip, eReg_CSM, 0);

    /* le xilinx ES peut ne pas etre encore pret, on attend. */
    msleep(600);

    0
}

unsafe fn lx_init_xilinx_test(chip: *mut lx6464es) -> c_int {
    let mut reg: u32;

    dev_dbg!((*(*chip).card).dev, "->lx_init_xilinx_test\n");

    /* TEST if we have access to Xilinx/MicroBlaze */
    lx_dsp_reg_write(chip, eReg_CSM, 0);

    reg = lx_dsp_reg_read(chip, eReg_CSM);

    if reg != 0 {
        dev_err!((*(*chip).card).dev, "Problem: Reg_CSM %x.\n", reg);

        /* PCI9056_SPACE0_REMAP */
        lx_plx_reg_write(chip, ePLX_PCICR, 1);

        reg = lx_dsp_reg_read(chip, eReg_CSM);
        if reg != 0 {
            dev_err!((*(*chip).card).dev, "Error: Reg_CSM %x.\n", reg);
            return -EAGAIN; /* seems to be appropriate */
        }
    }

    dev_dbg!((*(*chip).card).dev, "Xilinx/MicroBlaze access test successful\n");

    0
}

/* initialize ethersound */
unsafe fn lx_init_ethersound_config(chip: *mut lx6464es) -> c_int {
    let mut i: c_int;
    let orig_conf_es: u32 = lx_dsp_reg_read(chip, eReg_CONFES);

    /* configure 64 io channels */
    let conf_es: u32 = (orig_conf_es & CONFES_READ_PART_MASK)
        | (64 << IOCR_INPUTS_OFFSET)
        | (64 << IOCR_OUTPUTS_OFFSET)
        | (FREQ_RATIO_SINGLE_MODE << FREQ_RATIO_OFFSET);

    dev_dbg!((*(*chip).card).dev, "->lx_init_ethersound\n");

    (*chip).freq_ratio = FREQ_RATIO_SINGLE_MODE;

    /*
     * write it to the card !
     * this actually kicks the ES xilinx, the first time since poweron.
     * the MAC address in the Reg_ADMACESMSB Reg_ADMACESLSB registers
     * is not ready before this is done, and the bit 2 in Reg_CSES is set.
     */
    lx_dsp_reg_write(chip, eReg_CONFES, conf_es);

    i = 0;
    while i != 1000 {
        if (lx_dsp_reg_read(chip, eReg_CSES) & 4) != 0 {
            dev_dbg!((*(*chip).card).dev, "ethersound initialized after %dms\n", i);
            dev_dbg!((*(*chip).card).dev, "ethersound initialized\n");
            return 0;
        }
        msleep(1);
        i += 1;
    }
    dev_warn!((*(*chip).card).dev, "ethersound could not be initialized after %dms\n", i);
    -ETIMEDOUT
}

unsafe fn lx_init_get_version_features(chip: *mut lx6464es) -> c_int {
    let mut dsp_version: u32 = 0;
    let mut err: c_int;

    dev_dbg!((*(*chip).card).dev, "->lx_init_get_version_features\n");

    err = lx_dsp_get_version(chip, &mut dsp_version);

    if err == 0 {
        let mut freq: u32 = 0;

        dev_info!(
            (*(*chip).card).dev,
            "DSP version: V%02d.%02d #%d\n",
            (dsp_version >> 16) & 0xff,
            (dsp_version >> 8) & 0xff,
            dsp_version & 0xff
        );

        /* later: what firmware version do we expect? */

        /* retrieve Play/Rec features */
        /* done here because we may have to handle alternate
         * DSP files. */
        /* later */

        /* init the EtherSound sample rate */
        err = lx_dsp_get_clock_frequency(chip, &mut freq);
        if err == 0 {
            (*chip).board_sample_rate = freq;
        }
        dev_dbg!((*(*chip).card).dev, "actual clock frequency %d\n", freq);
    } else {
        dev_err!((*(*chip).card).dev, "DSP corrupted \n");
        err = -EAGAIN;
    }

    err
}

/* initialize and test the xilinx dsp chip */
unsafe fn lx_init_dsp(chip: *mut lx6464es) -> c_int {
    let mut err: c_int;
    let mut i: c_int;

    dev_dbg!((*(*chip).card).dev, "->lx_init_dsp\n");

    dev_dbg!((*(*chip).card).dev, "initialize board\n");
    err = lx_init_xilinx_reset(chip);
    if err != 0 {
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "testing board\n");
    err = lx_init_xilinx_test(chip);
    if err != 0 {
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "initialize ethersound configuration\n");
    err = lx_init_ethersound_config(chip);
    if err != 0 {
        return err;
    }

    lx_irq_enable(chip);

    /** \todo the mac address should be ready by not, but it isn't,
     *  so we wait for it */
    i = 0;
    while i != 1000 {
        err = lx_dsp_get_mac(chip);
        if err != 0 {
            return err;
        }
        if (*chip).mac_address[0] != 0
            || (*chip).mac_address[1] != 0
            || (*chip).mac_address[2] != 0
            || (*chip).mac_address[3] != 0
            || (*chip).mac_address[4] != 0
            || (*chip).mac_address[5] != 0
        {
            dev_dbg!((*(*chip).card).dev, "mac address ready read after: %dms\n", i);
            dev_info!(
                (*(*chip).card).dev,
                "mac address: %02X.%02X.%02X.%02X.%02X.%02X\n",
                (*chip).mac_address[0],
                (*chip).mac_address[1],
                (*chip).mac_address[2],
                (*chip).mac_address[3],
                (*chip).mac_address[4],
                (*chip).mac_address[5]
            );

            err = lx_init_get_version_features(chip);
            if err != 0 {
                return err;
            }

            lx_set_granularity(chip, MICROBLAZE_IBL_DEFAULT);
            (*chip).playback_mute = 0;
            return err;
        }
        msleep(1);
        i += 1;
    }
    -ETIMEDOUT
}

static lx_ops_playback: snd_pcm_ops = snd_pcm_ops {
    open: Some(lx_pcm_open),
    close: Some(lx_pcm_close),
    prepare: Some(lx_pcm_prepare),
    hw_params: Some(lx_pcm_hw_params_playback),
    hw_free: Some(lx_pcm_hw_free),
    trigger: Some(lx_pcm_trigger),
    pointer: Some(lx_pcm_stream_pointer),
    ..snd_pcm_ops::zeroed()
};

static lx_ops_capture: snd_pcm_ops = snd_pcm_ops {
    open: Some(lx_pcm_open),
    close: Some(lx_pcm_close),
    prepare: Some(lx_pcm_prepare),
    hw_params: Some(lx_pcm_hw_params_capture),
    hw_free: Some(lx_pcm_hw_free),
    trigger: Some(lx_pcm_trigger),
    pointer: Some(lx_pcm_stream_pointer),
    ..snd_pcm_ops::zeroed()
};

unsafe fn lx_pcm_create(chip: *mut lx6464es) -> c_int {
    let mut err: c_int;
    let mut pcm: *mut snd_pcm = ptr::null_mut();

    let mut size: u32 = 64 /* channels */
        * 3 /* 24 bit samples */
        * MAX_STREAM_BUFFER /* periods */
        * MICROBLAZE_IBL_MAX /* frames per period */
        * 2; /* duplex */

    size = PAGE_ALIGN(size);

    /* hardcoded device name & channel count */
    err = snd_pcm_new((*chip).card, card_name.as_ptr() as *mut c_char, 0, 1, 1, &mut pcm);
    if err < 0 {
        return err;
    }

    (*pcm).private_data = chip as *mut c_void;

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &lx_ops_playback);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &lx_ops_capture);

    (*pcm).info_flags = 0;
    (*pcm).nonatomic = true;
    strscpy((*pcm).name.as_mut_ptr(), card_name.as_ptr());

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        &mut (*(*chip).pci).dev,
        size,
        size,
    );

    (*chip).pcm = pcm;
    (*chip).capture_stream.is_capture = 1;

    0
}

unsafe extern "C" fn lx_control_playback_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn lx_control_playback_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut lx6464es = snd_kcontrol_chip(kcontrol);
    (*ucontrol).value.integer.value[0] = (*chip).playback_mute as c_long;
    0
}

unsafe extern "C" fn lx_control_playback_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut lx6464es = snd_kcontrol_chip(kcontrol);
    let mut changed: c_int = 0;
    let current_value: c_int = (*chip).playback_mute;

    if current_value as c_long != (*ucontrol).value.integer.value[0] {
        lx_level_unmute(chip, 0, (!current_value) as c_int);
        (*chip).playback_mute = (!current_value) as c_int;
        changed = 1;
    }
    changed
}

static lx_control_playback_switch: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: b"PCM Playback Switch\0".as_ptr() as *const c_char,
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE,
    private_value: 0,
    info: Some(lx_control_playback_info),
    get: Some(lx_control_playback_get),
    put: Some(lx_control_playback_put),
    ..snd_kcontrol_new::zeroed()
};

unsafe extern "C" fn lx_proc_levels_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let mut levels: [u32; 64] = [0; 64];
    let mut err: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let chip: *mut lx6464es = (*entry).private_data as *mut lx6464es;

    snd_iprintf!(buffer, "capture levels:\n");
    err = lx_level_peaks(chip, 1, 64, levels.as_mut_ptr());
    if err < 0 {
        return;
    }

    i = 0;
    while i != 8 {
        j = 0;
        while j != 8 {
            snd_iprintf!(buffer, "%08x ", levels[(i * 8 + j) as usize]);
            j += 1;
        }
        snd_iprintf!(buffer, "\n");
        i += 1;
    }

    snd_iprintf!(buffer, "\nplayback levels:\n");

    err = lx_level_peaks(chip, 0, 64, levels.as_mut_ptr());
    if err < 0 {
        return;
    }

    i = 0;
    while i != 8 {
        j = 0;
        while j != 8 {
            snd_iprintf!(buffer, "%08x ", levels[(i * 8 + j) as usize]);
            j += 1;
        }
        snd_iprintf!(buffer, "\n");
        i += 1;
    }

    snd_iprintf!(buffer, "\n");
}

unsafe fn lx_proc_create(card: *mut snd_card, chip: *mut lx6464es) -> c_int {
    snd_card_ro_proc_new(card, b"levels\0".as_ptr() as *const c_char, chip as *mut c_void, Some(lx_proc_levels_read))
}

unsafe fn snd_lx6464es_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip: *mut lx6464es = (*card).private_data as *mut lx6464es;
    let mut err: c_int;

    dev_dbg!((*card).dev, "->snd_lx6464es_create\n");

    /* enable PCI device */
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    pci_set_master(pci);

    /* check if we can restrict PCI DMA transfers to 32 bits */
    err = dma_set_mask(&mut (*pci).dev, DMA_BIT_MASK(32));
    if err < 0 {
        dev_err!(
            (*card).dev,
            "architecture does not support 32bit PCI busmaster DMA\n"
        );
        return -ENXIO;
    }

    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).irq = -1;

    /* initialize synchronization structs */
    mutex_init(&mut (*chip).lock);
    mutex_init(&mut (*chip).msg_lock);
    mutex_init(&mut (*chip).setup_mutex);

    /* request resources */
    err = pcim_request_all_regions(pci, card_name.as_ptr());
    if err < 0 {
        return err;
    }

    /* plx port */
    (*chip).port_plx = pci_resource_start(pci, 1);
    (*chip).port_plx_remapped = devm_ioport_map(
        &mut (*pci).dev,
        (*chip).port_plx,
        pci_resource_len(pci, 1),
    );
    if (*chip).port_plx_remapped.is_null() {
        return -ENOMEM;
    }

    /* dsp port */
    (*chip).port_dsp_bar = pcim_iomap(pci, 2, 0);
    if (*chip).port_dsp_bar.is_null() {
        return -ENOMEM;
    }

    err = devm_request_threaded_irq(
        &mut (*pci).dev,
        (*pci).irq,
        Some(lx_interrupt),
        Some(lx_threaded_irq),
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut c_void,
    );
    if err != 0 {
        dev_err!((*card).dev, "unable to grab IRQ %d\n", (*pci).irq);
        return err;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_lx6464es_free);

    err = lx_init_dsp(chip);
    if err < 0 {
        dev_err!((*card).dev, "error during DSP initialization\n");
        return err;
    }

    err = lx_pcm_create(chip);
    if err < 0 {
        return err;
    }

    err = lx_proc_create(card, chip);
    if err < 0 {
        return err;
    }

    err = snd_ctl_add(card, snd_ctl_new1(&lx_control_playback_switch, chip as *mut c_void));
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn snd_lx6464es_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut lx6464es;
    let mut err: c_int;

    dev_dbg!(&mut (*pci).dev, "->snd_lx6464es_probe\n");

    if dev >= SNDRV_CARDS {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(
        &mut (*pci).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<lx6464es>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut lx6464es;

    err = snd_lx6464es_create(card, pci);
    if err < 0 {
        dev_err!((*card).dev, "error during snd_lx6464es_create\n");
        snd_card_free(card);
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), b"LX6464ES\0".as_ptr() as *const c_char);
    sprintf!(
        (*card).id.as_mut_ptr(),
        "LX6464ES_%02X%02X%02X",
        (*chip).mac_address[3],
        (*chip).mac_address[4],
        (*chip).mac_address[5]
    );

    sprintf!(
        (*card).shortname.as_mut_ptr(),
        "LX6464ES %02X.%02X.%02X.%02X.%02X.%02X",
        (*chip).mac_address[0],
        (*chip).mac_address[1],
        (*chip).mac_address[2],
        (*chip).mac_address[3],
        (*chip).mac_address[4],
        (*chip).mac_address[5]
    );

    sprintf!(
        (*card).longname.as_mut_ptr(),
        "%s at 0x%lx, 0x%p, irq %i",
        (*card).shortname.as_ptr(),
        (*chip).port_plx,
        (*chip).port_dsp_bar,
        (*chip).irq
    );

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    dev_dbg!((*(*chip).card).dev, "initialization successful\n");
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

static mut lx6464es_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_lx6464es_ids.as_ptr(),
    probe: Some(snd_lx6464es_probe),
    ..pci_driver::zeroed()
};

/* module_pci_driver(lx6464es_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
