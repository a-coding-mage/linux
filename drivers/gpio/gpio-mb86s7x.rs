// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/drivers/gpio/gpio-mb86s7x.c
 *
 *  Copyright (C) 2015 Fujitsu Semiconductor Limited
 *  Copyright (C) 2015 Linaro Ltd.
 */

// Kernel dependencies supplied by the surrounding repository.

const fn pdr(x: usize) -> usize { x / 8 * 4 }
const fn ddr(x: usize) -> usize { 0x10 + x / 8 * 4 }
const fn pfr(x: usize) -> usize { 0x20 + x / 8 * 4 }
const fn offset(x: usize) -> u32 { 1u32 << (x % 8) }

#[repr(C)]
struct Mb86s70GpioChip {
    gc: gpio_chip,
    base: *mut core::ffi::c_void,
    lock: spinlock_t,
}

unsafe extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut Mb86s70GpioChip;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn platform_get_irq(pdev: *mut platform_device, index: c_uint) -> c_int;
    fn to_platform_device(parent: *mut device) -> *mut platform_device;
    fn irq_get_irq_data(irq: c_int) -> *mut irq_data;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut Mb86s70GpioChip);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut core::ffi::c_void;
    fn devm_clk_get_optional_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn gpiochip_add_data(gc: *mut gpio_chip, data: *mut Mb86s70GpioChip) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char) -> c_int;
    fn acpi_gpiochip_request_interrupts(gc: *mut gpio_chip);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut Mb86s70GpioChip;
    fn acpi_gpiochip_free_interrupts(gc: *mut gpio_chip);
    fn gpiochip_remove(gc: *mut gpio_chip) -> c_int;
}

unsafe fn mb86s70_gpio_request(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let gchip = gpiochip_get_data(gc);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*gchip).lock, &mut flags);
    let addr = (*gchip).base.add(pfr(gpio as usize));
    let mut val = readl(addr);
    val &= !offset(gpio as usize);
    writel(val, addr);
    spin_unlock_irqrestore(&mut (*gchip).lock, flags);
    0
}

unsafe fn mb86s70_gpio_free(gc: *mut gpio_chip, gpio: c_uint) {
    let gchip = gpiochip_get_data(gc);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*gchip).lock, &mut flags);
    let addr = (*gchip).base.add(pfr(gpio as usize));
    let mut val = readl(addr);
    val |= offset(gpio as usize);
    writel(val, addr);
    spin_unlock_irqrestore(&mut (*gchip).lock, flags);
}

unsafe fn mb86s70_gpio_direction_input(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let gchip = gpiochip_get_data(gc);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*gchip).lock, &mut flags);
    let addr = (*gchip).base.add(ddr(gpio as usize));
    let mut val = readl(addr) as u8;
    val &= !(offset(gpio as usize) as u8);
    writel(val as u32, addr);
    spin_unlock_irqrestore(&mut (*gchip).lock, flags);
    0
}

unsafe fn mb86s70_gpio_direction_output(gc: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    let gchip = gpiochip_get_data(gc);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*gchip).lock, &mut flags);
    let addr = (*gchip).base.add(pdr(gpio as usize));
    let mut val = readl(addr) as u8;
    if value != 0 { val |= offset(gpio as usize) as u8; } else { val &= !(offset(gpio as usize) as u8); }
    writel(val as u32, addr);
    let addr = (*gchip).base.add(ddr(gpio as usize));
    val = readl(addr) as u8;
    val |= offset(gpio as usize) as u8;
    writel(val as u32, addr);
    spin_unlock_irqrestore(&mut (*gchip).lock, flags);
    0
}

unsafe fn mb86s70_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let gchip = gpiochip_get_data(gc);
    (readl((*gchip).base.add(pdr(gpio as usize)) ) & offset(gpio as usize) != 0) as c_int
}

unsafe fn mb86s70_gpio_set(gc: *mut gpio_chip, gpio: c_uint, value: c_int) {
    let gchip = gpiochip_get_data(gc);
    let mut flags = 0;
    spin_lock_irqsave(&mut (*gchip).lock, &mut flags);
    let addr = (*gchip).base.add(pdr(gpio as usize));
    let mut val = readl(addr) as u8;
    if value != 0 { val |= offset(gpio as usize) as u8; } else { val &= !(offset(gpio as usize) as u8); }
    writel(val as u32, addr);
    spin_unlock_irqrestore(&mut (*gchip).lock, flags);
}

unsafe fn mb86s70_gpio_to_irq(gc: *mut gpio_chip, offset_: c_uint) -> c_int {
    let mut index = 0;
    loop {
        let irq = platform_get_irq(to_platform_device((*gc).parent), index);
        if irq < 0 { return irq; }
        if (*irq_get_irq_data(irq)).hwirq == offset_ as u64 { return irq; }
        index += 1;
    }
}

unsafe fn mb86s70_gpio_probe(pdev: *mut platform_device) -> c_int {
    let gchip = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Mb86s70GpioChip>(), GFP_KERNEL)
        as *mut Mb86s70GpioChip;
    if gchip.is_null() { return -ENOMEM; }
    platform_set_drvdata(pdev, gchip);
    (*gchip).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*gchip).base) { return ptr_err((*gchip).base); }
    let clk = devm_clk_get_optional_enabled(&mut (*pdev).dev, core::ptr::null());
    if is_err(clk as *mut core::ffi::c_void) { return ptr_err(clk as *mut core::ffi::c_void); }
    spin_lock_init(&mut (*gchip).lock);
    (*gchip).gc.direction_output = Some(mb86s70_gpio_direction_output);
    (*gchip).gc.direction_input = Some(mb86s70_gpio_direction_input);
    (*gchip).gc.request = Some(mb86s70_gpio_request);
    (*gchip).gc.free = Some(mb86s70_gpio_free);
    (*gchip).gc.get = Some(mb86s70_gpio_get);
    (*gchip).gc.set = Some(mb86s70_gpio_set);
    (*gchip).gc.to_irq = Some(mb86s70_gpio_to_irq);
    (*gchip).gc.ngpio = 32;
    (*gchip).gc.parent = &mut (*pdev).dev;
    (*gchip).gc.base = -1;
    let ret = gpiochip_add_data(&mut (*gchip).gc, gchip);
    if ret != 0 { return dev_err_probe(&mut (*pdev).dev, ret, b"couldn't register gpio driver\0".as_ptr() as *const c_char); }
    acpi_gpiochip_request_interrupts(&mut (*gchip).gc);
    0
}

unsafe fn mb86s70_gpio_remove(pdev: *mut platform_device) {
    let gchip = platform_get_drvdata(pdev);
    acpi_gpiochip_free_interrupts(&mut (*gchip).gc);
    gpiochip_remove(&mut (*gchip).gc);
}

#[repr(C)]
struct of_device_id { compatible: *const c_char }
static MB86S70_GPIO_DT_IDS: [of_device_id; 2] = [
    of_device_id { compatible: b"fujitsu,mb86s70-gpio\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

// CONFIG_ACPI conditionally includes the SCX0007 ACPI match table.
#[cfg(feature = "CONFIG_ACPI")]
static MB86S70_GPIO_ACPI_IDS: [&'static [u8]; 2] = [b"SCX0007\0", b"\0"];

// Equivalent of module_platform_driver(mb86s70_gpio_driver).
// MODULE_DESCRIPTION("MB86S7x GPIO Driver");
// MODULE_ALIAS("platform:mb86s70-gpio");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
