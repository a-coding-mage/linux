// SPDX-License-Identifier: GPL-2.0+
/* Generic driver for memory-mapped GPIO controllers. */

// Kernel dependencies supplied by the surrounding tree are intentionally not
// redefined here.

unsafe fn gpio_mmio_write8(reg: *mut core::ffi::c_void, data: usize) { writeb(data as u8, reg); }
unsafe fn gpio_mmio_read8(reg: *mut core::ffi::c_void) -> usize { readb(reg) as usize }
unsafe fn gpio_mmio_write16(reg: *mut core::ffi::c_void, data: usize) { writew(data as u16, reg); }
unsafe fn gpio_mmio_read16(reg: *mut core::ffi::c_void) -> usize { readw(reg) as usize }
unsafe fn gpio_mmio_write32(reg: *mut core::ffi::c_void, data: usize) { writel(data as u32, reg); }
unsafe fn gpio_mmio_read32(reg: *mut core::ffi::c_void) -> usize { readl(reg) as usize }

#[cfg(target_pointer_width = "64")]
unsafe fn gpio_mmio_write64(reg: *mut core::ffi::c_void, data: usize) { writeq(data as u64, reg); }
#[cfg(target_pointer_width = "64")]
unsafe fn gpio_mmio_read64(reg: *mut core::ffi::c_void) -> usize { readq(reg) as usize }

unsafe fn gpio_mmio_write16be(reg: *mut core::ffi::c_void, data: usize) { iowrite16be(data as u16, reg); }
unsafe fn gpio_mmio_read16be(reg: *mut core::ffi::c_void) -> usize { ioread16be(reg) as usize }
unsafe fn gpio_mmio_write32be(reg: *mut core::ffi::c_void, data: usize) { iowrite32be(data as u32, reg); }
unsafe fn gpio_mmio_read32be(reg: *mut core::ffi::c_void) -> usize { ioread32be(reg) as usize }

unsafe fn gpio_mmio_line2mask(gc: *mut gpio_chip, line: u32) -> usize {
    let chip = to_gpio_generic_chip(gc);
    if (*chip).be_bits { 1usize << ((*chip).bits - 1 - line) } else { 1usize << line }
}

unsafe fn gpio_mmio_get_set(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let chip = to_gpio_generic_chip(gc);
    let pinmask = gpio_mmio_line2mask(gc, gpio);
    let dir = ((*chip).sdir & pinmask) != 0;
    if dir { ((*chip).read_reg.unwrap()((*chip).reg_set) & pinmask != 0) as i32 }
    else { ((*chip).read_reg.unwrap()((*chip).reg_dat) & pinmask != 0) as i32 }
}

unsafe fn gpio_mmio_get_set_multiple(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let chip = to_gpio_generic_chip(gc);
    *bits &= !*mask;
    let set_mask = *mask & (*chip).sdir;
    let get_mask = *mask & !(*chip).sdir;
    if set_mask != 0 { *bits |= (*chip).read_reg.unwrap()((*chip).reg_set) & set_mask; }
    if get_mask != 0 { *bits |= (*chip).read_reg.unwrap()((*chip).reg_dat) & get_mask; }
    0
}

unsafe fn gpio_mmio_get(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let chip = to_gpio_generic_chip(gc);
    ((*chip).read_reg.unwrap()((*chip).reg_dat) & gpio_mmio_line2mask(gc, gpio) != 0) as i32
}

unsafe fn gpio_mmio_get_multiple(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let chip = to_gpio_generic_chip(gc);
    *bits &= !*mask;
    *bits |= (*chip).read_reg.unwrap()((*chip).reg_dat) & *mask;
    0
}

unsafe fn gpio_mmio_get_multiple_be(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let chip = to_gpio_generic_chip(gc);
    *bits &= !*mask;
    let mut readmask = 0usize;
    for bit in 0..(*gc).ngpio { if (*mask & (1usize << bit)) != 0 { readmask |= gpio_mmio_line2mask(gc, bit); } }
    let val = (*chip).read_reg.unwrap()((*chip).reg_dat) & readmask;
    for bit in 0..(*gc).ngpio { if (val & (1usize << bit)) != 0 { *bits |= gpio_mmio_line2mask(gc, bit); } }
    0
}

unsafe fn gpio_mmio_set_none(_gc: *mut gpio_chip, _gpio: u32, _val: i32) -> i32 { 0 }

unsafe fn gpio_mmio_set(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let chip = to_gpio_generic_chip(gc);
    let mask = gpio_mmio_line2mask(gc, gpio);
    // guard(raw_spinlock_irqsave)(&chip->lock)
    if val != 0 { (*chip).sdata |= mask; } else { (*chip).sdata &= !mask; }
    (*chip).write_reg.unwrap()((*chip).reg_dat, (*chip).sdata); 0
}

unsafe fn gpio_mmio_set_with_clear(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let chip = to_gpio_generic_chip(gc); let mask = gpio_mmio_line2mask(gc, gpio);
    if val != 0 { (*chip).write_reg.unwrap()((*chip).reg_set, mask); } else { (*chip).write_reg.unwrap()((*chip).reg_clr, mask); } 0
}

unsafe fn gpio_mmio_set_set(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let chip = to_gpio_generic_chip(gc); let mask = gpio_mmio_line2mask(gc, gpio);
    if val != 0 { (*chip).sdata |= mask; } else { (*chip).sdata &= !mask; }
    (*chip).write_reg.unwrap()((*chip).reg_set, (*chip).sdata); 0
}

unsafe fn gpio_mmio_multiple_get_masks(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize, set_mask: *mut usize, clear_mask: *mut usize) {
    let chip = to_gpio_generic_chip(gc); *set_mask = 0; *clear_mask = 0;
    for i in 0..(*chip).bits { if (*mask & (1usize << i)) != 0 { if (*bits & (1usize << i)) != 0 { *set_mask |= gpio_mmio_line2mask(gc, i); } else { *clear_mask |= gpio_mmio_line2mask(gc, i); } } }
}

unsafe fn gpio_mmio_set_multiple_single_reg(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize, reg: *mut core::ffi::c_void) {
    let chip = to_gpio_generic_chip(gc); let (mut set_mask, mut clear_mask) = (0, 0);
    gpio_mmio_multiple_get_masks(gc, mask, bits, &mut set_mask, &mut clear_mask);
    (*chip).sdata |= set_mask; (*chip).sdata &= !clear_mask; (*chip).write_reg.unwrap()(reg, (*chip).sdata);
}
unsafe fn gpio_mmio_set_multiple(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 { let c=to_gpio_generic_chip(gc); gpio_mmio_set_multiple_single_reg(gc,mask,bits,(*c).reg_dat); 0 }
unsafe fn gpio_mmio_set_multiple_set(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 { let c=to_gpio_generic_chip(gc); gpio_mmio_set_multiple_single_reg(gc,mask,bits,(*c).reg_set); 0 }
unsafe fn gpio_mmio_set_multiple_with_clear(gc: *mut gpio_chip, mask: *mut usize, bits: *mut usize) -> i32 {
    let c=to_gpio_generic_chip(gc); let (mut set_mask,mut clear_mask)=(0,0); gpio_mmio_multiple_get_masks(gc,mask,bits,&mut set_mask,&mut clear_mask);
    if set_mask != 0 { (*c).write_reg.unwrap()((*c).reg_set,set_mask); } if clear_mask != 0 { (*c).write_reg.unwrap()((*c).reg_clr,clear_mask); } 0
}

unsafe fn gpio_mmio_dir_return(gc: *mut gpio_chip, gpio: u32, dir_out: bool) -> i32 {
    let c=to_gpio_generic_chip(gc); if !(*c).pinctrl { return 0; }
    if dir_out { pinctrl_gpio_direction_output(gc,gpio) } else { pinctrl_gpio_direction_input(gc,gpio) }
}
unsafe fn gpio_mmio_dir_in_err(_gc:*mut gpio_chip,_gpio:u32)->i32 { -EINVAL }
unsafe fn gpio_mmio_simple_dir_in(gc:*mut gpio_chip,gpio:u32)->i32 { gpio_mmio_dir_return(gc,gpio,false) }
unsafe fn gpio_mmio_dir_out_err(_gc:*mut gpio_chip,_gpio:u32,_val:i32)->i32 { -EINVAL }
unsafe fn gpio_mmio_simple_dir_out(gc:*mut gpio_chip,gpio:u32,val:i32)->i32 { ((*gc).set.unwrap())(gc,gpio,val); gpio_mmio_dir_return(gc,gpio,true) }

unsafe fn gpio_mmio_dir_in(gc:*mut gpio_chip,gpio:u32)->i32 { let c=to_gpio_generic_chip(gc); (*c).sdir &= !gpio_mmio_line2mask(gc,gpio); if !(*c).reg_dir_in.is_null(){(*c).write_reg.unwrap()((*c).reg_dir_in,!(*c).sdir);} if !(*c).reg_dir_out.is_null(){(*c).write_reg.unwrap()((*c).reg_dir_out,(*c).sdir);} gpio_mmio_dir_return(gc,gpio,false) }
unsafe fn gpio_mmio_get_dir(gc:*mut gpio_chip,gpio:u32)->i32 { let c=to_gpio_generic_chip(gc); let m=gpio_mmio_line2mask(gc,gpio); if (*c).dir_unreadable { return if (*c).sdir&m!=0 {GPIO_LINE_DIRECTION_OUT} else {GPIO_LINE_DIRECTION_IN}; } if !(*c).reg_dir_out.is_null(){return if (*c).read_reg.unwrap()((*c).reg_dir_out)&m!=0{GPIO_LINE_DIRECTION_OUT}else{GPIO_LINE_DIRECTION_IN};} if !(*c).reg_dir_in.is_null()&&(*c).read_reg.unwrap()((*c).reg_dir_in)&m==0{return GPIO_LINE_DIRECTION_OUT;} GPIO_LINE_DIRECTION_IN }
unsafe fn gpio_mmio_dir_out(gc:*mut gpio_chip,gpio:u32,_val:i32){let c=to_gpio_generic_chip(gc);(*c).sdir|=gpio_mmio_line2mask(gc,gpio);if !(*c).reg_dir_in.is_null(){(*c).write_reg.unwrap()((*c).reg_dir_in,!(*c).sdir);}if !(*c).reg_dir_out.is_null(){(*c).write_reg.unwrap()((*c).reg_dir_out,(*c).sdir);}}
unsafe fn gpio_mmio_dir_out_dir_first(gc:*mut gpio_chip,gpio:u32,val:i32)->i32{gpio_mmio_dir_out(gc,gpio,val);((*gc).set.unwrap())(gc,gpio,val);gpio_mmio_dir_return(gc,gpio,true)}
unsafe fn gpio_mmio_dir_out_val_first(gc:*mut gpio_chip,gpio:u32,val:i32)->i32{((*gc).set.unwrap())(gc,gpio,val);gpio_mmio_dir_out(gc,gpio,val);gpio_mmio_dir_return(gc,gpio,true)}

// The remaining platform-driver registration is represented below with the
// same external kernel types and symbols; build-time CONFIG_GPIO_GENERIC_PLATFORM
// controls whether it is included.
#[cfg(feature = "CONFIG_GPIO_GENERIC_PLATFORM")]
unsafe fn gpio_mmio_setup_accessors(dev:*mut device, chip:*mut gpio_generic_chip, byte_be:bool)->i32{
    match (*chip).bits { 8=>{(*chip).read_reg=Some(gpio_mmio_read8);(*chip).write_reg=Some(gpio_mmio_write8)},16=>{if byte_be{(*chip).read_reg=Some(gpio_mmio_read16be);(*chip).write_reg=Some(gpio_mmio_write16be)}else{(*chip).read_reg=Some(gpio_mmio_read16);(*chip).write_reg=Some(gpio_mmio_write16)}},32=>{if byte_be{(*chip).read_reg=Some(gpio_mmio_read32be);(*chip).write_reg=Some(gpio_mmio_write32be)}else{(*chip).read_reg=Some(gpio_mmio_read32);(*chip).write_reg=Some(gpio_mmio_write32)}},64=>{if byte_be{return -EINVAL}(*chip).read_reg=Some(gpio_mmio_read64);(*chip).write_reg=Some(gpio_mmio_write64)},_=>{dev_err(dev,"unsupported data width %u bits\n",(*chip).bits);return -EINVAL}} 0 }

unsafe fn gpio_mmio_setup_io(chip:*mut gpio_generic_chip,cfg:*const gpio_generic_chip_config)->i32{let gc=&mut (*chip).gc;(*chip).reg_dat=(*cfg).dat;if (*chip).reg_dat.is_null(){return -EINVAL;}if !(*cfg).set.is_null()&&!(*cfg).clr.is_null(){(*chip).reg_set=(*cfg).set;(*chip).reg_clr=(*cfg).clr;gc.set=Some(gpio_mmio_set_with_clear);gc.set_multiple=Some(gpio_mmio_set_multiple_with_clear);}else if !(*cfg).set.is_null(){(*chip).reg_set=(*cfg).set;gc.set=Some(gpio_mmio_set_set);gc.set_multiple=Some(gpio_mmio_set_multiple_set);}else if (*cfg).flags&GPIO_GENERIC_NO_OUTPUT!=0{gc.set=Some(gpio_mmio_set_none);gc.set_multiple=None;}else{gc.set=Some(gpio_mmio_set);gc.set_multiple=Some(gpio_mmio_set_multiple);}if (*cfg).flags&GPIO_GENERIC_UNREADABLE_REG_SET==0&&(*cfg).flags&GPIO_GENERIC_READ_OUTPUT_REG_SET!=0{gc.get=Some(gpio_mmio_get_set);if !(*chip).be_bits{gc.get_multiple=Some(gpio_mmio_get_set_multiple);}}else{gc.get=Some(gpio_mmio_get);gc.get_multiple=Some(if (*chip).be_bits{gpio_mmio_get_multiple_be}else{gpio_mmio_get_multiple});}0}
unsafe fn gpio_mmio_setup_direction(chip:*mut gpio_generic_chip,cfg:*const gpio_generic_chip_config)->i32{let gc=&mut (*chip).gc;if !(*cfg).dirout.is_null()||!(*cfg).dirin.is_null(){(*chip).reg_dir_out=(*cfg).dirout;(*chip).reg_dir_in=(*cfg).dirin;gc.direction_output=Some(if (*cfg).flags&GPIO_GENERIC_NO_SET_ON_INPUT!=0{gpio_mmio_dir_out_dir_first}else{gpio_mmio_dir_out_val_first});gc.direction_input=Some(gpio_mmio_dir_in);gc.get_direction=Some(gpio_mmio_get_dir);}else{gc.direction_output=Some(if (*cfg).flags&GPIO_GENERIC_NO_OUTPUT!=0{gpio_mmio_dir_out_err}else{gpio_mmio_simple_dir_out});gc.direction_input=Some(if (*cfg).flags&GPIO_GENERIC_NO_INPUT!=0{gpio_mmio_dir_in_err}else{gpio_mmio_simple_dir_in});}0}
unsafe fn gpio_mmio_request(gc:*mut gpio_chip,gpio_pin:u32)->i32{let c=to_gpio_generic_chip(gc);if gpio_pin>=(*gc).ngpio{return -EINVAL;}if (*c).pinctrl{return gpiochip_generic_request(gc,gpio_pin)}0}
pub unsafe fn gpio_generic_chip_init(chip:*mut gpio_generic_chip,cfg:*const gpio_generic_chip_config)->i32{let gc=&mut (*chip).gc;let flags=(*cfg).flags;let dev=(*cfg).dev;if !is_power_of_2((*cfg).sz){return -EINVAL;}(*chip).bits=(*cfg).sz*8;if (*chip).bits>BITS_PER_LONG{return -EINVAL;}raw_spin_lock_init(&mut (*chip).lock);gc.parent=dev;gc.label=dev_name(dev);gc.base=-1;gc.request=Some(gpio_mmio_request);(*chip).be_bits=flags&GPIO_GENERIC_BIG_ENDIAN!=0;let ret=gpiochip_get_ngpios(gc,dev);if ret!=0{gc.ngpio=(*chip).bits as u32;}let ret=gpio_mmio_setup_io(chip,cfg);if ret!=0{return ret;}let ret=gpio_mmio_setup_accessors(dev,chip,flags&GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER!=0);if ret!=0{return ret;}let ret=gpio_mmio_setup_direction(chip,cfg);if ret!=0{return ret;}if flags&GPIO_GENERIC_PINCTRL_BACKEND!=0{(*chip).pinctrl=true;gc.free=Some(gpiochip_generic_free);}(*chip).sdata=(*chip).read_reg.unwrap()((*chip).reg_dat);if gc.set==Some(gpio_mmio_set_set)&&flags&GPIO_GENERIC_UNREADABLE_REG_SET==0{(*chip).sdata=(*chip).read_reg.unwrap()((*chip).reg_set);}if flags&GPIO_GENERIC_UNREADABLE_REG_DIR!=0{(*chip).dir_unreadable=true;}if (!(*chip).reg_dir_out.is_null()||!(*chip).reg_dir_in.is_null())&&flags&GPIO_GENERIC_UNREADABLE_REG_DIR==0{if !(*chip).reg_dir_out.is_null(){(*chip).sdir=(*chip).read_reg.unwrap()((*chip).reg_dir_out);}else if !(*chip).reg_dir_in.is_null(){(*chip).sdir=!(*chip).read_reg.unwrap()((*chip).reg_dir_in);}if !(*chip).reg_dir_out.is_null()&&!(*chip).reg_dir_in.is_null(){(*chip).write_reg.unwrap()((*chip).reg_dir_in,!(*chip).sdir);}}ret}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
