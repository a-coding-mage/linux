// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation. Kernel-provided types, macros, and
// functions referenced below are supplied by the surrounding crate.

unsafe fn macio_gpio_irq(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    pmf_do_irq(data);
    IRQ_HANDLED
}

unsafe fn macio_do_gpio_irq_enable(func: *mut pmf_function) -> i32 {
    let irq = irq_of_parse_and_map((*func).node, 0);
    if irq == 0 { return -EINVAL; }
    request_irq(irq, macio_gpio_irq, 0, (*(*func).node).name, func)
}

unsafe fn macio_do_gpio_irq_disable(func: *mut pmf_function) -> i32 {
    let irq = irq_of_parse_and_map((*func).node, 0);
    if irq == 0 { return -EINVAL; }
    free_irq(irq, func);
    0
}

unsafe fn macio_do_gpio_write(func: *mut pmf_function, args: *mut pmf_args, mut value: u8, mask: u8) -> i32 {
    let addr = (*func).driver_data as *mut u8;
    let mut flags: usize = 0;
    if !args.is_null() && (*args).count != 0 && !(*args).u[0].p.is_null() && (*args).u[0].v == 0 { value = !value; }
    raw_spin_lock_irqsave(&mut feature_lock, &mut flags);
    let mut tmp = readb(addr);
    tmp = (tmp & !mask) | (value & mask);
    writeb(tmp, addr);
    raw_spin_unlock_irqrestore(&mut feature_lock, flags);
    0
}

unsafe fn macio_do_gpio_read(func: *mut pmf_function, args: *mut pmf_args, mask: u8, rshift: i32, xor: u8) -> i32 {
    let addr = (*func).driver_data as *mut u8;
    if args.is_null() || (*args).count == 0 || (*args).u[0].p.is_null() { return -EINVAL; }
    let value = readb(addr);
    *((*args).u[0].p as *mut u8) = ((value & mask) >> rshift) ^ xor;
    0
}

unsafe fn macio_do_delay(_func: *mut pmf_function, _args: *mut pmf_args, duration: u32) -> i32 {
    msleep((duration + 999) / 1000);
    0
}

static mut macio_gpio_handlers: pmf_handlers = pmf_handlers {
    irq_enable: Some(macio_do_gpio_irq_enable), irq_disable: Some(macio_do_gpio_irq_disable),
    write_gpio: Some(macio_do_gpio_write), read_gpio: Some(macio_do_gpio_read), delay: Some(macio_do_delay),
};

unsafe fn macio_gpio_init_one(macio: *mut macio_chip) {
    let mut gparent: *mut device_node = core::ptr::null_mut();
    let mut gp: *mut device_node;
    for_each_child_of_node!((*macio).of_node, gparent) {
        if of_node_name_eq(gparent, c"gpio") { break; }
    }
    if gparent.is_null() { return; }
    for_each_child_of_node!(gparent, gp) {
        let reg = of_get_property(gp, c"reg", core::ptr::null_mut());
        if reg.is_null() { continue; }
        let mut offset = *reg as usize;
        if offset < 0x50 { offset += 0x50; }
        offset += (*macio).base as usize;
        pmf_register_driver(gp, &mut macio_gpio_handlers, offset as *mut core::ffi::c_void);
    }
    for_each_child_of_node!(gparent, gp) { pmf_do_functions(gp, core::ptr::null_mut(), 0, PMF_FLAGS_ON_INIT, core::ptr::null_mut()); }
    of_node_put(gparent);
}

unsafe fn macio_do_write_reg32(func: *mut pmf_function, _args: *mut pmf_args, offset: u32, value: u32, mask: u32) -> i32 {
    let macio = (*func).driver_data as *mut macio_chip; let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut feature_lock, &mut flags);
    MACIO_OUT32!(macio, offset, (MACIO_IN32!(macio, offset) & !mask) | (value & mask));
    raw_spin_unlock_irqrestore(&mut feature_lock, flags); 0
}
unsafe fn macio_do_read_reg32(func: *mut pmf_function, args: *mut pmf_args, offset: u32) -> i32 { let macio=(*func).driver_data as *mut macio_chip; if args.is_null()||(*args).count==0||(*args).u[0].p.is_null(){return -EINVAL;} *(*args).u[0].p=MACIO_IN32!(macio,offset); 0 }
unsafe fn macio_do_write_reg8(func:*mut pmf_function,_args:*mut pmf_args,offset:u32,value:u8,mask:u8)->i32 { let macio=(*func).driver_data as *mut macio_chip; let mut flags=0usize; raw_spin_lock_irqsave(&mut feature_lock,&mut flags); MACIO_OUT8!(macio,offset,(MACIO_IN8!(macio,offset)&!mask)|(value&mask)); raw_spin_unlock_irqrestore(&mut feature_lock,flags); 0 }
unsafe fn macio_do_read_reg8(func:*mut pmf_function,args:*mut pmf_args,offset:u32)->i32 { let macio=(*func).driver_data as *mut macio_chip; if args.is_null()||(*args).count==0||(*args).u[0].p.is_null(){return -EINVAL;} *((*args).u[0].p as *mut u8)=MACIO_IN8!(macio,offset); 0 }
unsafe fn macio_do_read_reg32_msrx(func:*mut pmf_function,args:*mut pmf_args,offset:u32,mask:u32,shift:u32,xor:u32)->i32 { let macio=(*func).driver_data as *mut macio_chip; if args.is_null()||(*args).count==0||(*args).u[0].p.is_null(){return -EINVAL;} *(*args).u[0].p=((MACIO_IN32!(macio,offset)&mask)>>shift)^xor; 0 }
unsafe fn macio_do_read_reg8_msrx(func:*mut pmf_function,args:*mut pmf_args,offset:u32,mask:u32,shift:u32,xor:u32)->i32 { let macio=(*func).driver_data as *mut macio_chip; if args.is_null()||(*args).count==0||(*args).u[0].p.is_null(){return -EINVAL;} *((*args).u[0].p as *mut u8)=(((MACIO_IN8!(macio,offset) as u32&mask)>>shift)^xor) as u8; 0 }
unsafe fn macio_do_write_reg32_slm(func:*mut pmf_function,args:*mut pmf_args,offset:u32,shift:u32,mask:u32)->i32 { let macio=(*func).driver_data as *mut macio_chip; if args.is_null()||(*args).count==0{return -EINVAL;} let mut flags=0usize; raw_spin_lock_irqsave(&mut feature_lock,&mut flags); let tmp=MACIO_IN32!(macio,offset); let val=(*args).u[0].v<<shift; MACIO_OUT32!(macio,offset,(tmp&!mask)|(val&mask)); raw_spin_unlock_irqrestore(&mut feature_lock,flags); 0 }
unsafe fn macio_do_write_reg8_slm(func:*mut pmf_function,args:*mut pmf_args,offset:u32,shift:u32,mask:u32)->i32 { let macio=(*func).driver_data as *mut macio_chip; if args.is_null()||(*args).count==0{return -EINVAL;} let mut flags=0usize; raw_spin_lock_irqsave(&mut feature_lock,&mut flags); let tmp=MACIO_IN8!(macio,offset) as u32; let val=(*args).u[0].v<<shift; MACIO_OUT8!(macio,offset,((tmp&!mask)|(val&mask)) as u8); raw_spin_unlock_irqrestore(&mut feature_lock,flags); 0 }

static mut unin_hwclock: *mut device_node = core::ptr::null_mut();
unsafe fn unin_do_write_reg32(_func:*mut pmf_function,_args:*mut pmf_args,offset:u32,value:u32,mask:u32)->i32 { let mut flags=0usize; raw_spin_lock_irqsave(&mut feature_lock,&mut flags); UN_OUT!(offset,(UN_IN!(offset)&!mask)|(value&mask)); raw_spin_unlock_irqrestore(&mut feature_lock,flags); 0 }

unsafe fn macio_mmio_init_one(macio:*mut macio_chip) { pmf_register_driver((*macio).of_node,&mut macio_mmio_handlers,macio as *mut _); }
static mut macio_mmio_handlers: pmf_handlers = pmf_handlers { write_reg32:Some(macio_do_write_reg32),read_reg32:Some(macio_do_read_reg32),write_reg8:Some(macio_do_write_reg8),read_reg8:Some(macio_do_read_reg8),read_reg32_msrx:Some(macio_do_read_reg32_msrx),read_reg8_msrx:Some(macio_do_read_reg8_msrx),write_reg32_slm:Some(macio_do_write_reg32_slm),write_reg8_slm:Some(macio_do_write_reg8_slm),delay:Some(macio_do_delay) };
static mut unin_mmio_handlers: pmf_handlers = pmf_handlers { write_reg32:Some(unin_do_write_reg32), delay:Some(macio_do_delay) };

unsafe fn uninorth_install_pfunc() { pmf_register_driver(uninorth_node,&mut unin_mmio_handlers,core::ptr::null_mut()); pmf_do_functions(uninorth_node,core::ptr::null_mut(),0,PMF_FLAGS_ON_INIT,core::ptr::null_mut()); let mut np=core::ptr::null_mut(); for_each_child_of_node!(uninorth_node,np){if of_node_name_eq(np,c"hw-clock"){unin_hwclock=np;break;}} if !unin_hwclock.is_null(){pmf_register_driver(unin_hwclock,&mut unin_mmio_handlers,core::ptr::null_mut());pmf_do_functions(unin_hwclock,core::ptr::null_mut(),0,PMF_FLAGS_ON_INIT,core::ptr::null_mut());} }

pub unsafe fn pmac_pfunc_base_install() -> i32 { static mut pfbase_inited: i32=0; if pfbase_inited!=0{return 0;} pfbase_inited=1; if !machine_is!(powermac){return 0;} for i in 0..MAX_MACIO_CHIPS { if !macio_chips[i].of_node.is_null(){macio_mmio_init_one(&mut macio_chips[i]);macio_gpio_init_one(&mut macio_chips[i]);} } if !uninorth_node.is_null()&&!uninorth_base.is_null(){uninorth_install_pfunc();} 0 }

#[cfg(CONFIG_PM)]
pub unsafe fn pmac_pfunc_base_suspend() { for i in 0..MAX_MACIO_CHIPS {if !macio_chips[i].of_node.is_null(){pmf_do_functions(macio_chips[i].of_node,core::ptr::null_mut(),0,PMF_FLAGS_ON_SLEEP,core::ptr::null_mut());}} if !uninorth_node.is_null(){pmf_do_functions(uninorth_node,core::ptr::null_mut(),0,PMF_FLAGS_ON_SLEEP,core::ptr::null_mut());} if !unin_hwclock.is_null(){pmf_do_functions(unin_hwclock,core::ptr::null_mut(),0,PMF_FLAGS_ON_SLEEP,core::ptr::null_mut());} }
#[cfg(CONFIG_PM)]
pub unsafe fn pmac_pfunc_base_resume() { if !unin_hwclock.is_null(){pmf_do_functions(unin_hwclock,core::ptr::null_mut(),0,PMF_FLAGS_ON_WAKE,core::ptr::null_mut());} if !uninorth_node.is_null(){pmf_do_functions(uninorth_node,core::ptr::null_mut(),0,PMF_FLAGS_ON_WAKE,core::ptr::null_mut());} for i in 0..MAX_MACIO_CHIPS {if !macio_chips[i].of_node.is_null(){pmf_do_functions(macio_chips[i].of_node,core::ptr::null_mut(),0,PMF_FLAGS_ON_WAKE,core::ptr::null_mut());}} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
