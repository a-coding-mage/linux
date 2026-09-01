// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of MPU-401 in UART mode
 *
 *   Modified for the Aureal Vortex based Soundcards
 *   by Manuel Jander (mjande@embedded.cl).
 */

/* Dependencies from the original C includes:
 * <linux/time.h>, <linux/init.h>, <sound/core.h>, <sound/mpu401.h>, "au88x0.h"
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Check for mpu401 mmio support. */
/* MPU401 legacy support is only provided as a emergency fallback *
 * for older versions of ALSA. Its usage is strongly discouraged. */
/* C used:
 * #ifndef MPU401_HW_AUREAL
 * #define VORTEX_MPU401_LEGACY
 * #endif
 */

/* Vortex MPU401 defines. */
const MIDI_CLOCK_DIV: c_int = 0x61;
/* Standart MPU401 defines. */
const MPU401_RESET: c_int = 0xff;
const MPU401_ENTER_UART: c_int = 0x3f;
const MPU401_ACK: c_int = 0xfe;

unsafe extern "C" {
    static CARD_NAME_SHORT: *const c_char;

    fn hwread(mmio: *mut core::ffi::c_void, reg: c_int) -> c_int;
    fn hwwrite(mmio: *mut core::ffi::c_void, reg: c_int, value: c_int);
    fn snd_mpu401_uart_new(
        card: *mut snd_card,
        device: c_int,
        hardware: c_int,
        port: c_ulong,
        info_flags: c_int,
        irq: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, format: *const c_char, ...);
}

#[repr(C)]
struct vortex_t {
    mmio: *mut c_void,
    card: *mut snd_card,
    rmidi: *mut snd_rawmidi,
}

#[repr(C)]
struct snd_rawmidi {
    private_data: *mut c_void,
    name: [c_char; 80],
}

#[repr(C)]
struct snd_mpu401 {
    cport: c_ulong,
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    number: c_int,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

unsafe fn snd_vortex_midi(vortex: *mut vortex_t) -> c_int {
    let mut rmidi: *mut snd_rawmidi = core::ptr::null_mut();
    let mut temp: c_int;
    let mode: c_int;
    #[allow(unused_variables)]
    let mut mpu: *mut snd_mpu401;
    let mut port: c_ulong = 0;

    #[cfg(not(MPU401_HW_AUREAL))]
    {
        /* EnableHardCodedMPU401Port() */
        /* Enable Legacy MIDI Interface port. */
        port = (0x03 << 5) as c_ulong; /* FIXME: static address. 0x330 */
        temp = (hwread((*vortex).mmio, VORTEX_CTRL) & !CTRL_MIDI_PORT)
            | CTRL_MIDI_EN
            | port as c_int;
        hwwrite((*vortex).mmio, VORTEX_CTRL, temp);
    }
    #[cfg(MPU401_HW_AUREAL)]
    {
        /* Disable Legacy MIDI Interface port. */
        temp = (hwread((*vortex).mmio, VORTEX_CTRL) & !CTRL_MIDI_PORT) & !CTRL_MIDI_EN;
        hwwrite((*vortex).mmio, VORTEX_CTRL, temp);
    }

    /* Mpu401UartInit() */
    mode = 1;
    temp = hwread((*vortex).mmio, VORTEX_CTRL2) & 0xffff00cfu32 as c_int;
    temp |= (MIDI_CLOCK_DIV << 8) | (((mode >> 24) & 0xff) << 4);
    hwwrite((*vortex).mmio, VORTEX_CTRL2, temp);
    hwwrite((*vortex).mmio, VORTEX_MIDI_CMD, MPU401_RESET);

    /* Check if anything is OK. */
    temp = hwread((*vortex).mmio, VORTEX_MIDI_DATA);
    if temp != MPU401_ACK {
        /* 0xfe */
        dev_err(
            (*(*vortex).card).dev,
            b"midi port doesn't acknowledge!\n\0".as_ptr() as *const c_char,
        );
        return -ENODEV;
    }

    /* Enable MPU401 interrupts. */
    hwwrite(
        (*vortex).mmio,
        VORTEX_IRQ_CTRL,
        hwread((*vortex).mmio, VORTEX_IRQ_CTRL) | IRQ_MIDI,
    );

    /* Create MPU401 instance. */
    #[cfg(not(MPU401_HW_AUREAL))]
    {
        temp = snd_mpu401_uart_new(
            (*vortex).card,
            0,
            MPU401_HW_MPU401,
            0x330,
            MPU401_INFO_IRQ_HOOK,
            -1,
            &mut rmidi,
        );
        if temp != 0 {
            hwwrite(
                (*vortex).mmio,
                VORTEX_CTRL,
                (hwread((*vortex).mmio, VORTEX_CTRL) & !CTRL_MIDI_PORT) & !CTRL_MIDI_EN,
            );
            return temp;
        }
    }
    #[cfg(MPU401_HW_AUREAL)]
    {
        port = ((*vortex).mmio as usize + VORTEX_MIDI_DATA as usize) as c_ulong;
        temp = snd_mpu401_uart_new(
            (*vortex).card,
            0,
            MPU401_HW_AUREAL,
            port,
            MPU401_INFO_INTEGRATED | MPU401_INFO_MMIO | MPU401_INFO_IRQ_HOOK,
            -1,
            &mut rmidi,
        );
        if temp != 0 {
            hwwrite(
                (*vortex).mmio,
                VORTEX_CTRL,
                (hwread((*vortex).mmio, VORTEX_CTRL) & !CTRL_MIDI_PORT) & !CTRL_MIDI_EN,
            );
            return temp;
        }
        mpu = (*rmidi).private_data as *mut snd_mpu401;
        (*mpu).cport = ((*vortex).mmio as usize + VORTEX_MIDI_CMD as usize) as c_ulong;
    }

    /* Overwrite MIDI name */
    snprintf(
        (*rmidi).name.as_mut_ptr(),
        core::mem::size_of_val(&(*rmidi).name),
        b"%s MIDI %d\0".as_ptr() as *const c_char,
        CARD_NAME_SHORT,
        (*(*vortex).card).number,
    );

    (*vortex).rmidi = rmidi;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
