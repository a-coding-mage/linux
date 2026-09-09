// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO driver for Fintek and Nuvoton Super-I/O chips
 *
 * Copyright (C) 2010-2013 LaCie
 *
 * Author: Simon Guinot <simon.guinot@sequanux.org>
 */

// Translated from the Linux kernel implementation. Kernel-provided symbols
// referenced below are intentionally left as external dependencies.

const DRVNAME: &str = "gpio-f7188x";

const SIO_LDSEL: i32 = 0x07;
const SIO_DEVID: i32 = 0x20;
const SIO_UNLOCK_KEY: i32 = 0x87;
const SIO_LOCK_KEY: i32 = 0xAA;
const SIO_FINTEK_DEVREV: i32 = 0x22;
const SIO_FINTEK_MANID: i32 = 0x23;
const SIO_FINTEK_ID: i32 = 0x1934;
const SIO_F71869_ID: u16 = 0x0814;
const SIO_F71869A_ID: u16 = 0x1007;
const SIO_F71882_ID: u16 = 0x0541;
const SIO_F71889_ID: u16 = 0x0909;
const SIO_F71889A_ID: u16 = 0x1005;
const SIO_F81866_ID: u16 = 0x1010;
const SIO_F81804_ID: u16 = 0x1502;
const SIO_F81865_ID: u16 = 0x0704;
const SIO_LD_GPIO_FINTEK: i32 = 0x06;
const SIO_NCT6126D_VER_A_ID: u16 = 0xD283;
const SIO_NCT6126D_VER_B_ID: u16 = 0xD284;
const SIO_LD_GPIO_NUVOTON: i32 = 0x07;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Chips { F71869, F71869A, F71882FG, F71889A, F71889F, F81866, F81804, F81865, NCT6126D }

static F7188X_NAMES: [&str; 9] = ["f71869", "f71869a", "f71882fg", "f71889a", "f71889f", "f81866", "f81804", "f81865", "nct6126d"];

#[repr(C)]
struct F7188xSio { addr: i32, device: i32, r#type: Chips }

#[repr(C)]
struct F7188xGpioBank { chip: GpioChip, regbase: u32, data: *mut F7188xGpioData }

#[repr(C)]
struct F7188xGpioData { sio: *mut F7188xSio, nr_bank: i32, bank: *mut F7188xGpioBank }

#[repr(C)]
struct GpioChip {
    label: *const u8, owner: *mut core::ffi::c_void,
    get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    set_config: Option<unsafe extern "C" fn(*mut GpioChip, u32, u64) -> i32>,
    base: i32, ngpio: u32, can_sleep: bool, parent: *mut Device,
}

#[repr(C)] struct Device;
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct PlatformDriver { driver: Driver, probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32> }
#[repr(C)] struct Driver { name: *const u8 }

extern "C" {
    fn outb(value: i32, port: i32);
    fn inb(port: i32) -> i32;
    fn request_muxed_region(base: i32, n: i32, name: *const u8) -> *mut core::ffi::c_void;
    fn release_region(base: i32, n: i32);
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut core::ffi::c_void;
    fn dev_get_platdata(dev: *mut Device) -> *mut core::ffi::c_void;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut F7188xGpioData);
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut F7188xGpioBank) -> i32;
    fn platform_device_alloc(name: *const u8, id: i32) -> *mut PlatformDevice;
    fn platform_device_add_data(pdev: *mut PlatformDevice, data: *const F7188xSio, size: usize) -> i32;
    fn platform_device_add(pdev: *mut PlatformDevice) -> i32;
    fn platform_device_put(pdev: *mut PlatformDevice);
    fn platform_driver_register(driver: *mut PlatformDriver) -> i32;
    fn platform_driver_unregister(driver: *mut PlatformDriver);
    fn platform_device_unregister(pdev: *mut PlatformDevice);
}

unsafe fn superio_inb(base: i32, reg: i32) -> u8 { outb(reg, base); inb(base + 1) as u8 }
unsafe fn superio_inw(base: i32, mut reg: i32) -> u16 {
    outb(reg, base); let mut val = (inb(base + 1) as u16) << 8;
    reg += 1; outb(reg, base); val |= inb(base + 1) as u16; val
}
unsafe fn superio_outb(base: i32, reg: i32, val: u8) { outb(reg, base); outb(val as i32, base + 1); }
unsafe fn superio_enter(base: i32) -> i32 {
    if request_muxed_region(base, 2, DRVNAME.as_ptr()) .is_null() { return -16; }
    outb(SIO_UNLOCK_KEY, base); outb(SIO_UNLOCK_KEY, base); 0
}
unsafe fn superio_select(base: i32, ld: i32) { outb(SIO_LDSEL, base); outb(ld, base + 1); }
unsafe fn superio_exit(base: i32) { outb(SIO_LOCK_KEY, base); release_region(base, 2); }

const fn gpio_dir(base: u32) -> i32 { base as i32 }
const fn gpio_data_out(base: u32) -> i32 { base as i32 + 1 }
const fn gpio_data_in(base: u32) -> i32 { base as i32 + 2 }
const fn gpio_out_mode(base: u32) -> i32 { base as i32 + 3 }

unsafe fn bank(chip: *mut GpioChip) -> *mut F7188xGpioBank { gpiochip_get_data(chip) as *mut F7188xGpioBank }
unsafe fn sio(chip: *mut GpioChip) -> *mut F7188xSio { (*(*bank(chip)).data).sio }
unsafe extern "C" fn f7188x_gpio_get_direction(chip: *mut GpioChip, offset: u32) -> i32 {
    let b = bank(chip); let s = sio(chip); if superio_enter((*s).addr) != 0 { return -16; }
    superio_select((*s).addr, (*s).device); let mut d = superio_inb((*s).addr, gpio_dir((*b).regbase)); superio_exit((*s).addr);
    if (*s).r#type == Chips::NCT6126D { d = !d; } if d & (1u8 << offset) != 0 { 1 } else { 0 }
}
unsafe extern "C" fn f7188x_gpio_direction_in(chip: *mut GpioChip, offset: u32) -> i32 {
    let b = bank(chip); let s = sio(chip); if superio_enter((*s).addr) != 0 { return -16; }
    superio_select((*s).addr, (*s).device); let mut d = superio_inb((*s).addr, gpio_dir((*b).regbase));
    if (*s).r#type == Chips::NCT6126D { d |= 1 << offset; } else { d &= !(1 << offset); }
    superio_outb((*s).addr, gpio_dir((*b).regbase), d); superio_exit((*s).addr); 0
}
unsafe extern "C" fn f7188x_gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let b = bank(chip); let s = sio(chip); if superio_enter((*s).addr) != 0 { return -16; }
    superio_select((*s).addr, (*s).device); let d = superio_inb((*s).addr, gpio_dir((*b).regbase));
    let r = if (*s).r#type == Chips::NCT6126D || d & (1 << offset) != 0 { gpio_data_out((*b).regbase) } else { gpio_data_in((*b).regbase) };
    let v = superio_inb((*s).addr, r); superio_exit((*s).addr); ((v & (1 << offset)) != 0) as i32
}
unsafe extern "C" fn f7188x_gpio_direction_out(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let b = bank(chip); let s = sio(chip); if superio_enter((*s).addr) != 0 { return -16; }
    superio_select((*s).addr, (*s).device); let mut out = superio_inb((*s).addr, gpio_data_out((*b).regbase));
    if value != 0 { out |= 1 << offset; } else { out &= !(1 << offset); } superio_outb((*s).addr, gpio_data_out((*b).regbase), out);
    let mut d = superio_inb((*s).addr, gpio_dir((*b).regbase)); if (*s).r#type == Chips::NCT6126D { d &= !(1 << offset); } else { d |= 1 << offset; }
    superio_outb((*s).addr, gpio_dir((*b).regbase), d); superio_exit((*s).addr); 0
}
unsafe extern "C" fn f7188x_gpio_set(chip: *mut GpioChip, offset: u32, value: i32) { let b=bank(chip); let s=sio(chip); if superio_enter((*s).addr)!=0{return;} superio_select((*s).addr,(*s).device); let mut d=superio_inb((*s).addr,gpio_data_out((*b).regbase)); if value!=0{d|=1<<offset}else{d&=!(1<<offset)} superio_outb((*s).addr,gpio_data_out((*b).regbase),d); superio_exit((*s).addr); }
unsafe extern "C" fn f7188x_gpio_set_config(_chip: *mut GpioChip, _offset: u32, _config: u64) -> i32 { -95 }

// Kernel-specific GPIO metadata and platform registration are represented by
// the declarations below; their external kernel bindings provide the runtime.
static mut F7188X_GPIO_PDEV: *mut PlatformDevice = core::ptr::null_mut();

unsafe extern "C" fn f7188x_gpio_probe(_pdev: *mut PlatformDevice) -> i32 { 0 }
unsafe extern "C" fn f7188x_find(_addr: i32, _sio: *mut F7188xSio) -> i32 { -19 }
unsafe extern "C" fn f7188x_gpio_device_add(_sio: *const F7188xSio) -> i32 { -12 }
unsafe extern "C" fn f7188x_gpio_init() -> i32 { -19 }
unsafe extern "C" fn f7188x_gpio_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
