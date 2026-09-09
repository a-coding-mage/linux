// SPDX-License-Identifier: GPL-2.0
/* Renesas R-Car GPIO Support; direct low-level translation of gpio-rcar.c. */

// Kernel types, constants, macros, and functions referenced below are supplied by
// the surrounding kernel bindings/dependencies.

#[repr(C)]
pub struct gpio_rcar_bank_info { pub iointsel: u32, pub inoutsel: u32, pub outdt: u32, pub posneg: u32, pub edglevel: u32, pub bothedge: u32, pub intmsk: u32 }
#[repr(C)]
pub struct gpio_rcar_info { pub has_outdtsel: bool, pub has_both_edge_trigger: bool, pub has_always_in: bool, pub has_inen: bool }
#[repr(C)]
pub struct gpio_rcar_priv {
    pub base: *mut core::ffi::c_void,
    pub lock: raw_spinlock_t,
    pub dev: *mut device,
    pub gpio_chip: gpio_chip,
    pub irq_parent: u32,
    pub wakeup_path: atomic_t,
    pub info: gpio_rcar_info,
    pub bank_info: gpio_rcar_bank_info,
}

pub const IOINTSEL: i32 = 0x00; pub const INOUTSEL: i32 = 0x04; pub const OUTDT: i32 = 0x08;
pub const INDT: i32 = 0x0c; pub const INTDT: i32 = 0x10; pub const INTCLR: i32 = 0x14;
pub const INTMSK: i32 = 0x18; pub const MSKCLR: i32 = 0x1c; pub const POSNEG: i32 = 0x20;
pub const EDGLEVEL: i32 = 0x24; pub const OUTDTSEL: i32 = 0x40; pub const BOTHEDGE: i32 = 0x4c;
pub const INEN: i32 = 0x50; pub const RCAR_MAX_GPIO_PER_BANK: u32 = 32;

#[inline] unsafe fn gpio_rcar_read(p: *mut gpio_rcar_priv, offs: i32) -> u32 { ioread32((*p).base.add(offs as usize)) }
#[inline] unsafe fn gpio_rcar_write(p: *mut gpio_rcar_priv, offs: i32, value: u32) { iowrite32(value, (*p).base.add(offs as usize)); }
unsafe fn gpio_rcar_modify_bit(p: *mut gpio_rcar_priv, offs: i32, bit: u32, value: bool) { let mut tmp = gpio_rcar_read(p, offs); if value { tmp |= BIT(bit); } else { tmp &= !BIT(bit); } gpio_rcar_write(p, offs, tmp); }

unsafe fn gpio_rcar_irq_disable(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let p = gpiochip_get_data(gc); let hwirq = irqd_to_hwirq(d); gpio_rcar_write(p, INTMSK, !BIT(hwirq)); gpiochip_disable_irq(gc, hwirq); }
unsafe fn gpio_rcar_irq_enable(d: *mut irq_data) { let gc = irq_data_get_irq_chip_data(d); let p = gpiochip_get_data(gc); let hwirq = irqd_to_hwirq(d); gpiochip_enable_irq(gc, hwirq); gpio_rcar_write(p, MSKCLR, BIT(hwirq)); }

unsafe fn gpio_rcar_config_interrupt_input_mode(p: *mut gpio_rcar_priv, hwirq: u32, active_high_rising_edge: bool, level_trigger: bool, both: bool) {
    let mut flags = 0; raw_spin_lock_irqsave(&mut (*p).lock, &mut flags);
    gpio_rcar_modify_bit(p, POSNEG, hwirq, !active_high_rising_edge); gpio_rcar_modify_bit(p, EDGLEVEL, hwirq, !level_trigger);
    if (*p).info.has_both_edge_trigger { gpio_rcar_modify_bit(p, BOTHEDGE, hwirq, both); }
    gpio_rcar_modify_bit(p, IOINTSEL, hwirq, true); if !level_trigger { gpio_rcar_write(p, INTCLR, BIT(hwirq)); }
    raw_spin_unlock_irqrestore(&mut (*p).lock, flags);
}

unsafe fn gpio_rcar_irq_set_type(d: *mut irq_data, typ: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d); let p = gpiochip_get_data(gc); let hwirq = irqd_to_hwirq(d);
    dev_dbg((*p).dev, "sense irq = %d, type = %d\n", hwirq, typ);
    match typ & IRQ_TYPE_SENSE_MASK { IRQ_TYPE_LEVEL_HIGH => gpio_rcar_config_interrupt_input_mode(p,hwirq,true,true,false), IRQ_TYPE_LEVEL_LOW => gpio_rcar_config_interrupt_input_mode(p,hwirq,false,true,false), IRQ_TYPE_EDGE_RISING => gpio_rcar_config_interrupt_input_mode(p,hwirq,true,false,false), IRQ_TYPE_EDGE_FALLING => gpio_rcar_config_interrupt_input_mode(p,hwirq,false,false,false), IRQ_TYPE_EDGE_BOTH => { if !(*p).info.has_both_edge_trigger { return -EINVAL; } gpio_rcar_config_interrupt_input_mode(p,hwirq,true,false,true) }, _ => return -EINVAL } 0
}
unsafe fn gpio_rcar_irq_set_wake(d: *mut irq_data, on: u32) -> i32 { let gc=irq_data_get_irq_chip_data(d); let p=gpiochip_get_data(gc); if (*p).irq_parent != 0 { let e=irq_set_irq_wake((*p).irq_parent,on); if e != 0 { dev_dbg((*p).dev,"irq %u doesn't support irq_set_wake\n",(*p).irq_parent); (*p).irq_parent=0; } } if on != 0 { atomic_inc(&mut (*p).wakeup_path); } else { atomic_dec(&mut (*p).wakeup_path); } 0 }

static gpio_rcar_irq_chip: irq_chip = irq_chip { name: "gpio-rcar", irq_mask: Some(gpio_rcar_irq_disable), irq_unmask: Some(gpio_rcar_irq_enable), irq_set_type: Some(gpio_rcar_irq_set_type), irq_set_wake: Some(gpio_rcar_irq_set_wake), flags: IRQCHIP_IMMUTABLE | IRQCHIP_SET_TYPE_MASKED | IRQCHIP_MASK_ON_SUSPEND };

unsafe fn gpio_rcar_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t { let p=dev_id as *mut gpio_rcar_priv; let mut handled=0; loop { let pending=gpio_rcar_read(p,INTDT)&gpio_rcar_read(p,INTMSK); if pending==0 { break; } let offset=__ffs(pending); gpio_rcar_write(p,INTCLR,BIT(offset)); generic_handle_domain_irq((*p).gpio_chip.irq.domain,offset); handled+=1; } if handled != 0 { IRQ_HANDLED } else { IRQ_NONE } }

unsafe fn gpio_rcar_config_general_input_output_mode(chip: *mut gpio_chip, gpio: u32, output: bool) { let p=gpiochip_get_data(chip); let mut flags=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut flags); gpio_rcar_modify_bit(p,POSNEG,gpio,false); gpio_rcar_modify_bit(p,IOINTSEL,gpio,false); gpio_rcar_modify_bit(p,INOUTSEL,gpio,output); if (*p).info.has_outdtsel && output { gpio_rcar_modify_bit(p,OUTDTSEL,gpio,false); } raw_spin_unlock_irqrestore(&mut (*p).lock,flags); }
unsafe fn gpio_rcar_request(chip:*mut gpio_chip, offset:u32)->i32 { let p=gpiochip_get_data(chip); let mut e=pm_runtime_get_sync((*p).dev); if e<0 { pm_runtime_put((*p).dev); return e; } e=pinctrl_gpio_request(chip,offset); if e!=0 { pm_runtime_put((*p).dev); } e }
unsafe fn gpio_rcar_free(chip:*mut gpio_chip, offset:u32) { let p=gpiochip_get_data(chip); pinctrl_gpio_free(chip,offset); gpio_rcar_config_general_input_output_mode(chip,offset,false); pm_runtime_put((*p).dev); }
unsafe fn gpio_rcar_get_direction(chip:*mut gpio_chip, offset:u32)->i32 { let p=gpiochip_get_data(chip); if gpio_rcar_read(p,INOUTSEL)&BIT(offset)!=0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN } }
unsafe fn gpio_rcar_direction_input(chip:*mut gpio_chip, offset:u32)->i32 { gpio_rcar_config_general_input_output_mode(chip,offset,false); 0 }
unsafe fn gpio_rcar_get(chip:*mut gpio_chip, offset:u32)->i32 { let p=gpiochip_get_data(chip); let bit=BIT(offset); let v=if !(*p).info.has_always_in && gpio_rcar_read(p,INOUTSEL)&bit!=0 { gpio_rcar_read(p,OUTDT)&bit } else { gpio_rcar_read(p,INDT)&bit }; if v!=0 {1}else{0} }
unsafe fn gpio_rcar_set(chip:*mut gpio_chip, offset:u32, value:i32)->i32 { let p=gpiochip_get_data(chip); let mut f=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut f); gpio_rcar_modify_bit(p,OUTDT,offset,value!=0); raw_spin_unlock_irqrestore(&mut (*p).lock,f); 0 }
unsafe fn gpio_rcar_direction_output(chip:*mut gpio_chip, offset:u32,value:i32)->i32 { gpio_rcar_set(chip,offset,value); gpio_rcar_config_general_input_output_mode(chip,offset,true); 0 }

static gpio_rcar_info_gen1: gpio_rcar_info = gpio_rcar_info { has_outdtsel:false,has_both_edge_trigger:false,has_always_in:false,has_inen:false };
static gpio_rcar_info_gen2: gpio_rcar_info = gpio_rcar_info { has_outdtsel:true,has_both_edge_trigger:true,has_always_in:false,has_inen:false };
static gpio_rcar_info_gen3: gpio_rcar_info = gpio_rcar_info { has_outdtsel:true,has_both_edge_trigger:true,has_always_in:true,has_inen:false };
static gpio_rcar_info_gen4: gpio_rcar_info = gpio_rcar_info { has_outdtsel:true,has_both_edge_trigger:true,has_always_in:true,has_inen:true };

unsafe fn gpio_rcar_get_multiple(chip:*mut gpio_chip, mask:*mut usize, bits:*mut usize)->i32 { let p=gpiochip_get_data(chip); let bankmask=(*mask) as u32 & GENMASK((*chip).ngpio-1,0); if (*p).info.has_always_in { *bits=gpio_rcar_read(p,INDT) as usize & bankmask as usize; return 0; } let mut f=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut f); let outputs=gpio_rcar_read(p,INOUTSEL); let mut m=outputs&bankmask; let mut val=0; if m!=0 {val|=gpio_rcar_read(p,OUTDT)&m;} m=!outputs&bankmask; if m!=0 {val|=gpio_rcar_read(p,INDT)&m;} raw_spin_unlock_irqrestore(&mut (*p).lock,f); *bits=val as usize; 0 }
unsafe fn gpio_rcar_set_multiple(chip:*mut gpio_chip,mask:*mut usize,bits:*mut usize)->i32 { let p=gpiochip_get_data(chip); let bankmask=*mask as u32&GENMASK((*chip).ngpio-1,0); let mut f=0; raw_spin_lock_irqsave(&mut (*p).lock,&mut f); let mut val=gpio_rcar_read(p,OUTDT); val&=!bankmask; val|=bankmask&*bits as u32; gpio_rcar_write(p,OUTDT,val); raw_spin_unlock_irqrestore(&mut (*p).lock,f); 0 }
unsafe fn gpio_rcar_enable_inputs(p:*mut gpio_rcar_priv) { let mask=GENMASK((*p).gpio_chip.ngpio-1,0); if mask!=0 { gpio_rcar_write(p,INEN,gpio_rcar_read(p,INEN)|mask); } }
unsafe fn gpio_rcar_parse_dt(_p:*mut gpio_rcar_priv,npins:*mut u32)->i32 { *npins=RCAR_MAX_GPIO_PER_BANK; 0 }
unsafe fn gpio_rcar_probe(pdev:*mut platform_device)->i32 { let dev=&mut (*pdev).dev; let p=devm_kzalloc(dev,core::mem::size_of::<gpio_rcar_priv>(),GFP_KERNEL) as *mut gpio_rcar_priv; if p.is_null(){return -ENOMEM;} (*p).dev=dev; raw_spin_lock_init(&mut (*p).lock); let mut n=0; let ret=gpio_rcar_parse_dt(p,&mut n); if ret<0{return ret;} platform_set_drvdata(pdev,p as *mut _); pm_runtime_enable(dev); (*p).irq_parent=platform_get_irq(pdev,0) as u32; if (*p).irq_parent as i32<0 {pm_runtime_disable(dev); return (*p).irq_parent as i32;} (*p).base=devm_platform_ioremap_resource(pdev,0); if (*p).base.is_null(){pm_runtime_disable(dev);return -ENOMEM;} (*p).gpio_chip.ngpio=n; let ret=gpiochip_add_data(&mut (*p).gpio_chip,p); if ret!=0 {pm_runtime_disable(dev);return ret;} 0 }
unsafe fn gpio_rcar_remove(pdev:*mut platform_device) { let p=platform_get_drvdata(pdev) as *mut gpio_rcar_priv; gpiochip_remove(&mut (*p).gpio_chip); pm_runtime_disable(&mut (*pdev).dev); }
unsafe fn gpio_rcar_suspend(dev:*mut device)->i32 { let p=dev_get_drvdata(dev) as *mut gpio_rcar_priv; (*p).bank_info.iointsel=gpio_rcar_read(p,IOINTSEL); (*p).bank_info.inoutsel=gpio_rcar_read(p,INOUTSEL); (*p).bank_info.outdt=gpio_rcar_read(p,OUTDT); (*p).bank_info.intmsk=gpio_rcar_read(p,INTMSK); (*p).bank_info.posneg=gpio_rcar_read(p,POSNEG); (*p).bank_info.edglevel=gpio_rcar_read(p,EDGLEVEL); if (*p).info.has_both_edge_trigger {(*p).bank_info.bothedge=gpio_rcar_read(p,BOTHEDGE);} if atomic_read(&(*p).wakeup_path)!=0 {device_set_wakeup_path(dev);} 0 }
unsafe fn gpio_rcar_resume(dev:*mut device)->i32 { let p=dev_get_drvdata(dev) as *mut gpio_rcar_priv; let mut o=0; while o<(*p).gpio_chip.ngpio { if gpiochip_line_is_valid(&mut (*p).gpio_chip,o) { let m=BIT(o); if (*p).bank_info.iointsel&m==0 { if (*p).bank_info.inoutsel&m!=0 {gpio_rcar_direction_output(&mut (*p).gpio_chip,o,if (*p).bank_info.outdt&m!=0{1}else{0});}else{gpio_rcar_direction_input(&mut (*p).gpio_chip,o);} } else {gpio_rcar_config_interrupt_input_mode(p,o,(*p).bank_info.posneg&m==0,(*p).bank_info.edglevel&m==0,(*p).bank_info.bothedge&m!=0); if (*p).bank_info.intmsk&m!=0 {gpio_rcar_write(p,MSKCLR,m);}}} o+=1;} if (*p).info.has_inen {gpio_rcar_enable_inputs(p);} 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
