// SPDX-License-Identifier: GPL-2.0+
/*
 * comedi/drivers/cb_pcimdas.c
 * Comedi driver for Computer Boards PCIM-DAS1602/16 and PCIe-DAS1602/16
 */

// Linux kernel/comedi dependencies are supplied by the surrounding repository.

const PCIMDAS_AI_REG: usize = 0x00;
const PCIMDAS_AI_SOFTTRIG_REG: usize = 0x00;
#[inline] const fn PCIMDAS_AO_REG(x: usize) -> usize { 0x02 + x * 2 }
const PCIMDAS_MUX_REG: usize = 0x00;
#[inline] const fn PCIMDAS_MUX(lo: u32, hi: u32) -> u32 { lo | (hi << 4) }
const PCIMDAS_DI_DO_REG: usize = 0x01;
const PCIMDAS_STATUS_REG: usize = 0x02;
const PCIMDAS_STATUS_EOC: u32 = 1 << 7;
const PCIMDAS_STATUS_UB: u32 = 1 << 6;
const PCIMDAS_STATUS_MUX: u32 = 1 << 5;
const PCIMDAS_STATUS_CLK: u32 = 1 << 4;
#[inline] const fn PCIMDAS_STATUS_TO_CURR_MUX(x: u32) -> u32 { x & 0xf }
const PCIMDAS_CONV_STATUS_REG: usize = 0x03;
const PCIMDAS_CONV_STATUS_EOC: u32 = 1 << 7;
const PCIMDAS_CONV_STATUS_EOB: u32 = 1 << 6;
const PCIMDAS_CONV_STATUS_EOA: u32 = 1 << 5;
const PCIMDAS_CONV_STATUS_FNE: u32 = 1 << 4;
const PCIMDAS_CONV_STATUS_FHF: u32 = 1 << 3;
const PCIMDAS_CONV_STATUS_OVERRUN: u32 = 1 << 2;
const PCIMDAS_IRQ_REG: usize = 0x04;
const PCIMDAS_IRQ_INTE: u32 = 1 << 7;
const PCIMDAS_IRQ_INT: u32 = 1 << 6;
const PCIMDAS_IRQ_OVERRUN: u32 = 1 << 4;
const PCIMDAS_IRQ_EOA: u32 = 1 << 3;
const PCIMDAS_IRQ_EOA_INT_SEL: u32 = 1 << 2;
#[inline] const fn PCIMDAS_IRQ_INTSEL(x: u32) -> u32 { x << 0 }
const PCIMDAS_IRQ_INTSEL_EOC: u32 = PCIMDAS_IRQ_INTSEL(0);
const PCIMDAS_IRQ_INTSEL_FNE: u32 = PCIMDAS_IRQ_INTSEL(1);
const PCIMDAS_IRQ_INTSEL_EOB: u32 = PCIMDAS_IRQ_INTSEL(2);
const PCIMDAS_IRQ_INTSEL_FHF_EOA: u32 = PCIMDAS_IRQ_INTSEL(3);
const PCIMDAS_PACER_REG: usize = 0x05;
const PCIMDAS_PACER_GATE_STATUS: u32 = 1 << 6;
const PCIMDAS_PACER_GATE_POL: u32 = 1 << 5;
const PCIMDAS_PACER_GATE_LATCH: u32 = 1 << 4;
const PCIMDAS_PACER_GATE_EN: u32 = 1 << 3;
const PCIMDAS_PACER_EXT_PACER_POL: u32 = 1 << 2;
#[inline] const fn PCIMDAS_PACER_SRC(x: u32) -> u32 { x << 0 }
const PCIMDAS_PACER_SRC_POLLED: u32 = PCIMDAS_PACER_SRC(0);
const PCIMDAS_PACER_SRC_EXT: u32 = PCIMDAS_PACER_SRC(2);
const PCIMDAS_PACER_SRC_INT: u32 = PCIMDAS_PACER_SRC(3);
const PCIMDAS_PACER_SRC_MASK: u32 = 3 << 0;
const PCIMDAS_BURST_REG: usize = 0x06;
const PCIMDAS_BURST_BME: u32 = 1 << 1;
const PCIMDAS_BURST_CONV_EN: u32 = 1 << 0;
const PCIMDAS_GAIN_REG: usize = 0x07;
const PCIMDAS_8254_BASE: usize = 0x08;
const PCIMDAS_USER_CNTR_REG: usize = 0x0c;
const PCIMDAS_USER_CNTR_CTR1_CLK_SEL: u32 = 1 << 0;
const PCIMDAS_RESIDUE_MSB_REG: usize = 0x0d;
const PCIMDAS_RESIDUE_LSB_REG: usize = 0x0e;
const PCIMDAS_8255_BASE: usize = 0x00;

static cb_pcimdas_ai_bip_range: comedi_lrange = comedi_lrange { length: 4, range: [BIP_RANGE(10), BIP_RANGE(5), BIP_RANGE(2.5), BIP_RANGE(1.25)] };
static cb_pcimdas_ai_uni_range: comedi_lrange = comedi_lrange { length: 4, range: [UNI_RANGE(10), UNI_RANGE(5), UNI_RANGE(2.5), UNI_RANGE(1.25)] };
static cb_pcimdas_ao_range: comedi_lrange = comedi_lrange { length: 6, range: [BIP_RANGE(10), BIP_RANGE(5), UNI_RANGE(10), UNI_RANGE(5), RANGE_ext(-1, 1), RANGE_ext(0, 1)] };

#[repr(C)]
struct cb_pcimdas_private { daqio: libc::c_ulong, BADR3: libc::c_ulong }

unsafe fn cb_pcimdas_ai_eoc(dev: *mut comedi_device, _s: *mut comedi_subdevice, _insn: *mut comedi_insn, _context: libc::c_ulong) -> libc::c_int {
    let devpriv = (*dev).private as *mut cb_pcimdas_private;
    let status = inb((*devpriv).BADR3 + PCIMDAS_STATUS_REG as libc::c_ulong);
    if status & PCIMDAS_STATUS_EOC != 0 { 0 } else { -EBUSY }
}

unsafe fn cb_pcimdas_ai_insn_read(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut libc::c_uint) -> libc::c_int {
    let devpriv = (*dev).private as *mut cb_pcimdas_private;
    let chan = CR_CHAN((*insn).chanspec); let range = CR_RANGE((*insn).chanspec);
    let mut d = inb((*devpriv).BADR3 + PCIMDAS_PACER_REG as libc::c_ulong);
    if d & PCIMDAS_PACER_SRC_MASK != PCIMDAS_PACER_SRC_POLLED { d &= !PCIMDAS_PACER_SRC_MASK; d |= PCIMDAS_PACER_SRC_POLLED; outb(d, (*devpriv).BADR3 + PCIMDAS_PACER_REG as libc::c_ulong); }
    outb(PCIMDAS_BURST_CONV_EN, (*devpriv).BADR3 + PCIMDAS_BURST_REG as libc::c_ulong);
    outb(range, (*devpriv).BADR3 + PCIMDAS_GAIN_REG as libc::c_ulong);
    outb(PCIMDAS_MUX(chan, chan), (*devpriv).BADR3 + PCIMDAS_MUX_REG as libc::c_ulong);
    let mut n = 0; while n < (*insn).n { outw(0, (*devpriv).daqio + PCIMDAS_AI_SOFTTRIG_REG as libc::c_ulong); let ret = comedi_timeout(dev, s, insn, Some(cb_pcimdas_ai_eoc), 0); if ret != 0 { return ret; } *data.add(n as usize) = inw((*devpriv).daqio + PCIMDAS_AI_REG as libc::c_ulong); n += 1; } n as libc::c_int
}

unsafe fn cb_pcimdas_ao_insn_write(_dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut libc::c_uint) -> libc::c_int {
    let devpriv = (*_dev).private as *mut cb_pcimdas_private; let chan = CR_CHAN((*insn).chanspec); let mut val = (*s).readback[chan as usize]; let mut i = 0; while i < (*insn).n { val = *data.add(i as usize); outw(val, (*devpriv).daqio + PCIMDAS_AO_REG(chan as usize) as libc::c_ulong); i += 1; } (*s).readback[chan as usize] = val; (*insn).n as libc::c_int
}

unsafe fn cb_pcimdas_di_insn_bits(_dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut libc::c_uint) -> libc::c_int { let devpriv = (*_dev).private as *mut cb_pcimdas_private; *data.add(1) = inb((*devpriv).BADR3 + PCIMDAS_DI_DO_REG as libc::c_ulong) & 0x0f; (*insn).n as libc::c_int }
unsafe fn cb_pcimdas_do_insn_bits(_dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut libc::c_uint) -> libc::c_int { let devpriv = (*_dev).private as *mut cb_pcimdas_private; if comedi_dio_update_state(s, data) != 0 { outb((*s).state, (*devpriv).BADR3 + PCIMDAS_DI_DO_REG as libc::c_ulong); } *data.add(1) = (*s).state; (*insn).n as libc::c_int }

unsafe fn cb_pcimdas_counter_insn_config(_dev: *mut comedi_device, _s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut libc::c_uint) -> libc::c_int { let devpriv = (*_dev).private as *mut cb_pcimdas_private; match *data { INSN_CONFIG_SET_CLOCK_SRC => match *data.add(1) { 0 => outb(PCIMDAS_USER_CNTR_CTR1_CLK_SEL, (*devpriv).BADR3 + PCIMDAS_USER_CNTR_REG as libc::c_ulong), 1 => outb(0, (*devpriv).BADR3 + PCIMDAS_USER_CNTR_REG as libc::c_ulong), _ => return -EINVAL }, INSN_CONFIG_GET_CLOCK_SRC => { let ctrl = inb((*devpriv).BADR3 + PCIMDAS_USER_CNTR_REG as libc::c_ulong); if ctrl & PCIMDAS_USER_CNTR_CTR1_CLK_SEL != 0 { *data.add(1)=0; *data.add(2)=I8254_OSC_BASE_100KHZ; } else { *data.add(1)=1; *data.add(2)=0; } }, _ => return -EINVAL } (*insn).n as libc::c_int }

unsafe fn cb_pcimdas_pacer_clk(dev: *mut comedi_device) -> libc::c_uint { let p = (*dev).private as *mut cb_pcimdas_private; if inb((*p).BADR3 + PCIMDAS_STATUS_REG as libc::c_ulong) & PCIMDAS_STATUS_CLK != 0 { I8254_OSC_BASE_10MHZ } else { I8254_OSC_BASE_1MHZ } }
unsafe fn cb_pcimdas_is_ai_se(dev: *mut comedi_device) -> bool { let p=(*dev).private as *mut cb_pcimdas_private; inb((*p).BADR3+PCIMDAS_STATUS_REG as libc::c_ulong)&PCIMDAS_STATUS_MUX != 0 }
unsafe fn cb_pcimdas_is_ai_uni(dev: *mut comedi_device) -> bool { let p=(*dev).private as *mut cb_pcimdas_private; inb((*p).BADR3+PCIMDAS_STATUS_REG as libc::c_ulong)&PCIMDAS_STATUS_UB != 0 }

unsafe fn cb_pcimdas_auto_attach(dev: *mut comedi_device, _context_unused: libc::c_ulong) -> libc::c_int {
    let pcidev = comedi_to_pci_dev(dev);
    let devpriv = comedi_alloc_devpriv(dev, core::mem::size_of::<cb_pcimdas_private>()) as *mut cb_pcimdas_private;
    if devpriv.is_null() { return -ENOMEM; }
    let ret = comedi_pci_enable(dev); if ret != 0 { return ret; }
    (*devpriv).daqio = pci_resource_start(pcidev, 2); (*devpriv).BADR3 = pci_resource_start(pcidev, 3); (*dev).iobase = pci_resource_start(pcidev, 4);
    (*dev).pacer = comedi_8254_io_alloc((*devpriv).BADR3 + PCIMDAS_8254_BASE as libc::c_ulong, cb_pcimdas_pacer_clk(dev), I8254_IO8, 0);
    if IS_ERR((*dev).pacer) { return PTR_ERR((*dev).pacer); }
    let ret = comedi_alloc_subdevices(dev, 6); if ret != 0 { return ret; }
    let s = (*dev).subdevices.add(0); (*s).type_ = COMEDI_SUBD_AI; (*s).subdev_flags = SDF_READABLE;
    if cb_pcimdas_is_ai_se(dev) { (*s).subdev_flags |= SDF_GROUND; (*s).n_chan=16; } else { (*s).subdev_flags |= SDF_DIFF; (*s).n_chan=8; }
    (*s).maxdata=0xffff; (*s).range_table=if cb_pcimdas_is_ai_uni(dev) { &cb_pcimdas_ai_uni_range } else { &cb_pcimdas_ai_bip_range }; (*s).insn_read=Some(cb_pcimdas_ai_insn_read);
    let s = (*dev).subdevices.add(1); (*s).type_=COMEDI_SUBD_AO; (*s).subdev_flags=SDF_WRITABLE; (*s).n_chan=2; (*s).maxdata=0xfff; (*s).range_table=&cb_pcimdas_ao_range; (*s).insn_write=Some(cb_pcimdas_ao_insn_write); ret = comedi_alloc_subdev_readback(s); if ret != 0 { return ret; }
    let s = (*dev).subdevices.add(2); ret=subdev_8255_io_init(dev,s,PCIMDAS_8255_BASE); if ret != 0{return ret;}
    let s=(*dev).subdevices.add(3); (*s).type_=COMEDI_SUBD_DI; (*s).subdev_flags=SDF_READABLE; (*s).n_chan=4; (*s).maxdata=1; (*s).range_table=&range_digital; (*s).insn_bits=Some(cb_pcimdas_di_insn_bits);
    let s=(*dev).subdevices.add(4); (*s).type_=COMEDI_SUBD_DO; (*s).subdev_flags=SDF_WRITABLE; (*s).n_chan=4; (*s).maxdata=1; (*s).range_table=&range_digital; (*s).insn_bits=Some(cb_pcimdas_do_insn_bits);
    let s=(*dev).subdevices.add(5); comedi_8254_subdevice_init(s,(*dev).pacer); (*(*dev).pacer).insn_config=Some(cb_pcimdas_counter_insn_config); comedi_8254_set_busy((*dev).pacer,1,true); comedi_8254_set_busy((*dev).pacer,2,true); 0
}

// PCI driver registration and module metadata are supplied by the kernel/comedi integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
