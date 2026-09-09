// SPDX-License-Identifier: GPL-2.0+
/*
 * pcmmio.c
 * Driver for Winsystems PC-104 based multifunction IO board.
 *
 * COMEDI - Linux Control and Measurement Device Interface
 * Copyright (C) 2007 Calin A. Culianu <calin@ajvar.org>
 */

/* Rust translation of the original Linux Comedi driver. */

const PCMMIO_AI_LSB_REG: u32 = 0x00;
const PCMMIO_AI_MSB_REG: u32 = 0x01;
const PCMMIO_AI_CMD_REG: u32 = 0x02;
const PCMMIO_AI_CMD_SE: u8 = 1 << 7;
const PCMMIO_AI_CMD_ODD_CHAN: u8 = 1 << 6;
const PCMMIO_AI_CMD_CHAN_SEL = |x: u32| ((x & 0x3) << 4);
const PCMMIO_AI_CMD_RANGE = |x: u32| ((x & 0x3) << 2);
const PCMMIO_RESOURCE_REG: u32 = 0x02;
const PCMMIO_RESOURCE_IRQ = |x: u32| ((x & 0xf) << 0);
const PCMMIO_AI_STATUS_REG: u32 = 0x03;
const PCMMIO_AI_STATUS_DATA_READY: u8 = 1 << 7;
const PCMMIO_AI_STATUS_DATA_DMA_PEND: u8 = 1 << 6;
const PCMMIO_AI_STATUS_CMD_DMA_PEND: u8 = 1 << 5;
const PCMMIO_AI_STATUS_IRQ_PEND: u8 = 1 << 4;
const PCMMIO_AI_STATUS_DATA_DRQ_ENA: u8 = 1 << 2;
const PCMMIO_AI_STATUS_REG_SEL: u8 = 1 << 3;
const PCMMIO_AI_STATUS_CMD_DRQ_ENA: u8 = 1 << 1;
const PCMMIO_AI_STATUS_IRQ_ENA: u8 = 1 << 0;
const PCMMIO_AI_RES_ENA_REG: u32 = 0x03;
const PCMMIO_AI_RES_ENA_CMD_REG_ACCESS: u8 = 0 << 3;
const PCMMIO_AI_RES_ENA_AI_RES_ACCESS: u8 = 1 << 3;
const PCMMIO_AI_RES_ENA_DIO_RES_ACCESS: u8 = 1 << 4;
const PCMMIO_AI_2ND_ADC_OFFSET: u32 = 0x04;

const PCMMIO_AO_LSB_REG: u32 = 0x08;
const PCMMIO_AO_LSB_SPAN = |x: u32| ((x & 0xf) << 0);
const PCMMIO_AO_MSB_REG: u32 = 0x09;
const PCMMIO_AO_CMD_REG: u32 = 0x0a;
const PCMMIO_AO_CMD_WR_SPAN: u8 = 0x2 << 4;
const PCMMIO_AO_CMD_WR_CODE: u8 = 0x3 << 4;
const PCMMIO_AO_CMD_UPDATE: u8 = 0x4 << 4;
const PCMMIO_AO_CMD_UPDATE_ALL: u8 = 0x5 << 4;
const PCMMIO_AO_CMD_WR_SPAN_UPDATE: u8 = 0x6 << 4;
const PCMMIO_AO_CMD_WR_CODE_UPDATE: u8 = 0x7 << 4;
const PCMMIO_AO_CMD_WR_SPAN_UPDATE_ALL: u8 = 0x8 << 4;
const PCMMIO_AO_CMD_WR_CODE_UPDATE_ALL: u8 = 0x9 << 4;
const PCMMIO_AO_CMD_RD_B1_SPAN: u8 = 0xa << 4;
const PCMMIO_AO_CMD_RD_B1_CODE: u8 = 0xb << 4;
const PCMMIO_AO_CMD_RD_B2_SPAN: u8 = 0xc << 4;
const PCMMIO_AO_CMD_RD_B2_CODE: u8 = 0xd << 4;
const PCMMIO_AO_CMD_NOP: u8 = 0xf << 4;
const PCMMIO_AO_CMD_CHAN_SEL = |x: u32| ((x & 0x03) << 1);
const PCMMIO_AO_CMD_CHAN_SEL_ALL: u8 = 0x0f;
const PCMMIO_AO_STATUS_REG: u32 = 0x0b;
const PCMMIO_AO_STATUS_DATA_READY: u8 = 1 << 7;
const PCMMIO_AO_RESOURCE_ENA_REG: u32 = 0x0b;
const PCMMIO_AO_2ND_DAC_OFFSET: u32 = 0x04;

const PCMMIO_PORT_REG = |x: u32| 0x10 + x;
const PCMMIO_INT_PENDING_REG: u32 = 0x16;
const PCMMIO_PAGE_LOCK_REG: u32 = 0x17;
const PCMMIO_LOCK_PORT = |x: u32| (1 << x) & 0x3f;
const PCMMIO_PAGE = |x: u32| ((x & 0x3) << 6);
const PCMMIO_PAGE_MASK: u32 = PCMMIO_PAGE(3);
const PCMMIO_PAGE_POL: i32 = 1;
const PCMMIO_PAGE_ENAB: i32 = 2;
const PCMMIO_PAGE_INT_ID: i32 = 3;
const PCMMIO_PAGE_REG = |x: u32| 0x18 + x;

static pcmmio_ai_ranges: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(5), BIP_RANGE(10), UNI_RANGE(5), UNI_RANGE(10)] };
static pcmmio_ao_ranges: comedi_lrange = comedi_lrange { length: 6, range: [UNI_RANGE(5), UNI_RANGE(10), BIP_RANGE(5), BIP_RANGE(10), BIP_RANGE(2.5), RANGE(-2.5, 7.5)] };

#[repr(C)]
struct pcmmio_private {
    pagelock: spinlock_t,
    spinlock: spinlock_t,
    enabled_mask: u32,
    active: u32,
}

unsafe fn pcmmio_dio_write(dev: *mut comedi_device, val: u32, page: i32, port: i32) {
    let devpriv = (*dev).private as *mut pcmmio_private;
    let iobase = (*dev).iobase;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*devpriv).pagelock, &mut flags);
    if page == 0 {
        outb((val & 0xff) as u8, iobase + PCMMIO_PORT_REG((port + 0) as u32));
        outb(((val >> 8) & 0xff) as u8, iobase + PCMMIO_PORT_REG((port + 1) as u32));
        outb(((val >> 16) & 0xff) as u8, iobase + PCMMIO_PORT_REG((port + 2) as u32));
    } else {
        outb(PCMMIO_PAGE(page as u32) as u8, iobase + PCMMIO_PAGE_LOCK_REG);
        outb((val & 0xff) as u8, iobase + PCMMIO_PAGE_REG(0));
        outb(((val >> 8) & 0xff) as u8, iobase + PCMMIO_PAGE_REG(1));
        outb(((val >> 16) & 0xff) as u8, iobase + PCMMIO_PAGE_REG(2));
    }
    spin_unlock_irqrestore(&mut (*devpriv).pagelock, flags);
}

unsafe fn pcmmio_dio_read(dev: *mut comedi_device, page: i32, port: i32) -> u32 {
    let devpriv = (*dev).private as *mut pcmmio_private;
    let iobase = (*dev).iobase;
    let mut flags: c_ulong = 0;
    let val;
    spin_lock_irqsave(&mut (*devpriv).pagelock, &mut flags);
    if page == 0 {
        val = inb(iobase + PCMMIO_PORT_REG((port + 0) as u32)) as u32
            | ((inb(iobase + PCMMIO_PORT_REG((port + 1) as u32)) as u32) << 8)
            | ((inb(iobase + PCMMIO_PORT_REG((port + 2) as u32)) as u32) << 16);
    } else {
        outb(PCMMIO_PAGE(page as u32) as u8, iobase + PCMMIO_PAGE_LOCK_REG);
        val = inb(iobase + PCMMIO_PAGE_REG(0)) as u32
            | ((inb(iobase + PCMMIO_PAGE_REG(1)) as u32) << 8)
            | ((inb(iobase + PCMMIO_PAGE_REG(2)) as u32) << 16);
    }
    spin_unlock_irqrestore(&mut (*devpriv).pagelock, flags);
    val
}

unsafe fn pcmmio_dio_insn_bits(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let port = if (*s).index == 2 { 0 } else { 3 };
    let chanmask = (1u32 << (*s).n_chan) - 1;
    let mask = comedi_dio_update_state(s, data);
    if mask != 0 {
        let mut val = !(*s).state & chanmask;
        val &= (*s).io_bits;
        pcmmio_dio_write(dev, val, 0, port);
    }
    let val = pcmmio_dio_read(dev, 0, port);
    *data.add(1) = !val & chanmask;
    (*insn).n as i32
}

unsafe fn pcmmio_dio_insn_config(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let port = if (*s).index == 2 { 0 } else { 3 };
    let ret = comedi_dio_insn_config(dev, s, insn, data, 0);
    if ret != 0 { return ret; }
    if *data == INSN_CONFIG_DIO_INPUT { pcmmio_dio_write(dev, (*s).io_bits, 0, port); }
    (*insn).n as i32
}

unsafe fn pcmmio_reset(dev: *mut comedi_device) {
    pcmmio_dio_write(dev, 0, 0, 0);
    pcmmio_dio_write(dev, 0, 0, 3);
    pcmmio_dio_write(dev, 0, PCMMIO_PAGE_POL, 0);
    pcmmio_dio_write(dev, 0, PCMMIO_PAGE_ENAB, 0);
    pcmmio_dio_write(dev, 0, PCMMIO_PAGE_INT_ID, 0);
}

unsafe fn pcmmio_stop_intr(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    let p = (*dev).private as *mut pcmmio_private;
    (*p).enabled_mask = 0;
    (*p).active = 0;
    (*(*s).async_).inttrig = None;
    pcmmio_dio_write(dev, 0, PCMMIO_PAGE_ENAB, 0);
}

unsafe fn pcmmio_start_intr(dev: *mut comedi_device, s: *mut comedi_subdevice) {
    let p = (*dev).private as *mut pcmmio_private;
    let cmd = &(*(*s).async_).cmd;
    let mut bits = 0u32;
    let mut pol_bits = 0u32;
    (*p).enabled_mask = 0;
    (*p).active = 1;
    if !cmd.chanlist.is_null() {
        for i in 0..cmd.chanlist_len {
            let chanspec = *cmd.chanlist.add(i as usize);
            let chan = CR_CHAN(chanspec);
            bits |= 1 << chan;
            pol_bits |= (if CR_AREF(chanspec) != 0 || CR_RANGE(chanspec) != 0 { 1 } else { 0 }) << chan;
        }
    }
    bits &= (1u32 << (*s).n_chan) - 1;
    (*p).enabled_mask = bits;
    pcmmio_dio_write(dev, pol_bits, PCMMIO_PAGE_POL, 0);
    pcmmio_dio_write(dev, bits, PCMMIO_PAGE_ENAB, 0);
}

unsafe fn pcmmio_cancel(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let p = (*dev).private as *mut pcmmio_private;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*p).spinlock, &mut flags);
    if (*p).active != 0 { pcmmio_stop_intr(dev, s); }
    spin_unlock_irqrestore(&mut (*p).spinlock, flags);
    0
}

unsafe fn pcmmio_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> i32 {
    if inb((*dev).iobase + PCMMIO_AI_STATUS_REG) & PCMMIO_AI_STATUS_DATA_READY != 0 { 0 } else { -EBUSY }
}

unsafe fn pcmmio_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let mut iobase = (*dev).iobase;
    let mut chan = CR_CHAN((*insn).chanspec);
    let range = CR_RANGE((*insn).chanspec);
    let aref = CR_AREF((*insn).chanspec);
    let mut cmd = 0u8;
    if chan > 7 { chan -= 8; iobase += PCMMIO_AI_2ND_ADC_OFFSET; }
    if aref == AREF_GROUND { cmd |= PCMMIO_AI_CMD_SE; }
    if chan % 2 != 0 { cmd |= PCMMIO_AI_CMD_ODD_CHAN; }
    cmd |= PCMMIO_AI_CMD_CHAN_SEL(chan / 2) as u8;
    cmd |= PCMMIO_AI_CMD_RANGE(range) as u8;
    outb(cmd, iobase + PCMMIO_AI_CMD_REG);
    let mut ret = comedi_timeout(dev, s, insn, Some(pcmmio_ai_eoc), 0);
    if ret != 0 { return ret; }
    let mut val = inb(iobase + PCMMIO_AI_LSB_REG) as u32 | ((inb(iobase + PCMMIO_AI_MSB_REG) as u32) << 8);
    for i in 0..(*insn).n {
        outb(cmd, iobase + PCMMIO_AI_CMD_REG);
        ret = comedi_timeout(dev, s, insn, Some(pcmmio_ai_eoc), 0);
        if ret != 0 { return ret; }
        val = inb(iobase + PCMMIO_AI_LSB_REG) as u32 | ((inb(iobase + PCMMIO_AI_MSB_REG) as u32) << 8);
        if comedi_range_is_bipolar(s, range) { val = comedi_offset_munge(s, val); }
        *data.add(i as usize) = val;
    }
    (*insn).n as i32
}

unsafe fn pcmmio_ao_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: c_ulong) -> i32 {
    if inb((*dev).iobase + PCMMIO_AO_STATUS_REG) & PCMMIO_AO_STATUS_DATA_READY != 0 { 0 } else { -EBUSY }
}

unsafe fn pcmmio_ao_insn_write(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    let mut io = (*dev).iobase;
    let chan = CR_CHAN((*insn).chanspec);
    let range = CR_RANGE((*insn).chanspec);
    let mut cmd = 0u8;
    if chan > 3 { cmd |= PCMMIO_AO_CMD_CHAN_SEL((chan - 4) as u32) as u8; io += PCMMIO_AO_2ND_DAC_OFFSET; }
    else { cmd |= PCMMIO_AO_CMD_CHAN_SEL(chan as u32) as u8; }
    outb(PCMMIO_AO_LSB_SPAN(range) as u8, io + PCMMIO_AO_LSB_REG);
    outb(0, io + PCMMIO_AO_MSB_REG);
    outb(cmd | PCMMIO_AO_CMD_WR_SPAN_UPDATE, io + PCMMIO_AO_CMD_REG);
    let mut ret = comedi_timeout(dev, s, insn, Some(pcmmio_ao_eoc), 0);
    if ret != 0 { return ret; }
    for i in 0..(*insn).n {
        let val = *data.add(i as usize);
        outb((val & 0xff) as u8, io + PCMMIO_AO_LSB_REG);
        outb(((val >> 8) & 0xff) as u8, io + PCMMIO_AO_MSB_REG);
        outb(cmd | PCMMIO_AO_CMD_WR_CODE_UPDATE, io + PCMMIO_AO_CMD_REG);
        ret = comedi_timeout(dev, s, insn, Some(pcmmio_ao_eoc), 0);
        if ret != 0 { return ret; }
        *(*s).readback.add(chan as usize) = val;
    }
    (*insn).n as i32
}

unsafe fn interrupt_pcmmio(_irq: i32, d: *mut c_void) -> irqreturn_t {
    let dev = d as *mut comedi_device;
    if (*dev).attached == 0 { return IRQ_NONE; }
    if inb((*dev).iobase + PCMMIO_INT_PENDING_REG) & 7 == 0 { return IRQ_NONE; }
    let triggered = pcmmio_dio_read(dev, PCMMIO_PAGE_INT_ID, 0);
    pcmmio_dio_write(dev, 0, PCMMIO_PAGE_INT_ID, 0);
    let s = (*dev).read_subdev;
    let p = (*dev).private as *mut pcmmio_private;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*p).spinlock, &mut flags);
    if (*p).active != 0 && triggered & (*p).enabled_mask != 0 {
        comedi_buf_write_samples(s, &triggered, 1);
    }
    spin_unlock_irqrestore(&mut (*p).spinlock, flags);
    comedi_handle_events(dev, s);
    IRQ_HANDLED
}

unsafe fn pcmmio_inttrig_start_intr(dev: *mut comedi_device, s: *mut comedi_subdevice, trig_num: u32) -> i32 {
    let cmd = &(*(*s).async_).cmd;
    if trig_num != cmd.start_arg { return -EINVAL; }
    let p = (*dev).private as *mut pcmmio_private;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*p).spinlock, &mut flags);
    (*(*s).async_).inttrig = None;
    if (*p).active != 0 { pcmmio_start_intr(dev, s); }
    spin_unlock_irqrestore(&mut (*p).spinlock, flags);
    1
}

unsafe fn pcmmio_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> i32 {
    let p = (*dev).private as *mut pcmmio_private;
    let cmd = &(*(*s).async_).cmd;
    let mut flags = 0;
    spin_lock_irqsave(&mut (*p).spinlock, &mut flags);
    (*p).active = 1;
    if cmd.start_src == TRIG_INT { (*(*s).async_).inttrig = Some(pcmmio_inttrig_start_intr); } else { pcmmio_start_intr(dev, s); }
    spin_unlock_irqrestore(&mut (*p).spinlock, flags);
    0
}

unsafe fn pcmmio_cmdtest(_dev: *mut comedi_device, _s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> i32 {
    let mut err = 0;
    err |= comedi_check_trigger_src(&mut (*cmd).start_src, TRIG_NOW | TRIG_INT);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_begin_src, TRIG_EXT);
    err |= comedi_check_trigger_src(&mut (*cmd).convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&mut (*cmd).scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&mut (*cmd).stop_src, TRIG_COUNT | TRIG_NONE);
    if err != 0 { return 1; }
    err |= comedi_check_trigger_is_unique((*cmd).start_src);
    err |= comedi_check_trigger_is_unique((*cmd).stop_src);
    if err != 0 { return 2; }
    err |= comedi_check_trigger_arg_is(&mut (*cmd).start_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_begin_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).convert_arg, 0);
    err |= comedi_check_trigger_arg_is(&mut (*cmd).scan_end_arg, (*cmd).chanlist_len);
    if (*cmd).stop_src == TRIG_COUNT { err |= comedi_check_trigger_arg_min(&mut (*cmd).stop_arg, 1); } else { err |= comedi_check_trigger_arg_is(&mut (*cmd).stop_arg, 0); }
    if err != 0 { 3 } else { 0 }
}

unsafe fn pcmmio_attach(dev: *mut comedi_device, it: *mut comedi_devconfig) -> i32 {
    let ret = comedi_check_request_region(dev, (*it).options[0], 32, 0, 0xffff, 32);
    if ret != 0 { return ret; }
    let p = comedi_alloc_devpriv(dev, core::mem::size_of::<pcmmio_private>()) as *mut pcmmio_private;
    if p.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*p).pagelock);
    spin_lock_init(&mut (*p).spinlock);
    pcmmio_reset(dev);
    if (*it).options[1] != 0 {
        let irq_ret = request_irq((*it).options[1], Some(interrupt_pcmmio), 0, (*dev).board_name, dev as *mut c_void);
        if irq_ret == 0 {
            (*dev).irq = (*it).options[1];
            outb(PCMMIO_AI_RES_ENA_DIO_RES_ACCESS, (*dev).iobase + PCMMIO_AI_RES_ENA_REG);
            outb(PCMMIO_RESOURCE_IRQ((*dev).irq as u32) as u8, (*dev).iobase + PCMMIO_RESOURCE_REG);
        }
    }
    let ret = comedi_alloc_subdevices(dev, 4);
    if ret != 0 { return ret; }
    let s = (*dev).subdevices;
    (*s.add(0)).type_ = COMEDI_SUBD_AI; (*s.add(0)).subdev_flags = SDF_READABLE | SDF_GROUND | SDF_DIFF; (*s.add(0)).n_chan = 16; (*s.add(0)).maxdata = 0xffff; (*s.add(0)).range_table = &pcmmio_ai_ranges; (*s.add(0)).insn_read = Some(pcmmio_ai_insn_read);
    outb(PCMMIO_AI_RES_ENA_CMD_REG_ACCESS, (*dev).iobase + PCMMIO_AI_RES_ENA_REG); outb(PCMMIO_AI_RES_ENA_CMD_REG_ACCESS, (*dev).iobase + PCMMIO_AI_RES_ENA_REG + PCMMIO_AI_2ND_ADC_OFFSET);
    (*s.add(1)).type_ = COMEDI_SUBD_AO; (*s.add(1)).subdev_flags = SDF_READABLE; (*s.add(1)).n_chan = 8; (*s.add(1)).maxdata = 0xffff; (*s.add(1)).range_table = &pcmmio_ao_ranges; (*s.add(1)).insn_write = Some(pcmmio_ao_insn_write); comedi_alloc_subdev_readback(s.add(1));
    outb(0, (*dev).iobase + PCMMIO_AO_RESOURCE_ENA_REG); outb(0, (*dev).iobase + PCMMIO_AO_2ND_DAC_OFFSET + PCMMIO_AO_RESOURCE_ENA_REG);
    for i in 2..4 { (*s.add(i)).type_ = COMEDI_SUBD_DIO; (*s.add(i)).subdev_flags = SDF_READABLE | SDF_WRITABLE; (*s.add(i)).n_chan = 24; (*s.add(i)).maxdata = 1; (*s.add(i)).range_table = &range_digital; (*s.add(i)).insn_bits = Some(pcmmio_dio_insn_bits); (*s.add(i)).insn_config = Some(pcmmio_dio_insn_config); }
    0
}

// The declarations below are intentionally kept as external dependencies: the
// surrounding Comedi translation supplies their types, helpers, and registration.
extern "C" {
    static mut pcmmio_driver: comedi_driver;
}

// module_comedi_driver(pcmmio_driver);
// MODULE_AUTHOR("Comedi https://www.comedi.org");
// MODULE_DESCRIPTION("Comedi driver for Winsystems PCM-MIO PC/104 board");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
