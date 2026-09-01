// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) by Paul Barton-Davis 1998-1999
 */

/* The low level driver for the WaveFront ICS2115 MIDI interface(s)
 *
 * Note that there is also an MPU-401 emulation (actually, a UART-401
 * emulation) on the CS4232 on the Tropez and Tropez Plus. This code
 * has nothing to do with that interface at all.
 *
 * The interface is essentially just a UART-401, but is has the
 * interesting property of supporting what Turtle Beach called
 * "Virtual MIDI" mode. In this mode, there are effectively *two*
 * MIDI buses accessible via the interface, one that is routed
 * solely to/from the external WaveFront synthesizer and the other
 * corresponding to the pin/socket connector used to link external
 * MIDI devices to the board.
 *
 * This driver fully supports this mode, allowing two distinct MIDI
 * busses to be used completely independently, giving 32 channels of
 * MIDI routing, 16 to the WaveFront synth and 16 to the external MIDI
 * bus. The devices are named /dev/snd/midiCnD0 and /dev/snd/midiCnD1,
 * where `n' is the card number. Note that the device numbers may be
 * something other than 0 and 1 if the CS4232 UART/MPU-401 interface
 * is enabled.
 *
 * Switching between the two is accomplished externally by the driver
 * using the two otherwise unused MIDI bytes. See the code for more details.
 *
 * NOTE: VIRTUAL MIDI MODE IS ON BY DEFAULT (see lowlevel/isa/wavefront.c)
 *
 * The main reason to turn off Virtual MIDI mode is when you want to
 * tightly couple the WaveFront synth with an external MIDI
 * device. You won't be able to distinguish the source of any MIDI
 * data except via SysEx ID, but thats probably OK, since for the most
 * part, the WaveFront won't be sending any MIDI data at all.
 *
 * The main reason to turn on Virtual MIDI Mode is to provide two
 * completely independent 16-channel MIDI buses, one to the
 * WaveFront and one to any external MIDI devices. Given the 32
 * voice nature of the WaveFront, its pretty easy to find a use
 * for all 16 channels driving just that synth.
 *
 */

/* Dependencies originally supplied by:
 * <linux/io.h>, <linux/init.h>, <linux/time.h>, <linux/wait.h>,
 * <sound/core.h>, and <sound/snd_wavefront.h>.
 */

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type snd_wavefront_mpu_id = usize;

const NULL: *mut core::ffi::c_void = core::ptr::null_mut();

extern "C" {
    static INPUT_AVAIL: c_int;
    static OUTPUT_READY: c_int;
    static MPU401_MODE_OUTPUT: c_int;
    static MPU401_MODE_OUTPUT_TRIGGER: c_int;
    static MPU401_MODE_INPUT: c_int;
    static MPU401_MODE_INPUT_TRIGGER: c_int;
    static WF_INTERNAL_SWITCH: u8;
    static WF_EXTERNAL_SWITCH: u8;
    static internal_mpu: snd_wavefront_mpu_id;
    static external_mpu: snd_wavefront_mpu_id;
    static UART_MODE_ON: u8;
    static MPU_ACK: c_int;
    static WFC_MISYNTH_ON: c_int;
    static WFC_VMIDI_OFF: c_int;
    static WFC_VMIDI_ON: c_int;
    static mut jiffies: c_ulong;
    static ENXIO: c_int;
    static EIO: c_int;

    fn inb(port: c_ulong) -> c_int;
    fn outb(byte: u8, port: c_ulong);
    fn snd_BUG_ON(cond: bool) -> c_int;
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buf: *mut u8,
        count: c_int,
    ) -> c_int;
    fn snd_rawmidi_transmit_empty(substream: *mut snd_rawmidi_substream) -> bool;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buf: *mut u8, count: c_int);
    fn timer_delete(timer: *mut timer_list);
    fn timer_delete_sync(timer: *mut timer_list);
    fn timer_setup(
        timer: *mut timer_list,
        function: unsafe extern "C" fn(*mut timer_list),
        flags: c_uint,
    );
    fn mod_timer(timer: *mut timer_list, expires: c_ulong);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn snd_wavefront_cmd(
        dev: *mut snd_wavefront_t,
        cmd: c_int,
        rbuf: *mut u8,
        wbuf: *mut u8,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_warn(dev: *mut device, fmt: *const u8, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_rawmidi {
    pub card: *mut snd_card,
    pub private_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    pub rmidi: *mut snd_rawmidi,
}

#[repr(C)]
pub struct snd_wavefront_midi_t {
    pub mpu_status_port: c_ulong,
    pub mpu_data_port: c_ulong,
    pub mpu_command_port: c_ulong,
    pub virtual_: spinlock_t,
    pub open: spinlock_t,
    pub mode: [c_int; 2],
    pub substream_output: [*mut snd_rawmidi_substream; 2],
    pub substream_input: [*mut snd_rawmidi_substream; 2],
    pub output_mpu: snd_wavefront_mpu_id,
    pub input_mpu: snd_wavefront_mpu_id,
    pub isvirtual: c_int,
    pub istimer: c_int,
    pub timer: timer_list,
    pub timer_card: *mut snd_wavefront_card_t,
}

#[repr(C)]
pub struct snd_wavefront_t {
    pub midi: snd_wavefront_midi_t,
    pub card: *mut snd_card,
    pub interrupts_are_midi: c_int,
    pub midi_in_to_synth: c_int,
}

#[repr(C)]
pub struct snd_wavefront_card_t {
    pub wavefront: snd_wavefront_t,
}

#[repr(C)]
pub struct snd_rawmidi_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_rawmidi_substream, c_int)>,
}

#[inline]
unsafe fn wf_mpu_status(midi: *mut snd_wavefront_midi_t) -> c_int {
    inb((*midi).mpu_status_port)
}

#[inline]
unsafe fn input_avail(midi: *mut snd_wavefront_midi_t) -> c_int {
    if (wf_mpu_status(midi) & INPUT_AVAIL) == 0 { 1 } else { 0 }
}

#[inline]
unsafe fn output_ready(midi: *mut snd_wavefront_midi_t) -> c_int {
    if (wf_mpu_status(midi) & OUTPUT_READY) == 0 { 1 } else { 0 }
}

#[inline]
unsafe fn read_data(midi: *mut snd_wavefront_midi_t) -> c_int {
    inb((*midi).mpu_data_port)
}

#[inline]
unsafe fn write_data(midi: *mut snd_wavefront_midi_t, byte: u8) {
    outb(byte, (*midi).mpu_data_port);
}

unsafe fn get_wavefront_midi(
    substream: *mut snd_rawmidi_substream,
) -> *mut snd_wavefront_midi_t {
    let mut card: *mut snd_card;
    let mut acard: *mut snd_wavefront_card_t;

    if substream.is_null() || (*substream).rmidi.is_null() {
        return core::ptr::null_mut();
    }

    card = (*(*substream).rmidi).card;

    if card.is_null() {
        return core::ptr::null_mut();
    }

    if (*card).private_data.is_null() {
        return core::ptr::null_mut();
    }

    acard = (*card).private_data as *mut snd_wavefront_card_t;

    &mut (*acard).wavefront.midi
}

unsafe fn snd_wavefront_midi_output_write(card: *mut snd_wavefront_card_t) {
    let midi: *mut snd_wavefront_midi_t = &mut (*card).wavefront.midi;
    let mut mpu: snd_wavefront_mpu_id;
    let mut midi_byte: u8 = 0;
    let mut max: c_int = 256;
    let mut mask: snd_wavefront_mpu_id = 1;
    let mut timeout: c_int;

    /* Its not OK to try to change the status of "virtuality" of
       the MIDI interface while we're outputting stuff.  See
       snd_wavefront_midi_{enable,disable}_virtual () for the
       other half of this.

       The first loop attempts to flush any data from the
       current output device, and then the second
       emits the switch byte (if necessary), and starts
       outputting data for the output device currently in use.
    */

    if (*midi).substream_output[(*midi).output_mpu].is_null() {
        // goto __second
    } else {
        while max > 0 {
            /* XXX fix me - no hard timing loops allowed! */

            timeout = 30000;
            while timeout > 0 {
                if output_ready(midi) != 0 {
                    break;
                }
                timeout -= 1;
            }

            let mut flags: c_ulong = 0;
            spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
            if ((*midi).mode[(*midi).output_mpu] & MPU401_MODE_OUTPUT) == 0 {
                spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
                break;
            }
            if output_ready(midi) != 0 {
                if snd_rawmidi_transmit(
                    (*midi).substream_output[(*midi).output_mpu],
                    &mut midi_byte,
                    1,
                ) == 1
                {
                    if (*midi).isvirtual == 0
                        || (midi_byte != WF_INTERNAL_SWITCH && midi_byte != WF_EXTERNAL_SWITCH)
                    {
                        write_data(midi, midi_byte);
                    }
                    max -= 1;
                    spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
                } else {
                    if (*midi).istimer != 0 {
                        (*midi).istimer -= 1;
                        if (*midi).istimer <= 0 {
                            timer_delete(&mut (*midi).timer);
                        }
                    }
                    (*midi).mode[(*midi).output_mpu] &= !MPU401_MODE_OUTPUT_TRIGGER;
                    spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
                    break;
                }
            } else {
                spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
                return;
            }
        }
    }

    if (*midi).substream_output[((*midi).output_mpu == 0) as usize].is_null() {
        return;
    }

    while max > 0 {
        /* XXX fix me - no hard timing loops allowed! */

        timeout = 30000;
        while timeout > 0 {
            if output_ready(midi) != 0 {
                break;
            }
            timeout -= 1;
        }

        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
        if (*midi).isvirtual == 0 {
            mask = 0;
        }
        mpu = (*midi).output_mpu ^ mask;
        mask = 0; /* don't invert the value from now */
        if ((*midi).mode[mpu] & MPU401_MODE_OUTPUT) == 0 {
            spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
            return;
        }
        if snd_rawmidi_transmit_empty((*midi).substream_output[mpu]) {
            if (*midi).istimer != 0 {
                (*midi).istimer -= 1;
                if (*midi).istimer <= 0 {
                    timer_delete(&mut (*midi).timer);
                }
            }
            (*midi).mode[mpu] &= !MPU401_MODE_OUTPUT_TRIGGER;
            spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
            return;
        }
        if output_ready(midi) != 0 {
            if mpu != (*midi).output_mpu {
                write_data(
                    midi,
                    if mpu == internal_mpu {
                        WF_INTERNAL_SWITCH
                    } else {
                        WF_EXTERNAL_SWITCH
                    },
                );
                (*midi).output_mpu = mpu;
                spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
            } else if snd_rawmidi_transmit((*midi).substream_output[mpu], &mut midi_byte, 1) == 1 {
                if (*midi).isvirtual == 0
                    || (midi_byte != WF_INTERNAL_SWITCH && midi_byte != WF_EXTERNAL_SWITCH)
                {
                    write_data(midi, midi_byte);
                }
                max -= 1;
                spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
            } else {
                if (*midi).istimer != 0 {
                    (*midi).istimer -= 1;
                    if (*midi).istimer <= 0 {
                        timer_delete(&mut (*midi).timer);
                    }
                }
                (*midi).mode[mpu] &= !MPU401_MODE_OUTPUT_TRIGGER;
                spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
                return;
            }
        } else {
            spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
            return;
        }
    }
}

unsafe extern "C" fn snd_wavefront_midi_input_open(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let midi: *mut snd_wavefront_midi_t;
    let mpu: snd_wavefront_mpu_id;

    if snd_BUG_ON(substream.is_null() || (*substream).rmidi.is_null()) != 0 {
        return -ENXIO;
    }
    if snd_BUG_ON((*(*substream).rmidi).private_data.is_null()) != 0 {
        return -ENXIO;
    }

    mpu = *((*(*substream).rmidi).private_data as *mut snd_wavefront_mpu_id);

    midi = get_wavefront_midi(substream);
    if midi.is_null() {
        return -EIO;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*midi).open, &mut flags);
    (*midi).mode[mpu] |= MPU401_MODE_INPUT;
    (*midi).substream_input[mpu] = substream;
    spin_unlock_irqrestore(&mut (*midi).open, flags);

    0
}

unsafe extern "C" fn snd_wavefront_midi_output_open(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let midi: *mut snd_wavefront_midi_t;
    let mpu: snd_wavefront_mpu_id;

    if snd_BUG_ON(substream.is_null() || (*substream).rmidi.is_null()) != 0 {
        return -ENXIO;
    }
    if snd_BUG_ON((*(*substream).rmidi).private_data.is_null()) != 0 {
        return -ENXIO;
    }

    mpu = *((*(*substream).rmidi).private_data as *mut snd_wavefront_mpu_id);

    midi = get_wavefront_midi(substream);
    if midi.is_null() {
        return -EIO;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*midi).open, &mut flags);
    (*midi).mode[mpu] |= MPU401_MODE_OUTPUT;
    (*midi).substream_output[mpu] = substream;
    spin_unlock_irqrestore(&mut (*midi).open, flags);

    0
}

unsafe extern "C" fn snd_wavefront_midi_input_close(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let midi: *mut snd_wavefront_midi_t;
    let mpu: snd_wavefront_mpu_id;

    if snd_BUG_ON(substream.is_null() || (*substream).rmidi.is_null()) != 0 {
        return -ENXIO;
    }
    if snd_BUG_ON((*(*substream).rmidi).private_data.is_null()) != 0 {
        return -ENXIO;
    }

    mpu = *((*(*substream).rmidi).private_data as *mut snd_wavefront_mpu_id);

    midi = get_wavefront_midi(substream);
    if midi.is_null() {
        return -EIO;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*midi).open, &mut flags);
    (*midi).substream_input[mpu] = core::ptr::null_mut();
    (*midi).mode[mpu] &= !MPU401_MODE_INPUT;
    spin_unlock_irqrestore(&mut (*midi).open, flags);

    0
}

unsafe extern "C" fn snd_wavefront_midi_output_close(
    substream: *mut snd_rawmidi_substream,
) -> c_int {
    let midi: *mut snd_wavefront_midi_t;
    let mpu: snd_wavefront_mpu_id;

    if snd_BUG_ON(substream.is_null() || (*substream).rmidi.is_null()) != 0 {
        return -ENXIO;
    }
    if snd_BUG_ON((*(*substream).rmidi).private_data.is_null()) != 0 {
        return -ENXIO;
    }

    mpu = *((*(*substream).rmidi).private_data as *mut snd_wavefront_mpu_id);

    midi = get_wavefront_midi(substream);
    if midi.is_null() {
        return -EIO;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*midi).open, &mut flags);
    (*midi).substream_output[mpu] = core::ptr::null_mut();
    (*midi).mode[mpu] &= !MPU401_MODE_OUTPUT;
    spin_unlock_irqrestore(&mut (*midi).open, flags);
    0
}

unsafe extern "C" fn snd_wavefront_midi_input_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let midi: *mut snd_wavefront_midi_t;
    let mpu: snd_wavefront_mpu_id;

    if substream.is_null() || (*substream).rmidi.is_null() {
        return;
    }

    if (*(*substream).rmidi).private_data.is_null() {
        return;
    }

    mpu = *((*(*substream).rmidi).private_data as *mut snd_wavefront_mpu_id);

    midi = get_wavefront_midi(substream);
    if midi.is_null() {
        return;
    }

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
    if up != 0 {
        (*midi).mode[mpu] |= MPU401_MODE_INPUT_TRIGGER;
    } else {
        (*midi).mode[mpu] &= !MPU401_MODE_INPUT_TRIGGER;
    }
    spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
}

unsafe extern "C" fn snd_wavefront_midi_output_timer(t: *mut timer_list) {
    let midi: *mut snd_wavefront_midi_t =
        (t as *mut u8).sub(core::mem::offset_of!(snd_wavefront_midi_t, timer))
            as *mut snd_wavefront_midi_t;
    let card: *mut snd_wavefront_card_t = (*midi).timer_card;

    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
        mod_timer(&mut (*midi).timer, 1 + jiffies);
        spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
    }
    snd_wavefront_midi_output_write(card);
}

unsafe extern "C" fn snd_wavefront_midi_output_trigger(
    substream: *mut snd_rawmidi_substream,
    up: c_int,
) {
    let midi: *mut snd_wavefront_midi_t;
    let mpu: snd_wavefront_mpu_id;

    if substream.is_null() || (*substream).rmidi.is_null() {
        return;
    }

    if (*(*substream).rmidi).private_data.is_null() {
        return;
    }

    mpu = *((*(*substream).rmidi).private_data as *mut snd_wavefront_mpu_id);

    midi = get_wavefront_midi(substream);
    if midi.is_null() {
        return;
    }

    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
        if up != 0 {
            if ((*midi).mode[mpu] & MPU401_MODE_OUTPUT_TRIGGER) == 0 {
                if (*midi).istimer == 0 {
                    timer_setup(&mut (*midi).timer, snd_wavefront_midi_output_timer, 0);
                    mod_timer(&mut (*midi).timer, 1 + jiffies);
                }
                (*midi).istimer += 1;
                (*midi).mode[mpu] |= MPU401_MODE_OUTPUT_TRIGGER;
            }
        } else {
            (*midi).mode[mpu] &= !MPU401_MODE_OUTPUT_TRIGGER;
        }
        spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
    }

    if up != 0 {
        snd_wavefront_midi_output_write((*(*(*substream).rmidi).card).private_data as *mut snd_wavefront_card_t);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_midi_interrupt(card: *mut snd_wavefront_card_t) {
    let midi: *mut snd_wavefront_midi_t;
    static mut substream: *mut snd_rawmidi_substream = core::ptr::null_mut();
    static mut mpu: snd_wavefront_mpu_id = 1;
    let mut max: c_int = 128;
    let mut byte: u8;

    mpu = external_mpu;
    midi = &mut (*card).wavefront.midi;

    if input_avail(midi) == 0 {
        /* not for us */
        snd_wavefront_midi_output_write(card);
        return;
    }

    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
        loop {
            max -= 1;
            if max == 0 {
                break;
            }

            if input_avail(midi) != 0 {
                byte = read_data(midi) as u8;

                if (*midi).isvirtual != 0 {
                    if byte == WF_EXTERNAL_SWITCH {
                        substream = (*midi).substream_input[external_mpu];
                        mpu = external_mpu;
                    } else if byte == WF_INTERNAL_SWITCH {
                        substream = (*midi).substream_output[internal_mpu];
                        mpu = internal_mpu;
                    } /* else just leave it as it is */
                } else {
                    substream = (*midi).substream_input[internal_mpu];
                    mpu = internal_mpu;
                }

                if substream.is_null() {
                    continue;
                }

                if ((*midi).mode[mpu] & MPU401_MODE_INPUT_TRIGGER) != 0 {
                    snd_rawmidi_receive(substream, &mut byte, 1);
                }
            } else {
                break;
            }
        }
        spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
    }

    snd_wavefront_midi_output_write(card);
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_midi_enable_virtual(card: *mut snd_wavefront_card_t) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*card).wavefront.midi.virtual_, &mut flags);
    (*card).wavefront.midi.isvirtual = 1;
    (*card).wavefront.midi.output_mpu = internal_mpu;
    (*card).wavefront.midi.input_mpu = internal_mpu;
    spin_unlock_irqrestore(&mut (*card).wavefront.midi.virtual_, flags);
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_midi_disable_virtual(card: *mut snd_wavefront_card_t) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*card).wavefront.midi.virtual_, &mut flags);
    // snd_wavefront_midi_input_close (card->ics2115_external_rmidi);
    // snd_wavefront_midi_output_close (card->ics2115_external_rmidi);
    (*card).wavefront.midi.isvirtual = 0;
    spin_unlock_irqrestore(&mut (*card).wavefront.midi.virtual_, flags);
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_midi_suspend(card: *mut snd_wavefront_card_t) {
    let midi: *mut snd_wavefront_midi_t = &mut (*card).wavefront.midi;

    if (*midi).istimer == 0 {
        return;
    }

    timer_delete_sync(&mut (*midi).timer);

    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
    (*midi).istimer = 0;
    spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_midi_resume(card: *mut snd_wavefront_card_t) {
    let midi: *mut snd_wavefront_midi_t = &mut (*card).wavefront.midi;
    let mut istimer: c_int = 0;
    let mut pending_output: bool = false;

    (*midi).timer_card = card;

    {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*midi).virtual_, &mut flags);
        if ((*midi).mode[internal_mpu] & MPU401_MODE_OUTPUT_TRIGGER) != 0 {
            istimer += 1;
        }
        if ((*midi).mode[external_mpu] & MPU401_MODE_OUTPUT_TRIGGER) != 0 {
            istimer += 1;
        }
        if istimer == 0 {
            spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
            return;
        }

        (*midi).istimer = istimer;
        timer_setup(&mut (*midi).timer, snd_wavefront_midi_output_timer, 0);
        mod_timer(&mut (*midi).timer, 1 + jiffies);
        pending_output = true;
        spin_unlock_irqrestore(&mut (*midi).virtual_, flags);
    }

    if pending_output {
        snd_wavefront_midi_output_write(card);
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_wavefront_midi_start(card: *mut snd_wavefront_card_t) -> c_int {
    let mut ok: c_int;
    let mut i: c_int;
    let mut rbuf: [u8; 4] = [0; 4];
    let mut wbuf: [u8; 4] = [0; 4];
    let dev: *mut snd_wavefront_t;
    let midi: *mut snd_wavefront_midi_t;

    dev = &mut (*card).wavefront;
    midi = &mut (*dev).midi;
    (*midi).timer_card = card;

    /* The ICS2115 MPU-401 interface doesn't do anything
       until its set into UART mode.
    */

    /* XXX fix me - no hard timing loops allowed! */

    i = 0;
    while i < 30000 && output_ready(midi) == 0 {
        i += 1;
    }

    if output_ready(midi) == 0 {
        dev_err(
            (*(*card).wavefront.card).dev,
            b"MIDI interface not ready for command\n\0".as_ptr(),
        );
        return -1;
    }

    /* Any interrupts received from now on
       are owned by the MIDI side of things.
    */

    (*dev).interrupts_are_midi = 1;

    outb(UART_MODE_ON, (*midi).mpu_command_port);

    ok = 0;
    i = 50000;
    while i > 0 && ok == 0 {
        if input_avail(midi) != 0 {
            if read_data(midi) == MPU_ACK {
                ok = 1;
                break;
            }
        }
        i -= 1;
    }

    if ok == 0 {
        dev_err(
            (*(*card).wavefront.card).dev,
            b"cannot set UART mode for MIDI interface\0".as_ptr(),
        );
        (*dev).interrupts_are_midi = 0;
        return -1;
    }

    /* Route external MIDI to WaveFront synth (by default) */

    if snd_wavefront_cmd(dev, WFC_MISYNTH_ON, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 {
        dev_warn(
            (*(*card).wavefront.card).dev,
            b"can't enable MIDI-IN-2-synth routing.\n\0".as_ptr(),
        );
        /* XXX error ? */
    } else {
        (*dev).midi_in_to_synth = 1;
    }

    /* Turn on Virtual MIDI, but first *always* turn it off,
       since otherwise consecutive reloads of the driver will
       never cause the hardware to generate the initial "internal" or
       "external" source bytes in the MIDI data stream. This
       is pretty important, since the internal hardware generally will
       be used to generate none or very little MIDI output, and
       thus the only source of MIDI data is actually external. Without
       the switch bytes, the driver will think it all comes from
       the internal interface. Duh.
    */

    if snd_wavefront_cmd(dev, WFC_VMIDI_OFF, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 {
        dev_warn(
            (*(*card).wavefront.card).dev,
            b"virtual MIDI mode not disabled\n\0".as_ptr(),
        );
        return 0; /* We're OK, but missing the external MIDI dev */
    }

    snd_wavefront_midi_enable_virtual(card);

    if snd_wavefront_cmd(dev, WFC_VMIDI_ON, rbuf.as_mut_ptr(), wbuf.as_mut_ptr()) != 0 {
        dev_warn(
            (*(*card).wavefront.card).dev,
            b"cannot enable virtual MIDI mode.\n\0".as_ptr(),
        );
        snd_wavefront_midi_disable_virtual(card);
    }
    0
}

#[no_mangle]
pub static snd_wavefront_midi_output: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_wavefront_midi_output_open),
    close: Some(snd_wavefront_midi_output_close),
    trigger: Some(snd_wavefront_midi_output_trigger),
};

#[no_mangle]
pub static snd_wavefront_midi_input: snd_rawmidi_ops = snd_rawmidi_ops {
    open: Some(snd_wavefront_midi_input_open),
    close: Some(snd_wavefront_midi_input_close),
    trigger: Some(snd_wavefront_midi_input_trigger),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
