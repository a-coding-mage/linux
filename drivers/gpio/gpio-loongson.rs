// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Loongson-2F/3A/3B GPIO Support
 *
 *  Copyright (c) 2008 Richard Liu,  STMicroelectronics	 <richard.liu@st.com>
 *  Copyright (c) 2008-2010 Arnaud Patard <apatard@mandriva.com>
 *  Copyright (c) 2013 Hongbing Hu <huhb@lemote.com>
 *  Copyright (c) 2014 Huacai Chen <chenhc@lemote.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const STLS2F_N_GPIO: u32 = 4;
const STLS3A_N_GPIO: u32 = 16;

// CONFIG_CPU_LOONGSON64 selects STLS3A_N_GPIO; otherwise STLS2F_N_GPIO.
#[cfg(CONFIG_CPU_LOONGSON64)]
const LOONGSON_N_GPIO: u32 = STLS3A_N_GPIO;
#[cfg(not(CONFIG_CPU_LOONGSON64))]
const LOONGSON_N_GPIO: u32 = STLS2F_N_GPIO;

/*
 * Offset into the register where we read lines, we write them from offset 0.
 * This offset is the only thing that stand between us and using
 * GPIO_GENERIC.
 */
const LOONGSON_GPIO_IN_OFFSET: u32 = 16;

static mut gpio_lock: spinlock_t = DEFINE_SPINLOCK!();

unsafe fn loongson_gpio_get_value(chip: *mut gpio_chip, gpio: u32) -> i32 {
    let val: u32;

    spin_lock(&raw mut gpio_lock);
    val = LOONGSON_GPIODATA;
    spin_unlock(&raw mut gpio_lock);

    if (val & BIT(gpio + LOONGSON_GPIO_IN_OFFSET)) != 0 { 1 } else { 0 }
}

unsafe fn loongson_gpio_set_value(
    chip: *mut gpio_chip,
    gpio: u32,
    value: i32,
) -> i32 {
    let mut val: u32;

    spin_lock(&raw mut gpio_lock);
    val = LOONGSON_GPIODATA;
    if value != 0 {
        val |= BIT(gpio);
    } else {
        val &= !BIT(gpio);
    }
    LOONGSON_GPIODATA = val;
    spin_unlock(&raw mut gpio_lock);

    0
}

unsafe fn loongson_gpio_direction_input(chip: *mut gpio_chip, gpio: u32) -> i32 {
    let mut temp: u32;

    spin_lock(&raw mut gpio_lock);
    temp = LOONGSON_GPIOIE;
    temp |= BIT(gpio);
    LOONGSON_GPIOIE = temp;
    spin_unlock(&raw mut gpio_lock);

    0
}

unsafe fn loongson_gpio_direction_output(
    chip: *mut gpio_chip,
    gpio: u32,
    level: i32,
) -> i32 {
    let mut temp: u32;

    loongson_gpio_set_value(chip, gpio, level);
    spin_lock(&raw mut gpio_lock);
    temp = LOONGSON_GPIOIE;
    temp &= !BIT(gpio);
    LOONGSON_GPIOIE = temp;
    spin_unlock(&raw mut gpio_lock);

    0
}

unsafe fn loongson_gpio_probe(pdev: *mut platform_device) -> i32 {
    let gc: *mut gpio_chip;
    let dev: *mut device = &raw mut (*pdev).dev;

    gc = devm_kzalloc(dev, core::mem::size_of::<gpio_chip>(), GFP_KERNEL);
    if gc.is_null() {
        return -ENOMEM;
    }

    (*gc).label = c"loongson-gpio-chip".as_ptr();
    (*gc).base = 0;
    (*gc).ngpio = LOONGSON_N_GPIO;
    (*gc).get = Some(loongson_gpio_get_value);
    (*gc).set = Some(loongson_gpio_set_value);
    (*gc).direction_input = Some(loongson_gpio_direction_input);
    (*gc).direction_output = Some(loongson_gpio_direction_output);

    gpiochip_add_data(gc, core::ptr::null_mut())
}

static mut loongson_gpio_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"loongson-gpio".as_ptr(),
    },
    probe: Some(loongson_gpio_probe),
};

unsafe fn loongson_gpio_setup() -> i32 {
    let pdev: *mut platform_device;
    let ret: i32;

    ret = platform_driver_register(&raw mut loongson_gpio_driver);
    if ret != 0 {
        pr_err!("error registering loongson GPIO driver\n");
        return ret;
    }

    pdev = platform_device_register_simple(
        c"loongson-gpio".as_ptr(),
        -1,
        core::ptr::null_mut(),
        0,
    );
    PTR_ERR_OR_ZERO(pdev)
}

postcore_initcall!(loongson_gpio_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
