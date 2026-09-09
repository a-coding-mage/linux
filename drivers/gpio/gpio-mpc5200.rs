// SPDX-License-Identifier: GPL-2.0-only
/*
 * MPC52xx gpio driver
 *
 * Copyright (c) 2008 Sascha Hauer <s.hauer@pengutronix.de>, Pengutronix
 */

// Dependencies supplied by the surrounding kernel translation.

static mut GPIO_LOCK: spinlock_t = spinlock_t::new();

#[repr(C)]
struct mpc52xx_gpiochip {
    gc: gpio_chip,
    regs: *mut core::ffi::c_void,
    shadow_dvo: c_uint,
    shadow_gpioe: c_uint,
    shadow_ddr: c_uint,
}

/*
 * GPIO LIB API implementation for wakeup GPIOs.
 *
 * There's a maximum of 8 wakeup GPIOs. Which of these are available
 * for use depends on your board setup.
 *
 * 0 -> GPIO_WKUP_7
 * 1 -> GPIO_WKUP_6
 * 2 -> PSC6_1
 * 3 -> PSC6_0
 * 4 -> ETH_17
 * 5 -> PSC3_9
 * 6 -> PSC2_4
 * 7 -> PSC1_4
 *
 */
unsafe extern "C" fn mpc52xx_wkup_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio_wkup;
    let ret = (in_8(&(*regs).wkup_ival) >> (7 - gpio)) & 1;

    pr_debug!("{}: gpio: {} ret: {}\n", "mpc52xx_wkup_gpio_get", gpio, ret);
    ret as c_int
}

unsafe fn __mpc52xx_wkup_gpio_set(gc: *mut gpio_chip, gpio: c_uint, val: c_int) {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio_wkup;

    if val != 0 {
        (*chip).shadow_dvo |= 1 << (7 - gpio);
    } else {
        (*chip).shadow_dvo &= !(1 << (7 - gpio));
    }
    out_8(&mut (*regs).wkup_dvo, (*chip).shadow_dvo as u8);
}

unsafe extern "C" fn mpc52xx_wkup_gpio_set(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut GPIO_LOCK, &mut flags);
    __mpc52xx_wkup_gpio_set(gc, gpio, val);
    spin_unlock_irqrestore(&raw mut GPIO_LOCK, flags);
    pr_debug!("{}: gpio: {} val: {}\n", "mpc52xx_wkup_gpio_set", gpio, val);
    0
}

unsafe extern "C" fn mpc52xx_wkup_gpio_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio_wkup;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut GPIO_LOCK, &mut flags);
    (*chip).shadow_ddr &= !(1 << (7 - gpio));
    out_8(&mut (*regs).wkup_ddr, (*chip).shadow_ddr as u8);
    (*chip).shadow_gpioe |= 1 << (7 - gpio);
    out_8(&mut (*regs).wkup_gpioe, (*chip).shadow_gpioe as u8);
    spin_unlock_irqrestore(&raw mut GPIO_LOCK, flags);
    0
}

unsafe extern "C" fn mpc52xx_wkup_gpio_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio_wkup;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut GPIO_LOCK, &mut flags);
    __mpc52xx_wkup_gpio_set(gc, gpio, val);
    (*chip).shadow_ddr |= 1 << (7 - gpio);
    out_8(&mut (*regs).wkup_ddr, (*chip).shadow_ddr as u8);
    (*chip).shadow_gpioe |= 1 << (7 - gpio);
    out_8(&mut (*regs).wkup_gpioe, (*chip).shadow_gpioe as u8);
    spin_unlock_irqrestore(&raw mut GPIO_LOCK, flags);
    pr_debug!("{}: gpio: {} val: {}\n", "mpc52xx_wkup_gpio_dir_out", gpio, val);
    0
}

/* GPIO LIB API implementation for simple GPIOs. */
unsafe extern "C" fn mpc52xx_simple_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio;
    ((in_be32(&(*regs).simple_ival) >> (31 - gpio)) & 1) as c_int
}

unsafe fn __mpc52xx_simple_gpio_set(gc: *mut gpio_chip, gpio: c_uint, val: c_int) {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio;
    if val != 0 { (*chip).shadow_dvo |= 1 << (31 - gpio); }
    else { (*chip).shadow_dvo &= !(1 << (31 - gpio)); }
    out_be32(&mut (*regs).simple_dvo, (*chip).shadow_dvo);
}

unsafe extern "C" fn mpc52xx_simple_gpio_set(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut GPIO_LOCK, &mut flags);
    __mpc52xx_simple_gpio_set(gc, gpio, val);
    spin_unlock_irqrestore(&raw mut GPIO_LOCK, flags);
    pr_debug!("{}: gpio: {} val: {}\n", "mpc52xx_simple_gpio_set", gpio, val);
    0
}

unsafe extern "C" fn mpc52xx_simple_gpio_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut GPIO_LOCK, &mut flags);
    (*chip).shadow_ddr &= !(1 << (31 - gpio));
    out_be32(&mut (*regs).simple_ddr, (*chip).shadow_ddr);
    (*chip).shadow_gpioe |= 1 << (31 - gpio);
    out_be32(&mut (*regs).simple_gpioe, (*chip).shadow_gpioe);
    spin_unlock_irqrestore(&raw mut GPIO_LOCK, flags);
    0
}

unsafe extern "C" fn mpc52xx_simple_gpio_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    let chip = gpiochip_get_data(gc) as *mut mpc52xx_gpiochip;
    let regs = (*chip).regs as *mut mpc52xx_gpio;
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut GPIO_LOCK, &mut flags);
    __mpc52xx_simple_gpio_set(gc, gpio, val);
    (*chip).shadow_ddr |= 1 << (31 - gpio);
    out_be32(&mut (*regs).simple_ddr, (*chip).shadow_ddr);
    (*chip).shadow_gpioe |= 1 << (31 - gpio);
    out_be32(&mut (*regs).simple_gpioe, (*chip).shadow_gpioe);
    spin_unlock_irqrestore(&raw mut GPIO_LOCK, flags);
    pr_debug!("{}: gpio: {} val: {}\n", "mpc52xx_simple_gpio_dir_out", gpio, val);
    0
}

unsafe extern "C" fn mpc52xx_wkup_gpiochip_probe(ofdev: *mut platform_device) -> c_int {
    let dev = &mut (*ofdev).dev;
    let np = (*dev).of_node;
    let chip = devm_kzalloc(dev, core::mem::size_of::<mpc52xx_gpiochip>(), GFP_KERNEL)
        as *mut mpc52xx_gpiochip;
    if chip.is_null() { return -ENOMEM; }
    platform_set_drvdata(ofdev, chip as *mut _);
    let gc = &mut (*chip).gc;
    gc.base = -1;
    gc.ngpio = 8;
    gc.direction_input = Some(mpc52xx_wkup_gpio_dir_in);
    gc.direction_output = Some(mpc52xx_wkup_gpio_dir_out);
    gc.get = Some(mpc52xx_wkup_gpio_get);
    gc.set = Some(mpc52xx_wkup_gpio_set);
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if gc.label.is_null() { return -ENOMEM; }
    (*chip).regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if IS_ERR((*chip).regs) { return PTR_ERR((*chip).regs); }
    let ret = devm_gpiochip_add_data(dev, gc, chip as *mut _);
    if ret != 0 { return ret; }
    let regs = (*chip).regs as *mut mpc52xx_gpio_wkup;
    (*chip).shadow_gpioe = in_8(&(*regs).wkup_gpioe) as c_uint;
    (*chip).shadow_ddr = in_8(&(*regs).wkup_ddr) as c_uint;
    (*chip).shadow_dvo = in_8(&(*regs).wkup_dvo) as c_uint;
    0
}

unsafe extern "C" fn mpc52xx_simple_gpiochip_probe(ofdev: *mut platform_device) -> c_int {
    let dev = &mut (*ofdev).dev;
    let np = (*dev).of_node;
    let chip = devm_kzalloc(dev, core::mem::size_of::<mpc52xx_gpiochip>(), GFP_KERNEL)
        as *mut mpc52xx_gpiochip;
    if chip.is_null() { return -ENOMEM; }
    platform_set_drvdata(ofdev, chip as *mut _);
    let gc = &mut (*chip).gc;
    gc.base = -1;
    gc.ngpio = 32;
    gc.direction_input = Some(mpc52xx_simple_gpio_dir_in);
    gc.direction_output = Some(mpc52xx_simple_gpio_dir_out);
    gc.get = Some(mpc52xx_simple_gpio_get);
    gc.set = Some(mpc52xx_simple_gpio_set);
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if gc.label.is_null() { return -ENOMEM; }
    (*chip).regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if IS_ERR((*chip).regs) { return PTR_ERR((*chip).regs); }
    let ret = devm_gpiochip_add_data(dev, gc, chip as *mut _);
    if ret != 0 { return ret; }
    let regs = (*chip).regs as *mut mpc52xx_gpio;
    (*chip).shadow_gpioe = in_be32(&(*regs).simple_gpioe);
    (*chip).shadow_ddr = in_be32(&(*regs).simple_ddr);
    (*chip).shadow_dvo = in_be32(&(*regs).simple_dvo);
    0
}

static MPC52XX_WKUP_GPIOCHIP_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "fsl,mpc5200-gpio-wkup" }, of_device_id { compatible: "" }
];
static MPC52XX_SIMPLE_GPIOCHIP_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "fsl,mpc5200-gpio" }, of_device_id { compatible: "" }
];

static mut MPC52XX_WKUP_GPIOCHIP_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: "mpc5200-gpio-wkup", of_match_table: MPC52XX_WKUP_GPIOCHIP_MATCH.as_ptr() },
    probe: Some(mpc52xx_wkup_gpiochip_probe),
};
static mut MPC52XX_SIMPLE_GPIOCHIP_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: "mpc5200-gpio", of_match_table: MPC52XX_SIMPLE_GPIOCHIP_MATCH.as_ptr() },
    probe: Some(mpc52xx_simple_gpiochip_probe),
};

unsafe extern "C" fn mpc52xx_gpio_init() -> c_int {
    platform_register_drivers(DRIVERS.as_ptr(), DRIVERS.len())
}

unsafe extern "C" fn mpc52xx_gpio_exit() {
    platform_unregister_drivers(DRIVERS.as_ptr(), DRIVERS.len());
}

static DRIVERS: [*mut platform_driver; 2] = [
    &raw mut MPC52XX_WKUP_GPIOCHIP_DRIVER, &raw mut MPC52XX_SIMPLE_GPIOCHIP_DRIVER
];

// Make sure we get initialised before anyone else tries to use us.
subsys_initcall!(mpc52xx_gpio_init);
module_exit!(mpc52xx_gpio_exit);
module_description!("Freescale MPC52xx gpio driver");
module_author!("Sascha Hauer <s.hauer@pengutronix.de");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
