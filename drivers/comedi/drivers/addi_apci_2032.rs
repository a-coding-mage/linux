// SPDX-License-Identifier: GPL-2.0+
/*
 * addi_apci_2032.c
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

// Dependencies supplied by the surrounding kernel/comedi translation.

/* PCI bar 1 I/O Register map */
const APCI2032_DO_REG: usize = 0x00;
const APCI2032_INT_CTRL_REG: usize = 0x04;
const APCI2032_INT_CTRL_VCC_ENA: u32 = 1 << 0;
const APCI2032_INT_CTRL_CC_ENA: u32 = 1 << 1;
const APCI2032_INT_STATUS_REG: usize = 0x08;
const APCI2032_INT_STATUS_VCC: u32 = 1 << 0;
const APCI2032_INT_STATUS_CC: u32 = 1 << 1;
const APCI2032_STATUS_REG: usize = 0x0c;
const APCI2032_STATUS_IRQ: u32 = 1 << 0;
const APCI2032_WDOG_REG: usize = 0x10;

#[repr(C)]
struct Apci2032IntPrivate {
    spinlock: SpinlockT,
    active: bool,
    enabled_isns: u8,
}

unsafe fn apci2032_do_insn_bits(
    dev: *mut ComediDevice, s: *mut ComediSubdevice, insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    (*s).state = inl((*dev).iobase + APCI2032_DO_REG as _);
    if comedi_dio_update_state(s, data) != 0 {
        outl((*s).state, (*dev).iobase + APCI2032_DO_REG as _);
    }
    *data.add(1) = (*s).state;
    (*insn).n
}

unsafe fn apci2032_int_insn_bits(
    dev: *mut ComediDevice, _s: *mut ComediSubdevice, insn: *mut ComediInsn,
    data: *mut u32,
) -> i32 {
    *data.add(1) = inl((*dev).iobase + APCI2032_INT_STATUS_REG as _) & 3;
    (*insn).n
}

unsafe fn apci2032_int_stop(dev: *mut ComediDevice, s: *mut ComediSubdevice) {
    let subpriv = (*s).private as *mut Apci2032IntPrivate;
    (*subpriv).active = false;
    (*subpriv).enabled_isns = 0;
    outl(0, (*dev).iobase + APCI2032_INT_CTRL_REG as _);
}

unsafe fn apci2032_int_cmdtest(_dev: *mut ComediDevice, _s: *mut ComediSubdevice, cmd: *mut ComediCmd) -> i32 {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).stop_src);
    if err != 0 { return 2; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    if (*cmd).stop_src == TRIG_COUNT {
        err |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 1);
    } else {
        err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0);
    }
    if err != 0 { return 3; }
    0
}

unsafe fn apci2032_int_cmd(dev: *mut ComediDevice, s: *mut ComediSubdevice) -> i32 {
    let cmd = &mut (*(*s).async_).cmd;
    let subpriv = (*s).private as *mut Apci2032IntPrivate;
    let mut enabled_isns: u8 = 0;
    for n in 0..cmd.chanlist_len {
        enabled_isns |= 1 << cr_chan(*cmd.chanlist.add(n as usize));
    }
    let mut flags = 0;
    spin_lock_irqsave(&mut (*subpriv).spinlock, &mut flags);
    (*subpriv).enabled_isns = enabled_isns;
    (*subpriv).active = true;
    outl(enabled_isns as u32, (*dev).iobase + APCI2032_INT_CTRL_REG as _);
    spin_unlock_irqrestore(&mut (*subpriv).spinlock, flags);
    0
}

unsafe fn apci2032_int_cancel(dev: *mut ComediDevice, s: *mut ComediSubdevice) -> i32 {
    let subpriv = (*s).private as *mut Apci2032IntPrivate;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*subpriv).spinlock, &mut flags);
    if (*subpriv).active { apci2032_int_stop(dev, s); }
    spin_unlock_irqrestore(&mut (*subpriv).spinlock, flags);
    0
}

unsafe extern "C" fn apci2032_interrupt(_irq: i32, d: *mut core::ffi::c_void) -> IrqreturnT {
    let dev = d as *mut ComediDevice;
    let s = (*dev).read_subdev;
    let cmd = &mut (*(*s).async_).cmd;
    if !(*dev).attached { return IRQ_NONE; }
    let mut val = inl((*dev).iobase + APCI2032_STATUS_REG as _) & APCI2032_STATUS_IRQ;
    if val == 0 { return IRQ_NONE; }
    let subpriv = (*s).private as *mut Apci2032IntPrivate;
    spin_lock(&mut (*subpriv).spinlock);
    val = inl((*dev).iobase + APCI2032_INT_STATUS_REG as _) & 3;
    outl(!val & 3, (*dev).iobase + APCI2032_INT_CTRL_REG as _);
    if (*subpriv).active && (val & (*subpriv).enabled_isns as u32) != 0 {
        let mut bits: u16 = 0;
        for i in 0..cmd.chanlist_len {
            let chan = cr_chan(*cmd.chanlist.add(i as usize));
            if val & (1 << chan) != 0 { bits |= 1 << i; }
        }
        comedi_buf_write_samples(s, &bits as *const _ as *const _, 1);
        if cmd.stop_src == TRIG_COUNT && (*(*s).async_).scans_done >= cmd.stop_arg {
            (*(*s).async_).events |= COMEDI_CB_EOA;
        }
    }
    spin_unlock(&mut (*subpriv).spinlock);
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

unsafe fn apci2032_reset(dev: *mut ComediDevice) -> i32 {
    outl(0, (*dev).iobase + APCI2032_DO_REG as _);
    outl(0, (*dev).iobase + APCI2032_INT_CTRL_REG as _);
    addi_watchdog_reset((*dev).iobase + APCI2032_WDOG_REG as _);
    0
}

// The remaining driver registration and subdevice setup mirror the C source;
// referenced kernel/comedi types and functions are supplied by future dependencies.
unsafe fn apci2032_auto_attach(dev: *mut ComediDevice, _context_unused: usize) -> i32 {
    let pcidev = comedi_to_pci_dev(dev);
    let ret = comedi_pci_enable(dev);
    if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 1);
    apci2032_reset(dev);
    if (*pcidev).irq > 0 {
        let irq_ret = request_irq((*pcidev).irq, apci2032_interrupt, IRQF_SHARED, (*dev).board_name, dev.cast());
        if irq_ret == 0 { (*dev).irq = (*pcidev).irq; }
    }
    let ret = comedi_alloc_subdevices(dev, 3);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s).type_ = COMEDI_SUBD_DO;
    (*s).subdev_flags = SDF_WRITABLE;
    (*s).n_chan = 32;
    (*s).maxdata = 1;
    (*s).range_table = &range_digital;
    (*s).insn_bits = Some(apci2032_do_insn_bits);
    let s1 = s.add(1);
    let ret = addi_watchdog_init(s1, (*dev).iobase + APCI2032_WDOG_REG as _);
    if ret != 0 { return ret; }
    let s2 = s.add(2);
    (*s2).type_ = COMEDI_SUBD_DI;
    (*s2).subdev_flags = SDF_READABLE;
    (*s2).n_chan = 2;
    (*s2).maxdata = 1;
    (*s2).range_table = &range_digital;
    (*s2).insn_bits = Some(apci2032_int_insn_bits);
    if (*dev).irq != 0 {
        (*dev).read_subdev = s2;
        let subpriv = kzalloc::<Apci2032IntPrivate>();
        if subpriv.is_null() { return -12; }
        spin_lock_init(&mut (*subpriv).spinlock);
        (*s2).private = subpriv.cast();
        (*s2).subdev_flags = SDF_READABLE | SDF_CMD_READ | SDF_PACKED;
        (*s2).len_chanlist = 2;
        (*s2).do_cmdtest = Some(apci2032_int_cmdtest);
        (*s2).do_cmd = Some(apci2032_int_cmd);
        (*s2).cancel = Some(apci2032_int_cancel);
    }
    0
}
unsafe fn apci2032_detach(dev: *mut ComediDevice) { if (*dev).iobase != 0 { apci2032_reset(dev); } comedi_pci_detach(dev); if !(*dev).read_subdev.is_null() { kfree((*dev).read_subdev.cast()); } }

#[repr(C)]
struct ComediDriver { driver_name: *const u8, module: *mut core::ffi::c_void, auto_attach: Option<unsafe fn(*mut ComediDevice, usize) -> i32>, detach: Option<unsafe fn(*mut ComediDevice)> }
static mut APCI2032_DRIVER: ComediDriver = ComediDriver { driver_name: b"addi_apci_2032\0".as_ptr(), module: core::ptr::null_mut(), auto_attach: Some(apci2032_auto_attach), detach: Some(apci2032_detach) };

unsafe fn apci2032_pci_probe(dev: *mut PciDev, id: *const PciDeviceId) -> i32 { comedi_pci_auto_config(dev, &mut APCI2032_DRIVER, (*id).driver_data) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
