// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of EMU10K1 MPU-401 in UART mode
 */

// Dependencies from the original C includes:
// <linux/time.h>, <linux/init.h>, <sound/core.h>, <sound/emu10k1.h>

const EMU10K1_MIDI_MODE_INPUT: u32 = 1 << 0;
const EMU10K1_MIDI_MODE_OUTPUT: u32 = 1 << 1;

#[inline]
unsafe fn mpu401_read(
    emu: *mut snd_emu10k1,
    mpu: *mut snd_emu10k1_midi,
    idx: i32,
) -> u8 {
    if (*emu).audigy != 0 {
        snd_emu10k1_ptr_read(emu, (*mpu).port + idx as u32, 0) as u8
    } else {
        inb((*emu).port + (*mpu).port as libc::c_ulong + idx as libc::c_ulong)
    }
}

#[inline]
unsafe fn mpu401_write(
    emu: *mut snd_emu10k1,
    mpu: *mut snd_emu10k1_midi,
    data: i32,
    idx: i32,
) {
    if (*emu).audigy != 0 {
        snd_emu10k1_ptr_write(emu, (*mpu).port + idx as u32, 0, data);
    } else {
        outb(data, (*emu).port + (*mpu).port as libc::c_ulong + idx as libc::c_ulong);
    }
}

#[inline]
unsafe fn mpu401_write_data(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi, data: i32) {
    mpu401_write(emu, mpu, data, 0);
}

#[inline]
unsafe fn mpu401_write_cmd(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi, data: i32) {
    mpu401_write(emu, mpu, data, 1);
}

#[inline]
unsafe fn mpu401_read_data(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi) -> u8 {
    mpu401_read(emu, mpu, 0)
}

#[inline]
unsafe fn mpu401_read_stat(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi) -> u8 {
    mpu401_read(emu, mpu, 1)
}

#[inline]
unsafe fn mpu401_input_avail(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi) -> bool {
    (mpu401_read_stat(emu, mpu) & 0x80) == 0
}

#[inline]
unsafe fn mpu401_output_ready(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi) -> bool {
    (mpu401_read_stat(emu, mpu) & 0x40) == 0
}

const MPU401_RESET: u8 = 0xff;
const MPU401_ENTER_UART: u8 = 0x3f;
const MPU401_ACK: u8 = 0xfe;

unsafe fn mpu401_clear_rx(emu: *mut snd_emu10k1, mpu: *mut snd_emu10k1_midi) {
    let mut timeout: i32 = 100000;
    while timeout > 0 && mpu401_input_avail(emu, mpu) {
        timeout -= 1;
        mpu401_read_data(emu, mpu);
    }

    // CONFIG_SND_DEBUG:
    // if timeout <= 0 {
    //     dev_err((*(*emu).card).dev,
    //         "cmd: clear rx timeout (status = 0x%x)\n",
    //         mpu401_read_stat(emu, mpu));
    // }
}

/*

 */

unsafe fn do_emu10k1_midi_interrupt(
    emu: *mut snd_emu10k1,
    midi: *mut snd_emu10k1_midi,
    status: u32,
) {
    let mut byte: u8 = 0;

    if (*midi).rmidi.is_null() {
        snd_emu10k1_intr_disable(emu, (*midi).tx_enable | (*midi).rx_enable);
        return;
    }

    scoped_guard!(spinlock, &mut (*midi).input_lock, {
        if (status & (*midi).ipr_rx) != 0 && mpu401_input_avail(emu, midi) {
            if ((*midi).midi_mode & EMU10K1_MIDI_MODE_INPUT) == 0 {
                mpu401_clear_rx(emu, midi);
            } else {
                byte = mpu401_read_data(emu, midi);
                if !(*midi).substream_input.is_null() {
                    snd_rawmidi_receive((*midi).substream_input, &mut byte, 1);
                }
            }
        }
    });

    scoped_guard!(spinlock, &mut (*midi).output_lock, {
        if (status & (*midi).ipr_tx) != 0 && mpu401_output_ready(emu, midi) {
            if !(*midi).substream_output.is_null()
                && snd_rawmidi_transmit((*midi).substream_output, &mut byte, 1) == 1
            {
                mpu401_write_data(emu, midi, byte as i32);
            } else {
                snd_emu10k1_intr_disable(emu, (*midi).tx_enable);
            }
        }
    });
}

unsafe fn snd_emu10k1_midi_interrupt(emu: *mut snd_emu10k1, status: u32) {
    do_emu10k1_midi_interrupt(emu, &mut (*emu).midi, status);
}

unsafe fn snd_emu10k1_midi_interrupt2(emu: *mut snd_emu10k1, status: u32) {
    do_emu10k1_midi_interrupt(emu, &mut (*emu).midi2, status);
}

unsafe fn snd_emu10k1_midi_cmd(
    emu: *mut snd_emu10k1,
    midi: *mut snd_emu10k1_midi,
    cmd: u8,
    ack: i32,
) -> i32 {
    let mut timeout: i32;
    let mut ok: i32;

    scoped_guard!(spinlock_irq, &mut (*midi).input_lock, {
        mpu401_write_data(emu, midi, 0x00);
        /* mpu401_clear_rx(emu, midi); */

        mpu401_write_cmd(emu, midi, cmd as i32);
        if ack != 0 {
            ok = 0;
            timeout = 10000;
            while ok == 0 && {
                let old = timeout;
                timeout -= 1;
                old > 0
            } {
                if mpu401_input_avail(emu, midi) {
                    if mpu401_read_data(emu, midi) == MPU401_ACK {
                        ok = 1;
                    }
                }
            }
            if ok == 0 && mpu401_read_data(emu, midi) == MPU401_ACK {
                ok = 1;
            }
        } else {
            ok = 1;
        }
    });

    if ok == 0 {
        dev_err!(
            (*(*emu).card).dev,
            "midi_cmd: 0x%x failed at 0x%lx (status = 0x%x, data = 0x%x)!!!\n",
            cmd,
            (*emu).port,
            mpu401_read_stat(emu, midi),
            mpu401_read_data(emu, midi)
        );
        return 1;
    }
    0
}

unsafe fn snd_emu10k1_midi_input_open(substream: *mut snd_rawmidi_substream) -> i32 {
    let emu: *mut snd_emu10k1;
    let midi: *mut snd_emu10k1_midi =
        (*(*substream).rmidi).private_data as *mut snd_emu10k1_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) != 0 {
        return -ENXIO;
    }
    scoped_guard!(spinlock_irq, &mut (*midi).open_lock, {
        (*midi).midi_mode |= EMU10K1_MIDI_MODE_INPUT;
        (*midi).substream_input = substream;
        if ((*midi).midi_mode & EMU10K1_MIDI_MODE_OUTPUT) != 0 {
            return 0;
        }
    });
    if snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 1) != 0 {
        return -EIO;
    }
    if snd_emu10k1_midi_cmd(emu, midi, MPU401_ENTER_UART, 1) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_emu10k1_midi_output_open(substream: *mut snd_rawmidi_substream) -> i32 {
    let emu: *mut snd_emu10k1;
    let midi: *mut snd_emu10k1_midi =
        (*(*substream).rmidi).private_data as *mut snd_emu10k1_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) != 0 {
        return -ENXIO;
    }
    scoped_guard!(spinlock_irq, &mut (*midi).open_lock, {
        (*midi).midi_mode |= EMU10K1_MIDI_MODE_OUTPUT;
        (*midi).substream_output = substream;
        if ((*midi).midi_mode & EMU10K1_MIDI_MODE_INPUT) != 0 {
            return 0;
        }
    });
    if snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 1) != 0 {
        return -EIO;
    }
    if snd_emu10k1_midi_cmd(emu, midi, MPU401_ENTER_UART, 1) != 0 {
        return -EIO;
    }
    0
}

unsafe fn snd_emu10k1_midi_input_close(substream: *mut snd_rawmidi_substream) -> i32 {
    let emu: *mut snd_emu10k1;
    let midi: *mut snd_emu10k1_midi =
        (*(*substream).rmidi).private_data as *mut snd_emu10k1_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) != 0 {
        return -ENXIO;
    }
    scoped_guard!(spinlock_irq, &mut (*midi).open_lock, {
        snd_emu10k1_intr_disable(emu, (*midi).rx_enable);
        (*midi).midi_mode &= !EMU10K1_MIDI_MODE_INPUT;
        (*midi).substream_input = core::ptr::null_mut();
        if ((*midi).midi_mode & EMU10K1_MIDI_MODE_OUTPUT) != 0 {
            return 0;
        }
    });
    snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 0)
}

unsafe fn snd_emu10k1_midi_output_close(substream: *mut snd_rawmidi_substream) -> i32 {
    let emu: *mut snd_emu10k1;
    let midi: *mut snd_emu10k1_midi =
        (*(*substream).rmidi).private_data as *mut snd_emu10k1_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) != 0 {
        return -ENXIO;
    }
    scoped_guard!(spinlock_irq, &mut (*midi).open_lock, {
        snd_emu10k1_intr_disable(emu, (*midi).tx_enable);
        (*midi).midi_mode &= !EMU10K1_MIDI_MODE_OUTPUT;
        (*midi).substream_output = core::ptr::null_mut();
        if ((*midi).midi_mode & EMU10K1_MIDI_MODE_INPUT) != 0 {
            return 0;
        }
    });
    snd_emu10k1_midi_cmd(emu, midi, MPU401_RESET, 0)
}

unsafe fn snd_emu10k1_midi_input_trigger(substream: *mut snd_rawmidi_substream, up: i32) {
    let emu: *mut snd_emu10k1;
    let midi: *mut snd_emu10k1_midi =
        (*(*substream).rmidi).private_data as *mut snd_emu10k1_midi;
    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) != 0 {
        return;
    }

    if up != 0 {
        snd_emu10k1_intr_enable(emu, (*midi).rx_enable);
    } else {
        snd_emu10k1_intr_disable(emu, (*midi).rx_enable);
    }
}

unsafe fn snd_emu10k1_midi_output_trigger(substream: *mut snd_rawmidi_substream, up: i32) {
    let emu: *mut snd_emu10k1;
    let midi: *mut snd_emu10k1_midi =
        (*(*substream).rmidi).private_data as *mut snd_emu10k1_midi;

    emu = (*midi).emu;
    if snd_BUG_ON(emu.is_null()) != 0 {
        return;
    }

    if up != 0 {
        let mut max: i32 = 4;
        let mut byte: u8 = 0;

        /* try to send some amount of bytes here before interrupts */
        scoped_guard!(spinlock_irq, &mut (*midi).output_lock, {
            while max > 0 {
                if mpu401_output_ready(emu, midi) {
                    if ((*midi).midi_mode & EMU10K1_MIDI_MODE_OUTPUT) == 0
                        || snd_rawmidi_transmit(substream, &mut byte, 1) != 1
                    {
                        /* no more data */
                        return;
                    }
                    mpu401_write_data(emu, midi, byte as i32);
                    max -= 1;
                } else {
                    break;
                }
            }
        });
        snd_emu10k1_intr_enable(emu, (*midi).tx_enable);
    } else {
        snd_emu10k1_intr_disable(emu, (*midi).tx_enable);
    }
}

/*

 */

static snd_emu10k1_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_emu10k1_midi_output_open),
    close: Some(snd_emu10k1_midi_output_close),
    trigger: Some(snd_emu10k1_midi_output_trigger),
};

static snd_emu10k1_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_emu10k1_midi_input_open),
    close: Some(snd_emu10k1_midi_input_close),
    trigger: Some(snd_emu10k1_midi_input_trigger),
};

unsafe fn snd_emu10k1_midi_free(rmidi: *mut snd_rawmidi) {
    let midi: *mut snd_emu10k1_midi = (*rmidi).private_data as *mut snd_emu10k1_midi;
    (*midi).interrupt = None;
    (*midi).rmidi = core::ptr::null_mut();
}

unsafe fn emu10k1_midi_init(
    emu: *mut snd_emu10k1,
    midi: *mut snd_emu10k1_midi,
    device: i32,
    name: *mut libc::c_char,
) -> i32 {
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();
    let err: i32;

    err = snd_rawmidi_new((*emu).card, name, device, 1, 1, &mut rmidi);
    if err < 0 {
        return err;
    }
    (*midi).emu = emu;
    spin_lock_init(&mut (*midi).open_lock);
    spin_lock_init(&mut (*midi).input_lock);
    spin_lock_init(&mut (*midi).output_lock);
    strscpy((*rmidi).name.as_mut_ptr(), name);
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_OUTPUT,
        &snd_emu10k1_midi_output,
    );
    snd_rawmidi_set_ops(
        rmidi,
        SNDRV_RAWMIDI_STREAM_INPUT,
        &snd_emu10k1_midi_input,
    );
    (*rmidi).info_flags |=
        SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT | SNDRV_RAWMIDI_INFO_DUPLEX;
    (*rmidi).private_data = midi as *mut libc::c_void;
    (*rmidi).private_free = Some(snd_emu10k1_midi_free);
    (*midi).rmidi = rmidi;
    0
}

pub unsafe fn snd_emu10k1_midi(emu: *mut snd_emu10k1) -> i32 {
    let midi: *mut snd_emu10k1_midi = &mut (*emu).midi;
    let err: i32;

    err = emu10k1_midi_init(
        emu,
        midi,
        0,
        c"EMU10K1 MPU-401 (UART)".as_ptr() as *mut libc::c_char,
    );
    if err < 0 {
        return err;
    }

    (*midi).tx_enable = INTE_MIDITXENABLE;
    (*midi).rx_enable = INTE_MIDIRXENABLE;
    (*midi).port = MUDATA;
    (*midi).ipr_tx = IPR_MIDITRANSBUFEMPTY;
    (*midi).ipr_rx = IPR_MIDIRECVBUFEMPTY;
    (*midi).interrupt = Some(snd_emu10k1_midi_interrupt);
    0
}

pub unsafe fn snd_emu10k1_audigy_midi(emu: *mut snd_emu10k1) -> i32 {
    let mut midi: *mut snd_emu10k1_midi;
    let err: i32;

    midi = &mut (*emu).midi;
    err = emu10k1_midi_init(
        emu,
        midi,
        0,
        c"Audigy MPU-401 (UART)".as_ptr() as *mut libc::c_char,
    );
    if err < 0 {
        return err;
    }

    (*midi).tx_enable = INTE_MIDITXENABLE;
    (*midi).rx_enable = INTE_MIDIRXENABLE;
    (*midi).port = A_MUDATA1;
    (*midi).ipr_tx = IPR_MIDITRANSBUFEMPTY;
    (*midi).ipr_rx = IPR_MIDIRECVBUFEMPTY;
    (*midi).interrupt = Some(snd_emu10k1_midi_interrupt);

    midi = &mut (*emu).midi2;
    err = emu10k1_midi_init(
        emu,
        midi,
        1,
        c"Audigy MPU-401 #2".as_ptr() as *mut libc::c_char,
    );
    if err < 0 {
        return err;
    }

    (*midi).tx_enable = INTE_A_MIDITXENABLE2;
    (*midi).rx_enable = INTE_A_MIDIRXENABLE2;
    (*midi).port = A_MUDATA2;
    (*midi).ipr_tx = IPR_A_MIDITRANSBUFEMPTY2;
    (*midi).ipr_rx = IPR_A_MIDIRECVBUFEMPTY2;
    (*midi).interrupt = Some(snd_emu10k1_midi_interrupt2);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
