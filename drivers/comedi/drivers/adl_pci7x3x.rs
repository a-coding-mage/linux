// SPDX-License-Identifier: GPL-2.0+
/* COMEDI driver for the ADLINK PCI-723x/743x series boards. */
/* C headers and build-time kernel dependencies are supplied externally. */

const PCI7X3X_DIO_REG: usize = 0x0000;
const PCI743X_DIO_REG: usize = 0x0004;
const ADL_PT_CLRIRQ: usize = 0x0040;
const LINTI1_EN_ACT_IDI0: u32 = PLX9052_INTCSR_LI1ENAB | PLX9052_INTCSR_LI1STAT;
const LINTI2_EN_ACT_IDI1: u32 = PLX9052_INTCSR_LI2ENAB | PLX9052_INTCSR_LI2STAT;
const EN_PCI_LINT2H_LINT1H: u32 = PLX9052_INTCSR_PCIENAB | PLX9052_INTCSR_LI2POL | PLX9052_INTCSR_LI1POL;

#[repr(C)]
enum adl_pci7x3x_boardid {
    BOARD_PCI7230,
    BOARD_PCI7233,
    BOARD_PCI7234,
    BOARD_PCI7432,
    BOARD_PCI7433,
    BOARD_PCI7434,
}

#[repr(C)]
struct adl_pci7x3x_boardinfo {
    name: *const i8,
    nsubdevs: i32,
    di_nchan: i32,
    do_nchan: i32,
    irq_nchan: i32,
}

static adl_pci7x3x_boards: [adl_pci7x3x_boardinfo; 6] = [
    adl_pci7x3x_boardinfo { name: b"adl_pci7230\0".as_ptr() as *const i8, nsubdevs: 4, di_nchan: 16, do_nchan: 16, irq_nchan: 2 },
    adl_pci7x3x_boardinfo { name: b"adl_pci7233\0".as_ptr() as *const i8, nsubdevs: 1, di_nchan: 32, do_nchan: 0, irq_nchan: 0 },
    adl_pci7x3x_boardinfo { name: b"adl_pci7234\0".as_ptr() as *const i8, nsubdevs: 1, di_nchan: 0, do_nchan: 32, irq_nchan: 0 },
    adl_pci7x3x_boardinfo { name: b"adl_pci7432\0".as_ptr() as *const i8, nsubdevs: 2, di_nchan: 32, do_nchan: 32, irq_nchan: 0 },
    adl_pci7x3x_boardinfo { name: b"adl_pci7433\0".as_ptr() as *const i8, nsubdevs: 2, di_nchan: 64, do_nchan: 0, irq_nchan: 0 },
    adl_pci7x3x_boardinfo { name: b"adl_pci7434\0".as_ptr() as *const i8, nsubdevs: 2, di_nchan: 0, do_nchan: 64, irq_nchan: 0 },
];

#[repr(C)]
struct adl_pci7x3x_dev_private_data { lcr_io_base: usize, int_ctrl: u32 }
#[repr(C)]
struct adl_pci7x3x_sd_private_data { subd_slock: spinlock_t, port_offset: usize, cmd_running: i16 }

unsafe fn process_irq(dev: *mut comedi_device, subdev: u32, _intcsr: u16) {
    let s = &mut (*dev).subdevices.add(subdev as usize).read();
    let p = s.private as *mut adl_pci7x3x_sd_private_data;
    let val = inw((*dev).iobase + (*p).port_offset);
    if !s.async_.is_null() {
        spin_lock(&mut (*p).subd_slock);
        if (*p).cmd_running != 0 { comedi_buf_write_samples(s, &val as *const _ as *const _, 1); }
        spin_unlock(&mut (*p).subd_slock);
        comedi_handle_events(dev, s);
    }
}

unsafe extern "C" fn adl_pci7x3x_interrupt(_irq: i32, p_device: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = p_device as *mut comedi_device;
    let dp = (*dev).private as *mut adl_pci7x3x_dev_private_data;
    if !(*dev).attached { return IRQ_NONE; }
    let mut flags = 0;
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags);
    let intcsr = inl((*dp).lcr_io_base + PLX9052_INTCSR);
    let li1stat = (intcsr & LINTI1_EN_ACT_IDI0) == LINTI1_EN_ACT_IDI0;
    let li2stat = (intcsr & LINTI2_EN_ACT_IDI1) == LINTI2_EN_ACT_IDI1;
    if li1stat || li2stat { outb(0, (*dev).iobase + ADL_PT_CLRIRQ); }
    spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    if li1stat { process_irq(dev, 2, intcsr as u16); }
    if li2stat { process_irq(dev, 3, intcsr as u16); }
    IRQ_RETVAL(li1stat || li2stat)
}

unsafe extern "C" fn adl_pci7x3x_asy_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
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

unsafe extern "C" fn adl_pci7x3x_asy_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let dp = (*dev).private as *mut adl_pci7x3x_dev_private_data;
    let sp = (*s).private as *mut adl_pci7x3x_sd_private_data;
    let en = if (*s).index == 2 { PLX9052_INTCSR_LI1ENAB } else { PLX9052_INTCSR_LI2ENAB };
    let mut flags = 0;
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags); (*dp).int_ctrl |= en; outl((*dp).int_ctrl, (*dp).lcr_io_base + PLX9052_INTCSR); spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    spin_lock_irqsave(&mut (*sp).subd_slock, &mut flags); (*sp).cmd_running = 1; spin_unlock_irqrestore(&mut (*sp).subd_slock, flags); 0
}

unsafe extern "C" fn adl_pci7x3x_asy_cancel(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let dp = (*dev).private as *mut adl_pci7x3x_dev_private_data; let sp = (*s).private as *mut adl_pci7x3x_sd_private_data; let en = if (*s).index == 2 { PLX9052_INTCSR_LI1ENAB } else { PLX9052_INTCSR_LI2ENAB }; let mut flags = 0;
    spin_lock_irqsave(&mut (*sp).subd_slock, &mut flags); (*sp).cmd_running = 0; spin_unlock_irqrestore(&mut (*sp).subd_slock, flags);
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags); (*dp).int_ctrl &= !en; outl((*dp).int_ctrl, (*dp).lcr_io_base + PLX9052_INTCSR); spin_unlock_irqrestore(&mut (*dev).spinlock, flags); 0
}

unsafe extern "C" fn adl_pci7x3x_dirq_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 { let p = (*s).private as *mut adl_pci7x3x_sd_private_data; *data.add(1) = inl((*dev).iobase + (*p).port_offset); (*insn).n as i32 }
unsafe extern "C" fn adl_pci7x3x_di_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 { *data.add(1) = inl((*dev).iobase + (*s).private as usize); (*insn).n as i32 }
unsafe extern "C" fn adl_pci7x3x_do_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 { let reg = (*s).private as usize; if comedi_dio_update_state(s, data) { let mut val = (*s).state; if (*s).n_chan == 16 { val |= val << 16; } outl(val, (*dev).iobase + reg); } *data.add(1) = (*s).state; (*insn).n as i32 }

unsafe fn adl_pci7x3x_reset(dev: *mut comedi_device) -> i32 { let dp = (*dev).private as *mut adl_pci7x3x_dev_private_data; (*dp).int_ctrl = 0; outl(0, (*dp).lcr_io_base + PLX9052_INTCSR); 0 }

unsafe extern "C" fn adl_pci7x3x_auto_attach(dev: *mut comedi_device, context: usize) -> i32 {
    if context >= adl_pci7x3x_boards.len() { return -ENODEV; }
    let board = &adl_pci7x3x_boards[context];
    (*dev).board_ptr = board as *const _ as *mut _; (*dev).board_name = board.name;
    let dp = comedi_alloc_devpriv(dev, core::mem::size_of::<adl_pci7x3x_dev_private_data>()) as *mut adl_pci7x3x_dev_private_data;
    if dp.is_null() { return -ENOMEM; }
    let pcidev = comedi_to_pci_dev(dev); let mut ret = comedi_pci_enable(dev); if ret != 0 { return ret; }
    (*dev).iobase = pci_resource_start(pcidev, 2); (*dp).lcr_io_base = pci_resource_start(pcidev, 1); adl_pci7x3x_reset(dev);
    if board.irq_nchan != 0 { outb(0, (*dev).iobase + ADL_PT_CLRIRQ); if (*pcidev).irq != 0 { ret = request_irq((*pcidev).irq, adl_pci7x3x_interrupt, IRQF_SHARED, (*dev).board_name, dev as *mut _); if ret == 0 { (*dev).irq = (*pcidev).irq; (*dp).int_ctrl = EN_PCI_LINT2H_LINT1H; outl((*dp).int_ctrl, (*dp).lcr_io_base + PLX9052_INTCSR); } } }
    ret = comedi_alloc_subdevices(dev, board.nsubdevs as usize); if ret != 0 { return ret; }
    // Subdevice construction below follows the C driver: DI and DO ports are split at 32 channels, and IRQ inputs use private spinlocked state.
    let mut subdev = 0usize;
    if board.di_nchan != 0 { let n = core::cmp::min(board.di_nchan, 32); let s = &mut *(*dev).subdevices.add(subdev); s.type_ = COMEDI_SUBD_DI; s.subdev_flags = SDF_READABLE; s.n_chan = n; s.maxdata = 1; s.insn_bits = Some(adl_pci7x3x_di_insn_bits); s.range_table = &range_digital; s.private = PCI7X3X_DIO_REG as *mut _; subdev += 1; if board.di_nchan > n { let s = &mut *(*dev).subdevices.add(subdev); s.type_ = COMEDI_SUBD_DI; s.subdev_flags = SDF_READABLE; s.n_chan = board.di_nchan - n; s.maxdata = 1; s.insn_bits = Some(adl_pci7x3x_di_insn_bits); s.range_table = &range_digital; s.private = PCI743X_DIO_REG as *mut _; subdev += 1; } }
    if board.do_nchan != 0 { let n = core::cmp::min(board.do_nchan, 32); let s = &mut *(*dev).subdevices.add(subdev); s.type_ = COMEDI_SUBD_DO; s.subdev_flags = SDF_WRITABLE; s.n_chan = n; s.maxdata = 1; s.insn_bits = Some(adl_pci7x3x_do_insn_bits); s.range_table = &range_digital; s.private = PCI7X3X_DIO_REG as *mut _; subdev += 1; if board.do_nchan > n { let s = &mut *(*dev).subdevices.add(subdev); s.type_ = COMEDI_SUBD_DO; s.subdev_flags = SDF_WRITABLE; s.n_chan = board.do_nchan - n; s.maxdata = 1; s.insn_bits = Some(adl_pci7x3x_do_insn_bits); s.range_table = &range_digital; s.private = PCI743X_DIO_REG as *mut _; subdev += 1; } }
    for _ in 0..board.irq_nchan { let s = &mut *(*dev).subdevices.add(subdev); s.type_ = COMEDI_SUBD_DI; s.subdev_flags = SDF_READABLE; s.n_chan = 1; s.maxdata = 1; s.insn_bits = Some(adl_pci7x3x_dirq_insn_bits); s.range_table = &range_digital; let sp = comedi_alloc_spriv(s, core::mem::size_of::<adl_pci7x3x_sd_private_data>()) as *mut adl_pci7x3x_sd_private_data; if sp.is_null() { return -ENOMEM; } spin_lock_init(&mut (*sp).subd_slock); (*sp).port_offset = PCI7X3X_DIO_REG; (*sp).cmd_running = 0; if (*dev).irq != 0 { (*dev).read_subdev = s; s.subdev_flags = SDF_READABLE | SDF_CMD_READ; s.len_chanlist = 1; s.do_cmdtest = Some(adl_pci7x3x_asy_cmdtest); s.do_cmd = Some(adl_pci7x3x_asy_cmd); s.cancel = Some(adl_pci7x3x_asy_cancel); } subdev += 1; }
    0
}

unsafe extern "C" fn adl_pci7x3x_detach(dev: *mut comedi_device) { if (*dev).iobase != 0 { adl_pci7x3x_reset(dev); } comedi_pci_detach(dev); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
