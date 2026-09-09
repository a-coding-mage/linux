// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO Driver for Loongson 1 SoC
 *
 * Copyright (C) 2015-2023 Keguang Zhang <keguang.zhang@gmail.com>
 */

// Kernel dependencies supplied by the surrounding Rust kernel environment.

const GPIO_CFG: usize = 0x0;
const GPIO_DIR: usize = 0x10;
const GPIO_DATA: usize = 0x20;
const GPIO_OUTPUT: usize = 0x30;

#[repr(C)]
struct GpioGenericChip {
    gc: GpioChip,
}

#[repr(C)]
struct GpioChip {
    owner: *mut Module,
    request: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    free: Option<unsafe extern "C" fn(*mut GpioChip, u32)>,
    ngpio: u32,
}

#[repr(C)]
struct GpioGenericChipConfig {
    dev: *mut Device,
    sz: u32,
    dat: *mut core::ffi::c_void,
    set: *mut core::ffi::c_void,
    dirin: *mut core::ffi::c_void,
}

#[repr(C)]
struct Ls1xGpioChip {
    chip: GpioGenericChip,
    reg_base: *mut u8,
}

#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice {
    dev: Device,
}
#[repr(C)]
struct Module;
#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}
#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    driver: Driver,
}
#[repr(C)]
struct Driver {
    name: *const core::ffi::c_char,
    of_match_table: *const OfDeviceId,
}

extern "C" {
    static THIS_MODULE: *mut Module;
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut Ls1xGpioChip;
    fn gpio_generic_chip_init(chip: *mut GpioGenericChip,
                               config: *mut GpioGenericChipConfig) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut Ls1xGpioChip;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut u8;
    fn devm_gpiochip_add_data(dev: *mut Device, gc: *mut GpioChip,
                              data: *mut Ls1xGpioChip) -> i32;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut Ls1xGpioChip);
    fn __raw_readl(addr: *mut u8) -> u32;
    fn __raw_writel(value: u32, addr: *mut u8);
    fn dev_info(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn gpio_generic_lock_irqsave(chip: *mut GpioGenericChip);
    fn gpio_generic_unlock_irqrestore(chip: *mut GpioGenericChip);
}

unsafe fn ls1x_gpio_request(gc: *mut GpioChip, offset: u32) -> i32 {
    let ls1x_gc = gpiochip_get_data(gc);
    gpio_generic_lock_irqsave(&mut (*ls1x_gc).chip);

    let reg = (*ls1x_gc).reg_base.add(GPIO_CFG);
    __raw_writel(__raw_readl(reg) | (1u32.wrapping_shl(offset)), reg);

    gpio_generic_unlock_irqrestore(&mut (*ls1x_gc).chip);
    0
}

unsafe fn ls1x_gpio_free(gc: *mut GpioChip, offset: u32) {
    let ls1x_gc = gpiochip_get_data(gc);
    gpio_generic_lock_irqsave(&mut (*ls1x_gc).chip);

    let reg = (*ls1x_gc).reg_base.add(GPIO_CFG);
    __raw_writel(__raw_readl(reg) & !(1u32.wrapping_shl(offset)), reg);

    gpio_generic_unlock_irqrestore(&mut (*ls1x_gc).chip);
}

unsafe fn ls1x_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let ls1x_gc = devm_kzalloc(dev, core::mem::size_of::<Ls1xGpioChip>(), 0);
    if ls1x_gc.is_null() {
        return -12;
    }

    (*ls1x_gc).reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (*ls1x_gc).reg_base.is_null() {
        return -1;
    }

    let config = GpioGenericChipConfig {
        dev,
        sz: 4,
        dat: (*ls1x_gc).reg_base.add(GPIO_DATA) as *mut core::ffi::c_void,
        set: (*ls1x_gc).reg_base.add(GPIO_OUTPUT) as *mut core::ffi::c_void,
        dirin: (*ls1x_gc).reg_base.add(GPIO_DIR) as *mut core::ffi::c_void,
    };

    let mut ret = gpio_generic_chip_init(&mut (*ls1x_gc).chip, &mut { config });
    if ret != 0 {
        dev_err(dev, b"failed to register GPIO controller\0".as_ptr() as *const _,);
        return ret;
    }

    (*ls1x_gc).chip.gc.owner = THIS_MODULE;
    (*ls1x_gc).chip.gc.request = Some(ls1x_gpio_request);
    (*ls1x_gc).chip.gc.free = Some(ls1x_gpio_free);
    // Clear ngpio to let gpiolib get the correct number by reading ngpios property.
    (*ls1x_gc).chip.gc.ngpio = 0;

    ret = devm_gpiochip_add_data(dev, &mut (*ls1x_gc).chip.gc, ls1x_gc);
    if ret != 0 {
        dev_err(dev, b"failed to register GPIO controller\0".as_ptr() as *const _,);
        return ret;
    }

    platform_set_drvdata(pdev, ls1x_gc);
    dev_info(dev, b"GPIO controller registered with %d pins\n\0".as_ptr() as *const _,
             (*ls1x_gc).chip.gc.ngpio as i32);
    0
}

static LS1X_GPIO_DT_IDS: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"loongson,ls1x-gpio\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null() },
];

static mut LS1X_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(ls1x_gpio_probe),
    driver: Driver {
        name: b"ls1x-gpio\0".as_ptr() as *const _,
        of_match_table: LS1X_GPIO_DT_IDS.as_ptr(),
    },
};

// module_platform_driver(ls1x_gpio_driver);
// MODULE_DEVICE_TABLE(of, ls1x_gpio_dt_ids);
// MODULE_AUTHOR("Keguang Zhang <keguang.zhang@gmail.com>");
// MODULE_DESCRIPTION("Loongson1 GPIO driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
