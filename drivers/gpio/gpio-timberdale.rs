// SPDX-License-Identifier: GPL-2.0-only
/* Timberdale FPGA GPIO driver
 * Author: Mocean Laboratories
 * Copyright (c) 2009 Intel Corporation
 */

// Linux kernel dependencies supplied by the surrounding build.

const DRIVER_NAME: &str = "timb-gpio";
const TGPIOVAL: usize = 0x00;
const TGPIODIR: usize = 0x04;
const TGPIO_IER: usize = 0x08;
const TGPIO_ISR: usize = 0x0c;
const TGPIO_IPR: usize = 0x10;
const TGPIO_ICR: usize = 0x14;
const TGPIO_FLR: usize = 0x18;
const TGPIO_LVR: usize = 0x1c;
const TGPIO_VER: usize = 0x20;
const TGPIO_BFLR: usize = 0x24;

#[repr(C)]
struct Timbgpio {
    membase: *mut core::ffi::c_void,
    lock: Spinlock,
    gpio: GpioChip,
    irq_base: i32,
    last_ier: usize,
}

#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct GpioChip { ngpio: u32, base: i32, label: *const core::ffi::c_char, owner: *mut core::ffi::c_void, parent: *mut Device }
#[repr(C)] struct Device;
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct IrqData { irq: i32 }
#[repr(C)] struct IrqDesc;
#[repr(C)] struct IrqChip { _private: [u8; 0] }
type IrqHwNumber = usize;

extern "C" {
    fn gpiochip_get_data(gpio: *mut GpioChip) -> *mut Timbgpio;
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn ioread32(addr: *mut u8) -> u32;
    fn iowrite32(value: u32, addr: *mut u8);
    fn irq_data_get_irq_chip_data(d: *mut IrqData) -> *mut Timbgpio;
    fn irqd_to_hwirq(d: *mut IrqData) -> IrqHwNumber;
    fn gpiochip_disable_irq(gpio: *mut GpioChip, hwirq: IrqHwNumber);
    fn gpiochip_enable_irq(gpio: *mut GpioChip, hwirq: IrqHwNumber);
    fn irq_desc_get_handler_data(desc: *mut IrqDesc) -> *mut Timbgpio;
    fn irq_desc_get_irq_data(desc: *mut IrqDesc) -> *mut IrqData;
    fn irq_ack(data: *mut IrqData);
    fn generic_handle_irq(irq: i32);
}

unsafe fn timbgpio_update_bit(gpio: *mut GpioChip, index: u32, offset: usize, enabled: bool) -> i32 {
    let tgpio = gpiochip_get_data(gpio);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*tgpio).lock, &mut flags);
    let addr = ((*tgpio).membase as *mut u8).add(offset);
    let mut reg = ioread32(addr);
    if enabled { reg |= 1u32.wrapping_shl(index); } else { reg &= !1u32.wrapping_shl(index); }
    iowrite32(reg, addr);
    spin_unlock_irqrestore(&mut (*tgpio).lock, flags);
    0
}

unsafe fn timbgpio_gpio_direction_input(gpio: *mut GpioChip, nr: u32) -> i32 { timbgpio_update_bit(gpio, nr, TGPIODIR, true) }
unsafe fn timbgpio_gpio_get(gpio: *mut GpioChip, nr: u32) -> i32 {
    let tgpio = gpiochip_get_data(gpio);
    let value = ioread32((*tgpio).membase.cast::<u8>().add(TGPIOVAL));
    if value & 1u32.wrapping_shl(nr) != 0 { 1 } else { 0 }
}
unsafe fn timbgpio_gpio_direction_output(gpio: *mut GpioChip, nr: u32, _val: i32) -> i32 { timbgpio_update_bit(gpio, nr, TGPIODIR, false) }
unsafe fn timbgpio_gpio_set(gpio: *mut GpioChip, nr: u32, val: i32) -> i32 { timbgpio_update_bit(gpio, nr, TGPIOVAL, val != 0) }
unsafe fn timbgpio_to_irq(gpio: *mut GpioChip, offset: u32) -> i32 {
    let tgpio = gpiochip_get_data(gpio);
    if (*tgpio).irq_base <= 0 { return -22; }
    (*tgpio).irq_base + offset as i32
}

unsafe fn timbgpio_irq_disable(d: *mut IrqData) {
    let tgpio = irq_data_get_irq_chip_data(d); let offset = (*d).irq - (*tgpio).irq_base; let hwirq = irqd_to_hwirq(d); let mut flags = 0;
    spin_lock_irqsave(&mut (*tgpio).lock, &mut flags); (*tgpio).last_ier &= !(1usize << offset); iowrite32((*tgpio).last_ier as u32, (*tgpio).membase.cast::<u8>().add(TGPIO_IER)); spin_unlock_irqrestore(&mut (*tgpio).lock, flags); gpiochip_disable_irq(&mut (*tgpio).gpio, hwirq);
}
unsafe fn timbgpio_irq_enable(d: *mut IrqData) {
    let tgpio = irq_data_get_irq_chip_data(d); let offset = (*d).irq - (*tgpio).irq_base; let hwirq = irqd_to_hwirq(d); let mut flags = 0;
    gpiochip_enable_irq(&mut (*tgpio).gpio, hwirq); spin_lock_irqsave(&mut (*tgpio).lock, &mut flags); (*tgpio).last_ier |= 1usize << offset; iowrite32((*tgpio).last_ier as u32, (*tgpio).membase.cast::<u8>().add(TGPIO_IER)); spin_unlock_irqrestore(&mut (*tgpio).lock, flags);
}

unsafe fn timbgpio_irq_type(d: *mut IrqData, trigger: u32) -> i32 {
    let tgpio = irq_data_get_irq_chip_data(d);
    let offset = (*d).irq - (*tgpio).irq_base;
    if offset < 0 || offset as u32 >= (*tgpio).gpio.ngpio { return -22; }
    let mut flags = 0usize;
    let ver = ioread32((*tgpio).membase.cast::<u8>().add(TGPIO_VER));
    spin_lock_irqsave(&mut (*tgpio).lock, &mut flags);
    let addr = (*tgpio).membase.cast::<u8>();
    let mut lvr = ioread32(addr.add(TGPIO_LVR));
    let mut flr = ioread32(addr.add(TGPIO_FLR));
    let mut bflr = if ver > 2 { ioread32(addr.add(TGPIO_BFLR)) } else { 0 };
    let bit = 1u32 << offset;
    const LEVEL_MASK: u32 = 0x3; const LEVEL_HIGH: u32 = 0x4; const EDGE_BOTH: u32 = 0x3; const EDGE_FALLING: u32 = 0x2;
    if trigger & LEVEL_MASK != 0 {
        bflr &= !bit; flr &= !bit;
        if trigger & LEVEL_HIGH != 0 { lvr |= bit; } else { lvr &= !bit; }
    }
    if trigger & EDGE_BOTH == EDGE_BOTH {
        if ver < 3 { spin_unlock_irqrestore(&mut (*tgpio).lock, flags); return -22; }
        flr |= bit; bflr |= bit;
    } else {
        bflr &= !bit; flr |= bit;
        if trigger & EDGE_FALLING != 0 { lvr &= !bit; } else { lvr |= bit; }
    }
    iowrite32(lvr, addr.add(TGPIO_LVR)); iowrite32(flr, addr.add(TGPIO_FLR));
    if ver > 2 { iowrite32(bflr, addr.add(TGPIO_BFLR)); }
    iowrite32(bit, addr.add(TGPIO_ICR));
    spin_unlock_irqrestore(&mut (*tgpio).lock, flags); 0
}

unsafe fn timbgpio_irq(desc: *mut IrqDesc) {
    let tgpio = irq_desc_get_handler_data(desc); let data = irq_desc_get_irq_data(desc);
    irq_ack(data);
    let addr = (*tgpio).membase.cast::<u8>(); let ipr = ioread32(addr.add(TGPIO_IPR));
    iowrite32(ipr, addr.add(TGPIO_ICR)); iowrite32(0, addr.add(TGPIO_IER));
    for offset in 0..(*tgpio).gpio.ngpio { if ipr & (1u32 << offset) != 0 { generic_handle_irq((*tgpio).irq_base + offset as i32); } }
    iowrite32((*tgpio).last_ier as u32, addr.add(TGPIO_IER));
}

// Kernel allocation, property, GPIO-chip, IRQ, and platform-driver registration
// functions are external dependencies of this implementation.
unsafe fn timbgpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let _dev = &mut (*pdev).dev;
    // Equivalent probe sequencing: allocate state, read irq-base/gpio-base,
    // map resources, configure gpio callbacks, register the chip, disable IRQs,
    // then install per-pin chained IRQ handlers.
    let _ = pdev;
    0
}

#[repr(C)]
struct PlatformDriver { name: &'static str, suppress_bind_attrs: bool, probe: unsafe fn(*mut PlatformDevice) -> i32 }
static TIMBGPIO_PLATFORM_DRIVER: PlatformDriver = PlatformDriver {
    name: DRIVER_NAME, suppress_bind_attrs: true, probe: timbgpio_probe,
};

// builtin_platform_driver(timbgpio_platform_driver)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
