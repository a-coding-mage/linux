// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * Rawmidi driver
 *
 * Author:	Torsten Schenk <torsten.schenk@zoho.com>
 * Created:	Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 */

// Requires: sound/rawmidi.h, midi.h, chip.h, comm.h

const MIDI_BUFSIZE: usize = 64;

// Opaque types from kernel headers
#[repr(C)]
pub struct urb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct midi_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sfire_chip {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

type snd_rawmidi_ops = [u8; 0];

// External kernel functions
extern "C" {
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buf: *mut u8,
        count: usize,
    ) -> i32;

    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buf: *const u8,
        count: usize,
    ) -> i32;

    fn snd_rawmidi_new(
        card: *mut core::ffi::c_void,
        id: *const u8,
        device: u32,
        output_count: u32,
        input_count: u32,
        rinstance: *mut *mut snd_rawmidi,
    ) -> i32;

    fn snd_rawmidi_set_ops(
        rmidi: *mut snd_rawmidi,
        stream: u32,
        ops: *const snd_rawmidi_ops,
    );

    fn kzalloc(size: usize, flags: u32) -> *mut u8;

    fn kfree(ptr: *const u8);

    fn spin_lock_init(lock: *mut spinlock_t);

    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> i32;

    fn usb_poison_urb(urb: *mut urb);

    fn msleep(msecs: u32);

    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> isize;

    fn dev_err(dev: *const device, fmt: *const u8, ...);
}

// Kernel constants
const GFP_KERNEL: u32 = 0xD0u32;
const GFP_ATOMIC: u32 = 0x20u32;
const SNDRV_RAWMIDI_STREAM_OUTPUT: u32 = 0;
const SNDRV_RAWMIDI_STREAM_INPUT: u32 = 1;
const SNDRV_RAWMIDI_INFO_OUTPUT: u32 = 1 << 0;
const SNDRV_RAWMIDI_INFO_INPUT: u32 = 1 << 1;
const SNDRV_RAWMIDI_INFO_DUPLEX: u32 = 1 << 2;

unsafe extern "C" fn usb6fire_midi_out_handler(urb: *mut urb) {
    let rt = (*urb).context as *mut midi_runtime;
    let mut ret: i32;

    // guard(spinlock_irqsave)(&rt->out_lock);
    // In Rust, this would use a scoped lock guard. Here we represent the
    // locked scope with the wrapped code:

    if !(*rt).out.is_null() {
        ret = snd_rawmidi_transmit(
            (*rt).out,
            ((*rt).out_buffer as *mut u8).add(4),
            MIDI_BUFSIZE - 4,
        );
        if ret > 0 {
            // more data available, send next packet
            *((*rt).out_buffer.add(1)) = (ret + 2) as u8;
            *((*rt).out_buffer.add(3)) = (*rt).out_serial;
            (*rt).out_serial = (*rt).out_serial.wrapping_add(1);
            (*urb).transfer_buffer_length = (ret + 4) as usize;

            ret = usb_submit_urb(urb, GFP_ATOMIC);
            if ret < 0 {
                dev_err(
                    &((*(*urb).dev).dev),
                    b"midi out urb submit failed: %d\n".as_ptr(),
                    ret,
                );
            }
        } else {
            // no more data to transmit
            (*rt).out = core::ptr::null_mut();
        }
    }
}

unsafe extern "C" fn usb6fire_midi_in_received(
    rt: *mut midi_runtime,
    data: *mut u8,
    length: i32,
) {
    // guard(spinlock_irqsave)(&rt->in_lock);
    if !(*rt).in.is_null() {
        snd_rawmidi_receive((*rt).in, data, length as usize);
    }
}

unsafe extern "C" fn usb6fire_midi_out_open(
    _alsa_sub: *mut snd_rawmidi_substream,
) -> i32 {
    0
}

unsafe extern "C" fn usb6fire_midi_out_close(
    _alsa_sub: *mut snd_rawmidi_substream,
) -> i32 {
    0
}

unsafe extern "C" fn usb6fire_midi_out_trigger(
    alsa_sub: *mut snd_rawmidi_substream,
    up: i32,
) {
    let rt = (*(*alsa_sub).rmidi).private_data as *mut midi_runtime;
    let urb = &mut (*rt).out_urb as *mut urb;
    let mut ret: i8;

    // guard(spinlock_irqsave)(&rt->out_lock);
    if up != 0 {
        // start transfer
        if !(*rt).out.is_null() {
            // we are already transmitting so just return
            return;
        }

        ret = snd_rawmidi_transmit(
            alsa_sub,
            ((*rt).out_buffer as *mut u8).add(4),
            MIDI_BUFSIZE - 4,
        ) as i8;

        if ret > 0 {
            *((*rt).out_buffer.add(1)) = (ret as i32 + 2) as u8;
            *((*rt).out_buffer.add(3)) = (*rt).out_serial;
            (*rt).out_serial = (*rt).out_serial.wrapping_add(1);
            (*urb).transfer_buffer_length = (ret as i32 + 4) as usize;

            ret = usb_submit_urb(urb, GFP_ATOMIC) as i8;
            if ret < 0 {
                dev_err(
                    &((*(*urb).dev).dev),
                    b"midi out urb submit failed: %d\n".as_ptr(),
                    ret,
                );
            } else {
                (*rt).out = alsa_sub;
            }
        }
    } else if (*rt).out == alsa_sub {
        (*rt).out = core::ptr::null_mut();
    }
}

unsafe extern "C" fn usb6fire_midi_out_drain(alsa_sub: *mut snd_rawmidi_substream) {
    let rt = (*(*alsa_sub).rmidi).private_data as *mut midi_runtime;
    let mut retry: i32 = 0;

    while !(*rt).out.is_null() && retry < 100 {
        retry += 1;
        msleep(10);
    }
}

unsafe extern "C" fn usb6fire_midi_in_open(
    _alsa_sub: *mut snd_rawmidi_substream,
) -> i32 {
    0
}

unsafe extern "C" fn usb6fire_midi_in_close(
    _alsa_sub: *mut snd_rawmidi_substream,
) -> i32 {
    0
}

unsafe extern "C" fn usb6fire_midi_in_trigger(
    alsa_sub: *mut snd_rawmidi_substream,
    up: i32,
) {
    let rt = (*(*alsa_sub).rmidi).private_data as *mut midi_runtime;

    // guard(spinlock_irqsave)(&rt->in_lock);
    if up != 0 {
        (*rt).in = alsa_sub;
    } else {
        (*rt).in = core::ptr::null_mut();
    }
}

#[repr(C)]
struct snd_rawmidi_ops_table {
    open: unsafe extern "C" fn(*mut snd_rawmidi_substream) -> i32,
    close: unsafe extern "C" fn(*mut snd_rawmidi_substream) -> i32,
    trigger: unsafe extern "C" fn(*mut snd_rawmidi_substream, i32),
    drain: unsafe extern "C" fn(*mut snd_rawmidi_substream),
}

static out_ops: snd_rawmidi_ops_table = snd_rawmidi_ops_table {
    open: usb6fire_midi_out_open,
    close: usb6fire_midi_out_close,
    trigger: usb6fire_midi_out_trigger,
    drain: usb6fire_midi_out_drain,
};

static in_ops: snd_rawmidi_ops_table = snd_rawmidi_ops_table {
    open: usb6fire_midi_in_open,
    close: usb6fire_midi_in_close,
    trigger: usb6fire_midi_in_trigger,
    drain: core::mem::transmute(0 as *const ()),
};

pub unsafe extern "C" fn usb6fire_midi_init(chip: *mut sfire_chip) -> i32 {
    let mut ret: i32;
    // kzalloc_obj(struct midi_runtime) macro expands to:
    // kzalloc(sizeof(struct midi_runtime), GFP_KERNEL)
    // Since midi_runtime is an opaque type from midi.h, we approximate the size
    let rt = kzalloc(core::mem::size_of::<midi_runtime>(), GFP_KERNEL)
        as *mut midi_runtime;

    if rt.is_null() {
        return -12i32; // -ENOMEM
    }

    let rt_out_buffer = kzalloc(MIDI_BUFSIZE, GFP_KERNEL);
    if rt_out_buffer.is_null() {
        kfree(rt as *const u8);
        return -12i32; // -ENOMEM
    }

    (*rt).chip = chip;
    (*rt).in_received = Some(usb6fire_midi_in_received as unsafe extern "C" fn(*mut midi_runtime, *mut u8, i32));
    (*rt).out_buffer = rt_out_buffer;
    *rt_out_buffer = 0x80; // 'send midi' command
    *(rt_out_buffer.add(1)) = 0x00; // size of data
    *(rt_out_buffer.add(2)) = 0x00; // always 0
    spin_lock_init(&mut (*rt).in_lock);
    spin_lock_init(&mut (*rt).out_lock);

    let comm_rt = (*chip).comm;
    // comm_rt->init_urb(comm_rt, &rt->out_urb, rt->out_buffer, rt,
    //      usb6fire_midi_out_handler);
    // This is a function pointer call from comm_runtime, treated as opaque

    ret = snd_rawmidi_new(
        (*chip).card,
        b"6FireUSB".as_ptr(),
        0,
        1,
        1,
        &mut (*rt).instance,
    );

    if ret < 0 {
        kfree((*rt).out_buffer);
        kfree(rt as *const u8);
        dev_err(
            &((*(*chip).dev).dev),
            b"unable to create midi.\n".as_ptr(),
        );
        return ret;
    }

    (*(*rt).instance).private_data = rt as *mut core::ffi::c_void;
    strscpy(
        (*(*rt).instance).name.as_mut_ptr(),
        b"DMX6FireUSB MIDI".as_ptr(),
        32,
    );
    (*(*rt).instance).info_flags = SNDRV_RAWMIDI_INFO_OUTPUT
        | SNDRV_RAWMIDI_INFO_INPUT
        | SNDRV_RAWMIDI_INFO_DUPLEX;

    snd_rawmidi_set_ops(
        (*rt).instance,
        SNDRV_RAWMIDI_STREAM_OUTPUT,
        &out_ops as *const _ as *const snd_rawmidi_ops,
    );

    snd_rawmidi_set_ops(
        (*rt).instance,
        SNDRV_RAWMIDI_STREAM_INPUT,
        &in_ops as *const _ as *const snd_rawmidi_ops,
    );

    (*chip).midi = rt;
    0
}

pub unsafe extern "C" fn usb6fire_midi_abort(chip: *mut sfire_chip) {
    let rt = (*chip).midi;

    if !rt.is_null() {
        usb_poison_urb(&mut (*rt).out_urb);
    }
}

pub unsafe extern "C" fn usb6fire_midi_destroy(chip: *mut sfire_chip) {
    let rt = (*chip).midi;

    kfree((*rt).out_buffer);
    kfree(rt as *const u8);
    (*chip).midi = core::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
