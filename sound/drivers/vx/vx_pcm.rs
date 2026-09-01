// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX soundcards
 *
 * PCM part
 *
 * Copyright (c) 2002,2003 by Takashi Iwai <tiwai@suse.de>
 *
 * STRATEGY
 *  for playback, we send series of "chunks", which size is equal with the
 *  IBL size, typically 126 samples.  at each end of chunk, the end-of-buffer
 *  interrupt is notified, and the interrupt handler will feed the next chunk.
 *
 *  the current position is calculated from the sample count RMH.
 *  pipe->transferred is the counter of data which has been already transferred.
 *  if this counter reaches to the period size, snd_pcm_period_elapsed() will
 *  be issued.
 *
 *  for capture, the situation is much easier.
 *  to get a low latency response, we'll check the capture streams at each
 *  interrupt (capture stream has no EOB notification).  if the pending
 *  data is accumulated to the period size, snd_pcm_period_elapsed() is
 *  called and the pointer is updated.
 *
 *  the current point of read buffer is kept in pipe->hw_ptr.  note that
 *  this is in bytes.
 *
 * TODO
 *  - linked trigger for full-duplex mode.
 *  - scheduled action on the stream.
 */

use crate::*;

/*
 * read three pending pcm bytes via inb()
 */
unsafe fn vx_pcm_read_per_bytes(
    chip: *mut vx_core,
    runtime: *mut snd_pcm_runtime,
    pipe: *mut vx_pipe,
) {
    let mut offset: i32 = (*pipe).hw_ptr;
    let mut buf = ((*runtime).dma_area as *mut u8).add(offset as usize);
    *buf = vx_inb(chip, RXH);
    buf = buf.add(1);
    offset += 1;
    if offset >= (*pipe).buffer_bytes {
        offset = 0;
        buf = (*runtime).dma_area as *mut u8;
    }
    *buf = vx_inb(chip, RXM);
    buf = buf.add(1);
    offset += 1;
    if offset >= (*pipe).buffer_bytes {
        offset = 0;
        buf = (*runtime).dma_area as *mut u8;
    }
    *buf = vx_inb(chip, RXL);
    offset += 1;
    if offset >= (*pipe).buffer_bytes {
        offset = 0;
    }
    (*pipe).hw_ptr = offset;
}

/*
 * vx_set_pcx_time - convert from the PC time to the RMH status time.
 * @pc_time: the pointer for the PC-time to set
 * @dsp_time: the pointer for RMH status time array
 */
unsafe fn vx_set_pcx_time(
    _chip: *mut vx_core,
    pc_time: *mut pcx_time_t,
    dsp_time: *mut u32,
) {
    *dsp_time.add(0) = (((*pc_time) >> 24) as u32) & PCX_TIME_HI_MASK;
    *dsp_time.add(1) = ((*pc_time) as u32) & MASK_DSP_WORD;
}

/*
 * vx_set_differed_time - set the differed time if specified
 * @rmh: the rmh record to modify
 * @pipe: the pipe to be checked
 *
 * if the pipe is programmed with the differed time, set the DSP time
 * on the rmh and changes its command length.
 *
 * returns the increase of the command length.
 */
unsafe fn vx_set_differed_time(
    chip: *mut vx_core,
    rmh: *mut vx_rmh,
    pipe: *mut vx_pipe,
) -> i32 {
    /* Update The length added to the RMH command by the timestamp */
    if ((*pipe).differed_type & DC_DIFFERED_DELAY) == 0 {
        return 0;
    }

    /* Set the T bit */
    (*rmh).Cmd[0] |= DSP_DIFFERED_COMMAND_MASK;

    /* Time stamp is the 1st following parameter */
    vx_set_pcx_time(chip, &mut (*pipe).pcx_time, &mut (*rmh).Cmd[1]);

    /* Add the flags to a notified differed command */
    if ((*pipe).differed_type & DC_NOTIFY_DELAY) != 0 {
        (*rmh).Cmd[1] |= NOTIFY_MASK_TIME_HIGH;
    }

    /* Add the flags to a multiple differed command */
    if ((*pipe).differed_type & DC_MULTIPLE_DELAY) != 0 {
        (*rmh).Cmd[1] |= MULTIPLE_MASK_TIME_HIGH;
    }

    /* Add the flags to a stream-time differed command */
    if ((*pipe).differed_type & DC_STREAM_TIME_DELAY) != 0 {
        (*rmh).Cmd[1] |= STREAM_MASK_TIME_HIGH;
    }

    (*rmh).LgCmd += 2;
    2
}

/*
 * vx_set_stream_format - send the stream format command
 * @pipe: the affected pipe
 * @data: format bitmask
 */
unsafe fn vx_set_stream_format(chip: *mut vx_core, pipe: *mut vx_pipe, data: u32) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(
        &mut rmh,
        if (*pipe).is_capture != 0 {
            CMD_FORMAT_STREAM_IN
        } else {
            CMD_FORMAT_STREAM_OUT
        },
    );
    rmh.Cmd[0] |= ((*pipe).number << FIELD_SIZE) as u32;

    /* Command might be longer since we may have to add a timestamp */
    vx_set_differed_time(chip, &mut rmh, pipe);

    rmh.Cmd[rmh.LgCmd as usize] = (data & 0xFFFFFF00) >> 8;
    rmh.Cmd[rmh.LgCmd as usize + 1] = (data & 0xFF) << 16 /*| (datal & 0xFFFF00) >> 8*/;
    rmh.LgCmd += 2;

    vx_send_msg(chip, &mut rmh)
}

/*
 * vx_set_format - set the format of a pipe
 * @pipe: the affected pipe
 * @runtime: pcm runtime instance to be referred
 *
 * returns 0 if successful, or a negative error code.
 */
unsafe fn vx_set_format(
    chip: *mut vx_core,
    pipe: *mut vx_pipe,
    runtime: *mut snd_pcm_runtime,
) -> i32 {
    let mut header: u32 = HEADER_FMT_BASE;

    if (*runtime).channels == 1 {
        header |= HEADER_FMT_MONO;
    }
    if snd_pcm_format_little_endian((*runtime).format) != 0 {
        header |= HEADER_FMT_INTEL;
    }
    if (*runtime).rate < 32000 && (*runtime).rate > 11025 {
        header |= HEADER_FMT_UPTO32;
    } else if (*runtime).rate <= 11025 {
        header |= HEADER_FMT_UPTO11;
    }

    match snd_pcm_format_physical_width((*runtime).format) {
        // case 8: break;
        16 => header |= HEADER_FMT_16BITS,
        24 => header |= HEADER_FMT_24BITS,
        _ => {
            snd_BUG();
            return -EINVAL;
        }
    }

    vx_set_stream_format(chip, pipe, header)
}

/*
 * set / query the IBL size
 */
unsafe fn vx_set_ibl(chip: *mut vx_core, info: *mut vx_ibl_info) -> i32 {
    let mut err: i32;
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_IBL);
    rmh.Cmd[0] |= (*info).size & 0x03ffff;
    err = vx_send_msg(chip, &mut rmh);
    if err < 0 {
        return err;
    }
    (*info).size = rmh.Stat[0];
    (*info).max_size = rmh.Stat[1];
    (*info).min_size = rmh.Stat[2];
    (*info).granularity = rmh.Stat[3];
    dev_dbg(
        (*(*chip).card).dev,
        c"%s: size = %d, max = %d, min = %d, gran = %d\n".as_ptr(),
        c"vx_set_ibl".as_ptr(),
        (*info).size,
        (*info).max_size,
        (*info).min_size,
        (*info).granularity,
    );
    0
}

/*
 * vx_get_pipe_state - get the state of a pipe
 * @pipe: the pipe to be checked
 * @state: the pointer for the returned state
 *
 * checks the state of a given pipe, and stores the state (1 = running,
 * 0 = paused) on the given pointer.
 *
 * called from trigger callback only
 */
unsafe fn vx_get_pipe_state(chip: *mut vx_core, pipe: *mut vx_pipe, state: *mut i32) -> i32 {
    let mut err: i32;
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_PIPE_STATE);
    vx_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number, 0);
    err = vx_send_msg(chip, &mut rmh);
    if err == 0 {
        *state = if (rmh.Stat[0] & (1 << (*pipe).number)) != 0 { 1 } else { 0 };
    }
    err
}

/*
 * vx_query_hbuffer_size - query available h-buffer size in bytes
 * @pipe: the pipe to be checked
 *
 * return the available size on h-buffer in bytes,
 * or a negative error code.
 *
 * NOTE: calling this function always switches to the stream mode.
 *       you'll need to disconnect the host to get back to the
 *       normal mode.
 */
unsafe fn vx_query_hbuffer_size(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut result: i32;
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_SIZE_HBUFFER);
    vx_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number, 0);
    if (*pipe).is_capture != 0 {
        rmh.Cmd[0] |= 0x00000001;
    }
    result = vx_send_msg(chip, &mut rmh);
    if result == 0 {
        result = (rmh.Stat[0] & 0xffff) as i32;
    }
    result
}

/*
 * vx_pipe_can_start - query whether a pipe is ready for start
 * @pipe: the pipe to be checked
 *
 * return 1 if ready, 0 if not ready, and negative value on error.
 *
 * called from trigger callback only
 */
unsafe fn vx_pipe_can_start(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut err: i32;
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_CAN_START_PIPE);
    vx_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number, 0);
    rmh.Cmd[0] |= 1;

    err = vx_send_msg(chip, &mut rmh);
    if err == 0 {
        if rmh.Stat[0] != 0 {
            err = 1;
        }
    }
    err
}

/*
 * vx_conf_pipe - tell the pipe to stand by and wait for IRQA.
 * @pipe: the pipe to be configured
 */
unsafe fn vx_conf_pipe(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_CONF_PIPE);
    if (*pipe).is_capture != 0 {
        rmh.Cmd[0] |= COMMAND_RECORD_MASK;
    }
    rmh.Cmd[1] = 1 << (*pipe).number;
    vx_send_msg(chip, &mut rmh)
}

/*
 * vx_send_irqa - trigger IRQA
 */
unsafe fn vx_send_irqa(chip: *mut vx_core) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_SEND_IRQA);
    vx_send_msg(chip, &mut rmh)
}

const MAX_WAIT_FOR_DSP: i32 = 250;
/*
 * vx boards do not support inter-card sync, besides
 * only 126 samples require to be prepared before a pipe can start
 */
const CAN_START_DELAY: i32 = 2; /* wait 2ms only before asking if the pipe is ready*/
const WAIT_STATE_DELAY: i32 = 2; /* wait 2ms after irqA was requested and check if the pipe state toggled*/

/*
 * vx_toggle_pipe - start / pause a pipe
 * @pipe: the pipe to be triggered
 * @state: start = 1, pause = 0
 *
 * called from trigger callback only
 *
 */
unsafe fn vx_toggle_pipe(chip: *mut vx_core, pipe: *mut vx_pipe, state: i32) -> i32 {
    let mut err: i32;
    let mut cur_state: i32 = 0;

    /* Check the pipe is not already in the requested state */
    if vx_get_pipe_state(chip, pipe, &mut cur_state) < 0 {
        return -EBADFD;
    }
    if state == cur_state {
        return 0;
    }

    /* If a start is requested, ask the DSP to get prepared
     * and wait for a positive acknowledge (when there are
     * enough sound buffer for this pipe)
     */
    if state != 0 {
        for _i in 0..MAX_WAIT_FOR_DSP {
            err = vx_pipe_can_start(chip, pipe);
            if err > 0 {
                break;
            }
            /* Wait for a few, before asking again
             * to avoid flooding the DSP with our requests
             */
            mdelay(1);
        }
    }

    err = vx_conf_pipe(chip, pipe);
    if err < 0 {
        return err;
    }

    err = vx_send_irqa(chip);
    if err < 0 {
        return err;
    }

    /* If it completes successfully, wait for the pipes
     * reaching the expected state before returning
     * Check one pipe only (since they are synchronous)
     */
    for _i in 0..MAX_WAIT_FOR_DSP {
        err = vx_get_pipe_state(chip, pipe, &mut cur_state);
        if err < 0 || cur_state == state {
            break;
        }
        err = -EIO;
        mdelay(1);
    }
    if err < 0 { -EIO } else { 0 }
}

/*
 * vx_stop_pipe - stop a pipe
 * @pipe: the pipe to be stopped
 *
 * called from trigger callback only
 */
unsafe fn vx_stop_pipe(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();
    vx_init_rmh(&mut rmh, CMD_STOP_PIPE);
    vx_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number, 0);
    vx_send_msg(chip, &mut rmh)
}

/*
 * vx_alloc_pipe - allocate a pipe and initialize the pipe instance
 * @capture: 0 = playback, 1 = capture operation
 * @audioid: the audio id to be assigned
 * @num_audio: number of audio channels
 * @pipep: the returned pipe instance
 *
 * return 0 on success, or a negative error code.
 */
unsafe fn vx_alloc_pipe(
    chip: *mut vx_core,
    capture: i32,
    audioid: i32,
    num_audio: i32,
    pipep: *mut *mut vx_pipe,
) -> i32 {
    let mut err: i32;
    let mut pipe: *mut vx_pipe;
    let mut rmh: vx_rmh = core::mem::zeroed();
    let data_mode: i32;

    *pipep = core::ptr::null_mut();
    vx_init_rmh(&mut rmh, CMD_RES_PIPE);
    vx_set_pipe_cmd_params(&mut rmh, capture, audioid, num_audio);
    /* #if 0 - NYI
     * if (underrun_skip_sound)
     *     rmh.Cmd[0] |= BIT_SKIP_SOUND;
     */
    data_mode = if ((*chip).uer_bits & IEC958_AES0_NONAUDIO) != 0 { 1 } else { 0 };
    if capture == 0 && data_mode != 0 {
        rmh.Cmd[0] |= BIT_DATA_MODE;
    }
    err = vx_send_msg(chip, &mut rmh);
    if err < 0 {
        return err;
    }

    /* initialize the pipe record */
    pipe = kzalloc_obj::<vx_pipe>();
    if pipe.is_null() {
        /* release the pipe */
        vx_init_rmh(&mut rmh, CMD_FREE_PIPE);
        vx_set_pipe_cmd_params(&mut rmh, capture, audioid, 0);
        vx_send_msg(chip, &mut rmh);
        return -ENOMEM;
    }

    /* the pipe index should be identical with the audio index */
    (*pipe).number = audioid;
    (*pipe).is_capture = capture;
    (*pipe).channels = num_audio;
    (*pipe).differed_type = 0;
    (*pipe).pcx_time = 0;
    (*pipe).data_mode = data_mode;
    *pipep = pipe;

    0
}

/*
 * vx_free_pipe - release a pipe
 * @pipe: pipe to be released
 */
unsafe fn vx_free_pipe(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_FREE_PIPE);
    vx_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number, 0);
    vx_send_msg(chip, &mut rmh);

    kfree(pipe as *mut core::ffi::c_void);
    0
}

/*
 * vx_start_stream - start the stream
 *
 * called from trigger callback only
 */
unsafe fn vx_start_stream(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_START_ONE_STREAM);
    vx_set_stream_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number);
    vx_set_differed_time(chip, &mut rmh, pipe);
    vx_send_msg(chip, &mut rmh)
}

/*
 * vx_stop_stream - stop the stream
 *
 * called from trigger callback only
 */
unsafe fn vx_stop_stream(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();

    vx_init_rmh(&mut rmh, CMD_STOP_STREAM);
    vx_set_stream_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number);
    vx_send_msg(chip, &mut rmh)
}

/*
 * playback hw information
 */

static vx_pcm_playback_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: (SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_MMAP_VALID /*|*/
        /*SNDRV_PCM_INFO_RESUME*/),
    formats: (/*SNDRV_PCM_FMTBIT_U8 |*/ SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE),
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 126,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: VX_MAX_PERIODS,
    fifo_size: 126,
};

/*
 * vx_pcm_playback_open - open callback for playback
 */
unsafe fn vx_pcm_playback_open(subs: *mut snd_pcm_substream) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let chip: *mut vx_core = snd_pcm_substream_chip(subs);
    let mut pipe: *mut vx_pipe = core::ptr::null_mut();
    let audio: u32;
    let mut err: i32;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    audio = (*(*subs).pcm).device * 2;
    if snd_BUG_ON(audio >= (*chip).audio_outs) != 0 {
        return -EINVAL;
    }

    /* playback pipe may have been already allocated for monitoring */
    pipe = *(*chip).playback_pipes.add(audio as usize);
    if pipe.is_null() {
        /* not allocated yet */
        err = vx_alloc_pipe(chip, 0, audio as i32, 2, &mut pipe); /* stereo playback */
        if err < 0 {
            return err;
        }
    }
    /* open for playback */
    (*pipe).references += 1;

    (*pipe).substream = subs;
    *(*chip).playback_pipes.add(audio as usize) = pipe;

    (*runtime).hw = vx_pcm_playback_hw;
    (*runtime).hw.period_bytes_min = (*chip).ibl.size;
    (*runtime).private_data = pipe as *mut core::ffi::c_void;

    /* align to 4 bytes (otherwise will be problematic when 24bit is used) */
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 4);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 4);

    0
}

/*
 * vx_pcm_playback_close - close callback for playback
 */
unsafe fn vx_pcm_playback_close(subs: *mut snd_pcm_substream) -> i32 {
    let chip: *mut vx_core = snd_pcm_substream_chip(subs);
    let pipe: *mut vx_pipe;

    if (*(*subs).runtime).private_data.is_null() {
        return -EINVAL;
    }

    pipe = (*(*subs).runtime).private_data as *mut vx_pipe;

    (*pipe).references -= 1;
    if (*pipe).references == 0 {
        *(*chip).playback_pipes.add((*pipe).number as usize) = core::ptr::null_mut();
        vx_free_pipe(chip, pipe);
    }

    0
}

/*
 * vx_notify_end_of_buffer - send "end-of-buffer" notifier at the given pipe
 * @pipe: the pipe to notify
 *
 * NB: call with a certain lock.
 */
unsafe fn vx_notify_end_of_buffer(chip: *mut vx_core, pipe: *mut vx_pipe) -> i32 {
    let mut err: i32 = 0;
    let mut rmh: vx_rmh = core::mem::zeroed(); /* use a temporary rmh here */

    /* Toggle Dsp Host Interface into Message mode */
    vx_send_rih_nolock(chip, IRQ_PAUSE_START_CONNECT);
    vx_init_rmh(&mut rmh, CMD_NOTIFY_END_OF_BUFFER);
    vx_set_stream_cmd_params(&mut rmh, 0, (*pipe).number);
    err = vx_send_msg_nolock(chip, &mut rmh);
    if err < 0 {
        return err;
    }
    /* Toggle Dsp Host Interface back to sound transfer mode */
    vx_send_rih_nolock(chip, IRQ_PAUSE_START_CONNECT);
    0
}

/*
 * vx_pcm_playback_transfer_chunk - transfer a single chunk
 * @subs: substream
 * @pipe: the pipe to transfer
 * @size: chunk size in bytes
 *
 * transfer a single buffer chunk.  EOB notificaton is added after that.
 * called from the interrupt handler, too.
 *
 * return 0 if ok.
 */
unsafe fn vx_pcm_playback_transfer_chunk(
    chip: *mut vx_core,
    runtime: *mut snd_pcm_runtime,
    pipe: *mut vx_pipe,
    size: i32,
) -> i32 {
    let space: i32;
    let err: i32;

    space = vx_query_hbuffer_size(chip, pipe);
    if space < 0 {
        /* disconnect the host, SIZE_HBUF command always switches to the stream mode */
        vx_send_rih(chip, IRQ_CONNECT_STREAM_NEXT);
        dev_dbg((*(*chip).card).dev, c"error hbuffer\n".as_ptr());
        return space;
    }
    if space < size {
        vx_send_rih(chip, IRQ_CONNECT_STREAM_NEXT);
        dev_dbg(
            (*(*chip).card).dev,
            c"no enough hbuffer space %d\n".as_ptr(),
            space,
        );
        return -EIO; /* XRUN */
    }

    /* we don't need irqsave here, because this function
     * is called from either trigger callback or irq handler
     */
    scoped_guard_mutex(&mut (*chip).lock, || {
        vx_pseudo_dma_write(chip, runtime, pipe, size);
        err = vx_notify_end_of_buffer(chip, pipe);
        /* disconnect the host, SIZE_HBUF command always switches to the stream mode */
        vx_send_rih_nolock(chip, IRQ_CONNECT_STREAM_NEXT);
    });
    err
}

/*
 * update the position of the given pipe.
 * pipe->position is updated and wrapped within the buffer size.
 * pipe->transferred is updated, too, but the size is not wrapped,
 * so that the caller can check the total transferred size later
 * (to call snd_pcm_period_elapsed).
 */
unsafe fn vx_update_pipe_position(
    chip: *mut vx_core,
    runtime: *mut snd_pcm_runtime,
    pipe: *mut vx_pipe,
) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();
    let err: i32;
    let update: i32;
    let count: u64;

    vx_init_rmh(&mut rmh, CMD_STREAM_SAMPLE_COUNT);
    vx_set_pipe_cmd_params(&mut rmh, (*pipe).is_capture, (*pipe).number, 0);
    err = vx_send_msg(chip, &mut rmh);
    if err < 0 {
        return err;
    }

    count = (((rmh.Stat[0] & 0xfffff) as u64) << 24) | (rmh.Stat[1] as u64);
    update = count.wrapping_sub((*pipe).cur_count) as i32;
    (*pipe).cur_count = count;
    (*pipe).position += update;
    if (*pipe).position >= (*runtime).buffer_size as i32 {
        (*pipe).position %= (*runtime).buffer_size as i32;
    }
    (*pipe).transferred += update;
    0
}

/*
 * transfer the pending playback buffer data to DSP
 * called from interrupt handler
 */
unsafe fn vx_pcm_playback_transfer(
    chip: *mut vx_core,
    subs: *mut snd_pcm_substream,
    pipe: *mut vx_pipe,
    nchunks: i32,
) {
    let mut err: i32;
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;

    if (*pipe).prepared == 0 || ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return;
    }
    for _i in 0..nchunks {
        err = vx_pcm_playback_transfer_chunk(chip, runtime, pipe, (*chip).ibl.size as i32);
        if err < 0 {
            return;
        }
    }
}

/*
 * update the playback position and call snd_pcm_period_elapsed() if necessary
 * called from interrupt handler
 */
unsafe fn vx_pcm_playback_update(
    chip: *mut vx_core,
    subs: *mut snd_pcm_substream,
    pipe: *mut vx_pipe,
) {
    let err: i32;
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;

    if (*pipe).running != 0 && ((*chip).chip_status & VX_STAT_IS_STALE) == 0 {
        err = vx_update_pipe_position(chip, runtime, pipe);
        if err < 0 {
            return;
        }
        if (*pipe).transferred >= (*runtime).period_size as i32 {
            (*pipe).transferred %= (*runtime).period_size as i32;
            snd_pcm_period_elapsed(subs);
        }
    }
}

/*
 * vx_pcm_playback_trigger - trigger callback for playback
 */
unsafe fn vx_pcm_trigger(subs: *mut snd_pcm_substream, cmd: i32) -> i32 {
    let chip: *mut vx_core = snd_pcm_substream_chip(subs);
    let pipe: *mut vx_pipe = (*(*subs).runtime).private_data as *mut vx_pipe;
    let mut err: i32;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            if (*pipe).is_capture == 0 {
                vx_pcm_playback_transfer(chip, subs, pipe, 2);
            }
            err = vx_start_stream(chip, pipe);
            if err < 0 {
                pr_debug(c"vx: cannot start stream\n".as_ptr());
                return err;
            }
            err = vx_toggle_pipe(chip, pipe, 1);
            if err < 0 {
                pr_debug(c"vx: cannot start pipe\n".as_ptr());
                vx_stop_stream(chip, pipe);
                return err;
            }
            (*chip).pcm_running += 1;
            (*pipe).running = 1;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            vx_toggle_pipe(chip, pipe, 0);
            vx_stop_pipe(chip, pipe);
            vx_stop_stream(chip, pipe);
            (*chip).pcm_running -= 1;
            (*pipe).running = 0;
        }
        SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            err = vx_toggle_pipe(chip, pipe, 0);
            if err < 0 {
                return err;
            }
        }
        SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            err = vx_toggle_pipe(chip, pipe, 1);
            if err < 0 {
                return err;
            }
        }
        _ => return -EINVAL,
    }
    0
}

/*
 * vx_pcm_playback_pointer - pointer callback for playback
 */
unsafe fn vx_pcm_playback_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let pipe: *mut vx_pipe = (*runtime).private_data as *mut vx_pipe;
    (*pipe).position as snd_pcm_uframes_t
}

/*
 * vx_pcm_prepare - prepare callback for playback and capture
 */
unsafe fn vx_pcm_prepare(subs: *mut snd_pcm_substream) -> i32 {
    let chip: *mut vx_core = snd_pcm_substream_chip(subs);
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let pipe: *mut vx_pipe = (*runtime).private_data as *mut vx_pipe;
    let mut err: i32;
    let data_mode: i32;
    // int max_size, nchunks;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    data_mode = if ((*chip).uer_bits & IEC958_AES0_NONAUDIO) != 0 { 1 } else { 0 };
    if data_mode != (*pipe).data_mode && (*pipe).is_capture == 0 {
        /* IEC958 status (raw-mode) was changed */
        /* we reopen the pipe */
        let mut rmh: vx_rmh = core::mem::zeroed();
        dev_dbg(
            (*(*chip).card).dev,
            c"reopen the pipe with data_mode = %d\n".as_ptr(),
            data_mode,
        );
        vx_init_rmh(&mut rmh, CMD_FREE_PIPE);
        vx_set_pipe_cmd_params(&mut rmh, 0, (*pipe).number, 0);
        err = vx_send_msg(chip, &mut rmh);
        if err < 0 {
            return err;
        }
        vx_init_rmh(&mut rmh, CMD_RES_PIPE);
        vx_set_pipe_cmd_params(&mut rmh, 0, (*pipe).number, (*pipe).channels);
        if data_mode != 0 {
            rmh.Cmd[0] |= BIT_DATA_MODE;
        }
        err = vx_send_msg(chip, &mut rmh);
        if err < 0 {
            return err;
        }
        (*pipe).data_mode = data_mode;
    }

    if (*chip).pcm_running != 0 && (*chip).freq != (*runtime).rate {
        dev_err(
            (*(*chip).card).dev,
            c"vx: cannot set different clock %d from the current %d\n".as_ptr(),
            (*runtime).rate,
            (*chip).freq,
        );
        return -EINVAL;
    }
    vx_set_clock(chip, (*runtime).rate);

    err = vx_set_format(chip, pipe, runtime);
    if err < 0 {
        return err;
    }

    if vx_is_pcmcia(chip) != 0 {
        (*pipe).align = 2; /* 16bit word */
    } else {
        (*pipe).align = 4; /* 32bit word */
    }

    (*pipe).buffer_bytes = frames_to_bytes(runtime, (*runtime).buffer_size);
    (*pipe).period_bytes = frames_to_bytes(runtime, (*runtime).period_size);
    (*pipe).hw_ptr = 0;

    /* set the timestamp */
    vx_update_pipe_position(chip, runtime, pipe);
    /* clear again */
    (*pipe).transferred = 0;
    (*pipe).position = 0;

    (*pipe).prepared = 1;

    0
}

/*
 * operators for PCM playback
 */
static vx_pcm_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(vx_pcm_playback_open),
    close: Some(vx_pcm_playback_close),
    prepare: Some(vx_pcm_prepare),
    trigger: Some(vx_pcm_trigger),
    pointer: Some(vx_pcm_playback_pointer),
};

/*
 * playback hw information
 */

static vx_pcm_capture_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: (SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_MMAP_VALID /*|*/
        /*SNDRV_PCM_INFO_RESUME*/),
    formats: (/*SNDRV_PCM_FMTBIT_U8 |*/ SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE),
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 5000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 126,
    period_bytes_max: 128 * 1024,
    periods_min: 2,
    periods_max: VX_MAX_PERIODS,
    fifo_size: 126,
};

/*
 * vx_pcm_capture_open - open callback for capture
 */
unsafe fn vx_pcm_capture_open(subs: *mut snd_pcm_substream) -> i32 {
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let chip: *mut vx_core = snd_pcm_substream_chip(subs);
    let mut pipe: *mut vx_pipe = core::ptr::null_mut();
    let mut pipe_out_monitoring: *mut vx_pipe = core::ptr::null_mut();
    let audio: u32;
    let mut err: i32;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    audio = (*(*subs).pcm).device * 2;
    if snd_BUG_ON(audio >= (*chip).audio_ins) != 0 {
        return -EINVAL;
    }
    err = vx_alloc_pipe(chip, 1, audio as i32, 2, &mut pipe);
    if err < 0 {
        return err;
    }
    (*pipe).substream = subs;
    *(*chip).capture_pipes.add(audio as usize) = pipe;

    /* check if monitoring is needed */
    if *(*chip).audio_monitor_active.add(audio as usize) != 0 {
        pipe_out_monitoring = *(*chip).playback_pipes.add(audio as usize);
        if pipe_out_monitoring.is_null() {
            /* allocate a pipe */
            err = vx_alloc_pipe(chip, 0, audio as i32, 2, &mut pipe_out_monitoring);
            if err < 0 {
                return err;
            }
            *(*chip).playback_pipes.add(audio as usize) = pipe_out_monitoring;
        }
        (*pipe_out_monitoring).references += 1;
        /*
           if an output pipe is available, it's audios still may need to be
           unmuted. hence we'll have to call a mixer entry point.
        */
        vx_set_monitor_level(
            chip,
            audio,
            *(*chip).audio_monitor.add(audio as usize),
            *(*chip).audio_monitor_active.add(audio as usize),
        );
        /* assuming stereo */
        vx_set_monitor_level(
            chip,
            audio + 1,
            *(*chip).audio_monitor.add(audio as usize + 1),
            *(*chip).audio_monitor_active.add(audio as usize + 1),
        );
    }

    (*pipe).monitoring_pipe = pipe_out_monitoring; /* default value NULL */

    (*runtime).hw = vx_pcm_capture_hw;
    (*runtime).hw.period_bytes_min = (*chip).ibl.size;
    (*runtime).private_data = pipe as *mut core::ffi::c_void;

    /* align to 4 bytes (otherwise will be problematic when 24bit is used) */
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 4);
    snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 4);

    0
}

/*
 * vx_pcm_capture_close - close callback for capture
 */
unsafe fn vx_pcm_capture_close(subs: *mut snd_pcm_substream) -> i32 {
    let chip: *mut vx_core = snd_pcm_substream_chip(subs);
    let pipe: *mut vx_pipe;
    let pipe_out_monitoring: *mut vx_pipe;

    if (*(*subs).runtime).private_data.is_null() {
        return -EINVAL;
    }
    pipe = (*(*subs).runtime).private_data as *mut vx_pipe;
    *(*chip).capture_pipes.add((*pipe).number as usize) = core::ptr::null_mut();

    pipe_out_monitoring = (*pipe).monitoring_pipe;

    /*
      if an output pipe is attached to this input,
      check if it needs to be released.
    */
    if !pipe_out_monitoring.is_null() {
        (*pipe_out_monitoring).references -= 1;
        if (*pipe_out_monitoring).references == 0 {
            vx_free_pipe(chip, pipe_out_monitoring);
            *(*chip).playback_pipes.add((*pipe).number as usize) = core::ptr::null_mut();
            (*pipe).monitoring_pipe = core::ptr::null_mut();
        }
    }

    vx_free_pipe(chip, pipe);
    0
}

const DMA_READ_ALIGN: i32 = 6; /* hardware alignment for read */

/*
 * vx_pcm_capture_update - update the capture buffer
 */
unsafe fn vx_pcm_capture_update(
    chip: *mut vx_core,
    subs: *mut snd_pcm_substream,
    pipe: *mut vx_pipe,
) {
    let mut size: i32;
    let mut space: i32;
    let mut count: i32;
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;

    if (*pipe).running == 0 || ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return;
    }

    size = ((*runtime).buffer_size - snd_pcm_capture_avail(runtime)) as i32;
    if size == 0 {
        return;
    }
    size = frames_to_bytes(runtime, size as snd_pcm_uframes_t);
    space = vx_query_hbuffer_size(chip, pipe);
    if space < 0 {
        vx_pcm_capture_update_error(chip);
        return;
    }
    if size > space {
        size = space;
    }
    size = (size / 3) * 3; /* align to 3 bytes */
    if size < DMA_READ_ALIGN {
        vx_pcm_capture_update_error(chip);
        return;
    }

    /* keep the last 6 bytes, they will be read after disconnection */
    count = size - DMA_READ_ALIGN;
    /* read bytes until the current pointer reaches to the aligned position
     * for word-transfer
     */
    while count > 0 {
        if ((*pipe).hw_ptr % (*pipe).align) == 0 {
            break;
        }
        if vx_wait_for_rx_full(chip) < 0 {
            vx_pcm_capture_update_error(chip);
            return;
        }
        vx_pcm_read_per_bytes(chip, runtime, pipe);
        count -= 3;
    }
    if count > 0 {
        /* ok, let's accelerate! */
        let align: i32 = (*pipe).align * 3;
        space = (count / align) * align;
        if space > 0 {
            vx_pseudo_dma_read(chip, runtime, pipe, space);
            count -= space;
        }
    }
    /* read the rest of bytes */
    while count > 0 {
        if vx_wait_for_rx_full(chip) < 0 {
            vx_pcm_capture_update_error(chip);
            return;
        }
        vx_pcm_read_per_bytes(chip, runtime, pipe);
        count -= 3;
    }
    /* disconnect the host, SIZE_HBUF command always switches to the stream mode */
    vx_send_rih(chip, IRQ_CONNECT_STREAM_NEXT);
    /* read the last pending 6 bytes */
    count = DMA_READ_ALIGN;
    while count > 0 {
        vx_pcm_read_per_bytes(chip, runtime, pipe);
        count -= 3;
    }
    /* update the position */
    (*pipe).transferred += size;
    if (*pipe).transferred >= (*pipe).period_bytes {
        (*pipe).transferred %= (*pipe).period_bytes;
        snd_pcm_period_elapsed(subs);
    }
}

unsafe fn vx_pcm_capture_update_error(chip: *mut vx_core) {
    /* disconnect the host, SIZE_HBUF command always switches to the stream mode */
    vx_send_rih(chip, IRQ_CONNECT_STREAM_NEXT);
}

/*
 * vx_pcm_capture_pointer - pointer callback for capture
 */
unsafe fn vx_pcm_capture_pointer(subs: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let runtime: *mut snd_pcm_runtime = (*subs).runtime;
    let pipe: *mut vx_pipe = (*runtime).private_data as *mut vx_pipe;
    bytes_to_frames(runtime, (*pipe).hw_ptr)
}

/*
 * operators for PCM capture
 */
static vx_pcm_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(vx_pcm_capture_open),
    close: Some(vx_pcm_capture_close),
    prepare: Some(vx_pcm_prepare),
    trigger: Some(vx_pcm_trigger),
    pointer: Some(vx_pcm_capture_pointer),
};

/*
 * interrupt handler for pcm streams
 */
pub unsafe fn vx_pcm_update_intr(chip: *mut vx_core, events: u32) {
    let mut i: u32;
    let mut pipe: *mut vx_pipe;

    const EVENT_MASK: u32 = END_OF_BUFFER_EVENTS_PENDING | ASYNC_EVENTS_PENDING;

    if (events & EVENT_MASK) != 0 {
        vx_init_rmh(&mut (*chip).irq_rmh, CMD_ASYNC);
        if (events & ASYNC_EVENTS_PENDING) != 0 {
            (*chip).irq_rmh.Cmd[0] |= 0x00000001; /* SEL_ASYNC_EVENTS */
        }
        if (events & END_OF_BUFFER_EVENTS_PENDING) != 0 {
            (*chip).irq_rmh.Cmd[0] |= 0x00000002; /* SEL_END_OF_BUF_EVENTS */
        }

        if vx_send_msg(chip, &mut (*chip).irq_rmh) < 0 {
            dev_dbg((*(*chip).card).dev, c"msg send error!!\n".as_ptr());
            return;
        }

        i = 1;
        while i < (*chip).irq_rmh.LgStat {
            let p: i32;
            let mut buf: i32;
            let capture: i32;
            let eob: i32;
            p = ((*chip).irq_rmh.Stat[i as usize] & MASK_FIRST_FIELD) as i32;
            capture = if ((*chip).irq_rmh.Stat[i as usize] & 0x400000) != 0 { 1 } else { 0 };
            eob = if ((*chip).irq_rmh.Stat[i as usize] & 0x800000) != 0 { 1 } else { 0 };
            i += 1;
            if (events & ASYNC_EVENTS_PENDING) != 0 {
                i += 1;
            }
            buf = 1; /* force to transfer */
            if (events & END_OF_BUFFER_EVENTS_PENDING) != 0 {
                if eob != 0 {
                    buf = (*chip).irq_rmh.Stat[i as usize] as i32;
                }
                i += 1;
            }
            if capture != 0 {
                continue;
            }
            if snd_BUG_ON(p < 0 || p as u32 >= (*chip).audio_outs) != 0 {
                continue;
            }
            pipe = *(*chip).playback_pipes.add(p as usize);
            if !pipe.is_null() && !(*pipe).substream.is_null() {
                vx_pcm_playback_update(chip, (*pipe).substream, pipe);
                vx_pcm_playback_transfer(chip, (*pipe).substream, pipe, buf);
            }
        }
    }

    /* update the capture pcm pointers as frequently as possible */
    i = 0;
    while i < (*chip).audio_ins {
        pipe = *(*chip).capture_pipes.add(i as usize);
        if !pipe.is_null() && !(*pipe).substream.is_null() {
            vx_pcm_capture_update(chip, (*pipe).substream, pipe);
        }
        i += 1;
    }
}

/*
 * vx_init_audio_io - check the available audio i/o and allocate pipe arrays
 */
unsafe fn vx_init_audio_io(chip: *mut vx_core) -> i32 {
    let mut rmh: vx_rmh = core::mem::zeroed();
    let preferred: i32;

    vx_init_rmh(&mut rmh, CMD_SUPPORTED);
    if vx_send_msg(chip, &mut rmh) < 0 {
        dev_err(
            (*(*chip).card).dev,
            c"vx: cannot get the supported audio data\n".as_ptr(),
        );
        return -ENXIO;
    }

    (*chip).audio_outs = rmh.Stat[0] & MASK_FIRST_FIELD;
    (*chip).audio_ins = (rmh.Stat[0] >> (FIELD_SIZE * 2)) & MASK_FIRST_FIELD;
    (*chip).audio_info = rmh.Stat[1];

    /* allocate pipes */
    (*chip).playback_pipes = kzalloc_objs::<*mut vx_pipe>((*chip).audio_outs);
    if (*chip).playback_pipes.is_null() {
        return -ENOMEM;
    }
    (*chip).capture_pipes = kzalloc_objs::<*mut vx_pipe>((*chip).audio_ins);
    if (*chip).capture_pipes.is_null() {
        kfree((*chip).playback_pipes as *mut core::ffi::c_void);
        return -ENOMEM;
    }

    preferred = (*chip).ibl.size as i32;
    (*chip).ibl.size = 0;
    vx_set_ibl(chip, &mut (*chip).ibl); /* query the info */
    if preferred > 0 {
        (*chip).ibl.size = roundup(preferred as u32, (*chip).ibl.granularity);
        if (*chip).ibl.size > (*chip).ibl.max_size {
            (*chip).ibl.size = (*chip).ibl.max_size;
        }
    } else {
        (*chip).ibl.size = (*chip).ibl.min_size; /* set to the minimum */
    }
    vx_set_ibl(chip, &mut (*chip).ibl);

    0
}

/*
 * free callback for pcm
 */
unsafe fn snd_vx_pcm_free(pcm: *mut snd_pcm) {
    let chip: *mut vx_core = (*pcm).private_data as *mut vx_core;
    (*chip).pcm[(*pcm).device as usize] = core::ptr::null_mut();
    kfree((*chip).playback_pipes as *mut core::ffi::c_void);
    (*chip).playback_pipes = core::ptr::null_mut();
    kfree((*chip).capture_pipes as *mut core::ffi::c_void);
    (*chip).capture_pipes = core::ptr::null_mut();
}

/*
 * snd_vx_pcm_new - create and initialize a pcm
 */
pub unsafe fn snd_vx_pcm_new(chip: *mut vx_core) -> i32 {
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();
    let mut i: u32;
    let mut err: i32;

    err = vx_init_audio_io(chip);
    if err < 0 {
        return err;
    }

    i = 0;
    while i < (*(*chip).hw).num_codecs {
        let outs: u32;
        let ins: u32;
        outs = if (*chip).audio_outs > i * 2 { 1 } else { 0 };
        ins = if (*chip).audio_ins > i * 2 { 1 } else { 0 };
        if outs == 0 && ins == 0 {
            break;
        }
        err = snd_pcm_new((*chip).card, c"VX PCM".as_ptr(), i, outs, ins, &mut pcm);
        if err < 0 {
            return err;
        }
        if outs != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &vx_pcm_playback_ops);
        }
        if ins != 0 {
            snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &vx_pcm_capture_ops);
        }
        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_VMALLOC,
            core::ptr::null_mut(),
            0,
            0,
        );

        (*pcm).private_data = chip as *mut core::ffi::c_void;
        (*pcm).private_free = Some(snd_vx_pcm_free);
        (*pcm).info_flags = 0;
        (*pcm).nonatomic = true;
        strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
        (*chip).pcm[i as usize] = pcm;
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
