// SPDX-License-Identifier: GPL-2.0+

/*
 * GPIO driver for the AMD G series FCH (eg. GX-412TC)
 *
 * Copyright (C) 2018 metux IT consult
 * Author: Enrico Weigelt, metux IT consult <info@metux.net>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const AMD_FCH_MMIO_BASE: usize = 0xFED8_0000;
const AMD_FCH_GPIO_BANK0_BASE: usize = 0x1500;
const AMD_FCH_GPIO_SIZE: usize = 0x0300;

const AMD_FCH_GPIO_FLAG_DIRECTION: u32 = 1u32 << 23;
const AMD_FCH_GPIO_FLAG_WRITE: u32 = 1u32 << 22;
const AMD_FCH_GPIO_FLAG_READ: u32 = 1u32 << 16;

static AMD_FCH_GPIO_IORES: Resource = define_res_mem_named(
    AMD_FCH_MMIO_BASE + AMD_FCH_GPIO_BANK0_BASE,
    AMD_FCH_GPIO_SIZE,
    "amd-fch-gpio-iomem",
);

#[repr(C)]
struct AmdFchGpioPriv {
    gc: GpioChip,
    base: *mut core::ffi::c_void,
    pdata: *mut AmdFchGpioPdata,
    lock: Spinlock,
}

unsafe fn amd_fch_gpio_addr(priv_: *mut AmdFchGpioPriv, gpio: u32) -> *mut core::ffi::c_void {
    (*priv_).base.add((*(*priv_).pdata).gpio_reg[gpio as usize] as usize * core::mem::size_of::<u32>())
}

unsafe extern "C" fn amd_fch_gpio_direction_input(gc: *mut GpioChip, offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc);
    let ptr = amd_fch_gpio_addr(priv_, offset);

    let _guard = guard_spinlock_irqsave(&mut (*priv_).lock);
    writel_relaxed(readl_relaxed(ptr) & !AMD_FCH_GPIO_FLAG_DIRECTION, ptr);
    0
}

unsafe extern "C" fn amd_fch_gpio_direction_output(
    gc: *mut GpioChip,
    gpio: u32,
    value: i32,
) -> i32 {
    let priv_ = gpiochip_get_data(gc);
    let ptr = amd_fch_gpio_addr(priv_, gpio);
    let mut val: u32;

    let _guard = guard_spinlock_irqsave(&mut (*priv_).lock);
    val = readl_relaxed(ptr);
    if value != 0 {
        val |= AMD_FCH_GPIO_FLAG_WRITE;
    } else {
        val &= !AMD_FCH_GPIO_FLAG_WRITE;
    }
    writel_relaxed(val | AMD_FCH_GPIO_FLAG_DIRECTION, ptr);
    0
}

unsafe extern "C" fn amd_fch_gpio_get_direction(gc: *mut GpioChip, gpio: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc);
    let ptr = amd_fch_gpio_addr(priv_, gpio);
    let _guard = guard_spinlock_irqsave(&mut (*priv_).lock);
    let ret = readl_relaxed(ptr) & AMD_FCH_GPIO_FLAG_DIRECTION;
    if ret != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn amd_fch_gpio_set(gc: *mut GpioChip, gpio: u32, value: i32) -> i32 {
    let priv_ = gpiochip_get_data(gc);
    let ptr = amd_fch_gpio_addr(priv_, gpio);
    let _guard = guard_spinlock_irqsave(&mut (*priv_).lock);
    let mut mask = readl_relaxed(ptr);
    if value != 0 {
        mask |= AMD_FCH_GPIO_FLAG_WRITE;
    } else {
        mask &= !AMD_FCH_GPIO_FLAG_WRITE;
    }
    writel_relaxed(mask, ptr);
    0
}

unsafe extern "C" fn amd_fch_gpio_get(gc: *mut GpioChip, offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc);
    let ptr = amd_fch_gpio_addr(priv_, offset);
    let _guard = guard_spinlock_irqsave(&mut (*priv_).lock);
    let val = readl_relaxed(ptr);
    ((val & AMD_FCH_GPIO_FLAG_READ) >> 16) as i32
}

unsafe extern "C" fn amd_fch_gpio_request(_chip: *mut GpioChip, _gpio_pin: u32) -> i32 { 0 }

unsafe extern "C" fn amd_fch_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let mut priv_: *mut AmdFchGpioPriv;
    let pdata: *mut AmdFchGpioPdata;

    pdata = dev_get_platdata((*pdev).dev());
    if pdata.is_null() {
        dev_err((*pdev).dev(), "no platform_data\n");
        return -ENOENT;
    }

    priv_ = devm_kzalloc((*pdev).dev(), core::mem::size_of::<AmdFchGpioPriv>(), GFP_KERNEL);
    if priv_.is_null() { return -ENOMEM; }

    (*priv_).pdata = pdata;
    (*priv_).gc.owner = THIS_MODULE;
    (*priv_).gc.parent = (*pdev).dev();
    (*priv_).gc.label = dev_name((*pdev).dev());
    (*priv_).gc.ngpio = (*pdata).gpio_num;
    (*priv_).gc.names = (*pdata).gpio_names;
    (*priv_).gc.base = -1;
    (*priv_).gc.request = Some(amd_fch_gpio_request);
    (*priv_).gc.direction_input = Some(amd_fch_gpio_direction_input);
    (*priv_).gc.direction_output = Some(amd_fch_gpio_direction_output);
    (*priv_).gc.get_direction = Some(amd_fch_gpio_get_direction);
    (*priv_).gc.get = Some(amd_fch_gpio_get);
    (*priv_).gc.set = Some(amd_fch_gpio_set);

    spin_lock_init(&mut (*priv_).lock);
    (*priv_).base = devm_ioremap_resource((*pdev).dev(), &AMD_FCH_GPIO_IORES);
    if is_err((*priv_).base) { return ptr_err((*priv_).base); }
    platform_set_drvdata(pdev, priv_);
    devm_gpiochip_add_data((*pdev).dev(), &mut (*priv_).gc, priv_)
}

static mut AMD_FCH_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: AMD_FCH_GPIO_DRIVER_NAME },
    probe: Some(amd_fch_gpio_probe),
};

module_platform_driver!(AMD_FCH_GPIO_DRIVER);

// MODULE_AUTHOR("Enrico Weigelt, metux IT consult <info@metux.net>");
// MODULE_DESCRIPTION("AMD G-series FCH GPIO driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" AMD_FCH_GPIO_DRIVER_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
