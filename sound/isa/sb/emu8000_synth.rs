// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *     and (c) 1999 Steve Ratcliffe <steve@parabola.demon.co.uk>
 *  Copyright (C) 1999-2000 Takashi Iwai <tiwai@suse.de>
 *
 *  Emu8000 synth plug-in routine
 */

/* Dependencies from: emu8000_local.h, linux/init.h, linux/module.h,
 * sound/initval.h
 */

/* MODULE_AUTHOR("Takashi Iwai, Steve Ratcliffe"); */
/* MODULE_DESCRIPTION("Emu8000 synth plug-in routine"); */
/* MODULE_LICENSE("GPL"); */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const EINVAL: c_int = 22;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;

extern "C" {
    static KBUILD_MODNAME: *const c_char;
    static SNDRV_SEQ_DEV_ID_EMU8000: c_int;
    static EMU8000_DRAM_VOICES: c_int;

    fn SNDRV_SEQ_DEVICE_ARGPTR(dev: *mut snd_seq_device) -> *mut c_void;

    fn snd_emux_new(remu: *mut *mut snd_emux) -> c_int;
    fn snd_emux_free(emu: *mut snd_emux);
    fn snd_emux_register(
        emu: *mut snd_emux,
        card: *mut snd_card,
        index: c_int,
        name: *const c_char,
    ) -> c_int;
    fn snd_emu8000_ops_setup(hw: *mut snd_emu8000);
    fn snd_util_memhdr_free(hdr: *mut snd_util_memhdr);
    fn snd_util_memhdr_new(size: c_int) -> *mut snd_util_memhdr;
    fn snd_emu8000_pcm_new(card: *mut snd_card, hw: *mut snd_emu8000, device: c_int) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_util_memhdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_emu8000 {
    pub emu: *mut snd_emux,
    pub seq_ports: c_int,
    pub memhdr: *mut snd_util_memhdr,
    pub card: *mut snd_card,
    pub mem_size: c_int,
    pub index: c_int,
    pub pcm: *mut c_void,
}

#[repr(C)]
pub struct snd_emux {
    pub hw: *mut snd_emu8000,
    pub max_voices: c_int,
    pub num_ports: c_int,
    pub memhdr: *mut snd_util_memhdr,
    pub midi_ports: c_int,
    pub midi_devidx: c_int,
    pub linear_panning: c_int,
    pub hwdep_idx: c_int,
}

#[repr(C)]
pub struct snd_seq_device {
    pub card: *mut snd_card,
    pub driver_data: *mut c_void,
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

/*----------------------------------------------------------------*/

/*
 * create a new hardware dependent device for Emu8000
 */
unsafe extern "C" fn snd_emu8000_probe(dev: *mut snd_seq_device) -> c_int {
    let hw: *mut snd_emu8000;
    let mut emu: *mut snd_emux = ptr::null_mut();

    hw = *(SNDRV_SEQ_DEVICE_ARGPTR(dev) as *mut *mut snd_emu8000);
    if hw.is_null() {
        return -EINVAL;
    }

    if !(*hw).emu.is_null() {
        return -EBUSY; /* already exists..? */
    }

    if snd_emux_new(&mut emu) < 0 {
        return -ENOMEM;
    }

    (*hw).emu = emu;
    snd_emu8000_ops_setup(hw);

    (*emu).hw = hw;
    (*emu).max_voices = EMU8000_DRAM_VOICES;
    (*emu).num_ports = (*hw).seq_ports;

    if !(*hw).memhdr.is_null() {
        dev_err(
            (*(*hw).card).dev,
            b"memhdr is already initialized!?\n\0".as_ptr() as *const c_char,
        );
        snd_util_memhdr_free((*hw).memhdr);
    }
    (*hw).memhdr = snd_util_memhdr_new((*hw).mem_size);
    if (*hw).memhdr.is_null() {
        snd_emux_free(emu);
        (*hw).emu = ptr::null_mut();
        return -ENOMEM;
    }

    (*emu).memhdr = (*hw).memhdr;
    (*emu).midi_ports = if (*hw).seq_ports < 2 {
        (*hw).seq_ports
    } else {
        2
    }; /* number of virmidi ports */
    (*emu).midi_devidx = 1;
    (*emu).linear_panning = 1;
    (*emu).hwdep_idx = 2; /* FIXED */

    if snd_emux_register(emu, (*dev).card, (*hw).index, b"Emu8000\0".as_ptr() as *const c_char) < 0 {
        snd_emux_free(emu);
        snd_util_memhdr_free((*hw).memhdr);
        (*hw).emu = ptr::null_mut();
        (*hw).memhdr = ptr::null_mut();
        return -ENOMEM;
    }

    if (*hw).mem_size > 0 {
        snd_emu8000_pcm_new((*dev).card, hw, 1);
    }

    (*dev).driver_data = hw as *mut c_void;

    0
}

/*
 * free all resources
 */
unsafe extern "C" fn snd_emu8000_remove(dev: *mut snd_seq_device) {
    let hw: *mut snd_emu8000;

    if (*dev).driver_data.is_null() {
        return; /* no synth was allocated actually */
    }

    hw = (*dev).driver_data as *mut snd_emu8000;
    if !(*hw).pcm.is_null() {
        snd_device_free((*dev).card, (*hw).pcm);
    }
    snd_emux_free((*hw).emu);
    snd_util_memhdr_free((*hw).memhdr);
    (*hw).emu = ptr::null_mut();
    (*hw).memhdr = ptr::null_mut();
}

/*
 *  INIT part
 */

static mut emu8000_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_emu8000_probe),
    remove: Some(snd_emu8000_remove),
    driver: snd_seq_driver_driver {
        name: unsafe { KBUILD_MODNAME },
    },
    id: unsafe { SNDRV_SEQ_DEV_ID_EMU8000 },
    argsize: size_of::<*mut snd_emu8000>(),
};

/* module_snd_seq_driver(emu8000_driver); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
