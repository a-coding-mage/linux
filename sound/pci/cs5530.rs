// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * cs5530.c - Initialisation code for Cyrix/NatSemi VSA1 softaudio
 *
 * 	(C) Copyright 2007 Ash Willis <ashwillis@programmer.net>
 *	(C) Copyright 2003 Red Hat Inc <alan@lxorguk.ukuu.org.uk>
 *
 * This driver was ported (shamelessly ripped ;) from oss/kahlua.c but I did
 * mess with it a bit. The chip seems to have to have trouble with full duplex
 * mode. If we're recording in 8bit 8000kHz, say, and we then attempt to
 * simultaneously play back audio at 16bit 44100kHz, the device actually plays
 * back in the same format in which it is capturing. By forcing the chip to
 * always play/capture in 16/44100, we can let alsa-lib convert the samples and
 * that way we can hack up some full duplex audio.
 *
 * XpressAudio(tm) is used on the Cyrix MediaGX (now NatSemi Geode) systems.
 * The older version (VSA1) provides fairly good soundblaster emulation
 * although there are a couple of bugs: large DMA buffers break record,
 * and the MPU event handling seems suspect. VSA2 allows the native driver
 * to control the AC97 audio engine directly and requires a different driver.
 *
 * Thanks to National Semiconductor for providing the needed information
 * on the XpressAudio(tm) internals.
 *
 * TO DO:
 *	Investigate whether we can portably support Cognac (5520) in the
 *	same manner.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

// C include dependencies:
// linux/delay.h, linux/module.h, linux/pci.h, linux/slab.h,
// sound/core.h, sound/sb.h, sound/initval.h
//
// MODULE_AUTHOR("Ash Willis");
// MODULE_DESCRIPTION("CS5530 Audio");
// MODULE_LICENSE("GPL");

extern "C" {
    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS];
    static SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS];

    static THIS_MODULE: *mut c_void;
    static KBUILD_MODNAME: *const c_char;

    static PCI_VENDOR_ID_CYRIX: u32;
    static PCI_DEVICE_ID_CYRIX_5530_AUDIO: u32;
    static PCI_ANY_ID: u32;
    static ENODEV: c_int;
    static ENOENT: c_int;
    static SB_HW_CS5530: c_int;

    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn udelay(usecs: c_ulong);
    fn pcim_enable_device(pci: *mut pci_dev) -> c_int;
    fn pcim_iomap_region(pci: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_resource_start(pci: *mut pci_dev, bar: c_int) -> c_ulong;
    fn readw(addr: *const c_void) -> u16;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_sbdsp_create(
        card: *mut snd_card,
        port: c_ulong,
        irq: u8,
        irq_handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        dma8: u8,
        dma16: u8,
        hardware: c_int,
        r_chip: *mut *mut snd_sb,
    ) -> c_int;
    fn snd_sb16dsp_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn snd_sb16dsp_pcm(chip: *mut snd_sb, device: c_int) -> c_int;
    fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut c_void);
}

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for CS5530 Audio driver.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for CS5530 Audio driver.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable CS5530 Audio driver.");
static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
}

#[repr(C)]
pub struct snd_sb {
    _private: [u8; 0],
}

pub type irqreturn_t = c_int;

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub dev: *mut device,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct pci_device_id {
    pub vendor: u32,
    pub device: u32,
    pub subvendor: u32,
    pub subdevice: u32,
    pub class: u32,
    pub class_mask: u32,
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

#[repr(C)]
struct snd_cs5530 {
    card: *mut snd_card,
    pci: *mut pci_dev,
    sb: *mut snd_sb,
    pci_base: c_ulong,
}

static snd_cs5530_ids: [pci_device_id; 2] = [
    pci_device_id {
        vendor: PCI_VENDOR_ID_CYRIX,
        device: PCI_DEVICE_ID_CYRIX_5530_AUDIO,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: 0,
        class_mask: 0,
    },
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
    },
];

// MODULE_DEVICE_TABLE(pci, snd_cs5530_ids);

unsafe fn snd_cs5530_mixer_read(io: c_ulong, mut reg: u8) -> u8 {
    outb(reg, io.wrapping_add(4));
    udelay(20);
    reg = inb(io.wrapping_add(5));
    udelay(20);
    reg
}

unsafe fn snd_cs5530_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip = (*card).private_data as *mut snd_cs5530;
    let sb_base: c_ulong;
    let mut irq: u8;
    let mut dma8: u8;
    let mut dma16: u8 = 0;
    let map: u16;
    let mem: *mut c_void;
    let mut err: c_int;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    (*chip).card = card;
    (*chip).pci = pci;

    mem = pcim_iomap_region(pci, 0, b"CS5530\0".as_ptr() as *const c_char);
    if IS_ERR(mem) {
        return PTR_ERR(mem);
    }
    (*chip).pci_base = pci_resource_start(pci, 0);
    map = readw((mem as *mut u8).add(0x18) as *const c_void);

    /*
     * Map bits
     *		0:1	* 0x20 + 0x200 = sb base
     *		2	sb enable
     *		3	adlib enable
     *		5	MPU enable 0x330
     *		6	MPU enable 0x300
     *
     * The other bits may be used internally so must be masked
     */

    sb_base = 0x220u64.wrapping_add(0x20u64.wrapping_mul((map & 3) as u64)) as c_ulong;

    if (map & (1 << 2)) != 0 {
        dev_info((*card).dev, b"XpressAudio at 0x%lx\n\0".as_ptr() as *const c_char, sb_base);
    } else {
        dev_err((*card).dev, b"Could not find XpressAudio!\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    if (map & (1 << 5)) != 0 {
        dev_info((*card).dev, b"MPU at 0x300\n\0".as_ptr() as *const c_char);
    } else if (map & (1 << 6)) != 0 {
        dev_info((*card).dev, b"MPU at 0x330\n\0".as_ptr() as *const c_char);
    }

    irq = snd_cs5530_mixer_read(sb_base, 0x80) & 0x0f;
    dma8 = snd_cs5530_mixer_read(sb_base, 0x81);

    if (dma8 & 0x20) != 0 {
        dma16 = 5;
    } else if (dma8 & 0x40) != 0 {
        dma16 = 6;
    } else if (dma8 & 0x80) != 0 {
        dma16 = 7;
    } else {
        dev_err((*card).dev, b"No 16bit DMA enabled\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    if (dma8 & 0x01) != 0 {
        dma8 = 0;
    } else if (dma8 & 0o2) != 0 {
        dma8 = 1;
    } else if (dma8 & 0x08) != 0 {
        dma8 = 3;
    } else {
        dev_err((*card).dev, b"No 8bit DMA enabled\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    if (irq & 1) != 0 {
        irq = 9;
    } else if (irq & 2) != 0 {
        irq = 5;
    } else if (irq & 4) != 0 {
        irq = 7;
    } else if (irq & 8) != 0 {
        irq = 10;
    } else {
        dev_err((*card).dev, b"SoundBlaster IRQ not set\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    dev_info(
        (*card).dev,
        b"IRQ: %d DMA8: %d DMA16: %d\n\0".as_ptr() as *const c_char,
        irq as c_int,
        dma8 as c_int,
        dma16 as c_int,
    );

    err = snd_sbdsp_create(
        card,
        sb_base,
        irq,
        snd_sb16dsp_interrupt,
        dma8,
        dma16,
        SB_HW_CS5530,
        &mut (*chip).sb,
    );
    if err < 0 {
        dev_err((*card).dev, b"Could not create SoundBlaster\n\0".as_ptr() as *const c_char);
        return err;
    }

    err = snd_sb16dsp_pcm((*chip).sb, 0);
    if err < 0 {
        dev_err((*card).dev, b"Could not create PCM\n\0".as_ptr() as *const c_char);
        return err;
    }

    err = snd_sbmixer_new((*chip).sb);
    if err < 0 {
        dev_err((*card).dev, b"Could not create Mixer\n\0".as_ptr() as *const c_char);
        return err;
    }

    0
}

unsafe extern "C" fn snd_cs5530_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut snd_cs5530;
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
        core::mem::size_of::<snd_cs5530>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data as *mut snd_cs5530;

    err = snd_cs5530_create(card, pci);
    if err < 0 {
        return err;
    }

    strscpy((*card).driver.as_mut_ptr(), b"CS5530\0".as_ptr() as *const c_char);
    strscpy(
        (*card).shortname.as_mut_ptr(),
        b"CS5530 Audio\0".as_ptr() as *const c_char,
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at 0x%lx\0".as_ptr() as *const c_char,
        (*card).shortname.as_ptr(),
        (*chip).pci_base,
    );

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    0
}

static mut cs5530_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_cs5530_ids.as_ptr(),
    probe: Some(snd_cs5530_probe),
};

// module_pci_driver(cs5530_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
