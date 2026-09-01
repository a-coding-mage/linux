// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX222 V2/Mic PCI soundcards
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

// C includes translated as external dependencies:
// linux/init.h, linux/interrupt.h, linux/pci.h, linux/slab.h, linux/module.h,
// sound/core.h, sound/initval.h, sound/tlv.h, and "vx222.h".

const CARD_NAME: *const i8 = b"VX222\0".as_ptr() as *const i8;

// MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
// MODULE_DESCRIPTION("Digigram VX222 V2/Mic");
// MODULE_LICENSE("GPL");

extern "C" {
    static SNDRV_CARDS: usize;
    static SNDRV_DEFAULT_IDX: [i32; 0];
    static SNDRV_DEFAULT_STR: [*mut i8; 0];
    static SNDRV_DEFAULT_ENABLE_PNP: [bool; 0];
}

static mut index: [i32; SNDRV_CARDS] = SNDRV_DEFAULT_IDX; /* Index 0-MAX */
static mut id: [*mut i8; SNDRV_CARDS] = SNDRV_DEFAULT_STR; /* ID for this card */
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP; /* Enable this card */
static mut mic: [bool; SNDRV_CARDS] = [false; SNDRV_CARDS]; /* microphone */
static mut ibl: [i32; SNDRV_CARDS] = [0; SNDRV_CARDS]; /* microphone */

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "Index value for Digigram " CARD_NAME " soundcard.");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string for Digigram " CARD_NAME " soundcard.");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "Enable Digigram " CARD_NAME " soundcard.");
// module_param_array(mic, bool, NULL, 0444);
// MODULE_PARM_DESC(mic, "Enable Microphone.");
// module_param_array(ibl, int, NULL, 0444);
// MODULE_PARM_DESC(ibl, "Capture IBL size.");

const VX_PCI_VX222_OLD: u32 = 0;
const VX_PCI_VX222_NEW: u32 = 1;

extern "C" {
    static PCI_ANY_ID: u32;
    static VX_TYPE_BOARD: u32;
    static VX_TYPE_V2: u32;
    static VX_TYPE_MIC: u32;
    static VX_ANALOG_OUT_LEVEL_MAX: i32;
    static VX2_AKM_LEVEL_MAX: i32;
    static IRQF_SHARED: u32;
    static KBUILD_MODNAME: *const i8;
    static THIS_MODULE: *mut module;
    static vx222_old_ops: snd_vx_ops;
    static vx222_ops: snd_vx_ops;

    fn PCI_DEVICE_SUB(vendor: u32, device: u32, subvendor: u32, subdevice: u32) -> pci_device_id;
    fn pcim_enable_device(pci: *mut pci_dev) -> i32;
    fn pci_set_master(pci: *mut pci_dev);
    fn snd_vx_create(
        card: *mut snd_card,
        hw: *const snd_vx_hardware,
        ops: *const snd_vx_ops,
        extra_size: usize,
    ) -> *mut vx_core;
    fn to_vx222(chip: *mut vx_core) -> *mut snd_vx222;
    fn pcim_request_all_regions(pci: *mut pci_dev, name: *const i8) -> i32;
    fn pci_resource_start(pci: *mut pci_dev, bar: i32) -> u64;
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: u32,
        handler: irq_handler_t,
        thread_fn: irq_handler_t,
        irqflags: u32,
        devname: *const i8,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn snd_vx_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    fn snd_vx_threaded_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn snd_devm_card_new(
        parent: *mut device,
        idx: i32,
        xid: *mut i8,
        module: *mut module,
        extra_size: i32,
        card_ret: *mut *mut snd_card,
    ) -> i32;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const i8, ...);
    fn snd_vx_setup_firmware(chip: *mut vx_core) -> i32;
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn pci_set_drvdata(pci: *mut pci_dev, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn snd_vx_suspend(chip: *mut vx_core) -> i32;
    fn snd_vx_resume(chip: *mut vx_core) -> i32;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct pci_dev {
    dev: device,
    irq: u32,
}

#[repr(C)]
struct pci_device_id {
    driver_data: usize,
}

#[repr(C)]
struct snd_vx_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct ibl_info {
    size: i32,
}

#[repr(C)]
struct vx_core {
    irq: u32,
    ibl: ibl_info,
    dev: *mut device,
}

#[repr(C)]
struct snd_vx222 {
    core: vx_core,
    pci: *mut pci_dev,
    port: [u64; 2],
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut core::ffi::c_void,
    sync_irq: u32,
    shortname: [i8; 32],
    longname: [i8; 80],
}

#[repr(C)]
struct snd_vx_hardware {
    name: *const i8,
    type_: u32,
    /* hw specs */
    num_codecs: i32,
    num_ins: i32,
    num_outs: i32,
    output_level_max: i32,
    output_level_db_scale: *const u32,
}

#[repr(C)]
struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> i32>,
    resume: Option<unsafe extern "C" fn(*mut device) -> i32>,
}

#[repr(C)]
struct pci_driver_inner {
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct pci_driver {
    name: *const i8,
    id_table: *const pci_device_id,
    probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> i32>,
    driver: pci_driver_inner,
}

type irqreturn_t = i32;
type irq_handler_t = Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t>;

const ENOMEM: i32 = 12;
const EBUSY: i32 = 16;
const ENODEV: i32 = 19;
const ENOENT: i32 = 2;

// DECLARE_TLV_DB_SCALE(db_scale_old_vol, -11350, 50, 0);
static db_scale_old_vol: [u32; 4] = [2, 2, (-11350i32) as u32, 50];
// DECLARE_TLV_DB_SCALE(db_scale_akm, -7350, 50, 0);
static db_scale_akm: [u32; 4] = [2, 2, (-7350i32) as u32, 50];

static vx222_old_hw: snd_vx_hardware = snd_vx_hardware {
    name: b"VX222/Old\0".as_ptr() as *const i8,
    type_: unsafe { VX_TYPE_BOARD },
    /* hw specs */
    num_codecs: 1,
    num_ins: 1,
    num_outs: 1,
    output_level_max: unsafe { VX_ANALOG_OUT_LEVEL_MAX },
    output_level_db_scale: db_scale_old_vol.as_ptr(),
};

static vx222_v2_hw: snd_vx_hardware = snd_vx_hardware {
    name: b"VX222/v2\0".as_ptr() as *const i8,
    type_: unsafe { VX_TYPE_V2 },
    /* hw specs */
    num_codecs: 1,
    num_ins: 1,
    num_outs: 1,
    output_level_max: unsafe { VX2_AKM_LEVEL_MAX },
    output_level_db_scale: db_scale_akm.as_ptr(),
};

static vx222_mic_hw: snd_vx_hardware = snd_vx_hardware {
    name: b"VX222/Mic\0".as_ptr() as *const i8,
    type_: unsafe { VX_TYPE_MIC },
    /* hw specs */
    num_codecs: 1,
    num_ins: 1,
    num_outs: 1,
    output_level_max: unsafe { VX2_AKM_LEVEL_MAX },
    output_level_db_scale: db_scale_akm.as_ptr(),
};

static snd_vx222_ids: [pci_device_id; 3] = [
    pci_device_id {
        driver_data: VX_PCI_VX222_OLD as usize,
        /* PLX */
    },
    pci_device_id {
        driver_data: VX_PCI_VX222_NEW as usize,
        /* PLX */
    },
    pci_device_id { driver_data: 0 },
];

// MODULE_DEVICE_TABLE(pci, snd_vx222_ids);

unsafe extern "C" fn snd_vx222_create(
    card: *mut snd_card,
    pci: *mut pci_dev,
    hw: *const snd_vx_hardware,
    rchip: *mut *mut snd_vx222,
) -> i32 {
    let mut chip: *mut vx_core;
    let mut vx: *mut snd_vx222;
    let mut i: i32;
    let mut err: i32;
    let vx_ops: *const snd_vx_ops;

    /* enable PCI device */
    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    pci_set_master(pci);

    vx_ops = if (*hw).type_ == VX_TYPE_BOARD {
        &vx222_old_ops
    } else {
        &vx222_ops
    };
    chip = snd_vx_create(
        card,
        hw,
        vx_ops,
        core::mem::size_of::<snd_vx222>() - core::mem::size_of::<vx_core>(),
    );
    if chip.is_null() {
        return -ENOMEM;
    }
    vx = to_vx222(chip);
    (*vx).pci = pci;

    err = pcim_request_all_regions(pci, KBUILD_MODNAME);
    if err < 0 {
        return err;
    }
    i = 0;
    while i < 2 {
        (*vx).port[i as usize] = pci_resource_start(pci, i + 1);
        i += 1;
    }

    if devm_request_threaded_irq(
        &mut (*pci).dev,
        (*pci).irq,
        Some(snd_vx_irq_handler),
        Some(snd_vx_threaded_irq_handler),
        IRQF_SHARED,
        KBUILD_MODNAME,
        chip as *mut core::ffi::c_void,
    ) != 0
    {
        dev_err(
            (*card).dev,
            b"unable to grab IRQ %d\n\0".as_ptr() as *const i8,
            (*pci).irq,
        );
        return -EBUSY;
    }
    (*chip).irq = (*pci).irq;
    (*card).sync_irq = (*chip).irq;
    *rchip = vx;

    0
}

unsafe extern "C" fn snd_vx222_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> i32 {
    static mut dev: i32 = 0;
    let mut card: *mut snd_card = core::ptr::null_mut();
    let hw: *const snd_vx_hardware;
    let mut vx: *mut snd_vx222 = core::ptr::null_mut();
    let mut err: i32;

    if dev >= SNDRV_CARDS as i32 {
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
        0,
        &mut card,
    );
    if err < 0 {
        return err;
    }

    match (*pci_id).driver_data as i32 {
        x if x == VX_PCI_VX222_OLD as i32 => {
            hw = &vx222_old_hw;
        }
        _ => {
            if mic[dev as usize] {
                hw = &vx222_mic_hw;
            } else {
                hw = &vx222_v2_hw;
            }
        }
    }
    err = snd_vx222_create(card, pci, hw, &mut vx);
    if err < 0 {
        return err;
    }
    (*card).private_data = vx as *mut core::ffi::c_void;
    (*vx).core.ibl.size = ibl[dev as usize];

    sprintf(
        (*card).longname.as_mut_ptr(),
        b"%s at 0x%lx & 0x%lx, irq %i\0".as_ptr() as *const i8,
        (*card).shortname.as_ptr(),
        (*vx).port[0],
        (*vx).port[1],
        (*vx).core.irq,
    );
    dev_dbg(
        (*card).dev,
        b"%s at 0x%lx & 0x%lx, irq %i\n\0".as_ptr() as *const i8,
        (*card).shortname.as_ptr(),
        (*vx).port[0],
        (*vx).port[1],
        (*vx).core.irq,
    );

    // #ifdef SND_VX_FW_LOADER
    (*vx).core.dev = &mut (*pci).dev;
    // #endif

    err = snd_vx_setup_firmware(&mut (*vx).core);
    if err < 0 {
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        return err;
    }

    pci_set_drvdata(pci, card as *mut core::ffi::c_void);
    dev += 1;
    0
}

unsafe extern "C" fn snd_vx222_suspend(dev: *mut device) -> i32 {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let vx: *mut snd_vx222 = (*card).private_data as *mut snd_vx222;

    snd_vx_suspend(&mut (*vx).core)
}

unsafe extern "C" fn snd_vx222_resume(dev: *mut device) -> i32 {
    let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
    let vx: *mut snd_vx222 = (*card).private_data as *mut snd_vx222;

    snd_vx_resume(&mut (*vx).core)
}

static snd_vx222_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(snd_vx222_suspend),
    resume: Some(snd_vx222_resume),
};

static mut vx222_driver: pci_driver = pci_driver {
    name: unsafe { KBUILD_MODNAME },
    id_table: snd_vx222_ids.as_ptr(),
    probe: Some(snd_vx222_probe),
    driver: pci_driver_inner {
        pm: unsafe { pm_ptr(&snd_vx222_pm) },
    },
};

// module_pci_driver(vx222_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
