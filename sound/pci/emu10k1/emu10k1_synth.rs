// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Routines for control of EMU10K1 WaveTable synth
 */

// C dependencies:
// #include "emu10k1_synth_local.h"
// #include <linux/init.h>
// #include <linux/module.h>

// MODULE_AUTHOR("Takashi Iwai");
// MODULE_DESCRIPTION("Routines for control of EMU10K1 WaveTable synth");
// MODULE_LICENSE("GPL");

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const SNDRV_SEQ_DEV_ID_EMU10K1_SYNTH: c_int = 0; // provided by translated dependencies

#[repr(C)]
pub struct snd_seq_device {
    pub card: *mut c_void,
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct snd_emu10k1_synth_arg {
    pub hwptr: *mut snd_emu10k1,
    pub seq_ports: c_int,
    pub max_voices: c_int,
    pub index: c_int,
}

#[repr(C)]
pub struct snd_emu10k1 {
    pub voice_lock: c_void,
    pub synth: *mut snd_emux,
    pub get_synth_voice: Option<unsafe extern "C" fn()>,
    pub memhdr: *mut c_void,
    pub audigy: c_int,
}

#[repr(C)]
pub struct snd_emux {
    pub hw: *mut snd_emu10k1,
    pub max_voices: c_int,
    pub num_ports: c_int,
    pub memhdr: *mut c_void,
    pub midi_ports: c_int,
    pub midi_devidx: c_int,
    pub linear_panning: c_int,
    pub hwdep_idx: c_int,
}

#[repr(C)]
pub struct snd_seq_driver_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_seq_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_seq_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_seq_device)>,
    pub driver: snd_seq_driver_driver,
    pub id: c_int,
    pub argsize: usize,
}

unsafe extern "C" {
    fn SNDRV_SEQ_DEVICE_ARGPTR(dev: *mut snd_seq_device) -> *mut snd_emu10k1_synth_arg;
    fn snd_emux_new(emux: *mut *mut snd_emux) -> c_int;
    fn snd_emu10k1_ops_setup(emux: *mut snd_emux);
    fn snd_emux_register(
        emux: *mut snd_emux,
        card: *mut c_void,
        index: c_int,
        name: *const c_char,
    ) -> c_int;
    fn snd_emux_free(emux: *mut snd_emux);
    fn snd_emu10k1_synth_get_voice();

    fn guard_spinlock_irq(lock: *mut c_void) -> SpinlockIrqGuard;
    fn scoped_guard_spinlock_irq(lock: *mut c_void) -> SpinlockIrqGuard;
}

#[repr(C)]
pub struct SpinlockIrqGuard {
    _private: [u8; 0],
}

const KBUILD_MODNAME: *const c_char = b"emu10k1_synth\0".as_ptr() as *const c_char;
const EMU10K1_NAME: *const c_char = b"Emu10k1\0".as_ptr() as *const c_char;

/*
 * create a new hardware dependent device for Emu10k1
 */
unsafe extern "C" fn snd_emu10k1_synth_probe(dev: *mut snd_seq_device) -> c_int {
    let mut emux: *mut snd_emux = ptr::null_mut();
    let hw: *mut snd_emu10k1;
    let arg: *mut snd_emu10k1_synth_arg;

    arg = SNDRV_SEQ_DEVICE_ARGPTR(dev);
    if arg.is_null() {
        return -EINVAL;
    }

    if (*arg).seq_ports <= 0 {
        return 0; /* nothing */
    }
    if (*arg).max_voices < 1 {
        (*arg).max_voices = 1;
    } else if (*arg).max_voices > 64 {
        (*arg).max_voices = 64;
    }

    if snd_emux_new(&mut emux) < 0 {
        return -ENOMEM;
    }

    snd_emu10k1_ops_setup(emux);
    hw = (*arg).hwptr;
    (*emux).hw = hw;
    (*emux).max_voices = (*arg).max_voices;
    (*emux).num_ports = (*arg).seq_ports;
    (*emux).memhdr = (*hw).memhdr;
    /* maximum two ports */
    (*emux).midi_ports = if (*arg).seq_ports < 2 { (*arg).seq_ports } else { 2 };
    /* audigy has two external midis */
    (*emux).midi_devidx = if (*hw).audigy != 0 { 2 } else { 1 };
    (*emux).linear_panning = 0;
    (*emux).hwdep_idx = 2; /* FIXED */

    if snd_emux_register(emux, (*dev).card, (*arg).index, EMU10K1_NAME) < 0 {
        snd_emux_free(emux);
        return -ENOMEM;
    }

    let _guard = guard_spinlock_irq(&mut (*hw).voice_lock);
    (*hw).synth = emux;
    (*hw).get_synth_voice = Some(core::mem::transmute::<
        unsafe extern "C" fn(),
        unsafe extern "C" fn(),
    >(snd_emu10k1_synth_get_voice));

    (*dev).driver_data = emux as *mut c_void;

    0
}

unsafe extern "C" fn snd_emu10k1_synth_remove(dev: *mut snd_seq_device) {
    let emux: *mut snd_emux;
    let hw: *mut snd_emu10k1;

    if (*dev).driver_data.is_null() {
        return; /* not registered actually */
    }

    emux = (*dev).driver_data as *mut snd_emux;

    hw = (*emux).hw;
    {
        let _guard = scoped_guard_spinlock_irq(&mut (*hw).voice_lock);
        (*hw).synth = ptr::null_mut();
        (*hw).get_synth_voice = None;
    }

    snd_emux_free(emux);
}

/*
 *  INIT part
 */

static mut emu10k1_synth_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_emu10k1_synth_probe),
    remove: Some(snd_emu10k1_synth_remove),
    driver: snd_seq_driver_driver {
        name: KBUILD_MODNAME,
    },
    id: SNDRV_SEQ_DEV_ID_EMU10K1_SYNTH,
    argsize: size_of::<snd_emu10k1_synth_arg>(),
};

// module_snd_seq_driver(emu10k1_synth_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
