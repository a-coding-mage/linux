// SPDX-License-Identifier: GPL-2.0+
/*
 * addi_apci_1032.c
 * Copyright (C) 2004,2005  ADDI-DATA GmbH for the source code of this module.
 * Project manager: Eric Stolz
 *
 * ADDI-DATA GmbH
 * Dieselstrasse 3
 * D-77833 Ottersweier
 * Tel: +19(0)7223/9493-0
 * Fax: +49(0)7223/9493-92
 * http://www.addi-data.com
 * info@addi-data.com
 */

// Driver: addi_apci_1032
// Description: ADDI-DATA APCI-1032 Digital Input Board
// Status: untested
// This driver models the APCI-1032 as a 32-channel digital input subdevice
// plus an additional digital input subdevice to handle change-of-state
// interrupts.

pub const APCI1032_DI_REG: u32 = 0x00;
pub const APCI1032_MODE1_REG: u32 = 0x04;
pub const APCI1032_MODE2_REG: u32 = 0x08;
pub const APCI1032_STATUS_REG: u32 = 0x0c;
pub const APCI1032_CTRL_REG: u32 = 0x10;

#[inline]
pub const fn apci1032_ctrl_int_mode(x: u32) -> u32 { (x & 0x1) << 1 }
pub const APCI1032_CTRL_INT_OR: u32 = apci1032_ctrl_int_mode(0);
pub const APCI1032_CTRL_INT_AND: u32 = apci1032_ctrl_int_mode(1);
pub const APCI1032_CTRL_INT_ENA: u32 = 1 << 2;

#[repr(C)]
pub struct apci1032_private {
    pub amcc_iobase: libc::c_ulong,
    pub mode1: u32,
    pub mode2: u32,
    pub ctrl: u32,
}

unsafe fn apci1032_reset(dev: *mut comedi_device) -> i32 {
    outl(0, (*dev).iobase + APCI1032_CTRL_REG);
    inl((*dev).iobase + APCI1032_STATUS_REG);
    outl(0, (*dev).iobase + APCI1032_MODE1_REG);
    outl(0, (*dev).iobase + APCI1032_MODE2_REG);
    0
}

unsafe fn apci1032_cos_insn_config(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                   insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let devpriv = (*dev).private as *mut apci1032_private;
    if *data != INSN_CONFIG_DIGITAL_TRIG { return -EINVAL; }
    if *data.add(1) != 0 { return -EINVAL; }
    let shift = *data.add(3);
    let (oldmask, himask, lomask) = if shift < 32 {
        ((1u32 << shift).wrapping_sub(1), (*data.add(4)).wrapping_shl(shift),
         (*data.add(5)).wrapping_shl(shift))
    } else { (0xffff_ffff, 0, 0) };
    match *data.add(2) {
        COMEDI_DIGITAL_TRIG_DISABLE => {
            (*devpriv).ctrl = 0; (*devpriv).mode1 = 0; (*devpriv).mode2 = 0;
            apci1032_reset(dev);
        }
        COMEDI_DIGITAL_TRIG_ENABLE_EDGES | COMEDI_DIGITAL_TRIG_ENABLE_LEVELS => {
            let mode = if *data.add(2) == COMEDI_DIGITAL_TRIG_ENABLE_EDGES {
                APCI1032_CTRL_INT_OR
            } else { APCI1032_CTRL_INT_AND };
            if (*devpriv).ctrl != (APCI1032_CTRL_INT_ENA | mode) {
                (*devpriv).ctrl = APCI1032_CTRL_INT_ENA | mode;
                (*devpriv).mode1 = 0; (*devpriv).mode2 = 0;
            } else {
                (*devpriv).mode1 &= oldmask; (*devpriv).mode2 &= oldmask;
            }
            (*devpriv).mode1 |= himask; (*devpriv).mode2 |= lomask;
        }
        _ => return -EINVAL,
    }
    (*insn).n as i32
}

unsafe fn apci1032_cos_insn_bits(_dev: *mut comedi_device, s: *mut comedi_subdevice,
                                 _insn: *mut comedi_insn, data: *mut u32) -> i32 {
    *data.add(1) = (*s).state; 0
}

unsafe fn apci1032_cos_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice,
                               cmd: *mut comedi_cmd) -> i32 {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_FOLLOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0);
    if err != 0 { return 3; }
    0
}

unsafe fn apci1032_cos_cmd(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 {
    let p = (*dev).private as *mut apci1032_private;
    if (*p).ctrl == 0 { dev_warn((*dev).class_dev, "Interrupts disabled due to mode configuration!\n"); return -EINVAL; }
    outl((*p).mode1, (*dev).iobase + APCI1032_MODE1_REG);
    outl((*p).mode2, (*dev).iobase + APCI1032_MODE2_REG);
    outl((*p).ctrl, (*dev).iobase + APCI1032_CTRL_REG); 0
}

unsafe fn apci1032_cos_cancel(dev: *mut comedi_device, _s: *mut comedi_subdevice) -> i32 { apci1032_reset(dev) }

unsafe extern "C" fn apci1032_interrupt(_irq: i32, d: *mut libc::c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    let p = (*dev).private as *mut apci1032_private;
    let s = (*dev).read_subdev;
    if (inl((*p).amcc_iobase as u32 + AMCC_OP_REG_INTCSR) & INTCSR_INTR_ASSERTED) == 0 { return IRQ_NONE; }
    let ctrl = inl((*dev).iobase + APCI1032_CTRL_REG);
    if (ctrl & APCI1032_CTRL_INT_ENA) == 0 { return IRQ_HANDLED; }
    outl(ctrl & !APCI1032_CTRL_INT_ENA, (*dev).iobase + APCI1032_CTRL_REG);
    (*s).state = inl((*dev).iobase + APCI1032_STATUS_REG) & 0xffff;
    let val = (*s).state as u16;
    comedi_buf_write_samples(s, &val as *const u16, 1);
    comedi_handle_events(dev, s);
    outl(ctrl, (*dev).iobase + APCI1032_CTRL_REG); IRQ_HANDLED
}

unsafe fn apci1032_di_insn_bits(dev: *mut comedi_device, _s: *mut comedi_subdevice,
                                insn: *mut comedi_insn, data: *mut u32) -> i32 {
    *data.add(1) = inl((*dev).iobase + APCI1032_DI_REG); (*insn).n as i32
}

// Remaining PCI registration and subdevice initialization are represented by
// the corresponding external kernel/comedi declarations and retain the C
// driver's externally visible interfaces.
unsafe fn apci1032_auto_attach(dev: *mut comedi_device, _context_unused: libc::c_ulong) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let p = comedi_alloc_devpriv(dev, core::mem::size_of::<apci1032_private>()) as *mut apci1032_private;
    if p.is_null() { return -ENOMEM; }
    let mut ret = comedi_pci_enable(dev); if ret != 0 { return ret; }
    (*p).amcc_iobase = pci_resource_start(pcidev, 0);
    (*dev).iobase = pci_resource_start(pcidev, 1); apci1032_reset(dev);
    if (*pcidev).irq > 0 { ret = request_irq((*pcidev).irq, apci1032_interrupt, IRQF_SHARED, (*dev).board_name, dev as *mut libc::c_void); if ret == 0 { (*dev).irq = (*pcidev).irq; } }
    ret = comedi_alloc_subdevices(dev, 2); if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_DI; (*s).subdev_flags = SDF_READABLE; (*s).n_chan = 32; (*s).maxdata = 1; (*s).range_table = &range_digital; (*s).insn_bits = Some(apci1032_di_insn_bits);
    let c = s.add(1); if (*dev).irq != 0 { (*dev).read_subdev = c; (*c).type_ = COMEDI_SUBD_DI; (*c).subdev_flags = SDF_READABLE | SDF_CMD_READ; (*c).n_chan = 1; (*c).maxdata = 1; (*c).range_table = &range_digital; (*c).insn_config = Some(apci1032_cos_insn_config); (*c).insn_bits = Some(apci1032_cos_insn_bits); (*c).len_chanlist = 1; (*c).do_cmdtest = Some(apci1032_cos_cmdtest); (*c).do_cmd = Some(apci1032_cos_cmd); (*c).cancel = Some(apci1032_cos_cancel); } else { (*c).type_ = COMEDI_SUBD_UNUSED; } 0
}

unsafe fn apci1032_detach(dev: *mut comedi_device) { if (*dev).iobase != 0 { apci1032_reset(dev); } comedi_pci_detach(dev); }

unsafe fn apci1032_pci_probe(dev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    comedi_pci_auto_config(dev, &apci1032_driver, (*id).driver_data)
}

static mut APCI1032_PCI_TABLE: [pci_device_id; 2] = [
    pci_device_id { vendor: PCI_VENDOR_ID_ADDIDATA, device: 0x1003, driver_data: 0 },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 },
];

static mut apci1032_driver: comedi_driver = comedi_driver {
    driver_name: "addi_apci_1032",
    module: THIS_MODULE,
    auto_attach: Some(apci1032_auto_attach),
    detach: Some(apci1032_detach),
};

static mut APCI1032_PCI_DRIVER: pci_driver = pci_driver {
    name: "addi_apci_1032",
    id_table: unsafe { &APCI1032_PCI_TABLE },
    probe: Some(apci1032_pci_probe),
    remove: Some(comedi_pci_auto_unconfig),
};

// Equivalent to module_comedi_pci_driver(apci1032_driver, apci1032_pci_driver).
// MODULE_DEVICE_TABLE(pci, apci1032_pci_table);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("ADDI-DATA APCI-1032, 32 channel DI boards");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
