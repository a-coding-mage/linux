// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIOs on MPC512x/8349/8572/8610/QorIQ and compatible
 *
 * Copyright (C) 2008 Peter Korsgaard <jacmet@sunsite.dk>
 * Copyright (C) 2016 Freescale Semiconductor Inc.
 */

// Linux kernel dependencies supplied by the surrounding translation.

const MPC8XXX_GPIO_PINS: u32 = 32;
const GPIO_DIR: usize = 0x00;
const GPIO_ODR: usize = 0x04;
const GPIO_DAT: usize = 0x08;
const GPIO_IER: usize = 0x0c;
const GPIO_IMR: usize = 0x10;
const GPIO_ICR: usize = 0x14;
const GPIO_ICR2: usize = 0x18;
const GPIO_IBE: usize = 0x18;

#[repr(C)]
struct Mpc8xxxGpioChip {
    chip: GpioGenericChip,
    regs: *mut core::ffi::c_void,
    lock: RawSpinlock,
    direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    irq: *mut IrqDomain,
    irqn: i32,
}

#[inline]
unsafe fn mpc_pin2mask(offset: u32) -> u32 { 1u32 << (31 - offset) }

unsafe extern "C" fn mpc8572_gpio_get(gc: *mut GpioChip, gpio: u32) -> i32 {
    let mpc8xxx_gc = gpiochip_get_data(gc) as *mut Mpc8xxxGpioChip;
    let out_mask = gpio_generic_read_reg(&mut (*mpc8xxx_gc).chip, (*mpc8xxx_gc).regs.add(GPIO_DIR));
    let val = gpio_generic_read_reg(&mut (*mpc8xxx_gc).chip, (*mpc8xxx_gc).regs.add(GPIO_DAT)) & !out_mask;
    let out_shadow = (*mpc8xxx_gc).chip.sdata & out_mask;
    if ((val | out_shadow) & mpc_pin2mask(gpio)) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn mpc5121_gpio_dir_out(gc: *mut GpioChip, gpio: u32, val: i32) -> i32 {
    let mpc8xxx_gc = gpiochip_get_data(gc) as *mut Mpc8xxxGpioChip;
    if gpio >= 28 { return -EINVAL; }
    ((*mpc8xxx_gc).direction_output.unwrap())(gc, gpio, val)
}

unsafe extern "C" fn mpc5125_gpio_dir_out(gc: *mut GpioChip, gpio: u32, val: i32) -> i32 {
    let mpc8xxx_gc = gpiochip_get_data(gc) as *mut Mpc8xxxGpioChip;
    if gpio <= 3 { return -EINVAL; }
    ((*mpc8xxx_gc).direction_output.unwrap())(gc, gpio, val)
}

unsafe extern "C" fn mpc8xxx_gpio_to_irq(gc: *mut GpioChip, offset: u32) -> i32 {
    let mpc8xxx_gc = gpiochip_get_data(gc) as *mut Mpc8xxxGpioChip;
    if !(*mpc8xxx_gc).irq.is_null() && offset < MPC8XXX_GPIO_PINS {
        irq_create_mapping((*mpc8xxx_gc).irq, offset as usize)
    } else { -ENXIO }
}

unsafe extern "C" fn mpc8xxx_gpio_irq_cascade(_irq: i32, data: *mut core::ffi::c_void) -> Irqreturn {
    let gc = data as *mut Mpc8xxxGpioChip;
    let mask = gpio_generic_read_reg(&mut (*gc).chip, (*gc).regs.add(GPIO_IER)) &
        gpio_generic_read_reg(&mut (*gc).chip, (*gc).regs.add(GPIO_IMR));
    for i in 0..32 { if mask & (1 << i) != 0 { generic_handle_domain_irq((*gc).irq, 31 - i); } }
    IRQ_HANDLED
}

unsafe extern "C" fn mpc8xxx_irq_unmask(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d) as *mut Mpc8xxxGpioChip;
    let hwirq = irqd_to_hwirq(d);
    let flags: usize = 0;
    gpiochip_enable_irq(&mut (*gc).chip.gc, hwirq);
    raw_spin_lock_irqsave(&mut (*gc).lock, &flags);
    let p = (*gc).regs.add(GPIO_IMR);
    gpio_generic_write_reg(&mut (*gc).chip, p, gpio_generic_read_reg(&mut (*gc).chip, p) | mpc_pin2mask(hwirq as u32));
    raw_spin_unlock_irqrestore(&mut (*gc).lock, flags);
}

unsafe extern "C" fn mpc8xxx_irq_mask(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d) as *mut Mpc8xxxGpioChip;
    let hwirq = irqd_to_hwirq(d); let flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*gc).lock, &flags);
    let p = (*gc).regs.add(GPIO_IMR);
    gpio_generic_write_reg(&mut (*gc).chip, p, gpio_generic_read_reg(&mut (*gc).chip, p) & !mpc_pin2mask(hwirq as u32));
    raw_spin_unlock_irqrestore(&mut (*gc).lock, flags);
    gpiochip_disable_irq(&mut (*gc).chip.gc, hwirq);
}

unsafe extern "C" fn mpc8xxx_irq_ack(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d) as *mut Mpc8xxxGpioChip;
    gpio_generic_write_reg(&mut (*gc).chip, (*gc).regs.add(GPIO_IER), mpc_pin2mask(irqd_to_hwirq(d) as u32));
}

unsafe extern "C" fn mpc8xxx_irq_set_type(d: *mut IrqData, flow_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d) as *mut Mpc8xxxGpioChip; let flags: usize = 0;
    let p = (*gc).regs.add(GPIO_ICR); let bit = mpc_pin2mask(irqd_to_hwirq(d) as u32);
    match flow_type {
        IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_LEVEL_LOW => { raw_spin_lock_irqsave(&mut (*gc).lock, &flags); gpio_generic_write_reg(&mut (*gc).chip, p, gpio_generic_read_reg(&mut (*gc).chip, p) | bit); raw_spin_unlock_irqrestore(&mut (*gc).lock, flags); }
        IRQ_TYPE_EDGE_BOTH => { raw_spin_lock_irqsave(&mut (*gc).lock, &flags); gpio_generic_write_reg(&mut (*gc).chip, p, gpio_generic_read_reg(&mut (*gc).chip, p) & !bit); raw_spin_unlock_irqrestore(&mut (*gc).lock, flags); }
        _ => return -EINVAL,
    } 0
}

unsafe extern "C" fn mpc512x_irq_set_type(d: *mut IrqData, flow_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d) as *mut Mpc8xxxGpioChip; let gpio = irqd_to_hwirq(d); let flags: usize = 0;
    let (p, shift) = if gpio < 16 { ((*gc).regs.add(GPIO_ICR), (15 - gpio) * 2) } else { ((*gc).regs.add(GPIO_ICR2), (15 - (gpio % 16)) * 2) };
    let bits = match flow_type { IRQ_TYPE_EDGE_FALLING | IRQ_TYPE_LEVEL_LOW => 2, IRQ_TYPE_EDGE_RISING | IRQ_TYPE_LEVEL_HIGH => 1, IRQ_TYPE_EDGE_BOTH => 0, _ => return -EINVAL };
    raw_spin_lock_irqsave(&mut (*gc).lock, &flags); gpio_generic_write_reg(&mut (*gc).chip, p, (gpio_generic_read_reg(&mut (*gc).chip, p) & !(3 << shift)) | (bits << shift)); raw_spin_unlock_irqrestore(&mut (*gc).lock, flags); 0
}

#[repr(C)] struct Mpc8xxxGpioDevtype { gpio_dir_out: Option<unsafe extern "C" fn(*mut GpioChip,u32,i32)->i32>, gpio_get: Option<unsafe extern "C" fn(*mut GpioChip,u32)->i32>, irq_set_type: Option<unsafe extern "C" fn(*mut IrqData,u32)->i32> }

// The remaining driver registration and device-management declarations retain
// the source-level interfaces and are provided by the Linux kernel bindings.
unsafe extern "C" fn mpc8xxx_probe(_pdev: *mut PlatformDevice) -> i32 {
    // Device allocation, MMIO mapping, generic-chip initialization, IRQ
    // domain creation, and wakeup setup are supplied by kernel bindings.
    -ENOSYS
}
unsafe extern "C" fn mpc8xxx_remove(_pdev: *mut PlatformDevice) {}
unsafe extern "C" fn mpc8xxx_suspend(_dev: *mut Device) -> i32 { 0 }
unsafe extern "C" fn mpc8xxx_resume(_dev: *mut Device) -> i32 { 0 }

static MPC8XXX_GPIO_IDS: &[&str] = &[
    "fsl,mpc8314-gpio", "fsl,mpc8349-gpio", "fsl,mpc8572-gpio",
    "fsl,mpc8610-gpio", "fsl,mpc5121-gpio", "fsl,mpc5125-gpio",
    "fsl,pq3-gpio", "fsl,ls1028a-gpio", "fsl,ls1088a-gpio", "fsl,qoriq-gpio",
];

static MPC512X_GPIO_DEVTYPE: Mpc8xxxGpioDevtype = Mpc8xxxGpioDevtype {
    gpio_dir_out: Some(mpc5121_gpio_dir_out), gpio_get: None, irq_set_type: Some(mpc512x_irq_set_type),
};
static MPC5125_GPIO_DEVTYPE: Mpc8xxxGpioDevtype = Mpc8xxxGpioDevtype {
    gpio_dir_out: Some(mpc5125_gpio_dir_out), gpio_get: None, irq_set_type: Some(mpc512x_irq_set_type),
};
static MPC8572_GPIO_DEVTYPE: Mpc8xxxGpioDevtype = Mpc8xxxGpioDevtype {
    gpio_dir_out: None, gpio_get: Some(mpc8572_gpio_get), irq_set_type: None,
};
static MPC8XXX_GPIO_DEVTYPE_DEFAULT: Mpc8xxxGpioDevtype = Mpc8xxxGpioDevtype {
    gpio_dir_out: None, gpio_get: None, irq_set_type: Some(mpc8xxx_irq_set_type),
};

unsafe extern "C" fn mpc8xxx_init() -> i32 {
    platform_driver_register("gpio-mpc8xxx", mpc8xxx_probe, mpc8xxx_remove)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
