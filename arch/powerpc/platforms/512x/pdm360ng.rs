// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2010 DENX Software Engineering
 *
 * Anatolij Gustschin, <agust@denx.de>
 *
 * PDM360NG board setup
 */

use core::ffi::c_void;

// The following declarations are supplied by the kernel and architecture
// headers included by the original C source.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
    pub platform_data: *mut c_void,
}
#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32>,
}
#[repr(C)]
pub struct spi_bus_type_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machine_desc {
    pub name: *const u8,
    pub compatible: *const u8,
    pub probe: Option<unsafe extern "C" fn() -> i32>,
    pub setup_arch: Option<unsafe extern "C" fn()>,
    pub init: Option<unsafe extern "C" fn()>,
    pub init_irq: Option<unsafe extern "C" fn()>,
    pub get_irq: Option<unsafe extern "C" fn() -> i32>,
    pub restart: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct ads7846_platform_data {
    pub model: i32,
    pub get_pendown_state: Option<unsafe extern "C" fn() -> i32>,
    pub irq_flags: u32,
}

extern "C" {
    static mut spi_bus_type: spi_bus_type_t;
    fn in_be32(addr: *mut u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn setbits32(addr: *mut u32, value: u32);
    fn of_find_compatible_node(from: *mut device_node, typ: *const u8, compatible: *const u8) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn of_device_is_compatible(np: *mut device_node, compatible: *const u8) -> bool;
    fn bus_register_notifier(bus: *mut spi_bus_type_t, nb: *mut notifier_block) -> i32;
    fn mpc512x_init();
    fn mpc512x_init_early();
    fn mpc512x_setup_arch();
    fn mpc512x_init_IRQ();
    fn ipic_get_irq() -> i32;
    fn mpc512x_restart();
}

const ENODEV: i32 = 19;
const IRQF_TRIGGER_LOW: u32 = 0x0000_0008;
const BUS_NOTIFY_ADD_DEVICE: usize = 0x0000_0003;
const NOTIFY_OK: i32 = 0x0000_0001;
const NOTIFY_DONE: i32 = 0x0000_0000;

// Configuration condition preserved from the original source:
// CONFIG_TOUCHSCREEN_ADS7846 || CONFIG_TOUCHSCREEN_ADS7846_MODULE
static mut pdm360ng_gpio_base: *mut u8 = core::ptr::null_mut();

unsafe extern "C" fn pdm360ng_get_pendown_state() -> i32 {
    let mut reg: u32;

    reg = in_be32(pdm360ng_gpio_base.add(0xc) as *mut u32);
    if reg & 0x40 != 0 {
        setbits32(pdm360ng_gpio_base.add(0xc) as *mut u32, 0x40);
    }

    reg = in_be32(pdm360ng_gpio_base.add(0x8) as *mut u32);

    /* return 1 if pen is down */
    if reg & 0x40 == 0 { 1 } else { 0 }
}

static mut pdm360ng_ads7846_pdata: ads7846_platform_data = ads7846_platform_data {
    model: 7845,
    get_pendown_state: Some(pdm360ng_get_pendown_state),
    irq_flags: IRQF_TRIGGER_LOW,
};

unsafe extern "C" fn pdm360ng_penirq_init() -> i32 {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"fsl,mpc5121-gpio\0".as_ptr());
    if np.is_null() {
        return -ENODEV;
    }

    pdm360ng_gpio_base = of_iomap(np, 0) as *mut u8;
    of_node_put(np);
    if pdm360ng_gpio_base.is_null() {
        return -ENODEV;
    }
    out_be32(pdm360ng_gpio_base.add(0xc) as *mut u32, 0xffff_ffff);
    setbits32(pdm360ng_gpio_base.add(0x18) as *mut u32, 0x2000);
    setbits32(pdm360ng_gpio_base.add(0x10) as *mut u32, 0x40);

    0
}

unsafe extern "C" fn pdm360ng_touchscreen_notifier_call(
    _nb: *mut notifier_block,
    event: usize,
    __dev: *mut c_void,
) -> i32 {
    let dev = __dev as *mut device;

    if event == BUS_NOTIFY_ADD_DEVICE
        && of_device_is_compatible((*dev).of_node, b"ti,ads7846\0".as_ptr())
    {
        (*dev).platform_data = core::ptr::addr_of_mut!(pdm360ng_ads7846_pdata) as *mut c_void;
        return NOTIFY_OK;
    }
    NOTIFY_DONE
}

static mut pdm360ng_touchscreen_nb: notifier_block = notifier_block {
    notifier_call: Some(pdm360ng_touchscreen_notifier_call),
};

unsafe extern "C" fn pdm360ng_touchscreen_init() {
    if pdm360ng_penirq_init() != 0 {
        return;
    }

    bus_register_notifier(&mut spi_bus_type, core::ptr::addr_of_mut!(pdm360ng_touchscreen_nb));
}

unsafe extern "C" fn pdm360ng_init() {
    mpc512x_init();
    pdm360ng_touchscreen_init();
}

unsafe extern "C" fn pdm360ng_probe() -> i32 {
    mpc512x_init_early();

    1
}

#[no_mangle]
pub static mut pdm360ng: machine_desc = machine_desc {
    name: b"PDM360NG\0".as_ptr(),
    compatible: b"ifm,pdm360ng\0".as_ptr(),
    probe: Some(pdm360ng_probe),
    setup_arch: Some(mpc512x_setup_arch),
    init: Some(pdm360ng_init),
    init_irq: Some(mpc512x_init_IRQ),
    get_irq: Some(ipic_get_irq),
    restart: Some(mpc512x_restart),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
