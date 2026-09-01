// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Dummy soundcard for virtual rawmidi devices
 *
 *  Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>
 */

/*
 * VIRTUAL RAW MIDI DEVICE CARDS
 *
 * This dummy card contains up to 4 virtual rawmidi devices.
 * They are not real rawmidi devices but just associated with sequencer
 * clients, so that any input/output sources can be connected as a raw
 * MIDI device arbitrary.
 * Also, multiple access is allowed to a single rawmidi device.
 *
 * Typical usage is like following:
 * - Load snd-virmidi module.
 *	# modprobe snd-virmidi index=2
 *   Then, sequencer clients 72:0 to 75:0 will be created, which are
 *   mapped from /dev/snd/midiC1D0 to /dev/snd/midiC1D3, respectively.
 *
 * - Connect input/output via aconnect.
 *	% aconnect 64:0 72:0	# keyboard input redirection 64:0 -> 72:0
 *	% aconnect 72:0 65:0	# output device redirection 72:0 -> 65:0
 *
 * - Run application using a midi device (eg. /dev/snd/midiC1D0)
 */

/*
 * C dependencies removed from executable Rust:
 * linux/init.h, linux/wait.h, linux/err.h, linux/platform_device.h,
 * linux/module.h, sound/core.h, sound/seq_kernel.h, sound/seq_virmidi.h,
 * sound/initval.h.
 *
 * Module metadata and module_param_array declarations are preserved as comments
 * because their Rust mapping is supplied by the surrounding kernel/module build.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

// MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
// MODULE_DESCRIPTION("Dummy soundcard for virtual rawmidi devices");
// MODULE_LICENSE("GPL");

const MAX_MIDI_DEVICES: usize = 4;
const SND_VIRMIDI_DRIVER: &[u8] = b"snd_virmidi\0";

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub id: c_int,
    pub dev: device,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_rawmidi {
    pub name: [c_char; 80],
}

#[repr(C)]
pub struct snd_card_virmidi {
    pub card: *mut snd_card,
    pub midi: [*mut snd_rawmidi; MAX_MIDI_DEVICES],
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;

    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_virmidi_new(
        card: *mut snd_card,
        device: c_int,
        rmidi: *mut *mut snd_rawmidi,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut c_void,
        num: c_uint,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);

    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
}

// Supplied by sound/initval.h in C; kept as external build-time constants here.
const SNDRV_CARDS: usize = 8;
const ENODEV: c_int = 19;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = {
    let mut values = [false; SNDRV_CARDS];
    values[0] = true;
    values
};
static mut midi_devs: [c_int; SNDRV_CARDS] = [4; SNDRV_CARDS];

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for virmidi soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for virmidi soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable this soundcard.");
// module_param_array(midi_devs, int, NULL, 0444);
// MODULE_PARM_DESC(midi_devs, "MIDI devices # (1-4)");

static mut devices: [*mut platform_device; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];

unsafe extern "C" fn snd_virmidi_probe(devptr: *mut platform_device) -> c_int {
    let mut card: *mut snd_card = ptr::null_mut();
    let mut idx: c_int;
    let mut err: c_int;
    let mut dev: c_int = (*devptr).id;

    if dev < 0 || dev >= SNDRV_CARDS as c_int {
        dev_warn(
            &mut (*devptr).dev,
            b"Invalid card index %d, using default 0\n\0".as_ptr() as *const c_char,
            dev,
        );
        dev = 0;
    }

    err = snd_devm_card_new(
        &mut (*devptr).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        core::mem::size_of::<snd_card_virmidi>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    let vmidi = (*card).private_data as *mut snd_card_virmidi;
    (*vmidi).card = card;

    if midi_devs[dev as usize] > MAX_MIDI_DEVICES as c_int {
        dev_warn(
            &mut (*devptr).dev,
            b"too much midi devices for virmidi %d: force to use %d\n\0".as_ptr()
                as *const c_char,
            dev,
            MAX_MIDI_DEVICES as c_int,
        );
        midi_devs[dev as usize] = MAX_MIDI_DEVICES as c_int;
    }

    idx = 0;
    while idx < midi_devs[dev as usize] {
        let mut rmidi: *mut snd_rawmidi = ptr::null_mut();

        err = snd_virmidi_new(card, idx, &mut rmidi);
        if err < 0 {
            return err;
        }
        (*vmidi).midi[idx as usize] = rmidi;
        strscpy(
            (*rmidi).name.as_mut_ptr(),
            b"Virtual Raw MIDI\0".as_ptr() as *const c_char,
        );

        idx += 1;
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        b"VirMIDI\0".as_ptr() as *const c_char,
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        b"VirMIDI\0".as_ptr() as *const c_char,
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"Virtual MIDI Card %i\0".as_ptr() as *const c_char,
        dev + 1,
    );

    err = snd_card_register(card);
    if err != 0 {
        return err;
    }

    platform_set_drvdata(devptr, card as *mut c_void);
    0
}

static mut snd_virmidi_driver: platform_driver = platform_driver {
    probe: Some(snd_virmidi_probe),
    driver: device_driver {
        name: SND_VIRMIDI_DRIVER.as_ptr() as *const c_char,
    },
};

unsafe extern "C" fn snd_virmidi_unregister_all() {
    let mut i: usize = 0;

    while i < devices.len() {
        platform_device_unregister(devices[i]);
        i += 1;
    }
    platform_driver_unregister(&mut snd_virmidi_driver);
}

unsafe extern "C" fn alsa_card_virmidi_init() -> c_int {
    let mut i: c_int;
    let mut cards: c_int;
    let mut err: c_int;

    err = platform_driver_register(&mut snd_virmidi_driver);
    if err < 0 {
        return err;
    }

    cards = 0;
    i = 0;
    while i < SNDRV_CARDS as c_int {
        let device: *mut platform_device;

        if !enable[i as usize] {
            i += 1;
            continue;
        }
        device = platform_device_register_simple(
            SND_VIRMIDI_DRIVER.as_ptr() as *const c_char,
            i,
            ptr::null_mut(),
            0,
        );
        if IS_ERR(device as *const c_void) {
            i += 1;
            continue;
        }
        if platform_get_drvdata(device).is_null() {
            platform_device_unregister(device);
            i += 1;
            continue;
        }
        devices[i as usize] = device;
        cards += 1;

        i += 1;
    }
    if cards == 0 {
        // #ifdef MODULE
        pr_err(b"Card-VirMIDI soundcard not found or device busy\n\0".as_ptr() as *const c_char);
        // #endif
        snd_virmidi_unregister_all();
        return -ENODEV;
    }
    0
}

unsafe extern "C" fn alsa_card_virmidi_exit() {
    snd_virmidi_unregister_all();
}

// module_init(alsa_card_virmidi_init)
// module_exit(alsa_card_virmidi_exit)

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
