// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support code for the SCOOP interface found on various Sharp PDAs
 *
 * Copyright (c) 2004 Richard Purdie
 *
 *	Based on code written by Sharp/Lineo for 2.4 kernels
 */

// PCMCIA to Scoop linkage
//
// There is no easy way to link multiple scoop devices into one
// single entity for the pxa2xx_pcmcia device so this structure
// is used which is setup by the platform code.
//
// This file is never modular so this symbol is always
// accessile to the board support files.
pub static mut platform_scoop_config: *mut scoop_pcmcia_config = core::ptr::null_mut();

#[repr(C)]
pub struct scoop_dev {
    base: *mut core::ffi::c_void,
    gpio: gpio_chip,
    scoop_lock: spinlock_t,
    suspend_clr: u16,
    suspend_set: u16,
    scoop_gpwr: u32,
}

pub unsafe fn reset_scoop(dev: *mut device) {
    let sdev = dev_get_drvdata(dev) as *mut scoop_dev;

    iowrite16(0x0100, (*sdev).base.wrapping_add(SCOOP_MCR as usize)); // 00
    iowrite16(0x0000, (*sdev).base.wrapping_add(SCOOP_CDR as usize)); // 04
    iowrite16(0x0000, (*sdev).base.wrapping_add(SCOOP_CCR as usize)); // 10
    iowrite16(0x0000, (*sdev).base.wrapping_add(SCOOP_IMR as usize)); // 18
    iowrite16(0x00FF, (*sdev).base.wrapping_add(SCOOP_IRM as usize)); // 14
    iowrite16(0x0000, (*sdev).base.wrapping_add(SCOOP_ISR as usize)); // 1C
    iowrite16(0x0000, (*sdev).base.wrapping_add(SCOOP_IRM as usize));
}

unsafe fn __scoop_gpio_set(sdev: *mut scoop_dev, offset: u32, value: i32) {
    let mut gpwr = ioread16((*sdev).base.wrapping_add(SCOOP_GPWR as usize));
    if value != 0 {
        gpwr |= 1u16.wrapping_shl(offset.wrapping_add(1));
    } else {
        gpwr &= !(1u16.wrapping_shl(offset.wrapping_add(1)));
    }
    iowrite16(gpwr, (*sdev).base.wrapping_add(SCOOP_GPWR as usize));
}

unsafe fn scoop_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let sdev = gpiochip_get_data(chip) as *mut scoop_dev;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sdev).scoop_lock, &mut flags);
    __scoop_gpio_set(sdev, offset, value);
    spin_unlock_irqrestore(&mut (*sdev).scoop_lock, flags);
    0
}

unsafe fn scoop_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let sdev = gpiochip_get_data(chip) as *mut scoop_dev;
    // XXX: I'm unsure, but it seems so
    if ioread16((*sdev).base.wrapping_add(SCOOP_GPRR as usize))
        & 1u16.wrapping_shl(offset.wrapping_add(1)) != 0 { 1 } else { 0 }
}

unsafe fn scoop_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let sdev = gpiochip_get_data(chip) as *mut scoop_dev;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sdev).scoop_lock, &mut flags);
    let mut gpcr = ioread16((*sdev).base.wrapping_add(SCOOP_GPCR as usize));
    gpcr &= !(1u16.wrapping_shl(offset.wrapping_add(1)));
    iowrite16(gpcr, (*sdev).base.wrapping_add(SCOOP_GPCR as usize));
    spin_unlock_irqrestore(&mut (*sdev).scoop_lock, flags);
    0
}

unsafe fn scoop_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let sdev = gpiochip_get_data(chip) as *mut scoop_dev;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*sdev).scoop_lock, &mut flags);
    __scoop_gpio_set(sdev, offset, value);
    let mut gpcr = ioread16((*sdev).base.wrapping_add(SCOOP_GPCR as usize));
    gpcr |= 1u16.wrapping_shl(offset.wrapping_add(1));
    iowrite16(gpcr, (*sdev).base.wrapping_add(SCOOP_GPCR as usize));
    spin_unlock_irqrestore(&mut (*sdev).scoop_lock, flags);
    0
}

pub unsafe fn read_scoop_reg(dev: *mut device, reg: u16) -> u16 {
    let sdev = dev_get_drvdata(dev) as *mut scoop_dev;
    ioread16((*sdev).base.wrapping_add(reg as usize))
}

pub unsafe fn write_scoop_reg(dev: *mut device, reg: u16, data: u16) {
    let sdev = dev_get_drvdata(dev) as *mut scoop_dev;
    iowrite16(data, (*sdev).base.wrapping_add(reg as usize));
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn check_scoop_reg(sdev: *mut scoop_dev) {
    let mcr = ioread16((*sdev).base.wrapping_add(SCOOP_MCR as usize));
    if mcr & 0x100 == 0 {
        iowrite16(0x0101, (*sdev).base.wrapping_add(SCOOP_MCR as usize));
    }
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn scoop_suspend(dev: *mut platform_device, _state: pm_message_t) -> i32 {
    let sdev = platform_get_drvdata(dev) as *mut scoop_dev;
    check_scoop_reg(sdev);
    (*sdev).scoop_gpwr = ioread16((*sdev).base.wrapping_add(SCOOP_GPWR as usize)) as u32;
    iowrite16((((*sdev).scoop_gpwr as u16 & !(*sdev).suspend_clr) | (*sdev).suspend_set),
              (*sdev).base.wrapping_add(SCOOP_GPWR as usize));
    0
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn scoop_resume(dev: *mut platform_device) -> i32 {
    let sdev = platform_get_drvdata(dev) as *mut scoop_dev;
    check_scoop_reg(sdev);
    iowrite16((*sdev).scoop_gpwr as u16, (*sdev).base.wrapping_add(SCOOP_GPWR as usize));
    0
}

#[cfg(not(feature = "CONFIG_PM"))]
const scoop_suspend: Option<unsafe fn(*mut platform_device, pm_message_t) -> i32> = None;
#[cfg(not(feature = "CONFIG_PM"))]
const scoop_resume: Option<unsafe fn(*mut platform_device) -> i32> = None;

unsafe fn scoop_probe(pdev: *mut platform_device) -> i32 {
    let mut devptr: *mut scoop_dev;
    let inf: *mut scoop_config;
    let mem = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    let mut ret: i32;
    if mem.is_null() { return -EINVAL; }
    devptr = kzalloc_obj::<scoop_dev>();
    if devptr.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*devptr).scoop_lock);
    inf = (*pdev).dev.platform_data as *mut scoop_config;
    (*devptr).base = ioremap((*mem).start, resource_size(mem));
    if (*devptr).base.is_null() { ret = -ENOMEM; iounmap((*devptr).base); kfree(devptr as *mut core::ffi::c_void); return ret; }
    platform_set_drvdata(pdev, devptr);
    printk("Sharp Scoop Device found at 0x%08x -> 0x%8p\n", (*mem).start as u32, (*devptr).base);
    iowrite16(0x0140, (*devptr).base.wrapping_add(SCOOP_MCR as usize));
    reset_scoop(&mut (*pdev).dev);
    iowrite16(0x0000, (*devptr).base.wrapping_add(SCOOP_CPR as usize));
    iowrite16((*inf).io_dir as u16, (*devptr).base.wrapping_add(SCOOP_GPCR as usize));
    iowrite16((*inf).io_out as u16, (*devptr).base.wrapping_add(SCOOP_GPWR as usize));
    (*devptr).suspend_clr = (*inf).suspend_clr;
    (*devptr).suspend_set = (*inf).suspend_set;
    (*devptr).gpio.base = -1;
    if (*inf).gpio_base != 0 {
        (*devptr).gpio.label = dev_name(&mut (*pdev).dev);
        (*devptr).gpio.base = (*inf).gpio_base;
        (*devptr).gpio.ngpio = 12;
        (*devptr).gpio.set = Some(scoop_gpio_set);
        (*devptr).gpio.get = Some(scoop_gpio_get);
        (*devptr).gpio.direction_input = Some(scoop_gpio_direction_input);
        (*devptr).gpio.direction_output = Some(scoop_gpio_direction_output);
        ret = gpiochip_add_data(&mut (*devptr).gpio, devptr);
        if ret != 0 { platform_set_drvdata(pdev, core::ptr::null_mut()); iounmap((*devptr).base); kfree(devptr as *mut core::ffi::c_void); return ret; }
    }
    0
}

unsafe fn scoop_remove(pdev: *mut platform_device) {
    let sdev = platform_get_drvdata(pdev) as *mut scoop_dev;
    if (*sdev).gpio.base != -1 { gpiochip_remove(&mut (*sdev).gpio); }
    platform_set_drvdata(pdev, core::ptr::null_mut());
    iounmap((*sdev).base);
    kfree(sdev as *mut core::ffi::c_void);
}

static mut scoop_driver: platform_driver = platform_driver {
    probe: Some(scoop_probe), remove: Some(scoop_remove), suspend: scoop_suspend, resume: scoop_resume,
    driver: driver { name: "sharp-scoop" },
};

unsafe fn scoop_init() -> i32 { platform_driver_register(&mut scoop_driver) }

// Equivalent of subsys_initcall(scoop_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
