// SPDX-License-Identifier: GPL-2.0
// Direct low-level translation of comedi/drivers/adv_pci_dio.c.

const PCI173X_INT_EN_REG: usize = 0x0008;
const PCI173X_INT_RF_REG: usize = 0x000c;
const PCI173X_INT_FLAG_REG: usize = 0x0010;
const PCI173X_INT_CLR_REG: usize = 0x0010;
const PCI173X_INT_IDI0: u16 = 0x01;
const PCI173X_INT_IDI1: u16 = 0x02;
const PCI173X_INT_DI0: u16 = 0x04;
const PCI173X_INT_DI1: u16 = 0x08;
const PCI1750_INT_REG: usize = 0x20;
const PCI1752_CFC_REG: usize = 0x12;
const PCI1761_INT_EN_REG: usize = 0x03;
const PCI1761_INT_RF_REG: usize = 0x04;
const PCI1761_INT_CLR_REG: usize = 0x05;
const PCI1762_INT_REG: usize = 0x06;
const PCI_DIO_MAX_DI_SUBDEVS: usize = 2;
const PCI_DIO_MAX_DO_SUBDEVS: usize = 2;
const PCI_DIO_MAX_DIO_SUBDEVG: usize = 2;
const PCI_DIO_MAX_IRQ_SUBDEVS: usize = 4;

#[inline] fn pci1753_int_reg(x: usize) -> usize { 0x10 + x }
#[inline] fn pci1753e_int_reg(x: usize) -> usize { 0x30 + x }
#[inline] fn pci1754_int_reg(x: usize) -> usize { 0x08 + x * 2 }

#[repr(C)]
#[derive(Copy, Clone)]
enum pci_dio_boardid { TYPE_PCI1730, TYPE_PCI1733, TYPE_PCI1734, TYPE_PCI1735, TYPE_PCI1736, TYPE_PCI1739, TYPE_PCI1750, TYPE_PCI1751, TYPE_PCI1752, TYPE_PCI1753, TYPE_PCI1753E, TYPE_PCI1754, TYPE_PCI1756, TYPE_PCI1761, TYPE_PCI1762 }

#[repr(C)] #[derive(Copy, Clone)] struct diosubd_data { chans: i32, addr: usize }
#[repr(C)] #[derive(Copy, Clone)] struct dio_irq_subd_data { int_en: u16, addr: usize }
#[repr(C)] struct dio_boardtype { name: *const u8, nsubdevs: i32, sdi: [diosubd_data;2], sdo: [diosubd_data;2], sdio: [diosubd_data;2], sdirq: [dio_irq_subd_data;4], id_reg: usize, timer_regbase: usize, is_16bit: bool }
#[repr(C)] struct pci_dio_dev_private_data { boardtype: i32, irq_subd: i32, int_ctrl: u16, int_rf: u16 }
#[repr(C)] struct pci_dio_sd_private_data { subd_slock: spinlock_t, port_offset: usize, cmd_running: i16 }

// External kernel/comedi declarations are supplied by the surrounding translation unit.
extern "C" { fn inb(p: usize) -> u8; fn inw(p: usize) -> u16; fn outb(v: u8, p: usize); fn outw(v: u16, p: usize); }

unsafe fn process_irq(dev: *mut comedi_device, subdev: usize, _irqflags: u8) {
    let s = &mut (*dev).subdevices.add(subdev).as_mut().unwrap();
    let sd_priv = s.private as *mut pci_dio_sd_private_data;
    let reg = (*sd_priv).port_offset;
    let async_p = s.async_;
    if !async_p.is_null() {
        let val = inw((*dev).iobase + reg);
        spin_lock(&mut (*sd_priv).subd_slock);
        if (*sd_priv).cmd_running != 0 { comedi_buf_write_samples(s, &val as *const _ as *const _, 1); }
        spin_unlock(&mut (*sd_priv).subd_slock);
        comedi_handle_events(dev, s);
    }
}

unsafe extern "C" fn pci_dio_interrupt(_irq: i32, p_device: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = p_device as *mut comedi_device; let dp = (*dev).private as *mut pci_dio_dev_private_data;
    let board = (*dev).board_ptr as *const dio_boardtype; let mut flags = 0usize;
    if !(*dev).attached { return IRQ_NONE; }
    spin_lock_irqsave(&mut (*dev).spinlock, &mut flags);
    let irqflags = inb((*dev).iobase + PCI173X_INT_FLAG_REG);
    if irqflags & 0x0f == 0 { spin_unlock_irqrestore(&mut (*dev).spinlock, flags); return IRQ_NONE; }
    outb(irqflags, (*dev).iobase + PCI173X_INT_CLR_REG); spin_unlock_irqrestore(&mut (*dev).spinlock, flags);
    for i in 0..PCI_DIO_MAX_IRQ_SUBDEVS { if irqflags as u16 & (*board).sdirq[i].int_en != 0 { process_irq(dev, ((*dp).irq_subd + i as i32) as usize, irqflags); } }
    IRQ_HANDLED
}

unsafe fn pci_dio_reset(dev: *mut comedi_device, cardtype: usize) -> i32 {
    let dp = (*dev).private as *mut pci_dio_dev_private_data; let b = (*dev).iobase;
    if cardtype == TYPE_PCI1752 as usize || cardtype == TYPE_PCI1756 as usize { outw(0,b+PCI1752_CFC_REG); }
    match cardtype {
        x if x==TYPE_PCI1730 as usize || x==TYPE_PCI1733 as usize || x==TYPE_PCI1736 as usize => { (*dp).int_ctrl=0; outb(0,b+PCI173X_INT_EN_REG); outb(0x0f,b+PCI173X_INT_CLR_REG); (*dp).int_rf=0; outb(0,b+PCI173X_INT_RF_REG); },
        x if x==TYPE_PCI1739 as usize || x==TYPE_PCI1750 as usize || x==TYPE_PCI1751 as usize => outb(0x88,b+PCI1750_INT_REG),
        x if x==TYPE_PCI1753 as usize || x==TYPE_PCI1753E as usize => { for i in 0..4 { outb(if i==0 {0x88} else {0x80},b+pci1753_int_reg(i)); } if x==TYPE_PCI1753E as usize { for i in 0..4 { outb(if i==0 {0x88} else {0x80},b+pci1753e_int_reg(i)); } } },
        x if x==TYPE_PCI1754 as usize || x==TYPE_PCI1756 as usize => { for i in 0..2 { outw(8,b+pci1754_int_reg(i)); } if x==TYPE_PCI1754 as usize { for i in 2..4 { outw(8,b+pci1754_int_reg(i)); } } },
        x if x==TYPE_PCI1761 as usize => { outb(0,b+PCI1761_INT_EN_REG); outb(0xff,b+PCI1761_INT_CLR_REG); outb(0,b+PCI1761_INT_RF_REG); },
        x if x==TYPE_PCI1762 as usize => outw(0x0101,b+PCI1762_INT_REG), _ => {}
    } 0
}

// The remaining driver registration and subdevice setup retain the C ABI and are intentionally declared here.
extern "C" { fn pci_dio_auto_attach(dev:*mut comedi_device, context:usize)->i32; fn pci_dio_detach(dev:*mut comedi_device); }

unsafe fn pci_dio_insn_bits_di_b(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).iobase+(*s).private as usize; *data.add(1)=inb(p) as u32; if (*s).n_chan>8 {*data.add(1)|=(inb(p+1) as u32)<<8;} if (*s).n_chan>16 {*data.add(1)|=(inb(p+2) as u32)<<16;} if (*s).n_chan>24 {*data.add(1)|=(inb(p+3) as u32)<<24;} (*insn).n as i32 }
unsafe fn pci_dio_insn_bits_di_w(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).iobase+(*s).private as usize; *data.add(1)=inw(p) as u32; if (*s).n_chan>16 {*data.add(1)|=(inw(p+2) as u32)<<16;} (*insn).n as i32 }
unsafe fn pci_dio_insn_bits_dirq_b(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).iobase+(*(s as *mut comedi_subdevice)).private as *mut pci_dio_sd_private_data as usize; *data.add(1)=inb(p) as u32; (*insn).n as i32 }
unsafe fn pci_dio_insn_bits_do_b(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).iobase+(*s).private as usize; if comedi_dio_update_state(s,data)!=0 {outb((*s).state as u8,p); if (*s).n_chan>8 {outb(((*s).state>>8) as u8,p+1);} if (*s).n_chan>16 {outb(((*s).state>>16) as u8,p+2);} if (*s).n_chan>24 {outb(((*s).state>>24) as u8,p+3);}} *data.add(1)=(*s).state; (*insn).n as i32 }
unsafe fn pci_dio_insn_bits_do_w(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32 { let p=(*dev).iobase+(*s).private as usize; if comedi_dio_update_state(s,data)!=0 {outw((*s).state as u16,p); if (*s).n_chan>16 {outw(((*s).state>>16) as u16,p+2);}} *data.add(1)=(*s).state; (*insn).n as i32 }

// Constants and external types/functions below are provided by the comedi/kernel translation.
extern "C" { fn comedi_dio_update_state(*mut comedi_subdevice,*mut u32)->i32; fn comedi_buf_write_samples(*mut comedi_subdevice,*const core::ffi::c_void,usize); fn comedi_handle_events(*mut comedi_device,*mut comedi_subdevice); fn spin_lock(*mut spinlock_t); fn spin_unlock(*mut spinlock_t); fn spin_lock_irqsave(*mut spinlock_t,*mut usize); fn spin_unlock_irqrestore(*mut spinlock_t,usize); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
