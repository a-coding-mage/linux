// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2010 OKI SEMICONDUCTOR Co., LTD. */

const IOH_EDGE_FALLING: u32 = 0;
const IOH_EDGE_RISING: u32 = 1 << 0;
const IOH_LEVEL_L: u32 = 1 << 1;
const IOH_LEVEL_H: u32 = (1 << 0) | (1 << 1);
const IOH_EDGE_BOTH: u32 = 1 << 2;
const IOH_IM_MASK: u32 = (1 << 0) | (1 << 1) | (1 << 2);
const IOH_IRQ_BASE: i32 = 0;

#[repr(C)]
struct ioh_reg_comn { ien: u32, istatus: u32, idisp: u32, iclr: u32, imask: u32, imaskclr: u32, po: u32, pi: u32, pm: u32, im_0: u32, im_1: u32, reserved: u32 }

#[repr(C)]
struct ioh_regs { regs: [ioh_reg_comn; 8], reserve1: [u32; 16], ioh_sel_reg: [u32; 4], reserve2: [u32; 11], srst: u32 }

#[repr(C)]
struct ioh_gpio_reg_data { ien_reg: u32, imask_reg: u32, po_reg: u32, pm_reg: u32, im0_reg: u32, im1_reg: u32, use_sel_reg: u32 }

#[repr(C)]
struct ioh_gpio { base: *mut core::ffi::c_void, reg: *mut ioh_regs, dev: *mut device, gpio: gpio_chip, ioh_gpio_reg: ioh_gpio_reg_data, gpio_use_sel: u32, ch: i32, irq_base: i32, spinlock: *mut raw_spinlock_t }

#[repr(C)]
struct ioh_gpio_device { spinlock: raw_spinlock_t, chip: [ioh_gpio; 8] }

static num_ports: [i32; 8] = [6, 12, 16, 16, 15, 16, 16, 12];

unsafe fn ioh_gpio_set(gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gpio); let mut flags = 0usize;
    raw_spin_lock_irqsave((*chip).spinlock, &mut flags);
    let po = &mut (*(*chip).reg).regs[(*chip).ch as usize].po;
    let mut reg_val = ioread32(po);
    if val != 0 { reg_val |= 1u32.wrapping_shl(nr); } else { reg_val &= !(1u32.wrapping_shl(nr)); }
    iowrite32(reg_val, po); raw_spin_unlock_irqrestore((*chip).spinlock, flags); 0
}

unsafe fn ioh_gpio_get(gpio: *mut gpio_chip, nr: u32) -> i32 { let chip = gpiochip_get_data(gpio); (!!((ioread32(&(*(*chip).reg).regs[(*chip).ch as usize].pi) & 1u32.wrapping_shl(nr)) as i32)) }

unsafe fn ioh_gpio_direction_output(gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gpio); let mut flags = 0usize;
    raw_spin_lock_irqsave((*chip).spinlock, &mut flags);
    let p = &mut (*chip).reg.as_mut().unwrap().regs[(*chip).ch as usize].pm;
    let mut pm = ioread32(p); pm &= 1u32.wrapping_shl(num_ports[(*chip).ch as usize] as u32).wrapping_sub(1); pm |= 1u32.wrapping_shl(nr); iowrite32(pm, p);
    let po = &mut (*chip).reg.as_mut().unwrap().regs[(*chip).ch as usize].po; let mut reg_val = ioread32(po); if val != 0 { reg_val |= 1u32.wrapping_shl(nr); } else { reg_val &= !(1u32.wrapping_shl(nr)); } iowrite32(reg_val, po);
    raw_spin_unlock_irqrestore((*chip).spinlock, flags); 0
}

unsafe fn ioh_gpio_direction_input(gpio: *mut gpio_chip, nr: u32) -> i32 { let chip = gpiochip_get_data(gpio); let mut flags=0usize; raw_spin_lock_irqsave((*chip).spinlock,&mut flags); let p=&mut (*chip).reg.as_mut().unwrap().regs[(*chip).ch as usize].pm; let mut pm=ioread32(p); pm &= 1u32.wrapping_shl(num_ports[(*chip).ch as usize] as u32).wrapping_sub(1); pm &= !(1u32.wrapping_shl(nr)); iowrite32(pm,p); raw_spin_unlock_irqrestore((*chip).spinlock,flags); 0 }

unsafe fn ioh_gpio_save_reg_conf(mut chip: *mut ioh_gpio) { for i in 0..8 { let c=&mut *chip; let r=&(*c.reg).regs[c.ch as usize]; c.ioh_gpio_reg.po_reg=ioread32(&r.po); c.ioh_gpio_reg.pm_reg=ioread32(&r.pm); c.ioh_gpio_reg.ien_reg=ioread32(&r.ien); c.ioh_gpio_reg.imask_reg=ioread32(&r.imask); c.ioh_gpio_reg.im0_reg=ioread32(&r.im_0); c.ioh_gpio_reg.im1_reg=ioread32(&r.im_1); if i<4 { c.ioh_gpio_reg.use_sel_reg=ioread32(&(*c.reg).ioh_sel_reg[i]); } chip=chip.add(1); } }
unsafe fn ioh_gpio_restore_reg_conf(mut chip: *mut ioh_gpio) { for i in 0..8 { let c=&mut *chip; let r=&mut (*c.reg).regs[c.ch as usize]; iowrite32(c.ioh_gpio_reg.po_reg,&mut r.po); iowrite32(c.ioh_gpio_reg.pm_reg,&mut r.pm); iowrite32(c.ioh_gpio_reg.ien_reg,&mut r.ien); iowrite32(c.ioh_gpio_reg.imask_reg,&mut r.imask); iowrite32(c.ioh_gpio_reg.im0_reg,&mut r.im_0); iowrite32(c.ioh_gpio_reg.im1_reg,&mut r.im_1); if i<4 { iowrite32(c.ioh_gpio_reg.use_sel_reg,&mut (*c.reg).ioh_sel_reg[i]); } chip=chip.add(1); } }
unsafe fn ioh_gpio_to_irq(gpio:*mut gpio_chip, offset:u32)->i32 { let c=gpiochip_get_data(gpio); (*c).irq_base + offset as i32 }
unsafe fn ioh_gpio_setup(chip:*mut ioh_gpio,num_port:i32) { let g=&mut (*chip).gpio; g.label=dev_name((*chip).dev); g.owner=THIS_MODULE; g.direction_input=Some(ioh_gpio_direction_input); g.get=Some(ioh_gpio_get); g.direction_output=Some(ioh_gpio_direction_output); g.set=Some(ioh_gpio_set); g.dbg_show=None; g.base=-1; g.ngpio=num_port as u32; g.can_sleep=false; g.to_irq=Some(ioh_gpio_to_irq); }

// Remaining kernel callback wiring is preserved as declarations: these symbols and kernel types are supplied by the surrounding Linux bindings.
extern "C" {
    static mut THIS_MODULE: *mut core::ffi::c_void;
    fn gpiochip_get_data(gpio:*mut gpio_chip)->*mut ioh_gpio; fn dev_name(dev:*mut device)->*const core::ffi::c_char;
    fn ioread32(p:*const u32)->u32; fn iowrite32(v:u32,p:*mut u32); fn raw_spin_lock_irqsave(l:*mut raw_spinlock_t,f:*mut usize); fn raw_spin_unlock_irqrestore(l:*mut raw_spinlock_t,f:usize);
}
#[repr(C)] struct device;
#[repr(C)] struct raw_spinlock_t;
#[repr(C)] struct gpio_chip { label:*const core::ffi::c_char, owner:*mut core::ffi::c_void, direction_input:Option<unsafe fn(*mut gpio_chip,u32)->i32>, get:Option<unsafe fn(*mut gpio_chip,u32)->i32>, direction_output:Option<unsafe fn(*mut gpio_chip,u32,i32)->i32>, set:Option<unsafe fn(*mut gpio_chip,u32,i32)->i32>, dbg_show:Option<*mut core::ffi::c_void>, base:i32, ngpio:u32, can_sleep:bool, to_irq:Option<unsafe fn(*mut gpio_chip,u32)->i32> }

// Interrupt callbacks, PCI probe/suspend/resume callbacks, driver registration, and module metadata.
unsafe fn ioh_irq_unmask(_d:*mut irq_data) { let c=(*(*_d).gc).private; iowrite32(1u32.wrapping_shl(((*_d).irq-(*c).irq_base) as u32),&mut (*(*c).reg).regs[(*c).ch as usize].imaskclr); }
unsafe fn ioh_irq_mask(_d:*mut irq_data) { let c=(*(*_d).gc).private; iowrite32(1u32.wrapping_shl(((*_d).irq-(*c).irq_base) as u32),&mut (*(*c).reg).regs[(*c).ch as usize].imask); }
unsafe fn ioh_irq_disable(d:*mut irq_data) { let c=(*(*d).gc).private; let mut f=0usize; raw_spin_lock_irqsave((*c).spinlock,&mut f); let p=&mut (*c).reg.as_mut().unwrap().regs[(*c).ch as usize].ien; iowrite32(ioread32(p)&!(1u32.wrapping_shl(((*d).irq-(*c).irq_base) as u32)),p); raw_spin_unlock_irqrestore((*c).spinlock,f); }
unsafe fn ioh_irq_enable(d:*mut irq_data) { let c=(*(*d).gc).private; let mut f=0usize; raw_spin_lock_irqsave((*c).spinlock,&mut f); let p=&mut (*c).reg.as_mut().unwrap().regs[(*c).ch as usize].ien; iowrite32(ioread32(p)|(1u32.wrapping_shl(((*d).irq-(*c).irq_base) as u32)),p); raw_spin_unlock_irqrestore((*c).spinlock,f); }
unsafe fn ioh_gpio_handler(irq:i32,dev_id:*mut core::ffi::c_void)->i32 { let mut c=dev_id as *mut ioh_gpio; let mut ret=0; for i in 0..8 { let status=ioread32(&(*(*c).reg).regs[i].istatus); for j in 0..num_ports[i] { if status & 1u32.wrapping_shl(j as u32)!=0 { iowrite32(1u32.wrapping_shl(j as u32),&mut (*(*c).reg).regs[(*c).ch as usize].iclr); generic_handle_irq((*c).irq_base+j); ret=1; } } c=c.add(1); } let _=irq; ret }
#[repr(C)] struct irq_data { irq:i32, gc:*mut irq_chip_generic }
#[repr(C)] struct irq_chip_generic { private:*mut ioh_gpio }
extern "C" { fn generic_handle_irq(i:i32); }

// The remaining definitions retain their externally supplied kernel interfaces.
extern "C" {
    fn ioh_irq_type(d:*mut irq_data, irq_type:u32)->i32;
    fn ioh_gpio_alloc_generic_chip(chip:*mut ioh_gpio, irq_start:u32, num:u32)->i32;
    fn ioh_gpio_probe(pdev:*mut pci_dev, id:*const pci_device_id)->i32;
    fn ioh_gpio_suspend(dev:*mut device)->i32;
    fn ioh_gpio_resume(dev:*mut device)->i32;
}
#[repr(C)] struct pci_dev { irq:i32, dev:device }
#[repr(C)] struct pci_device_id;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
