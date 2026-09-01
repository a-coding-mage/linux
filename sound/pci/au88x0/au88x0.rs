// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA driver for the Aureal Vortex family of soundprocessors.
 * Author: Manuel Jander (mjander@embedded.cl)
 *
 *   This driver is the result of the OpenVortex Project from Savannah
 * (savannah.nongnu.org/projects/openvortex). I would like to thank
 * the developers of OpenVortex, Jeff Muizelaar and Kester Maddock, from
 * whom i got plenty of help, and their codebase was invaluable.
 *   Thanks to the ALSA developers, they helped a lot working out
 * the ALSA part.
 *   Thanks also to Sourceforge for maintaining the old binary drivers,
 * and the forum, where developers could communicate.
 *
 * Now at least i can play Legacy DOOM with MIDI music :-)
 */

// Dependencies from au88x0.h, Linux PCI/module/DMA APIs, and ALSA headers are
// expected to be supplied by the surrounding translation unit or bindings.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    static snd_vortex_ids: pci_device_id;
    static THIS_MODULE: c_void;
    static KBUILD_MODNAME: c_char;
    static CARD_NAME: c_char;
    static CARD_NAME_SHORT: c_char;

    fn pci_write_config_byte(dev: *mut pci_dev, where_: c_int, val: u8) -> c_int;
    fn pci_read_config_byte(dev: *mut pci_dev, where_: c_int, val: *mut u8) -> c_int;
    fn pci_get_device(vendor: c_uint, device: c_uint, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_dev_put(dev: *mut pci_dev);
    fn pcim_enable_device(dev: *mut pci_dev) -> c_int;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn pcim_iomap_region(dev: *mut pci_dev, bar: c_int, name: *const c_char) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn pci_resource_start(dev: *mut pci_dev, bar: c_int) -> c_ulong;
    fn vortex_core_init(vortex: *mut vortex_t) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn vortex_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    fn pci_set_master(dev: *mut pci_dev);
    fn snd_devm_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *const c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn vortex_gameport_unregister(vortex: *mut vortex_t);
    fn vortex_core_shutdown(vortex: *mut vortex_t);
    fn snd_vortex_mixer(vortex: *mut vortex_t) -> c_int;
    fn snd_vortex_new_pcm(vortex: *mut vortex_t, pcm: c_int, nr: c_int) -> c_int;
    fn snd_vortex_midi(vortex: *mut vortex_t) -> c_int;
    fn vortex_gameport_register(vortex: *mut vortex_t);
    fn pci_read_config_word(dev: *mut pci_dev, where_: c_int, val: *mut u16) -> c_int;
    fn pcibios_err_to_errno(err: c_int) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn pci_set_drvdata(dev: *mut pci_dev, data: *mut c_void);
    fn vortex_connect_default(vortex: *mut vortex_t, en: c_int);
    fn vortex_enable_int(vortex: *mut vortex_t);
    fn snd_card_free_on_error(dev: *mut device, err: c_int) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_alert(dev: *mut device, fmt: *const c_char, ...);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub vendor: u16,
    pub device: u16,
    pub irq: c_uint,
    pub revision: u8,
}

#[repr(C)]
pub struct pci_device_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut vortex_t,
    pub dev: *mut device,
    pub sync_irq: c_int,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub longname: [c_char; 80],
}

#[repr(C)]
pub struct vortex_t {
    pub card: *mut snd_card,
    pub pci_dev: *mut pci_dev,
    pub vendor: u16,
    pub device: u16,
    pub irq: c_int,
    pub mmio: *mut c_void,
    pub io: c_ulong,
    pub rev: u8,
}

pub type irqreturn_t = c_int;

const SNDRV_CARDS: usize = 8;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool; SNDRV_CARDS] = [true; SNDRV_CARDS];
const PCI_VENDOR_ID_VIA: c_uint = 0x1106;
const PCI_DEVICE_ID_VIA_8365_1: c_uint = 0x8305;
const PCI_DEVICE_ID_VIA_82C598_1: c_uint = 0x8598;
const PCI_VENDOR_ID_AMD: c_uint = 0x1022;
const PCI_DEVICE_ID_AMD_FE_GATE_7007: c_uint = 0x7007;
const DMA_BIT_MASK_32: u64 = (1u64 << 32) - 1;
const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IRQF_SHARED: c_ulong = 0x80;
const PCI_DEVICE_ID: c_int = 0x02;
const PCI_VENDOR_ID: c_int = 0x00;
const VORTEX_PCM_ADB: c_int = 0;
const VORTEX_PCM_SPDIF: c_int = 1;
const VORTEX_PCM_A3D: c_int = 2;
const VORTEX_PCM_WT: c_int = 3;
const NR_PCM: c_int = 16;
const NR_A3D: c_int = 4;
const NR_WT: c_int = 32;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
static mut pcifix: [c_int; SNDRV_CARDS] = [255; SNDRV_CARDS];

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable " CARD_NAME " soundcard.");
// module_param_array(pcifix, int, NULL, 0444);
// MODULE_PARM_DESC(pcifix, "Enable VIA-workaround for " CARD_NAME " soundcard.");
//
// MODULE_DESCRIPTION("Aureal vortex");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, snd_vortex_ids);

unsafe extern "C" fn vortex_fix_latency(vortex: *mut pci_dev) {
    let mut rc: c_int;
    rc = pci_write_config_byte(vortex, 0x40, 0xff);
    if rc == 0 {
        dev_info(
            &mut (*vortex).dev,
            b"vortex latency is 0xff\n\0".as_ptr() as *const c_char,
        );
    } else {
        dev_warn(
            &mut (*vortex).dev,
            b"could not set vortex latency: pci error 0x%x\n\0".as_ptr() as *const c_char,
            rc,
        );
    }
}

unsafe extern "C" fn vortex_fix_agp_bridge(via: *mut pci_dev) {
    let mut rc: c_int;
    let mut value: u8 = 0;

    /*
     * only set the bit (Extend PCI#2 Internal Master for
     * Efficient Handling of Dummy Requests) if the can
     * read the config and it is not already set
     */

    rc = pci_read_config_byte(via, 0x42, &mut value);
    if rc == 0 {
        if (value & 0x10) == 0 {
            rc = pci_write_config_byte(via, 0x42, value | 0x10);
        }
    }
    if rc == 0 {
        dev_info(
            &mut (*via).dev,
            b"bridge config is 0x%x\n\0".as_ptr() as *const c_char,
            value | 0x10,
        );
    } else {
        dev_warn(
            &mut (*via).dev,
            b"could not set vortex latency: pci error 0x%x\n\0".as_ptr() as *const c_char,
            rc,
        );
    }
}

unsafe extern "C" fn snd_vortex_workaround(vortex: *mut pci_dev, fix: c_int) {
    let mut via: *mut pci_dev = ptr::null_mut();

    /* autodetect if workarounds are required */
    if fix == 255 {
        /* VIA KT133 */
        via = pci_get_device(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_8365_1, ptr::null_mut());
        /* VIA Apollo */
        if via.is_null() {
            via = pci_get_device(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C598_1, ptr::null_mut());
            /* AMD Irongate */
            if via.is_null() {
                via = pci_get_device(
                    PCI_VENDOR_ID_AMD,
                    PCI_DEVICE_ID_AMD_FE_GATE_7007,
                    ptr::null_mut(),
                );
            }
        }
        if !via.is_null() {
            dev_info(
                &mut (*vortex).dev,
                b"Activating latency workaround...\n\0".as_ptr() as *const c_char,
            );
            vortex_fix_latency(vortex);
            vortex_fix_agp_bridge(via);
        }
    } else {
        if (fix & 0x1) != 0 {
            vortex_fix_latency(vortex);
        }
        if (fix & 0x2) != 0 {
            via = pci_get_device(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_8365_1, ptr::null_mut());
        } else if (fix & 0x4) != 0 {
            via = pci_get_device(PCI_VENDOR_ID_VIA, PCI_DEVICE_ID_VIA_82C598_1, ptr::null_mut());
        } else if (fix & 0x8) != 0 {
            via = pci_get_device(
                PCI_VENDOR_ID_AMD,
                PCI_DEVICE_ID_AMD_FE_GATE_7007,
                ptr::null_mut(),
            );
        }
        if !via.is_null() {
            vortex_fix_agp_bridge(via);
        }
    }
    pci_dev_put(via);
}

// component-destructor
// (see "Management of Cards and Components")
unsafe extern "C" fn snd_vortex_free(card: *mut snd_card) {
    let vortex: *mut vortex_t = (*card).private_data;

    vortex_gameport_unregister(vortex);
    vortex_core_shutdown(vortex);
}

// chip-specific constructor
// (see "Management of Cards and Components")
unsafe extern "C" fn snd_vortex_create(card: *mut snd_card, pci: *mut pci_dev) -> c_int {
    let chip: *mut vortex_t = (*card).private_data;
    let mut err: c_int;

    // check PCI availability (DMA).
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK_32) != 0 {
        dev_err((*card).dev, b"error to set DMA mask\n\0".as_ptr() as *const c_char);
        return -ENXIO;
    }

    (*chip).card = card;

    // initialize the stuff
    (*chip).pci_dev = pci;
    (*chip).vendor = (*pci).vendor;
    (*chip).device = (*pci).device;
    (*chip).card = card;
    (*chip).irq = -1;

    // (1) PCI resource allocation
    // Get MMIO area
    //
    (*chip).mmio = pcim_iomap_region(pci, 0, &KBUILD_MODNAME as *const c_char);
    if IS_ERR((*chip).mmio) {
        return PTR_ERR((*chip).mmio);
    }

    (*chip).io = pci_resource_start(pci, 0);

    /* Init audio core.
     * This must be done before we do request_irq otherwise we can get spurious
     * interrupts that we do not handle properly and make a mess of things */
    err = vortex_core_init(chip);
    if err != 0 {
        dev_err((*card).dev, b"hw core init failed\n\0".as_ptr() as *const c_char);
        return err;
    }

    err = devm_request_irq(
        &mut (*pci).dev,
        (*pci).irq,
        vortex_interrupt,
        IRQF_SHARED,
        &KBUILD_MODNAME as *const c_char,
        chip as *mut c_void,
    );
    if err != 0 {
        dev_err((*card).dev, b"cannot grab irq\n\0".as_ptr() as *const c_char);
        return err;
    }
    (*chip).irq = (*pci).irq as c_int;
    (*card).sync_irq = (*chip).irq;
    (*card).private_free = Some(snd_vortex_free);

    pci_set_master(pci);
    // End of PCI setup.
    0
}

// constructor -- see "Constructor" sub-section
unsafe extern "C" fn __snd_vortex_probe(
    pci: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    static mut dev: c_int = 0;
    let mut card: *mut snd_card = ptr::null_mut();
    let mut chip: *mut vortex_t;
    let mut err: c_int;

    // (1)
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        dev += 1;
        return -ENOENT;
    }
    // (2)
    err = snd_devm_card_new(
        &mut (*pci).dev,
        index[dev as usize],
        id[dev as usize],
        &THIS_MODULE as *const c_void,
        size_of::<vortex_t>(),
        &mut card,
    );
    if err < 0 {
        return err;
    }
    chip = (*card).private_data;

    // (3)
    err = snd_vortex_create(card, pci);
    if err < 0 {
        return err;
    }
    snd_vortex_workaround(pci, pcifix[dev as usize]);

    // Card details needed in snd_vortex_midi
    strscpy((*card).driver.as_mut_ptr(), &CARD_NAME_SHORT as *const c_char);
    sprintf(
        (*card).shortname.as_mut_ptr(),
        b"Aureal Vortex %s\0".as_ptr() as *const c_char,
        &CARD_NAME_SHORT as *const c_char,
    );
    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at 0x%lx irq %i\0".as_ptr() as *const c_char,
        (*card).shortname.as_mut_ptr(),
        (*chip).io,
        (*chip).irq,
    );

    // (4) Alloc components.
    err = snd_vortex_mixer(chip);
    if err < 0 {
        return err;
    }
    // ADB pcm.
    err = snd_vortex_new_pcm(chip, VORTEX_PCM_ADB, NR_PCM);
    if err < 0 {
        return err;
    }
    #[cfg(not(CHIP_AU8820))]
    {
        // ADB SPDIF
        err = snd_vortex_new_pcm(chip, VORTEX_PCM_SPDIF, 1);
        if err < 0 {
            return err;
        }
        // A3D
        err = snd_vortex_new_pcm(chip, VORTEX_PCM_A3D, NR_A3D);
        if err < 0 {
            return err;
        }
    }
    /*
       // ADB I2S
       if ((err = snd_vortex_new_pcm(chip, VORTEX_PCM_I2S, 1)) < 0) {
       return err;
       }
     */
    #[cfg(not(CHIP_AU8810))]
    {
        // WT pcm.
        err = snd_vortex_new_pcm(chip, VORTEX_PCM_WT, NR_WT);
        if err < 0 {
            return err;
        }
    }
    err = snd_vortex_midi(chip);
    if err < 0 {
        return err;
    }

    vortex_gameport_register(chip);

    // Original C has this block disabled with #if 0.
    /*
    if snd_seq_device_new(card, 1, SNDRV_SEQ_DEV_ID_VORTEX_SYNTH,
                          size_of::<snd_vortex_synth_arg_t>(), &mut wave) < 0
        || wave.is_null()
    {
        dev_err((*card).dev, b"Can't initialize Aureal wavetable synth\n\0".as_ptr() as *const c_char);
    } else {
        let arg: *mut snd_vortex_synth_arg_t;

        arg = SNDRV_SEQ_DEVICE_ARGPTR(wave);
        strscpy((*wave).name.as_mut_ptr(), b"Aureal Synth\0".as_ptr() as *const c_char);
        (*arg).hwptr = vortex;
        (*arg).index = 1;
        (*arg).seq_ports = seq_ports[dev as usize];
        (*arg).max_voices = max_synth_voices[dev as usize];
    }
    */

    // (5)
    err = pci_read_config_word(pci, PCI_DEVICE_ID, &mut (*chip).device);
    if err != 0 {
        return pcibios_err_to_errno(err);
    }
    err = pci_read_config_word(pci, PCI_VENDOR_ID, &mut (*chip).vendor);
    if err != 0 {
        return pcibios_err_to_errno(err);
    }
    (*chip).rev = (*pci).revision;
    #[cfg(CHIP_AU8830)]
    {
        if (*chip).rev != 0xfe && (*chip).rev != 0xfa {
            dev_alert(
                (*card).dev,
                b"The revision (%x) of your card has not been seen before.\n\0".as_ptr()
                    as *const c_char,
                (*chip).rev as c_uint,
            );
            dev_alert(
                (*card).dev,
                b"Please email the results of 'lspci -vv' to openvortex-dev@nongnu.org.\n\0"
                    .as_ptr() as *const c_char,
            );
            return -ENODEV;
        }
    }

    // (6)
    err = snd_card_register(card);
    if err < 0 {
        return err;
    }
    // (7)
    pci_set_drvdata(pci, card as *mut c_void);
    dev += 1;
    vortex_connect_default(chip, 1);
    vortex_enable_int(chip);
    0
}

unsafe extern "C" fn snd_vortex_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    snd_card_free_on_error(&mut (*pci).dev, __snd_vortex_probe(pci, pci_id))
}

#[repr(C)]
pub struct pci_driver {
    pub name: *const c_char,
    pub id_table: *const pci_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>,
}

// pci_driver definition
static mut vortex_driver: pci_driver = pci_driver {
    name: unsafe { &KBUILD_MODNAME as *const c_char },
    id_table: unsafe { &snd_vortex_ids as *const pci_device_id },
    probe: Some(snd_vortex_probe),
};

// module_pci_driver(vortex_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
