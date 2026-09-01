// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for audio on multifunction CS5535/6 companion device
 * Copyright (C) Jaya Kumar
 *
 * Based on Jaroslav Kysela and Takashi Iwai's examples.
 * This work was sponsored by CIS(M) Sdn Bhd.
 */

// C dependencies:
// linux/delay.h, linux/interrupt.h, linux/init.h, linux/pci.h,
// linux/slab.h, linux/module.h, linux/io.h, sound/core.h,
// sound/control.h, sound/pcm.h, sound/rawmidi.h, sound/ac97_codec.h,
// sound/initval.h, sound/asoundef.h, and "cs5535audio.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};
use core::mem;
use core::ptr;

const DRIVER_NAME: &[u8] = b"cs5535audio\0";

static mut ac97_quirk: *mut c_char = ptr::null_mut();
// module_param(ac97_quirk, charp, 0444);
// MODULE_PARM_DESC(ac97_quirk, "AC'97 board specific workarounds.");

static ac97_quirks: [ac97_quirk; 1] = [
    /*
     * #if 0
     * Not yet confirmed if all 5536 boards are HP only:
     * {
     *     .subvendor = PCI_VENDOR_ID_AMD,
     *     .subdevice = PCI_DEVICE_ID_AMD_CS5536_AUDIO,
     *     .name = "AMD RDK",
     *     .type = AC97_TUNE_HP_ONLY
     * },
     * #endif
     */
    ac97_quirk {
        subvendor: 0,
        subdevice: 0,
        name: ptr::null(),
        type_: 0,
    },
];

static mut index: [c_int; SNDRV_CARDS as usize] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS as usize] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS as usize] = SNDRV_DEFAULT_ENABLE_PNP;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " DRIVER_NAME);
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " DRIVER_NAME);
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " DRIVER_NAME);

static snd_cs5535audio_ids: [pci_device_id; 3] = [
    PCI_DEVICE(PCI_VENDOR_ID_NS, PCI_DEVICE_ID_NS_CS5535_AUDIO),
    PCI_DEVICE(PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_CS5536_AUDIO),
    pci_device_id::default(),
];

// MODULE_DEVICE_TABLE(pci, snd_cs5535audio_ids);

unsafe fn wait_till_cmd_acked(cs5535au: *mut cs5535audio, mut timeout: c_ulong) {
    let mut tmp: c_uint;

    loop {
        tmp = cs_readl(cs5535au, ACC_CODEC_CNTL);
        if tmp & CMD_NEW == 0 {
            break;
        }
        udelay(1);
        timeout = timeout.wrapping_sub(1);
        if timeout == 0 {
            break;
        }
    }
    if timeout == 0 {
        dev_err(
            (*(*cs5535au).card).dev,
            c"Failure writing to cs5535 codec\n".as_ptr(),
        );
    }
}

unsafe fn snd_cs5535audio_codec_read(
    cs5535au: *mut cs5535audio,
    reg: c_ushort,
) -> c_ushort {
    let mut regdata: c_uint;
    let mut timeout: c_uint;
    let mut val: c_uint;

    regdata = (reg as c_uint) << 24;
    regdata |= ACC_CODEC_CNTL_RD_CMD;
    regdata |= CMD_NEW;

    cs_writel(cs5535au, ACC_CODEC_CNTL, regdata);
    wait_till_cmd_acked(cs5535au, 50);

    timeout = 50;
    loop {
        val = cs_readl(cs5535au, ACC_CODEC_STATUS);
        if (val & STS_NEW) != 0 && reg == (val >> 24) as c_ushort {
            break;
        }
        udelay(1);
        timeout = timeout.wrapping_sub(1);
        if timeout == 0 {
            break;
        }
    }
    if timeout == 0 {
        dev_err(
            (*(*cs5535au).card).dev,
            c"Failure reading codec reg 0x%x, Last value=0x%x\n".as_ptr(),
            reg as c_uint,
            val,
        );
    }

    val as c_ushort
}

unsafe fn snd_cs5535audio_codec_write(
    cs5535au: *mut cs5535audio,
    reg: c_ushort,
    val: c_ushort,
) {
    let mut regdata: c_uint;

    regdata = (reg as c_uint) << 24;
    regdata |= val as c_uint;
    regdata &= CMD_MASK;
    regdata |= CMD_NEW;
    regdata &= ACC_CODEC_CNTL_WR_CMD;

    cs_writel(cs5535au, ACC_CODEC_CNTL, regdata);
    wait_till_cmd_acked(cs5535au, 50);
}

unsafe extern "C" fn snd_cs5535audio_ac97_codec_write(
    ac97: *mut snd_ac97,
    reg: c_ushort,
    val: c_ushort,
) {
    let cs5535au: *mut cs5535audio = (*ac97).private_data as *mut cs5535audio;
    snd_cs5535audio_codec_write(cs5535au, reg, val);
}

unsafe extern "C" fn snd_cs5535audio_ac97_codec_read(
    ac97: *mut snd_ac97,
    reg: c_ushort,
) -> c_ushort {
    let cs5535au: *mut cs5535audio = (*ac97).private_data as *mut cs5535audio;
    snd_cs5535audio_codec_read(cs5535au, reg)
}

unsafe fn snd_cs5535audio_mixer(cs5535au: *mut cs5535audio) -> c_int {
    let card: *mut snd_card = (*cs5535au).card;
    let mut pbus: *mut snd_ac97_bus = ptr::null_mut();
    let mut ac97: snd_ac97_template;
    let mut err: c_int;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(snd_cs5535audio_ac97_codec_write),
        read: Some(snd_cs5535audio_ac97_codec_read),
    };

    err = snd_ac97_bus(card, 0, &ops, ptr::null_mut(), &mut pbus);
    if err < 0 {
        return err;
    }

    ac97 = mem::zeroed();
    ac97.scaps = AC97_SCAP_AUDIO | AC97_SCAP_SKIP_MODEM | AC97_SCAP_POWER_SAVE;
    ac97.private_data = cs5535au as *mut c_void;
    ac97.pci = (*cs5535au).pci;

    /* set any OLPC-specific scaps */
    olpc_prequirks(card, &mut ac97);

    err = snd_ac97_mixer(pbus, &mut ac97, &mut (*cs5535au).ac97);
    if err < 0 {
        dev_err((*card).dev, c"mixer failed\n".as_ptr());
        return err;
    }

    snd_ac97_tune_hardware((*cs5535au).ac97, ac97_quirks.as_ptr(), ac97_quirk);

    err = olpc_quirks(card, (*cs5535au).ac97);
    if err < 0 {
        dev_err((*card).dev, c"olpc quirks failed\n".as_ptr());
        return err;
    }

    0
}

unsafe fn process_bm0_irq(cs5535au: *mut cs5535audio) {
    let bm_stat: u8;

    spin_lock(&mut (*cs5535au).reg_lock);
    bm_stat = cs_readb(cs5535au, ACC_BM0_STATUS);
    spin_unlock(&mut (*cs5535au).reg_lock);

    if bm_stat & EOP != 0 {
        snd_pcm_period_elapsed((*cs5535au).playback_substream);
    } else {
        dev_err(
            (*(*cs5535au).card).dev,
            c"unexpected bm0 irq src, bm_stat=%x\n".as_ptr(),
            bm_stat as c_uint,
        );
    }
}

unsafe fn process_bm1_irq(cs5535au: *mut cs5535audio) {
    let bm_stat: u8;

    spin_lock(&mut (*cs5535au).reg_lock);
    bm_stat = cs_readb(cs5535au, ACC_BM1_STATUS);
    spin_unlock(&mut (*cs5535au).reg_lock);

    if bm_stat & EOP != 0 {
        snd_pcm_period_elapsed((*cs5535au).capture_substream);
    }
}

unsafe extern "C" fn snd_cs5535audio_interrupt(
    _irq: c_int,
    dev_id: *mut c_void,
) -> irqreturn_t {
    let acc_irq_stat: u16;
    let mut count: u8;
    let cs5535au: *mut cs5535audio = dev_id as *mut cs5535audio;

    if cs5535au.is_null() {
        return IRQ_NONE;
    }

    acc_irq_stat = cs_readw(cs5535au, ACC_IRQ_STATUS);

    if acc_irq_stat == 0 {
        return IRQ_NONE;
    }
    count = 0;
    while count < 4 {
        if (acc_irq_stat as c_int & (1 << count)) != 0 {
            match count as c_int {
                IRQ_STS => {
                    cs_readl(cs5535au, ACC_GPIO_STATUS);
                }
                WU_IRQ_STS => {
                    cs_readl(cs5535au, ACC_GPIO_STATUS);
                }
                BM0_IRQ_STS => {
                    process_bm0_irq(cs5535au);
                }
                BM1_IRQ_STS => {
                    process_bm1_irq(cs5535au);
                }
                _ => {
                    dev_err(
                        (*(*cs5535au).card).dev,
                        c"Unexpected irq src: 0x%x\n".as_ptr(),
                        acc_irq_stat as c_uint,
                    );
                }
            }
        }
        count = count.wrapping_add(1);
    }
    IRQ_HANDLED
}

unsafe extern "C" fn snd_cs5535audio_free(_card: *mut snd_card) {
    olpc_quirks_cleanup();
}

unsafe fn snd_cs5535audio_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let cs5535au: *mut cs5535audio = (*card).private_data as *mut cs5535audio;
    let mut err: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(32)) != 0 {
        dev_warn((*card).dev, c"unable to get 32bit dma\n".as_ptr());
        return -ENXIO;
    }

    spin_lock_init(&mut (*cs5535au).reg_lock);
    (*cs5535au).card = card;
    (*cs5535au).pci = pci;
    (*cs5535au).irq = -1;

    err = pcim_request_all_regions(pci, c"CS5535 Audio".as_ptr());
    if err < 0 {
        return err;
    }

    (*cs5535au).port = pci_resource_start(pci, 0);

    if devm_request_irq(
        &mut (*pci).dev,
        (*pci).irq,
        Some(snd_cs5535audio_interrupt),
        IRQF_SHARED,
        KBUILD_MODNAME,
        cs5535au as *mut c_void,
    ) != 0
    {
        dev_err(
            (*card).dev,
            c"unable to grab IRQ %d\n".as_ptr(),
            (*pci).irq,
        );
        return -EBUSY;
    }

    (*cs5535au).irq = (*pci).irq;
    (*card).sync_irq = (*cs5535au).irq;
    pci_set_master(pci);

    0
}

unsafe extern "C" fn __snd_cs5535audio_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut cs5535au: *mut cs5535audio;
    let mut err: c_int;

    if dev >= SNDRV_CARDS {
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
        mem::size_of::<cs5535audio>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    cs5535au = (*card).private_data as *mut cs5535audio;
    (*card).private_free = Some(snd_cs5535audio_free);

    err = snd_cs5535audio_create(card, pci);
    if err < 0 {
        return err;
    }

    err = snd_cs5535audio_mixer(cs5535au);
    if err < 0 {
        return err;
    }

    err = snd_cs5535audio_pcm(cs5535au);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), DRIVER_NAME.as_ptr() as *const c_char);

    strscpy((*card).shortname.as_mut_ptr(), c"CS5535 Audio".as_ptr());
    sprintf(
        (*card).longname.as_mut_ptr(),
        c"%s %s at 0x%lx, irq %i".as_ptr(),
        (*card).shortname.as_ptr(),
        (*card).driver.as_ptr(),
        (*cs5535au).port,
        (*cs5535au).irq,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_cs5535audio_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_cs5535audio_probe(pci, pci_id))
}

static mut cs5535audio_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_cs5535audio_ids.as_ptr(),
    probe: Some(snd_cs5535audio_probe),
    /*
     * #ifdef CONFIG_PM_SLEEP
     * .driver = {
     *     .pm = &snd_cs5535audio_pm,
     * },
     * #endif
     */
    driver: device_driver {
        pm: ptr::null(),
    },
};

// module_pci_driver(cs5535audio_driver);

// MODULE_AUTHOR("Jaya Kumar");
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("CS5535 Audio");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
