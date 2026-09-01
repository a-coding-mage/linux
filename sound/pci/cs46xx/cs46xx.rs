// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/*
  NOTES:
  - sometimes the sound is metallic and sibilant, unloading and
    reloading the module may solve this.
*/

/* C dependencies removed from executable Rust:
 * <linux/pci.h>, <linux/time.h>, <linux/init.h>, <linux/module.h>,
 * <sound/core.h>, "cs46xx.h", <sound/initval.h>
 */

MODULE_AUTHOR!("Jaroslav Kysela <perex@perex.cz>");
MODULE_DESCRIPTION!("Cirrus Logic Sound Fusion CS46XX");
MODULE_LICENSE!("GPL");

static mut index: [::core::ffi::c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut ::core::ffi::c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut external_amp: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
static mut thinkpad: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS];
static mut mmap_valid: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];

module_param_array!(index, int, NULL, 0o444);
MODULE_PARM_DESC!(index, "Index value for the CS46xx soundcard.");
module_param_array!(id, charp, NULL, 0o444);
MODULE_PARM_DESC!(id, "ID string for the CS46xx soundcard.");
module_param_array!(enable, bool, NULL, 0o444);
MODULE_PARM_DESC!(enable, "Enable CS46xx soundcard.");
module_param_array!(external_amp, bool, NULL, 0o444);
MODULE_PARM_DESC!(external_amp, "Force to enable external amplifier.");
module_param_array!(thinkpad, bool, NULL, 0o444);
MODULE_PARM_DESC!(thinkpad, "Force to enable Thinkpad's CLKRUN control.");
module_param_array!(mmap_valid, bool, NULL, 0o444);
MODULE_PARM_DESC!(mmap_valid, "Support OSS mmap.");

static snd_cs46xx_ids: [pci_device_id; 4] = [
    PCI_VDEVICE!(CIRRUS, 0x6001), /* CS4280 */
    PCI_VDEVICE!(CIRRUS, 0x6003), /* CS4612 */
    PCI_VDEVICE!(CIRRUS, 0x6004), /* CS4615 */
    pci_device_id {},
];

MODULE_DEVICE_TABLE!(pci, snd_cs46xx_ids);

unsafe extern "C" fn snd_card_cs46xx_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> ::core::ffi::c_int {
    static mut dev: ::core::ffi::c_int = 0;
    let mut card: *mut snd_card = ::core::ptr::null_mut();
    let mut chip: *mut snd_cs46xx;
    let mut err: ::core::ffi::c_int;

    let _ = pci_id;

    if dev >= SNDRV_CARDS as ::core::ffi::c_int {
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
        ::core::mem::size_of::<snd_cs46xx>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut snd_cs46xx;
    err = snd_cs46xx_create(
        card,
        pci,
        external_amp[dev as usize],
        thinkpad[dev as usize],
    );
    if err < 0 {
        goto_error(card, err);
        return err;
    }
    (*card).private_data = chip as *mut ::core::ffi::c_void;
    (*chip).accept_valid = mmap_valid[dev as usize];
    err = snd_cs46xx_pcm(chip, 0);
    if err < 0 {
        goto_error(card, err);
        return err;
    }

    /* #ifdef CONFIG_SND_CS46XX_NEW_DSP */
    #[cfg(CONFIG_SND_CS46XX_NEW_DSP)]
    {
        err = snd_cs46xx_pcm_rear(chip, 1);
        if err < 0 {
            goto_error(card, err);
            return err;
        }
        err = snd_cs46xx_pcm_iec958(chip, 2);
        if err < 0 {
            goto_error(card, err);
            return err;
        }
    }
    /* #endif */

    err = snd_cs46xx_mixer(chip, 2);
    if err < 0 {
        goto_error(card, err);
        return err;
    }

    /* #ifdef CONFIG_SND_CS46XX_NEW_DSP */
    #[cfg(CONFIG_SND_CS46XX_NEW_DSP)]
    {
        if (*chip).nr_ac97_codecs == 2 {
            err = snd_cs46xx_pcm_center_lfe(chip, 3);
            if err < 0 {
                goto_error(card, err);
                return err;
            }
        }
    }
    /* #endif */

    err = snd_cs46xx_midi(chip, 0);
    if err < 0 {
        goto_error(card, err);
        return err;
    }
    err = snd_cs46xx_start_dsp(chip);
    if err < 0 {
        goto_error(card, err);
        return err;
    }

    snd_cs46xx_gameport(chip);

    strscpy((*card).driver.as_mut_ptr(), c"CS46xx".as_ptr());
    strscpy(
        (*card).shortname.as_mut_ptr(),
        c"Sound Fusion CS46xx".as_ptr(),
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s at 0x%lx/0x%lx, irq %i".as_ptr(),
        (*card).shortname.as_ptr(),
        (*chip).ba0_addr,
        (*chip).ba1_addr,
        (*chip).irq,
    );

    err = snd_card_register(card);
    if err < 0 {
        goto_error(card, err);
        return err;
    }

    pci_set_drvdata(pci, card as *mut ::core::ffi::c_void);
    dev += 1;
    0
}

unsafe fn goto_error(card: *mut snd_card, err: ::core::ffi::c_int) {
    let _ = err;
    snd_card_free(card);
}

static mut cs46xx_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_cs46xx_ids.as_ptr(),
    probe: Some(snd_card_cs46xx_probe),
    /* #ifdef CONFIG_PM_SLEEP
     * .driver = {
     *     .pm = &snd_cs46xx_pm,
     * },
     * #endif
     */
    #[cfg(CONFIG_PM_SLEEP)]
    driver: device_driver {
        pm: &snd_cs46xx_pm,
    },
};

module_pci_driver!(cs46xx_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
