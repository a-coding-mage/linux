// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and ath25 platform headers are
// intentionally left as external Rust declarations/constants.

use core::ffi::c_void;

#[repr(C)]
pub struct ar231x_board_config {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub name: *const u8,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const u8,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub dev: device,
}

#[repr(C)]
pub struct uart_port {
    pub flags: u32,
    pub iotype: u32,
    pub irq: i32,
    pub regshift: u32,
    pub mapbase: u32,
    pub uartclk: u32,
}

extern "C" {
    pub fn is_ar5312() -> bool;
    pub fn ar5312_init_devices();
    pub fn ar2315_init_devices();
    pub fn ar5312_arch_init();
    pub fn ar2315_arch_init();
    pub fn early_serial_setup(port: *mut uart_port) -> i32;
    pub fn platform_device_register(dev: *mut platform_device) -> i32;
    pub fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
}

pub const IORESOURCE_MEM: u64 = 0x0000_0200;
pub const IORESOURCE_IRQ: u64 = 0x0000_0400;
pub const UPF_BOOT_AUTOCONF: u32 = 1 << 0;
pub const UPF_SKIP_TEST: u32 = 1 << 1;
pub const UPF_IOREMAP: u32 = 1 << 2;
pub const UPIO_MEM32: u32 = 3;

pub const ATH25_SOC_AR5312: usize = 0;
pub const ATH25_SOC_AR2312: usize = 1;
pub const ATH25_SOC_AR2313: usize = 2;
pub const ATH25_SOC_AR2315: usize = 3;
pub const ATH25_SOC_AR2316: usize = 4;
pub const ATH25_SOC_AR2317: usize = 5;
pub const ATH25_SOC_AR2318: usize = 6;
pub const ATH25_SOC_UNKNOWN: usize = 7;

pub static mut ath25_board: ar231x_board_config = ar231x_board_config { _opaque: [] };
pub static mut ath25_soc: usize = ATH25_SOC_UNKNOWN;

static mut ath25_wmac0_res: [resource; 2] = [
    resource { name: b"wmac0_membase\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_MEM },
    resource { name: b"wmac0_irq\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];

static mut ath25_wmac1_res: [resource; 2] = [
    resource { name: b"wmac1_membase\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_MEM },
    resource { name: b"wmac1_irq\0".as_ptr(), start: 0, end: 0, flags: IORESOURCE_IRQ },
];

static mut ath25_wmac: [platform_device; 2] = [
    platform_device { id: 0, name: b"ar231x-wmac\0".as_ptr(), resource: unsafe { ath25_wmac0_res.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut ath25_board as *mut _ as *mut c_void } } },
    platform_device { id: 1, name: b"ar231x-wmac\0".as_ptr(), resource: unsafe { ath25_wmac1_res.as_mut_ptr() }, num_resources: 2, dev: device { platform_data: unsafe { &mut ath25_board as *mut _ as *mut c_void } } },
];

static soc_type_strings: [Option<&'static [u8]>; 8] = [
    Some(b"Atheros AR5312\0"), Some(b"Atheros AR2312\0"),
    Some(b"Atheros AR2313\0"), Some(b"Atheros AR2315\0"),
    Some(b"Atheros AR2316\0"), Some(b"Atheros AR2317\0"),
    Some(b"Atheros AR2318\0"), Some(b"Atheros (unknown)\0"),
];

pub unsafe fn get_system_type() -> *const u8 {
    if ath25_soc >= soc_type_strings.len() || soc_type_strings[ath25_soc].is_none() {
        return soc_type_strings[ATH25_SOC_UNKNOWN].unwrap().as_ptr();
    }
    soc_type_strings[ath25_soc].unwrap().as_ptr()
}

pub unsafe fn ath25_serial_setup(mapbase: u32, irq: i32, uartclk: u32) {
    // CONFIG_SERIAL_8250_CONSOLE controls this block in the C source.
    let mut s: uart_port = core::mem::zeroed();
    s.flags = UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_IOREMAP;
    s.iotype = UPIO_MEM32;
    s.irq = irq;
    s.regshift = 2;
    s.mapbase = mapbase;
    s.uartclk = uartclk;
    early_serial_setup(&mut s);
}

pub unsafe fn ath25_add_wmac(nr: i32, base: u32, irq: i32) -> i32 {
    let dev = &mut ath25_wmac[nr as usize];
    dev.dev.platform_data = &mut ath25_board as *mut _ as *mut c_void;
    let res = &mut *dev.resource;
    res.start = base as usize;
    res.end = base.wrapping_add(0x10000).wrapping_sub(1) as usize;
    let res = res.add(1);
    res.start = irq as usize;
    res.end = irq as usize;
    platform_device_register(dev)
}

unsafe fn ath25_register_devices() -> i32 {
    if is_ar5312() { ar5312_init_devices(); } else { ar2315_init_devices(); }
    0
}

unsafe fn ath25_arch_init() -> i32 {
    if is_ar5312() { ar5312_arch_init(); } else { ar2315_arch_init(); }
    0
}

// C registration annotations: device_initcall(ath25_register_devices);
// arch_initcall(ath25_arch_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
