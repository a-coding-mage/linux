// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  The driver for the EMU10K1 (SB Live!) based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   James Courtier-Dutton <James@superbug.co.uk>
 */

// C includes translated as external dependencies:
// <linux/init.h>, <linux/pci.h>, <linux/string.h>, <linux/time.h>,
// <linux/module.h>, <sound/core.h>, <sound/emu10k1.h>, <sound/initval.h>

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("EMU10K1");
// MODULE_LICENSE("GPL");

// #if IS_ENABLED(CONFIG_SND_SEQUENCER)
// #define ENABLE_SYNTH
// #include <sound/emu10k1_synth.h>
// #endif

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SNDRV_CARDS: usize = 32;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const SNDRV_DMA_TYPE_DEV: c_int = 0;
const SNDRV_CTL_POWER_D3hot: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const SNDRV_SEQ_DEV_ID_EMU10K1_SYNTH: *const c_char = b"emu10k1-synth\0".as_ptr() as *const c_char;
const KBUILD_MODNAME: *const c_char = b"emu10k1\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: c_uint,
    pub device: c_uint,
    pub subvendor: c_uint,
    pub subdevice: c_uint,
    pub class: c_uint,
    pub class_mask: c_uint,
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
    pub private_data: *mut c_void,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct snd_emu10k1_card_capabilities {
    pub driver: *const c_char,
    pub name: *const c_char,
    pub ac97_chip: bool,
    pub ca0151_chip: bool,
}

#[repr(C)]
pub struct snd_emu1010 {
    pub work: c_void,
}

#[repr(C)]
pub struct snd_emu10k1 {
    pub card: *mut snd_card,
    pub card_capabilities: *mut snd_emu10k1_card_capabilities,
    pub p16v_buffer: *mut c_void,
    pub audigy: c_int,
    pub revision: c_int,
    pub serial: c_uint,
    pub port: c_ulong,
    pub irq: c_int,
    pub suspend: c_int,
    pub emu1010: snd_emu1010,
    pub ac97: *mut c_void,
}

type c_ulong = u64;

#[repr(C)]
pub struct snd_seq_device {
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct snd_emu10k1_synth_arg {
    pub hwptr: *mut snd_emu10k1,
    pub index: c_int,
    pub seq_ports: c_int,
    pub max_voices: c_int,
}

unsafe extern "C" {
    static THIS_MODULE: *mut c_void;
    static snd_emu10k1_pm: dev_pm_ops;

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_emu10k1_create(
        card: *mut snd_card,
        pci: *mut pci_dev,
        extin: c_int,
        extout: c_int,
        max_cache_bytes: c_long,
        enable_ir: bool,
        subsystem: c_uint,
    ) -> c_int;
    fn snd_emu10k1_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_emu10k1_pcm_mic(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_emu10k1_pcm_efx(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_devm_alloc_pages(dev: *mut device, dma_type: c_int, size: usize) -> *mut c_void;
    fn snd_emu10k1_mixer(emu: *mut snd_emu10k1, pcm_device: c_int, multi_device: c_int) -> c_int;
    fn snd_emu10k1_timer(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_emu10k1_pcm_multi(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_p16v_pcm(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_emu10k1_audigy_midi(emu: *mut snd_emu10k1) -> c_int;
    fn snd_emu10k1_midi(emu: *mut snd_emu10k1) -> c_int;
    fn snd_emu10k1_fx8010_new(emu: *mut snd_emu10k1, device: c_int) -> c_int;
    fn snd_seq_device_new(
        card: *mut snd_card,
        device: c_int,
        id: *const c_char,
        argsize: usize,
        result: *mut *mut snd_seq_device,
    ) -> c_int;
    fn SNDRV_SEQ_DEVICE_ARGPTR(device: *mut snd_seq_device) -> *mut snd_emu10k1_synth_arg;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn cancel_work_sync(work: *mut c_void) -> bool;
    fn snd_ac97_suspend(ac97: *mut c_void);
    fn snd_emu10k1_efx_suspend(emu: *mut snd_emu10k1);
    fn snd_emu10k1_suspend_regs(emu: *mut snd_emu10k1);
    fn snd_p16v_suspend(emu: *mut snd_emu10k1);
    fn snd_emu10k1_done(emu: *mut snd_emu10k1);
    fn snd_emu10k1_resume_init(emu: *mut snd_emu10k1);
    fn snd_emu10k1_efx_resume(emu: *mut snd_emu10k1);
    fn snd_ac97_resume(ac97: *mut c_void);
    fn snd_emu10k1_resume_regs(emu: *mut snd_emu10k1);
    fn snd_p16v_resume(emu: *mut snd_emu10k1);
    fn module_pci_driver(driver: *mut pci_driver);
}

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut extin: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut extout: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut seq_ports: [c_int; SNDRV_CARDS] = [4; SNDRV_CARDS];
static mut max_synth_voices: [c_int; SNDRV_CARDS] = [64; SNDRV_CARDS];
static mut max_buffer_size: [c_int; SNDRV_CARDS] = [128; SNDRV_CARDS];
static mut enable_ir: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
static mut subsystem: [c_uint; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* Force card subsystem model */

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for the EMU10K1 soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for the EMU10K1 soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable the EMU10K1 soundcard.");
// module_param_array(extin, int, NULL, 0444);
// MODULE_PARM_DESC(extin, "Available external inputs for FX8010. Zero=default.");
// module_param_array(extout, int, NULL, 0444);
// MODULE_PARM_DESC(extout, "Available external outputs for FX8010. Zero=default.");
// module_param_array(seq_ports, int, NULL, 0444);
// MODULE_PARM_DESC(seq_ports, "Allocated sequencer ports for internal synthesizer.");
// module_param_array(max_synth_voices, int, NULL, 0444);
// MODULE_PARM_DESC(max_synth_voices, "Maximum number of voices for WaveTable.");
// module_param_array(max_buffer_size, int, NULL, 0444);
// MODULE_PARM_DESC(max_buffer_size, "Maximum sample buffer size in MB.");
// module_param_array(enable_ir, bool, NULL, 0444);
// MODULE_PARM_DESC(enable_ir, "Enable IR.");
// module_param_array(subsystem, uint, NULL, 0444);
// MODULE_PARM_DESC(subsystem, "Force card subsystem model.");
/*
 * Class 0401: 1102:0008 (rev 00) Subsystem: 1102:1001 -> Audigy2 Value  Model:SB0400
 */
static snd_emu10k1_ids: [pci_device_id; 4] = [
    pci_device_id {
        vendor: 0x1102,
        device: 0x0002,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    }, /* EMU10K1 */
    pci_device_id {
        vendor: 0x1102,
        device: 0x0004,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 1,
    }, /* Audigy */
    pci_device_id {
        vendor: 0x1102,
        device: 0x0008,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 1,
    }, /* Audigy 2 Value SB0400 */
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(pci, snd_emu10k1_ids);

unsafe extern "C" fn snd_card_emu10k1_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut emu: *mut snd_emu10k1;
    // #ifdef ENABLE_SYNTH
    let mut wave: *mut snd_seq_device = ptr::null_mut();
    // #endif
    let mut err: c_int;

    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }

    err = snd_devm_card_new(
        &mut (*pci).dev,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        size_of::<snd_emu10k1>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    emu = (*card).private_data as *mut snd_emu10k1;

    if max_buffer_size[dev as usize] < 32 {
        max_buffer_size[dev as usize] = 32;
    } else if max_buffer_size[dev as usize] > 1024 {
        max_buffer_size[dev as usize] = 1024;
    }
    err = snd_emu10k1_create(
        card,
        pci,
        extin[dev as usize],
        extout[dev as usize],
        (max_buffer_size[dev as usize] as c_long) * 1024 * 1024,
        enable_ir[dev as usize],
        subsystem[dev as usize],
    );
    if err < 0 {
        return err;
    }
    err = snd_emu10k1_pcm(emu, 0);
    if err < 0 {
        return err;
    }
    if (*(*emu).card_capabilities).ac97_chip {
        err = snd_emu10k1_pcm_mic(emu, 1);
        if err < 0 {
            return err;
        }
    }
    err = snd_emu10k1_pcm_efx(emu, 2);
    if err < 0 {
        return err;
    }
    /* This stores the periods table. */
    if (*(*emu).card_capabilities).ca0151_chip {
        /* P16V */
        (*emu).p16v_buffer = snd_devm_alloc_pages(&mut (*pci).dev, SNDRV_DMA_TYPE_DEV, 1024);
        if (*emu).p16v_buffer.is_null() {
            return -ENOMEM;
        }
    }

    err = snd_emu10k1_mixer(emu, 0, 3);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1_timer(emu, 0);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1_pcm_multi(emu, 3);
    if err < 0 {
        return err;
    }
    if (*(*emu).card_capabilities).ca0151_chip {
        /* P16V */
        err = snd_p16v_pcm(emu, 4);
        if err < 0 {
            return err;
        }
    }
    if (*emu).audigy != 0 {
        err = snd_emu10k1_audigy_midi(emu);
        if err < 0 {
            return err;
        }
    } else {
        err = snd_emu10k1_midi(emu);
        if err < 0 {
            return err;
        }
    }
    err = snd_emu10k1_fx8010_new(emu, 0);
    if err < 0 {
        return err;
    }
    // #ifdef ENABLE_SYNTH
    if snd_seq_device_new(
        card,
        1,
        SNDRV_SEQ_DEV_ID_EMU10K1_SYNTH,
        size_of::<snd_emu10k1_synth_arg>(),
        &mut wave,
    ) < 0
        || wave.is_null()
    {
        dev_warn(
            (*(*emu).card).dev,
            b"can't initialize Emu10k1 wavetable synth\n\0".as_ptr() as *const c_char,
        );
    } else {
        let arg: *mut snd_emu10k1_synth_arg;
        arg = SNDRV_SEQ_DEVICE_ARGPTR(wave);
        strscpy((*wave).name.as_mut_ptr(), b"Emu-10k1 Synth\0".as_ptr() as *const c_char, (*wave).name.len());
        (*arg).hwptr = emu;
        (*arg).index = 1;
        (*arg).seq_ports = seq_ports[dev as usize];
        (*arg).max_voices = max_synth_voices[dev as usize];
    }
    // #endif

    strscpy(
        (*card).driver.as_mut_ptr(),
        (*(*emu).card_capabilities).driver,
        (*card).driver.len(),
    );
    strscpy(
        (*card).shortname.as_mut_ptr(),
        (*(*emu).card_capabilities).name,
        (*card).shortname.len(),
    );
    snprintf(
        (*card).longname.as_mut_ptr(),
        (*card).longname.len(),
        b"%s (rev.%d, serial:0x%x) at 0x%lx, irq %i\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        (*emu).revision,
        (*emu).serial,
        (*emu).port,
        (*emu).irq,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

// #ifdef CONFIG_PM_SLEEP
unsafe extern "C" fn snd_emu10k1_suspend(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let emu: *mut snd_emu10k1 = (*card).private_data as *mut snd_emu10k1;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);

    (*emu).suspend = 1;

    cancel_work_sync(&mut (*emu).emu1010.work);

    snd_ac97_suspend((*emu).ac97);

    snd_emu10k1_efx_suspend(emu);
    snd_emu10k1_suspend_regs(emu);
    if (*(*emu).card_capabilities).ca0151_chip {
        snd_p16v_suspend(emu);
    }

    snd_emu10k1_done(emu);
    0
}

unsafe extern "C" fn snd_emu10k1_resume(dev: *mut device) -> c_int {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let emu: *mut snd_emu10k1 = (*card).private_data as *mut snd_emu10k1;

    snd_emu10k1_resume_init(emu);
    snd_emu10k1_efx_resume(emu);
    snd_ac97_resume((*emu).ac97);
    snd_emu10k1_resume_regs(emu);

    if (*(*emu).card_capabilities).ca0151_chip {
        snd_p16v_resume(emu);
    }

    (*emu).suspend = 0;

    snd_power_change_state(card, SNDRV_CTL_POWER_D0);

    0
}

// static SIMPLE_DEV_PM_OPS(snd_emu10k1_pm, snd_emu10k1_suspend, snd_emu10k1_resume);
// #define SND_EMU10K1_PM_OPS &snd_emu10k1_pm
const SND_EMU10K1_PM_OPS: *const dev_pm_ops = unsafe { &snd_emu10k1_pm };
// #else
// #define SND_EMU10K1_PM_OPS NULL
// #endif /* CONFIG_PM_SLEEP */

static mut emu10k1_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_emu10k1_ids.as_ptr(),
    probe: Some(snd_card_emu10k1_probe),
    driver: device_driver {
        pm: SND_EMU10K1_PM_OPS,
    },
};

#[allow(non_snake_case)]
unsafe fn __module_pci_driver_emu10k1_driver() {
    module_pci_driver(&mut emu10k1_driver);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
