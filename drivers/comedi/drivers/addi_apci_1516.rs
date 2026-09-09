// SPDX-License-Identifier: GPL-2.0+
/*
 * addi_apci_1516.c
 * Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
 * Project manager: Eric Stolz
 *
 *	ADDI-DATA GmbH
 *	Dieselstrasse 3
 *	D-77833 Ottersweier
 *	Tel: +19(0)7223/9493-0
 *	Fax: +49(0)7223/9493-92
 *	http://www.addi-data.com
 *	info@addi-data.com
 */

// External kernel, Comedi, PCI, and addi_watchdog declarations are supplied by
// the surrounding translation unit.

/* PCI bar 1 I/O Register map - Digital input/output */
const APCI1516_DI_REG: usize = 0x00;
const APCI1516_DO_REG: usize = 0x04;

/* PCI bar 2 I/O Register map - Watchdog (APCI-1516 and APCI-2016) */
const APCI1516_WDOG_REG: usize = 0x00;

#[repr(C)]
#[derive(Copy, Clone)]
enum Apci1516Boardid {
    BOARD_APCI1016,
    BOARD_APCI1516,
    BOARD_APCI2016,
}

#[repr(C)]
struct Apci1516Boardinfo {
    name: *const core::ffi::c_char,
    di_nchan: i32,
    do_nchan: i32,
    has_wdog: i32,
}

static APCI1516_BOARDTYPES: [Apci1516Boardinfo; 3] = [
    Apci1516Boardinfo {
        name: b"apci1016\0".as_ptr() as *const core::ffi::c_char,
        di_nchan: 16,
        do_nchan: 0,
        has_wdog: 0,
    },
    Apci1516Boardinfo {
        name: b"apci1516\0".as_ptr() as *const core::ffi::c_char,
        di_nchan: 8,
        do_nchan: 8,
        has_wdog: 1,
    },
    Apci1516Boardinfo {
        name: b"apci2016\0".as_ptr() as *const core::ffi::c_char,
        di_nchan: 0,
        do_nchan: 16,
        has_wdog: 1,
    },
];

#[repr(C)]
struct Apci1516Private {
    wdog_iobase: usize,
}

unsafe fn apci1516_di_insn_bits(
    dev: *mut ComediDevice,
    _s: *mut ComediSubdevice,
    insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    (*data.add(1)) = inw((*dev).iobase + APCI1516_DI_REG) as u32;
    (*insn).n as i32
}

unsafe fn apci1516_do_insn_bits(
    dev: *mut ComediDevice,
    s: *mut ComediSubdevice,
    insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    (*s).state = inw((*dev).iobase + APCI1516_DO_REG) as u32;
    if comedi_dio_update_state(s, data) != 0 {
        outw((*s).state as u16, (*dev).iobase + APCI1516_DO_REG);
    }
    (*data.add(1)) = (*s).state;
    (*insn).n as i32
}

unsafe fn apci1516_reset(dev: *mut ComediDevice) -> i32 {
    let board = (*dev).board_ptr as *const Apci1516Boardinfo;
    let devpriv = (*dev).private as *mut Apci1516Private;
    if (*board).has_wdog == 0 {
        return 0;
    }
    outw(0, (*dev).iobase + APCI1516_DO_REG);
    addi_watchdog_reset((*devpriv).wdog_iobase);
    0
}

unsafe fn apci1516_auto_attach(dev: *mut ComediDevice, context: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let board: *const Apci1516Boardinfo;
    let devpriv: *mut Apci1516Private;
    let mut s: *mut ComediSubdevice;
    let mut ret: i32;

    if context < APCI1516_BOARDTYPES.len() {
        board = &APCI1516_BOARDTYPES[context];
    } else {
        board = core::ptr::null();
    }
    if board.is_null() {
        return -ENODEV;
    }
    (*dev).board_ptr = board as *mut _;
    (*dev).board_name = (*board).name;
    devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<Apci1516Private>()) as *mut Apci1516Private;
    if devpriv.is_null() {
        return -ENOMEM;
    }
    ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 1);
    (*devpriv).wdog_iobase = pci_resource_start(pcidev, 2);
    ret = comedi_alloc_subdevices(dev, 3);
    if ret != 0 { return ret; }

    // Initialize the digital input subdevice
    s = (*dev).subdevices.add(0);
    if (*board).di_nchan != 0 {
        (*s).type_ = COMEDI_SUBD_DI;
        (*s).subdev_flags = SDF_READABLE;
        (*s).n_chan = (*board).di_nchan as u32;
        (*s).maxdata = 1;
        (*s).range_table = &range_digital;
        (*s).insn_bits = Some(apci1516_di_insn_bits);
    } else { (*s).type_ = COMEDI_SUBD_UNUSED; }

    // Initialize the digital output subdevice
    s = (*dev).subdevices.add(1);
    if (*board).do_nchan != 0 {
        (*s).type_ = COMEDI_SUBD_DO;
        (*s).subdev_flags = SDF_WRITABLE;
        (*s).n_chan = (*board).do_nchan as u32;
        (*s).maxdata = 1;
        (*s).range_table = &range_digital;
        (*s).insn_bits = Some(apci1516_do_insn_bits);
    } else { (*s).type_ = COMEDI_SUBD_UNUSED; }

    // Initialize the watchdog subdevice
    s = (*dev).subdevices.add(2);
    if (*board).has_wdog != 0 {
        ret = addi_watchdog_init(s, (*devpriv).wdog_iobase);
        if ret != 0 { return ret; }
    } else { (*s).type_ = COMEDI_SUBD_UNUSED; }
    apci1516_reset(dev);
    0
}

unsafe fn apci1516_detach(dev: *mut ComediDevice) {
    if (*dev).iobase != 0 { apci1516_reset(dev); }
    comedi_pci_detach(dev);
}

// The following driver objects and PCI table correspond directly to the C
// registration declarations; their concrete surrounding types are external.
static mut APCI1516_DRIVER: ComediDriver = ComediDriver {
    driver_name: b"addi_apci_1516\0".as_ptr() as *const core::ffi::c_char,
    module_: THIS_MODULE,
    auto_attach: Some(apci1516_auto_attach),
    detach: Some(apci1516_detach),
};

unsafe fn apci1516_pci_probe(dev: *mut PciDev, id: *const PciDeviceId) -> i32 {
    comedi_pci_auto_config(dev, &mut APCI1516_DRIVER, (*id).driver_data)
}

static APCI1516_PCI_TABLE: [PciDeviceId; 4] = [
    PciDeviceId::vdevice(ADDIDATA, 0x1000, BOARD_APCI1016 as usize),
    PciDeviceId::vdevice(ADDIDATA, 0x1001, BOARD_APCI1516 as usize),
    PciDeviceId::vdevice(ADDIDATA, 0x1002, BOARD_APCI2016 as usize),
    PciDeviceId::default(),
];

static mut APCI1516_PCI_DRIVER: PciDriver = PciDriver {
    name: b"addi_apci_1516\0".as_ptr() as *const core::ffi::c_char,
    id_table: APCI1516_PCI_TABLE.as_ptr(),
    probe: Some(apci1516_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// MODULE_DEVICE_TABLE(pci, apci1516_pci_table);
// module_comedi_pci_driver(apci1516_driver, apci1516_pci_driver);
// MODULE_DESCRIPTION("ADDI-DATA APCI-1016/1516/2016, 16 channel DIO boards");
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
