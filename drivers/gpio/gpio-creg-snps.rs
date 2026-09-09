// SPDX-License-Identifier: GPL-2.0+
//
// Synopsys CREG (Control REGisters) GPIO driver
//
// Copyright (C) 2018 Synopsys
// Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>

// Dependencies supplied by the Linux kernel headers are intentionally left as
// external Rust items.

const MAX_GPIO: usize = 32;

#[repr(C)]
struct creg_layout {
    ngpio: u8,
    shift: [u8; MAX_GPIO],
    on: [u8; MAX_GPIO],
    off: [u8; MAX_GPIO],
    bit_per_gpio: [u8; MAX_GPIO],
}

#[repr(C)]
struct creg_gpio {
    gc: gpio_chip,
    regs: *mut core::ffi::c_void,
    lock: spinlock_t,
    layout: *const creg_layout,
}

#[repr(C)]
struct gpio_chip {
    parent: *mut device,
    label: *const core::ffi::c_char,
    base: i32,
    ngpio: u32,
    set: Option<unsafe fn(*mut gpio_chip, u32, i32) -> i32>,
    direction_output: Option<unsafe fn(*mut gpio_chip, u32, i32) -> i32>,
}

#[repr(C)]
struct spinlock_t {
    _opaque: [u8; 0],
}

#[repr(C)]
struct device {
    _opaque: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
    _opaque: [u8; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

#[repr(C)]
struct platform_driver {
    _opaque: [u8; 0],
}

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn device_get_match_data(dev: *mut device) -> *const creg_layout;
    fn of_property_read_u32(
        node: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
        value: *mut u32,
    ) -> i32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn devm_gpiochip_add_data(
        dev: *mut device,
        gc: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn dev_err(dev: *mut device, format: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, format: *const core::ffi::c_char, ...);
}

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

#[inline]
const fn genmask(high: u32, low: u32) -> u32 {
    if high == 31 && low == 0 {
        u32::MAX
    } else {
        (((1u64 << (high - low + 1)) - 1) << low) as u32
    }
}

unsafe fn creg_gpio_set(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let hcg = gpiochip_get_data(gc) as *mut creg_gpio;
    let layout = (*hcg).layout;
    let value: u32 = if val != 0 {
        (*layout).on[offset as usize] as u32
    } else {
        (*layout).off[offset as usize] as u32
    };
    let mut reg_shift = (*layout).shift[offset as usize] as u32;
    let mut i = 0u32;
    while i < offset {
        reg_shift += (*layout).bit_per_gpio[i as usize] as u32
            + (*layout).shift[i as usize] as u32;
        i += 1;
    }

    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*hcg).lock, &mut flags);
    let mut reg = readl((*hcg).regs);
    reg &= !(genmask((*layout).bit_per_gpio[i as usize] as u32 - 1, 0) << reg_shift);
    reg |= value << reg_shift;
    writel(reg, (*hcg).regs);
    spin_unlock_irqrestore(&mut (*hcg).lock, flags);

    0
}

unsafe fn creg_gpio_dir_out(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    creg_gpio_set(gc, offset, val)
}

unsafe fn creg_gpio_validate_pg(_dev: *mut device, hcg: *mut creg_gpio, i: usize) -> i32 {
    let layout = (*hcg).layout;
    if (*layout).bit_per_gpio[i] < 1 || (*layout).bit_per_gpio[i] > 8 {
        return -EINVAL;
    }
    // Check that on value fits its placeholder
    if (genmask(31, (*layout).bit_per_gpio[i] as u32) & (*layout).on[i] as u32) != 0 {
        return -EINVAL;
    }
    // Check that off value fits its placeholder
    if (genmask(31, (*layout).bit_per_gpio[i] as u32) & (*layout).off[i] as u32) != 0 {
        return -EINVAL;
    }
    if (*layout).on[i] == (*layout).off[i] {
        return -EINVAL;
    }
    0
}

unsafe fn creg_gpio_validate(dev: *mut device, hcg: *mut creg_gpio, ngpios: u32) -> i32 {
    let mut reg_len = 0u32;
    let layout = (*hcg).layout;
    if (*layout).ngpio < 1 || (*layout).ngpio > MAX_GPIO as u8 {
        return -EINVAL;
    }
    if ngpios < 1 || ngpios > (*layout).ngpio as u32 {
        dev_err(dev, b"ngpios must be in [1:%u]\0".as_ptr() as *const _, (*layout).ngpio as u32);
        return -EINVAL;
    }
    for i in 0..(*layout).ngpio as usize {
        if creg_gpio_validate_pg(dev, hcg, i) != 0 {
            return -EINVAL;
        }
        reg_len += (*layout).shift[i] as u32 + (*layout).bit_per_gpio[i] as u32;
    }
    // Check that we fit in 32 bit register
    if reg_len > 32 {
        return -EINVAL;
    }
    0
}

static hsdk_cs_ctl: creg_layout = creg_layout {
    ngpio: 10,
    shift: [0; MAX_GPIO],
    off: [2; MAX_GPIO],
    on: [3; MAX_GPIO],
    bit_per_gpio: [2; MAX_GPIO],
};

static axs10x_flsh_cs_ctl: creg_layout = creg_layout {
    ngpio: 1,
    shift: [0; MAX_GPIO],
    off: [1; MAX_GPIO],
    on: [3; MAX_GPIO],
    bit_per_gpio: [2; MAX_GPIO],
};

static creg_gpio_ids: [of_device_id; 3] = [
    of_device_id { compatible: b"snps,creg-gpio-axs10x\0".as_ptr() as *const _, data: &axs10x_flsh_cs_ctl as *const _ as *const _ },
    of_device_id { compatible: b"snps,creg-gpio-hsdk\0".as_ptr() as *const _, data: &hsdk_cs_ctl as *const _ as *const _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() }, // sentinel
];

// The platform-driver probe and registration use kernel-provided structures
// and macros; their declaration-only interfaces are preserved here.
extern "C" {
}

unsafe fn creg_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let hcg = devm_kzalloc(dev, core::mem::size_of::<creg_gpio>(), 0) as *mut creg_gpio;
    if hcg.is_null() {
        return -ENOMEM;
    }
    (*hcg).regs = devm_platform_ioremap_resource(pdev, 0);
    if (*hcg).regs.is_null() {
        return -EINVAL;
    }
    (*hcg).layout = device_get_match_data(dev);
    if (*hcg).layout.is_null() {
        return -EINVAL;
    }
    let mut ngpios = 0u32;
    let ret = of_property_read_u32(core::ptr::null_mut(), b"ngpios\0".as_ptr() as *const _, &mut ngpios);
    if ret != 0 {
        return ret;
    }
    let ret = creg_gpio_validate(dev, hcg, ngpios);
    if ret != 0 {
        return ret;
    }
    spin_lock_init(&mut (*hcg).lock);
    (*hcg).gc.parent = dev;
    (*hcg).gc.label = dev_name(dev);
    (*hcg).gc.base = -1;
    (*hcg).gc.ngpio = ngpios;
    (*hcg).gc.set = Some(creg_gpio_set);
    (*hcg).gc.direction_output = Some(creg_gpio_dir_out);
    let ret = devm_gpiochip_add_data(dev, &mut (*hcg).gc, hcg as *mut _ as *mut _);
    if ret != 0 {
        return ret;
    }
    dev_info(dev, b"GPIO controller with %d gpios probed\n\0".as_ptr() as *const _, ngpios as i32);
    0
}

static mut creg_gpio_snps_driver: platform_driver = platform_driver { _opaque: [] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
