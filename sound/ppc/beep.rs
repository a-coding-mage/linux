// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Beep using pcm
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 */

// C dependencies: linux/io.h, asm/irq.h, linux/init.h, linux/slab.h,
// linux/input.h, linux/pci.h, linux/dma-mapping.h, sound/core.h,
// sound/control.h, and "pmac.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub type dma_addr_t = usize;

#[repr(C)]
pub struct pmac_beep {
    pub running: c_int,     /* boolean */
    pub volume: c_int,      /* mixer volume: 0-100 */
    pub volume_play: c_int, /* currently playing volume */
    pub hz: c_int,
    pub nsamples: c_int,
    pub buf: *mut i16,      /* allocated wave buffer */
    pub addr: dma_addr_t,   /* physical address of buffer */
    pub dev: *mut input_dev,
}

#[repr(C)]
pub struct snd_pmac {
    pub beep: *mut pmac_beep,
    pub playback: pmac_stream,
    pub capture: pmac_stream,
    pub freq_table: *mut c_int,
    pub reg_lock: c_void,
    pub pdev: *mut pci_dev,
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct pmac_stream {
    pub running: c_int,
}

#[repr(C)]
pub struct input_dev_id {
    pub bustype: c_uint,
    pub vendor: c_uint,
    pub product: c_uint,
    pub version: c_uint,
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct input_dev {
    pub name: *const c_char,
    pub phys: *const c_char,
    pub id: input_dev_id,
    pub evbit: [usize; 1],
    pub sndbit: [usize; 1],
    pub event: Option<unsafe extern "C" fn(*mut input_dev, c_uint, c_uint, c_int) -> c_int>,
    pub dev: device,
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

pub const EV_SND: c_uint = 0x12;
pub const SND_BELL: c_uint = 0x01;
pub const SND_TONE: c_uint = 0x02;
pub const BUS_ADB: c_uint = 0x17;
pub const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
pub const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
pub const GFP_KERNEL: c_uint = 0;
pub const ENXIO: c_int = 6;
pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;

pub const BEEP_SRATE: c_int = 22050; /* 22050 Hz sample rate */
pub const BEEP_BUFLEN: c_int = 512;
pub const BEEP_VOLUME: c_int = 15; /* 0 - 100 */

#[inline]
pub const fn BIT_MASK(nr: c_uint) -> usize {
    1usize << nr
}

extern "C" {
    fn snd_pmac_beep_dma_stop(chip: *mut snd_pmac);
    fn snd_pmac_beep_dma_start(
        chip: *mut snd_pmac,
        bytes: c_int,
        addr: dma_addr_t,
        speed: c_int,
    );
    fn snd_pmac_rate_index(chip: *mut snd_pmac, rec: *mut pmac_stream, rate: c_int) -> c_int;
    fn input_get_drvdata(dev: *mut input_dev) -> *mut c_void;
    fn input_set_drvdata(dev: *mut input_dev, data: *mut c_void);
    fn input_allocate_device() -> *mut input_dev;
    fn input_free_device(dev: *mut input_dev);
    fn input_register_device(dev: *mut input_dev) -> c_int;
    fn input_unregister_device(dev: *mut input_dev);
    fn dma_alloc_coherent(
        dev: *mut device,
        size: usize,
        dma_handle: *mut dma_addr_t,
        flag: c_uint,
    ) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: usize, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn kfree(ptr: *mut c_void);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pmac;
    fn snd_BUG_ON(cond: bool) -> bool;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol);
}

/*
 * stop beep if running
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_beep_stop(chip: *mut snd_pmac) {
    let beep = (*chip).beep;
    if !beep.is_null() && (*beep).running != 0 {
        (*beep).running = 0;
        snd_pmac_beep_dma_stop(chip);
    }
}

/*
 * Stuff for outputting a beep.  The values range from -327 to +327
 * so we can multiply by an amplitude in the range 0..100 to get a
 * signed short value to put in the output buffer.
 */
static beep_wform: [i16; 256] = [
    0, 40, 79, 117, 153, 187, 218, 245, 269, 288, 304, 316, 323, 327, 327, 324, 318, 310,
    299, 288, 275, 262, 249, 236, 224, 213, 204, 196, 190, 186, 183, 182, 182, 183, 186,
    189, 192, 196, 200, 203, 206, 208, 209, 209, 209, 207, 204, 201, 197, 193, 188, 183,
    179, 174, 170, 166, 163, 161, 160, 159, 159, 160, 161, 162, 164, 166, 168, 169, 171,
    171, 171, 170, 169, 167, 163, 159, 155, 150, 144, 139, 133, 128, 122, 117, 113, 110,
    107, 105, 103, 103, 103, 103, 104, 104, 105, 105, 105, 103, 101, 97, 92, 86, 78, 68,
    58, 45, 32, 18, 3, -11, -26, -41, -55, -68, -79, -88, -95, -100, -102, -102, -99,
    -93, -85, -75, -62, -48, -33, -16, 0, 16, 33, 48, 62, 75, 85, 93, 99, 102, 102,
    100, 95, 88, 79, 68, 55, 41, 26, 11, -3, -18, -32, -45, -58, -68, -78, -86, -92,
    -97, -101, -103, -105, -105, -105, -104, -104, -103, -103, -103, -103, -105, -107,
    -110, -113, -117, -122, -128, -133, -139, -144, -150, -155, -159, -163, -167, -169,
    -170, -171, -171, -171, -169, -168, -166, -164, -162, -161, -160, -159, -159, -160,
    -161, -163, -166, -170, -174, -179, -183, -188, -193, -197, -201, -204, -207, -209,
    -209, -209, -208, -206, -203, -200, -196, -192, -189, -186, -183, -182, -182, -183,
    -186, -190, -196, -204, -213, -224, -236, -249, -262, -275, -288, -299, -310, -318,
    -324, -327, -327, -323, -316, -304, -288, -269, -245, -218, -187, -153, -117, -79,
    -40,
];

unsafe extern "C" fn snd_pmac_beep_event(
    dev: *mut input_dev,
    type_: c_uint,
    code: c_uint,
    mut hz: c_int,
) -> c_int {
    let chip: *mut snd_pmac;
    let beep: *mut pmac_beep;
    let beep_speed: c_int;
    let srate: c_int;
    let mut period: c_int;
    let mut ncycles: c_int;
    let mut nsamples: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let f: c_int;
    let mut p: *mut i16;

    if type_ != EV_SND {
        return -1;
    }

    match code {
        SND_BELL => {
            if hz != 0 {
                hz = 1000;
            }
        }
        SND_TONE => {}
        _ => return -1,
    }

    chip = input_get_drvdata(dev) as *mut snd_pmac;
    if chip.is_null() {
        return -1;
    }
    beep = (*chip).beep;
    if beep.is_null() {
        return -1;
    }

    if hz == 0 {
        // C used guard(spinlock_irqsave)(&chip->reg_lock) for this critical section.
        if (*beep).running != 0 {
            snd_pmac_beep_stop(chip);
        }
        return 0;
    }

    beep_speed = snd_pmac_rate_index(chip, &mut (*chip).playback, BEEP_SRATE);
    srate = *(*chip).freq_table.add(beep_speed as usize);

    if hz <= srate / BEEP_BUFLEN || hz > srate / 2 {
        hz = 1000;
    }

    // C used scoped_guard(spinlock_irqsave, &chip->reg_lock) for this critical section.
    if (*chip).playback.running != 0 || (*chip).capture.running != 0 || (*beep).running != 0 {
        return 0;
    }
    (*beep).running = 1;

    if hz == (*beep).hz && (*beep).volume == (*beep).volume_play {
        nsamples = (*beep).nsamples;
    } else {
        period = srate * 256 / hz; /* fixed point */
        ncycles = BEEP_BUFLEN * 256 / period;
        nsamples = (period * ncycles) >> 8;
        f = ncycles * 65536 / nsamples;
        j = 0;
        p = (*beep).buf;
        i = 0;
        while i < nsamples {
            let sample = beep_wform[(j >> 8) as usize] as c_int * (*beep).volume;
            *p.add(0) = sample as i16;
            *p.add(1) = sample as i16;
            j = (j + f) & 0xffff;
            i += 1;
            p = p.add(2);
        }
        (*beep).hz = hz;
        (*beep).volume_play = (*beep).volume;
        (*beep).nsamples = nsamples;
    }

    // C used guard(spinlock_irqsave)(&chip->reg_lock) before starting DMA.
    snd_pmac_beep_dma_start(chip, (*beep).nsamples * 4, (*beep).addr, beep_speed);
    0
}

/*
 * beep volume mixer
 */

unsafe extern "C" fn snd_pmac_info_beep(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 100;
    0
}

unsafe extern "C" fn snd_pmac_get_beep(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    if snd_BUG_ON((*chip).beep.is_null()) {
        return -ENXIO;
    }
    (*ucontrol).value.integer.value[0] = (*(*chip).beep).volume as i64;
    0
}

unsafe extern "C" fn snd_pmac_put_beep(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip = snd_kcontrol_chip(kcontrol);
    let oval: c_uint;
    let nval: c_uint;
    if snd_BUG_ON((*chip).beep.is_null()) {
        return -ENXIO;
    }
    oval = (*(*chip).beep).volume as c_uint;
    nval = (*ucontrol).value.integer.value[0] as c_uint;
    if nval > 100 {
        return -EINVAL;
    }
    (*(*chip).beep).volume = nval as c_int;
    (oval != (*(*chip).beep).volume as c_uint) as c_int
}

static snd_pmac_beep_mixer_name: &[u8; 21] = b"Beep Playback Volume\0";

static snd_pmac_beep_mixer: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    name: snd_pmac_beep_mixer_name.as_ptr() as *const c_char,
    info: Some(snd_pmac_info_beep),
    get: Some(snd_pmac_get_beep),
    put: Some(snd_pmac_put_beep),
};

/* Initialize beep stuff */
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_attach_beep(chip: *mut snd_pmac) -> c_int {
    let beep: *mut pmac_beep;
    let input_dev: *mut input_dev;
    let beep_ctl: *mut snd_kcontrol;
    let dmabuf: *mut c_void;
    let mut err: c_int = -ENOMEM;

    beep = kzalloc_obj_pmac_beep();
    if beep.is_null() {
        return -ENOMEM;
    }
    dmabuf = dma_alloc_coherent(
        &mut (*(*chip).pdev).dev,
        (BEEP_BUFLEN * 4) as usize,
        &mut (*beep).addr,
        GFP_KERNEL,
    );
    input_dev = input_allocate_device();
    if dmabuf.is_null() || input_dev.is_null() {
        input_free_device(input_dev);
        if !dmabuf.is_null() {
            dma_free_coherent(
                &mut (*(*chip).pdev).dev,
                (BEEP_BUFLEN * 4) as usize,
                dmabuf,
                (*beep).addr,
            );
        }
        kfree(beep as *mut c_void);
        return err;
    }

    /* FIXME: set more better values */
    (*input_dev).name = c"PowerMac Beep".as_ptr();
    (*input_dev).phys = c"powermac/beep".as_ptr();
    (*input_dev).id.bustype = BUS_ADB;
    (*input_dev).id.vendor = 0x001f;
    (*input_dev).id.product = 0x0001;
    (*input_dev).id.version = 0x0100;

    (*input_dev).evbit[0] = BIT_MASK(EV_SND);
    (*input_dev).sndbit[0] = BIT_MASK(SND_BELL) | BIT_MASK(SND_TONE);
    (*input_dev).event = Some(snd_pmac_beep_event);
    (*input_dev).dev.parent = &mut (*(*chip).pdev).dev;
    input_set_drvdata(input_dev, chip as *mut c_void);

    (*beep).dev = input_dev;
    (*beep).buf = dmabuf as *mut i16;
    (*beep).volume = BEEP_VOLUME;
    (*beep).running = 0;

    beep_ctl = snd_ctl_new1(&snd_pmac_beep_mixer, chip as *mut c_void);
    err = snd_ctl_add((*chip).card, beep_ctl);
    if err < 0 {
        input_free_device(input_dev);
        if !dmabuf.is_null() {
            dma_free_coherent(
                &mut (*(*chip).pdev).dev,
                (BEEP_BUFLEN * 4) as usize,
                dmabuf,
                (*beep).addr,
            );
        }
        kfree(beep as *mut c_void);
        return err;
    }

    (*chip).beep = beep;

    err = input_register_device((*beep).dev);
    if err != 0 {
        snd_ctl_remove((*chip).card, beep_ctl);
        input_free_device(input_dev);
        if !dmabuf.is_null() {
            dma_free_coherent(
                &mut (*(*chip).pdev).dev,
                (BEEP_BUFLEN * 4) as usize,
                dmabuf,
                (*beep).addr,
            );
        }
        kfree(beep as *mut c_void);
        return err;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_pmac_detach_beep(chip: *mut snd_pmac) {
    if !(*chip).beep.is_null() {
        input_unregister_device((*(*chip).beep).dev);
        dma_free_coherent(
            &mut (*(*chip).pdev).dev,
            (BEEP_BUFLEN * 4) as usize,
            (*(*chip).beep).buf as *mut c_void,
            (*(*chip).beep).addr,
        );
        kfree((*chip).beep as *mut c_void);
        (*chip).beep = ptr::null_mut();
    }
}

unsafe fn kzalloc_obj_pmac_beep() -> *mut pmac_beep {
    extern "C" {
        fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    }

    kzalloc(core::mem::size_of::<pmac_beep>(), GFP_KERNEL) as *mut pmac_beep
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
