// SPDX-License-Identifier: GPL-2.0
/*
 * GPIO latch driver
 *
 *  Copyright (C) 2022 Sascha Hauer <s.hauer@pengutronix.de>
 *
 * This driver implements a GPIO (or better GPO as there is no input)
 * multiplexer based on latches like this:
 *
 * CLK0 ----------------------.        ,--------.
 * CLK1 -------------------.  `--------|>    #0 |
 *                         |           |        |
 * OUT0 ----------------+--|-----------|D0    Q0|-----|<
 * OUT1 --------------+-|--|-----------|D1    Q1|-----|<
 * OUT2 ------------+-|-|--|-----------|D2    Q2|-----|<
 * OUT3 ----------+-|-|-|--|-----------|D3    Q3|-----|<
 * OUT4 --------+-|-|-|-|--|-----------|D4    Q4|-----|<
 * OUT5 ------+-|-|-|-|-|--|-----------|D5    Q5|-----|<
 * OUT6 ----+-|-|-|-|-|-|--|-----------|D6    Q6|-----|<
 * OUT7 --+-|-|-|-|-|-|-|--|-----------|D7    Q7|-----|<
 *        | | | | | | | |  |           `--------'
 *        | | | | | | | |  |
 *        | | | | | | | |  |           ,--------.
 *        | | | | | | | |  `-----------|>    #1 |
 *        | | | | | | | |              |        |
 *        | | | | | | | `--------------|D0    Q0|-----|<
 *        | | | | | | `----------------|D1    Q1|-----|<
 *        | | | | | `------------------|D2    Q2|-----|<
 *        | | | | `--------------------|D3    Q3|-----|<
 *        | | | `----------------------|D4    Q4|-----|<
 *        | | `------------------------|D5    Q5|-----|<
 *        | `--------------------------|D6    Q6|-----|<
 *        `----------------------------|D7    Q7|-----|<
 *                                     `--------'
 *
 * The above is just an example. The actual number of number of latches and
 * the number of inputs per latch is derived from the number of GPIOs given
 * in the corresponding device tree properties.
 */

// Linux dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct gpio_latch_priv {
    pub gc: gpio_chip,
    pub clk_gpios: *mut gpio_descs,
    pub latched_gpios: *mut gpio_descs,
    pub n_latched_gpios: i32,
    pub setup_duration_ns: u32,
    pub clock_duration_ns: u32,
    pub shadow: *mut c_ulong,
    /*
     * Depending on whether any of the underlying GPIOs may sleep we either
     * use a mutex or a spinlock to protect our shadow map.
     */
    pub lock: gpio_latch_priv_lock,
}

#[repr(C)]
pub union gpio_latch_priv_lock {
    pub mutex: mutex,
    pub spinlock: spinlock_t,
}

unsafe fn gpio_latch_get_direction(_gc: *mut gpio_chip, _offset: u32) -> i32 {
    GPIO_LINE_DIRECTION_OUT
}

unsafe fn gpio_latch_set_unlocked(
    priv_: *mut gpio_latch_priv,
    set: unsafe extern "C" fn(*mut gpio_desc, i32) -> i32,
    offset: u32,
    val: bool,
) -> i32 {
    let latch = offset / (*priv_).n_latched_gpios as u32;
    let mut ret: i32;

    assign_bit(offset, (*priv_).shadow, val);

    let mut i = 0;
    while i < (*priv_).n_latched_gpios {
        ret = set(
            (*(*priv_).latched_gpios).desc.add(i as usize),
            test_bit(
                latch * (*priv_).n_latched_gpios as u32 + i as u32,
                (*priv_).shadow,
            ) as i32,
        );
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    ndelay((*priv_).setup_duration_ns);
    set((*(*priv_).clk_gpios).desc.add(latch as usize), 1);
    ndelay((*priv_).clock_duration_ns);
    set((*(*priv_).clk_gpios).desc.add(latch as usize), 0);

    0
}

unsafe extern "C" fn gpio_latch_set(gc: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut gpio_latch_priv;

    let _guard = guard_spinlock_irqsave(&mut (*priv_).lock.spinlock);

    gpio_latch_set_unlocked(priv_, gpiod_set_value, offset, val != 0)
}

unsafe extern "C" fn gpio_latch_set_can_sleep(
    gc: *mut gpio_chip,
    offset: u32,
    val: i32,
) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut gpio_latch_priv;

    let _guard = guard_mutex(&mut (*priv_).lock.mutex);

    gpio_latch_set_unlocked(priv_, gpiod_set_value_cansleep, offset, val != 0)
}

unsafe fn gpio_latch_can_sleep(priv_: *mut gpio_latch_priv, n_latches: u32) -> bool {
    let mut i = 0;
    while i < n_latches {
        if gpiod_cansleep((*(*priv_).clk_gpios).desc.add(i as usize)) {
            return true;
        }
        i += 1;
    }

    i = 0;
    while i < (*priv_).n_latched_gpios as u32 {
        if gpiod_cansleep((*(*priv_).latched_gpios).desc.add(i as usize)) {
            return true;
        }
        i += 1;
    }

    false
}

/*
 * Some value which is still acceptable to delay in atomic context.
 * If we need to go higher we might have to switch to usleep_range(),
 * but that cannot ne used in atomic context and the driver would have
 * to be adjusted to support that.
 */
pub const DURATION_NS_MAX: u32 = 5000;

unsafe extern "C" fn gpio_latch_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut priv_: *mut gpio_latch_priv;
    let n_latches: u32;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<gpio_latch_priv>(), GFP_KERNEL)
        as *mut gpio_latch_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).clk_gpios = devm_gpiod_get_array(dev, b"clk\0".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*priv_).clk_gpios) {
        return PTR_ERR((*priv_).clk_gpios);
    }

    (*priv_).latched_gpios = devm_gpiod_get_array(dev, b"latched\0".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*priv_).latched_gpios) {
        return PTR_ERR((*priv_).latched_gpios);
    }

    n_latches = (*(*priv_).clk_gpios).ndescs;
    (*priv_).n_latched_gpios = (*(*priv_).latched_gpios).ndescs as i32;

    (*priv_).shadow = devm_bitmap_zalloc(
        dev,
        n_latches * (*priv_).n_latched_gpios as u32,
        GFP_KERNEL,
    );
    if (*priv_).shadow.is_null() {
        return -ENOMEM;
    }

    if gpio_latch_can_sleep(priv_, n_latches) {
        (*priv_).gc.can_sleep = true;
        (*priv_).gc.set = Some(gpio_latch_set_can_sleep);
        mutex_init(&mut (*priv_).lock.mutex);
    } else {
        (*priv_).gc.can_sleep = false;
        (*priv_).gc.set = Some(gpio_latch_set);
        spin_lock_init(&mut (*priv_).lock.spinlock);
    }

    device_property_read_u32(dev, b"setup-duration-ns\0".as_ptr(), &mut (*priv_).setup_duration_ns);
    if (*priv_).setup_duration_ns > DURATION_NS_MAX {
        dev_warn(dev, b"setup-duration-ns too high, limit to %d\n\0".as_ptr(), DURATION_NS_MAX);
        (*priv_).setup_duration_ns = DURATION_NS_MAX;
    }

    device_property_read_u32(dev, b"clock-duration-ns\0".as_ptr(), &mut (*priv_).clock_duration_ns);
    if (*priv_).clock_duration_ns > DURATION_NS_MAX {
        dev_warn(dev, b"clock-duration-ns too high, limit to %d\n\0".as_ptr(), DURATION_NS_MAX);
        (*priv_).clock_duration_ns = DURATION_NS_MAX;
    }

    (*priv_).gc.get_direction = Some(gpio_latch_get_direction);
    (*priv_).gc.ngpio = n_latches * (*priv_).n_latched_gpios as u32;
    (*priv_).gc.owner = THIS_MODULE;
    (*priv_).gc.base = -1;
    (*priv_).gc.parent = dev;

    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);

    devm_gpiochip_add_data(dev, &mut (*priv_).gc, priv_ as *mut core::ffi::c_void)
}

static mut gpio_latch_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"gpio-latch\0".as_ptr(),
    },
    of_device_id { compatible: core::ptr::null() },
];

MODULE_DEVICE_TABLE!(of, gpio_latch_ids);

static mut gpio_latch_driver: platform_driver = platform_driver {
    driver: driver {
        name: b"gpio-latch\0".as_ptr(),
        of_match_table: gpio_latch_ids.as_ptr(),
    },
    probe: Some(gpio_latch_probe),
};

module_platform_driver!(gpio_latch_driver);

MODULE_LICENSE!("GPL v2");
MODULE_AUTHOR!("Sascha Hauer <s.hauer@pengutronix.de>");
MODULE_DESCRIPTION!("GPIO latch driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
