// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of gpio-tegra186.c. External kernel
 * types, helpers, and constants are intentionally left as dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const TEGRA186_GPIO_CTL_SCR: usize = 0x0c;
const TEGRA186_GPIO_CTL_SCR_SEC_WEN: u32 = 1 << 28;
const TEGRA186_GPIO_CTL_SCR_SEC_REN: u32 = 1 << 27;
const TEGRA186_GPIO_VM: usize = 0x00;
const TEGRA186_GPIO_VM_RW_MASK: u32 = 0x03;
const TEGRA186_GPIO_SCR: usize = 0x04;
const TEGRA186_GPIO_SCR_PIN_SIZE: usize = 0x08;
const TEGRA186_GPIO_SCR_PORT_SIZE: usize = 0x40;
const TEGRA186_GPIO_SCR_SEC_WEN: u32 = 1 << 28;
const TEGRA186_GPIO_SCR_SEC_REN: u32 = 1 << 27;
const TEGRA186_GPIO_SCR_SEC_G1W: u32 = 1 << 9;
const TEGRA186_GPIO_SCR_SEC_G1R: u32 = 1 << 1;
const TEGRA186_GPIO_ENABLE_CONFIG: usize = 0x00;
const TEGRA186_GPIO_ENABLE_CONFIG_ENABLE: u32 = 1 << 0;
const TEGRA186_GPIO_ENABLE_CONFIG_OUT: u32 = 1 << 1;
const TEGRA186_GPIO_ENABLE_CONFIG_TRIGGER_TYPE_NONE: u32 = 0 << 2;
const TEGRA186_GPIO_ENABLE_CONFIG_TRIGGER_TYPE_LEVEL: u32 = 1 << 2;
const TEGRA186_GPIO_ENABLE_CONFIG_TRIGGER_TYPE_SINGLE_EDGE: u32 = 2 << 2;
const TEGRA186_GPIO_ENABLE_CONFIG_TRIGGER_TYPE_DOUBLE_EDGE: u32 = 3 << 2;
const TEGRA186_GPIO_ENABLE_CONFIG_TRIGGER_TYPE_MASK: u32 = 3 << 2;
const TEGRA186_GPIO_ENABLE_CONFIG_TRIGGER_LEVEL: u32 = 1 << 4;
const TEGRA186_GPIO_ENABLE_CONFIG_DEBOUNCE: u32 = 1 << 5;
const TEGRA186_GPIO_ENABLE_CONFIG_INTERRUPT: u32 = 1 << 6;
const TEGRA186_GPIO_ENABLE_CONFIG_TIMESTAMP_FUNC: u32 = 1 << 7;
const TEGRA186_GPIO_DEBOUNCE_CONTROL: usize = 0x04;
const TEGRA186_GPIO_INPUT: usize = 0x08;
const TEGRA186_GPIO_INPUT_HIGH: u32 = 1;
const TEGRA186_GPIO_OUTPUT_CONTROL: usize = 0x0c;
const TEGRA186_GPIO_OUTPUT_CONTROL_FLOATED: u32 = 1;
const TEGRA186_GPIO_OUTPUT_VALUE: usize = 0x10;
const TEGRA186_GPIO_OUTPUT_VALUE_HIGH: u32 = 1;
const TEGRA186_GPIO_INTERRUPT_CLEAR: usize = 0x14;

#[inline]
const fn tegra186_gpio_int_route_mapping(p: usize, x: usize) -> usize { 0x14 + p * 0x20 + x * 4 }
#[inline]
const fn tegra186_gpio_interrupt_status(x: usize) -> usize { 0x100 + x * 4 }

#[repr(C)]
pub struct tegra_gpio_port { pub name: *const core::ffi::c_char, pub bank: u32, pub port: u32, pub pins: u32 }
#[repr(C)]
pub struct tegra186_pin_range { pub offset: u32, pub group: *const core::ffi::c_char }
#[repr(C)]
pub struct tegra_gpio_soc {
    pub ports: *const tegra_gpio_port, pub num_ports: u32, pub name: *const core::ffi::c_char,
    pub prefix: *const core::ffi::c_char, pub instance: u32, pub num_irqs_per_bank: u32,
    pub pin_ranges: *const tegra186_pin_range, pub num_pin_ranges: u32,
    pub pinmux: *const core::ffi::c_char, pub has_gte: bool, pub has_vm_support: bool,
}
#[repr(C)]
pub struct tegra_gpio { pub gpio: gpio_chip, pub num_irq: u32, pub soc: *const tegra_gpio_soc,
    pub num_irqs_per_bank: u32, pub num_banks: u32, pub secure: *mut u8, pub base: *mut u8,
    pub irq: *mut u32 }

// Kernel interfaces supplied by the surrounding tree.
#[repr(C)] pub struct gpio_chip { _private: [u8; 0] }
extern "C" {
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut tegra_gpio;
    fn readl(addr: *mut u8) -> u32; fn __raw_readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
}

unsafe fn tegra186_gpio_get_port(gpio: *mut tegra_gpio, pin: &mut u32) -> *const tegra_gpio_port {
    let soc = &*(*gpio).soc; let mut start = 0;
    for i in 0..soc.num_ports { let port = &*soc.ports.add(i as usize);
        if *pin >= start && *pin < start + port.pins { *pin -= start; return port; }
        start += port.pins;
    } core::ptr::null()
}
unsafe fn tegra186_gpio_get_base(gpio: *mut tegra_gpio, mut pin: u32) -> *mut u8 {
    let port = tegra186_gpio_get_port(gpio, &mut pin); if port.is_null() { return core::ptr::null_mut(); }
    (*gpio).base.add((*port).bank as usize * 0x1000 + (*port).port as usize * 0x200 + pin as usize * 0x20)
}
unsafe fn tegra186_gpio_get_secure_base(gpio: *mut tegra_gpio, mut pin: u32) -> *mut u8 {
    let port = tegra186_gpio_get_port(gpio, &mut pin); if port.is_null() { return core::ptr::null_mut(); }
    (*gpio).secure.add((*port).bank as usize * 0x1000 + (*port).port as usize * TEGRA186_GPIO_SCR_PORT_SIZE + pin as usize * TEGRA186_GPIO_SCR_PIN_SIZE)
}
unsafe fn tegra186_gpio_is_accessible(gpio: *mut tegra_gpio, pin: u32) -> bool {
    let secure = tegra186_gpio_get_secure_base(gpio, pin); let soc = &*(*gpio).soc;
    if soc.has_vm_support && (readl(secure.add(TEGRA186_GPIO_VM)) & TEGRA186_GPIO_VM_RW_MASK) != TEGRA186_GPIO_VM_RW_MASK { return false; }
    let value = __raw_readl(secure.add(TEGRA186_GPIO_SCR));
    (value & TEGRA186_GPIO_SCR_SEC_REN == 0 || value & TEGRA186_GPIO_SCR_SEC_G1R != 0) &&
    (value & TEGRA186_GPIO_SCR_SEC_WEN == 0 || value & TEGRA186_GPIO_SCR_SEC_G1W != 0)
}

// The remaining callbacks and SoC tables retain the C driver's ABI and are
// declared here for linkage with the kernel translation unit.
extern "C" {
    pub fn tegra186_gpio_probe(pdev: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
