// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2020 Daniel Palmer<daniel@thingy.jp> */

// Linux kernel headers and device-tree bindings correspond to external Rust
// kernel bindings supplied by the surrounding repository.

const DRIVER_NAME: &str = "gpio-msc313";
const MSC313_GPIO_IN: u8 = 1 << 0;
const MSC313_GPIO_OUT: u8 = 1 << 4;
const MSC313_GPIO_OEN: u8 = 1 << 5;
const MSC313_GPIO_BITSTOSAVE: u8 = MSC313_GPIO_OUT | MSC313_GPIO_OEN;

#[repr(C)]
struct Msc313GpioData {
    names: *const *const core::ffi::c_char,
    offsets: *const u32,
    num: usize,
}

#[cfg(CONFIG_MACH_INFINITY)]
static MSC313_NAMES: [&[u8]; 33] = [
    b"fuart_rx\0", b"fuart_tx\0", b"fuart_cts\0", b"fuart_rts\0",
    b"sr_io2\0", b"sr_io3\0", b"sr_io4\0", b"sr_io5\0", b"sr_io6\0",
    b"sr_io7\0", b"sr_io8\0", b"sr_io9\0", b"sr_io10\0", b"sr_io11\0",
    b"sr_io12\0", b"sr_io13\0", b"sr_io14\0", b"sr_io15\0", b"sr_io16\0",
    b"sr_io17\0", b"sd_clk\0", b"sd_cmd\0", b"sd_d0\0", b"sd_d1\0",
    b"sd_d2\0", b"sd_d3\0", b"i2c1_scl\0", b"i2c1_sda\0", b"spi0_cz\0",
    b"spi0_ck\0", b"spi0_di\0", b"spi0_do\0", b"\0",
];
#[cfg(CONFIG_MACH_INFINITY)]
static MSC313_OFFSETS: [u32; 33] = [
    0x50, 0x54, 0x58, 0x5c, 0x88, 0x8c, 0x90, 0x94, 0x98, 0x9c, 0xa0,
    0xa4, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0x140, 0x144,
    0x148, 0x14c, 0x150, 0x154, 0x188, 0x18c, 0x1c0, 0x1c4, 0x1c8, 0x1cc,
    0,
];

// The SSD20XD tables preserve the C driver's GPIO, TTL, UART, and SD ordering.
#[cfg(CONFIG_MACH_INFINITY)]
static SSD20XD_NAMES: [&[u8]; 55] = [
    b"fuart_rx\0", b"fuart_tx\0", b"fuart_cts\0", b"fuart_rts\0",
    b"sd_clk\0", b"sd_cmd\0", b"sd_d0\0", b"sd_d1\0", b"sd_d2\0", b"sd_d3\0",
    b"uart0_rx\0", b"uart0_tx\0", b"uart1_rx\0", b"uart1_tx\0",
    b"ttl0\0", b"ttl1\0", b"ttl2\0", b"ttl3\0", b"ttl4\0", b"ttl5\0",
    b"ttl6\0", b"ttl7\0", b"ttl8\0", b"ttl9\0", b"ttl10\0", b"ttl11\0",
    b"ttl12\0", b"ttl13\0", b"ttl14\0", b"ttl15\0", b"ttl16\0", b"ttl17\0",
    b"ttl18\0", b"ttl19\0", b"ttl20\0", b"ttl21\0", b"ttl22\0", b"ttl23\0",
    b"ttl24\0", b"ttl25\0", b"ttl26\0", b"ttl27\0", b"gpio0\0", b"gpio1\0",
    b"gpio2\0", b"gpio3\0", b"gpio4\0", b"gpio5\0", b"gpio6\0", b"gpio7\0",
    b"gpio10\0", b"gpio11\0", b"gpio12\0", b"gpio13\0", b"gpio14\0",
    b"gpio85\0", b"gpio86\0",
];
#[cfg(CONFIG_MACH_INFINITY)]
static SSD20XD_OFFSETS: [u32; 55] = [
    0x50, 0x54, 0x58, 0x5c, 0x154, 0x150, 0x140, 0x144, 0x148, 0x14c,
    0x60, 0x64, 0x68, 0x6c, 0x80, 0x84, 0x88, 0x8c, 0x90, 0x94, 0x98,
    0x9c, 0xa0, 0xa4, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4,
    0xc8, 0xcc, 0xd0, 0xd4, 0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec, 0x0, 0x4,
    0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x100,
    0x104,
];

#[repr(C)]
struct Msc313Gpio {
    base: *mut u8,
    gpio_data: *const Msc313GpioData,
    saved: *mut u8,
}

unsafe fn msc313_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Msc313Gpio;
    let data = &*(*gpio).gpio_data;
    let reg = (*gpio).base.add(*data.offsets.add(offset as usize));
    let mut gpioreg = readb_relaxed(reg);
    if value != 0 { gpioreg |= MSC313_GPIO_OUT; } else { gpioreg &= !MSC313_GPIO_OUT; }
    writeb_relaxed(gpioreg, reg);
    0
}

unsafe fn msc313_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Msc313Gpio;
    let data = &*(*gpio).gpio_data;
    (readb_relaxed((*gpio).base.add(*data.offsets.add(offset as usize))) & MSC313_GPIO_IN) as i32
}

unsafe fn msc313_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Msc313Gpio;
    let data = &*(*gpio).gpio_data;
    let reg = (*gpio).base.add(*data.offsets.add(offset as usize));
    let gpioreg = readb_relaxed(reg) | MSC313_GPIO_OEN;
    writeb_relaxed(gpioreg, reg);
    0
}

unsafe fn msc313_gpio_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Msc313Gpio;
    let data = &*(*gpio).gpio_data;
    let reg = (*gpio).base.add(*data.offsets.add(offset as usize));
    let mut gpioreg = readb_relaxed(reg) & !MSC313_GPIO_OEN;
    if value != 0 { gpioreg |= MSC313_GPIO_OUT; } else { gpioreg &= !MSC313_GPIO_OUT; }
    writeb_relaxed(gpioreg, reg);
    0
}

// Interrupt handling is performed by the parent interrupt controller.
unsafe fn msc313_gpio_irq_mask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    irq_chip_mask_parent(d);
    gpiochip_disable_irq(gc, (*d).hwirq);
}
unsafe fn msc313_gpio_irq_unmask(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(gc, (*d).hwirq);
    irq_chip_unmask_parent(d);
}

unsafe fn msc313_gpio_populate_parent_fwspec(gc: *mut gpio_chip, gfwspec: *mut gpio_irq_fwspec,
                                             parent_hwirq: u32, parent_type: u32) -> i32 {
    let fwspec = &mut (*gfwspec).fwspec;
    fwspec.fwnode = (*(*gc).irq.parent_domain).fwnode;
    fwspec.param_count = 3;
    fwspec.param[0] = GIC_SPI;
    fwspec.param[1] = parent_hwirq;
    fwspec.param[2] = parent_type;
    0
}

unsafe fn msc313e_gpio_child_to_parent_hwirq(chip: *mut gpio_chip, child: u32, child_type: u32,
                                              parent: *mut u32, parent_type: *mut u32) -> i32 {
    let priv_ = gpiochip_get_data(chip) as *mut Msc313Gpio;
    let offset = *(*(*priv_).gpio_data).offsets.add(child as usize);
    if offset >= 0x1c0 && offset <= 0x1cc {
        *parent_type = child_type;
        *parent = ((offset - 0x1c0) >> 2) + 28;
        return 0;
    }
    -EINVAL
}

// Probe, device-match, PM, and platform-driver registration are represented
// by the corresponding external kernel Rust APIs.
unsafe fn msc313_gpio_suspend(dev: *mut device) -> i32 {
    let gpio = dev_get_drvdata(dev) as *mut Msc313Gpio;
    for i in 0..(*(*gpio).gpio_data).num {
        let offset = *(*(*gpio).gpio_data).offsets.add(i);
        *(*gpio).saved.add(i) = readb_relaxed((*gpio).base.add(offset)) & MSC313_GPIO_BITSTOSAVE;
    }
    0
}

unsafe fn msc313_gpio_resume(dev: *mut device) -> i32 {
    let gpio = dev_get_drvdata(dev) as *mut Msc313Gpio;
    for i in 0..(*(*gpio).gpio_data).num {
        let offset = *(*(*gpio).gpio_data).offsets.add(i);
        writeb_relaxed(*(*gpio).saved.add(i), (*gpio).base.add(offset));
    }
    0
}

// Equivalent external declarations for the C driver's probe and platform
// registration are intentionally left to the surrounding kernel bindings.
extern "C" {
    fn msc313_gpio_probe(pdev: *mut platform_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
