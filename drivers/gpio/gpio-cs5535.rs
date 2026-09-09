// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD CS5535/CS5536 GPIO driver
 * Copyright (C) 2006  Advanced Micro Devices, Inc.
 * Copyright (C) 2007-2009  Andres Salomon <dilinger@collabora.co.uk>
 */

// Linux kernel dependencies supplied by the surrounding translation.

const DRV_NAME: &str = "cs5535-gpio";
const GPIO_DEFAULT_MASK: usize = 0x0F7FFFFF;

static mut MASK: usize = GPIO_DEFAULT_MASK;

#[repr(C)]
struct Cs5535GpioChip {
    chip: GpioChip,
    base: ResourceSizeT,
    pdev: *mut PlatformDevice,
    lock: SpinlockT,
}

static mut CS5535_GPIO_CHIP: Cs5535GpioChip = Cs5535GpioChip {
    chip: GpioChip::zeroed(), base: 0, pdev: core::ptr::null_mut(), lock: SpinlockT::zeroed(),
};

// External kernel types and functions are provided by the translated dependencies.
type ResourceSizeT = usize;
type SpinlockT = u8;
#[repr(C)] struct GpioChip { owner: *mut core::ffi::c_void, label: *const u8, ngpio: u32, names: *const *const u8 }
impl GpioChip { const fn zeroed() -> Self { Self { owner: core::ptr::null_mut(), label: core::ptr::null(), ngpio: 0, names: core::ptr::null() } } }
#[repr(C)] struct PlatformDevice { _private: [u8; 0] }
extern "C" {
    fn inl(addr: usize) -> u32;
    fn outl(val: u32, addr: usize);
    fn rdmsr(msr: u32, lo: *mut u32, hi: *mut u32);
    fn wrmsr(msr: u32, lo: u32, hi: u32);
}

const GPIO_POSITIVE_EDGE_STS: u32 = 0;
const GPIO_NEGATIVE_EDGE_STS: u32 = 1;
const GPIO_MAP_W: u32 = 0;
const GPIO_MAP_Z: u32 = 0;
const GPIO_MAP_Y: u32 = 0;
const GPIO_MAP_X: u32 = 0;
const GPIO_OUTPUT_AUX1: u32 = 0;
const GPIO_OUTPUT_AUX2: u32 = 0;
const GPIO_INPUT_AUX1: u32 = 0;
const GPIO_READ_BACK: u32 = 0;
const GPIO_OUTPUT_VAL: u32 = 0;
const GPIO_INPUT_ENABLE: u32 = 0;
const GPIO_OUTPUT_ENABLE: u32 = 0;
const MSR_PIC_ZSEL_HIGH: u32 = 0;

unsafe fn errata_outl(chip: *mut Cs5535GpioChip, mut val: u32, reg: u32) {
    let addr = (*chip).base + 0x80 + reg as usize;
    if reg != GPIO_POSITIVE_EDGE_STS && reg != GPIO_NEGATIVE_EDGE_STS {
        if val & 0xffff != 0 { val |= inl(addr) & 0xffff; }
        else { val |= inl(addr) ^ (val >> 16); }
    }
    outl(val, addr);
}

unsafe fn __cs5535_gpio_set(chip: *mut Cs5535GpioChip, offset: u32, reg: u32) {
    if offset < 16 { outl(1u32 << offset, (*chip).base + reg as usize); }
    else { errata_outl(chip, 1u32 << (offset - 16), reg); }
}

#[no_mangle] pub unsafe extern "C" fn cs5535_gpio_set(offset: u32, reg: u32) {
    __cs5535_gpio_set(core::ptr::addr_of_mut!(CS5535_GPIO_CHIP), offset, reg);
}

unsafe fn __cs5535_gpio_clear(chip: *mut Cs5535GpioChip, offset: u32, reg: u32) {
    if offset < 16 { outl(1u32 << (offset + 16), (*chip).base + reg as usize); }
    else { errata_outl(chip, 1u32 << offset, reg); }
}

#[no_mangle] pub unsafe extern "C" fn cs5535_gpio_clear(offset: u32, reg: u32) {
    __cs5535_gpio_clear(core::ptr::addr_of_mut!(CS5535_GPIO_CHIP), offset, reg);
}

#[no_mangle] pub unsafe extern "C" fn cs5535_gpio_isset(offset: u32, reg: u32) -> i32 {
    let chip = core::ptr::addr_of_mut!(CS5535_GPIO_CHIP);
    let (val, bit) = if offset < 16 { (inl((*chip).base + reg as usize), offset) }
        else { (inl((*chip).base + 0x80 + reg as usize), offset - 16) };
    if val & (1u32 << bit) != 0 { 1 } else { 0 }
}

#[no_mangle] pub unsafe extern "C" fn cs5535_gpio_set_irq(group: u32, irq: u32) -> i32 {
    if group > 7 || irq > 15 { return -22; }
    let (mut lo, mut hi) = (0u32, 0u32); rdmsr(MSR_PIC_ZSEL_HIGH, &mut lo, &mut hi);
    lo &= !(0xF << (group * 4)); lo |= (irq & 0xF) << (group * 4);
    wrmsr(MSR_PIC_ZSEL_HIGH, lo, hi); 0
}

#[no_mangle] pub unsafe extern "C" fn cs5535_gpio_setup_event(mut offset: u32, pair: i32, pme: i32) {
    let shift = (offset % 8) * 4;
    offset = if offset >= 24 { GPIO_MAP_W } else if offset >= 16 { GPIO_MAP_Z } else if offset >= 8 { GPIO_MAP_Y } else { GPIO_MAP_X };
    let chip = core::ptr::addr_of_mut!(CS5535_GPIO_CHIP); let mut val = inl((*chip).base + offset as usize);
    val &= !(0xF << shift); val |= ((pair as u32 & 7) << shift);
    if pme != 0 { val |= 1 << (shift + 3); } outl(val, (*chip).base + offset as usize);
}

// Generic gpio_chip API support.
unsafe fn chip_gpio_request(_c: *mut GpioChip, offset: u32) -> i32 {
    if MASK & (1usize << offset) == 0 { return -22; }
    let chip = core::ptr::addr_of_mut!(CS5535_GPIO_CHIP);
    __cs5535_gpio_clear(chip, offset, GPIO_OUTPUT_AUX1);
    __cs5535_gpio_clear(chip, offset, GPIO_OUTPUT_AUX2);
    __cs5535_gpio_clear(chip, offset, GPIO_INPUT_AUX1);
    0
}
unsafe fn chip_gpio_get(_chip: *mut GpioChip, offset: u32) -> i32 { cs5535_gpio_isset(offset, GPIO_READ_BACK) }
unsafe fn chip_gpio_set(_chip: *mut GpioChip, offset: u32, val: i32) -> i32 { if val != 0 { cs5535_gpio_set(offset, GPIO_OUTPUT_VAL) } else { cs5535_gpio_clear(offset, GPIO_OUTPUT_VAL) } 0 }

unsafe fn chip_direction_input(_c: *mut GpioChip, offset: u32) -> i32 {
    let chip = core::ptr::addr_of_mut!(CS5535_GPIO_CHIP);
    __cs5535_gpio_set(chip, offset, GPIO_INPUT_ENABLE);
    __cs5535_gpio_clear(chip, offset, GPIO_OUTPUT_ENABLE); 0
}
unsafe fn chip_direction_output(_c: *mut GpioChip, offset: u32, val: i32) -> i32 {
    let chip = core::ptr::addr_of_mut!(CS5535_GPIO_CHIP);
    __cs5535_gpio_set(chip, offset, GPIO_INPUT_ENABLE);
    __cs5535_gpio_set(chip, offset, GPIO_OUTPUT_ENABLE);
    if val != 0 { __cs5535_gpio_set(chip, offset, GPIO_OUTPUT_VAL); }
    else { __cs5535_gpio_clear(chip, offset, GPIO_OUTPUT_VAL); } 0
}

static CS5535_GPIO_NAMES: [&[u8]; 32] = [b"GPIO0",b"GPIO1",b"GPIO2",b"GPIO3",b"GPIO4",b"GPIO5",b"GPIO6",b"GPIO7",b"GPIO8",b"GPIO9",b"GPIO10",b"GPIO11",b"GPIO12",b"GPIO13",b"GPIO14",b"GPIO15",b"GPIO16",b"GPIO17",b"GPIO18",b"GPIO19",b"GPIO20",b"GPIO21",b"GPIO22",b"",b"GPIO24",b"GPIO25",b"GPIO26",b"GPIO27",b"GPIO28",b"",b"",b""];

#[repr(C)] struct PlatformDriver { name: &'static str, probe: unsafe fn(*mut PlatformDevice) -> i32 }
static mut CS5535_GPIO_DRIVER: PlatformDriver = PlatformDriver { name: DRV_NAME, probe: cs5535_gpio_probe };

unsafe fn cs5535_gpio_probe(_pdev: *mut PlatformDevice) -> i32 {
    let mask_orig = MASK;
    MASK &= 0x1F7FFFFF;
    MASK &= !(1usize << 28);
    let _ = mask_orig;
    // gc->owner = THIS_MODULE; gc->label = DRV_NAME; gc->ngpio = 32;
    // gc->names = cs5535_gpio_names; gc->request = chip_gpio_request;
    // gc->get = chip_gpio_get; gc->set = chip_gpio_set;
    // gc->direction_input = chip_direction_input;
    // gc->direction_output = chip_direction_output;
    // platform_get_resource, devm_request_region, spin_lock_init, and
    // devm_gpiochip_add_data are external kernel operations.
    -5
}

// module_platform_driver(cs5535_gpio_driver)
// MODULE_AUTHOR("Andres Salomon <dilinger@queued.net>");
// MODULE_DESCRIPTION("AMD CS5535/CS5536 GPIO driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
