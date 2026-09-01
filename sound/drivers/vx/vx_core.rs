// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX soundcards
 *
 * Hardware core part
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

// C dependencies translated as external Rust dependencies:
// linux/delay.h, linux/slab.h, linux/interrupt.h, linux/init.h,
// linux/device.h, linux/firmware.h, linux/module.h, linux/io.h,
// sound/core.h, sound/pcm.h, sound/asoundef.h, sound/info.h,
// sound/vx_core.h, and "vx_cmd.h".

pub const MASK_MORE_THAN_1_WORD_COMMAND: u32 = 0x0000_8000;
pub const MASK_1_WORD_COMMAND: u32 = 0x00ff_7fff;
pub const END_OF_RESET_WAIT_TIME: u32 = 500; /* us */

/*
 * vx_check_reg_bit - wait for the specified bit is set/reset on a register
 * @reg: register to check
 * @mask: bit mask
 * @bit: resultant bit to be checked
 * @time: time-out of loop in msec
 *
 * returns zero if a bit matches, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_check_reg_bit(
    chip: *mut vx_core,
    reg: core::ffi::c_int,
    mask: core::ffi::c_int,
    bit: core::ffi::c_int,
    time: core::ffi::c_int,
) -> core::ffi::c_int {
    let end_time: core::ffi::c_ulong =
        jiffies.wrapping_add(((time * HZ + 999) / 1000) as core::ffi::c_ulong);
    static REG_NAMES: [&[u8]; VX_REG_MAX as usize] = [
        b"ICR\0", b"CVR\0", b"ISR\0", b"IVR\0", b"RXH\0", b"RXM\0", b"RXL\0",
        b"DMA\0", b"CDSP\0", b"RFREQ\0", b"RUER/V2\0", b"DATA\0", b"MEMIRQ\0",
        b"ACQ\0", b"BIT0\0", b"BIT1\0", b"MIC0\0", b"MIC1\0", b"MIC2\0",
        b"MIC3\0", b"INTCSR\0", b"CNTRL\0", b"GPIOC\0",
        b"LOFREQ\0", b"HIFREQ\0", b"CSUER\0", b"RUER\0",
    ];

    loop {
        if (snd_vx_inb(chip, reg) & mask) == bit {
            return 0;
        }
        // msleep(10);
        if !time_after_eq(end_time, jiffies) {
            break;
        }
    }
    dev_dbg(
        (*(*chip).card).dev,
        b"vx_check_reg_bit: timeout, reg=%s, mask=0x%x, val=0x%x\n\0".as_ptr() as *const _,
        REG_NAMES[reg as usize].as_ptr(),
        mask,
        snd_vx_inb(chip, reg),
    );
    -EIO
}

/*
 * vx_send_irq_dsp - set command irq bit
 * @num: the requested IRQ type, IRQ_XXX
 *
 * this triggers the specified IRQ request
 * returns 0 if successful, or a negative error code.
 *
 */
unsafe extern "C" fn vx_send_irq_dsp(
    chip: *mut vx_core,
    num: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut nirq: core::ffi::c_int;

    /* wait for Hc = 0 */
    if snd_vx_check_reg_bit(chip, VX_CVR, CVR_HC, 0, 200) < 0 {
        return -EIO;
    }

    nirq = num;
    if vx_has_new_dsp(chip) != 0 {
        nirq += VXP_IRQ_OFFSET;
    }
    vx_outb(chip, CVR, (nirq >> 1) | CVR_HC);
    0
}

/*
 * vx_reset_chk - reset CHK bit on ISR
 *
 * returns 0 if successful, or a negative error code.
 */
unsafe extern "C" fn vx_reset_chk(chip: *mut vx_core) -> core::ffi::c_int {
    /* Reset irq CHK */
    if vx_send_irq_dsp(chip, IRQ_RESET_CHK) < 0 {
        return -EIO;
    }
    /* Wait until CHK = 0 */
    if vx_check_isr(chip, ISR_CHK, 0, 200) < 0 {
        return -EIO;
    }
    0
}

/*
 * vx_transfer_end - terminate message transfer
 * @cmd: IRQ message to send (IRQ_MESS_XXX_END)
 *
 * returns 0 if successful, or a negative error code.
 * the error code can be VX-specific, retrieved via vx_get_error().
 * NB: call with mutex held!
 */
unsafe extern "C" fn vx_transfer_end(
    chip: *mut vx_core,
    cmd: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    err = vx_reset_chk(chip);
    if err < 0 {
        return err;
    }

    /* irq MESS_READ/WRITE_END */
    err = vx_send_irq_dsp(chip, cmd);
    if err < 0 {
        return err;
    }

    /* Wait CHK = 1 */
    err = vx_wait_isr_bit(chip, ISR_CHK);
    if err < 0 {
        return err;
    }

    /* If error, Read RX */
    err = vx_inb(chip, ISR);
    if (err & ISR_ERR) != 0 {
        err = vx_wait_for_rx_full(chip);
        if err < 0 {
            dev_dbg(
                (*(*chip).card).dev,
                b"transfer_end: error in rx_full\n\0".as_ptr() as *const _,
            );
            return err;
        }
        err = vx_inb(chip, RXH) << 16;
        err |= vx_inb(chip, RXM) << 8;
        err |= vx_inb(chip, RXL);
        dev_dbg(
            (*(*chip).card).dev,
            b"transfer_end: error = 0x%x\n\0".as_ptr() as *const _,
            err,
        );
        return -(VX_ERR_MASK | err);
    }
    0
}

/*
 * vx_read_status - return the status rmh
 * @rmh: rmh record to store the status
 *
 * returns 0 if successful, or a negative error code.
 * the error code can be VX-specific, retrieved via vx_get_error().
 * NB: call with mutex held!
 */
unsafe extern "C" fn vx_read_status(
    chip: *mut vx_core,
    rmh: *mut vx_rmh,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;
    let mut err: core::ffi::c_int;
    let mut val: core::ffi::c_int;
    let mut size: core::ffi::c_int;

    /* no read necessary? */
    if (*rmh).DspStat == RMH_SSIZE_FIXED && (*rmh).LgStat == 0 {
        return 0;
    }

    /*
     * Wait for RX full (with timeout protection)
     * The first word of status is in RX
     */
    err = vx_wait_for_rx_full(chip);
    if err < 0 {
        return err;
    }

    /* Read RX */
    val = vx_inb(chip, RXH) << 16;
    val |= vx_inb(chip, RXM) << 8;
    val |= vx_inb(chip, RXL);

    /* If status given by DSP, let's decode its size */
    match (*rmh).DspStat {
        RMH_SSIZE_ARG => {
            size = val & 0xff;
            (*rmh).Stat[0] = val & 0xffff00;
            (*rmh).LgStat = size + 1;
        }
        RMH_SSIZE_MASK => {
            /* Let's count the arg numbers from a mask */
            (*rmh).Stat[0] = val;
            size = 0;
            while val != 0 {
                if (val & 0x01) != 0 {
                    size += 1;
                }
                val >>= 1;
            }
            (*rmh).LgStat = size + 1;
        }
        _ => {
            /* else retrieve the status length given by the driver */
            size = (*rmh).LgStat;
            (*rmh).Stat[0] = val; /* Val is the status 1st word */
            size -= 1; /* hence adjust remaining length */
        }
    }

    if size < 1 {
        return 0;
    }
    if snd_BUG_ON((size >= SIZE_MAX_STATUS) as core::ffi::c_int) != 0 {
        return -EINVAL;
    }

    i = 1;
    while i <= size {
        /* trigger an irq MESS_WRITE_NEXT */
        err = vx_send_irq_dsp(chip, IRQ_MESS_WRITE_NEXT);
        if err < 0 {
            return err;
        }
        /* Wait for RX full (with timeout protection) */
        err = vx_wait_for_rx_full(chip);
        if err < 0 {
            return err;
        }
        (*rmh).Stat[i as usize] = vx_inb(chip, RXH) << 16;
        (*rmh).Stat[i as usize] |= vx_inb(chip, RXM) << 8;
        (*rmh).Stat[i as usize] |= vx_inb(chip, RXL);
        i += 1;
    }

    vx_transfer_end(chip, IRQ_MESS_WRITE_END)
}

/*
 * vx_send_msg_nolock - send a DSP message and read back the status
 * @rmh: the rmh record to send and receive
 *
 * returns 0 if successful, or a negative error code.
 * the error code can be VX-specific, retrieved via vx_get_error().
 *
 * this function doesn't call mutex lock at all.
 */
#[no_mangle]
pub unsafe extern "C" fn vx_send_msg_nolock(
    chip: *mut vx_core,
    rmh: *mut vx_rmh,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;
    let mut err: core::ffi::c_int;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    err = vx_reset_chk(chip);
    if err < 0 {
        dev_dbg((*(*chip).card).dev, b"vx_send_msg: vx_reset_chk error\n\0".as_ptr() as *const _);
        return err;
    }

    /* Check bit M is set according to length of the command */
    if (*rmh).LgCmd > 1 {
        (*rmh).Cmd[0] |= MASK_MORE_THAN_1_WORD_COMMAND as core::ffi::c_int;
    } else {
        (*rmh).Cmd[0] &= MASK_1_WORD_COMMAND as core::ffi::c_int;
    }

    /* Wait for TX empty */
    err = vx_wait_isr_bit(chip, ISR_TX_EMPTY);
    if err < 0 {
        dev_dbg((*(*chip).card).dev, b"vx_send_msg: wait tx empty error\n\0".as_ptr() as *const _);
        return err;
    }

    /* Write Cmd[0] */
    vx_outb(chip, TXH, ((*rmh).Cmd[0] >> 16) & 0xff);
    vx_outb(chip, TXM, ((*rmh).Cmd[0] >> 8) & 0xff);
    vx_outb(chip, TXL, (*rmh).Cmd[0] & 0xff);

    /* Trigger irq MESSAGE */
    err = vx_send_irq_dsp(chip, IRQ_MESSAGE);
    if err < 0 {
        dev_dbg((*(*chip).card).dev, b"vx_send_msg: send IRQ_MESSAGE error\n\0".as_ptr() as *const _);
        return err;
    }

    /* Wait for CHK = 1 */
    err = vx_wait_isr_bit(chip, ISR_CHK);
    if err < 0 {
        return err;
    }

    /* If error, get error value from RX */
    if (vx_inb(chip, ISR) & ISR_ERR) != 0 {
        err = vx_wait_for_rx_full(chip);
        if err < 0 {
            dev_dbg((*(*chip).card).dev, b"vx_send_msg: rx_full read error\n\0".as_ptr() as *const _);
            return err;
        }
        err = vx_inb(chip, RXH) << 16;
        err |= vx_inb(chip, RXM) << 8;
        err |= vx_inb(chip, RXL);
        dev_dbg((*(*chip).card).dev, b"msg got error = 0x%x at cmd[0]\n\0".as_ptr() as *const _, err);
        err = -(VX_ERR_MASK | err);
        return err;
    }

    /* Send the other words */
    if (*rmh).LgCmd > 1 {
        i = 1;
        while i < (*rmh).LgCmd {
            /* Wait for TX ready */
            err = vx_wait_isr_bit(chip, ISR_TX_READY);
            if err < 0 {
                dev_dbg((*(*chip).card).dev, b"vx_send_msg: tx_ready error\n\0".as_ptr() as *const _);
                return err;
            }

            /* Write Cmd[i] */
            vx_outb(chip, TXH, ((*rmh).Cmd[i as usize] >> 16) & 0xff);
            vx_outb(chip, TXM, ((*rmh).Cmd[i as usize] >> 8) & 0xff);
            vx_outb(chip, TXL, (*rmh).Cmd[i as usize] & 0xff);

            /* Trigger irq MESS_READ_NEXT */
            err = vx_send_irq_dsp(chip, IRQ_MESS_READ_NEXT);
            if err < 0 {
                dev_dbg((*(*chip).card).dev, b"vx_send_msg: IRQ_READ_NEXT error\n\0".as_ptr() as *const _);
                return err;
            }
            i += 1;
        }
        /* Wait for TX empty */
        err = vx_wait_isr_bit(chip, ISR_TX_READY);
        if err < 0 {
            dev_dbg((*(*chip).card).dev, b"vx_send_msg: TX_READY error\n\0".as_ptr() as *const _);
            return err;
        }
        /* End of transfer */
        err = vx_transfer_end(chip, IRQ_MESS_READ_END);
        if err < 0 {
            return err;
        }
    }

    vx_read_status(chip, rmh)
}

/*
 * vx_send_msg - send a DSP message with mutex
 * @rmh: the rmh record to send and receive
 *
 * returns 0 if successful, or a negative error code.
 * see vx_send_msg_nolock().
 */
#[no_mangle]
pub unsafe extern "C" fn vx_send_msg(
    chip: *mut vx_core,
    rmh: *mut vx_rmh,
) -> core::ffi::c_int {
    let _guard = guard_mutex(&mut (*chip).lock);
    vx_send_msg_nolock(chip, rmh)
}

/*
 * vx_send_rih_nolock - send an RIH to xilinx
 * @cmd: the command to send
 *
 * returns 0 if successful, or a negative error code.
 * the error code can be VX-specific, retrieved via vx_get_error().
 *
 * this function doesn't call mutex at all.
 *
 * unlike RMH, no command is sent to DSP.
 */
#[no_mangle]
pub unsafe extern "C" fn vx_send_rih_nolock(
    chip: *mut vx_core,
    cmd: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return -EBUSY;
    }

    err = vx_reset_chk(chip);
    if err < 0 {
        return err;
    }
    /* send the IRQ */
    err = vx_send_irq_dsp(chip, cmd);
    if err < 0 {
        return err;
    }
    /* Wait CHK = 1 */
    err = vx_wait_isr_bit(chip, ISR_CHK);
    if err < 0 {
        return err;
    }
    /* If error, read RX */
    if (vx_inb(chip, ISR) & ISR_ERR) != 0 {
        err = vx_wait_for_rx_full(chip);
        if err < 0 {
            return err;
        }
        err = vx_inb(chip, RXH) << 16;
        err |= vx_inb(chip, RXM) << 8;
        err |= vx_inb(chip, RXL);
        return -(VX_ERR_MASK | err);
    }
    0
}

/*
 * vx_send_rih - send an RIH with mutex
 * @cmd: the command to send
 *
 * see vx_send_rih_nolock().
 */
#[no_mangle]
pub unsafe extern "C" fn vx_send_rih(
    chip: *mut vx_core,
    cmd: core::ffi::c_int,
) -> core::ffi::c_int {
    let _guard = guard_mutex(&mut (*chip).lock);
    vx_send_rih_nolock(chip, cmd)
}

/**
 * snd_vx_load_boot_image - boot up the xilinx interface
 * @chip: VX core instance
 * @boot: the boot record to load
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_load_boot_image(
    chip: *mut vx_core,
    boot: *const firmware,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_uint;
    let no_fillup: core::ffi::c_int = vx_has_new_dsp(chip);

    /* check the length of boot image */
    if (*boot).size <= 0 {
        return -EINVAL;
    }
    if ((*boot).size % 3) != 0 {
        return -EINVAL;
    }
    /* Disabled C preprocessor block (#if 0): stricter boot image length check. */

    /* reset dsp */
    vx_reset_dsp(chip);

    udelay(END_OF_RESET_WAIT_TIME); /* another wait? */

    /* download boot strap */
    i = 0;
    while i < 0x600 {
        if (i as usize) >= (*boot).size {
            if no_fillup != 0 {
                break;
            }
            if vx_wait_isr_bit(chip, ISR_TX_EMPTY) < 0 {
                dev_err((*(*chip).card).dev, b"dsp boot failed at %d\n\0".as_ptr() as *const _, i);
                return -EIO;
            }
            vx_outb(chip, TXH, 0);
            vx_outb(chip, TXM, 0);
            vx_outb(chip, TXL, 0);
        } else {
            let image: *const core::ffi::c_uchar = (*boot).data.add(i as usize);
            if vx_wait_isr_bit(chip, ISR_TX_EMPTY) < 0 {
                dev_err((*(*chip).card).dev, b"dsp boot failed at %d\n\0".as_ptr() as *const _, i);
                return -EIO;
            }
            vx_outb(chip, TXH, *image.add(0) as core::ffi::c_int);
            vx_outb(chip, TXM, *image.add(1) as core::ffi::c_int);
            vx_outb(chip, TXL, *image.add(2) as core::ffi::c_int);
        }
        i += 3;
    }
    0
}

/*
 * vx_test_irq_src - query the source of interrupts
 *
 * called from irq handler only
 */
unsafe extern "C" fn vx_test_irq_src(
    chip: *mut vx_core,
    ret: *mut core::ffi::c_uint,
) -> core::ffi::c_int {
    let err: core::ffi::c_int;

    vx_init_rmh(&mut (*chip).irq_rmh, CMD_TEST_IT);
    let _guard = guard_mutex(&mut (*chip).lock);
    err = vx_send_msg_nolock(chip, &mut (*chip).irq_rmh);
    if err < 0 {
        *ret = 0;
    } else {
        *ret = (*chip).irq_rmh.Stat[0] as core::ffi::c_uint;
    }
    err
}

/*
 * snd_vx_threaded_irq_handler - threaded irq handler
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_threaded_irq_handler(
    _irq: core::ffi::c_int,
    dev: *mut core::ffi::c_void,
) -> irqreturn_t {
    let chip: *mut vx_core = dev as *mut vx_core;
    let mut events: core::ffi::c_uint = 0;

    if ((*chip).chip_status & VX_STAT_IS_STALE) != 0 {
        return IRQ_HANDLED;
    }

    if vx_test_irq_src(chip, &mut events) < 0 {
        return IRQ_HANDLED;
    }

    /*
     * We must prevent any application using this DSP
     * and block any further request until the application
     * either unregisters or reloads the DSP
     */
    if (events & FATAL_DSP_ERROR) != 0 {
        dev_err((*(*chip).card).dev, b"vx_core: fatal DSP error!!\n\0".as_ptr() as *const _);
        return IRQ_HANDLED;
    }

    /*
     * The start on time code conditions are filled (ie the time code
     * received by the board is equal to one of those given to it).
     */
    if (events & TIME_CODE_EVENT_PENDING) != 0 {
        /* so far, nothing to do yet */
    }

    /* The frequency has changed on the board (UER mode). */
    if (events & FREQUENCY_CHANGE_EVENT_PENDING) != 0 {
        vx_change_frequency(chip);
    }

    /* update the pcm streams */
    vx_pcm_update_intr(chip, events);
    IRQ_HANDLED
}

/**
 * snd_vx_irq_handler - interrupt handler
 * @irq: irq number
 * @dev: VX core instance
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_irq_handler(
    _irq: core::ffi::c_int,
    dev: *mut core::ffi::c_void,
) -> irqreturn_t {
    let chip: *mut vx_core = dev as *mut vx_core;

    if ((*chip).chip_status & VX_STAT_CHIP_INIT) == 0
        || ((*chip).chip_status & VX_STAT_IS_STALE) != 0
    {
        return IRQ_NONE;
    }
    if vx_test_and_ack(chip) == 0 {
        return IRQ_WAKE_THREAD;
    }
    IRQ_NONE
}

/*
 */
unsafe extern "C" fn vx_reset_board(chip: *mut vx_core, cold_reset: core::ffi::c_int) {
    if snd_BUG_ON((*chip).ops.is_null() || (*(*chip).ops).reset_board.is_none()) != 0 {
        return;
    }

    /* current source, later sync'ed with target */
    (*chip).audio_source = VX_AUDIO_SRC_LINE;
    if cold_reset != 0 {
        (*chip).audio_source_target = (*chip).audio_source;
        (*chip).clock_source = INTERNAL_QUARTZ;
        (*chip).clock_mode = VX_CLOCK_MODE_AUTO;
        (*chip).freq = 48000;
        (*chip).uer_detected = VX_UER_MODE_NOT_PRESENT;
        (*chip).uer_bits = SNDRV_PCM_DEFAULT_CON_SPDIF;
    }

    ((*(*chip).ops).reset_board.unwrap())(chip, cold_reset);

    vx_reset_codec(chip, cold_reset);

    vx_set_internal_clock(chip, (*chip).freq);

    /* Reset the DSP */
    vx_reset_dsp(chip);

    if vx_is_pcmcia(chip) != 0 {
        /* Acknowledge any pending IRQ and reset the MEMIRQ flag. */
        vx_test_and_ack(chip);
        vx_validate_irq(chip, 1);
    }

    /* init CBits */
    vx_set_iec958_status(chip, (*chip).uer_bits);
}

/*
 * proc interface
 */

unsafe extern "C" fn vx_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let chip: *mut vx_core = (*entry).private_data as *mut vx_core;
    static AUDIO_SRC_VXP: [&[u8]; 3] = [b"Line\0", b"Mic\0", b"Digital\0"];
    static AUDIO_SRC_VX2: [&[u8]; 3] = [b"Analog\0", b"Analog\0", b"Digital\0"];
    static CLOCK_MODE: [&[u8]; 3] = [b"Auto\0", b"Internal\0", b"External\0"];
    static CLOCK_SRC: [&[u8]; 2] = [b"Internal\0", b"External\0"];
    static UER_TYPE: [&[u8]; 3] = [b"Consumer\0", b"Professional\0", b"Not Present\0"];

    snd_iprintf(buffer, b"%s\n\0".as_ptr() as *const _, (*(*chip).card).longname.as_ptr());
    snd_iprintf(
        buffer,
        b"Xilinx Firmware: %s\n\0".as_ptr() as *const _,
        if ((*chip).chip_status & VX_STAT_XILINX_LOADED) != 0 { b"Loaded\0".as_ptr() } else { b"No\0".as_ptr() },
    );
    snd_iprintf(
        buffer,
        b"Device Initialized: %s\n\0".as_ptr() as *const _,
        if ((*chip).chip_status & VX_STAT_DEVICE_INIT) != 0 { b"Yes\0".as_ptr() } else { b"No\0".as_ptr() },
    );
    snd_iprintf(buffer, b"DSP audio info:\0".as_ptr() as *const _);
    if ((*chip).audio_info & VX_AUDIO_INFO_REAL_TIME) != 0 {
        snd_iprintf(buffer, b" realtime\0".as_ptr() as *const _);
    }
    if ((*chip).audio_info & VX_AUDIO_INFO_OFFLINE) != 0 {
        snd_iprintf(buffer, b" offline\0".as_ptr() as *const _);
    }
    if ((*chip).audio_info & VX_AUDIO_INFO_MPEG1) != 0 {
        snd_iprintf(buffer, b" mpeg1\0".as_ptr() as *const _);
    }
    if ((*chip).audio_info & VX_AUDIO_INFO_MPEG2) != 0 {
        snd_iprintf(buffer, b" mpeg2\0".as_ptr() as *const _);
    }
    if ((*chip).audio_info & VX_AUDIO_INFO_LINEAR_8) != 0 {
        snd_iprintf(buffer, b" linear8\0".as_ptr() as *const _);
    }
    if ((*chip).audio_info & VX_AUDIO_INFO_LINEAR_16) != 0 {
        snd_iprintf(buffer, b" linear16\0".as_ptr() as *const _);
    }
    if ((*chip).audio_info & VX_AUDIO_INFO_LINEAR_24) != 0 {
        snd_iprintf(buffer, b" linear24\0".as_ptr() as *const _);
    }
    snd_iprintf(buffer, b"\n\0".as_ptr() as *const _);
    snd_iprintf(
        buffer,
        b"Input Source: %s\n\0".as_ptr() as *const _,
        if vx_is_pcmcia(chip) != 0 {
            AUDIO_SRC_VXP[(*chip).audio_source as usize].as_ptr()
        } else {
            AUDIO_SRC_VX2[(*chip).audio_source as usize].as_ptr()
        },
    );
    snd_iprintf(buffer, b"Clock Mode: %s\n\0".as_ptr() as *const _, CLOCK_MODE[(*chip).clock_mode as usize].as_ptr());
    snd_iprintf(buffer, b"Clock Source: %s\n\0".as_ptr() as *const _, CLOCK_SRC[(*chip).clock_source as usize].as_ptr());
    snd_iprintf(buffer, b"Frequency: %d\n\0".as_ptr() as *const _, (*chip).freq);
    snd_iprintf(buffer, b"Detected Frequency: %d\n\0".as_ptr() as *const _, (*chip).freq_detected);
    snd_iprintf(buffer, b"Detected UER type: %s\n\0".as_ptr() as *const _, UER_TYPE[(*chip).uer_detected as usize].as_ptr());
    snd_iprintf(
        buffer,
        b"Min/Max/Cur IBL: %d/%d/%d (granularity=%d)\n\0".as_ptr() as *const _,
        (*chip).ibl.min_size,
        (*chip).ibl.max_size,
        (*chip).ibl.size,
        (*chip).ibl.granularity,
    );
}

unsafe extern "C" fn vx_proc_init(chip: *mut vx_core) {
    snd_card_ro_proc_new((*chip).card, b"vx-status\0".as_ptr() as *const _, chip as *mut _, Some(vx_proc_read));
}

/**
 * snd_vx_dsp_boot - load the DSP boot
 * @chip: VX core instance
 * @boot: firmware data
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_dsp_boot(
    chip: *mut vx_core,
    boot: *const firmware,
) -> core::ffi::c_int {
    let err: core::ffi::c_int;
    let cold_reset: core::ffi::c_int = (((*chip).chip_status & VX_STAT_DEVICE_INIT) == 0) as core::ffi::c_int;

    vx_reset_board(chip, cold_reset);
    vx_validate_irq(chip, 0);

    err = snd_vx_load_boot_image(chip, boot);
    if err < 0 {
        return err;
    }
    msleep(10);

    0
}

/**
 * snd_vx_dsp_load - load the DSP image
 * @chip: VX core instance
 * @dsp: firmware data
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_dsp_load(
    chip: *mut vx_core,
    dsp: *const firmware,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_uint;
    let mut err: core::ffi::c_int;
    let mut csum: core::ffi::c_uint = 0;
    let mut image: *const core::ffi::c_uchar;
    let mut cptr: *const core::ffi::c_uchar;

    if ((*dsp).size % 3) != 0 {
        return -EINVAL;
    }

    vx_toggle_dac_mute(chip, 1);

    /* Transfert data buffer from PC to DSP */
    i = 0;
    while (i as usize) < (*dsp).size {
        image = (*dsp).data.add(i as usize);
        /* Wait DSP ready for a new read */
        err = vx_wait_isr_bit(chip, ISR_TX_EMPTY);
        if err < 0 {
            dev_err((*(*chip).card).dev, b"dsp loading error at position %d\n\0".as_ptr() as *const _, i);
            return err;
        }
        cptr = image;
        csum ^= *cptr as core::ffi::c_uint;
        csum = (csum >> 24) | (csum << 8);
        vx_outb(chip, TXH, *cptr as core::ffi::c_int);
        cptr = cptr.add(1);
        csum ^= *cptr as core::ffi::c_uint;
        csum = (csum >> 24) | (csum << 8);
        vx_outb(chip, TXM, *cptr as core::ffi::c_int);
        cptr = cptr.add(1);
        csum ^= *cptr as core::ffi::c_uint;
        csum = (csum >> 24) | (csum << 8);
        vx_outb(chip, TXL, *cptr as core::ffi::c_int);
        cptr = cptr.add(1);

        i += 3;
    }

    msleep(200);

    err = vx_wait_isr_bit(chip, ISR_CHK);
    if err < 0 {
        return err;
    }

    vx_toggle_dac_mute(chip, 0);

    vx_test_and_ack(chip);
    vx_validate_irq(chip, 1);

    0
}

// Original C code conditionally compiled the following suspend/resume functions
// under CONFIG_PM.

/*
 * suspend
 */
#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn snd_vx_suspend(chip: *mut vx_core) -> core::ffi::c_int {
    snd_power_change_state((*chip).card, SNDRV_CTL_POWER_D3hot);
    (*chip).chip_status |= VX_STAT_IN_SUSPEND;

    0
}

/*
 * resume
 */
#[cfg(CONFIG_PM)]
#[no_mangle]
pub unsafe extern "C" fn snd_vx_resume(chip: *mut vx_core) -> core::ffi::c_int {
    let mut i: core::ffi::c_int;
    let mut err: core::ffi::c_int;

    (*chip).chip_status &= !VX_STAT_CHIP_INIT;

    i = 0;
    while i < 4 {
        if (*chip).firmware[i as usize].is_null() {
            i += 1;
            continue;
        }
        err = ((*(*chip).ops).load_dsp.unwrap())(chip, i, (*chip).firmware[i as usize]);
        if err < 0 {
            dev_err((*(*chip).card).dev, b"vx: firmware resume error at DSP %d\n\0".as_ptr() as *const _, i);
            return -EIO;
        }
        i += 1;
    }

    (*chip).chip_status |= VX_STAT_CHIP_INIT;
    (*chip).chip_status &= !VX_STAT_IN_SUSPEND;

    snd_power_change_state((*chip).card, SNDRV_CTL_POWER_D0);
    0
}

unsafe extern "C" fn snd_vx_release(_dev: *mut device, data: *mut core::ffi::c_void) {
    snd_vx_free_firmware(data);
}

/**
 * snd_vx_create - constructor for struct vx_core
 * @card: card instance
 * @hw: hardware specific record
 * @ops: VX ops pointer
 * @extra_size: extra byte size to allocate appending to chip
 *
 * this function allocates the instance and prepare for the hardware
 * initialization.
 *
 * The object is managed via devres, and will be automatically released.
 *
 * return the instance pointer if successful, NULL in error.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_vx_create(
    card: *mut snd_card,
    hw: *const snd_vx_hardware,
    ops: *const snd_vx_ops,
    extra_size: core::ffi::c_int,
) -> *mut vx_core {
    let chip: *mut vx_core;

    if snd_BUG_ON(card.is_null() || hw.is_null() || ops.is_null()) != 0 {
        return core::ptr::null_mut();
    }

    chip = devres_alloc(
        Some(snd_vx_release),
        core::mem::size_of::<vx_core>() + extra_size as usize,
        GFP_KERNEL,
    ) as *mut vx_core;
    if chip.is_null() {
        return core::ptr::null_mut();
    }
    mutex_init(&mut (*chip).lock);
    (*chip).irq = -1;
    (*chip).hw = hw;
    (*chip).type_ = (*hw).type_;
    (*chip).ops = ops;
    mutex_init(&mut (*chip).mixer_mutex);

    (*chip).card = card;
    (*card).private_data = chip as *mut _;
    strscpy((*card).driver.as_mut_ptr(), (*hw).name);
    sprintf((*card).shortname.as_mut_ptr(), b"Digigram %s\0".as_ptr() as *const _, (*hw).name);

    vx_proc_init(chip);

    chip
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
