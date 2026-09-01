// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Driver for SoundBlaster 16/AWE32/AWE64 soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/* C includes translated as external dependency intent:
 * <asm/dma.h>, <linux/init.h>, <linux/pnp.h>, <linux/err.h>,
 * <linux/isa.h>, <linux/module.h>, <linux/string.h>,
 * <sound/core.h>, <sound/sb.h>, <sound/sb16_csp.h>,
 * <sound/mpu401.h>, <sound/opl3.h>, <sound/emu8000.h>,
 * <sound/seq_device.h>, <sound/initval.h>
 */

/* MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>"); */
/* MODULE_LICENSE("GPL"); */
/* If SNDRV_SBAWE is not defined: MODULE_DESCRIPTION("Sound Blaster 16");
 * otherwise: MODULE_DESCRIPTION("Sound Blaster AWE");
 */

/* #if 0
 * #define SNDRV_DEBUG_IRQ
 * #endif
 */

/* If defined(SNDRV_SBAWE) && IS_ENABLED(CONFIG_SND_SEQUENCER):
 * #define SNDRV_SBAWE_EMU8000
 */

extern "C" {
    static mut index: [::core::ffi::c_int; SNDRV_CARDS];
    static mut id: [*mut ::core::ffi::c_char; SNDRV_CARDS];
    static mut enable: [bool; SNDRV_CARDS];
}

/* module_param_array(index, int, NULL, 0444); */
/* MODULE_PARM_DESC(index, "Index value for SoundBlaster 16 soundcard."); */
/* module_param_array(id, charp, NULL, 0444); */
/* MODULE_PARM_DESC(id, "ID string for SoundBlaster 16 soundcard."); */
/* module_param_array(enable, bool, NULL, 0444); */
/* MODULE_PARM_DESC(enable, "Enable SoundBlaster 16 soundcard."); */

static mut isapnp: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
static mut port: [::core::ffi::c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS];
static mut mpu_port: [::core::ffi::c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS];
static mut fm_port: [::core::ffi::c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS];
/* Under SNDRV_SBAWE_EMU8000:
 * static long awe_port[SNDRV_CARDS] = SNDRV_DEFAULT_PORT;
 */
static mut awe_port: [::core::ffi::c_long; SNDRV_CARDS] = [SNDRV_DEFAULT_PORT; SNDRV_CARDS];
static mut irq: [::core::ffi::c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_IRQ; SNDRV_CARDS];
static mut dma8: [::core::ffi::c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_DMA; SNDRV_CARDS];
static mut dma16: [::core::ffi::c_int; SNDRV_CARDS] = [SNDRV_DEFAULT_DMA; SNDRV_CARDS];
static mut mic_agc: [::core::ffi::c_int; SNDRV_CARDS] = [1; SNDRV_CARDS];
/* Under CONFIG_SND_SB16_CSP: */
static mut csp: [::core::ffi::c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
/* Under SNDRV_SBAWE_EMU8000: */
static mut seq_ports: [::core::ffi::c_int; SNDRV_CARDS] = [4; SNDRV_CARDS];

/* module_param_hw_array/module_param_array declarations and MODULE_PARM_DESC
 * entries are preserved as external module metadata intent.
 */

/* Under CONFIG_PNP: */
static mut isa_registered: ::core::ffi::c_int = 0;
static mut pnp_registered: ::core::ffi::c_int = 0;

#[repr(C)]
pub struct snd_card_sb16 {
    pub fm_res: *mut resource, /* used to block FM i/o region for legacy cards */
    pub chip: *mut snd_sb,
    /* Under CONFIG_PNP: */
    pub dev_no: ::core::ffi::c_int,
    pub dev: *mut pnp_dev,
    /* Under SNDRV_SBAWE_EMU8000: */
    pub devwt: *mut pnp_dev,
}

/* Under CONFIG_PNP:
 * static const struct pnp_card_device_id snd_sb16_pnpids[] = { ... };
 * The table contains Sound Blaster 16 PnP entries when SNDRV_SBAWE is absent,
 * Sound Blaster AWE PnP entries when SNDRV_SBAWE is defined, then a sentinel.
 * MODULE_DEVICE_TABLE(pnp_card, snd_sb16_pnpids);
 */

/* Under SNDRV_SBAWE_EMU8000:
 * #define DRIVER_NAME "snd-card-sbawe"
 * otherwise:
 * #define DRIVER_NAME "snd-card-sb16"
 */
const DRIVER_NAME: *const ::core::ffi::c_char = b"snd-card-sb16\0".as_ptr().cast();

unsafe extern "C" fn snd_card_sb16_pnp(
    dev: ::core::ffi::c_int,
    acard: *mut snd_card_sb16,
    card: *mut pnp_card_link,
    id: *const pnp_card_device_id,
) -> ::core::ffi::c_int {
    let mut pdev: *mut pnp_dev;
    let mut err: ::core::ffi::c_int;

    (*acard).dev = pnp_request_card_device(card, (*(*id).devs.as_ptr()).id, ::core::ptr::null_mut());
    if (*acard).dev.is_null() {
        return -ENODEV;
    }

    /* Under SNDRV_SBAWE_EMU8000:
     * acard->devwt = pnp_request_card_device(card, id->devs[1].id, acard->dev);
     */
    (*acard).devwt = pnp_request_card_device(card, (*(*id).devs.as_ptr().add(1)).id, (*acard).dev);

    /* Audio initialization */
    pdev = (*acard).dev;

    err = pnp_activate_dev(pdev);
    if err < 0 {
        dev_err(&mut (*pdev).dev, b"AUDIO pnp configure failure\n\0".as_ptr().cast());
        return err;
    }
    port[dev as usize] = pnp_port_start(pdev, 0) as ::core::ffi::c_long;
    mpu_port[dev as usize] = pnp_port_start(pdev, 1) as ::core::ffi::c_long;
    fm_port[dev as usize] = pnp_port_start(pdev, 2) as ::core::ffi::c_long;
    dma8[dev as usize] = pnp_dma(pdev, 0);
    dma16[dev as usize] = pnp_dma(pdev, 1);
    irq[dev as usize] = pnp_irq(pdev, 0);
    dev_dbg(
        &mut (*pdev).dev,
        b"pnp SB16: port=0x%lx, mpu port=0x%lx, fm port=0x%lx\n\0".as_ptr().cast(),
        port[dev as usize],
        mpu_port[dev as usize],
        fm_port[dev as usize],
    );
    dev_dbg(
        &mut (*pdev).dev,
        b"pnp SB16: dma1=%i, dma2=%i, irq=%i\n\0".as_ptr().cast(),
        dma8[dev as usize],
        dma16[dev as usize],
        irq[dev as usize],
    );

    /* Under SNDRV_SBAWE_EMU8000: WaveTable initialization */
    pdev = (*acard).devwt;
    if !pdev.is_null() {
        err = pnp_activate_dev(pdev);
        if err < 0 {
            if !pdev.is_null() {
                pnp_release_card_device(pdev);
                dev_err(&mut (*pdev).dev, b"WaveTable pnp configure failure\n\0".as_ptr().cast());
            }
            (*acard).devwt = ::core::ptr::null_mut();
            awe_port[dev as usize] = -1;
        } else {
            awe_port[dev as usize] = pnp_port_start(pdev, 0) as ::core::ffi::c_long;
            dev_dbg(
                &mut (*pdev).dev,
                b"pnp SB16: wavetable port=0x%llx\n\0".as_ptr().cast(),
                pnp_port_start(pdev, 0) as ::core::ffi::c_ulonglong,
            );
        }
    } else {
        (*acard).devwt = ::core::ptr::null_mut();
        awe_port[dev as usize] = -1;
    }

    0
}

#[inline]
unsafe fn is_isapnp_selected(dev: ::core::ffi::c_int) -> ::core::ffi::c_int {
    isapnp[dev as usize] as ::core::ffi::c_int
}

unsafe extern "C" fn snd_sb16_card_new(
    devptr: *mut device,
    dev: ::core::ffi::c_int,
    cardp: *mut *mut snd_card,
) -> ::core::ffi::c_int {
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let mut err: ::core::ffi::c_int;

    err = snd_devm_card_new(
        devptr,
        index[dev as usize],
        id[dev as usize],
        THIS_MODULE,
        ::core::mem::size_of::<snd_card_sb16>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    *cardp = card;
    0
}

unsafe extern "C" fn snd_sb16_probe(card: *mut snd_card, dev: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut xirq: ::core::ffi::c_int;
    let mut xdma8: ::core::ffi::c_int;
    let mut xdma16: ::core::ffi::c_int;
    let mut chip: *mut snd_sb = ::core::ptr::null_mut();
    let acard: *mut snd_card_sb16 = (*card).private_data.cast();
    let mut opl3: *mut snd_opl3 = ::core::ptr::null_mut();
    let mut synth: *mut snd_hwdep = ::core::ptr::null_mut();
    let mut xcsp: *mut snd_hwdep = ::core::ptr::null_mut();
    let mut err: ::core::ffi::c_int;

    xirq = irq[dev as usize];
    xdma8 = dma8[dev as usize];
    xdma16 = dma16[dev as usize];

    err = snd_sbdsp_create(
        card,
        port[dev as usize],
        xirq,
        Some(snd_sb16dsp_interrupt),
        xdma8,
        xdma16,
        SB_HW_AUTO,
        &mut chip,
    );
    if err < 0 {
        return err;
    }

    (*acard).chip = chip;
    if (*chip).hardware != SB_HW_16 {
        dev_err(
            (*card).dev,
            b"SB 16 chip was not detected at 0x%lx\n\0".as_ptr().cast(),
            port[dev as usize],
        );
        return -ENODEV;
    }
    (*chip).mpu_port = mpu_port[dev as usize];
    if is_isapnp_selected(dev) == 0 {
        err = snd_sb16dsp_configure(chip);
        if err < 0 {
            return err;
        }
    }

    err = snd_sb16dsp_pcm(chip, 0);
    if err < 0 {
        return err;
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        if awe_port[dev as usize] > 0 {
            b"SB AWE\0".as_ptr().cast()
        } else {
            b"SB16\0".as_ptr().cast()
        },
    );
    strscpy((*card).shortname.as_mut_ptr(), (*chip).name.as_ptr());
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at 0x%lx, irq %i, dma \0".as_ptr().cast(),
        (*chip).name.as_ptr(),
        (*chip).port,
        xirq,
    );
    if xdma8 >= 0 {
        sprintf(
            (*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr()) as usize),
            b"%d\0".as_ptr().cast(),
            xdma8,
        );
    }
    if xdma16 >= 0 {
        sprintf(
            (*card).longname.as_mut_ptr().add(strlen((*card).longname.as_ptr()) as usize),
            b"%s%d\0".as_ptr().cast(),
            if xdma8 >= 0 { b"&\0".as_ptr().cast::<::core::ffi::c_char>() } else { b"\0".as_ptr().cast() },
            xdma16,
        );
    }

    if (*chip).mpu_port > 0 && (*chip).mpu_port != SNDRV_AUTO_PORT {
        err = snd_mpu401_uart_new(
            card,
            0,
            MPU401_HW_SB,
            (*chip).mpu_port,
            MPU401_INFO_IRQ_HOOK,
            -1,
            &mut (*chip).rmidi,
        );
        if err < 0 {
            return err;
        }
        (*chip).rmidi_callback = Some(snd_mpu401_uart_interrupt);
    }

    if awe_port[dev as usize] == SNDRV_AUTO_PORT {
        awe_port[dev as usize] = 0; /* disable */
    }

    if fm_port[dev as usize] > 0 && fm_port[dev as usize] != SNDRV_AUTO_PORT {
        if snd_opl3_create(
            card,
            fm_port[dev as usize],
            fm_port[dev as usize] + 2,
            OPL3_HW_OPL3,
            (!(*acard).fm_res.is_null() || fm_port[dev as usize] == port[dev as usize]) as ::core::ffi::c_int,
            &mut opl3,
        ) < 0
        {
            dev_err(
                (*card).dev,
                b"no OPL device at 0x%lx-0x%lx\n\0".as_ptr().cast(),
                fm_port[dev as usize],
                fm_port[dev as usize] + 2,
            );
        } else {
            let seqdev: ::core::ffi::c_int = if awe_port[dev as usize] > 0 { 2 } else { 1 };
            err = snd_opl3_hwdep_new(opl3, 0, seqdev, &mut synth);
            if err < 0 {
                return err;
            }
        }
    }

    err = snd_sbmixer_new(chip);
    if err < 0 {
        return err;
    }

    /* CSP chip on SB16ASP/AWE32 */
    if (*chip).hardware == SB_HW_16 && csp[dev as usize] != 0 {
        snd_sb_csp_new(chip, if !synth.is_null() { 1 } else { 0 }, &mut xcsp);
        if !xcsp.is_null() {
            (*chip).csp = (*xcsp).private_data;
            (*chip).hardware = SB_HW_16CSP;
        } else {
            dev_info(
                (*card).dev,
                b"warning - CSP chip not detected on soundcard #%i\n\0".as_ptr().cast(),
                dev + 1,
            );
        }
    }

    if awe_port[dev as usize] > 0 {
        err = snd_emu8000_new(card, 1, awe_port[dev as usize], seq_ports[dev as usize], ::core::ptr::null_mut());
        if err < 0 {
            dev_err(
                (*card).dev,
                b"fatal error - EMU-8000 synthesizer not detected at 0x%lx\n\0".as_ptr().cast(),
                awe_port[dev as usize],
            );

            return err;
        }
    }

    /* setup Mic AGC */
    {
        let flags = spin_lock_irqsave(&mut (*chip).mixer_lock);
        snd_sbmixer_write(
            chip,
            SB_DSP4_MIC_AGC,
            (snd_sbmixer_read(chip, SB_DSP4_MIC_AGC) & 0x01)
                | if mic_agc[dev as usize] != 0 { 0x00 } else { 0x01 },
        );
        spin_unlock_irqrestore(&mut (*chip).mixer_lock, flags);
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    0
}

unsafe extern "C" fn snd_sb16_suspend(card: *mut snd_card, state: pm_message_t) -> ::core::ffi::c_int {
    let acard: *mut snd_card_sb16 = (*card).private_data.cast();
    let chip: *mut snd_sb = (*acard).chip;

    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    snd_sbmixer_suspend(chip);
    0
}

unsafe extern "C" fn snd_sb16_resume(card: *mut snd_card) -> ::core::ffi::c_int {
    let acard: *mut snd_card_sb16 = (*card).private_data.cast();
    let chip: *mut snd_sb = (*acard).chip;

    snd_sbdsp_reset(chip);
    snd_sbmixer_resume(chip);
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    0
}

unsafe extern "C" fn snd_sb16_isa_probe1(dev: ::core::ffi::c_int, pdev: *mut device) -> ::core::ffi::c_int {
    let mut acard: *mut snd_card_sb16;
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let mut err: ::core::ffi::c_int;

    err = snd_sb16_card_new(pdev, dev, &mut card);
    if err < 0 {
        return err;
    }

    acard = (*card).private_data.cast();
    /* non-PnP FM port address is hardwired with base port address */
    fm_port[dev as usize] = port[dev as usize];
    /* block the 0x388 port to avoid PnP conflicts */
    (*acard).fm_res = devm_request_region((*card).dev, 0x388, 4, b"SoundBlaster FM\0".as_ptr().cast());
    /* Under SNDRV_SBAWE_EMU8000:
     * non-PnP AWE port address is hardwired with base port address
     */
    awe_port[dev as usize] = port[dev as usize] + 0x400;

    err = snd_sb16_probe(card, dev);
    if err < 0 {
        return err;
    }
    dev_set_drvdata(pdev, card.cast());
    0
}

unsafe extern "C" fn snd_sb16_isa_match(pdev: *mut device, dev: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = pdev;
    (enable[dev as usize] && is_isapnp_selected(dev as ::core::ffi::c_int) == 0) as ::core::ffi::c_int
}

unsafe extern "C" fn snd_sb16_isa_probe(pdev: *mut device, dev: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    static possible_irqs: [::core::ffi::c_int; 5] = [5, 9, 10, 7, -1];
    static possible_dmas8: [::core::ffi::c_int; 4] = [1, 3, 0, -1];
    static possible_dmas16: [::core::ffi::c_int; 4] = [5, 6, 7, -1];

    if irq[dev as usize] == SNDRV_AUTO_IRQ {
        irq[dev as usize] = snd_legacy_find_free_irq(possible_irqs.as_ptr());
        if irq[dev as usize] < 0 {
            dev_err(pdev, b"unable to find a free IRQ\n\0".as_ptr().cast());
            return -EBUSY;
        }
    }
    if dma8[dev as usize] == SNDRV_AUTO_DMA {
        dma8[dev as usize] = snd_legacy_find_free_dma(possible_dmas8.as_ptr());
        if dma8[dev as usize] < 0 {
            dev_err(pdev, b"unable to find a free 8-bit DMA\n\0".as_ptr().cast());
            return -EBUSY;
        }
    }
    if dma16[dev as usize] == SNDRV_AUTO_DMA {
        dma16[dev as usize] = snd_legacy_find_free_dma(possible_dmas16.as_ptr());
        if dma16[dev as usize] < 0 {
            dev_err(pdev, b"unable to find a free 16-bit DMA\n\0".as_ptr().cast());
            return -EBUSY;
        }
    }

    if port[dev as usize] != SNDRV_AUTO_PORT {
        snd_sb16_isa_probe1(dev as ::core::ffi::c_int, pdev)
    } else {
        static possible_ports: [::core::ffi::c_int; 4] = [0x220, 0x240, 0x260, 0x280];
        let mut i: usize = 0;
        while i < possible_ports.len() {
            port[dev as usize] = possible_ports[i] as ::core::ffi::c_long;
            err = snd_sb16_isa_probe1(dev as ::core::ffi::c_int, pdev);
            if err == 0 {
                return 0;
            }
            i += 1;
        }
        err
    }
}

unsafe extern "C" fn snd_sb16_isa_suspend(
    dev: *mut device,
    n: ::core::ffi::c_uint,
    state: pm_message_t,
) -> ::core::ffi::c_int {
    let _ = n;
    snd_sb16_suspend(dev_get_drvdata(dev).cast(), state)
}

unsafe extern "C" fn snd_sb16_isa_resume(dev: *mut device, n: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let _ = n;
    snd_sb16_resume(dev_get_drvdata(dev).cast())
}

/* Under SNDRV_SBAWE:
 * #define DEV_NAME "sbawe"
 * otherwise:
 * #define DEV_NAME "sb16"
 */
const DEV_NAME: *const ::core::ffi::c_char = b"sb16\0".as_ptr().cast();

static mut snd_sb16_isa_driver: isa_driver = isa_driver {
    match_: Some(snd_sb16_isa_match),
    probe: Some(snd_sb16_isa_probe),
    suspend: Some(snd_sb16_isa_suspend),
    resume: Some(snd_sb16_isa_resume),
    driver: device_driver { name: DEV_NAME },
};

unsafe extern "C" fn snd_sb16_pnp_detect(
    pcard: *mut pnp_card_link,
    pid: *const pnp_card_device_id,
) -> ::core::ffi::c_int {
    static mut dev: ::core::ffi::c_int = 0;
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let mut res: ::core::ffi::c_int;

    while dev < SNDRV_CARDS as ::core::ffi::c_int {
        if !enable[dev as usize] || !isapnp[dev as usize] {
            dev += 1;
            continue;
        }
        res = snd_sb16_card_new(&mut (*(*pcard).card).dev, dev, &mut card);
        if res < 0 {
            return res;
        }
        res = snd_card_sb16_pnp(dev, (*card).private_data.cast(), pcard, pid);
        if res < 0 {
            return res;
        }
        res = snd_sb16_probe(card, dev);
        if res < 0 {
            return res;
        }
        pnp_set_card_drvdata(pcard, card.cast());
        dev += 1;
        return 0;
    }

    -ENODEV
}

unsafe extern "C" fn snd_sb16_pnp_suspend(pcard: *mut pnp_card_link, state: pm_message_t) -> ::core::ffi::c_int {
    snd_sb16_suspend(pnp_get_card_drvdata(pcard).cast(), state)
}

unsafe extern "C" fn snd_sb16_pnp_resume(pcard: *mut pnp_card_link) -> ::core::ffi::c_int {
    snd_sb16_resume(pnp_get_card_drvdata(pcard).cast())
}

static mut sb16_pnpc_driver: pnp_card_driver = pnp_card_driver {
    flags: PNP_DRIVER_RES_DISABLE,
    name: b"sb16\0".as_ptr().cast(),
    id_table: snd_sb16_pnpids.as_ptr(),
    probe: Some(snd_sb16_pnp_detect),
    suspend: Some(snd_sb16_pnp_suspend),
    resume: Some(snd_sb16_pnp_resume),
};

unsafe extern "C" fn alsa_card_sb16_init() -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int;

    err = isa_register_driver(&mut snd_sb16_isa_driver, SNDRV_CARDS as ::core::ffi::c_uint);
    if err == 0 {
        isa_registered = 1;
    }

    err = pnp_register_card_driver(&mut sb16_pnpc_driver);
    if err == 0 {
        pnp_registered = 1;
    }

    if isa_registered != 0 {
        err = 0;
    }
    err
}

unsafe extern "C" fn alsa_card_sb16_exit() {
    if pnp_registered != 0 {
        pnp_unregister_card_driver(&mut sb16_pnpc_driver);
    }
    if isa_registered != 0 {
        isa_unregister_driver(&mut snd_sb16_isa_driver);
    }
}

/* module_init(alsa_card_sb16_init) */
/* module_exit(alsa_card_sb16_exit) */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
