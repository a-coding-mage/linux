// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Sound Core PDAudioCF soundcard
 *
 * Copyright (c) 2003 by Jaroslav Kysela <perex@perex.cz>
 */

// C dependencies: <sound/core.h>, <linux/slab.h>, <linux/module.h>,
// <pcmcia/ciscode.h>, <pcmcia/cisreg.h>, "pdaudiocf.h",
// <sound/initval.h>, <linux/init.h>

/*
 */

const CARD_NAME: &[u8] = b"PDAudio-CF\0";

// MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
// MODULE_DESCRIPTION("Sound Core " CARD_NAME);
// MODULE_LICENSE("GPL");

static mut index: [::std::os::raw::c_int; SNDRV_CARDS as usize] = SNDRV_DEFAULT_IDX;
static mut id: [*mut ::std::os::raw::c_char; SNDRV_CARDS as usize] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS as usize] = SNDRV_DEFAULT_ENABLE_PNP;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " CARD_NAME " soundcard.");

/*
 */

static mut card_list: [*mut snd_card; SNDRV_CARDS as usize] =
    [::std::ptr::null_mut(); SNDRV_CARDS as usize];

/*
 * prototypes
 */
unsafe fn pdacf_config(link: *mut pcmcia_device) -> ::std::os::raw::c_int;
unsafe fn snd_pdacf_detach(p_dev: *mut pcmcia_device);

unsafe fn pdacf_release(link: *mut pcmcia_device) {
    free_irq((*link).irq, (*link).priv);
    pcmcia_disable_device(link);
}

/*
 * destructor
 */
unsafe fn snd_pdacf_free(pdacf: *mut snd_pdacf) -> ::std::os::raw::c_int {
    let link: *mut pcmcia_device = (*pdacf).p_dev;

    pdacf_release(link);

    card_list[(*pdacf).index as usize] = ::std::ptr::null_mut();
    (*pdacf).card = ::std::ptr::null_mut();

    kfree(pdacf as *const ::std::ffi::c_void);
    0
}

unsafe fn snd_pdacf_dev_free(device: *mut snd_device) -> ::std::os::raw::c_int {
    let chip: *mut snd_pdacf = (*device).device_data as *mut snd_pdacf;
    snd_pdacf_free(chip)
}

/*
 * snd_pdacf_attach - attach callback for cs
 */
unsafe fn snd_pdacf_probe(link: *mut pcmcia_device) -> ::std::os::raw::c_int {
    let mut i: ::std::os::raw::c_int;
    let mut err: ::std::os::raw::c_int;
    let mut pdacf: *mut snd_pdacf;
    let mut card: *mut snd_card = ::std::ptr::null_mut();
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_pdacf_dev_free),
    };

    /* find an empty slot from the card list */
    i = 0;
    while i < SNDRV_CARDS {
        if card_list[i as usize].is_null() {
            break;
        }
        i += 1;
    }
    if i >= SNDRV_CARDS {
        dev_err(
            &mut (*link).dev,
            b"pdacf: too many cards found\n\0".as_ptr() as *const ::std::os::raw::c_char,
        );
        return -EINVAL;
    }
    if !enable[i as usize] {
        return -ENODEV; /* disabled explicitly */
    }

    /* ok, create a card instance */
    err = snd_card_new(
        &mut (*link).dev,
        index[i as usize],
        id[i as usize],
        THIS_MODULE,
        0,
        &mut card,
    );
    if err < 0 {
        dev_err(
            &mut (*link).dev,
            b"pdacf: cannot create a card instance\n\0".as_ptr()
                as *const ::std::os::raw::c_char,
        );
        return err;
    }

    pdacf = snd_pdacf_create(card);
    if pdacf.is_null() {
        snd_card_free(card);
        return -ENOMEM;
    }

    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, pdacf as *mut ::std::ffi::c_void, &ops);
    if err < 0 {
        kfree(pdacf as *const ::std::ffi::c_void);
        snd_card_free(card);
        return err;
    }

    (*pdacf).index = i;
    card_list[i as usize] = card;

    (*pdacf).p_dev = link;
    (*link).priv = pdacf as *mut ::std::ffi::c_void;

    (*(*link).resource[0]).flags |= IO_DATA_PATH_WIDTH_AUTO;
    (*(*link).resource[0]).end = 16;

    (*link).config_flags = CONF_ENABLE_IRQ | CONF_ENABLE_PULSE_IRQ;
    (*link).config_index = 1;
    (*link).config_regs = PRESENT_OPTION;

    err = pdacf_config(link);
    if err < 0 {
        card_list[i as usize] = ::std::ptr::null_mut();
        snd_card_free(card);
        return err;
    }
    0
}

/**
 * snd_pdacf_assign_resources - initialize the hardware and card instance.
 * @pdacf: context
 * @port: i/o port for the card
 * @irq: irq number for the card
 *
 * this function assigns the specified port and irq, boot the card,
 * create pcm and control instances, and initialize the rest hardware.
 *
 * returns 0 if successful, or a negative error code.
 */
unsafe fn snd_pdacf_assign_resources(
    pdacf: *mut snd_pdacf,
    port: ::std::os::raw::c_int,
    irq: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let mut err: ::std::os::raw::c_int;
    let card: *mut snd_card = (*pdacf).card;

    dev_dbg(
        (*card).dev,
        b"pdacf assign resources: port = 0x%x, irq = %d\n\0".as_ptr()
            as *const ::std::os::raw::c_char,
        port,
        irq,
    );
    (*pdacf).port = port;
    (*pdacf).irq = irq;
    (*pdacf).chip_status |= PDAUDIOCF_STAT_IS_CONFIGURED;

    err = snd_pdacf_ak4117_create(pdacf);
    if err < 0 {
        return err;
    }

    strscpy(
        (*card).driver.as_mut_ptr(),
        b"PDAudio-CF\0".as_ptr() as *const ::std::os::raw::c_char,
    );
    sprintf(
        (*card).shortname.as_mut_ptr(),
        b"Core Sound %s\0".as_ptr() as *const ::std::os::raw::c_char,
        (*card).driver.as_mut_ptr(),
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at 0x%x, irq %i\0".as_ptr() as *const ::std::os::raw::c_char,
        (*card).shortname.as_mut_ptr(),
        port,
        irq,
    );

    err = snd_pdacf_pcm_new(pdacf);
    if err < 0 {
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    0
}

/*
 * snd_pdacf_detach - detach callback for cs
 */
unsafe fn snd_pdacf_detach(link: *mut pcmcia_device) {
    let chip: *mut snd_pdacf = (*link).priv as *mut snd_pdacf;

    if (*chip).chip_status & PDAUDIOCF_STAT_IS_CONFIGURED != 0 {
        snd_pdacf_powerdown(chip);
    }
    (*chip).chip_status |= PDAUDIOCF_STAT_IS_STALE; /* to be sure */
    snd_card_disconnect((*chip).card);
    snd_card_free_when_closed((*chip).card);
}

/*
 * configuration callback
 */

unsafe fn pdacf_config(link: *mut pcmcia_device) -> ::std::os::raw::c_int {
    let pdacf: *mut snd_pdacf = (*link).priv as *mut snd_pdacf;
    let mut ret: ::std::os::raw::c_int;

    (*link).config_index = 0x5;
    (*link).config_flags |= CONF_ENABLE_IRQ | CONF_ENABLE_PULSE_IRQ;

    ret = pcmcia_request_io(link);
    if ret != 0 {
        return pdacf_config_failed_preirq(link);
    }

    ret = request_threaded_irq(
        (*link).irq,
        Some(pdacf_interrupt),
        Some(pdacf_threaded_irq),
        IRQF_SHARED,
        (*link).devname,
        (*link).priv,
    );
    if ret != 0 {
        return pdacf_config_failed_preirq(link);
    }

    ret = pcmcia_enable_device(link);
    if ret != 0 {
        return pdacf_config_failed(link);
    }

    if snd_pdacf_assign_resources(pdacf, (*(*link).resource[0]).start, (*link).irq) < 0 {
        return pdacf_config_failed(link);
    }

    (*(*pdacf).card).sync_irq = (*link).irq;
    0
}

unsafe fn pdacf_config_failed(link: *mut pcmcia_device) -> ::std::os::raw::c_int {
    free_irq((*link).irq, (*link).priv);
    pdacf_config_failed_preirq(link)
}

unsafe fn pdacf_config_failed_preirq(link: *mut pcmcia_device) -> ::std::os::raw::c_int {
    pcmcia_disable_device(link);
    -ENODEV
}

// #ifdef CONFIG_PM
// Power-management callbacks are present when CONFIG_PM is enabled in C.

unsafe fn pdacf_suspend(link: *mut pcmcia_device) -> ::std::os::raw::c_int {
    let chip: *mut snd_pdacf = (*link).priv as *mut snd_pdacf;

    if !chip.is_null() {
        snd_pdacf_suspend(chip);
    }

    0
}

unsafe fn pdacf_resume(link: *mut pcmcia_device) -> ::std::os::raw::c_int {
    let chip: *mut snd_pdacf = (*link).priv as *mut snd_pdacf;

    if pcmcia_dev_present(link) != 0 {
        if !chip.is_null() {
            snd_pdacf_resume(chip);
        }
    }

    0
}

// #endif

/*
 * Module entry points
 */
static snd_pdacf_ids: [pcmcia_device_id; 2] = [
    /* this is too general PCMCIA_DEVICE_MANF_CARD(0x015d, 0x4c45), */
    PCMCIA_DEVICE_PROD_ID12(
        b"Core Sound\0".as_ptr() as *const ::std::os::raw::c_char,
        b"PDAudio-CF\0".as_ptr() as *const ::std::os::raw::c_char,
        0x396d19d2,
        0x71717b49,
    ),
    PCMCIA_DEVICE_NULL,
];
// MODULE_DEVICE_TABLE(pcmcia, snd_pdacf_ids);

static mut pdacf_cs_driver: pcmcia_driver = pcmcia_driver {
    owner: THIS_MODULE,
    name: b"snd-pdaudiocf\0".as_ptr() as *const ::std::os::raw::c_char,
    probe: Some(snd_pdacf_probe),
    remove: Some(snd_pdacf_detach),
    id_table: snd_pdacf_ids.as_ptr(),
    // #ifdef CONFIG_PM
    suspend: Some(pdacf_suspend),
    resume: Some(pdacf_resume),
    // #endif
};
// module_pcmcia_driver(pdacf_cs_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
