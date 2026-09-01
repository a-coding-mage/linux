// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for SoundBlaster 1.0/2.0/Pro soundcards and compatible
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

// Translated from Linux kernel C. External kernel/ALSA declarations are kept as
// declarations; include/module/parameter macro intent is preserved in comments.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("Sound Blaster 1.0/2.0/Pro");
// MODULE_LICENSE("GPL");

const DEV_NAME: &[u8] = b"sb8\0";

extern "C" {
    static THIS_MODULE: *mut c_void;

    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE: [bool; SNDRV_CARDS];
    static SNDRV_DEFAULT_PORT: [c_long; SNDRV_CARDS];
    static SNDRV_DEFAULT_IRQ: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_DMA: [c_int; SNDRV_CARDS];

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn devm_request_region(
        dev: *mut device,
        start: c_ulong,
        n: c_ulong,
        name: *const c_char,
    ) -> *mut resource;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;

    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_sbdsp_create(
        card: *mut snd_card,
        port: c_long,
        irq: c_int,
        irq_handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        dma8: c_int,
        dma16: c_int,
        hardware: c_int,
        rchip: *mut *mut snd_sb,
    ) -> c_int;
    fn snd_sb8dsp_interrupt(chip: *mut snd_sb) -> irqreturn_t;
    fn snd_sb8dsp_midi_interrupt(chip: *mut snd_sb) -> irqreturn_t;
    fn snd_sb8dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
    fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
    fn snd_opl3_create(
        card: *mut snd_card,
        l_port: c_ulong,
        r_port: c_ulong,
        hardware: c_int,
        integrated: c_int,
        ropl3: *mut *mut snd_opl3,
    ) -> c_int;
    fn snd_opl3_hwdep_new(
        opl3: *mut snd_opl3,
        device: c_int,
        seq_device: c_int,
        info: *mut c_void,
    ) -> c_int;
    fn snd_sb8dsp_midi(chip: *mut snd_sb, device: c_int) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;

    fn snd_power_change_state(card: *mut snd_card, state: c_int);
    fn snd_sbmixer_suspend(chip: *mut snd_sb);
    fn snd_sbdsp_reset(chip: *mut snd_sb);
    fn snd_sbmixer_resume(chip: *mut snd_sb);
}

const SNDRV_CARDS: usize = 8;
const SNDRV_AUTO_IRQ: c_int = -1;
const SNDRV_AUTO_DMA: c_int = -1;
const SNDRV_AUTO_PORT: c_long = -1;
const SB_OPEN_PCM: c_int = 0x01;
const SB_HW_AUTO: c_int = 0;
const SB_HW_10: c_int = 1;
const SB_HW_20: c_int = 2;
const SB_HW_PRO: c_int = 3;
const SB_HW_16: c_int = 4;
const SB_HW_ALS100: c_int = 5;
const OPL3_HW_AUTO: c_int = 0;
const SNDRV_CTL_POWER_D3HOT: c_int = 3;
const SNDRV_CTL_POWER_D0: c_int = 0;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

type irqreturn_t = c_int;
type pm_message_t = c_int;

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_opl3 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sb {
    open: c_int,
    hardware: c_int,
    port: c_ulong,
    name: *const c_char,
}

#[repr(C)]
pub struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    driver: *mut c_char,
    shortname: *mut c_char,
    longname: *mut c_char,
}

#[repr(C)]
pub struct isa_driver_driver {
    name: *const c_char,
}

#[repr(C)]
pub struct isa_driver {
    match_: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    probe: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    // Present when CONFIG_PM is enabled in the original C build.
    suspend: Option<unsafe extern "C" fn(*mut device, c_uint, pm_message_t) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_int>,
    driver: isa_driver_driver,
}

#[repr(C)]
struct snd_sb8 {
    fm_res: *mut resource, /* used to block FM i/o region for legacy cards */
    chip: *mut snd_sb,
}

static mut index: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IDX }; /* Index 0-MAX */
static mut id: [*mut c_char; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_STR }; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_ENABLE }; /* Enable this card */
static mut port: [c_long; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_PORT }; /* 0x220,0x240,0x260 */
static mut irq: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_IRQ }; /* 5,7,9,10 */
static mut dma8: [c_int; SNDRV_CARDS] = unsafe { SNDRV_DEFAULT_DMA }; /* 1,3 */

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for Sound Blaster soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for Sound Blaster soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable Sound Blaster soundcard.");
// module_param_hw_array(port, long, ioport, NULL, 0444);
// MODULE_PARM_DESC(port, "Port # for SB8 driver.");
// module_param_hw_array(irq, int, irq, NULL, 0444);
// MODULE_PARM_DESC(irq, "IRQ # for SB8 driver.");
// module_param_hw_array(dma8, int, dma, NULL, 0444);
// MODULE_PARM_DESC(dma8, "8-bit DMA # for SB8 driver.");

unsafe extern "C" fn snd_sb8_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip = dev_id as *mut snd_sb;

    if ((*chip).open & SB_OPEN_PCM) != 0 {
        return snd_sb8dsp_interrupt(chip);
    } else {
        return snd_sb8dsp_midi_interrupt(chip);
    }
}

unsafe extern "C" fn snd_sb8_match(pdev: *mut device, dev: c_uint) -> c_int {
    let dev = dev as usize;

    if !enable[dev] {
        return 0;
    }
    if irq[dev] == SNDRV_AUTO_IRQ {
        dev_err(pdev, c"please specify irq\n".as_ptr());
        return 0;
    }
    if dma8[dev] == SNDRV_AUTO_DMA {
        dev_err(pdev, c"please specify dma8\n".as_ptr());
        return 0;
    }
    1
}

unsafe extern "C" fn snd_sb8_probe(pdev: *mut device, dev: c_uint) -> c_int {
    let dev = dev as usize;
    let mut chip: *mut snd_sb = core::ptr::null_mut();
    let mut card: *mut snd_card = core::ptr::null_mut();
    let mut opl3: *mut snd_opl3 = core::ptr::null_mut();
    let mut err: c_int;

    err = snd_devm_card_new(
        pdev,
        index[dev],
        id[dev],
        THIS_MODULE,
        core::mem::size_of::<snd_sb8>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    let acard = (*card).private_data as *mut snd_sb8;

    /*
     * Block the 0x388 port to avoid PnP conflicts.
     * No need to check this value after request_region,
     * as we never do anything with it.
     */
    (*acard).fm_res = devm_request_region((*card).dev, 0x388, 4, c"SoundBlaster FM".as_ptr());

    if port[dev] != SNDRV_AUTO_PORT {
        err = snd_sbdsp_create(
            card,
            port[dev],
            irq[dev],
            Some(snd_sb8_interrupt),
            dma8[dev],
            -1,
            SB_HW_AUTO,
            &mut chip,
        );
        if err < 0 {
            return err;
        }
    } else {
        /* auto-probe legacy ports */
        static POSSIBLE_PORTS: [c_ulong; 3] = [0x220, 0x240, 0x260];
        let mut i: usize = 0;
        while i < POSSIBLE_PORTS.len() {
            err = snd_sbdsp_create(
                card,
                POSSIBLE_PORTS[i] as c_long,
                irq[dev],
                Some(snd_sb8_interrupt),
                dma8[dev],
                -1,
                SB_HW_AUTO,
                &mut chip,
            );
            if err >= 0 {
                port[dev] = POSSIBLE_PORTS[i] as c_long;
                break;
            }
            i += 1;
        }
        if i >= POSSIBLE_PORTS.len() {
            return -EINVAL;
        }
    }
    (*acard).chip = chip;

    if (*chip).hardware >= SB_HW_16 {
        if (*chip).hardware == SB_HW_ALS100 {
            dev_warn(
                pdev,
                c"ALS100 chip detected at 0x%lx, try snd-als100 module\n".as_ptr(),
                port[dev],
            );
        } else {
            dev_warn(
                pdev,
                c"SB 16 chip detected at 0x%lx, try snd-sb16 module\n".as_ptr(),
                port[dev],
            );
        }
        return -ENODEV;
    }

    err = snd_sb8dsp_pcm(chip, 0);
    if err < 0 {
        return err;
    }

    err = snd_sbmixer_new(chip);
    if err < 0 {
        return err;
    }

    if (*chip).hardware == SB_HW_10 || (*chip).hardware == SB_HW_20 {
        err = snd_opl3_create(card, (*chip).port + 8, 0, OPL3_HW_AUTO, 1, &mut opl3);
        if err < 0 {
            dev_warn(
                pdev,
                c"sb8: no OPL device at 0x%lx\n".as_ptr(),
                (*chip).port + 8,
            );
        }
    } else {
        err = snd_opl3_create(
            card,
            (*chip).port,
            (*chip).port + 2,
            OPL3_HW_AUTO,
            1,
            &mut opl3,
        );
        if err < 0 {
            dev_warn(
                pdev,
                c"sb8: no OPL device at 0x%lx-0x%lx\n".as_ptr(),
                (*chip).port,
                (*chip).port + 2,
            );
        }
    }
    if err >= 0 {
        err = snd_opl3_hwdep_new(opl3, 0, 1, core::ptr::null_mut());
        if err < 0 {
            return err;
        }
    }

    err = snd_sb8dsp_midi(chip, 0);
    if err < 0 {
        return err;
    }

    strscpy(
        (*card).driver,
        if (*chip).hardware == SB_HW_PRO {
            c"SB Pro".as_ptr()
        } else {
            c"SB8".as_ptr()
        },
    );
    strscpy((*card).shortname, (*chip).name);
    sprintf(
        (*card).longname,
        c"%s at 0x%lx, irq %d, dma %d".as_ptr(),
        (*chip).name,
        (*chip).port,
        irq[dev],
        dma8[dev],
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    dev_set_drvdata(pdev, card as *mut c_void);
    0
}

// Original code is guarded by #ifdef CONFIG_PM.
unsafe extern "C" fn snd_sb8_suspend(
    dev: *mut device,
    _n: c_uint,
    _state: pm_message_t,
) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_sb8;
    let chip = (*acard).chip;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3HOT);
    snd_sbmixer_suspend(chip);
    0
}

// Original code is guarded by #ifdef CONFIG_PM.
unsafe extern "C" fn snd_sb8_resume(dev: *mut device, _n: c_uint) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let acard = (*card).private_data as *mut snd_sb8;
    let chip = (*acard).chip;

    snd_sbdsp_reset(chip);
    snd_sbmixer_resume(chip);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

static mut snd_sb8_driver: isa_driver = isa_driver {
    match_: Some(snd_sb8_match),
    probe: Some(snd_sb8_probe),
    // Original fields are present only under CONFIG_PM.
    suspend: Some(snd_sb8_suspend),
    resume: Some(snd_sb8_resume),
    driver: isa_driver_driver {
        name: DEV_NAME.as_ptr() as *const c_char,
    },
};

// module_isa_driver(snd_sb8_driver, SNDRV_CARDS);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
