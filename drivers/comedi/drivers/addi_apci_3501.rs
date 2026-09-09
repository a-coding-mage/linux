// SPDX-License-Identifier: GPL-2.0+
/*
 * addi_apci_3501.c
 * Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
 * Project manager: Eric Stolz
 *
 * Driver: addi_apci_3501
 * Description: ADDI-DATA APCI-3501 Analog output board
 * Devices: [ADDI-DATA] APCI-3501 (addi_apci_3501)
 * Author: H Hartley Sweeten <hsweeten@visionengravers.com>
 * Updated: Mon, 20 Jun 2016 10:57:01 -0700
 * Status: untested
 *
 * This board has 4 or 8 analog output channels, 2 digital inputs,
 * 2 digital outputs, and one 12-bit watchdog/timer.
 */

// External Linux, Comedi, and AMCC S5933 declarations are supplied by dependencies.

const APCI3501_AO_CTRL_STATUS_REG: u32 = 0x00;
const APCI3501_AO_CTRL_BIPOLAR: u32 = 1 << 0;
const APCI3501_AO_STATUS_READY: u32 = 1 << 8;
const APCI3501_AO_DATA_REG: u32 = 0x04;
#[inline]
const fn apci3501_ao_data_chan(x: u32) -> u32 { x << 0 }
#[inline]
const fn apci3501_ao_data_val(x: u32) -> u32 { x << 8 }
const APCI3501_AO_DATA_BIPOLAR: u32 = 1 << 31;
const APCI3501_AO_TRIG_SCS_REG: u32 = 0x08;
const APCI3501_TIMER_BASE: u32 = 0x20;
const APCI3501_DO_REG: u32 = 0x40;
const APCI3501_DI_REG: u32 = 0x50;

const NVRAM_USER_DATA_START: u16 = 0x100;
const NVCMD_BEGIN_READ: u8 = 0x7 << 5;
const NVCMD_LOAD_LOW: u8 = 0x4 << 5;
const NVCMD_LOAD_HIGH: u8 = 0x5 << 5;

const EEPROM_DIGITALINPUT: u8 = 0;
const EEPROM_DIGITALOUTPUT: u8 = 1;
const EEPROM_ANALOGINPUT: u8 = 2;
const EEPROM_ANALOGOUTPUT: u8 = 3;
const EEPROM_TIMER: u8 = 4;
const EEPROM_WATCHDOG: u8 = 5;
const EEPROM_TIMER_WATCHDOG_COUNTER: u8 = 10;

#[repr(C)]
struct apci3501_private {
    amcc: ::core::ffi::c_ulong,
    timer_mode: u8,
}

static mut apci3501_ao_range: comedi_lrange = comedi_lrange {
    length: 2,
    range: [BIP_RANGE(10), UNI_RANGE(10)],
};

unsafe fn apci3501_wait_for_dac(dev: *mut comedi_device) -> i32 {
    let mut status: u32;
    loop {
        status = inl((*dev).iobase + APCI3501_AO_CTRL_STATUS_REG as ::core::ffi::c_ulong);
        if status & APCI3501_AO_STATUS_READY != 0 { break; }
    }
    0
}

unsafe fn apci3501_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                 insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec);
    let range = CR_RANGE((*insn).chanspec);
    let mut cfg = apci3501_ao_data_chan(chan);
    if range != 0 {
        outl(0, (*dev).iobase + APCI3501_AO_CTRL_STATUS_REG as ::core::ffi::c_ulong);
    } else {
        cfg |= APCI3501_AO_DATA_BIPOLAR;
        outl(APCI3501_AO_CTRL_BIPOLAR, (*dev).iobase + APCI3501_AO_CTRL_STATUS_REG as ::core::ffi::c_ulong);
    }
    for i in 0..(*insn).n {
        let val = *data.add(i as usize);
        if range == 1 && val > 0x1fff {
            dev_err((*dev).class_dev, "Unipolar resolution is only 13-bits\n");
            return -EINVAL;
        }
        let ret = apci3501_wait_for_dac(dev);
        if ret != 0 { return ret; }
        outl(cfg | apci3501_ao_data_val(val), (*dev).iobase + APCI3501_AO_DATA_REG as ::core::ffi::c_ulong);
        (*s).readback[chan as usize] = val;
    }
    (*insn).n as i32
}

unsafe fn apci3501_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                insn: *mut comedi_insn, data: *mut u32) -> i32 {
    *data.add(1) = inl((*dev).iobase + APCI3501_DI_REG as ::core::ffi::c_ulong) & 0x3;
    (*insn).n as i32
}

unsafe fn apci3501_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice,
                                insn: *mut comedi_insn, data: *mut u32) -> i32 {
    (*s).state = inl((*dev).iobase + APCI3501_DO_REG as ::core::ffi::c_ulong);
    if comedi_dio_update_state(s, data) != 0 {
        outl((*s).state, (*dev).iobase + APCI3501_DO_REG as ::core::ffi::c_ulong);
    }
    *data.add(1) = (*s).state;
    (*insn).n as i32
}

unsafe fn apci3501_eeprom_wait(iobase: ::core::ffi::c_ulong) {
    let mut val: u8;
    loop {
        val = inb(iobase + AMCC_OP_REG_MCSR_NVCMD as ::core::ffi::c_ulong);
        if val & 0x80 == 0 { break; }
    }
}

unsafe fn apci3501_eeprom_readw(iobase: ::core::ffi::c_ulong, mut addr: u16) -> u16 {
    let mut val: u16 = 0;
    addr = addr.wrapping_add(NVRAM_USER_DATA_START);
    for i in 0..2u16 {
        outb(NVCMD_LOAD_LOW, iobase + AMCC_OP_REG_MCSR_NVCMD as ::core::ffi::c_ulong);
        apci3501_eeprom_wait(iobase);
        outb(((addr + i) & 0xff) as u8, iobase + AMCC_OP_REG_MCSR_NVDATA as ::core::ffi::c_ulong);
        apci3501_eeprom_wait(iobase);
        outb(NVCMD_LOAD_HIGH, iobase + AMCC_OP_REG_MCSR_NVCMD as ::core::ffi::c_ulong);
        apci3501_eeprom_wait(iobase);
        outb((((addr + i) >> 8) & 0xff) as u8, iobase + AMCC_OP_REG_MCSR_NVDATA as ::core::ffi::c_ulong);
        apci3501_eeprom_wait(iobase);
        outb(NVCMD_BEGIN_READ, iobase + AMCC_OP_REG_MCSR_NVCMD as ::core::ffi::c_ulong);
        apci3501_eeprom_wait(iobase);
        let tmp = inb(iobase + AMCC_OP_REG_MCSR_NVDATA as ::core::ffi::c_ulong);
        apci3501_eeprom_wait(iobase);
        if i == 0 { val |= tmp as u16; } else { val |= (tmp as u16) << 8; }
    }
    val
}

unsafe fn apci3501_eeprom_get_ao_n_chan(dev: *mut comedi_device) -> i32 {
    let devpriv = (*dev).private as *mut apci3501_private;
    let nfuncs = (apci3501_eeprom_readw((*devpriv).amcc, 10) & 0xff) as i32;
    for i in 0..nfuncs {
        let offset = (i * 4) as u16;
        let func = (apci3501_eeprom_readw((*devpriv).amcc, 12 + offset) & 0x3f) as u8;
        let addr = apci3501_eeprom_readw((*devpriv).amcc, 14 + offset);
        if func == EEPROM_ANALOGOUTPUT {
            let val = apci3501_eeprom_readw((*devpriv).amcc, addr + 10);
            return ((val >> 4) & 0x3ff) as i32;
        }
    }
    0
}

unsafe fn apci3501_eeprom_insn_read(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                    insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let devpriv = (*dev).private as *mut apci3501_private;
    let addr = CR_CHAN((*insn).chanspec) as u16;
    if (*insn).n != 0 {
        let val = apci3501_eeprom_readw((*devpriv).amcc, 2 * addr) as u32;
        for i in 0..(*insn).n { *data.add(i as usize) = val; }
    }
    (*insn).n as i32
}

unsafe fn apci3501_reset(dev: *mut comedi_device) -> i32 {
    outl(0, (*dev).iobase + APCI3501_DO_REG as ::core::ffi::c_ulong);
    outl(APCI3501_AO_CTRL_BIPOLAR, (*dev).iobase + APCI3501_AO_CTRL_STATUS_REG as ::core::ffi::c_ulong);
    let val = APCI3501_AO_DATA_BIPOLAR | apci3501_ao_data_val(0);
    for chan in 0..8 {
        if apci3501_wait_for_dac(dev) != 0 {
            dev_warn((*dev).class_dev, "%s: DAC not-ready for channel %i\n", __func__, chan);
        } else {
            outl(val | apci3501_ao_data_chan(chan), (*dev).iobase + APCI3501_AO_DATA_REG as ::core::ffi::c_ulong);
        }
    }
    0
}

unsafe fn apci3501_auto_attach(dev: *mut comedi_device, _context_unused: ::core::ffi::c_ulong) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<apci3501_private>()) as *mut apci3501_private;
    if devpriv.is_null() { return -ENOMEM; }
    let ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*devpriv).amcc = pci_resource_start(pcidev, 0);
    (*dev).iobase = pci_resource_start(pcidev, 1);
    let ao_n_chan = apci3501_eeprom_get_ao_n_chan(dev);
    let ret = comedi_alloc_subdevices(dev, 5);
    if ret != 0 { return ret; }

    let mut s = (*dev).subdevices.add(0);
    if ao_n_chan != 0 {
        (*s).type_ = COMEDI_SUBD_AO;
        (*s).subdev_flags = SDF_WRITABLE | SDF_GROUND | SDF_COMMON;
        (*s).n_chan = ao_n_chan as u32;
        (*s).maxdata = 0x3fff;
        (*s).range_table = &mut apci3501_ao_range;
        (*s).insn_write = Some(apci3501_ao_insn_write);
        let ret = comedi_alloc_subdev_readback(s);
        if ret != 0 { return ret; }
    } else { (*s).type_ = COMEDI_SUBD_UNUSED; }

    s = (*dev).subdevices.add(1);
    (*s).type_ = COMEDI_SUBD_DI;
    (*s).subdev_flags = SDF_READABLE;
    (*s).n_chan = 2;
    (*s).maxdata = 1;
    (*s).range_table = &mut range_digital;
    (*s).insn_bits = Some(apci3501_di_insn_bits);

    s = (*dev).subdevices.add(2);
    (*s).type_ = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 2;
    (*s).maxdata = 1;
    (*s).range_table = &mut range_digital;
    (*s).insn_bits = Some(apci3501_do_insn_bits);

    s = (*dev).subdevices.add(3);
    (*s).type_ = COMEDI_SUBD_UNUSED;
    s = (*dev).subdevices.add(4);
    (*s).type_ = COMEDI_SUBD_MEMORY;
    (*s).subdev_flags = SDF_READABLE | SDF_INTERNAL;
    (*s).n_chan = 256;
    (*s).maxdata = 0xffff;
    (*s).insn_read = Some(apci3501_eeprom_insn_read);
    apci3501_reset(dev);
    0
}

unsafe fn apci3501_detach(dev: *mut comedi_device) {
    if (*dev).iobase != 0 { apci3501_reset(dev); }
    comedi_pci_detach(dev);
}

#[no_mangle]
pub static mut apci3501_driver: comedi_driver = comedi_driver {
    driver_name: "addi_apci_3501",
    module: THIS_MODULE,
    auto_attach: Some(apci3501_auto_attach),
    detach: Some(apci3501_detach),
};

unsafe fn apci3501_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &mut apci3501_driver, (*id).driver_data)
}

static mut apci3501_pci_table: [pci_device_id; 2] = [
    PCI_VDEVICE(ADDIDATA, 0x3001),
    pci_device_id::default(),
];

static mut apci3501_pci_driver: pci_driver = pci_driver {
    name: "addi_apci_3501",
    id_table: apci3501_pci_table.as_ptr(),
    probe: Some(apci3501_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// Equivalent of module_comedi_pci_driver(apci3501_driver, apci3501_pci_driver).
// MODULE_DESCRIPTION("ADDI-DATA APCI-3501 Analog output board");
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
