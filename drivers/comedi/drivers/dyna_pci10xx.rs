// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/dyna_pci10xx.c
 * Copyright (C) 2011 Prashant Shah, pshah.mumbai@gmail.com
 */

/*
 * Driver: dyna_pci10xx
 * Description: Dynalog India PCI DAQ Cards, http://www.dynalogindia.com/
 * Devices: [Dynalog] PCI-1050 (dyna_pci1050)
 * Author: Prashant Shah <pshah.mumbai@gmail.com>
 * Status: Stable
 *
 * Developed at Automation Labs, Chemical Dept., IIT Bombay, India.
 * Prof. Kannan Moudgalya <kannan@iitb.ac.in>
 * http://www.iitb.ac.in
 *
 * Notes :
 * - Dynalog India Pvt. Ltd. does not have a registered PCI Vendor ID and
 *   they are using the PLX Technlogies Vendor ID since that is the PCI Chip
 *   used in the card.
 * - Dynalog India Pvt. Ltd. has provided the internal register specification
 *   for their cards in their manuals.
 */

// External Linux/Comedi declarations supplied by the surrounding crate.

const READ_TIMEOUT: u32 = 50;

static range_pci1050_ai: comedi_lrange = comedi_lrange {
    length: 3,
    range: [BIP_RANGE(10), BIP_RANGE(5), UNI_RANGE(10)],
};

static range_codes_pci1050_ai: [i8; 3] = [0x00, 0x10, 0x30];

#[repr(C)]
struct dyna_pci10xx_private {
    mutex: mutex,
    BADR3: c_ulong,
}

unsafe extern "C" fn dyna_pci10xx_ai_eoc(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    _insn: *mut comedi_insn,
    _context: c_ulong,
) -> c_int {
    let status: c_uint = inw_p((*dev).iobase);
    if status & BIT(15) != 0 {
        return 0;
    }
    -EBUSY
}

unsafe extern "C" fn dyna_pci10xx_insn_read_ai(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut c_uint,
) -> c_int {
    let devpriv = (*dev).private as *mut dyna_pci10xx_private;
    let mut n: c_int;
    let mut d: u16 = 0;
    let mut ret: c_int = 0;
    let chan: c_uint = CR_CHAN((*insn).chanspec);
    let range: c_uint = range_codes_pci1050_ai[CR_RANGE((*insn).chanspec)] as c_uint;

    mutex_lock(&mut (*devpriv).mutex);
    n = 0;
    while n < (*insn).n as c_int {
        smp_mb();
        outw_p(0x0000u16.wrapping_add(range as u16).wrapping_add(chan as u16), (*dev).iobase + 2);
        usleep_range(10, 20);

        ret = comedi_timeout(dev, s, insn, Some(dyna_pci10xx_ai_eoc), 0);
        if ret != 0 {
            break;
        }

        d = inw_p((*dev).iobase);
        d &= 0x0fff;
        *data.add(n as usize) = d as c_uint;
        n += 1;
    }
    mutex_unlock(&mut (*devpriv).mutex);

    if ret != 0 { ret } else { n }
}

unsafe extern "C" fn dyna_pci10xx_insn_write_ao(
    dev: *mut comedi_device,
    _s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut c_uint,
) -> c_int {
    let devpriv = (*dev).private as *mut dyna_pci10xx_private;
    let mut n: c_int = 0;

    mutex_lock(&mut (*devpriv).mutex);
    while n < (*insn).n as c_int {
        smp_mb();
        outw_p(*data.add(n as usize) as u16, (*dev).iobase);
        usleep_range(10, 20);
        n += 1;
    }
    mutex_unlock(&mut (*devpriv).mutex);
    n
}

unsafe extern "C" fn dyna_pci10xx_di_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut c_uint,
) -> c_int {
    let devpriv = (*dev).private as *mut dyna_pci10xx_private;
    let mut d: u16 = 0;

    mutex_lock(&mut (*devpriv).mutex);
    smp_mb();
    d = inw_p((*devpriv).BADR3);
    usleep_range(10, 100);
    *data.add(1) = d as c_uint;
    *data = (*s).state;
    mutex_unlock(&mut (*devpriv).mutex);
    (*insn).n as c_int
}

unsafe extern "C" fn dyna_pci10xx_do_insn_bits(
    dev: *mut comedi_device,
    s: *mut comedi_subdevice,
    insn: *mut comedi_insn,
    data: *mut c_uint,
) -> c_int {
    let devpriv = (*dev).private as *mut dyna_pci10xx_private;

    mutex_lock(&mut (*devpriv).mutex);
    if comedi_dio_update_state(s, data) != 0 {
        smp_mb();
        outw_p((*s).state as u16, (*devpriv).BADR3);
        usleep_range(10, 100);
    }
    *data.add(1) = (*s).state;
    mutex_unlock(&mut (*devpriv).mutex);
    (*insn).n as c_int
}

unsafe extern "C" fn dyna_pci10xx_auto_attach(
    dev: *mut comedi_device,
    _context_unused: c_ulong,
) -> c_int {
    let pcidev = comedi_to_pci_dev(dev);
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<dyna_pci10xx_private>());
    if devpriv.is_null() { return -ENOMEM; }

    let mut ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 2);
    (*(devpriv as *mut dyna_pci10xx_private)).BADR3 = pci_resource_start(pcidev, 3);
    mutex_init(&mut (*(devpriv as *mut dyna_pci10xx_private)).mutex);

    ret = comedi_alloc_subdevices(dev, 4);
    if ret != 0 { return ret; }

    let s = (*dev).subdevices.add(0);
    (*s).type_ = COMEDI_SUBD_AI;
    (*s).subdev_flags = SDF_READABLE | SDF_GROUND | SDF_DIFF;
    (*s).n_chan = 16;
    (*s).maxdata = 0x0fff;
    (*s).range_table = &range_pci1050_ai;
    (*s).insn_read = Some(dyna_pci10xx_insn_read_ai);

    let s = (*dev).subdevices.add(1);
    (*s).type_ = COMEDI_SUBD_AO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 1;
    (*s).maxdata = 0x0fff;
    (*s).range_table = &range_unipolar10;
    (*s).insn_write = Some(dyna_pci10xx_insn_write_ao);

    let s = (*dev).subdevices.add(2);
    (*s).type_ = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(dyna_pci10xx_di_insn_bits);

    let s = (*dev).subdevices.add(3);
    (*s).type_ = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 16;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).state = 0;
    (*s).insn_bits = Some(dyna_pci10xx_do_insn_bits);
    0
}

unsafe extern "C" fn dyna_pci10xx_detach(dev: *mut comedi_device) {
    let devpriv = (*dev).private as *mut dyna_pci10xx_private;
    comedi_pci_detach(dev);
    if !devpriv.is_null() {
        mutex_destroy(&mut (*devpriv).mutex);
    }
}

static mut dyna_pci10xx_driver: comedi_driver = comedi_driver {
    driver_name: b"dyna_pci10xx\0".as_ptr() as *const c_char,
    module: THIS_MODULE,
    auto_attach: Some(dyna_pci10xx_auto_attach),
    detach: Some(dyna_pci10xx_detach),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
